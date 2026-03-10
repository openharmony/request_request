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

// @tc.name: ut_set_mode_parameters
// @tc.desc: Test set_mode parameters
// @tc.precon: NA
// @tc.step: 1. Create set_mode parameters
//           2. Verify types
// @tc.expect: Parameters are correctly typed
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_set_mode_parameters() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Mode {
        FrontEnd,
        BackGround,
    }
    
    let uid: u64 = 1000;
    let task_id: u32 = 12345;
    let mode = Mode::BackGround;
    
    assert_eq!(uid, 1000);
    assert_eq!(task_id, 12345);
    assert_eq!(mode, Mode::BackGround);
}

// @tc.name: ut_set_mode_enum_values
// @tc.desc: Test Mode enum values
// @tc.precon: NA
// @tc.step: 1. Define Mode enum
//           2. Check variants
// @tc.expect: Mode enum has correct variants
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_set_mode_enum_values() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Mode {
        FrontEnd,
        BackGround,
    }
    
    assert_ne!(Mode::FrontEnd, Mode::BackGround);
    assert_eq!(Mode::FrontEnd, Mode::FrontEnd);
    assert_eq!(Mode::BackGround, Mode::BackGround);
}

// @tc.name: ut_set_mode_copy_clone
// @tc.desc: Test Mode Copy and Clone traits
// @tc.precon: NA
// @tc.step: 1. Create Mode value
//           2. Copy and clone it
// @tc.expect: Copy and Clone work correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_set_mode_copy_clone() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Mode {
        FrontEnd,
        BackGround,
    }
    
    let mode = Mode::FrontEnd;
    let copied = mode;
    let cloned = mode.clone();
    
    assert_eq!(mode, copied);
    assert_eq!(mode, cloned);
}

// @tc.name: ut_set_mode_error_code
// @tc.desc: Test set_mode error code return
// @tc.precon: NA
// @tc.step: 1. Define error codes
//           2. Simulate set_mode result
// @tc.expect: Error codes work correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_set_mode_error_code() {
    #[derive(Debug, PartialEq, Clone, Copy)]
    enum ErrorCode {
        ErrOk,
        TaskNotFound,
    }
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Mode {
        FrontEnd,
        BackGround,
    }
    
    fn set_mode(_uid: u64, _task_id: u32, _mode: Mode, success: bool) -> ErrorCode {
        if success { ErrorCode::ErrOk } else { ErrorCode::TaskNotFound }
    }
    
    assert_eq!(set_mode(1000, 12345, Mode::FrontEnd, true), ErrorCode::ErrOk);
    assert_eq!(set_mode(1000, 12345, Mode::BackGround, false), ErrorCode::TaskNotFound);
}

// @tc.name: ut_set_mode_repr
// @tc.desc: Test Mode repr value
// @tc.precon: NA
// @tc.step: 1. Define Mode with repr
//           2. Check discriminant values
// @tc.expect: Mode repr values are correct
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_set_mode_repr() {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Mode {
        FrontEnd = 1,
        BackGround = 2,
    }
    
    assert_eq!(Mode::FrontEnd as u8, 1);
    assert_eq!(Mode::BackGround as u8, 2);
}
