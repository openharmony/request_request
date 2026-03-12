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

// @tc.name: ut_get_uuid_from_uid_calculation
// @tc.desc: Test UUID calculation from UID
// @tc.precon: NA
// @tc.step: 1. Call convert_path with various UID values
//           2. Verify UUID is correctly calculated (uid / 200000)
// @tc.expect: UUID matches uid / 200000
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_get_uuid_from_uid_calculation() {
    let result = convert_path(200000, "test.bundle", "storage/base/file.txt");
    assert!(result.contains("/1/base/test.bundle"));

    let result = convert_path(400000, "test.bundle", "storage/base/file.txt");
    assert!(result.contains("/2/base/test.bundle"));

    let result = convert_path(1000000, "test.bundle", "storage/base/file.txt");
    assert!(result.contains("/5/base/test.bundle"));
}

// @tc.name: ut_convert_path_storage_replacement
// @tc.desc: Test that 'storage' is replaced with 'app'
// @tc.precon: NA
// @tc.step: 1. Call convert_path with path starting with 'storage'
//           2. Verify 'storage' is replaced with 'app'
// @tc.expect: Path starts with 'app' instead of 'storage'
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_convert_path_storage_replacement() {
    let result = convert_path(200000, "test.bundle", "storage/base/file.txt");
    assert!(result.starts_with("app/"));
    assert!(!result.contains("storage/"));
}

// @tc.name: ut_convert_path_base_replacement
// @tc.desc: Test that 'base' is replaced with uuid/bundle format
// @tc.precon: NA
// @tc.step: 1. Call convert_path with path containing 'base'
//           2. Verify 'base' is replaced with uuid/bundle format
// @tc.expect: Path contains uuid/bundle instead of 'base'
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_convert_path_base_replacement() {
    let result = convert_path(200000, "com.example.app", "storage/base/dir/file.txt");
    assert!(result.contains("/1/base/com.example.app/"));
    assert!(!result.contains("/base/"));
}

// @tc.name: ut_convert_path_no_base_segment
// @tc.desc: Test convert_path when path has no 'base' segment
// @tc.precon: NA
// @tc.step: 1. Call convert_path with path not containing 'base'
//           2. Verify only 'storage' is replaced
// @tc.expect: Path has 'app' but no uuid/bundle replacement
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_convert_path_no_base_segment() {
    let result = convert_path(200000, "test.bundle", "storage/cache/file.txt");
    assert!(result.starts_with("app/cache/file.txt"));
}

// @tc.name: ut_convert_path_complex_path
// @tc.desc: Test convert_path with complex nested path
// @tc.precon: NA
// @tc.step: 1. Call convert_path with deeply nested path
//           2. Verify path is correctly converted
// @tc.expect: Complex path is correctly converted
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_convert_path_complex_path() {
    let result = convert_path(
        200000,
        "com.test.app",
        "storage/base/dir1/dir2/dir3/file.txt",
    );
    assert!(result.contains("app/1/base/com.test.app/dir1/dir2/dir3/file.txt"));
}

// @tc.name: ut_convert_path_special_bundle_name
// @tc.desc: Test convert_path with special characters in bundle name
// @tc.precon: NA
// @tc.step: 1. Call convert_path with bundle name containing dots
//           2. Verify bundle name is preserved correctly
// @tc.expect: Bundle name with dots is handled correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_convert_path_special_bundle_name() {
    let result = convert_path(200000, "com.example.test.app", "storage/base/file.txt");
    assert!(result.contains("com.example.test.app"));
}

// @tc.name: ut_convert_path_zero_uid
// @tc.desc: Test convert_path with zero UID
// @tc.precon: NA
// @tc.step: 1. Call convert_path with uid = 0
//           2. Verify uuid is 0
// @tc.expect: Path contains uuid 0
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_convert_path_zero_uid() {
    let result = convert_path(0, "test.bundle", "storage/base/file.txt");
    assert!(result.contains("/0/base/"));
}

// @tc.name: ut_convert_path_only_storage
// @tc.desc: Test convert_path with path containing only storage
// @tc.precon: NA
// @tc.step: 1. Call convert_path with minimal path
//           2. Verify conversion handles minimal path
// @tc.expect: Minimal path is converted correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_convert_path_only_storage() {
    let result = convert_path(200000, "test.bundle", "storage");
    assert_eq!(result, "app");
}
