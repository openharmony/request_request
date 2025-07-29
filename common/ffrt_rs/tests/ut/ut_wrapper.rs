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
mod ut_wrapper {
    use super::*;
    use std::sync::{Arc, Mutex};

    // @tc.name: ut_closure_wrapper_new
    // @tc.desc: Test creation of ClosureWrapper
    // @tc.precon: NA
    // @tc.step: 1. Create a new ClosureWrapper with a simple closure
    // 2. Verify the wrapper contains a closure
    // @tc.expect: ClosureWrapper is successfully created with non-None inner value
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_closure_wrapper_new_001() {
        let closure = || {};
        let wrapper = ClosureWrapper::new(closure);
        assert!(wrapper.inner.is_some());
    }

    // @tc.name: ut_closure_wrapper_run
    // @tc.desc: Test execution of closure in ClosureWrapper
    // @tc.precon: NA
    // @tc.step: 1. Create a shared counter
    // 2. Create ClosureWrapper with closure that increments counter
    // 3. Run the closure
    // 4. Verify counter was incremented
    // @tc.expect: Closure executes successfully and modifies external state
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_closure_wrapper_run_001() {
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = Arc::clone(&counter);

        let mut wrapper = ClosureWrapper::new(move || {
            *counter_clone.lock().unwrap() += 1;
        });

        wrapper.run();
        assert_eq!(*counter.lock().unwrap(), 1);
    }

    // @tc.name: ut_closure_wrapper_run_twice
    // @tc.desc: Test behavior when running closure twice
    // @tc.precon: NA
    // @tc.step: 1. Create ClosureWrapper with closure that increments counter
    // 2. Run the closure twice
    // 3. Verify counter was only incremented once
    // @tc.expect: Closure executes only once, second run has no effect
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_closure_wrapper_run_twice_001() {
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = Arc::clone(&counter);

        let mut wrapper = ClosureWrapper::new(move || {
            *counter_clone.lock().unwrap() += 1;
        });

        wrapper.run();
        wrapper.run(); // Second run should have no effect
        assert_eq!(*counter.lock().unwrap(), 1);
    }

    // @tc.name: ut_closure_wrapper_empty
    // @tc.desc: Test behavior with empty closure
    // @tc.precon: NA
    // @tc.step: 1. Create ClosureWrapper with empty closure
    // 2. Run the closure
    // @tc.expect: No panic occurs, closure executes successfully
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_closure_wrapper_empty_001() {
        let mut wrapper = ClosureWrapper::new(|| {});
        wrapper.run(); // Should not panic
    }

    // @tc.name: ut_ffrt_sleep
    // @tc.desc: Test FfrtSleep function
    // @tc.precon: NA
    // @tc.step: 1. Record start time
    // 2. Call FfrtSleep with 10ms
    // 3. Record end time
    // 4. Verify elapsed time is at least 10ms
    // @tc.expect: Function sleeps for approximately the specified duration
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_ffrt_sleep_001() {
        let start = std::time::Instant::now();
        FfrtSleep(10);
        let duration = start.elapsed();
        assert!(duration.as_millis() >= 10);
    }

    // @tc.name: ut_ffrt_spawn
    // @tc.desc: Test FfrtSpawn function
    // @tc.precon: NA
    // @tc.step: 1. Create a shared counter
    // 2. Spawn a closure that increments the counter
    // 3. Sleep to allow task to complete
    // 4. Verify counter was incremented
    // @tc.expect: Spawned closure executes successfully
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_ffrt_spawn_001() {
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = Arc::clone(&counter);

        FfrtSpawn(ClosureWrapper::new(move || {
            *counter_clone.lock().unwrap() += 1;
        }));

        FfrtSleep(100); // Give time for the spawned task to execute
        assert_eq!(*counter.lock().unwrap(), 1);
    }
}
