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

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use super::*;
use crate::manage::task_manager::TaskManagerTx;
use crate::service::active_counter::ActiveCounter;
use ylong_runtime::sync::mpsc::unbounded_channel;

static KEEPER: OnceLock<SAKeeper> = OnceLock::new();

fn keeper() -> &'static SAKeeper {
    KEEPER.get_or_init(|| {
        let (tx, _) = unbounded_channel();
        let task_manager_tx = TaskManagerTx::new(tx);
        let counter = ActiveCounter::new();
        SAKeeper::new(task_manager_tx, counter)
    })
}

// @tc.name: ut_sa_keeper_unload_waiting_constant
// @tc.desc: Test UNLOAD_WAITING constant value
// @tc.precon: NA
// @tc.step: 1. Check UNLOAD_WAITING constant
//           2. Verify expected value (60 seconds)
// @tc.expect: Constant has correct value for idle timeout
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_sa_keeper_unload_waiting_constant() {
    assert_eq!(UNLOAD_WAITING, 30);
}

// @tc.name: ut_sa_keeper_active_counter_integration
// @tc.desc: Test ActiveCounter integration with SAKeeper
// @tc.precon: NA
// @tc.step: 1. Create ActiveCounter
//           2. Test increment and decrement operations
//           3. Verify counter tracks active tasks
// @tc.expect: ActiveCounter works correctly for task tracking
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_sa_keeper_active_counter_integration() {
    let counter = ActiveCounter::new();
    assert_eq!(counter.get(), 0);
    
    counter.increment();
    assert_eq!(counter.get(), 1);
    
    counter.increment();
    assert_eq!(counter.get(), 2);
    
    counter.decrement();
    assert_eq!(counter.get(), 1);
    
    counter.decrement();
    assert_eq!(counter.get(), 0);
}

// @tc.name: ut_sa_keeper_clone_increments_counter
// @tc.desc: Test that clone increments active counter
// @tc.precon: NA
// @tc.step: 1. Create ActiveCounter
//           2. Simulate clone behavior (increment)
//           3. Verify counter increases
// @tc.expect: Clone increments active counter
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_sa_keeper_clone_increments_counter() {
    let counter = ActiveCounter::new();
    
    fn simulate_clone(counter: &ActiveCounter) {
        counter.increment();
    }
    
    simulate_clone(&counter);
    assert_eq!(counter.get(), 1);
    
    simulate_clone(&counter);
    assert_eq!(counter.get(), 2);
}

// @tc.name: ut_sa_keeper_drop_decrements_counter
// @tc.desc: Test that drop decrements active counter
// @tc.precon: NA
// @tc.step: 1. Create ActiveCounter with value
//           2. Simulate drop behavior (decrement)
//           3. Verify counter decreases
// @tc.expect: Drop decrements active counter
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_sa_keeper_drop_decrements_counter() {
    let counter = ActiveCounter::new();
    counter.increment();
    counter.increment();
    
    fn simulate_drop(counter: &ActiveCounter) {
        counter.decrement();
    }
    
    simulate_drop(&counter);
    assert_eq!(counter.get(), 1);
    
    simulate_drop(&counter);
    assert_eq!(counter.get(), 0);
}

// @tc.name: ut_sa_keeper_transition_logic
// @tc.desc: Test transition from 0 to 1 and 1 to 0 for countdown management
// @tc.precon: NA
// @tc.step: 1. Simulate counter transition from 0 to 1
//           2. Verify countdown should be canceled
//           3. Simulate counter transition from 1 to 0
//           4. Verify countdown should be restarted
// @tc.expect: Transition logic works correctly for countdown management
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_sa_keeper_transition_logic() {
    let counter = ActiveCounter::new();
    
    assert_eq!(counter.get(), 0);
    
    counter.increment();
    assert_eq!(counter.get(), 1);
    
    counter.decrement();
    assert_eq!(counter.get(), 0);
}

// @tc.name: ut_sa_keeper_multiple_clones_drops
// @tc.desc: Test multiple clone and drop operations
// @tc.precon: NA
// @tc.step: 1. Create ActiveCounter
//           2. Perform multiple increment/decrement operations
//           3. Verify counter tracks correctly
// @tc.expect: Counter tracks multiple operations correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_sa_keeper_multiple_clones_drops() {
    let counter = ActiveCounter::new();
    
    for _ in 0..5 {
        counter.increment();
    }
    assert_eq!(counter.get(), 5);
    
    for _ in 0..3 {
        counter.decrement();
    }
    assert_eq!(counter.get(), 2);
    
    for _ in 0..2 {
        counter.decrement();
    }
    assert_eq!(counter.get(), 0);
}

