// Copyright (C) 2023 Huawei Device Co., Ltd.
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

//! Thread-safe counter for tracking active tasks or operations.
//! 
//! Provides a simple, atomic counter implementation for tracking whether any operations
//! are currently active. This is useful for determining when a system can safely shut down
//! or enter an idle state.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

/// Thread-safe counter for tracking active operations.
/// 
/// Uses an atomic counter to safely track whether operations are in progress across multiple threads.
/// Provides methods to increment, decrement, and check if any operations are currently active.
#[derive(Clone)]
pub(crate) struct ActiveCounter {
    /// Atomic counter storing the number of active operations
    count: Arc<AtomicU32>,
}

impl ActiveCounter {
    /// Creates a new active counter with an initial value of zero.
    /// 
    /// # Returns
    /// 
    /// A new `ActiveCounter` instance that is not active initially
    pub(crate) fn new() -> Self {
        Self {
            count: Arc::new(AtomicU32::new(0)),
        }
    }

    /// Increments the active count by one.
    /// 
    /// # Note
    /// 
    /// Uses `Ordering::Relaxed` as the memory ordering since this counter is primarily
    /// used for liveness checks rather than precise synchronization.
    pub(crate) fn increment(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrements the active count by one.
    /// 
    /// # Note
    /// 
    /// Uses `Ordering::Relaxed` as the memory ordering. Decrementing below zero will cause
    /// an underflow, which will result in the counter wrapping to a large value.
    pub(crate) fn decrement(&self) {
        self.count.fetch_sub(1, Ordering::Relaxed);
    }

    /// Checks if there are any active operations.
    /// 
    /// # Returns
    /// 
    /// `true` if the count is greater than zero, indicating active operations
    pub(crate) fn is_active(&self) -> bool {
        // Load the current count with relaxed ordering
        let count = self.count.load(Ordering::Relaxed);
        info!("active count: {}", count);
        count > 0
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use std::thread;

    use super::*;

    // @tc.name: ut_active_counter_new
    // @tc.desc: Test ActiveCounter initialization
    // @tc.precon: NA
    // @tc.step: 1. Create a new ActiveCounter instance
    //           2. Check if is_active returns false
    // @tc.expect: New counter should not be active
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
    #[test]
    fn ut_active_counter_new_001() {
        let counter = ActiveCounter::new();
        assert!(!counter.is_active());
    }

    // @tc.name: ut_active_counter_increment
    // @tc.desc: Test single increment operation
    // @tc.precon: NA
    // @tc.step: 1. Create a new ActiveCounter instance
    //           2. Call increment method once
    //           3. Check if is_active returns true
    // @tc.expect: Counter should be active after increment
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
    #[test]
    fn ut_active_counter_increment_001() {
        let counter = ActiveCounter::new();
        counter.increment();
        assert!(counter.is_active());
    }

    // @tc.name: ut_active_counter_decrement
    // @tc.desc: Test single decrement operation
    // @tc.precon: Counter should have count > 0
    // @tc.step: 1. Create a new ActiveCounter instance
    //           2. Call increment method once
    //           3. Call decrement method once
    //           4. Check if is_active returns false
    // @tc.expect: Counter should not be active after decrement
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
    #[test]
    fn ut_active_counter_decrement_001() {
        let counter = ActiveCounter::new();
        counter.increment();
        counter.decrement();
        assert!(!counter.is_active());
    }

    // @tc.name: ut_active_counter_multiple_increments
    // @tc.desc: Test multiple increment operations
    // @tc.precon: NA
    // @tc.step: 1. Create a new ActiveCounter instance
    //           2. Call increment method multiple times
    //           3. Check if is_active returns true
    // @tc.expect: Counter should remain active with count > 1
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
    #[test]
    fn ut_active_counter_multiple_increments_001() {
        let counter = ActiveCounter::new();
        for _ in 0..5 {
            counter.increment();
        }
        assert!(counter.is_active());
    }

    // @tc.name: ut_active_counter_multiple_decrements
    // @tc.desc: Test multiple decrement operations
    // @tc.precon: Counter should have count >= number of decrements
    // @tc.step: 1. Create a new ActiveCounter instance
    //           2. Call increment method multiple times
    //           3. Call decrement method multiple times
    //           4. Check if is_active returns false when count reaches 0
    // @tc.expect: Counter should not be active after sufficient decrements
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
    #[test]
    fn ut_active_counter_multiple_decrements_001() {
        let counter = ActiveCounter::new();
        for _ in 0..3 {
            counter.increment();
        }
        for _ in 0..3 {
            counter.decrement();
        }
        assert!(!counter.is_active());
    }

    // @tc.name: ut_active_counter_decrement_below_zero
    // @tc.desc: Test decrement operation when count is 0 (negative case)
    // @tc.precon: Counter should have count = 0
    // @tc.step: 1. Create a new ActiveCounter instance
    //           2. Call decrement method
    //           3. Check if is_active returns false
    // @tc.expect: Counter should remain not active, count should wrap around
    // (underflow)
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
    #[test]
    fn ut_active_counter_decrement_below_zero_001() {
        let counter = ActiveCounter::new();
        counter.decrement(); // This will underflow to u32::MAX
        assert!(counter.is_active()); // Should be true due to underflow
    }

    // @tc.name: sdv_active_counter_concurrent_access
    // @tc.desc: Test concurrent increment and decrement operations
    // @tc.precon: NA
    // @tc.step: 1. Create a shared ActiveCounter instance
    //           2. Spawn multiple threads to increment and decrement
    //           3. Wait for all threads to complete
    //           4. Check final state
    // @tc.expect: Counter should be thread-safe and maintain consistency
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
    #[test]
    fn sdv_active_counter_concurrent_access_001() {
        let counter = Arc::new(ActiveCounter::new());
        let mut handles = vec![];

        // Spawn 5 threads, each incrementing 100 times
        for _ in 0..5 {
            let counter_clone = counter.clone();
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    counter_clone.increment();
                }
            });
            handles.push(handle);
        }

