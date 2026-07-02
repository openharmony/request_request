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

use super::*;
use crate::task::http_error_registry::{set_http_status_code, take_http_status_code};
use crate::task::reason::Reason;
use crate::utils::form_item::FileSpec;

// @tc.name: ut_enum_state
// @tc.desc: Test the repr values of State enum
// @tc.precon: NA
// @tc.step: 1. Check the repr value of each State enum variant
// @tc.expect: Each State variant has the correct repr value
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_enum_state() {
    assert_eq!(State::Initialized.repr, 0);
    assert_eq!(State::Waiting.repr, 16);
    assert_eq!(State::Running.repr, 32);
    assert_eq!(State::Retrying.repr, 33);
    assert_eq!(State::Paused.repr, 48);
    assert_eq!(State::Stopped.repr, 49);
    assert_eq!(State::Completed.repr, 64);
    assert_eq!(State::Failed.repr, 65);
    assert_eq!(State::Removed.repr, 80);
    assert_eq!(State::Any.repr, 97);
}

// @tc.name: ut_build_each_file_status_protocol_error_with_http_code
// @tc.desc: Test build_each_file_status enriches message with HTTP code for ProtocolError
// @tc.precon: NA
// @tc.step: 1. Create TaskInfo with ProtocolError reason
//           2. Set http_status_code via global map
//           3. Call build_each_file_status
// @tc.expect: message contains "Http protocol error: 404 Not Found"
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_build_each_file_status_protocol_error_with_http_code() {
    let mut info = TaskInfo::new();
    info.common_data.task_id = 888001;
    info.common_data.reason = 17; // Reason::ProtocolError
    info.progress.common_data.index = 0;
    info.file_specs = vec![FileSpec {
        name: "f".to_string(),
        path: "/tmp/f".to_string(),
        file_name: "f".to_string(),
        mime_type: "text/plain".to_string(),
        is_user_file: false,
        fd: None,
    }];
    set_http_status_code(888001, 404);
    let result = info.build_each_file_status();
    assert_eq!(result[0].message, "Http protocol error: 404 Not Found");
}

// @tc.name: ut_build_each_file_status_protocol_error_without_http_code
// @tc.desc: Test build_each_file_status with ProtocolError but no HTTP code recorded
// @tc.precon: NA
// @tc.step: 1. Create TaskInfo with ProtocolError reason
//           2. Do NOT set http_status_code
//           3. Call build_each_file_status
// @tc.expect: message is the default "Http protocol error"
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_build_each_file_status_protocol_error_without_http_code() {
    let mut info = TaskInfo::new();
    info.common_data.task_id = 888002;
    info.common_data.reason = 17; // Reason::ProtocolError
    info.progress.common_data.index = 0;
    info.file_specs = vec![FileSpec {
        name: "f".to_string(),
        path: "/tmp/f".to_string(),
        file_name: "f".to_string(),
        mime_type: "text/plain".to_string(),
        is_user_file: false,
        fd: None,
    }];
    // Ensure no leftover code from other tests
    take_http_status_code(888002);
    let result = info.build_each_file_status();
    assert_eq!(result[0].message, "Http protocol error");
}

// @tc.name: ut_build_each_file_status_non_protocol_error
// @tc.desc: Test build_each_file_status does not modify message for non-ProtocolError reasons
// @tc.precon: NA
// @tc.step: 1. Create TaskInfo with IoError reason
//           2. Call build_each_file_status
// @tc.expect: message is the default IoError message
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_build_each_file_status_non_protocol_error() {
    let mut info = TaskInfo::new();
    info.common_data.task_id = 888003;
    info.common_data.reason = 18; // Reason::IoError
    info.progress.common_data.index = 0;
    info.file_specs = vec![FileSpec {
        name: "f".to_string(),
        path: "/tmp/f".to_string(),
        file_name: "f".to_string(),
        mime_type: "text/plain".to_string(),
        is_user_file: false,
        fd: None,
    }];
    let result = info.build_each_file_status();
    assert_eq!(result[0].message, Reason::IoError.to_str());
}