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

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::sync::{Arc, LazyLock, Mutex};

use crate::cache::{CacheManager, Fetcher, RamCache, Updater};
use crate::download::TaskHandle;
use crate::utils::url_hash;
use crate::DownloadError;

cfg_ohos! {
    use crate::wrapper::ffi::{FfiPredownloadOptions,PreloadCallbackWrapper};
    use crate::wrapper::FfiCallback;
}

#[allow(unused_variables)]
pub trait CustomCallback: Send {
    fn on_success(&mut self, data: Arc<RamCache>, task_id: &str) {}
    fn on_fail(&mut self, error: DownloadError, task_id: &str) {}
    fn on_cancel(&mut self) {}
    fn on_progress(&mut self, progress: u64, total: u64) {}
}

pub struct DownloadAgent {
    running_tasks: Mutex<HashMap<TaskId, Arc<Mutex<Updater>>>>,
}

pub struct DownloadRequest<'a> {
    pub url: &'a str,
    pub headers: Option<Vec<(&'a str, &'a str)>>,
}

impl<'a> DownloadRequest<'a> {
    pub fn new(url: &'a str) -> Self {
        Self { url, headers: None }
    }

    pub fn headers(&mut self, headers: Vec<(&'a str, &'a str)>) -> &mut Self {
        self.headers = Some(headers);
        self
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub(crate) struct TaskId {
    hash: String,
}

impl TaskId {
    pub fn new(hash: String) -> Self {
        Self { hash }
    }

    pub fn from_url(url: &str) -> Self {
        Self {
            hash: url_hash(url),
        }
    }

    pub fn brief(&self) -> &str {
        let len = self.hash.len();
        &self.hash.as_str()[..len / 4]
    }

    #[cfg(test)]
    pub fn random() -> Self {
        use request_utils::fastrand;
        Self {
            hash: url_hash(fastrand::fast_random().to_string().as_str()),
        }
    }
}

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hash)
    }
}

