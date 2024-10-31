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

use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

use request_utils::fastrand::fast_random;

use crate::cache::{Cache, CacheManager, Fetcher, Updater};
use crate::download::CancelHandle;

cfg_ohos! {
    use crate::wrapper::ffi::PreDownloadCallback;
    use crate::wrapper::FfiCallback;
}

#[allow(unused_variables)]
pub trait CustomCallback: Send {
    fn on_success(&mut self, data: Arc<Cache>) {}
    fn on_fail(&mut self, error: &str) {}
    fn on_cancel(&mut self) {}
}

pub struct DownloadAgent {
    tasks: Mutex<HashMap<String, u64>>,
    running_tasks: Mutex<HashMap<u64, Updater>>,
}

impl DownloadAgent {
    fn new() -> Self {
        Self {
            tasks: Mutex::new(HashMap::new()),
            running_tasks: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_instance() -> &'static Self {
        static CACHE_MANAGER: LazyLock<DownloadAgent> = LazyLock::new(DownloadAgent::new);
        &CACHE_MANAGER
    }

    pub fn cancel(&self, url: String) {
        if let Some(task_id) = self.tasks.lock().unwrap().get(&url) {
            if let Some(updater) = self.running_tasks.lock().unwrap().get_mut(task_id) {
                updater.cancel();
            }
        }
    }

    pub fn remove(&self, url: String) {
        if let Some(task_id) = self.tasks.lock().unwrap().remove(&url) {
            self.running_tasks.lock().unwrap().remove(&task_id);
        }
    }

    pub fn pre_download(
        &self,
        url: String,
        mut callback: Box<dyn CustomCallback>,
        update: bool,
    ) -> Option<CancelHandle> {
        let mut tasks = self.tasks.lock().unwrap();
        let mut running_tasks = self.running_tasks.lock().unwrap();

        if let Some(task_id) = tasks.get(&url) {
            info!("task {} exist", task_id);
            if let Some(updater) = running_tasks.get_mut(task_id) {
                if let Err(ret) = updater.try_add_callback(callback) {
                    info!("task {} completed", task_id);
                    callback = ret;
                } else {
                    info!("task {} add callback success", task_id);
                    return Some(updater.cancel_handle());
                }
            }
            if !update {
                if let Err(ret) = self.fetch(task_id, callback) {
                    error!("{} fetch fail", task_id);
                    callback = ret;
                } else {
                    info!("{} fetch success", task_id);
                    return None;
                }
            }
        }

        let task_id = fast_random();
        info!("new pre_download task {}", task_id);

        let updater = Updater::new(task_id, &url, callback);
        let handle = updater.cancel_handle();
        tasks.insert(url, task_id);
        running_tasks.insert(task_id, updater);
        Some(handle)
    }

    #[cfg(feature = "ohos")]
    pub(crate) fn ffi_pre_download(
        &self,
        url: String,
        callback: cxx::UniquePtr<PreDownloadCallback>,
        update: bool,
    ) {
        let Some(callback) = FfiCallback::from_ffi(callback) else {
            error!("ffi_pre_download callback is null");
            return;
        };
        let _ = self.pre_download(url, Box::new(callback), update);
    }

    fn fetch(
        &self,
        task_id: &u64,
        callback: Box<dyn CustomCallback>,
    ) -> Result<(), Box<dyn CustomCallback>> {
        let fetcher = Fetcher::new(*task_id);
        fetcher.fetch_with_callback(callback)
    }
}

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    use super::*;
    use crate::{init, TEST_URL};

    const ERROR_IP: &str = "127.12.31.12";

    struct TestCallbackN;
    impl CustomCallback for TestCallbackN {}

    struct TestCallbackS {
        flag: Arc<AtomicUsize>,
    }

    impl CustomCallback for TestCallbackS {
        fn on_success(&mut self, data: Arc<Cache>) {
            if data.size() != 0 {
                self.flag.fetch_add(1, Ordering::SeqCst);
            }
        }
    }

