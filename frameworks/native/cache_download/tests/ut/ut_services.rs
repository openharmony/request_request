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
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use std::thread;
use std::time::Duration;

use request_utils::test::log::init;
use request_utils::test::server::test_server;

use super::*;
use crate::download::CANCEL;

const ERROR_IP: &str = "127.12.31.12";
const NO_DATA: usize = 1359;
const TEST_URL: &str = "http://www.baidu.com";

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
    fn on_fail(&mut self, error: CacheDownloadError, _task_id: &str) {
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

    let handle_0 =
        SERVICE.preload(DownloadRequest::new(test_url), callback_0, true, DOWNLOADER);
    let handle_1 =
        SERVICE.preload(DownloadRequest::new(test_url), callback_1, true, DOWNLOADER);
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
