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

//! Unit tests for the exception error code constants.
//!
//! This module tests the error code definitions used throughout the
//! request ANI implementation.

use request_ani::constant::ExceptionErrorCode;

// @tc.name: ut_error_code_e_ok
// @tc.desc: Test that E_OK error code has value 0
// @tc.precon: NA
// @tc.step: 1. Verify E_OK variant has value 0
// @tc.expect: E_OK should equal 0
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_error_code_e_ok() {
    assert_eq!(ExceptionErrorCode::E_OK as i32, 0);
}

// @tc.name: ut_error_code_permission
// @tc.desc: Test permission-related error codes
// @tc.precon: NA
// @tc.step: 1. Verify E_PERMISSION error code value
//           2. Verify E_NOT_SYSTEM_APP error code value
// @tc.expect: E_PERMISSION should be 201, E_NOT_SYSTEM_APP should be 202
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_error_code_permission() {
    assert_eq!(ExceptionErrorCode::E_PERMISSION as i32, 201);
    assert_eq!(ExceptionErrorCode::E_NOT_SYSTEM_APP as i32, 202);
}

// @tc.name: ut_error_code_parameter
// @tc.desc: Test parameter check error code
// @tc.precon: NA
// @tc.step: 1. Verify E_PARAMETER_CHECK error code value
// @tc.expect: E_PARAMETER_CHECK should be 401
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_error_code_parameter() {
    assert_eq!(ExceptionErrorCode::E_PARAMETER_CHECK as i32, 401);
}

// @tc.name: ut_error_code_unsupported
// @tc.desc: Test unsupported feature error code
// @tc.precon: NA
// @tc.step: 1. Verify E_UNSUPPORTED error code value
// @tc.expect: E_UNSUPPORTED should be 801
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_error_code_unsupported() {
    assert_eq!(ExceptionErrorCode::E_UNSUPPORTED as i32, 801);
}

// @tc.name: ut_error_code_file_errors
// @tc.desc: Test file-related error codes
// @tc.precon: NA
// @tc.step: 1. Verify E_FILE_IO error code value
//           2. Verify E_FILE_PATH error code value
// @tc.expect: E_FILE_IO should be 13400001, E_FILE_PATH should be 13400002
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_error_code_file_errors() {
    assert_eq!(ExceptionErrorCode::E_FILE_IO as i32, 13400001);
    assert_eq!(ExceptionErrorCode::E_FILE_PATH as i32, 13400002);
}

// @tc.name: ut_error_code_service
// @tc.desc: Test service-related error codes
// @tc.precon: NA
// @tc.step: 1. Verify E_SERVICE_ERROR error code value
//           2. Verify E_OTHER error code value
// @tc.expect: E_SERVICE_ERROR should be 13400003, E_OTHER should be 13499999
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_error_code_service() {
    assert_eq!(ExceptionErrorCode::E_SERVICE_ERROR as i32, 13400003);
    assert_eq!(ExceptionErrorCode::E_OTHER as i32, 13499999);
}

// @tc.name: ut_error_code_task_errors
// @tc.desc: Test task-related error codes
// @tc.precon: NA
// @tc.step: 1. Verify task queue error code
//           2. Verify task mode error code
//           3. Verify task not found error code
//           4. Verify task state error code
//           5. Verify group not found error code
// @tc.expect: Task error codes should have expected values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_error_code_task_errors() {
    assert_eq!(ExceptionErrorCode::E_TASK_QUEUE as i32, 21900004);
    assert_eq!(ExceptionErrorCode::E_TASK_MODE as i32, 21900005);
    assert_eq!(ExceptionErrorCode::E_TASK_NOT_FOUND as i32, 21900006);
    assert_eq!(ExceptionErrorCode::E_TASK_STATE as i32, 21900007);
    assert_eq!(ExceptionErrorCode::E_GROUP_NOT_FOUND as i32, 21900008);
}

// @tc.name: ut_error_code_ipc_errors
// @tc.desc: Test IPC-related error codes
// @tc.precon: NA
// @tc.step: 1. Verify E_UNLOADING_SA error code
//           2. Verify E_IPC_SIZE_TOO_LARGE error code
// @tc.expect: IPC error codes should have expected values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_error_code_ipc_errors() {
    assert_eq!(ExceptionErrorCode::E_UNLOADING_SA as i32, 1);
    assert_eq!(ExceptionErrorCode::E_IPC_SIZE_TOO_LARGE as i32, 2);
}

// @tc.name: ut_error_code_channel
// @tc.desc: Test channel error code
// @tc.precon: NA
// @tc.step: 1. Verify E_CHANNEL_NOT_OPEN error code value
// @tc.expect: E_CHANNEL_NOT_OPEN should be 5
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_error_code_channel() {
    assert_eq!(ExceptionErrorCode::E_CHANNEL_NOT_OPEN as i32, 5);
}

