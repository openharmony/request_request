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

// @tc.name: ut_convert_path_basic
// @tc.desc: Test convert_path function with basic path
// @tc.precon: NA
// @tc.step: 1. Call convert_path with uid, bundle_name and path
//           2. Verify the path is correctly converted
// @tc.expect: Path is converted with app and uuid format
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_convert_path_basic() {
    let uid: u64 = 200000;
    let bundle_name = "com.example.test";
    let path = "storage/base/test.txt";

    let result = convert_path(uid, bundle_name, path);

    assert!(result.contains("app"));
    assert!(result.contains("com.example.test"));
    assert!(!result.contains("storage"));
}

// @tc.name: ut_convert_path_different_uid
// @tc.desc: Test convert_path function with different uid values
// @tc.precon: NA
// @tc.step: 1. Call convert_path with different uid values
//           2. Verify the uuid is correctly calculated from uid
// @tc.expect: Different uid values produce different uuid in path
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_convert_path_different_uid() {
    let bundle_name = "com.example.test";
    let path = "storage/base/test.txt";

    let result1 = convert_path(200000, bundle_name, path);
    let result2 = convert_path(400000, bundle_name, path);

    assert_ne!(result1, result2);
    assert!(result1.contains("/1/"));
    assert!(result2.contains("/2/"));
}

// @tc.name: ut_convert_path_no_base
// @tc.desc: Test convert_path function when path has no base segment
// @tc.precon: NA
// @tc.step: 1. Call convert_path with path that doesn't contain 'base'
//           2. Verify only storage is replaced
// @tc.expect: Only storage is replaced with app
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_convert_path_no_base() {
    let uid: u64 = 200000;
    let bundle_name = "com.example.test";
    let path = "storage/cache/test.txt";

    let result = convert_path(uid, bundle_name, path);

    assert!(result.starts_with("app/cache/test.txt"));
}

// @tc.name: ut_get_uuid_from_uid
// @tc.desc: Test get_uuid_from_uid internal calculation
// @tc.precon: NA
// @tc.step: 1. Verify uuid calculation from uid (uid / 200000)
// @tc.expect: uuid is correctly calculated
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_get_uuid_from_uid() {
    let result1 = convert_path(200000, "test", "storage/base/file.txt");
    let result2 = convert_path(400000, "test", "storage/base/file.txt");
    let result3 = convert_path(600000, "test", "storage/base/file.txt");

    assert!(result1.contains("/1/"));
    assert!(result2.contains("/2/"));
    assert!(result3.contains("/3/"));
}

// @tc.name: ut_files_len_empty
// @tc.desc: Test Files struct len method with empty vector
// @tc.precon: NA
// @tc.step: 1. Create Files with empty vector
//           2. Verify len returns 0
// @tc.expect: len returns 0
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_files_len_empty() {
    let files = Files::new(vec![]);
    assert_eq!(files.len(), 0);
}

// @tc.name: ut_files_get_empty
// @tc.desc: Test Files struct get method with empty vector
// @tc.precon: NA
// @tc.step: 1. Create Files with empty vector
//           2. Verify get returns None
// @tc.expect: get returns None
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_files_get_empty() {
    let files = Files::new(vec![]);
    assert!(files.get(0).is_none());
}

// @tc.name: ut_files_get_invalid_index
// @tc.desc: Test Files struct get method with invalid index
// @tc.precon: NA
// @tc.step: 1. Create Files with file
//           2. Get file at out-of-bounds index
// @tc.expect: Returns None
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_files_get_invalid_index() {
    use std::fs::File;
    use std::sync::{Arc, Mutex};

    let file = File::open("/dev/null").unwrap();
    let files = Files::new(vec![Arc::new(Mutex::new(file))]);

    let result = files.get(10);
    assert!(result.is_none());
}

// @tc.name: ut_files_get_valid_index
// @tc.desc: Test Files struct get method with valid index
// @tc.precon: NA
// @tc.step: 1. Create Files with file
//           2. Get file at valid index
// @tc.expect: Returns Some with the file
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_files_get_valid_index() {
    use std::fs::File;
    use std::sync::{Arc, Mutex};

    let file = File::open("/dev/null").unwrap();
    let file_arc = Arc::new(Mutex::new(file));
    let files = Files::new(vec![file_arc.clone()]);

    let result = files.get(0);
    assert!(result.is_some());
}

// @tc.name: ut_files_len_single
// @tc.desc: Test Files struct len method with single file
// @tc.precon: NA
// @tc.step: 1. Create Files with single file
//           2. Verify len returns 1
// @tc.expect: len returns 1
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_files_len_single() {
    use std::fs::File;
    use std::sync::{Arc, Mutex};

    let file = File::open("/dev/null").unwrap();
    let files = Files::new(vec![Arc::new(Mutex::new(file))]);

    assert_eq!(files.len(), 1);
}

// @tc.name: ut_files_len_multiple
// @tc.desc: Test Files struct len method with multiple files
// @tc.precon: NA
// @tc.step: 1. Create Files with multiple files
//           2. Verify len returns correct count
// @tc.expect: len returns correct count
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_files_len_multiple() {
    use std::fs::File;
    use std::sync::{Arc, Mutex};

    let file1 = File::open("/dev/null").unwrap();
    let file2 = File::open("/dev/null").unwrap();
    let files = Files::new(vec![
        Arc::new(Mutex::new(file1)),
        Arc::new(Mutex::new(file2)),
    ]);

    assert_eq!(files.len(), 2);
}

// @tc.name: ut_bundle_cache_new
// @tc.desc: Test BundleCache new method
// @tc.precon: NA
// @tc.step: 1. Create BundleCache with TaskConfig
//           2. Verify initial state
// @tc.expect: BundleCache is created with None value
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_bundle_cache_new() {
    let config = TaskConfig::default();
    let cache = BundleCache::new(&config);

    assert!(cache.value.is_none());
}

// @tc.name: ut_bundle_cache_get_value_caching
// @tc.desc: Test BundleCache caches value correctly
// @tc.precon: NA
// @tc.step: 1. Create BundleCache with TaskConfig
//           2. Call get_value multiple times
// @tc.expect: Value is cached after first call
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_bundle_cache_get_value_caching() {
    let config = TaskConfig::default();
    let mut cache = BundleCache::new(&config);

    assert!(cache.value.is_none());

    let result1 = cache.get_value();
    assert!(cache.value.is_some());

    let result2 = cache.get_value();
    assert_eq!(result1, result2);
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
