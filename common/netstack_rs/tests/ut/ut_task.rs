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

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::Arc;

use super::*;
use crate::error::HttpClientError;
use crate::info::DownloadInfo;
use crate::wrapper::ffi::NewHttpClientRequest;
const TEST_URL: &str = "https://www.w3cschool.cn/statics/demosource/movie.mp4";
const LOCAL_URL: &str = "https://127.0.0.1";

// @tc.name: ut_task_from_http_request
// @tc.desc: Test creating RequestTask from HttpClientRequest
// @tc.precon: NA
// @tc.step: 1. Create a new HttpClientRequest instance
//           2. Set URL and method for the request
//           3. Call RequestTask::from_http_request
//           4. Verify task is created successfully and status is Idle
// @tc.expect: Task is not None and status is Idle
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
#[test]
fn ut_task_from_http_request() {
    let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
        NewHttpClientRequest();
    cxx::let_cxx_string!(url = TEST_URL);
    request.pin_mut().SetURL(&url);
    cxx::let_cxx_string!(method = "GET");
    request.pin_mut().SetMethod(&method);
    let opt_task = RequestTask::from_http_request(&request);
    assert!(opt_task.is_some());
    let mut task = opt_task.unwrap();
    assert!(matches!(task.status(), TaskStatus::Idle));
}

struct TestCallback {
    pub(crate) finished: Arc<AtomicBool>,
    pub(crate) response_code: Arc<AtomicU32>,
    pub(crate) error: Arc<AtomicU32>,
    pub(crate) result: Arc<AtomicU32>,
}

impl TestCallback {
    fn new(
        finished: Arc<AtomicBool>,
        response_code: Arc<AtomicU32>,
        error: Arc<AtomicU32>,
        result: Arc<AtomicU32>,
    ) -> Self {
        Self {
            finished,
            response_code,
            error,
            result,
        }
    }
}

impl RequestCallback for TestCallback {
    fn on_success(&mut self, response: Response) {
        self.response_code
            .store(response.status() as u32, Ordering::SeqCst);
        self.finished.store(true, Ordering::SeqCst);
    }

    fn on_fail(&mut self, error: HttpClientError, _info: DownloadInfo) {
        self.error
            .store(error.code().clone() as u32, Ordering::SeqCst);
        self.finished.store(true, Ordering::SeqCst);
    }

    fn on_cancel(&mut self) {
        self.error.store(123456, Ordering::SeqCst);
        self.finished.store(true, Ordering::SeqCst);
    }

    fn on_data_receive(&mut self, data: &[u8], _task: RequestTask) {
        self.result.fetch_add(data.len() as u32, Ordering::SeqCst);
    }
}

// @tc.name: ut_request_task_start_success
// @tc.desc: Test successful start and completion of request task
// @tc.precon: NA
// @tc.step: 1. Create request with valid URL
//           2. Create task and set callback
//           3. Start task and wait for completion
//           4. Verify response code, error code and received data length
// @tc.expect: Response code is 200, error code is 0, data length matches
// content-length header @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
#[test]
fn ut_request_task_start_success() {
    let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
        NewHttpClientRequest();
    cxx::let_cxx_string!(url = TEST_URL);
    request.pin_mut().SetURL(&url);
    cxx::let_cxx_string!(method = "GET");
    request.pin_mut().SetMethod(&method);
    let opt_task = RequestTask::from_http_request(&request);
    assert!(opt_task.is_some());
    let mut task = opt_task.unwrap();
    let finished = Arc::new(AtomicBool::new(false));
    let response_code = Arc::new(AtomicU32::new(0));
    let error = Arc::new(AtomicU32::new(0));
    let result = Arc::new(AtomicU32::new(0));
    let callback = Box::new(TestCallback::new(
        finished.clone(),
        response_code.clone(),
        error.clone(),
        result.clone(),
    ));
    let info_mgr = Arc::new(DownloadInfoMgr::new());
    task.set_callback(callback, info_mgr, TaskId::from_url(TEST_URL));
    task.start();
    while !finished.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    assert_eq!(response_code.load(Ordering::SeqCst), 200);
    assert_eq!(error.load(Ordering::SeqCst), 0);
    assert_eq!(
        result.load(Ordering::SeqCst),
        task.headers()
            .get("content-length")
            .unwrap()
            .parse()
            .unwrap()
    );
}

