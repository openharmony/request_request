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
mod ut_common {
    use super::*;

    // Mock struct implementing CommonResponse for testing
    struct MockResponse {
        status_code: u32,
    }

    impl CommonResponse for MockResponse {
        fn code(&self) -> u32 {
            self.status_code
        }
    }

    // Mock struct implementing CommonError for testing
    struct MockError {
        error_code: i32,
        error_msg: String,
    }

    impl CommonError for MockError {
        fn code(&self) -> i32 {
            self.error_code
        }

        fn msg(&self) -> String {
            self.error_msg.clone()
        }
    }

    // Mock struct implementing CommonHandle for testing
    struct MockHandle {
        cancelled: std::sync::atomic::AtomicBool,
        count: std::sync::atomic::AtomicUsize,
    }

    impl MockHandle {
        fn new() -> Self {
            Self {
                cancelled: std::sync::atomic::AtomicBool::new(false),
                count: std::sync::atomic::AtomicUsize::new(1),
            }
        }
    }

    impl CommonHandle for MockHandle {
        fn cancel(&self) -> bool {
            if self.count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst) == 1 {
                self.cancelled
                    .store(true, std::sync::atomic::Ordering::Release);
                true
            } else {
                false
            }
        }

        fn add_count(&self) {
            self.count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }

        #[cfg(feature = "netstack")]
        fn reset(&self) {
            self.cancelled
                .store(false, std::sync::atomic::Ordering::Release);
        }
    }

    // @tc.name: ut_common_response_code
    // @tc.desc: Test CommonResponse trait code method
    // @tc.precon: NA
    // @tc.step: 1. Create MockResponse with status code 200
    //           2. Call code() method
    //           3. Verify returned value
    // @tc.expect: code() returns the correct status code
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_common_response_code() {
        let response = MockResponse { status_code: 200 };
        assert_eq!(response.code(), 200);
    }

    // @tc.name: ut_common_response_code_404
    // @tc.desc: Test CommonResponse trait with 404 status code
    // @tc.precon: NA
    // @tc.step: 1. Create MockResponse with status code 404
    //           2. Call code() method
    //           3. Verify returned value
    // @tc.expect: code() returns 404
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_common_response_code_404() {
        let response = MockResponse { status_code: 404 };
        assert_eq!(response.code(), 404);
    }

    // @tc.name: ut_common_response_code_500
    // @tc.desc: Test CommonResponse trait with 500 status code
    // @tc.precon: NA
    // @tc.step: 1. Create MockResponse with status code 500
    //           2. Call code() method
    //           3. Verify returned value
    // @tc.expect: code() returns 500
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_common_response_code_500() {
        let response = MockResponse { status_code: 500 };
        assert_eq!(response.code(), 500);
    }

    // @tc.name: ut_common_error_code
    // @tc.desc: Test CommonError trait code method
    // @tc.precon: NA
    // @tc.step: 1. Create MockError with error code
    //           2. Call code() method
    //           3. Verify returned value
    // @tc.expect: code() returns the correct error code
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_common_error_code() {
        let error = MockError {
            error_code: -1,
            error_msg: "Test error".to_string(),
        };
        assert_eq!(error.code(), -1);
    }

    // @tc.name: ut_common_error_msg
    // @tc.desc: Test CommonError trait msg method
    // @tc.precon: NA
    // @tc.step: 1. Create MockError with message
    //           2. Call msg() method
    //           3. Verify returned value
    // @tc.expect: msg() returns the correct error message
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_common_error_msg() {
        let error = MockError {
            error_code: 404,
            error_msg: "Not Found".to_string(),
        };
        assert_eq!(error.msg(), "Not Found");
    }

    // @tc.name: ut_common_error_empty_msg
    // @tc.desc: Test CommonError trait with empty message
    // @tc.precon: NA
    // @tc.step: 1. Create MockError with empty message
    //           2. Call msg() method
    //           3. Verify returned value is empty
    // @tc.expect: msg() returns empty string
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_common_error_empty_msg() {
        let error = MockError {
            error_code: 0,
            error_msg: "".to_string(),
        };
        assert_eq!(error.msg(), "");
    }

    // @tc.name: ut_common_handle_cancel
    // @tc.desc: Test CommonHandle trait cancel method
    // @tc.precon: NA
    // @tc.step: 1. Create MockHandle
    //           2. Call cancel() method
    //           3. Verify cancellation was successful
    // @tc.expect: cancel() returns true and sets cancelled flag
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_common_handle_cancel() {
        let handle = MockHandle::new();
        assert!(handle.cancel());
    }

    // @tc.name: ut_common_handle_add_count
    // @tc.desc: Test CommonHandle trait add_count method
    // @tc.precon: NA
    // @tc.step: 1. Create MockHandle
    //           2. Call add_count() method
    //           3. Call cancel() method
    //           4. Verify cancel returns false due to count
    // @tc.expect: add_count increments reference count
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_common_handle_add_count() {
        let handle = MockHandle::new();
        handle.add_count();
        assert!(!handle.cancel());
        assert!(handle.cancel());
    }

    // @tc.name: ut_common_handle_multiple_cancels
    // @tc.desc: Test CommonHandle with multiple cancel calls
    // @tc.precon: NA
    // @tc.step: 1. Create MockHandle
    //           2. Call cancel() multiple times
    //           3. Verify only first cancel returns true
    // @tc.expect: Only first cancel returns true
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_common_handle_multiple_cancels() {
        let handle = MockHandle::new();
        assert!(handle.cancel());
        assert!(!handle.cancel());
        assert!(!handle.cancel());
    }

    // @tc.name: ut_common_handle_thread_safety
    // @tc.desc: Test CommonHandle thread safety
    // @tc.precon: NA
    // @tc.step: 1. Create MockHandle wrapped in Arc
    //           2. Spawn multiple threads to call add_count
    //           3. Call cancel() multiple times
    //           4. Verify only the last cancel returns true
    // @tc.expect: Handle operations are thread-safe
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_common_handle_thread_safety() {
        use std::sync::Arc;
        use std::thread;

        let handle = Arc::new(MockHandle::new());
        let mut handles = vec![];

        for _ in 0..4 {
            let h = handle.clone();
            handles.push(thread::spawn(move || {
                h.add_count();
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        // After 4 add_count calls, count = 1 + 4 = 5
        // First 4 cancels: count 5->4->3->2->1, all return false
        for _ in 0..4 {
            assert!(!handle.cancel());
        }
        // Fifth cancel: count 1->0, returns true
        assert!(handle.cancel());
    }
}
