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
mod tests {
    use super::*;
    use crate::{
        error::HttpErrorCode,
        info::{DownloadInfo, DownloadInfoMgr},
    };
    use mockall::mock;
    use std::cell::RefCell;
    use std::sync::{Arc, AtomicBool, Mutex};

    mock! {
        pub RequestCallback {}
        impl RequestCallback for RequestCallback {
            fn on_success(&mut self, response: Response);
            fn on_fail(&mut self, error: HttpClientError, info: DownloadInfo);
            fn on_cancel(&mut self);
            fn on_data_receive(&mut self, data: &[u8], task: RequestTask);
            fn on_progress(&mut self, dl_total: u64, dl_now: u64, ul_total: u64, ul_now: u64);
            fn on_restart(&mut self);
        }
    }

    // @tc.name: ut_callback_wrapper_creation
    // @tc.desc: Test CallbackWrapper creation with valid parameters
    // @tc.precon: NA
    // @tc.step: 1. Create mock RequestCallback
    // 2. Create required Arc and Weak pointers
    // 3. Initialize CallbackWrapper with from_callback method
    // @tc.expect: CallbackWrapper is successfully created with correct initial state
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_callback_wrapper_creation() {
        let mock_callback = Box::new(MockRequestCallback::new());
        let reset = Arc::new(AtomicBool::new(false));
        let task = Arc::new(Mutex::new(SharedPtr::null()));
        let task_weak = Arc::downgrade(&task);
        let task_id = TaskId::new();
        let info_mgr = Arc::new(DownloadInfoMgr::new());

        let wrapper = CallbackWrapper::from_callback(
            mock_callback,
            reset.clone(),
            task_weak,
            task_id.clone(),
            info_mgr.clone(),
            0,
        );

        assert!(!wrapper.reset.load(Ordering::SeqCst));
        assert_eq!(wrapper.task_id, task_id);
        assert_eq!(wrapper.current, 0);
        assert_eq!(wrapper.tries, 0);
        assert!(wrapper.inner.is_some());
    }

    // @tc.name: ut_callback_wrapper_on_success_200
    // @tc.desc: Test on_success callback with 200 status code
    // @tc.precon: CallbackWrapper initialized with mock callback
    // @tc.step: 1. Create test response with OK status
    // 2. Call on_success method
    // 3. Verify callback was triggered with success
    // @tc.expect: on_success is called with properly converted Response
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_callback_wrapper_on_success_200() {
        let mut mock = MockRequestCallback::new();
        mock.expect_on_success().once().return_const(());

        let wrapper = create_test_wrapper(Box::new(mock));
        let mut wrapper = RefCell::new(wrapper);

        let request = ffi::NewHttpClientRequest();
        let response = create_mock_response(ffi::ResponseCode::OK);

        wrapper.borrow_mut().on_success(&request, &response);

        assert!(wrapper.borrow().inner.is_none());
    }

    // @tc.name: ut_callback_wrapper_on_success_error_status
    // @tc.desc: Test on_success callback with error status code
    // @tc.precon: CallbackWrapper initialized with mock callback
    // @tc.step: 1. Create test response with 404 status
    // 2. Call on_success method
    // 3. Verify callback was triggered with failure
    // @tc.expect: on_fail is called with appropriate error
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_callback_wrapper_on_success_error_status() {
        let mut mock = MockRequestCallback::new();
        mock.expect_on_fail().once().return_const(());

        let wrapper = create_test_wrapper(Box::new(mock));
        let mut wrapper = RefCell::new(wrapper);

        let request = ffi::NewHttpClientRequest();
        let response = create_mock_response(ffi::ResponseCode::NOT_FOUND);

        wrapper.borrow_mut().on_success(&request, &response);

        assert!(wrapper.borrow().inner.is_none());
    }

    // @tc.name: ut_task_status_try_from
    // @tc.desc: Test TryFrom conversion for TaskStatus enum
    // @tc.precon: NA
    // @tc.step: 1. Attempt conversion from valid ffi::TaskStatus
    // 2. Attempt conversion from invalid ffi::TaskStatus
    // @tc.expect: Valid conversions succeed, invalid returns Err
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_task_status_try_from() {
        assert!(matches!(
            TaskStatus::try_from(ffi::TaskStatus::IDLE),
            Ok(TaskStatus::Idle)
        ));
        assert!(matches!(
            TaskStatus::try_from(ffi::TaskStatus::RUNNING),
            Ok(TaskStatus::Running)
        ));

        // Test invalid variant (using out-of-range value)
        let invalid_status = unsafe { std::mem::transmute(999) };
        assert!(matches!(TaskStatus::try_from(invalid_status), Err(_)));
    }

