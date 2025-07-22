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

use crate::task::request_task::change_upload_size;

// @tc.name: ut_upload_size
// @tc.desc: Test the change_upload_size function with various parameters
// @tc.precon: NA
// @tc.step: 1. Call change_upload_size(0, -1, 30) and check result
//           2. Call change_upload_size(10, -1, 30) and check result
//           3. Call change_upload_size(0, 10, 30) and check result
//           4. Call change_upload_size(10, 10, 100) and check result
//           5. Call change_upload_size(0, 30, 30) and check result
//           6. Call change_upload_size(0, 0, 0) and check result
//           7. Call change_upload_size(10, 9, 100) and check result
// @tc.expect: All calls return expected values as per assertions
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_upload_size() {
    assert_eq!(change_upload_size(0, -1, 30), 30);
    assert_eq!(change_upload_size(10, -1, 30), 20);
    assert_eq!(change_upload_size(0, 10, 30), 11);
    assert_eq!(change_upload_size(10, 10, 100), 1);
    assert_eq!(change_upload_size(0, 30, 30), 30);
    assert_eq!(change_upload_size(0, 0, 0), 0);
    assert_eq!(change_upload_size(10, 9, 100), 100);
}