// @tc.name: ut_error_code_all_variants
// @tc.desc: Test all error code variants are unique
// @tc.precon: NA
// @tc.step: 1. Collect all error code values
//           2. Verify all values are unique
// @tc.expect: All error codes should have distinct values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_error_code_all_variants() {
    use std::collections::HashSet;

    let codes = vec![
        ExceptionErrorCode::E_OK as i32,
        ExceptionErrorCode::E_UNLOADING_SA as i32,
        ExceptionErrorCode::E_IPC_SIZE_TOO_LARGE as i32,
        ExceptionErrorCode::E_MIMETYPE_NOT_FOUND as i32,
        ExceptionErrorCode::E_TASK_INDEX_TOO_LARGE as i32,
        ExceptionErrorCode::E_CHANNEL_NOT_OPEN as i32,
        ExceptionErrorCode::E_PERMISSION as i32,
        ExceptionErrorCode::E_NOT_SYSTEM_APP as i32,
        ExceptionErrorCode::E_PARAMETER_CHECK as i32,
        ExceptionErrorCode::E_UNSUPPORTED as i32,
        ExceptionErrorCode::E_FILE_IO as i32,
        ExceptionErrorCode::E_FILE_PATH as i32,
        ExceptionErrorCode::E_SERVICE_ERROR as i32,
        ExceptionErrorCode::E_OTHER as i32,
        ExceptionErrorCode::E_TASK_QUEUE as i32,
        ExceptionErrorCode::E_TASK_MODE as i32,
        ExceptionErrorCode::E_TASK_NOT_FOUND as i32,
        ExceptionErrorCode::E_TASK_STATE as i32,
        ExceptionErrorCode::E_GROUP_NOT_FOUND as i32,
    ];

    let unique_codes: HashSet<_> = codes.iter().cloned().collect();
    assert_eq!(
        codes.len(),
        unique_codes.len(),
        "Duplicate error code values found"
    );
}

// @tc.name: ut_error_code_categorization
// @tc.desc: Test error code value ranges by category
// @tc.precon: NA
// @tc.step: 1. Verify success codes are 0
//           2. Verify permission codes are in 200 range
//           3. Verify parameter codes are in 400 range
//           4. Verify unsupported codes are in 800 range
//           5. Verify file errors are in 13400000 range
//           6. Verify task errors are in 21900000 range
// @tc.expect: Error codes should be in expected ranges
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_error_code_categorization() {
    // Success codes
    assert_eq!(ExceptionErrorCode::E_OK as i32, 0);

    // Permission codes (200 range)
    let perm_code = ExceptionErrorCode::E_PERMISSION as i32;
    assert!(perm_code >= 200 && perm_code < 300);

    // Parameter codes (400 range)
    let param_code = ExceptionErrorCode::E_PARAMETER_CHECK as i32;
    assert!(param_code >= 400 && param_code < 500);

    // Unsupported codes (800 range)
    let unsup_code = ExceptionErrorCode::E_UNSUPPORTED as i32;
    assert!(unsup_code >= 800 && unsup_code < 900);

    // File errors (13400000 range)
    let file_io_code = ExceptionErrorCode::E_FILE_IO as i32;
    assert!(file_io_code >= 13400000 && file_io_code < 13410000);

    // Task errors (21900000 range)
    let task_queue_code = ExceptionErrorCode::E_TASK_QUEUE as i32;
    assert!(task_queue_code >= 21900000 && task_queue_code < 21910000);
}

// @tc.name: ut_error_code_debug_trait
// @tc.desc: Test that ExceptionErrorCode implements Debug trait
// @tc.precon: NA
// @tc.step: 1. Format error code using Debug trait
//           2. Verify output contains variant name
// @tc.expect: Debug output should contain variant name
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_error_code_debug_trait() {
    let code = ExceptionErrorCode::E_PERMISSION;
    let debug_str = format!("{:?}", code);
    assert!(debug_str.contains("E_PERMISSION"));
}

// @tc.name: ut_error_code_clone_trait
// @tc.desc: Test that ExceptionErrorCode implements Clone trait
// @tc.precon: NA
// @tc.step: 1. Clone an error code
//           2. Verify cloned value equals original
// @tc.expect: Cloned value should equal original
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_error_code_clone_trait() {
    let code = ExceptionErrorCode::E_TASK_NOT_FOUND;
    let cloned = code.clone();
    assert_eq!(code as i32, cloned as i32);
}

// @tc.name: ut_error_code_copy_trait
// @tc.desc: Test that ExceptionErrorCode implements Copy trait
// @tc.precon: NA
// @tc.step: 1. Copy an error code to a new variable
//           2. Verify both variables have same value
// @tc.expect: Both variables should have same value
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_error_code_copy_trait() {
    let code = ExceptionErrorCode::E_SERVICE_ERROR;
    let copied = code;
    assert_eq!(code as i32, copied as i32);
    // Original should still be valid (Copy trait)
    let _ = code;
}

// @tc.name: ut_error_code_eq_trait
// @tc.desc: Test that ExceptionErrorCode implements PartialEq trait
// @tc.precon: NA
// @tc.step: 1. Compare equal error codes
//           2. Compare different error codes
// @tc.expect: Equal codes should compare equal, different codes should not
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_error_code_eq_trait() {
    let code1 = ExceptionErrorCode::E_PERMISSION;
    let code2 = ExceptionErrorCode::E_PERMISSION;
    let code3 = ExceptionErrorCode::E_PARAMETER_CHECK;

    assert_eq!(code1, code2);
    assert_ne!(code1, code3);
}
