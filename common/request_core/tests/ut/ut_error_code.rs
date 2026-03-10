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

use request_core::error_code::*;

// @tc.name: ut_error_code_constants
// @tc.desc: Test all error code constants have expected values
// @tc.precon: NA
// @tc.step: 1. Check each error code constant
//           2. Verify expected values
// @tc.expect: All constants have correct values
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_error_code_constants() {
    assert_eq!(EXCEPTION_SERVICE, 13400003);
    assert_eq!(ERR_OK, 0);
    assert_eq!(IPC_SIZE_TOO_LARGE, 2);
    assert_eq!(CHANNEL_NOT_OPEN, 5);
    assert_eq!(PERMISSION, 201);
    assert_eq!(SYSTEM_API, 202);
    assert_eq!(PARAMETER_CHECK, 401);
    assert_eq!(FILE_OPERATION_ERR, 13400001);
    assert_eq!(OTHER, 13499999);
    assert_eq!(TASK_ENQUEUE_ERR, 21900004);
    assert_eq!(TASK_MODE_ERR, 21900005);
    assert_eq!(TASK_NOT_FOUND, 21900006);
    assert_eq!(TASK_STATE_ERR, 21900007);
    assert_eq!(GROUP_NOT_FOUND, 21900008);
}

// @tc.name: ut_error_code_distinct_values
// @tc.desc: Test all error codes have distinct values
// @tc.precon: NA
// @tc.step: 1. Collect all error codes
//           2. Verify they are all distinct
// @tc.expect: All error codes have unique values
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_error_code_distinct_values() {
    let codes = [
        EXCEPTION_SERVICE,
        ERR_OK,
        IPC_SIZE_TOO_LARGE,
        CHANNEL_NOT_OPEN,
        PERMISSION,
        SYSTEM_API,
        PARAMETER_CHECK,
        FILE_OPERATION_ERR,
        OTHER,
        TASK_ENQUEUE_ERR,
        TASK_MODE_ERR,
        TASK_NOT_FOUND,
        TASK_STATE_ERR,
        GROUP_NOT_FOUND,
    ];
    
    for i in 0..codes.len() {
        for j in (i + 1)..codes.len() {
            assert_ne!(codes[i], codes[j], "Error codes should be distinct");
        }
    }
}

// @tc.name: ut_error_code_positive_or_zero
// @tc.desc: Test all error codes are positive or zero
// @tc.precon: NA
// @tc.step: 1. Check each error code
//           2. Verify it is >= 0
// @tc.expect: All error codes are non-negative
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_error_code_positive_or_zero() {
    assert!(ERR_OK >= 0);
    assert!(EXCEPTION_SERVICE >= 0);
    assert!(IPC_SIZE_TOO_LARGE >= 0);
    assert!(CHANNEL_NOT_OPEN >= 0);
    assert!(PERMISSION >= 0);
    assert!(SYSTEM_API >= 0);
    assert!(PARAMETER_CHECK >= 0);
    assert!(FILE_OPERATION_ERR >= 0);
    assert!(OTHER >= 0);
    assert!(TASK_ENQUEUE_ERR >= 0);
    assert!(TASK_MODE_ERR >= 0);
    assert!(TASK_NOT_FOUND >= 0);
    assert!(TASK_STATE_ERR >= 0);
    assert!(GROUP_NOT_FOUND >= 0);
}

// @tc.name: ut_error_code_task_errors_range
// @tc.desc: Test task-related error codes are in expected range
// @tc.precon: NA
// @tc.step: 1. Check task error codes
//           2. Verify they start with 219 prefix
// @tc.expect: Task errors are in 21900000 range
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_error_code_task_errors_range() {
    let task_errors = [TASK_ENQUEUE_ERR, TASK_MODE_ERR, TASK_NOT_FOUND, TASK_STATE_ERR, GROUP_NOT_FOUND];
    
    for code in task_errors {
        assert!(code >= 21900000 && code < 22000000, "Task error should be in 219xxxxx range");
    }
}

// @tc.name: ut_error_code_file_operation_range
// @tc.desc: Test file operation error codes are in expected range
// @tc.precon: NA
// @tc.step: 1. Check file operation error codes
//           2. Verify they start with 134 prefix
// @tc.expect: File errors are in 13400000 range
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_error_code_file_operation_range() {
    assert!(FILE_OPERATION_ERR >= 13400000 && FILE_OPERATION_ERR < 13500000);
    assert!(EXCEPTION_SERVICE >= 13400000 && EXCEPTION_SERVICE < 13500000);
    assert!(OTHER >= 13400000 && OTHER < 13500000);
}
