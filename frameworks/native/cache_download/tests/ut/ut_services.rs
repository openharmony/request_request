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

use std::collections::HashSet;
use std::io::{BufReader, Lines};
use std::net::TcpStream;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use std::thread;
use std::time::Duration;

use request_utils;
use request_utils::test::log::init;
use request_utils::test::server::test_server;

use super::*;
use crate::download::CANCEL;

const ERROR_IP: &str = "127.12.31.12";
const NO_DATA: usize = 1359;
const TEST_URL: &str = "http://www.baidu.com";
const TEST_VIDEO_URL: &str = "https://www.w3cschool.cn/statics/demosource/movie.mp4";
static TEST_TEXT_URL: Mutex<&'static str> = Mutex::new(
    "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt",
);
const FINISH_SUFFIX: &str = "_F";

#[cfg(feature = "ohos")]
const DOWNLOADER: Downloader = Downloader::Netstack;

#[cfg(not(feature = "ohos"))]
const DOWNLOADER: Downloader = Downloader::Ylong;

struct TestCallbackN;
impl PreloadCallback for TestCallbackN {}

struct TestCallbackS {
    flag: Arc<AtomicUsize>,
}

impl PreloadCallback for TestCallbackS {
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

impl PreloadCallback for TestCallbackF {
    fn on_fail(&mut self, error: CacheDownloadError, _info: RustDownloadInfo, _task_id: &str) {
        *self.flag.lock().unwrap() = error.message().to_string();
    }
}

struct TestCallbackC {
    flag: Arc<AtomicUsize>,
}

impl PreloadCallback for TestCallbackC {
    fn on_cancel(&mut self) {
        self.flag.fetch_add(1, Ordering::SeqCst);
    }
}

// @tc.name: ut_preload_success
// @tc.desc: Test successful preload operation
// @tc.precon: NA
// @tc.step: 1. Initialize CacheDownloadService
//           2. Create success callback with flag
//           3. Call preload with valid URL
//           4. Wait for task completion
// @tc.expect: Callback flag is set to 1 indicating success
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
#[test]
fn ut_preload_success() {
    init();
    static SERVICE: LazyLock<CacheDownloadService> = LazyLock::new(CacheDownloadService::new);
    let success_flag = Arc::new(AtomicUsize::new(0));
    let callback = Box::new(TestCallbackS {
        flag: success_flag.clone(),
    });
    let handle = SERVICE.preload(DownloadRequest::new(TEST_URL), callback, true, DOWNLOADER);
    assert!(handle.is_some());
    let handle = handle.unwrap();
    while !handle.is_finish() {
        thread::sleep(Duration::from_millis(500));
    }
    assert_eq!(success_flag.load(Ordering::SeqCst), 1);
}

// @tc.name: ut_preload_success_add_callback
// @tc.desc: Test adding multiple callbacks to successful preload
// @tc.precon: NA
// @tc.step: 1. Initialize CacheDownloadService
//           2. Create two success callbacks
//           3. Call preload twice with same URL
//           4. Wait for task completion
// @tc.expect: Both callback flags are set to 1
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
#[test]
fn ut_preload_success_add_callback() {
    init();
    static SERVICE: LazyLock<CacheDownloadService> = LazyLock::new(CacheDownloadService::new);
    let success_flag_0 = Arc::new(AtomicUsize::new(0));
    let callback_0 = Box::new(TestCallbackS {
        flag: success_flag_0.clone(),
    });

    let success_flag_1 = Arc::new(AtomicUsize::new(0));
    let callback_1 = Box::new(TestCallbackS {
        flag: success_flag_1.clone(),
    });

    let handle = SERVICE.preload(DownloadRequest::new(TEST_URL), callback_0, true, DOWNLOADER);
    SERVICE.preload(DownloadRequest::new(TEST_URL), callback_1, true, DOWNLOADER);
    assert!(handle.is_some());
    let handle = handle.unwrap();
    while !handle.is_finish() {
        thread::sleep(Duration::from_millis(500));
    }
    assert_eq!(success_flag_0.load(Ordering::SeqCst), 1);
    assert_eq!(success_flag_1.load(Ordering::SeqCst), 1);
}

// @tc.name: ut_preload_fail
// @tc.desc: Test preload failure with invalid URL
// @tc.precon: NA
// @tc.step: 1. Initialize CacheDownloadService
//           2. Create failure callback
//           3. Call preload with invalid URL
//           4. Wait for task completion
// @tc.expect: Error message is captured in callback
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level2
#[test]
fn ut_preload_fail() {
    init();
    static SERVICE: LazyLock<CacheDownloadService> = LazyLock::new(CacheDownloadService::new);
    let error = Arc::new(Mutex::new(String::new()));
    let callback = Box::new(TestCallbackF {
        flag: error.clone(),
    });
    let handle = SERVICE.preload(DownloadRequest::new(ERROR_IP), callback, true, DOWNLOADER);
    assert!(handle.is_some());
    let handle = handle.unwrap();
    while !handle.is_finish() {
        thread::sleep(Duration::from_millis(500));
    }
    assert!(!error.lock().unwrap().as_str().is_empty());
}

// @tc.name: ut_preload_fail_add_callback
// @tc.desc: Test adding multiple callbacks to failed preload
// @tc.precon: NA
// @tc.step: 1. Initialize CacheDownloadService
//           2. Create two failure callbacks
//           3. Call preload twice with invalid URL
//           4. Wait for task completion
// @tc.expect: Both callbacks capture error messages
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level2
#[test]
fn ut_preload_fail_add_callback() {
    init();
    static SERVICE: LazyLock<CacheDownloadService> = LazyLock::new(CacheDownloadService::new);
    let error_0 = Arc::new(Mutex::new(String::new()));
    let callback_0 = Box::new(TestCallbackF {
        flag: error_0.clone(),
    });
    let error_1 = Arc::new(Mutex::new(String::new()));
    let callback_1 = Box::new(TestCallbackF {
        flag: error_1.clone(),
    });

    let handle = SERVICE.preload(DownloadRequest::new(ERROR_IP), callback_0, true, DOWNLOADER);
    assert!(handle.is_some());
    let handle = handle.unwrap();
    SERVICE.preload(DownloadRequest::new(ERROR_IP), callback_1, true, DOWNLOADER);
    while !handle.is_finish() {
        thread::sleep(Duration::from_millis(500));
    }

    assert!(!error_0.lock().unwrap().as_str().is_empty());
    assert!(!error_1.lock().unwrap().as_str().is_empty());
}

// @tc.name: ut_preload_cancel_0
// @tc.desc: Test preload cancellation through TaskHandle
// @tc.precon: NA
// @tc.step: 1. Initialize CacheDownloadService
//           2. Create cancellation callback
//           3. Call preload and get handle
//           4. Cancel task through handle
// @tc.expect: Cancellation flag is set to 1
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level2
#[test]
fn ut_preload_cancel_0() {
    init();
    static SERVICE: LazyLock<CacheDownloadService> = LazyLock::new(CacheDownloadService::new);
    let cancel_flag = Arc::new(AtomicUsize::new(0));
    let callback = Box::new(TestCallbackC {
        flag: cancel_flag.clone(),
    });
    let handle = SERVICE.preload(DownloadRequest::new(TEST_URL), callback, true, DOWNLOADER);
    assert!(handle.is_some());
    let mut handle = handle.unwrap();
    handle.cancel();
    while handle.state() != CANCEL {
        std::thread::sleep(Duration::from_millis(500));
    }

    assert_eq!(cancel_flag.load(Ordering::SeqCst), 1);
}

// @tc.name: ut_preload_cancel_1
// @tc.desc: Test preload cancellation through service method
// @tc.precon: NA
// @tc.step: 1. Initialize CacheDownloadService
//           2. Create cancellation callback
//           3. Call preload and then cancel through service
// @tc.expect: Cancellation flag is set to 1
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level2
#[test]
fn ut_preload_cancel_1() {
    init();
    static SERVICE: LazyLock<CacheDownloadService> = LazyLock::new(CacheDownloadService::new);
    let cancel_flag = Arc::new(AtomicUsize::new(0));
    let callback = Box::new(TestCallbackC {
        flag: cancel_flag.clone(),
    });
    let handle = SERVICE.preload(DownloadRequest::new(TEST_URL), callback, true, DOWNLOADER);
    SERVICE.cancel(TEST_URL);
    assert!(handle.is_some());
    let handle = handle.unwrap();
    while handle.state() != CANCEL {
        std::thread::sleep(Duration::from_millis(500));
    }
    assert_eq!(cancel_flag.load(Ordering::SeqCst), 1);
}

// @tc.name: ut_preload_cancel_add_callback
// @tc.desc: Test cancellation with multiple callbacks
// @tc.precon: NA
// @tc.step: 1. Initialize CacheDownloadService
//           2. Create two cancellation callbacks
//           3. Call preload twice with same URL
//           4. Cancel both tasks
// @tc.expect: Both cancellation flags are set to 1
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level3
#[test]
fn ut_preload_cancel_add_callback() {
    init();
    let test_url = "https://www.gitee.com";

    static SERVICE: LazyLock<CacheDownloadService> = LazyLock::new(CacheDownloadService::new);
    let cancel_flag_0 = Arc::new(AtomicUsize::new(0));
    let callback_0 = Box::new(TestCallbackC {
        flag: cancel_flag_0.clone(),
    });
    let cancel_flag_1 = Arc::new(AtomicUsize::new(0));
    let callback_1 = Box::new(TestCallbackC {
        flag: cancel_flag_1.clone(),
    });

    let handle_0 = SERVICE.preload(DownloadRequest::new(test_url), callback_0, true, DOWNLOADER);
    let handle_1 = SERVICE.preload(DownloadRequest::new(test_url), callback_1, true, DOWNLOADER);
    assert!(handle_0.is_some());
    assert!(handle_1.is_some());
    let mut handle_0 = handle_0.unwrap();
    let mut handle_1 = handle_1.unwrap();
    handle_0.cancel();
    assert_eq!(cancel_flag_0.load(Ordering::SeqCst), 0);
    assert_eq!(cancel_flag_1.load(Ordering::SeqCst), 0);
    handle_1.cancel();
    assert!(handle_0.is_finish());
    assert!(handle_1.is_finish());

    while handle_1.state() != CANCEL {
        std::thread::sleep(Duration::from_millis(500));
    }
    assert_eq!(cancel_flag_0.load(Ordering::SeqCst), 1);
    assert_eq!(cancel_flag_1.load(Ordering::SeqCst), 1);
}

// @tc.name: ut_preload_already_success
// @tc.desc: Test preload with existing cache
// @tc.precon: NA
// @tc.step: 1. Initialize CacheDownloadService
//           2. Complete preload once to populate cache
//           3. Call preload again with update=false
// @tc.expect: Success callback triggers immediately
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
#[test]
fn ut_preload_already_success() {
    init();
    static SERVICE: LazyLock<CacheDownloadService> = LazyLock::new(CacheDownloadService::new);
    let handle = SERVICE.preload(
        DownloadRequest::new(TEST_URL),
        Box::new(TestCallbackN),
        true,
        DOWNLOADER,
    );
    assert!(handle.is_some());
    let handle = handle.unwrap();
    while !handle.is_finish() {
        thread::sleep(Duration::from_millis(500));
    }
    let success_flag = Arc::new(AtomicUsize::new(0));
    let callback = Box::new(TestCallbackS {
        flag: success_flag.clone(),
    });
    SERVICE.preload(DownloadRequest::new(TEST_URL), callback, false, DOWNLOADER);
    std::thread::sleep(Duration::from_millis(50));
    assert_eq!(success_flag.load(Ordering::SeqCst), 1);
}

// @tc.name: ut_preload_local_headers
// @tc.desc: Test preload with custom headers
// @tc.precon: NA
// @tc.step: 1. Initialize CacheDownloadService
//           2. Create test server with header validation
//           3. Call preload with custom headers
//           4. Verify headers received by server
// @tc.expect: All headers are correctly transmitted
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
#[test]
fn ut_preload_local_headers() {
    init();
    static SERVICE: LazyLock<CacheDownloadService> = LazyLock::new(CacheDownloadService::new);

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

    let flag = Arc::new(AtomicBool::new(true));
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
    let handle = SERVICE.preload(request, callback, true, DOWNLOADER);
    assert!(handle.is_some());
    let handle = handle.unwrap();
    while !handle.is_finish() {
        thread::sleep(Duration::from_millis(500));
    }
    assert!(flag.load(Ordering::SeqCst));
    assert_eq!(success_flag.load(Ordering::SeqCst), NO_DATA);
}

// @tc.name: ut_preload_fetch
// @tc.desc: Test fetching cached data after preload
// @tc.precon: NA
// @tc.step: 1. Initialize CacheDownloadService
//           2. Complete preload to populate cache
//           3. Call fetch method with same URL
// @tc.expect: Cached data is returned successfully
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
#[test]
fn ut_preload_fetch() {
    init();
    static SERVICE: LazyLock<CacheDownloadService> = LazyLock::new(CacheDownloadService::new);
    let success_flag = Arc::new(AtomicUsize::new(0));
    let callback = Box::new(TestCallbackS {
        flag: success_flag.clone(),
    });
    let handle = SERVICE.preload(DownloadRequest::new(TEST_URL), callback, true, DOWNLOADER);
    assert!(handle.is_some());
    let handle = handle.unwrap();
    while !handle.is_finish() {
        thread::sleep(Duration::from_millis(500));
    }
    let cache = SERVICE.fetch(TEST_URL);
    assert!(cache.is_some());
    assert_eq!(success_flag.load(Ordering::SeqCst), 1);
}

// @tc.name: ut_download_request_ssl_type
// @tc.desc: Test DownloadRequest set ssl_type
// @tc.precon: NA
// @tc.step: 1. Create a DownloadRequest object.
//           2. Call the ssl_type function to set the ssl_type
//           3. Check whether ssl_type is set
// @tc.expect: The ssl_type is set successfully
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
#[test]
fn ut_download_request_ssl_type() {
    let mut request = DownloadRequest::new(TEST_URL);
    request.ssl_type("TLS");
    assert_eq!(request.ssl_type, Some("TLS"));
}

// @tc.name: ut_remove_file_cache
// @tc.desc: Test removing file cache
// @tc.precon: NA
// @tc.step: 1. Preload a file to create cache
//           2. Verify cache exists and cache file exists
//           3. Call remove method to remove cache
//           4. Verify cache is still available and cache file does not exist
// @tc.expect: Cache file is removed from service but cache is still available
// @tc.type: FUNC
// @tc.require: issue#1643
// @tc.level: level1
#[test]
fn ut_remove_file_cache() {
    let test_url = TEST_TEXT_URL.lock().unwrap();
    CacheDownloadService::get_instance().remove(test_url.as_ref());
    let success_flag = Arc::new(AtomicUsize::new(0));
    let callback = Box::new(TestCallbackS {
        flag: success_flag.clone(),
    });
    let handle = CacheDownloadService::get_instance().preload(
        DownloadRequest::new(test_url.as_ref()),
        callback,
        true,
        DOWNLOADER,
    );
    assert!(handle.is_some());
    let handle = handle.unwrap();
    while !handle.is_finish() {
        thread::sleep(Duration::from_millis(500));
    }
    let cache = CacheDownloadService::get_instance().fetch(test_url.as_ref());
    assert!(cache.is_some());
    let path = get_curr_store_dir();
    let task_id = handle.task_id();
    let file_name = format!("{}{}", task_id, FINISH_SUFFIX);
    let file_path = path.join(file_name);
    assert!(file_path.exists());
    CacheDownloadService::get_instance().clear_file_cache();
    assert!(!file_path.exists());
    let cache = CacheDownloadService::get_instance().fetch(test_url.as_ref());
    assert!(cache.is_some());
}

// @tc.name: ut_remove_ram_cache
// @tc.desc: Test removing RAM cache
// @tc.precon: NA
// @tc.step: 1. Preload a file to create cache
//           2. Verify cache exists and cache file exists
//           3. Call clear_memory_cache method to remove memory cache
//           4. Verify cache is still available
//           5. Call clear_file_cache and clear_memory_cache to remove all caches
//           6. Verify cache is removed
// @tc.expect: Cache is removed after clear_memory_cache and clear_file_cache
// @tc.type: FUNC
// @tc.require: issue#1643
// @tc.level: level1
#[test]
fn ut_remove_ram_cache() {
    let test_url = TEST_TEXT_URL.lock().unwrap();
    CacheDownloadService::get_instance().remove(test_url.as_ref());
    let success_flag = Arc::new(AtomicUsize::new(0));
    let callback = Box::new(TestCallbackS {
        flag: success_flag.clone(),
    });
    let handle = CacheDownloadService::get_instance().preload(
        DownloadRequest::new(test_url.as_ref()),
        callback,
        true,
        DOWNLOADER,
    );
    assert!(handle.is_some());
    let handle = handle.unwrap();
    while !handle.is_finish() {
        thread::sleep(Duration::from_millis(500));
    }
    let cache = CacheDownloadService::get_instance().fetch(test_url.as_ref());
    assert!(cache.is_some());
    CacheDownloadService::get_instance().clear_memory_cache();
    let cache = CacheDownloadService::get_instance().fetch(test_url.as_ref());
    assert!(cache.is_some());
    CacheDownloadService::get_instance().clear_memory_cache();
    CacheDownloadService::get_instance().clear_file_cache();
    let cache = CacheDownloadService::get_instance().fetch(test_url.as_ref());
    assert!(cache.is_none());
}

// @tc.name: ut_remove_finished_caches
// @tc.desc: Test removing finished task caches and keep running task caches
// @tc.precon: NA
// @tc.step: 1. Preload a file to create cache
//           2. Wait for preload to finish
//           3. Preload another file but do not wait for it to finish
//           4. Call remove method to remove finished caches
//           5. Verify cache is still available for running task
// @tc.expect: Cache is removed for finished task but still available for running task
// @tc.type: FUNC
// @tc.require: issue#1643
// @tc.level: level1
#[test]
fn ut_remove_finished_caches() {
    let test_url = TEST_TEXT_URL.lock().unwrap();
    CacheDownloadService::get_instance().remove(test_url.as_ref());
    CacheDownloadService::get_instance().remove(TEST_VIDEO_URL);
    let success_flag = Arc::new(AtomicUsize::new(0));
    let callback = Box::new(TestCallbackS {
        flag: success_flag.clone(),
    });
    let handle = CacheDownloadService::get_instance().preload(
        DownloadRequest::new(test_url.as_ref()),
        callback,
        true,
        DOWNLOADER,
    );
    assert!(handle.is_some());
    let handle = handle.unwrap();
    while !handle.is_finish() {
        thread::sleep(Duration::from_millis(500));
    }
    let success_flag2 = Arc::new(AtomicUsize::new(0));
    let callback2 = Box::new(TestCallbackS {
        flag: success_flag2.clone(),
    });
    let handle2 = CacheDownloadService::get_instance().preload(
        DownloadRequest::new(TEST_VIDEO_URL),
        callback2,
        true,
        DOWNLOADER,
    );
    CacheDownloadService::get_instance().clear_memory_cache();
    CacheDownloadService::get_instance().clear_file_cache();
    assert!(handle2.is_some());
    let handle2 = handle2.unwrap();
    while !handle2.is_finish() {
        thread::sleep(Duration::from_millis(500));
    }
    let cache = CacheDownloadService::get_instance().fetch(test_url.as_ref());
    assert!(cache.is_none());
    let cache = CacheDownloadService::get_instance().fetch(TEST_VIDEO_URL);
    assert!(cache.is_some());
}

pub fn get_curr_store_dir() -> PathBuf {
    let mut path = match request_utils::context::get_cache_dir() {
        Some(dir) => PathBuf::from_str(&dir).unwrap(),
        None => {
            error!("get cache dir failed");
            // Fallback to standard cache directory if context retrieval fails
            PathBuf::from_str("/data/storage/el2/base/cache").unwrap()
        }
    };
    path.push("preload_caches");
    path
}
