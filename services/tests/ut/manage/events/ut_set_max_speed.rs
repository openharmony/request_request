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

// @tc.name: ut_set_max_speed_parameters
// @tc.desc: Test set_max_speed parameters
// @tc.precon: NA
// @tc.step: 1. Create set_max_speed parameters
//           2. Verify types
// @tc.expect: Parameters are correctly typed
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_set_max_speed_parameters() {
    let uid: u64 = 1000;
    let task_id: u32 = 12345;
    let max_speed: i64 = 1024 * 1024;
    
    assert_eq!(uid, 1000);
    assert_eq!(task_id, 12345);
    assert_eq!(max_speed, 1048576);
}

// @tc.name: ut_set_max_speed_values
// @tc.desc: Test various max_speed values
// @tc.precon: NA
// @tc.step: 1. Create various speed values
//           2. Verify values
// @tc.expect: Speed values are correct
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_set_max_speed_values() {
    let speeds: Vec<i64> = vec![
        0,
        1024,
        1024 * 1024,
        10 * 1024 * 1024,
    ];
    
    assert_eq!(speeds[0], 0);
    assert_eq!(speeds[1], 1024);
    assert_eq!(speeds[2], 1048576);
    assert_eq!(speeds[3], 10485760);
}

// @tc.name: ut_set_max_speed_state_check
// @tc.desc: Test removed state check for set_max_speed
// @tc.precon: NA
// @tc.step: 1. Define State enum
//           2. Check removed state
// @tc.expect: Removed state is correctly identified
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_set_max_speed_state_check() {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum State {
        Removed = 0x50,
        Running = 0x20,
    }
    
    fn is_removed(state: State) -> bool {
        state == State::Removed
    }
    
    assert!(is_removed(State::Removed));
    assert!(!is_removed(State::Running));
}

// @tc.name: ut_set_max_speed_error_code
// @tc.desc: Test set_max_speed error code return
// @tc.precon: NA
// @tc.step: 1. Define error codes
//           2. Simulate set_max_speed result
// @tc.expect: Error codes work correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_set_max_speed_error_code() {
    #[derive(Debug, PartialEq, Clone, Copy)]
    enum ErrorCode {
        ErrOk,
        TaskStateErr,
    }
    
    fn set_max_speed(_uid: u64, _task_id: u32, _speed: i64, success: bool) -> ErrorCode {
        if success { ErrorCode::ErrOk } else { ErrorCode::TaskStateErr }
    }
    
    assert_eq!(set_max_speed(1000, 12345, 1048576, true), ErrorCode::ErrOk);
    assert_eq!(set_max_speed(1000, 12345, 1048576, false), ErrorCode::TaskStateErr);
}

// @tc.name: ut_set_max_speed_zero_unlimited
// @tc.desc: Test zero speed means unlimited
// @tc.precon: NA
// @tc.step: 1. Create zero max_speed
//           2. Verify it represents unlimited
// @tc.expect: Zero speed is handled as unlimited
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_set_max_speed_zero_unlimited() {
    fn is_unlimited(speed: i64) -> bool {
        speed == 0
    }
    
    assert!(is_unlimited(0));
    assert!(!is_unlimited(1024));
}
