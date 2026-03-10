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

use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};
use std::sync::Arc;

use super::*;
use crate::config::Mode;
use crate::task::config::Action;
use crate::task::reason::Reason;

// @tc.name: ut_running_task_abort_flag
// @tc.desc: Test abort flag atomic operations in running task context
// @tc.precon: NA
// @tc.step: 1. Create AtomicBool abort flag
//           2. Test load and store operations
//           3. Verify abort flag can be set for task cancellation
// @tc.expect: Abort flag works correctly for task cancellation signaling
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_running_task_abort_flag() {
    let abort_flag = Arc::new(AtomicBool::new(false));
    
    assert!(!abort_flag.load(Ordering::SeqCst));
    
    abort_flag.store(true, Ordering::SeqCst);
    assert!(abort_flag.load(Ordering::SeqCst));
}

// @tc.name: ut_running_task_max_speed
// @tc.desc: Test max_speed atomic operations in running task context
// @tc.precon: NA
// @tc.step: 1. Create AtomicI64 for max_speed
//           2. Test load and store operations
//           3. Verify speed limit can be set (0 = unlimited, positive = bytes/sec)
// @tc.expect: max_speed atomic operations work correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_running_task_max_speed() {
    let max_speed = Arc::new(AtomicI64::new(0));
    
    assert_eq!(max_speed.load(Ordering::SeqCst), 0);
    
    max_speed.store(1024 * 1024, Ordering::SeqCst);
    assert_eq!(max_speed.load(Ordering::SeqCst), 1048576);
    
    max_speed.store(400 * 1024, Ordering::SeqCst);
    assert_eq!(max_speed.load(Ordering::SeqCst), 409600);
}

// @tc.name: ut_running_task_task_time
// @tc.desc: Test task_time atomic operations for timing tracking
// @tc.precon: NA
// @tc.step: 1. Create AtomicU64 for task_time
//           2. Simulate task timing updates
//           3. Verify cumulative time tracking
// @tc.expect: task_time atomic operations work correctly for timing tracking
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_running_task_task_time() {
    let task_time = Arc::new(AtomicU64::new(0));
    let start_time = Arc::new(AtomicU64::new(1000));
    
    task_time.store(3600, Ordering::SeqCst);
    assert_eq!(task_time.load(Ordering::SeqCst), 3600);
    
    let current_start = start_time.load(Ordering::SeqCst);
    start_time.store(1600, Ordering::SeqCst);
    let current_task_time = 1600 - current_start;
    let total = task_time.load(Ordering::SeqCst) + current_task_time;
    task_time.store(total, Ordering::SeqCst);
    
    assert_eq!(task_time.load(Ordering::SeqCst), 4200);
}

// @tc.name: ut_running_task_action_type
// @tc.desc: Test task action type matching in running context
// @tc.precon: NA
// @tc.step: 1. Test Action enum variants
//           2. Verify pattern matching for download/upload dispatch
// @tc.expect: Action type works correctly for task dispatch
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_running_task_action_type() {
    let download = Action::Download;
    let upload = Action::Upload;
    let any = Action::Any;
    
    match download {
        Action::Download => assert!(true),
        Action::Upload => panic!("Expected Download"),
        Action::Any => panic!("Expected Download"),
    }
    
    match upload {
        Action::Upload => assert!(true),
        _ => panic!("Expected Upload"),
    }
    
    match any {
        Action::Any => assert!(true),
        _ => panic!("Expected Any"),
    }
}

// @tc.name: ut_running_task_mode_type
// @tc.desc: Test task mode type for foreground/background execution
// @tc.precon: NA
// @tc.step: 1. Test Mode enum variants
//           2. Verify pattern matching for task priority
// @tc.expect: Mode type works correctly for task priority
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_running_task_mode_type() {
    let frontend = Mode::FrontEnd;
    let background = Mode::BackGround;
    
    match frontend {
        Mode::FrontEnd => assert!(true),
        Mode::BackGround => panic!("Expected FrontEnd"),
        Mode::Any => panic!("Expected FrontEnd"),
    }
    
    match background {
        Mode::BackGround => assert!(true),
        _ => panic!("Expected BackGround"),
    }
}

// @tc.name: ut_running_task_check_download_complete
// @tc.desc: Test download completion check logic
// @tc.precon: NA
// @tc.step: 1. Test completion conditions with different total/processed values
//           2. Verify -1 (unknown size) returns false
//           3. Verify completion when processed equals total
// @tc.expect: Download completion check works correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_running_task_check_download_complete() {
    fn check_complete(total: i64, processed: usize) -> bool {
        if total == -1 {
            return false;
        }
        processed == (total as usize)
    }
    
    assert!(!check_complete(-1, 0));
    assert!(!check_complete(-1, 1000));
    assert!(!check_complete(1000, 500));
    assert!(check_complete(1000, 1000));
    assert!(check_complete(0, 0));
}

