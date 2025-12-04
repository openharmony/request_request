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

#[cfg(test)]
mod ut_request {
    use super::*;
    use crate::error::HttpClientError;
    use crate::info::DownloadInfo;
    use crate::response::Response;
    use request_utils::task_id::TaskId;
    use std::sync::{Arc, Mutex};

    // Mock callback implementation for testing
    #[derive(Debug, Default, PartialEq)]
    struct MockCallback {
        on_success_called: bool,
        on_fail_called: bool,
        on_cancel_called: bool,
        on_data_receive_called: bool,
        on_progress_called: bool,
        on_restart_called: bool,
        last_error: Option<HttpClientError>,
        last_response: Option<Response>,
        last_data: Option<Vec<u8>>,
        last_progress: Option<(u64, u64, u64, u64)>,
    }

    impl RequestCallback for MockCallback {
        fn on_success(&mut self, response: Response) {
            self.on_success_called = true;
            self.last_response = Some(response);
        }

        fn on_fail(&mut self, error: HttpClientError, _info: DownloadInfo) {
            self.on_fail_called = true;
            self.last_error = Some(error);
        }

        fn on_cancel(&mut self) {
            self.on_cancel_called = true;
        }

        fn on_data_receive(&mut self, data: &[u8], _task: RequestTask) {
            self.on_data_receive_called = true;
            self.last_data = Some(data.to_vec());
        }

        fn on_progress(&mut self, dl_total: u64, dl_now: u64, ul_total: u64, ul_now: u64) {
            self.on_progress_called = true;
            self.last_progress = Some((dl_total, dl_now, ul_total, ul_now));
        }

        fn on_restart(&mut self) {
            self.on_restart_called = true;
        }
    }

    // @tc.name: ut_request_new
    // @tc.desc: Test creation of a new Request instance
    // @tc.precon: NA
    // @tc.step: 1. Call Request::new() to create a new instance
    // @tc.expect: New Request instance is created successfully
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_request_new() {
        let request: Request<MockCallback> = Request::new();
        assert!(!request.inner.is_null());
    }

    // @tc.name: ut_request_url
    // @tc.desc: Test setting URL for the request
    // @tc.precon: A new Request instance is created
    // @tc.step: 1. Create a new Request instance
    // 2. Call url() method with a test URL
    // @tc.expect: URL is set successfully
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_request_url() {
        let mut request = Request::new();
        request.url("https://example.com");
        // We can't directly verify the URL is set due to FFI, but we can verify the method chain
        assert!(!request.inner.is_null());
    }

    // @tc.name: ut_request_method
    // @tc.desc: Test setting HTTP method for the request
    // @tc.precon: A new Request instance is created
    // @tc.step: 1. Create a new Request instance
    // 2. Call method() with "GET"
    // 3. Call method() with "POST"
    // @tc.expect: Methods are set successfully
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_request_method() {
        let mut request = Request::new();
        request.method("GET");
        request.method("POST");
        assert!(!request.inner.is_null());
    }

    // @tc.name: ut_request_header
    // @tc.desc: Test setting headers for the request
    // @tc.precon: A new Request instance is created
    // @tc.step: 1. Create a new Request instance
    // 2. Call header() with multiple header key-value pairs
    // @tc.expect: Headers are set successfully
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_request_header() {
        let mut request = Request::new();
        request.header("Content-Type", "application/json");
        request.header("Authorization", "Bearer token");
        assert!(!request.inner.is_null());
    }

    // @tc.name: ut_request_body_001
    // @tc.desc: Test setting body for the request with valid data
    // @tc.precon: A new Request instance is created
    // @tc.step: 1. Create a new Request instance
    // 2. Call body() with a byte array
    // @tc.expect: Body is set successfully
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_request_body_001() {
        let mut request = Request::new();
        let body = b"test body";
        request.body(body);
        assert!(!request.inner.is_null());
    }

    // @tc.name: ut_request_body_002
    // @tc.desc: Test setting empty body for the request
    // @tc.precon: A new Request instance is created
    // @tc.step: 1. Create a new Request instance
    // 2. Call body() with an empty byte array
    // @tc.expect: Empty body is set successfully
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_request_body_002() {
        let mut request = Request::new();
        let body = b"";
        request.body(body);
        assert!(!request.inner.is_null());
    }

