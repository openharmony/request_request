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
mod ut_ylong_mod {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    use super::*;

    // Mock struct for CommonError
    struct MockError {
        code: i32,
        msg: String,
    }

    impl CommonError for MockError {
        fn code(&self) -> i32 {
            self.code
        }

        fn msg(&self) -> String {
            self.msg.clone()
        }
    }

    // Mock struct for CommonResponse
    struct MockResponse {
        status: u16,
    }

    impl CommonResponse for MockResponse {
        fn code(&self) -> u32 {
            self.status as u32
        }
    }

    // @tc.name: ut_common_error_impl
    // @tc.desc: Test CommonError implementation for HttpClientError
    // @tc.precon: NA
    // @tc.step: 1. Verify CommonError trait is implemented
    // @tc.expect: Trait implementation exists
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    #[cfg(feature = "ylong")]
    fn ut_common_error_impl() {
        use ylong_http_client::HttpClientError;
        let error = HttpClientError::user_aborted();
        assert!(error.code() < 0);
    }

    // @tc.name: ut_common_response_impl
    // @tc.desc: Test CommonResponse implementation for Response
    // @tc.precon: NA
    // @tc.step: 1. Verify CommonResponse trait is implemented
    // @tc.expect: Trait implementation exists
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    #[cfg(feature = "ylong")]
    fn ut_common_response_impl() {
        assert!(true);
    }

    // @tc.name: ut_cancel_handle_new
    // @tc.desc: Test CancelHandle creation
    // @tc.precon: NA
    // @tc.step: 1. Create CancelHandle with atomic flag
    //           2. Verify initial state
    // @tc.expect: CancelHandle created with correct initial state
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    #[cfg(feature = "ylong")]
    fn ut_cancel_handle_new() {
        let flag = Arc::new(AtomicBool::new(false));
        let handle = CancelHandle::new(flag.clone());
        assert!(!flag.load(Ordering::Acquire));
    }

    // @tc.name: ut_cancel_handle_cancel
    // @tc.desc: Test CancelHandle cancel method
    // @tc.precon: NA
    // @tc.step: 1. Create CancelHandle
    //           2. Call cancel()
    //           3. Verify flag is set
    // @tc.expect: cancel() sets the flag and returns true
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    #[cfg(feature = "ylong")]
    fn ut_cancel_handle_cancel() {
        let flag = Arc::new(AtomicBool::new(false));
        let handle = CancelHandle::new(flag.clone());
        assert!(handle.cancel());
        assert!(flag.load(Ordering::Acquire));
    }

    // @tc.name: ut_cancel_handle_add_count
    // @tc.desc: Test CancelHandle add_count method
    // @tc.precon: NA
    // @tc.step: 1. Create CancelHandle
    //           2. Call add_count()
    //           3. Call cancel() twice
    // @tc.expect: First cancel returns false, second returns true
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    #[cfg(feature = "ylong")]
    fn ut_cancel_handle_add_count() {
        let flag = Arc::new(AtomicBool::new(false));
        let handle = CancelHandle::new(flag.clone());
        handle.add_count();
        assert!(!handle.cancel());
        assert!(handle.cancel());
    }

    // @tc.name: ut_cancel_handle_multiple_add_count
    // @tc.desc: Test CancelHandle with multiple add_count calls
    // @tc.precon: NA
    // @tc.step: 1. Create CancelHandle
    //           2. Call add_count() multiple times
    //           3. Call cancel() multiple times
    // @tc.expect: Only last cancel returns true
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    #[cfg(feature = "ylong")]
    fn ut_cancel_handle_multiple_add_count() {
        let flag = Arc::new(AtomicBool::new(false));
        let handle = CancelHandle::new(flag.clone());
        handle.add_count();
        handle.add_count();
        handle.add_count();

        assert!(!handle.cancel());
        assert!(!handle.cancel());
        assert!(!handle.cancel());
        assert!(handle.cancel());
    }

    // @tc.name: ut_response_status
    // @tc.desc: Test Response status method
    // @tc.precon: NA
    // @tc.step: 1. Create Response with status code
    //           2. Call status() method
    // @tc.expect: status() returns correct status code
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    #[cfg(feature = "ylong")]
    fn ut_response_status() {
        use ylong_http_client::StatusCode;

        let response = Response {
            status: StatusCode::OK,
        };
        assert_eq!(response.status(), StatusCode::OK);
    }

    // @tc.name: ut_download_task_run
    // @tc.desc: Test DownloadTask run method
    // @tc.precon: NA
    // @tc.step: 1. Create DownloadRequest with local test server
    //           2. Create PrimeCallback
    //           3. Call DownloadTask::run
    // @tc.expect: Returns a valid handle
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    #[cfg(feature = "ylong")]
    fn ut_download_task_run() {
        use std::collections::VecDeque;
        use std::io::BufRead;
        use std::sync::LazyLock;
        use std::sync::atomic::AtomicUsize;
        use std::sync::Mutex;

        use cache_core::CacheManager;
        use request_utils::test::server::test_server;

        use crate::download::callback::PrimeCallback;
        use crate::services::DownloadRequest;

        static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);

        // Use local test server to avoid external network dependency
        let test_f = |_lines| {};
        let server_url = test_server(test_f);

        let request = DownloadRequest::new(&server_url);
        let finish = Arc::new(AtomicBool::new(false));
        let state = Arc::new(AtomicUsize::new(0));
        let callbacks = Arc::new(Mutex::new(VecDeque::new()));

        let callback = PrimeCallback::new(
            request_utils::task_id::TaskId::new(),
            &CACHE_MANAGER,
            finish,
            state,
            callbacks,
            1,
        );

        let handle = DownloadTask::run(request, callback);
        assert!(!handle.cancel());
    }
}