// @tc.name: ut_request_task_cancel
// @tc.desc: Test cancellation functionality of request task
// @tc.precon: NA
// @tc.step: 1. Create request with valid URL
//           2. Create task and set callback
//           3. Start task and immediately cancel
//           4. Verify cancellation is handled correctly
// @tc.expect: on_cancel is called with error code 123456
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level2
#[test]
fn ut_request_task_cancel() {
    let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
        NewHttpClientRequest();
    cxx::let_cxx_string!(url = TEST_URL);
    request.pin_mut().SetURL(&url);
    cxx::let_cxx_string!(method = "GET");
    request.pin_mut().SetMethod(&method);
    let opt_task = RequestTask::from_http_request(&request);
    assert!(opt_task.is_some());
    let mut task = opt_task.unwrap();
    let finished = Arc::new(AtomicBool::new(false));
    let response_code = Arc::new(AtomicU32::new(0));
    let error = Arc::new(AtomicU32::new(0));
    let result = Arc::new(AtomicU32::new(0));
    let callback = Box::new(TestCallback::new(
        finished.clone(),
        response_code.clone(),
        error.clone(),
        result.clone(),
    ));
    let info_mgr = Arc::new(DownloadInfoMgr::new());
    task.set_callback(callback, info_mgr, TaskId::from_url(TEST_URL));
    task.start();
    std::thread::sleep(std::time::Duration::from_millis(1));
    task.cancel();
    while !finished.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    assert_eq!(error.load(Ordering::SeqCst), 123456);
}

// @tc.name: ut_request_task_fail
// @tc.desc: Test request task failure handling with invalid local URL
// @tc.precon: NA
// @tc.step: 1. Create request with invalid local URL
//           2. Create task and set callback
//           3. Start task and wait for failure
//           4. Verify failure error code
// @tc.expect: on_fail is called with HttpCouldntConnect error code
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level2
#[test]
fn ut_request_task_fail() {
    let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
        NewHttpClientRequest();
    cxx::let_cxx_string!(url = LOCAL_URL);
    request.pin_mut().SetURL(&url);
    cxx::let_cxx_string!(method = "GET");
    request.pin_mut().SetMethod(&method);
    let opt_task = RequestTask::from_http_request(&request);
    assert!(opt_task.is_some());
    let mut task = opt_task.unwrap();
    let finished = Arc::new(AtomicBool::new(false));
    let response_code = Arc::new(AtomicU32::new(0));
    let error = Arc::new(AtomicU32::new(0));
    let result = Arc::new(AtomicU32::new(0));
    let callback = Box::new(TestCallback::new(
        finished.clone(),
        response_code.clone(),
        error.clone(),
        result.clone(),
    ));
    let info_mgr = Arc::new(DownloadInfoMgr::new());
    task.set_callback(callback, info_mgr, TaskId::from_url(LOCAL_URL));
    task.start();
    while !finished.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    assert_eq!(
        error.load(Ordering::SeqCst),
        crate::error::HttpErrorCode::HttpCouldntConnect as u32
    );
}

// @tc.name: ut_request_task_connect_timeout
// @tc.desc: Test connection timeout handling
// @tc.precon: NA
// @tc.step: 1. Create request with unreachable IP and short connect timeout
//           2. Create task and set callback
//           3. Start task and wait for timeout
//           4. Verify timeout error code
// @tc.expect: on_fail is called with HttpOperationTimedout error code
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level2
#[test]
fn ut_request_task_connect_timeout() {
    let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
        NewHttpClientRequest();
    cxx::let_cxx_string!(url = "222.222.222.222");
    request.pin_mut().SetURL(&url);
    cxx::let_cxx_string!(method = "GET");
    request.pin_mut().SetMethod(&method);
    request.pin_mut().SetConnectTimeout(1);
    let opt_task = RequestTask::from_http_request(&request);
    assert!(opt_task.is_some());
    let mut task = opt_task.unwrap();
    let finished = Arc::new(AtomicBool::new(false));
    let response_code = Arc::new(AtomicU32::new(0));
    let error = Arc::new(AtomicU32::new(0));
    let result = Arc::new(AtomicU32::new(0));
    let callback = Box::new(TestCallback::new(
        finished.clone(),
        response_code.clone(),
        error.clone(),
        result.clone(),
    ));
    let info_mgr = Arc::new(DownloadInfoMgr::new());
    task.set_callback(callback, info_mgr, TaskId::from_url("222.222.222.222"));
    task.start();
    while !finished.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    assert_eq!(
        error.load(Ordering::SeqCst),
        crate::error::HttpErrorCode::HttpOperationTimedout as u32
    );
}

