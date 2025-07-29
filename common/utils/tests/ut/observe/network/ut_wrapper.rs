// Copyright (C) 2025 Huawei Device Co., Ltd.
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
mod ut_wrapper {
    use super::*;
    use super::ffi::{NetBearType, NetCap, NetInfo};
    use mockall::mock;
    use mockall::automock;
    use std::sync::{Arc, Barrier};

    mock! {
        pub Observer {
            fn net_available(&self, net_id: i32);
            fn net_lost(&self, net_id: i32);
            fn net_capability_changed(&self, net_id: i32, net_info: &NetInfo);
        }

        impl Send for Observer {}
        impl Sync for Observer {}
    }

    // @tc.name: ut_net_observer_wrapper_create
    // @tc.desc: Test creation of NetObserverWrapper instance
    // @tc.precon: NA
    // @tc.step: 1. Call NetObserverWrapper::new()
    // @tc.expect: Successfully create a non-null NetObserverWrapper instance
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_net_observer_wrapper_create() {
        let wrapper = NetObserverWrapper::new();
        assert!(!Arc::strong_count(&wrapper) == 0);
    }

    // @tc.name: ut_net_observer_wrapper_add_observer
    // @tc.desc: Test adding an observer and verifying event forwarding
    // @tc.precon: NA
    // @tc.step: 1. Create wrapper instance
    // 2. Create mock observer with expected net_available call
    // 3. Add observer to wrapper
    // 4. Trigger net_available event
    // @tc.expect: Mock observer receives net_available call with correct net_id
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_net_observer_wrapper_add_observer() {
        let wrapper = NetObserverWrapper::new();
        let mut mock = MockObserver::new();
        mock.expect_net_available()
            .with(eq(42))
            .times(1)
            .return_const(());

        wrapper.add_observer(Box::new(mock));
        wrapper.net_available(42);
    }

    // @tc.name: ut_net_observer_wrapper_multiple_observers
    // @tc.desc: Test event forwarding to multiple observers
    // @tc.precon: NA
    // @tc.step: 1. Create wrapper instance
    // 2. Create two mock observers with expected calls
    // 3. Add both observers to wrapper
    // 4. Trigger net_lost event
    // @tc.expect: Both observers receive net_lost call
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_net_observer_wrapper_multiple_observers() {
        let wrapper = NetObserverWrapper::new();
        let mut mock1 = MockObserver::new();
        let mut mock2 = MockObserver::new();

        mock1.expect_net_lost()
            .with(eq(100))
            .times(1)
            .return_const(());

        mock2.expect_net_lost()
            .with(eq(100))
            .times(1)
            .return_const(());

        wrapper.add_observer(Box::new(mock1));
        wrapper.add_observer(Box::new(mock2));
        wrapper.net_lost(100);
    }

    // @tc.name: ut_net_observer_wrapper_remove_observer
    // @tc.desc: Test removing an observer stops event forwarding
    // @tc.precon: NA
    // @tc.step: 1. Create wrapper instance
    // 2. Create mock observer and get its ID
    // 3. Add observer to wrapper
    // 4. Remove observer using ID
    // 5. Trigger net_available event
    // @tc.expect: Removed observer does not receive event
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_net_observer_wrapper_remove_observer() {
        let wrapper = NetObserverWrapper::new();
        let mut mock = MockObserver::new();
        mock.expect_net_available()
            .times(0);

        let observer_id = wrapper.add_observer(Box::new(mock));
        wrapper.remove_observer(observer_id);
        wrapper.net_available(42);
    }

    // @tc.name: ut_net_observer_wrapper_empty_observers
    // @tc.desc: Test behavior with empty observer list
    // @tc.precon: NA
    // @tc.step: 1. Create wrapper instance
    // 2. Trigger net_capability_changed with empty observer list
    // @tc.expect: No panic occurs and method completes successfully
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_net_observer_wrapper_empty_observers() {
        let wrapper = NetObserverWrapper::new();
        let net_info = NetInfo {
            caps: vec![],
            bear_types: vec![],
        };
        wrapper.net_capability_changed(0, &net_info);
    }

    // @tc.name: ut_net_observer_wrapper_mutex_poisoning
    // @tc.desc: Test recovery from mutex poisoning
    // @tc.precon: NA
    // @tc.step: 1. Create wrapper and spawn thread that panics while holding lock
    // 2. Attempt to add observer after mutex poisoning
    // @tc.expect: Subsequent operations recover from poisoning
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_net_observer_wrapper_mutex_poisoning() {
        let wrapper = Arc::new(NetObserverWrapper::new());
        let wrapper_clone = Arc::clone(&wrapper);

        // Create a poisoned mutex scenario
        std::thread::spawn(move || {
            let mut observers = wrapper_clone.observers.lock().unwrap();
            panic!();
        }).join().unwrap_err();

        // Verify we can still operate after poisoning
        let mut mock = MockObserver::new();
        mock.expect_net_available()
            .times(0);

        wrapper.add_observer(Box::new(mock));
    }

    // @tc.name: ut_net_observer_wrapper_thread_safety
    // @tc.desc: Test thread safety of observer operations
    // @tc.precon: NA
    // @tc.step: 1. Create wrapper shared across 10 threads
    // 2. Each thread adds and removes observers concurrently
    // 3. Verify no data races or panics occur
    // @tc.expect: All operations complete without errors
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 3
    #[test]
    fn ut_net_observer_wrapper_thread_safety() {
        let wrapper = Arc::new(NetObserverWrapper::new());
        let barrier = Arc::new(Barrier::new(10));
        let mut handles = vec![];

        for _ in 0..10 {
            let wrapper_clone = Arc::clone(&wrapper);
            let barrier_clone = Arc::clone(&barrier);

            handles.push(std::thread::spawn(move || {
                barrier_clone.wait();
                let observer_id = wrapper_clone.add_observer(Box::new(MockObserver::new()));
                wrapper_clone.remove_observer(observer_id);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}