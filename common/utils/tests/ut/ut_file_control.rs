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

use super::*;

// @tc.name: ut_split_whole_path
// @tc.desc: Test the standardized path checking function with various valid and
// invalid paths
// @tc.precon: NA
// @tc.step: 1. Call check_standardized_path with different path inputs
//           2. Verify the return value for each case
// @tc.expect: Function returns true for valid paths and false for invalid paths
// containing relative segments or malformed structure
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level2
#[test]
fn ut_split_whole_path() {
    assert!(check_standardized_path("/A/B/C"));
    assert!(!check_standardized_path("/A/B/C/../D"));
    assert!(!check_standardized_path("/A/B/../C/../D"));
    assert!(!check_standardized_path("/A/B/C/../../D"));
    assert!(!check_standardized_path("/A/B/C/../.."));
    assert!(!check_standardized_path("/A/B/../../C"));
    assert!(!check_standardized_path("/A/B/../../../C"));
    assert!(!check_standardized_path("/../B/C/D"));
    assert!(!check_standardized_path("/A/B/./C"));
    assert!(!check_standardized_path("/A/B/C/"));
    assert!(!check_standardized_path("A/B/C/"));
    assert!(!check_standardized_path("A/B/C"));
    assert!(!check_standardized_path("//A//B//C"));
    assert!(!check_standardized_path("/A/B//C"));
    assert!(!check_standardized_path("/"));
    assert!(!check_standardized_path(""));
    assert!(!check_standardized_path(r"/A/B/../C"));
    assert!(!check_standardized_path(r"/A/B/\.\./C"));
    assert!(!check_standardized_path(r"/A/B/\.\.\/C"));
    assert!(!check_standardized_path(r"/A/B/..\/C"));
    assert!(!check_standardized_path(r"/A/B/.\./C"));
    assert!(!check_standardized_path(r"/A/B/\../C"));
}

// @tc.name: ut_delete_base
// @tc.desc: Test the delete_base_for_list function to retain paths longer than
// AREA1 length @tc.precon: NA
// @tc.step: 1. Create a vector with various paths
//           2. Call delete_base_for_list to filter the vector
//           3. Compare the result with expected filtered vector
// @tc.expect: Only paths longer than AREA1 length are retained in the vector
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
#[test]
fn ut_delete_base() {
    let mut v = vec![
        "/data",
        "/data/storage",
        "/data/storage/el1",
        "/data/storage/el1/base",
        "/data/storage/el1/base/A",
        "/data/storage/el1/base/A/B",
    ];
    delete_base_for_list(&mut v);
    let v2 = vec!["/data/storage/el1/base/A", "/data/storage/el1/base/A/B"];
    assert_eq!(v, v2);
}
