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
mod ut_callback {
    use super::*;
    use cache_core::CacheManager;
    use mockall::automock;
    use mockall::mock;
    use request_utils::task_id::TaskId;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    // Mock for PreloadCallback trait
    mock! {
        pub PreloadCallback {}
        impl PreloadCallback for PreloadCallback {
            fn on_progress(&mut self, current: u64, total: u64);
            fn on_success(&mut self, cache: cache_core::Cache, task_id: &str);
            fn on_fail(&mut self, error: CacheDownloadError, info: RustDownloadInfo, task_id: &str);
            fn on_cancel(&mut self);
        }
    }

    // Mock for CacheManager
    mock! {
        pub CacheManager {}
        impl CacheManager for CacheManager {
            fn get_updater(&self, task_id: &TaskId) -> Updater {
                Updater::new(task_id.clone(), self)
            }
        }
    }

    // @tc.name: ut_prime_callback_new
    // @tc.desc: Test PrimeCallback creation with proper initialization
    // @tc.precon: NA
    // @tc.step: 1. Create required atomic variables and mock objects
    // 2. Initialize PrimeCallback with new()
    // 3. Verify initialized fields match expectations
    // @tc.expect: PrimeCallback instance created with correct task_id and initial state
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_prime_callback_new() {
        let task_id = TaskId::new();
        let mock_cache_manager = MockCacheManager::new();
        let finish = Arc::new(AtomicBool::new(false));
        let state = Arc::new(AtomicUsize::new(0));
        let callbacks = Arc::new(Mutex::new(VecDeque::new()));
        let seq = 1;

        let callback = PrimeCallback::new(
            task_id.clone(),
            &mock_cache_manager,
            finish.clone(),
            state.clone(),
            callbacks.clone(),
            seq,
        );

        assert_eq!(callback.task_id(), task_id);
        assert!(!callback.finish.load(Ordering::Acquire));
        assert_eq!(callback.state.load(Ordering::Acquire), 0);
        assert_eq!(callback.seq, seq);
    }

    // @tc.name: ut_prime_callback_set_running
    // @tc.desc: Test state transition when set_running is called
    // @tc.precon: PrimeCallback instance created with initial state 0
    // @tc.step: 1. Create PrimeCallback instance
    // 2. Call set_running() method
    // 3. Check state value
    // @tc.expect: State is set to RUNNING constant
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_prime_callback_set_running() {
        let task_id = TaskId::new();
        let mock_cache_manager = MockCacheManager::new();
        let finish = Arc::new(AtomicBool::new(false));
        let state = Arc::new(AtomicUsize::new(0));
        let callbacks = Arc::new(Mutex::new(VecDeque::new()));

        let callback = PrimeCallback::new(
            task_id,
            &mock_cache_manager,
            finish,
            state.clone(),
            callbacks,
            1,
        );

        callback.set_running();
        assert_eq!(state.load(Ordering::Acquire), RUNNING);
    }

    // @tc.name: ut_prime_callback_common_success
    // @tc.desc: Test success path handling
    // @tc.precon: PrimeCallback instance with mock callback
    // @tc.step: 1. Create PrimeCallback with mock callback
    // 2. Call common_success with mock response
    // 3. Verify state and callback interactions
    // @tc.expect: State set to SUCCESS, finish flag set, callback on_success called
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_prime_callback_common_success() {
        let task_id = TaskId::new();
        let mock_cache_manager = MockCacheManager::new();
        let finish = Arc::new(AtomicBool::new(false));
        let state = Arc::new(AtomicUsize::new(RUNNING));
        let mut mock_callback = MockPreloadCallback::new();
        mock_callback.expect_on_success().once();
        mock_callback.expect_on_progress().once();
        let callbacks = Arc::new(Mutex::new(VecDeque::from([
            Box::new(mock_callback) as Box<dyn PreloadCallback>
        ])));

        let mut callback = PrimeCallback::new(
            task_id.clone(),
            &mock_cache_manager,
            finish.clone(),
            state.clone(),
            callbacks.clone(),
            1,
        );

        struct MockResponse;
        impl CommonResponse for MockResponse {
            fn code(&self) -> u16 {
                200
            }
        }

        callback.common_success(MockResponse);

        assert_eq!(state.load(Ordering::Acquire), SUCCESS);
        assert!(finish.load(Ordering::Acquire));
        assert!(callbacks.lock().unwrap().is_empty());
    }