        // Spawn 5 threads, each decrementing 100 times
        for _ in 0..5 {
            let counter_clone = counter.clone();
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    counter_clone.decrement();
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Since we have equal increments and decrements, counter should be 0
        assert!(!counter.is_active());
    }

    // @tc.name: ut_active_counter_clone_behavior
    // @tc.desc: Test clone behavior of ActiveCounter
    // @tc.precon: NA
    // @tc.step: 1. Create a new ActiveCounter instance
    //           2. Clone the counter
    //           3. Modify original counter
    //           4. Check if cloned counter reflects changes
    // @tc.expect: Cloned counter should share the same underlying state
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
    #[test]
    fn ut_active_counter_clone_behavior_001() {
        let counter1 = ActiveCounter::new();
        let counter2 = counter1.clone();

        counter1.increment();
        assert!(counter2.is_active()); // counter2 should see the change

        counter2.decrement();
        assert!(!counter1.is_active()); // counter1 should see the change
    }

    // @tc.name: ut_active_counter_large_count
    // @tc.desc: Test ActiveCounter with large count values
    // @tc.precon: NA
    // @tc.step: 1. Create a new ActiveCounter instance
    //           2. Increment a large number of times
    //           3. Check if is_active returns true
    //           4. Decrement the same number of times
    //           5. Check if is_active returns false
    // @tc.expect: Counter should handle large counts correctly
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
    #[test]
    fn ut_active_counter_large_count_001() {
        let counter = ActiveCounter::new();
        let large_count = 10000;

        for _ in 0..large_count {
            counter.increment();
        }
        assert!(counter.is_active());

        for _ in 0..large_count {
            counter.decrement();
        }
        assert!(!counter.is_active());
    }
}
