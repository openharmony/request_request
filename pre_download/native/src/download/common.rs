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
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub(crate) use super::CancelHandle;
use super::{DownloadTask, HttpClientError, RequestCallback, RequestTask, Response};
use crate::agent::CustomCallback;
use crate::cache::{Cache, CacheManager};
pub(crate) struct DownloadCallback {
    task_id: u64,
    cache: Option<Cache>,
    finish: Arc<AtomicBool>,
    callbacks: Arc<Mutex<Vec<Box<dyn CustomCallback>>>>,
}

impl DownloadCallback {
    pub(crate) fn new(
        task_id: u64,
        finish: Arc<AtomicBool>,
        callbacks: Arc<Mutex<Vec<Box<dyn CustomCallback>>>>,
    ) -> Self {
        Self {
            task_id,
            cache: None,
            finish,
            callbacks,
        }
    }
}

impl RequestCallback for DownloadCallback {
    fn on_success(&mut self, response: Response) {
        info!("{} success with code {}", self.task_id, response.status());
        let mut cache = self.cache.take().unwrap();
        let cache = cache.complete_write();
        self.finish.store(true, Ordering::Release);
        let mut callbacks = self.callbacks.lock().unwrap();

        while let Some(mut callback) = callbacks.pop() {
            callback.on_success(cache.clone());
        }
    }

    fn on_fail(&mut self, error: HttpClientError) {
        error!("{} download fail {}", self.task_id, error,);

        self.finish.store(true, Ordering::Release);
        let mut callbacks = self.callbacks.lock().unwrap();
        while let Some(mut callback) = callbacks.pop() {
            callback.on_fail(&error.to_string());
        }
    }

    fn on_cancel(&mut self) {
        info!("{} cancel download", self.task_id);
        self.finish.store(true, Ordering::Release);
        let mut callbacks = self.callbacks.lock().unwrap();
        while let Some(mut callback) = callbacks.pop() {
            callback.on_cancel();
        }
    }

    fn on_data_receive(&mut self, data: &[u8], mut task: RequestTask) {
        if self.cache.is_none() {
            let headers = task.headers();
            let length = parse_content_length(&headers);
            self.cache = Some(
                CacheManager::get_instance()
                    .apply_for_cache(self.task_id, length)
                    .unwrap(),
            );
        }
        self.cache.as_mut().unwrap().write_all(data).unwrap();
    }

    fn on_progress(&mut self, dl_total: u64, dl_now: u64, ul_total: u64, ul_now: u64) {}
}

fn parse_content_length(headers: &str) -> Option<usize> {
    headers.find("content-length").and_then(|position| {
        headers
            .split_at(position)
            .1
            .lines()
            .next()
            .and_then(|line| {
                line.find(":").map(|position| {
                    line.split_at(position + 1)
                        .1
                        .trim()
                        .parse::<usize>()
                        .unwrap_or(0)
                })
            })
    })
}

pub struct TaskHandle {
    cancel: CancelHandle,
    finish: Arc<AtomicBool>,
    callbacks: Arc<Mutex<Vec<Box<dyn CustomCallback>>>>,
}

impl TaskHandle {
    pub(crate) fn new(
        cancel: CancelHandle,
        finish: Arc<AtomicBool>,
        callbacks: Arc<Mutex<Vec<Box<dyn CustomCallback>>>>,
    ) -> Self {
        Self {
            cancel,
            finish,
            callbacks,
        }
    }
    pub(crate) fn cancel(&mut self) {
        self.cancel.cancel();
    }

    pub(crate) fn cancel_handle(&self) -> CancelHandle {
        self.cancel.clone()
    }

    pub(crate) fn try_add_callback(
        &mut self,
        callback: Box<dyn CustomCallback>,
    ) -> Result<(), Box<dyn CustomCallback>> {
        let mut callbacks = self.callbacks.lock().unwrap();
        if !self.finish.load(Ordering::Acquire) {
            callbacks.push(callback);
            Ok(())
        } else {
            Err(callback)
        }
    }
}

pub(crate) fn download(
    task_id: u64,
    url: &str,
    callback: Option<Box<dyn CustomCallback>>,
) -> TaskHandle {
    let callbacks = match callback {
        Some(callback) => Arc::new(Mutex::new(vec![callback])),
        None => Arc::new(Mutex::new(vec![])),
    };

    let finish = Arc::new(AtomicBool::new(false));
    let callback = DownloadCallback::new(task_id, finish.clone(), callbacks.clone());

    let cancel_handle = DownloadTask::run(url, callback);
    TaskHandle::new(cancel_handle, finish, callbacks)
}

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    use request_utils::fastrand::fast_random;

    use super::*;
    use crate::agent::CustomCallback;
    use crate::TEST_URL;

    struct TestCallback {
        flag: Arc<AtomicBool>,
    }

    impl CustomCallback for TestCallback {
        fn on_success(&mut self, data: Arc<Cache>) {
            if data.size() != 0 {
                self.flag.store(true, Ordering::Release);
            }
        }
    }

    #[test]
    fn ut_pre_download() {
        let success_flag = Arc::new(AtomicBool::new(false));
        download(
            fast_random(),
            TEST_URL,
            Some(Box::new(TestCallback {
                flag: success_flag.clone(),
            })),
        );
        std::thread::sleep(Duration::from_secs(1));
        assert!(success_flag.load(Ordering::Acquire));
    }
}
