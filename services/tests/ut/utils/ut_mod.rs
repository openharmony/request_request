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
use crate::tests::test_init;
// @tc.name: ut_utils_oh
// @tc.desc: Test utility functions under OH feature
// @tc.precon: NA
// @tc.step: 1. Call is_system_api and query_calling_bundle functions
// @tc.expect: is_system_api returns false, query_calling_bundle returns empty
// string @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_utils_oh() {
    assert!(!is_system_api());
    assert_eq!(query_calling_bundle(), "");
}

// @tc.name: ut_utils_check_permission
// @tc.desc: Test permission checking utility function
// @tc.precon: NA
// @tc.step: 1. Call check_permission with various permissions
// @tc.expect: All permission checks return false
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_utils_check_permission() {
    assert!(!check_permission("ohos.permission.INTERNET"));
    assert!(!check_permission("ohos.permission.GET_NETWORK_INFO"));
    assert!(!check_permission("ohos.permission.READ_MEDIA"));
    assert!(!check_permission("ohos.permission.WRITE_MEDIA"));
    assert!(!check_permission("ohos.permission.RUNNING_STATE_OBSERVER"));
    assert!(!check_permission("ohos.permission.GET_NETWORK_INFO"));
    assert!(!check_permission("ohos.permission.CONNECTIVITY_INTERNAL"));
    assert!(!check_permission(
        "ohos.permission.SEND_TASK_COMPLETE_EVENT"
    ));
    assert!(!check_permission("ohos.permission.ACCESS_CERT_MANAGER"));
    assert!(!check_permission(
        "ohos.permission.INTERACT_ACROSS_LOCAL_ACCOUNTS"
    ));
    assert!(!check_permission("ohos.permission.MANAGE_LOCAL_ACCOUNTS"));
}

// @tc.name: ut_utils_check_permission_oh
// @tc.desc: Test permission checking under OH feature
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Call check_permission with various permissions
// @tc.expect: Most permission checks return true, specific permission returns
// false @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_utils_check_permission_oh() {
    test_init();
    assert!(check_permission("ohos.permission.INTERNET"));
    assert!(check_permission("ohos.permission.GET_NETWORK_INFO"));
    assert!(check_permission("ohos.permission.READ_MEDIA"));
    assert!(check_permission("ohos.permission.WRITE_MEDIA"));
    assert!(check_permission("ohos.permission.RUNNING_STATE_OBSERVER"));
    assert!(check_permission("ohos.permission.GET_NETWORK_INFO"));
    assert!(check_permission("ohos.permission.CONNECTIVITY_INTERNAL"));
    assert!(check_permission("ohos.permission.SEND_TASK_COMPLETE_EVENT"));
    assert!(check_permission("ohos.permission.ACCESS_CERT_MANAGER"));
    assert!(check_permission(
        "ohos.permission.INTERACT_ACROSS_LOCAL_ACCOUNTS"
    ));
    assert!(check_permission("ohos.permission.MANAGE_LOCAL_ACCOUNTS"));
    assert!(!check_permission(
        "ohos.permission.INTERACT_ACROSS_LOCAL_ACCOUNTS_EXTENSION"
    ));
}