// @tc.name: ut_request_task_timeout
// @tc.desc: Test request timeout handling
// @tc.precon: NA
// @tc.step: 1. Create request with valid URL and short timeout
//           2. Create task and set callback
//           3. Start task and wait for timeout
//           4. Verify timeout error code
// @tc.expect: on_fail is called with HttpOperationTimedout error code
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level2
#[test]
fn ut_request_task_timeout() {
    let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
        NewHttpClientRequest();
    cxx::let_cxx_string!(url = TEST_URL);
    request.pin_mut().SetURL(&url);
    cxx::let_cxx_string!(method = "GET");
    request.pin_mut().SetMethod(&method);
    request.pin_mut().SetTimeout(1);
    let opt_task = RequestTask::from_http_request(&request);
    assert!(opt_task.is_some());
    let mut task = opt_task.unwrap();
    let finished = Arc::new(AtomicBool::new(false));
    let response_code = Arc::new(AtomicU32::new(0));
    let error = Arc::new(AtomicU32::new(0));
    let result = Arc::new(AtomicU32::new(0));
    let callback = Box::new(TestCallback::new(
        finished.clone(),
        response_code.clone(),
        error.clone(),
        result.clone(),
    ));
    let info_mgr = Arc::new(DownloadInfoMgr::new());
    task.set_callback(callback, info_mgr, TaskId::from_url(TEST_URL));
    task.start();
    while !finished.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    assert_eq!(
        error.load(Ordering::SeqCst),
        crate::error::HttpErrorCode::HttpOperationTimedout as u32
    );
}

// @tc.name: ut_request_task_reset_range
// @tc.desc: Test task reset functionality with range support
// @tc.precon: NA
// @tc.step: 1. Create request with range-supported URL
//           2. Create task with custom callback
//           3. Start task, wait for data receive, then reset
//           4. Verify total received data matches expected length
// @tc.expect: Total received data length equals expected file size, no failure
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level3
#[test]
fn ut_request_task_reset_range() {
    const RANGE_TEST_URL:&str = "https://vd4.bdstatic.com/mda-pm7bte3t6fs50rsh/sc/cae_h264/1702057792414494257/mda-pm7bte3t6fs50rsh.mp4?v_from_s=bdapp-author-nanjing";
    const LENGTH: usize = 1984562;
    struct RestartTest {
        finished: Arc<AtomicBool>,
        data_receive: Arc<AtomicBool>,
        failed: Arc<AtomicBool>,
        total: Arc<AtomicUsize>,
    }
    impl RequestCallback for RestartTest {
        fn on_success(&mut self, _response: Response) {
            self.finished.store(true, Ordering::SeqCst);
        }

        fn on_fail(&mut self, _error: HttpClientError, _info: DownloadInfo) {
            self.finished.store(true, Ordering::SeqCst);
            self.failed.store(true, Ordering::SeqCst);
        }

        fn on_cancel(&mut self) {
            self.finished.store(true, Ordering::SeqCst);
            self.failed.store(true, Ordering::SeqCst);
        }

        fn on_data_receive(&mut self, data: &[u8], _task: RequestTask) {
            self.data_receive.store(true, Ordering::SeqCst);
            self.total.fetch_add(data.len(), Ordering::SeqCst);
        }
    }

    let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
        NewHttpClientRequest();
    cxx::let_cxx_string!(url = RANGE_TEST_URL);
    request.pin_mut().SetURL(&url);
    cxx::let_cxx_string!(method = "GET");
    request.pin_mut().SetMethod(&method);

    let opt_task = RequestTask::from_http_request(&request);
    assert!(opt_task.is_some());
    let mut task = opt_task.unwrap();
    let finished = Arc::new(AtomicBool::new(false));
    let total = Arc::new(AtomicUsize::new(0));
    let failed = Arc::new(AtomicBool::new(false));
    let data_receive = Arc::new(AtomicBool::new(false));

    let callback = Box::new(RestartTest {
        finished: finished.clone(),
        data_receive: data_receive.clone(),
        failed: failed.clone(),
        total: total.clone(),
    });
    let info_mgr = Arc::new(DownloadInfoMgr::new());
    task.set_callback(callback, info_mgr, TaskId::from_url(RANGE_TEST_URL));
    task.start();

    while !data_receive.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    task.reset();
    let part_size = total.load(Ordering::SeqCst);
    while !finished.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    assert_eq!(total.load(Ordering::SeqCst), LENGTH + part_size);
    assert!(!failed.load(Ordering::SeqCst));
}

