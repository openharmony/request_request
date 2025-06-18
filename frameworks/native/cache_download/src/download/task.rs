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

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use cache_core::CacheManager;
use netstack_rs::info::DownloadInfoMgr;
use request_utils::info;
use request_utils::task_id::TaskId;

use super::callback::PrimeCallback;
use super::common::CommonHandle;
use super::{INIT, SUCCESS};

cfg_ylong! {
    use crate::download::ylong;
}

cfg_netstack! {
    use crate::download::netstack;
}

use crate::services::{DownloadRequest, PreloadCallback};

pub enum Downloader {
    Netstack,
    Ylong,
}

pub(crate) struct DownloadTask {
    pub(crate) remove_flag: bool,
    pub(crate) seq: usize,
    pub(crate) handle: TaskHandle,
}

impl DownloadTask {
    pub(crate) fn new(
        task_id: TaskId,
        cache_manager: &'static CacheManager,
        info_mgr: Arc<DownloadInfoMgr>,
        request: DownloadRequest,
        callback: Box<dyn PreloadCallback>,
        downloader: Downloader,
        seq: usize,
    ) -> Self {
        info!("new preload task {} seq {}", task_id.brief(), seq);
        let mut handle = None;
        match downloader {
            Downloader::Netstack => {
                #[cfg(feature = "netstack")]
                {
                    handle = Some(download_inner(
                        task_id,
                        cache_manager,
                        info_mgr,
                        request,
                        Some(callback),
                        netstack::DownloadTask::run,
                        seq,
                    ));
                }
            }
            Downloader::Ylong => {
                #[cfg(feature = "ylong")]
                {
                    handle = Some(download_inner(
                        task_id,
                        cache_manager,
                        request,
                        Some(callback),
                        ylong::DownloadTask::run,
                        seq,
                    ));
                }
            }
        };
        Self {
            remove_flag: false,
            seq,
            handle: handle.unwrap(),
        }
    }

    pub(crate) fn cancel(&mut self) {
        self.handle.cancel();
    }

    pub(crate) fn task_handle(&self) -> TaskHandle {
        self.handle.clone()
    }

    pub(crate) fn try_add_callback(
        &mut self,
        callback: Box<dyn PreloadCallback>,
    ) -> Result<(), Box<dyn PreloadCallback>> {
        self.handle.try_add_callback(callback)
    }
}

#[derive(Clone)]
pub struct TaskHandle {
    task_id: TaskId,
    handle: Option<Arc<dyn CommonHandle>>,
    state: Arc<AtomicUsize>,
    finish: Arc<AtomicBool>,
    callbacks: Arc<Mutex<VecDeque<Box<dyn PreloadCallback>>>>,
}

impl TaskHandle {
    pub(crate) fn new(task_id: TaskId) -> Self {
        Self {
            state: Arc::new(AtomicUsize::new(INIT)),
            task_id,
            handle: None,
            finish: Arc::new(AtomicBool::new(false)),
            callbacks: Arc::new(Mutex::new(VecDeque::with_capacity(1))),
        }
    }
    pub(crate) fn cancel(&mut self) {
        if let Some(handle) = self.handle.take() {
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

    pub(crate) fn reset(&mut self) {
        if self.finish.load(Ordering::Acquire) {
            return;
        }
        if let Some(handle) = self.handle.as_ref() {
            handle.reset();
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
        callback: Box<dyn PreloadCallback>,
    ) -> Result<(), Box<dyn PreloadCallback>> {
        let mut callbacks = self.callbacks.lock().unwrap();
        if !self.finish.load(Ordering::Acquire) {
            info!("add callback to task {}", self.task_id.brief());
            callbacks.push_back(callback);
            if let Some(handle) = self.handle.as_ref() {
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
    fn callbacks(&self) -> Arc<Mutex<VecDeque<Box<dyn PreloadCallback>>>> {
        self.callbacks.clone()
    }

    #[inline]
    fn set_handle(&mut self, handle: Arc<dyn CommonHandle>) {
        self.handle = Some(handle);
    }
}

fn download_inner<F>(
    task_id: TaskId,
    cache_manager: &'static CacheManager,
    info_mgr: Arc<DownloadInfoMgr>,
    request: DownloadRequest,
    callback: Option<Box<dyn PreloadCallback>>,
    downloader: F,
    seq: usize,
) -> TaskHandle
where
    F: Fn(DownloadRequest, PrimeCallback, Arc<DownloadInfoMgr>) -> Arc<dyn CommonHandle>,
{
    let mut handle = TaskHandle::new(task_id.clone());
    if let Some(callback) = callback {
        handle.callbacks.lock().unwrap().push_back(callback);
    }

    let callback = PrimeCallback::new(
        task_id,
        cache_manager,
        handle.finish_flag(),
        handle.state_flag(),
        handle.callbacks(),
        seq,
    );

    let task = downloader(request, callback, info_mgr);
    handle.set_handle(task);
    handle
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::io::{BufReader, Lines};
    use std::net::TcpStream;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, LazyLock};
    use std::thread;
    use std::time::Duration;

    use cache_core::{CacheManager, RamCache};
    use netstack_rs::info::DownloadInfoMgr;
    use request_utils::test::log::init;
    use request_utils::test::server::test_server;

    use super::*;
    use crate::services::PreloadCallback;

    const TEST_URL: &str = "https://www.baidu.com";

    struct TestCallback {
        flag: Arc<AtomicBool>,
    }

    impl PreloadCallback for TestCallback {
        fn on_success(&mut self, data: Arc<RamCache>, _task_id: &str) {
            if data.size() != 0 {
                self.flag.store(true, Ordering::Release);
            }
        }
    }

    #[cfg(feature = "ohos")]
    const DOWNLOADER: for<'a> fn(
        DownloadRequest<'a>,
        PrimeCallback,
        Arc<DownloadInfoMgr>,
    ) -> Arc<(dyn CommonHandle + 'static)> = netstack::DownloadTask::run;

    #[cfg(not(feature = "ohos"))]
    const DOWNLOADER: for<'a> fn(
        DownloadRequest<'a>,
        PrimeCallback,
    ) -> Arc<(dyn CommonHandle + 'static)> = ylong::DownloadTask::run;

    #[test]
    fn ut_preload() {
        init();
        static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
        let success_flag = Arc::new(AtomicBool::new(false));
        let request = DownloadRequest::new(TEST_URL);
        let info_mgr = Arc::new(DownloadInfoMgr::new());
        let handle = download_inner(
            TaskId::from_url(TEST_URL),
            &CACHE_MANAGER,
            info_mgr,
            request,
            Some(Box::new(TestCallback {
                flag: success_flag.clone(),
            })),
            DOWNLOADER,
            0,
        );
        while !handle.is_finish() {
            thread::sleep(Duration::from_millis(500));
        }
        assert!(success_flag.load(Ordering::Acquire));
    }

    #[test]
    fn ut_download_headers() {
        init();
        static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
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
        let info_mgr = Arc::new(DownloadInfoMgr::new());
        let handle = download_inner(
            TaskId::from_url(&server),
            &CACHE_MANAGER,
            info_mgr,
            request,
            None,
            DOWNLOADER,
            0,
        );
        while !handle.is_finish() {
            thread::sleep(Duration::from_millis(500));
        }
        assert!(flag.load(Ordering::SeqCst));
    }
}