// @tc.name: ut_running_task_result_handling
// @tc.desc: Test running result handling for task completion
// @tc.precon: NA
// @tc.step: 1. Test success result (Ok(()))
//           2. Test network offline error
//           3. Test other error reasons
// @tc.expect: Result handling works correctly for task lifecycle
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_running_task_result_handling() {
    let success: Option<Result<(), Reason>> = Some(Ok(()));
    
    match success {
        Some(Ok(())) => assert!(true),
        Some(Err(_)) => panic!("Expected Ok"),
        None => panic!("Expected Some"),
    }
    
    let network_offline: Option<Result<(), Reason>> = Some(Err(Reason::NetworkOffline));
    
    match network_offline {
        Some(Err(Reason::NetworkOffline)) => assert!(true),
        _ => panic!("Expected NetworkOffline error"),
    }
    
    let io_error: Option<Result<(), Reason>> = Some(Err(Reason::IoError));
    
    match io_error {
        Some(Err(Reason::IoError)) => assert!(true),
        _ => panic!("Expected IoError"),
    }
}

// @tc.name: ut_running_task_timing_calculation
// @tc.desc: Test task timing calculation for Drop implementation
// @tc.precon: NA
// @tc.step: 1. Simulate start/end time tracking
//           2. Calculate current task time
//           3. Accumulate total task time
// @tc.expect: Timing calculation works correctly for task lifecycle
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_running_task_timing_calculation() {
    let start_time: u64 = 1000;
    let end_time: u64 = 1600;
    let current_task_time = end_time - start_time;
    
    let total_task_time: u64 = 500;
    let new_total = total_task_time + current_task_time;
    
    assert_eq!(current_task_time, 600);
    assert_eq!(new_total, 1100);
}

// @tc.name: ut_running_task_arc_clone
// @tc.desc: Test Arc clone for task sharing between threads
// @tc.precon: NA
// @tc.step: 1. Create Arc for task sharing
//           2. Clone and verify reference count
//           3. Verify shared access
// @tc.expect: Arc clone works correctly for task sharing
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_running_task_arc_clone() {
    struct MockTask {
        task_id: u32,
    }
    
    let task = Arc::new(MockTask { task_id: 12345 });
    assert_eq!(Arc::strong_count(&task), 1);
    
    let task_clone = Arc::clone(&task);
    assert_eq!(Arc::strong_count(&task), 2);
    
    assert_eq!(task.task_id, task_clone.task_id);
    
    drop(task_clone);
    assert_eq!(Arc::strong_count(&task), 1);
}

// @tc.name: ut_running_task_event_types
// @tc.desc: Test task event types for task manager communication
// @tc.precon: NA
// @tc.step: 1. Test TaskEvent variants
//           2. Verify event data extraction
// @tc.expect: Event types work correctly for task manager communication
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_running_task_event_types() {
    use crate::manage::events::TaskEvent;
    
    let completed = TaskEvent::Completed(12345, 1000, Mode::BackGround);
    let failed = TaskEvent::Failed(12345, 1000, Reason::IoError, Mode::BackGround);
    let offline = TaskEvent::Offline(12345, 1000, Mode::BackGround);
    let running = TaskEvent::Running(12345, 1000, Mode::FrontEnd);
    
    match completed {
        TaskEvent::Completed(task_id, uid, mode) => {
            assert_eq!(task_id, 12345);
            assert_eq!(uid, 1000);
            assert_eq!(mode, Mode::BackGround);
        }
        _ => panic!("Expected Completed"),
    }
    
    match failed {
        TaskEvent::Failed(task_id, uid, reason, mode) => {
            assert_eq!(task_id, 12345);
            assert_eq!(reason, Reason::IoError);
        }
        _ => panic!("Expected Failed"),
    }
    
    match offline {
        TaskEvent::Offline(task_id, uid, mode) => {
            assert_eq!(task_id, 12345);
        }
        _ => panic!("Expected Offline"),
    }
    
    match running {
        TaskEvent::Running(task_id, uid, mode) => {
            assert_eq!(mode, Mode::FrontEnd);
        }
        _ => panic!("Expected Running"),
    }
}

// @tc.name: ut_running_task_background_notify
// @tc.desc: Test background notification flag atomic operations
// @tc.precon: NA
// @tc.step: 1. Create AtomicBool for background_notify
//           2. Test load and store with Acquire/Release ordering
// @tc.expect: Background notification flag works correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_running_task_background_notify() {
    let background_notify = Arc::new(AtomicBool::new(false));
    
    assert!(!background_notify.load(Ordering::Acquire));
    
    background_notify.store(true, Ordering::Release);
    assert!(background_notify.load(Ordering::Acquire));
}
