// Copyright (C) 2024 Huawei Device Co., Ltd.
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

// @tc.name: ut_app_info_struct_fields
// @tc.desc: Test AppInfo struct fields with various values
// @tc.precon: NA
// @tc.step: 1. Create AppInfo with ret = true
//           2. Create AppInfo with ret = false
//           3. Verify all fields are correctly stored
// @tc.expect: All fields are correctly stored for both cases
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_app_info_struct_fields() {
    let app_info_true = ffi::AppInfo {
        ret: true,
        index: 1,
        name: "com.example.app".to_string(),
    };
    assert!(app_info_true.ret);
    assert_eq!(app_info_true.index, 1);
    assert_eq!(app_info_true.name, "com.example.app");

    let app_info_false = ffi::AppInfo {
        ret: false,
        index: 0,
        name: String::new(),
    };
    assert!(!app_info_false.ret);
    assert_eq!(app_info_false.index, 0);
    assert!(app_info_false.name.is_empty());
}

// @tc.name: ut_app_info_field_values
// @tc.desc: Test AppInfo with various index and name values
// @tc.precon: NA
// @tc.step: 1. Create AppInfo with different index values (0, positive, negative)
//           2. Create AppInfo with different name formats (dots, underscore, empty)
// @tc.expect: All field values are preserved correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_app_info_field_values() {
    let app_info_zero = ffi::AppInfo {
        ret: true,
        index: 0,
        name: "app0".to_string(),
    };
    assert_eq!(app_info_zero.index, 0);

    let app_info_positive = ffi::AppInfo {
        ret: true,
        index: 100,
        name: "com.example.test.app".to_string(),
    };
    assert_eq!(app_info_positive.index, 100);
    assert_eq!(app_info_positive.name, "com.example.test.app");

    let app_info_negative = ffi::AppInfo {
        ret: true,
        index: -1,
        name: "com_example_app".to_string(),
    };
    assert_eq!(app_info_negative.index, -1);
    assert_eq!(app_info_negative.name, "com_example_app");
}

// @tc.name: ut_app_info_clone
// @tc.desc: Test AppInfo clone functionality
// @tc.precon: NA
// @tc.step: 1. Create AppInfo and clone it
//           2. Verify clone has same values
// @tc.expect: Cloned AppInfo has identical values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_app_info_clone() {
    let original = ffi::AppInfo {
        ret: true,
        index: 5,
        name: "com.test.app".to_string(),
    };

    let cloned = original.clone();

    assert_eq!(original.ret, cloned.ret);
    assert_eq!(original.index, cloned.index);
    assert_eq!(original.name, cloned.name);
}

// @tc.name: ut_app_info_debug_format
// @tc.desc: Test AppInfo debug formatting
// @tc.precon: NA
// @tc.step: 1. Create AppInfo and format with Debug
//           2. Verify output contains field names
// @tc.expect: Debug format shows field names and values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_app_info_debug_format() {
    let app_info = ffi::AppInfo {
        ret: true,
        index: 1,
        name: "test.app".to_string(),
    };

    let debug_str = format!("{:?}", app_info);

    assert!(debug_str.contains("ret"));
    assert!(debug_str.contains("index"));
    assert!(debug_str.contains("name"));
}

// @tc.name: ut_app_info_comparison
// @tc.desc: Test AppInfo equality and inequality comparison
// @tc.precon: NA
// @tc.step: 1. Create identical AppInfo instances and compare for equality
//           2. Create different AppInfo instances and compare for inequality
// @tc.expect: Identical instances are equal, different instances are not
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_app_info_comparison() {
    let app_info1 = ffi::AppInfo {
        ret: true,
        index: 1,
        name: "com.test".to_string(),
    };

    let app_info2 = ffi::AppInfo {
        ret: true,
        index: 1,
        name: "com.test".to_string(),
    };
    assert_eq!(app_info1, app_info2);

    let app_info3 = ffi::AppInfo {
        ret: false,
        index: 1,
        name: "com.test".to_string(),
    };
    assert_ne!(app_info1, app_info3);

    let app_info4 = ffi::AppInfo {
        ret: true,
        index: 2,
        name: "com.test".to_string(),
    };
    assert_ne!(app_info1, app_info4);

    let app_info5 = ffi::AppInfo {
        ret: true,
        index: 1,
        name: "com.other".to_string(),
    };
    assert_ne!(app_info1, app_info5);
}