impl DownloadAgent {
    fn new() -> Self {
        Self {
            running_tasks: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_instance() -> &'static Self {
        static DOWNLOAD_AGENT: LazyLock<DownloadAgent> = LazyLock::new(|| {
            #[cfg(not(test))]
            CacheManager::get_instance().init();
            DownloadAgent::new()
        });

        &DOWNLOAD_AGENT
    }

    pub fn cancel(&self, url: &str) {
        let task_id = TaskId::from_url(url);
        if let Some(updater) = self.running_tasks.lock().unwrap().get(&task_id).cloned() {
            updater.lock().unwrap().cancel();
        }
    }

    pub fn remove(&self, url: &str) {
        let task_id = TaskId::from_url(url);
        if let Some(updater) = self.running_tasks.lock().unwrap().remove(&task_id) {
            updater.lock().unwrap().cancel();
        }
        CacheManager::get_instance().remove(task_id);
    }

    pub fn preload(
        &self,
        request: DownloadRequest,
        mut callback: Box<dyn CustomCallback>,
        update: bool,
    ) -> TaskHandle {
        let url = request.url;
        let task_id = TaskId::from_url(url);

        if !update {
            if let Err(ret) = self.fetch(&task_id, callback) {
                error!("{} fetch fail", task_id.brief());
                callback = ret;
            } else {
                info!("{} fetch success", task_id.brief());
                let handle = TaskHandle::new(task_id);
                handle.set_completed();
                return handle;
            }
        }

        loop {
            let cb = callback;
            let updater = match self.running_tasks.lock().unwrap().entry(task_id.clone()) {
                Entry::Occupied(entry) => entry.get().clone(),
                Entry::Vacant(entry) => {
                    let updater =
                        Arc::new(Mutex::new(Updater::new(task_id.clone(), request, cb, 0)));
                    let handle = updater.lock().unwrap().task_handle();
                    entry.insert(updater);
                    return handle;
                }
            };

            let mut updater = updater.lock().unwrap();

            let handle = match updater.try_add_callback(cb) {
                Ok(()) => updater.task_handle(),
                Err(cb) => {
                    if let Err(cb) = self.fetch(&task_id, cb) {
                        error!("{} fetch fail after update", task_id.brief());
                        if !updater.remove_flag {
                            let seq = updater.seq + 1;
                            *updater = Updater::new(task_id.clone(), request, cb, seq);
                            updater.task_handle()
                        } else {
                            callback = cb;
                            continue;
                        }
                    } else {
                        info!("{} fetch success", task_id.brief());
                        let handle = TaskHandle::new(task_id);
                        handle.set_completed();
                        handle
                    }
                }
            };
            break handle;
        }
    }

    pub(crate) fn task_finish(&self, task_id: &TaskId, seq: usize) {
        let Some(updater) = self.running_tasks.lock().unwrap().get(task_id).cloned() else {
            return;
        };
        let mut updater = updater.lock().unwrap();
        if updater.seq == seq {
            updater.remove_flag = true;
            self.running_tasks.lock().unwrap().remove(task_id);
        }
    }

    #[cfg(feature = "ohos")]
    pub(crate) fn ffi_preload(
        &self,
        url: &str,
        callback: cxx::UniquePtr<PreloadCallbackWrapper>,
        update: bool,
        options: &FfiPredownloadOptions,
    ) -> Box<TaskHandle> {
        let Some(callback) = FfiCallback::from_ffi(callback) else {
            error!("ffi_preload callback is null");
            return Box::new(TaskHandle::new(TaskId::from_url(url)));
        };

        let mut request = DownloadRequest::new(url);
        if !options.headers.is_empty() {
            let headers = options
                .headers
                .chunks(2)
                .map(|a| (a[0], a[1]))
                .collect::<Vec<(&str, &str)>>();
            request.headers(headers);
        }

        Box::new(self.preload(request, Box::new(callback), update))
    }

    pub fn set_file_cache_size(&self, size: u64) {
        info!("set file cache size to {}", size);
        CacheManager::get_instance().set_file_cache_size(size);
    }

    pub fn set_ram_cache_size(&self, size: u64) {
        info!("set ram cache size to {}", size);
        CacheManager::get_instance().set_ram_cache_size(size);
    }

    fn fetch(
        &self,
        task_id: &TaskId,
        callback: Box<dyn CustomCallback>,
    ) -> Result<(), Box<dyn CustomCallback>> {
        let fetcher = Fetcher::new(task_id);
        fetcher.fetch_with_callback(callback)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::io::{BufReader, Lines};
    use std::net::TcpStream;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    use super::*;
    use crate::{init, test_server, TEST_URL};

    const ERROR_IP: &str = "127.12.31.12";
    const NO_DATA: usize = 1359;

    struct TestCallbackN;
    impl CustomCallback for TestCallbackN {}

    struct TestCallbackS {
        flag: Arc<AtomicUsize>,
    }

    impl CustomCallback for TestCallbackS {
        fn on_success(&mut self, data: Arc<RamCache>, _task_id: &str) {
            if data.size() != 0 {
                self.flag.fetch_add(1, Ordering::SeqCst);
            } else {
                self.flag.store(NO_DATA, Ordering::SeqCst);
            }
        }
    }

    struct TestCallbackF {
        flag: Arc<Mutex<String>>,
    }

    impl CustomCallback for TestCallbackF {
        fn on_fail(&mut self, error: DownloadError, _task_id: &str) {
            *self.flag.lock().unwrap() = error.message().to_string();
        }
    }

    struct TestCallbackC {
        flag: Arc<AtomicUsize>,
    }

    impl CustomCallback for TestCallbackC {
        fn on_cancel(&mut self) {
            self.flag.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn ut_preload_success() {
        init();
        let agent = DownloadAgent::new();
        let success_flag = Arc::new(AtomicUsize::new(0));
        let callback = Box::new(TestCallbackS {
            flag: success_flag.clone(),
        });
        agent.preload(DownloadRequest::new(TEST_URL), callback, false);
        std::thread::sleep(Duration::from_secs(1));
        assert_eq!(success_flag.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn ut_preload_success_add_callback() {
        init();
        let agent = DownloadAgent::new();
        let success_flag_0 = Arc::new(AtomicUsize::new(0));
        let callback_0 = Box::new(TestCallbackS {
            flag: success_flag_0.clone(),
        });

        let success_flag_1 = Arc::new(AtomicUsize::new(0));
        let callback_1 = Box::new(TestCallbackS {
            flag: success_flag_1.clone(),
        });

        agent.preload(DownloadRequest::new(TEST_URL), callback_0, false);
        agent.preload(DownloadRequest::new(TEST_URL), callback_1, false);
        std::thread::sleep(Duration::from_secs(1));
        assert_eq!(success_flag_0.load(Ordering::SeqCst), 1);
        assert_eq!(success_flag_1.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn ut_preload_fail() {
        init();
        let agent = DownloadAgent::new();
        let error = Arc::new(Mutex::new(String::new()));
        let callback = Box::new(TestCallbackF {
            flag: error.clone(),
        });
        agent.preload(DownloadRequest::new(ERROR_IP), callback, false);
        std::thread::sleep(Duration::from_secs(1));
        assert!(!error.lock().unwrap().as_str().is_empty());
    }

    #[test]
    fn ut_preload_fail_add_callback() {
        init();
        let agent = DownloadAgent::new();
        let error_0 = Arc::new(Mutex::new(String::new()));
        let callback_0 = Box::new(TestCallbackF {
            flag: error_0.clone(),
        });
        let error_1 = Arc::new(Mutex::new(String::new()));
        let callback_1 = Box::new(TestCallbackF {
            flag: error_1.clone(),
        });

        agent.preload(DownloadRequest::new(ERROR_IP), callback_0, false);
        agent.preload(DownloadRequest::new(ERROR_IP), callback_1, false);
        std::thread::sleep(Duration::from_secs(1));
        assert!(!error_0.lock().unwrap().as_str().is_empty());
        assert!(!error_1.lock().unwrap().as_str().is_empty());
    }

    #[test]
    fn ut_preload_cancel_0() {
        init();
        let agent = DownloadAgent::new();
        let cancel_flag = Arc::new(AtomicUsize::new(0));
        let callback = Box::new(TestCallbackC {
            flag: cancel_flag.clone(),
        });
        let handle = agent.preload(DownloadRequest::new(TEST_URL), callback, true);
        handle.cancel();
        std::thread::sleep(Duration::from_secs(1));
        assert_eq!(cancel_flag.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn ut_preload_cancel_1() {
        init();
        let agent = DownloadAgent::new();
        let cancel_flag = Arc::new(AtomicUsize::new(0));
        let callback = Box::new(TestCallbackC {
            flag: cancel_flag.clone(),
        });
        agent.preload(DownloadRequest::new(TEST_URL), callback, true);
        agent.cancel(TEST_URL);
        std::thread::sleep(Duration::from_secs(1));
        assert_eq!(cancel_flag.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn ut_preload_cancel_add_callback() {
        init();
        let agent = DownloadAgent::new();
        let cancel_flag_0 = Arc::new(AtomicUsize::new(0));
        let callback_0 = Box::new(TestCallbackC {
            flag: cancel_flag_0.clone(),
        });
        let cancel_flag_1 = Arc::new(AtomicUsize::new(0));
        let callback_1 = Box::new(TestCallbackC {
            flag: cancel_flag_1.clone(),
        });

        let handle_0 = agent.preload(DownloadRequest::new(TEST_URL), callback_0, false);
        agent.preload(DownloadRequest::new(TEST_URL), callback_1, false);
        handle_0.cancel();
        std::thread::sleep(Duration::from_secs(1));
        assert_eq!(cancel_flag_0.load(Ordering::SeqCst), 1);
        assert_eq!(cancel_flag_1.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn ut_preload_already_success() {
        init();
        let agent = DownloadAgent::new();
        agent.preload(
            DownloadRequest::new(TEST_URL),
            Box::new(TestCallbackN),
            false,
        );
        std::thread::sleep(Duration::from_secs(1));

        let success_flag = Arc::new(AtomicUsize::new(0));
        let callback = Box::new(TestCallbackS {
            flag: success_flag.clone(),
        });
        agent.preload(DownloadRequest::new(TEST_URL), callback, false);
        std::thread::sleep(Duration::from_millis(500));
        assert_eq!(success_flag.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn ut_preload_local_headers() {
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
        let success_flag = Arc::new(AtomicUsize::new(0));
        let callback = Box::new(TestCallbackS {
            flag: success_flag.clone(),
        });
        DownloadAgent::new().preload(request, callback, true);
        std::thread::sleep(Duration::from_millis(200));
        assert!(flag.load(Ordering::SeqCst));
        assert_eq!(success_flag.load(Ordering::SeqCst), NO_DATA);
    }
}