    // @tc.name: ut_prime_callback_common_fail
    // @tc.desc: Test failure path handling
    // @tc.precon: PrimeCallback instance with mock callback
    // @tc.step: 1. Create PrimeCallback with mock callback
    // 2. Call common_fail with mock error
    // 3. Verify state and callback interactions
    // @tc.expect: State set to FAIL, finish flag set, callback on_fail called
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_prime_callback_common_fail() {
        let task_id = TaskId::new();
        let mock_cache_manager = MockCacheManager::new();
        let finish = Arc::new(AtomicBool::new(false));
        let state = Arc::new(AtomicUsize::new(RUNNING));
        let mut mock_callback = MockPreloadCallback::new();
        mock_callback.expect_on_fail().once();
        let callbacks = Arc::new(Mutex::new(VecDeque::from([
            Box::new(mock_callback) as Box<dyn PreloadCallback>
        ])));

        let mut callback = PrimeCallback::new(
            task_id.clone(),
            &mock_cache_manager,
            finish.clone(),
            state.clone(),
            callbacks.clone(),
            1,
        );

        struct MockError;
        impl CommonError for MockError {
            fn code(&self) -> u16 {
                404
            }
        }

        callback.common_fail(MockError);

        assert_eq!(state.load(Ordering::Acquire), FAIL);
        assert!(finish.load(Ordering::Acquire));
        assert!(callbacks.lock().unwrap().is_empty());
    }

    // @tc.name: ut_prime_callback_common_cancel
    // @tc.desc: Test cancellation path handling
    // @tc.precon: PrimeCallback instance with mock callback
    // @tc.step: 1. Create PrimeCallback with mock callback
    // 2. Call common_cancel()
    // 3. Verify state and callback interactions
    // @tc.expect: State set to CANCEL, finish flag set, callback on_cancel called
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_prime_callback_common_cancel() {
        let task_id = TaskId::new();
        let mock_cache_manager = MockCacheManager::new();
        let finish = Arc::new(AtomicBool::new(false));
        let state = Arc::new(AtomicUsize::new(RUNNING));
        let mut mock_callback = MockPreloadCallback::new();
        mock_callback.expect_on_cancel().once();
        let callbacks = Arc::new(Mutex::new(VecDeque::from([
            Box::new(mock_callback) as Box<dyn PreloadCallback>
        ])));

        let mut callback = PrimeCallback::new(
            task_id.clone(),
            &mock_cache_manager,
            finish.clone(),
            state.clone(),
            callbacks.clone(),
            1,
        );

        callback.common_cancel();

        assert_eq!(state.load(Ordering::Acquire), CANCEL);
        assert!(finish.load(Ordering::Acquire));
        assert!(callbacks.lock().unwrap().is_empty());
    }

    // @tc.name: ut_prime_callback_common_progress_001
    // @tc.desc: Test progress updates under normal conditions
    // @tc.precon: PrimeCallback with initialized progress restriction
    // @tc.step: 1. Create PrimeCallback with mock callback
    // 2. Set data_receive to true
    // 3. Call common_progress with sample values
    // 4. Verify progress callback called
    // @tc.expect: Progress callback invoked with correct parameters
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_prime_callback_common_progress_001() {
        let task_id = TaskId::new();
        let mock_cache_manager = MockCacheManager::new();
        let finish = Arc::new(AtomicBool::new(false));
        let state = Arc::new(AtomicUsize::new(RUNNING));
        let mut mock_callback = MockPreloadCallback::new();
        mock_callback
            .expect_on_progress()
            .withf(|&current, &total| current == 50 && total == 100)
            .once();
        let callbacks = Arc::new(Mutex::new(VecDeque::from([
            Box::new(mock_callback) as Box<dyn PreloadCallback>
        ])));

        let mut callback = PrimeCallback::new(
            task_id.clone(),
            &mock_cache_manager,
            finish.clone(),
            state.clone(),
            callbacks.clone(),
            1,
        );

        callback.progress_restriction.data_receive = true;
        callback.progress_restriction.count = PROGRESS_INTERVAL - 1; // Ensure next call triggers
        callback.common_progress(100, 50, 0, 0);
    }

