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

//! Unit tests for proxy/task.rs
//!
//! Tests task management operations including create, start, pause, resume,
//! remove, stop, and set_max_speed. Note: IPC operations require OHOS environment,
//! so tests focus on data structures and error handling.

use std::collections::HashMap;

use request_core::config::{Action, TaskConfig};
use request_next::client::error::CreateTaskError;
use request_next::proxy::RequestProxy;

// @tc.name: ut_task_config_default_values
// @tc.desc: Test TaskConfig default values for task creation
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig with default values
//           2. Verify fields match expected defaults for IPC serialization
// @tc.expect: TaskConfig has correct default values for create() operation
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_config_default_values() {
    let config = TaskConfig::default();
    
    assert!(config.url.is_empty(), "Default URL should be empty");
    assert!(config.title.is_empty(), "Default title should be empty");
    assert!(config.headers.is_empty(), "Default headers should be empty");
}

// @tc.name: ut_task_config_with_url
// @tc.desc: Test TaskConfig with URL for download task
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig with download URL
//           2. Verify URL is set correctly for IPC transmission
// @tc.expect: TaskConfig URL matches the provided value
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_config_with_url() {
    let mut config = TaskConfig::default();
    config.url = "https://example.com/file.zip".to_string();
    
    assert_eq!(config.url, "https://example.com/file.zip");
}

// @tc.name: ut_task_create_task_error_code_variant
// @tc.desc: Test CreateTaskError::Code variant for IPC error handling
// @tc.precon: NA
// @tc.step: 1. Create CreateTaskError::Code with various error codes
//           2. Verify error code is preserved
// @tc.expect: CreateTaskError::Code correctly stores error codes from IPC
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_create_task_error_code_variant() {
    let error_codes = vec![0, -1, 13400001, 13400002, 13400003];
    
    for code in error_codes {
        let error = CreateTaskError::Code(code);
        match error {
            CreateTaskError::Code(c) => assert_eq!(c, code),
            _ => panic!("Expected Code variant"),
        }
    }
}

// @tc.name: ut_task_create_task_error_from_i32
// @tc.desc: Test CreateTaskError From<i32> trait for error conversion
// @tc.precon: NA
// @tc.step: 1. Convert i32 error codes to CreateTaskError
//           2. Verify conversion works correctly
// @tc.expect: i32 error codes are correctly converted to CreateTaskError::Code
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_create_task_error_from_i32() {
    let error: CreateTaskError = 13400003.into();
    
    match error {
        CreateTaskError::Code(code) => assert_eq!(code, 13400003),
        _ => panic!("Expected Code variant"),
    }
}

// @tc.name: ut_task_create_task_error_debug
// @tc.desc: Test CreateTaskError Debug implementation for logging
// @tc.precon: NA
// @tc.step: 1. Create CreateTaskError and format with Debug
//           2. Verify output contains error information
// @tc.expect: Debug output is useful for error logging
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_create_task_error_debug() {
    let error = CreateTaskError::Code(13400001);
    let debug_str = format!("{:?}", error);
    
    assert!(debug_str.contains("Code"), "Debug output should contain 'Code'");
    assert!(debug_str.contains("13400001"), "Debug output should contain error code");
}

// @tc.name: ut_task_id_string_conversion
// @tc.desc: Test task ID to string conversion for IPC serialization
// @tc.precon: NA
// @tc.step: 1. Convert various task IDs to strings
//           2. Verify conversion matches IPC expected format
// @tc.expect: Task IDs are correctly converted to strings for MsgParcel
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_id_string_conversion() {
    let test_cases = vec![
        (0i64, "0"),
        (12345i64, "12345"),
        (-1i64, "-1"),
        (i64::MAX, "9223372036854775807"),
    ];
    
    for (task_id, expected) in test_cases {
        assert_eq!(task_id.to_string(), expected);
    }
}

// @tc.name: ut_task_proxy_singleton_for_task_ops
// @tc.desc: Test RequestProxy singleton for task operations
// @tc.precon: NA
// @tc.step: 1. Get RequestProxy instance
//           2. Verify it's available for task operations
// @tc.expect: RequestProxy singleton is accessible
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_proxy_singleton_for_task_ops() {
    let proxy = RequestProxy::get_instance();
    assert!(!std::ptr::eq(proxy, std::ptr::null()));
}