    struct TestCallbackF {
        flag: Arc<Mutex<String>>,
    }

    impl CustomCallback for TestCallbackF {
        fn on_fail(&mut self, error: &str) {
            self.flag.lock().unwrap().push_str(error);
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
    fn ut_pre_download_success() {
        init();
        let agent = DownloadAgent::new();
        let success_flag = Arc::new(AtomicUsize::new(0));
        let callback = Box::new(TestCallbackS {
            flag: success_flag.clone(),
        });
        agent.pre_download(TEST_URL.to_string(), callback, false);
        std::thread::sleep(Duration::from_secs(1));
        assert_eq!(success_flag.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn ut_pre_download_success_add_callback() {
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

        agent.pre_download(TEST_URL.to_string(), callback_0, false);
        agent.pre_download(TEST_URL.to_string(), callback_1, false);
        std::thread::sleep(Duration::from_secs(1));
        assert_eq!(success_flag_0.load(Ordering::SeqCst), 1);
        assert_eq!(success_flag_1.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn ut_pre_download_fail() {
        init();
        let agent = DownloadAgent::new();
        let error = Arc::new(Mutex::new(String::new()));
        let callback = Box::new(TestCallbackF {
            flag: error.clone(),
        });
        agent.pre_download(ERROR_IP.to_string(), callback, false);
        std::thread::sleep(Duration::from_secs(1));
        assert!(!error.lock().unwrap().as_str().is_empty());
    }

    #[test]
    fn ut_pre_download_fail_add_callback() {
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

        agent.pre_download(ERROR_IP.to_string(), callback_0, false);
        agent.pre_download(ERROR_IP.to_string(), callback_1, false);
        std::thread::sleep(Duration::from_secs(1));
        assert!(!error_0.lock().unwrap().as_str().is_empty());
        assert!(!error_1.lock().unwrap().as_str().is_empty());
    }

    #[test]
    fn ut_pre_download_cancel() {
        init();
        let agent = DownloadAgent::new();
        let cancel_flag = Arc::new(AtomicUsize::new(0));
        let callback = Box::new(TestCallbackC {
            flag: cancel_flag.clone(),
        });
        let mut handle = agent
            .pre_download(TEST_URL.to_string(), callback, false)
            .unwrap();
        handle.cancel();
        std::thread::sleep(Duration::from_secs(1));
        assert_eq!(cancel_flag.load(Ordering::SeqCst), 1);

        let cancel_flag = Arc::new(AtomicUsize::new(0));
        let callback = Box::new(TestCallbackC {
            flag: cancel_flag.clone(),
        });
        agent
            .pre_download(TEST_URL.to_string(), callback, false)
            .unwrap();
        agent.cancel(TEST_URL.to_string());
        std::thread::sleep(Duration::from_secs(1));
        assert_eq!(cancel_flag.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn ut_pre_download_cancel_add_callback() {
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

        let mut handle_0 = agent
            .pre_download(TEST_URL.to_string(), callback_0, false)
            .unwrap();
        agent
            .pre_download(TEST_URL.to_string(), callback_1, false)
            .unwrap();
        handle_0.cancel();
        std::thread::sleep(Duration::from_secs(1));
        assert_eq!(cancel_flag_0.load(Ordering::SeqCst), 1);
        assert_eq!(cancel_flag_1.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn ut_pre_download_already_success() {
        init();
        let agent = DownloadAgent::new();
        agent.pre_download(TEST_URL.to_string(), Box::new(TestCallbackN), false);
        std::thread::sleep(Duration::from_secs(1));

        let success_flag = Arc::new(AtomicUsize::new(0));
        let callback = Box::new(TestCallbackS {
            flag: success_flag.clone(),
        });
        agent.pre_download(TEST_URL.to_string(), callback, false);
        std::thread::sleep(Duration::from_millis(500));
        assert_eq!(success_flag.load(Ordering::SeqCst), 1);
    }
}