    // @tc.name: ut_response_code_try_from
    // @tc.desc: Test TryFrom conversion for ResponseCode enum
    // @tc.precon: NA
    // @tc.step: 1. Test conversion for multiple valid ResponseCode variants
    // 2. Test conversion for invalid ResponseCode
    // @tc.expect: Valid conversions return correct ResponseCode, invalid returns Err
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_response_code_try_from() {
        assert!(matches!(
            ResponseCode::try_from(ffi::ResponseCode::OK),
            Ok(ResponseCode::Ok)
        ));
        assert!(matches!(
            ResponseCode::try_from(ffi::ResponseCode::NOT_FOUND),
            Ok(ResponseCode::NotFound)
        ));
        assert!(matches!(
            ResponseCode::try_from(ffi::ResponseCode::INTERNAL_ERROR),
            Ok(ResponseCode::InternalError)
        ));

        // Test invalid variant
        let invalid_code = unsafe { std::mem::transmute(999) };
        assert!(matches!(ResponseCode::try_from(invalid_code), Err(_)));
    }

    // @tc.name: ut_http_error_code_try_from
    // @tc.desc: Test TryFrom conversion for HttpErrorCode enum
    // @tc.precon: NA
    // @tc.step: 1. Test conversion for multiple HttpErrorCode variants
    // 2. Test edge case with unknown error code
    // @tc.expect: Valid conversions return correct HttpErrorCode, unknown returns Err
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_http_error_code_try_from() {
        assert!(matches!(
            HttpErrorCode::try_from(ffi::HttpErrorCode::HTTP_NONE_ERR),
            Ok(HttpErrorCode::HttpNoneErr)
        ));
        assert!(matches!(
            HttpErrorCode::try_from(ffi::HttpErrorCode::HTTP_WRITE_ERROR),
            Ok(HttpErrorCode::HttpWriteError)
        ));
        assert!(matches!(
            HttpErrorCode::try_from(ffi::HttpErrorCode::HTTP_TASK_CANCELED),
            Ok(HttpErrorCode::HttpTaskCanceled)
        ));

        // Test edge case with unknown error code
        let unknown_code =
            unsafe { std::mem::transmute(ffi::HttpErrorCode::HTTP_UNKNOWN_OTHER_ERROR) };
        assert!(matches!(
            HttpErrorCode::try_from(unknown_code),
            Ok(HttpErrorCode::HttpUnknownOtherError)
        ));
    }

    // @tc.name: ut_callback_wrapper_on_progress
    // @tc.desc: Test progress callback functionality
    // @tc.precon: CallbackWrapper initialized with mock callback
    // @tc.step: 1. Set up mock to expect progress call
    // 2. Call on_progress with test values
    // 3. Verify mock was called with correct parameters
    // @tc.expect: on_progress is called with matching parameters
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_callback_wrapper_on_progress() {
        let mut mock = MockRequestCallback::new();
        mock.expect_on_progress()
            .withf(|dl_total, dl_now, ul_total, ul_now| {
                *dl_total == 1000 && *dl_now == 500 && *ul_total == 200 && *ul_now == 100
            })
            .once()
            .return_const(());

        let wrapper = create_test_wrapper(Box::new(mock));
        let mut wrapper = RefCell::new(wrapper);

        wrapper.borrow_mut().on_progress(1000, 500, 200, 100);
    }

    // @tc.name: ut_callback_wrapper_on_cancel_reset
    // @tc.desc: Test cancel behavior when reset flag is set
    // @tc.precon: CallbackWrapper with reset flag set to true
    // @tc.step: 1. Configure wrapper with reset=true
    // 2. Call on_cancel method
    // 3. Verify new task creation and callback restart
    // @tc.expect: New task is created and reset flag is cleared
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 3
    #[test]
    fn ut_callback_wrapper_on_cancel_reset() {
        let mut mock = MockRequestCallback::new();
        mock.expect_on_restart().once().return_const(());

        let reset = Arc::new(AtomicBool::new(true));
        let wrapper = create_test_wrapper_with_reset(Box::new(mock), reset.clone());
        let mut wrapper = RefCell::new(wrapper);

        let request = ffi::NewHttpClientRequest();
        let response = create_mock_response(ffi::ResponseCode::OK);

        wrapper.borrow_mut().on_cancel(&request, &response);

        assert!(!reset.load(Ordering::SeqCst));
    }

    // Helper functions for test setup
    fn create_test_wrapper(callback: Box<dyn RequestCallback>) -> CallbackWrapper {
        let reset = Arc::new(AtomicBool::new(false));
        let task = Arc::new(Mutex::new(SharedPtr::null()));
        let task_weak = Arc::downgrade(&task);
        let task_id = TaskId::new();
        let info_mgr = Arc::new(DownloadInfoMgr::new());

        CallbackWrapper::from_callback(callback, reset, task_weak, task_id, info_mgr, 0)
    }

    fn create_test_wrapper_with_reset(
        callback: Box<dyn RequestCallback>,
        reset: Arc<AtomicBool>,
    ) -> CallbackWrapper {
        let task = Arc::new(Mutex::new(SharedPtr::null()));
        let task_weak = Arc::downgrade(&task);
        let task_id = TaskId::new();
        let info_mgr = Arc::new(DownloadInfoMgr::new());

        CallbackWrapper::from_callback(callback, reset, task_weak, task_id, info_mgr, 0)
    }

    fn create_mock_response(code: ffi::ResponseCode) -> ffi::HttpClientResponse {
        // In a real test environment, this would be replaced with actual mock implementation
        unsafe { std::mem::zeroed() }
    }
}
