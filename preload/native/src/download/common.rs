// Copyright (C) 2024 Huawei Device Co., Ltd.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

cfg_ohos! {
    use netstack_rs::error::HttpErrorCode;
}

use super::{CancelHandle, DownloadTask, HttpClientError, RequestCallback, RequestTask, Response};
use crate::agent::{CustomCallback, DownloadRequest, TaskId};
use crate::cache::{CacheManager, RamCache};
use crate::{DownloadAgent, DownloadError};

const INIT: usize = 0;
const RUNNING: usize = 1;
const SUCCESS: usize = 2;
const FAIL: usize = 3;
const CANCEL: usize = 4;

pub(crate) struct DownloadCallback {
    task_id: TaskId,
    cache: Option<RamCache>,
    finish: Arc<AtomicBool>,
    state: Arc<AtomicUsize>,
    callbacks: Arc<Mutex<Vec<Box<dyn CustomCallback>>>>,
    processed: u64,
    seq: usize,
}

impl DownloadCallback {
    pub(crate) fn new(
        task_id: TaskId,
        finish: Arc<AtomicBool>,
        callbacks: Arc<Mutex<Vec<Box<dyn CustomCallback>>>>,
        state: Arc<AtomicUsize>,
        seq: usize,
    ) -> Self {
        Self {
            task_id,
            state,
            cache: None,
            finish,
            callbacks,
            processed: 0,
            seq,
        }
    }

    pub(crate) fn set_running(&self) {
        self.state.store(RUNNING, Ordering::Release);
    }
}

impl DownloadCallback {
    fn on_success_inner(&mut self) {
        info!("{} success", self.task_id.brief());

        let cache = match self.cache.take() {
            Some(cache) => cache.finish_write(),
            None => Arc::new(RamCache::new(
                self.task_id.clone(),
                CacheManager::get_instance(),
                Some(0),
            )),
        };
        self.state.store(SUCCESS, Ordering::Release);
        self.finish.store(true, Ordering::Release);
        let mut callbacks = self.callbacks.lock().unwrap();
        while let Some(mut callback) = callbacks.pop() {
            callback.on_success(cache.clone(), self.task_id.brief());
        }
        drop(callbacks);
        self.notify_agent_finish();
    }

    fn on_fail_inner(&mut self, error: HttpClientError) {
        error!("{} download fail {}", self.task_id.brief(), error,);
        self.state.store(FAIL, Ordering::Release);
        self.finish.store(true, Ordering::Release);
        let mut callbacks = self.callbacks.lock().unwrap();
        while let Some(mut callback) = callbacks.pop() {
            callback.on_fail(DownloadError::from(&error), self.task_id.brief());
        }
        drop(callbacks);
        self.notify_agent_finish();
    }

    fn notify_agent_finish(&self) {
        DownloadAgent::get_instance().task_finish(&self.task_id, self.seq);
    }
}

impl RequestCallback for DownloadCallback {
    fn on_success(&mut self, response: Response) {
        let status = response.status();
        info!("{} status code {}", self.task_id.brief(), status);
        #[cfg(feature = "ohos")]
        if (status.clone() as u32 >= 300) || (status.clone() as u32) < 200 {
            self.on_fail_inner(HttpClientError::new(
                HttpErrorCode::HttpNoneErr,
                status.to_string(),
            ));
            return;
        }
        self.on_success_inner();
    }

    fn on_fail(&mut self, error: HttpClientError) {
        #[cfg(feature = "ohos")]
        if *error.code() == HttpErrorCode::HttpWriteError {
            self.on_cancel();
            return;
        }
        self.on_fail_inner(error);
    }

    fn on_cancel(&mut self) {
        info!("{} is cancel", self.task_id.brief());
        self.state.store(CANCEL, Ordering::Release);

        self.finish.store(true, Ordering::Release);
        let mut callbacks = self.callbacks.lock().unwrap();
        while let Some(mut callback) = callbacks.pop() {
            callback.on_cancel();
        }
        drop(callbacks);
        self.notify_agent_finish();
    }

    #[allow(unused_mut)]
    fn on_data_receive(&mut self, data: &[u8], mut task: RequestTask) {
        if self.cache.is_none() {
            let headers = task.headers();
            let is_chunked = headers
                .get("transfer-encoding")
                .map(|s| s == "chunked")
                .unwrap_or(false);

            let size = if is_chunked {
                None
            } else {
                headers
                    .get("content-length")
                    .and_then(|s| s.parse::<usize>().ok())
            };

            info!("{} content-length info {:?}", self.task_id.brief(), size);
            let apply_cache =
                RamCache::new(self.task_id.clone(), CacheManager::get_instance(), size);

            self.cache = Some(apply_cache)
        }
        self.cache.as_mut().unwrap().write_all(data).unwrap();
    }

    fn on_progress(&mut self, dl_total: u64, dl_now: u64, _ul_total: u64, _ul_now: u64) {
        if dl_now > self.processed {
            self.processed = dl_now;
        } else {
            return;
        }
        let mut callbacks = self.callbacks.lock().unwrap();
        for callback in callbacks.iter_mut() {
            callback.on_progress(dl_now, dl_total);
        }
    }
}

#[derive(Clone)]
pub struct TaskHandle {
    task_id: TaskId,
    cancel_handle: Option<CancelHandle>,
    state: Arc<AtomicUsize>,
    finish: Arc<AtomicBool>,
    callbacks: Arc<Mutex<Vec<Box<dyn CustomCallback>>>>,
}

