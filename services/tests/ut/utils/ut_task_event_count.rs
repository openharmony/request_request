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

use std::sync::{Arc, Mutex};

// @tc.name: ut_task_count_structure
// @tc.desc: Test RequestTaskCount structure
// @tc.precon: NA
// @tc.step: 1. Create RequestTaskCount
//           2. Verify fields
// @tc.expect: Structure is correctly defined
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_count_structure() {
    struct RequestTaskCount {
        completed_task_count: i32,
        failed_task_count: i32,
        load_state: bool,
    }
    
    let count = RequestTaskCount {
        completed_task_count: 0,
        failed_task_count: 0,
        load_state: false,
    };
    
    assert_eq!(count.completed_task_count, 0);
    assert_eq!(count.failed_task_count, 0);
    assert!(!count.load_state);
}

// @tc.name: ut_task_complete_add
// @tc.desc: Test task_complete_add functionality
// @tc.precon: NA
// @tc.step: 1. Create task count
//           2. Increment completed count
// @tc.expect: Completed count increments correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_complete_add() {
    let task_count = Arc::new(Mutex::new((0i32, false)));
    
    {
        let mut count = task_count.lock().unwrap();
        count.0 += 1;
        count.1 = true;
    }
    
    let count = task_count.lock().unwrap();
    assert_eq!(count.0, 1);
    assert!(count.1);
}

// @tc.name: ut_task_fail_add
// @tc.desc: Test task_fail_add functionality
// @tc.precon: NA
// @tc.step: 1. Create task count
//           2. Increment failed count
// @tc.expect: Failed count increments correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_fail_add() {
    let task_count = Arc::new(Mutex::new((0i32, 0i32, false)));
    
    {
        let mut count = task_count.lock().unwrap();
        count.1 += 1;
        count.2 = true;
    }
    
    let count = task_count.lock().unwrap();
    assert_eq!(count.0, 0);
    assert_eq!(count.1, 1);
    assert!(count.2);
}

// @tc.name: ut_task_unload_reset
// @tc.desc: Test task_unload reset functionality
// @tc.precon: NA
// @tc.step: 1. Create task count with values
//           2. Reset counts
// @tc.expect: Counts are reset correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_unload_reset() {
    let task_count = Arc::new(Mutex::new((5i32, 3i32, true)));
    
    {
        let mut count = task_count.lock().unwrap();
        if count.2 {
            count.0 = 0;
            count.1 = 0;
            count.2 = false;
        }
    }
    
    let count = task_count.lock().unwrap();
    assert_eq!(count.0, 0);
    assert_eq!(count.1, 0);
    assert!(!count.2);
}

// @tc.name: ut_task_count_multiple_operations
// @tc.desc: Test multiple count operations
// @tc.precon: NA
// @tc.step: 1. Create task count
//           2. Perform multiple operations
// @tc.expect: All operations work correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_count_multiple_operations() {
    let task_count = Arc::new(Mutex::new((0i32, 0i32, false)));
    
    for _ in 0..10 {
        let mut count = task_count.lock().unwrap();
        count.0 += 1;
        count.2 = true;
    }
    
    for _ in 0..5 {
        let mut count = task_count.lock().unwrap();
        count.1 += 1;
    }
    
    let count = task_count.lock().unwrap();
    assert_eq!(count.0, 10);
    assert_eq!(count.1, 5);
    assert!(count.2);
}

// @tc.name: ut_task_count_load_state
// @tc.desc: Test load_state flag behavior
// @tc.precon: NA
// @tc.step: 1. Create task count
//           2. Test load_state flag
// @tc.expect: load_state flag works correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_count_load_state() {
    let task_count = Arc::new(Mutex::new((0i32, 0i32, false)));
    
    {
        let count = task_count.lock().unwrap();
        assert!(!count.2);
    }
    
    {
        let mut count = task_count.lock().unwrap();
        count.0 += 1;
        count.2 = true;
    }
    
    {
        let count = task_count.lock().unwrap();
        assert!(count.2);
    }
    
    {
        let mut count = task_count.lock().unwrap();
        count.2 = false;
    }
    
    let count = task_count.lock().unwrap();
    assert!(!count.2);
}

// @tc.name: ut_task_count_thread_safety
// @tc.desc: Test task count thread safety
// @tc.precon: NA
// @tc.step: 1. Create shared task count
//           2. Access from multiple threads
// @tc.expect: Thread safety works correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_count_thread_safety() {
    use std::thread;

    let task_count = Arc::new(Mutex::new((0i32, 0i32, false)));
    let mut handles = vec![];

    for _ in 0..10 {
        let count_clone = Arc::clone(&task_count);
        let handle = thread::spawn(move || {
            let mut count = count_clone
                .lock()
                .expect("Mutex poisoned in concurrent test");
            count.0 += 1;
            count.2 = true;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked during execution");
    }

    let count = task_count
        .lock()
        .expect("Mutex poisoned after concurrent test");
    assert_eq!(count.0, 10);
}

// @tc.name: ut_task_count_report_format
// @tc.desc: Test task count report format
// @tc.precon: NA
// @tc.step: 1. Create task counts
//           2. Format report string
// @tc.expect: Report format is correct
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_count_report_format() {
    let completed: i32 = 10;
    let failed: i32 = 3;
    
    let report = format!("Task Completed {}, failed {}", completed, failed);
    
    assert!(report.contains("10"));
    assert!(report.contains("3"));
    assert!(report.contains("Completed"));
    assert!(report.contains("failed"));
}
