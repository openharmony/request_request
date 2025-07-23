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
// @tc.name: ut_enum_reason
// @tc.desc: Test the repr values of Reason enum
// @tc.precon: NA
// @tc.step: 1. Check the repr value of each Reason enum variant
// @tc.expect: Each Reason variant has the correct repr value
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_enum_reason() {
    assert_eq!(Reason::Default.repr, 0);
    assert_eq!(Reason::TaskSurvivalOneMonth.repr, 1);
    assert_eq!(Reason::RunningTaskMeetLimits.repr, 4);
    assert_eq!(Reason::UserOperation.repr, 5);
    assert_eq!(Reason::AppBackgroundOrTerminate.repr, 6);
    assert_eq!(Reason::NetworkOffline.repr, 7);
    assert_eq!(Reason::UnsupportedNetworkType.repr, 8);
    assert_eq!(Reason::BuildRequestFailed.repr, 10);
    assert_eq!(Reason::GetFileSizeFailed.repr, 11);
    assert_eq!(Reason::ContinuousTaskTimeout.repr, 12);
    assert_eq!(Reason::RequestError.repr, 14);
    assert_eq!(Reason::UploadFileError.repr, 15);
    assert_eq!(Reason::RedirectError.repr, 16);
    assert_eq!(Reason::ProtocolError.repr, 17);
    assert_eq!(Reason::IoError.repr, 18);
    assert_eq!(Reason::UnsupportedRangeRequest.repr, 19);
    assert_eq!(Reason::OthersError.repr, 20);
    assert_eq!(Reason::AccountStopped.repr, 21);
    assert_eq!(Reason::Dns.repr, 23);
    assert_eq!(Reason::Tcp.repr, 24);
    assert_eq!(Reason::Ssl.repr, 25);
    assert_eq!(Reason::InsufficientSpace.repr, 26);
    assert_eq!(Reason::NetworkApp.repr, 27);
    assert_eq!(Reason::NetworkAccount.repr, 28);
    assert_eq!(Reason::AppAccount.repr, 29);
    assert_eq!(Reason::NetworkAppAccount.repr, 30);
    assert_eq!(Reason::LowSpeed.repr, 31);
}