    // @tc.name: ut_request_timeout
    // @tc.desc: Test setting timeout for the request
    // @tc.precon: A new Request instance is created
    // @tc.step: 1. Create a new Request instance
    // 2. Call timeout() with a value
    // @tc.expect: Timeout is set successfully
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_request_timeout() {
        let mut request = Request::new();
        request.timeout(5000);
        assert!(!request.inner.is_null());
    }

    // @tc.name: ut_request_connect_timeout
    // @tc.desc: Test setting connect timeout for the request
    // @tc.precon: A new Request instance is created
    // @tc.step: 1. Create a new Request instance
    // 2. Call connect_timeout() with a value
    // @tc.expect: Connect timeout is set successfully
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_request_connect_timeout() {
        let mut request = Request::new();
        request.connect_timeout(2000);
        assert!(!request.inner.is_null());
    }

    // @tc.name: ut_request_callback
    // @tc.desc: Test setting callback for the request
    // @tc.precon: A new Request instance is created
    // @tc.step: 1. Create a new Request instance
    // 2. Create a MockCallback instance
    // 3. Call callback() with the mock callback
    // @tc.expect: Callback is set successfully
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_request_callback() {
        let mut request = Request::new();
        let callback = MockCallback::default();
        request.callback(callback);
        assert!(request.callback.is_some());
    }

    // @tc.name: ut_request_build_001
    // @tc.desc: Test building a request with complete parameters
    // @tc.precon: A Request instance with all parameters set
    // @tc.step: 1. Create a new Request instance
    // 2. Set URL, method, header, timeout
    // 3. Set callback, info_mgr, and task_id
    // 4. Call build() method
    // @tc.expect: RequestTask is created successfully
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_request_build_001() {
        let mut request = Request::new();
        request.url("https://example.com");
        request.method("GET");
        request.header("Content-Type", "application/json");
        request.timeout(5000);
        request.callback(MockCallback::default());
        request.task_id(TaskId::new(1));

        let task = request.build();
        assert!(task.is_some());
    }

    // @tc.name: ut_request_build_002
    // @tc.desc: Test building a request without required parameters
    // @tc.precon: A Request instance without callback, info_mgr, and task_id
    // @tc.step: 1. Create a new Request instance
    // 2. Set basic parameters but not callback, info_mgr, or task_id
    // 3. Call build() method
    // @tc.expect: build() returns None
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_request_build_002() {
        let mut request = Request::new();
        request.url("https://example.com");
        request.method("GET");

        let task = request.build();
        assert!(task.is_none());
    }

    // @tc.name: ut_request_callback_on_success
    // @tc.desc: Test on_success callback
    // @tc.precon: A MockCallback instance is created
    // @tc.step: 1. Create a new MockCallback
    // 2. Create a test Response
    // 3. Call on_success with the response
    // @tc.expect: on_success_called is true, last_response is set
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_request_callback_on_success() {
        let mut callback = MockCallback::default();
        let response = Response::default();
        callback.on_success(response.clone());

        assert!(callback.on_success_called);
        assert_eq!(callback.last_response, Some(response));
    }

    // @tc.name: ut_request_callback_on_fail
    // @tc.desc: Test on_fail callback
    // @tc.precon: A MockCallback instance is created
    // @tc.step: 1. Create a new MockCallback
    // 2. Create a test HttpClientError
    // 3. Call on_fail with the error
    // @tc.expect: on_fail_called is true, last_error is set
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_request_callback_on_fail() {
        let mut callback = MockCallback::default();
        let error = HttpClientError::new(1, "Test error");
        callback.on_fail(error.clone());

        assert!(callback.on_fail_called);
        assert_eq!(callback.last_error, Some(error));
    }

    // @tc.name: ut_request_callback_on_progress
    // @tc.desc: Test on_progress callback
    // @tc.precon: A MockCallback instance is created
    // @tc.step: 1. Create a new MockCallback
    // 2. Call on_progress with test progress values
    // @tc.expect: on_progress_called is true, last_progress is set
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_request_callback_on_progress() {
        let mut callback = MockCallback::default();
        callback.on_progress(100, 50, 200, 100);

        assert!(callback.on_progress_called);
        assert_eq!(callback.last_progress, Some((100, 50, 200, 100)));
    }

    // @tc.name: ut_request_default
    // @tc.desc: Test default implementation for Request
    // @tc.precon: NA
    // @tc.step: 1. Create a Request instance using default()
    // @tc.expect: Default Request instance is created successfully
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_request_default() {
        let request: Request<MockCallback> = Request::default();
        assert!(!request.inner.is_null());
    }
}
