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

use request_client::client::error::CreateTaskError;
use request_client::check::file::DownloadPathError;

// @tc.name: ut_create_task_error_from_download_path_error
// @tc.desc: Test CreateTaskError conversion from DownloadPathError
// @tc.precon: NA
// @tc.step: 1. Create DownloadPathError::EmptyPath
//           2. Convert to CreateTaskError using From trait
//           3. Verify conversion result
// @tc.expect: CreateTaskError::DownloadPath variant is created
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_create_task_error_from_download_path_error() {
    let path_error = DownloadPathError::EmptyPath;
    let task_error: CreateTaskError = path_error.into();
    
    match task_error {
        CreateTaskError::DownloadPath(err) => {
            match err {
                DownloadPathError::EmptyPath => {}
                _ => panic!("Expected EmptyPath variant"),
            }
        }
        _ => panic!("Expected DownloadPath variant"),
    }
}

// @tc.name: ut_create_task_error_from_i32
// @tc.desc: Test CreateTaskError conversion from i32
// @tc.precon: NA
// @tc.step: 1. Create i32 error code
//           2. Convert to CreateTaskError using From trait
//           3. Verify conversion result
// @tc.expect: CreateTaskError::Code variant is created with correct code
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_create_task_error_from_i32() {
    let error_code: i32 = -1;
    let task_error: CreateTaskError = error_code.into();
    
    match task_error {
        CreateTaskError::Code(code) => assert_eq!(code, -1),
        _ => panic!("Expected Code variant"),
    }
}

// @tc.name: ut_create_task_error_debug_code
// @tc.desc: Test Debug trait for CreateTaskError::Code
// @tc.precon: NA
// @tc.step: 1. Create CreateTaskError::Code variant
//           2. Format with Debug trait
//           3. Verify output contains expected content
// @tc.expect: Debug output contains "Code"
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_create_task_error_debug_code() {
    let error = CreateTaskError::Code(123);
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("Code"));
    assert!(debug_str.contains("123"));
}

// @tc.name: ut_create_task_error_debug_download_path
// @tc.desc: Test Debug trait for CreateTaskError::DownloadPath
// @tc.precon: NA
// @tc.step: 1. Create CreateTaskError::DownloadPath variant
//           2. Format with Debug trait
//           3. Verify output contains expected content
// @tc.expect: Debug output contains "DownloadPath"
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_create_task_error_debug_download_path() {
    let error = CreateTaskError::DownloadPath(DownloadPathError::TooLongPath);
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("DownloadPath"));
    assert!(debug_str.contains("TooLongPath"));
}

// @tc.name: ut_create_task_error_from_download_path_invalid
// @tc.desc: Test CreateTaskError conversion from DownloadPathError::InvalidPath
// @tc.precon: NA
// @tc.step: 1. Create DownloadPathError::InvalidPath
//           2. Convert to CreateTaskError
//           3. Verify correct variant
// @tc.expect: CreateTaskError::DownloadPath contains InvalidPath
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_create_task_error_from_download_path_invalid() {
    let path_error = DownloadPathError::InvalidPath;
    let task_error: CreateTaskError = path_error.into();
    
    match task_error {
        CreateTaskError::DownloadPath(DownloadPathError::InvalidPath) => {}
        _ => panic!("Expected DownloadPath(InvalidPath) variant"),
    }
}

// @tc.name: ut_create_task_error_from_download_path_already_exists
// @tc.desc: Test CreateTaskError conversion from DownloadPathError::AlreadyExists
// @tc.precon: NA
// @tc.step: 1. Create DownloadPathError::AlreadyExists
//           2. Convert to CreateTaskError
//           3. Verify correct variant
// @tc.expect: CreateTaskError::DownloadPath contains AlreadyExists
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_create_task_error_from_download_path_already_exists() {
    let path_error = DownloadPathError::AlreadyExists;
    let task_error: CreateTaskError = path_error.into();
    
    match task_error {
        CreateTaskError::DownloadPath(DownloadPathError::AlreadyExists) => {}
        _ => panic!("Expected DownloadPath(AlreadyExists) variant"),
    }
}

// @tc.name: ut_create_task_error_from_download_path_bundle_name
// @tc.desc: Test CreateTaskError conversion from DownloadPathError::BundleNameNotMap
// @tc.precon: NA
// @tc.step: 1. Create DownloadPathError::BundleNameNotMap
//           2. Convert to CreateTaskError
//           3. Verify correct variant
// @tc.expect: CreateTaskError::DownloadPath contains BundleNameNotMap
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_create_task_error_from_download_path_bundle_name() {
    let path_error = DownloadPathError::BundleNameNotMap;
    let task_error: CreateTaskError = path_error.into();
    
    match task_error {
        CreateTaskError::DownloadPath(DownloadPathError::BundleNameNotMap) => {}
        _ => panic!("Expected DownloadPath(BundleNameNotMap) variant"),
    }
}

// @tc.name: ut_create_task_error_from_i32_zero
// @tc.desc: Test CreateTaskError conversion from i32 zero
// @tc.precon: NA
// @tc.step: 1. Create i32 zero error code
//           2. Convert to CreateTaskError
//           3. Verify correct code
// @tc.expect: CreateTaskError::Code contains 0
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_create_task_error_from_i32_zero() {
    let error_code: i32 = 0;
    let task_error: CreateTaskError = error_code.into();
    
    match task_error {
        CreateTaskError::Code(code) => assert_eq!(code, 0),
        _ => panic!("Expected Code variant"),
    }
}

// @tc.name: ut_create_task_error_from_i32_positive
// @tc.desc: Test CreateTaskError conversion from positive i32
// @tc.precon: NA
// @tc.step: 1. Create positive i32 error code
//           2. Convert to CreateTaskError
//           3. Verify correct code
// @tc.expect: CreateTaskError::Code contains positive value
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_create_task_error_from_i32_positive() {
    let error_code: i32 = 100;
    let task_error: CreateTaskError = error_code.into();
    
    match task_error {
        CreateTaskError::Code(code) => assert_eq!(code, 100),
        _ => panic!("Expected Code variant"),
    }
}

// @tc.name: ut_create_task_error_multiple_conversions
// @tc.desc: Test multiple error conversions in sequence
// @tc.precon: NA
// @tc.step: 1. Create multiple different errors
//           2. Convert each to CreateTaskError
//           3. Verify each conversion
// @tc.expect: All conversions produce correct variants
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_create_task_error_multiple_conversions() {
    let errors: Vec<CreateTaskError> = vec![
        DownloadPathError::EmptyPath.into(),
        DownloadPathError::TooLongPath.into(),
        DownloadPathError::InvalidPath.into(),
        (-1_i32).into(),
        (0_i32).into(),
        (1_i32).into(),
    ];
    
    assert_eq!(errors.len(), 6);
    
    match &errors[0] {
        CreateTaskError::DownloadPath(DownloadPathError::EmptyPath) => {}
        _ => panic!("Expected EmptyPath"),
    }
    
    match &errors[3] {
        CreateTaskError::Code(-1) => {}
        _ => panic!("Expected Code(-1)"),
    }
}