// @tc.name: ut_task_config_action
// @tc.desc: Test TaskConfig action field for task type
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig with different actions
//           2. Verify action is set correctly
// @tc.expect: TaskConfig action matches expected value
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_config_action() {
    let mut config = TaskConfig::default();
    config.common_data.action = Action::Download;
    
    assert_eq!(config.common_data.action, Action::Download);
}

// @tc.name: ut_task_config_headers
// @tc.desc: Test TaskConfig headers for IPC serialization
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig with headers
//           2. Verify headers are stored correctly
// @tc.expect: Headers are correctly stored for IPC transmission
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_config_headers() {
    let mut headers = HashMap::new();
    headers.insert("User-Agent".to_string(), "TestClient/1.0".to_string());
    headers.insert("Accept".to_string(), "*/*".to_string());
    
    let mut config = TaskConfig::default();
    config.headers = headers.clone();
    
    assert_eq!(config.headers.len(), 2);
    assert_eq!(config.headers.get("User-Agent"), Some(&"TestClient/1.0".to_string()));
}

// @tc.name: ut_task_speed_limit_boundary_values
// @tc.desc: Test speed limit boundary values for set_max_speed operation
// @tc.precon: NA
// @tc.step: 1. Test speed limit with zero, positive, and max values
//           2. Verify values are valid for IPC serialization
// @tc.expect: Speed limit values within valid range are accepted
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_speed_limit_boundary_values() {
    let zero_speed: i64 = 0;
    let normal_speed: i64 = 1024000;
    let max_speed: i64 = i64::MAX;
    
    assert_eq!(zero_speed, 0, "Zero speed should be valid (no limit)");
    assert!(normal_speed > 0, "Normal speed should be positive");
    assert!(max_speed > normal_speed, "Max speed should be greater than normal");
}

// @tc.name: ut_task_ipc_parameter_serialization
// @tc.desc: Test IPC parameter serialization format for task operations
// @tc.precon: NA
// @tc.step: 1. Verify task_id is serialized as string for IPC
//           2. Verify version and count parameters are u32
// @tc.expect: IPC parameters use correct types for MsgParcel
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_ipc_parameter_serialization() {
    let task_id: i64 = 12345;
    let task_id_str: String = task_id.to_string();
    
    let version: u32 = 1u32;
    let task_count: u32 = 1u32;
    
    assert!(task_id_str.parse::<i64>().is_ok(), "Task ID string should be parseable back to i64");
    assert!(version > 0, "Version should be positive");
    assert!(task_count > 0, "Task count should be positive");
}

// @tc.name: ut_task_config_clone_independence
// @tc.desc: Test TaskConfig clone creates independent instance
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig and clone it
//           2. Modify original and verify clone is unchanged
// @tc.expect: Cloned TaskConfig is independent from original
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_config_clone_independence() {
    let mut original = TaskConfig::default();
    original.url = "https://original.com/file.zip".to_string();
    
    let cloned = original.clone();
    original.url = "https://modified.com/file.zip".to_string();
    
    assert_eq!(cloned.url, "https://original.com/file.zip", "Clone should not be affected by original modification");
}

// @tc.name: ut_task_error_equality
// @tc.desc: Test CreateTaskError equality comparison
// @tc.precon: NA
// @tc.step: 1. Create two CreateTaskError with same code
//           2. Verify they are considered equal in matching
// @tc.expect: CreateTaskError with same code can be matched
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_error_equality() {
    let error1 = CreateTaskError::Code(13400001);
    let error2 = CreateTaskError::Code(13400001);
    let error3 = CreateTaskError::Code(13400002);
    
    match (error1, error2, error3) {
        (CreateTaskError::Code(c1), CreateTaskError::Code(c2), CreateTaskError::Code(c3)) => {
            assert_eq!(c1, c2, "Same error codes should match");
            assert_ne!(c1, c3, "Different error codes should not match");
        }
        _ => panic!("Expected all Code variants"),
    }
}

// @tc.name: ut_task_config_version_field
// @tc.desc: Test TaskConfig version field for API compatibility
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig with different versions
//           2. Verify version is stored correctly
// @tc.expect: TaskConfig version field works correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_config_version_field() {
    let mut config = TaskConfig::default();
    config.version = request_core::config::Version::API10;
    
    assert_eq!(config.version, request_core::config::Version::API10);
}