    // @tc.name: ut_prime_callback_common_progress_002
    // @tc.desc: Test progress restriction logic
    // @tc.precon: PrimeCallback with initialized progress restriction
    // @tc.step: 1. Create PrimeCallback with mock callback
    // 2. Set data_receive to true
    // 3. Call common_progress with same dl_now value
    // 4. Verify progress callback not called
    // @tc.expect: Progress callback not invoked when progress doesn't change
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_prime_callback_common_progress_002() {
        let task_id = TaskId::new();
        let mock_cache_manager = MockCacheManager::new();
        let finish = Arc::new(AtomicBool::new(false));
        let state = Arc::new(AtomicUsize::new(RUNNING));
        let mut mock_callback = MockPreloadCallback::new();
        mock_callback.expect_on_progress().never();
        let callbacks = Arc::new(Mutex::new(VecDeque::from([
            Box::new(mock_callback) as Box<dyn PreloadCallback>
        ])));

        let mut callback = PrimeCallback::new(
            task_id.clone(),
            &mock_cache_manager,
            finish.clone(),
            state.clone(),
            callbacks.clone(),
            1,
        );

        callback.progress_restriction.data_receive = true;
        callback.progress_restriction.processed = 50;
        callback.common_progress(100, 50, 0, 0); // Same as processed
    }

    // @tc.name: ut_prime_callback_common_data_receive
    // @tc.desc: Test data receiving functionality
    // @tc.precon: PrimeCallback instance
    // @tc.step: 1. Create PrimeCallback
    // 2. Call common_data_receive with sample data
    // 3. Verify data_receive flag set and cache updated
    // @tc.expect: data_receive flag set to true, cache receives data
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_prime_callback_common_data_receive() {
        let task_id = TaskId::new();
        let mock_cache_manager = MockCacheManager::new();
        let finish = Arc::new(AtomicBool::new(false));
        let state = Arc::new(AtomicUsize::new(RUNNING));
        let callbacks = Arc::new(Mutex::new(VecDeque::new()));

        let mut callback = PrimeCallback::new(
            task_id.clone(),
            &mock_cache_manager,
            finish.clone(),
            state.clone(),
            callbacks.clone(),
            1,
        );

        assert!(!callback.progress_restriction.data_receive);

        let test_data = b"test data";
        callback.common_data_receive(test_data, || Some(test_data.len()));

        assert!(callback.progress_restriction.data_receive);
    }

    // @tc.name: ut_prime_callback_empty_callbacks
    // @tc.desc: Test callback handling with empty callbacks list
    // @tc.precon: PrimeCallback with empty callbacks queue
    // @tc.step: 1. Create PrimeCallback with empty callbacks
    // 2. Call common_success
    // 3. Verify no panics occur
    // @tc.expect: Method completes successfully without panics
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_prime_callback_empty_callbacks() {
        let task_id = TaskId::new();
        let mock_cache_manager = MockCacheManager::new();
        let finish = Arc::new(AtomicBool::new(false));
        let state = Arc::new(AtomicUsize::new(RUNNING));
        let callbacks = Arc::new(Mutex::new(VecDeque::new()));

        let mut callback = PrimeCallback::new(
            task_id.clone(),
            &mock_cache_manager,
            finish.clone(),
            state.clone(),
            callbacks.clone(),
            1,
        );

        struct MockResponse;
        impl CommonResponse for MockResponse {
            fn code(&self) -> u16 {
                200
            }
        }

        // Should complete without panicking
        callback.common_success(MockResponse);
    }
}