// @tc.name: ut_request_task_reset_not_range
// @tc.desc: Test task reset functionality without range support
// @tc.precon: NA
// @tc.step: 1. Create request with non-range-supported URL
//           2. Create task with custom callback
//           3. Start task, wait for data receive, then reset
//           4. Verify total received data matches expected length
// @tc.expect: Total received data length equals expected file size, no failure
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level3
#[test]
fn ut_request_task_reset_not_range() {
    const NOT_SUPPORT_RANGE_TEST_URL: &str =
        "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
    const LENGTH: usize = 1042003;
    struct RestartTest {
        finished: Arc<AtomicBool>,
        data_receive: Arc<AtomicBool>,
        failed: Arc<AtomicBool>,
        total: Arc<AtomicUsize>,
    }
    impl RequestCallback for RestartTest {
        fn on_success(&mut self, _response: Response) {
            self.finished.store(true, Ordering::SeqCst);
        }

        fn on_fail(&mut self, _error: HttpClientError, _info: DownloadInfo) {
            self.finished.store(true, Ordering::SeqCst);
            self.failed.store(true, Ordering::SeqCst);
        }

        fn on_cancel(&mut self) {
            self.finished.store(true, Ordering::SeqCst);
            self.failed.store(true, Ordering::SeqCst);
        }

        fn on_data_receive(&mut self, data: &[u8], _task: RequestTask) {
            self.data_receive.store(true, Ordering::SeqCst);
            self.total.fetch_add(data.len(), Ordering::SeqCst);
        }

        fn on_restart(&mut self) {
            self.total.store(0, Ordering::SeqCst);
        }
    }

    let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
        NewHttpClientRequest();
    cxx::let_cxx_string!(url = NOT_SUPPORT_RANGE_TEST_URL);
    request.pin_mut().SetURL(&url);
    cxx::let_cxx_string!(method = "GET");
    request.pin_mut().SetMethod(&method);

    let opt_task = RequestTask::from_http_request(&request);
    assert!(opt_task.is_some());
    let mut task = opt_task.unwrap();
    let finished = Arc::new(AtomicBool::new(false));
    let total = Arc::new(AtomicUsize::new(0));
    let failed = Arc::new(AtomicBool::new(false));
    let data_receive = Arc::new(AtomicBool::new(false));

    let callback = Box::new(RestartTest {
        finished: finished.clone(),
        data_receive: data_receive.clone(),
        failed: failed.clone(),
        total: total.clone(),
    });
    let info_mgr = Arc::new(DownloadInfoMgr::new());
    task.set_callback(
        callback,
        info_mgr,
        TaskId::from_url(NOT_SUPPORT_RANGE_TEST_URL),
    );
    task.start();

    while !data_receive.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
    task.reset();
    while !finished.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    assert_eq!(total.load(Ordering::SeqCst), LENGTH);
    assert!(!failed.load(Ordering::SeqCst));
}
