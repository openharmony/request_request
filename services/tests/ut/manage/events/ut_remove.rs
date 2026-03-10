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

use std::collections::HashMap;

// @tc.name: ut_remove_task_count_decrement
// @tc.desc: Test task count decrement on remove
// @tc.precon: NA
// @tc.step: 1. Create task count map
//           2. Decrement count
// @tc.expect: Task count decrements correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_remove_task_count_decrement() {
    let mut task_count: HashMap<u64, (usize, usize)> = HashMap::new();
    task_count.insert(1000, (5, 3));
    
    if let Some(count) = task_count.get_mut(&1000) {
        if count.0 > 0 { count.0 -= 1; }
    }
    
    assert_eq!(task_count.get(&1000), Some(&(4, 3)));
}

// @tc.name: ut_remove_terminal_state_check
// @tc.desc: Test terminal state check for remove
// @tc.precon: NA
// @tc.step: 1. Define State enum
//           2. Check terminal states
// @tc.expect: Terminal states are correctly identified
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_remove_terminal_state_check() {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum State {
        Failed = 0x41,
        Completed = 0x40,
        Removed = 0x50,
        Running = 0x20,
    }
    
    fn is_terminal(state: State) -> bool {
        matches!(state, State::Failed | State::Completed | State::Removed)
    }
    
    assert!(is_terminal(State::Failed));
    assert!(is_terminal(State::Completed));
    assert!(is_terminal(State::Removed));
    assert!(!is_terminal(State::Running));
}

// @tc.name: ut_remove_error_code
// @tc.desc: Test remove error code return
// @tc.precon: NA
// @tc.step: 1. Define error codes
//           2. Simulate remove result
// @tc.expect: Error codes work correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_remove_error_code() {
    #[derive(Debug, PartialEq, Clone, Copy)]
    enum ErrorCode {
        ErrOk,
        TaskNotFound,
    }
    
    fn remove_task(_uid: u64, _task_id: u32, success: bool) -> ErrorCode {
        if success { ErrorCode::ErrOk } else { ErrorCode::TaskNotFound }
    }
    
    assert_eq!(remove_task(1000, 12345, true), ErrorCode::ErrOk);
    assert_eq!(remove_task(1000, 12345, false), ErrorCode::TaskNotFound);
}

// @tc.name: ut_remove_mode_selection
// @tc.desc: Test mode selection for counter decrement
// @tc.precon: NA
// @tc.step: 1. Create task count
//           2. Select counter based on mode
// @tc.expect: Mode selection works correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_remove_mode_selection() {
    let mut task_count: HashMap<u64, (usize, usize)> = HashMap::new();
    task_count.insert(1000, (5, 3));
    
    let mode: u8 = 1;
    if let Some(count) = task_count.get_mut(&1000) {
        let counter = match mode { 1 => &mut count.0, _ => &mut count.1 };
        if *counter > 0 { *counter -= 1; }
    }
    assert_eq!(task_count.get(&1000), Some(&(4, 3)));
    
    let mode: u8 = 2;
    if let Some(count) = task_count.get_mut(&1000) {
        let counter = match mode { 1 => &mut count.0, _ => &mut count.1 };
        if *counter > 0 { *counter -= 1; }
    }
    assert_eq!(task_count.get(&1000), Some(&(4, 2)));
}