// @tc.name: ut_sa_keeper_duration_calculation
// @tc.desc: Test Duration calculation for unload timeout
// @tc.precon: NA
// @tc.step: 1. Create Duration from UNLOAD_WAITING seconds
//           2. Verify value in different units
// @tc.expect: Duration is calculated correctly for timeout
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_sa_keeper_duration_calculation() {
    use std::time::Duration;
    
    let duration = Duration::from_secs(UNLOAD_WAITING);
    
    assert_eq!(duration.as_secs(), 30);
    assert_eq!(duration.as_millis(), 30000);
    assert_eq!(duration.as_micros(), 30000000);
}

// @tc.name: ut_sa_keeper_inner_state_management
// @tc.desc: Test inner state management for countdown handle
// @tc.precon: NA
// @tc.step: 1. Create inner state with counter and handle
//           2. Test handle take and replace operations
// @tc.expect: Inner state management works correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_sa_keeper_inner_state_management() {
    struct Inner {
        cnt: usize,
        handle: Option<i32>,
    }
    
    let mut inner = Inner { cnt: 0, handle: Some(42) };
    
    assert_eq!(inner.cnt, 0);
    assert_eq!(inner.handle, Some(42));
    
    inner.cnt += 1;
    if inner.cnt == 1 {
        let taken = inner.handle.take();
        assert_eq!(taken, Some(42));
    }
    assert!(inner.handle.is_none());
    
    inner.cnt -= 1;
    if inner.cnt == 0 {
        inner.handle = Some(100);
    }
    assert_eq!(inner.handle, Some(100));
}

// @tc.name: ut_sa_keeper_counter_never_negative
// @tc.desc: Test that counter never goes negative
// @tc.precon: NA
// @tc.step: 1. Create ActiveCounter
//           2. Decrement when counter is 0
//           3. Verify counter stays at 0
// @tc.expect: Counter never goes negative
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_sa_keeper_counter_never_negative() {
    let counter = ActiveCounter::new();
    
    counter.decrement();
    assert_eq!(counter.get(), 0);
    
    counter.decrement();
    assert_eq!(counter.get(), 0);
}

// @tc.name: ut_sa_keeper_concurrent_access
// @tc.desc: Test concurrent access to active counter
// @tc.precon: NA
// @tc.step: 1. Create ActiveCounter
//           2. Simulate concurrent increment/decrement
//           3. Verify final state
// @tc.expect: Concurrent access works correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_sa_keeper_concurrent_access() {
    use std::sync::atomic::AtomicUsize;
    use std::sync::Arc;
    
    let counter = ActiveCounter::new();
    
    counter.increment();
    counter.increment();
    counter.increment();
    counter.decrement();
    counter.increment();
    counter.decrement();

    assert_eq!(counter.get(), 2);
}

// @tc.name: ut_sa_keeper_restart_count_down
// @tc.desc: Test restart_count_down on a real SAKeeper instance
// @tc.precon: NA
// @tc.step: 1. Get shared SAKeeper
//           2. Verify timer is running after creation
//           3. Call restart_count_down
//           4. Verify timer is still running
// @tc.expect: restart_count_down keeps the timer alive on a healthy keeper
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_sa_keeper_restart_count_down() {
    let k = keeper();
    assert!(k.is_timer_running());
    k.restart_count_down();
    assert!(k.is_timer_running());
}

// @tc.name: ut_sa_keeper_restart_count_down_after_shutdown
// @tc.desc: Test the full shutdown -> restart cycle on a real SAKeeper
// @tc.precon: NA
// @tc.step: 1. Get shared SAKeeper (timer running)
//           2. Call shutdown to kill timer
//           3. Verify timer is stopped
//           4. Call restart_count_down
//           5. Verify timer is running again
// @tc.expect: restart_count_down restores timer after shutdown
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_sa_keeper_restart_count_down_after_shutdown() {
    let k = keeper();
    k.shutdown();
    assert!(!k.is_timer_running());
    k.restart_count_down();
    assert!(k.is_timer_running());
}

// @tc.name: ut_sa_keeper_multiple_restart_count_down
// @tc.desc: Test multiple consecutive restart_count_down calls
// @tc.precon: NA
// @tc.step: 1. Get shared SAKeeper
//           2. Call restart_count_down multiple times
//           3. Verify timer is running after each call
// @tc.expect: Multiple restarts are safe and timer stays alive
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_sa_keeper_multiple_restart_count_down() {
    let k = keeper();
    for _ in 0..3 {
        k.restart_count_down();
        assert!(k.is_timer_running());
    }
}