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
) -> Option<Arc<(dyn CommonHandle + 'static)>> = netstack::DownloadTask::run;

#[cfg(not(feature = "ohos"))]
const DOWNLOADER: for<'a> fn(
    DownloadRequest<'a>,
    PrimeCallback,
) -> Arc<(dyn CommonHandle + 'static)> = ylong::DownloadTask::run;

// @tc.name: ut_preload
// @tc.desc: Test preload functionality
// @tc.precon: NA
// @tc.step: 1. Initialize CacheManager
//           2. Create download request with test URL
//           3. Call download_inner function
//           4. Wait for task completion
// @tc.expect: success_flag is set to true
// @tc.type: FUNC
// @tc.require: issue#ICN31I
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
    assert!(handle.is_some());
    let handle = handle.unwrap();
    while !handle.is_finish() {
        thread::sleep(Duration::from_millis(500));
    }
    assert!(success_flag.load(Ordering::Acquire));
}

// @tc.name: ut_download_headers
// @tc.desc: Test download headers are correctly sent
// @tc.precon: NA
// @tc.step: 1. Initialize CacheManager
//           2. Create test server to verify headers
//           3. Create download request with custom headers
//           4. Call download_inner function
//           5. Wait for task completion
// @tc.expect: All custom headers are received by server
// @tc.type: FUNC
// @tc.require: issue#ICN31I
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
    assert!(handle.is_some());
    let handle = handle.unwrap();
    while !handle.is_finish() {
        thread::sleep(Duration::from_millis(500));
    }
    assert!(flag.load(Ordering::SeqCst));
}
