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