impl TaskHandle {
    pub(crate) fn new(task_id: TaskId) -> Self {
        Self {
            state: Arc::new(AtomicUsize::new(INIT)),
            task_id,
            cancel_handle: None,
            finish: Arc::new(AtomicBool::new(false)),
            callbacks: Arc::new(Mutex::new(vec![])),
        }
    }
    pub(crate) fn cancel(&mut self) {
        if let Some(handle) = self.cancel_handle.take() {
            info!("cancel task {}", self.task_id.brief());
            if self.finish.load(Ordering::Acquire) {
                return;
            }
            let _callback = self.callbacks.lock().unwrap();
            if self.finish.load(Ordering::Acquire) {
                return;
            }
            if handle.cancel() {
                self.finish.store(true, Ordering::Release);
            }
        } else {
            error!("cancel task {} not exist", self.task_id.brief());
        }
    }

    pub fn task_id(&self) -> String {
        self.task_id.to_string()
    }

    pub fn is_finish(&self) -> bool {
        self.finish.load(Ordering::Acquire)
    }

    pub fn state(&self) -> usize {
        self.state.load(Ordering::Acquire)
    }

    pub(crate) fn set_completed(&self) {
        self.state.store(SUCCESS, Ordering::Relaxed);
        self.finish.store(true, Ordering::Relaxed);
    }

    pub(crate) fn try_add_callback(
        &mut self,
        callback: Box<dyn CustomCallback>,
    ) -> Result<(), Box<dyn CustomCallback>> {
        let mut callbacks = self.callbacks.lock().unwrap();
        if !self.finish.load(Ordering::Acquire) {
            info!("add callback to task {}", self.task_id.brief());
            callbacks.push(callback);
            if let Some(handle) = self.cancel_handle.as_ref() {
                handle.add_count();
            }
            Ok(())
        } else {
            Err(callback)
        }
    }

    #[inline]
    fn state_flag(&self) -> Arc<AtomicUsize> {
        self.state.clone()
    }

    #[inline]
    fn finish_flag(&self) -> Arc<AtomicBool> {
        self.finish.clone()
    }

    #[inline]
    fn callbacks(&self) -> Arc<Mutex<Vec<Box<dyn CustomCallback>>>> {
        self.callbacks.clone()
    }

    #[inline]
    fn set_cancel_handle(&mut self, handle: CancelHandle) {
        self.cancel_handle = Some(handle);
    }
}

pub(crate) fn download(
    task_id: TaskId,
    request: DownloadRequest,
    callback: Option<Box<dyn CustomCallback>>,
    seq: usize,
) -> TaskHandle {
    let mut handle = TaskHandle::new(task_id.clone());
    if let Some(callback) = callback {
        handle.callbacks.lock().unwrap().push(callback);
    }

    let callback = DownloadCallback::new(
        task_id,
        handle.finish_flag(),
        handle.callbacks(),
        handle.state_flag(),
        seq,
    );
    let cancel_handle = DownloadTask::run(request, callback);
    handle.set_cancel_handle(cancel_handle);
    handle
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::io::{BufReader, Lines};
    use std::net::TcpStream;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    use super::*;
    use crate::agent::CustomCallback;
    use crate::{init, test_server, TEST_URL};

    struct TestCallback {
        flag: Arc<AtomicBool>,
    }

    impl CustomCallback for TestCallback {
        fn on_success(&mut self, data: Arc<RamCache>, _task_id: &str) {
            if data.size() != 0 {
                self.flag.store(true, Ordering::Release);
            }
        }
    }

    #[test]
    fn ut_preload() {
        let success_flag = Arc::new(AtomicBool::new(false));
        let request = DownloadRequest::new(TEST_URL);
        let handle = download(
            TaskId::from_url(TEST_URL),
            request,
            Some(Box::new(TestCallback {
                flag: success_flag.clone(),
            })),
            0,
        );
        if !handle.is_finish() {
            thread::sleep(Duration::from_millis(500));
        }
        thread::sleep(Duration::from_millis(10));
        assert!(success_flag.load(Ordering::Acquire));
    }

    #[test]
    fn ut_download_headers() {
        init();
        let headers = vec![
            ("User-Agent", "Mozilla/5.0"),
            ("Accept", "text/html"),
            ("Accept-Language", "en-US"),
            ("Accept-Encoding", "gzip, deflate"),
            ("Connection", "keep-alive"),
        ];
        let mut headers_clone: HashSet<String> = headers
            .iter()
            .map(|(k, v)| format!("{}:{}", k.to_ascii_lowercase(), v.to_ascii_lowercase()))
            .collect();

        let flag = Arc::new(AtomicBool::new(false));
        let flag_clone = flag.clone();
        let test_f = move |mut lines: Lines<BufReader<&mut TcpStream>>| {
            for line in lines.by_ref() {
                let line = line.unwrap();
                let line = line.to_ascii_lowercase();
                if line.is_empty() {
                    break;
                }
                headers_clone.remove(&line);
            }
            if headers_clone.is_empty() {
                flag_clone.store(true, Ordering::SeqCst);
            }
        };
        let server = test_server(test_f);
        let mut request = DownloadRequest::new(&server);
        request.headers(headers);
        let handle = download(TaskId::from_url(&server), request, None, 0);
        if !handle.is_finish() {
            thread::sleep(Duration::from_millis(500));
        }
        thread::sleep(Duration::from_millis(10));
        assert!(flag.load(Ordering::SeqCst));
    }
}
