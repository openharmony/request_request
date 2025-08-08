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

// @tc.name: ut_enum_action
// @tc.desc: Test Action enum variant representations
// @tc.precon: NA
// @tc.step: 1. Verify the repr value of Action::Download
//           2. Verify the repr value of Action::Upload
//           3. Verify the repr value of Action::Any
// @tc.expect: Action::Download repr is 0, Action::Upload repr is 1, Action::Any
// repr is 2
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_enum_action() {
    assert_eq!(Action::Download.repr, 0);
    assert_eq!(Action::Upload.repr, 1);
    assert_eq!(Action::Any.repr, 2);
}

// @tc.name: ut_enum_mode
// @tc.desc: Test Mode enum variant representations
// @tc.precon: NA
// @tc.step: 1. Verify the repr value of Mode::BackGround
//           2. Verify the repr value of Mode::FrontEnd
//           3. Verify the repr value of Mode::Any
// @tc.expect: Mode::BackGround repr is 0, Mode::FrontEnd repr is 1, Mode::Any
// repr is 2
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_enum_mode() {
    assert_eq!(Mode::BackGround.repr, 0);
    assert_eq!(Mode::FrontEnd.repr, 1);
    assert_eq!(Mode::Any.repr, 2);
}

// @tc.name: ut_enum_version
// @tc.desc: Test Version enum variant representations
// @tc.precon: NA
// @tc.step: 1. Verify the u32 value of Version::API9
//           2. Verify the u32 value of Version::API10
// @tc.expect: Version::API9 as u32 is 1, Version::API10 as u32 is 2
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_enum_version() {
    assert_eq!(Version::API9 as u32, 1);
    assert_eq!(Version::API10 as u32, 2);
}

// @tc.name: ut_enum_network_config
// @tc.desc: Test NetworkConfig enum variant representations
// @tc.precon: NA
// @tc.step: 1. Verify the u32 value of NetworkConfig::Any
//           2. Verify the u32 value of NetworkConfig::Wifi
//           3. Verify the u32 value of NetworkConfig::Cellular
// @tc.expect: NetworkConfig::Any as u32 is 0, NetworkConfig::Wifi as u32 is 1,
// NetworkConfig::Cellular as u32 is 2
// @tc.type: FUNC
// @tc.require: issues#ICN31I
#[test]
fn ut_enum_network_config() {
    assert_eq!(NetworkConfig::Any as u32, 0);
    assert_eq!(NetworkConfig::Wifi as u32, 1);
    assert_eq!(NetworkConfig::Cellular as u32, 2);
}
