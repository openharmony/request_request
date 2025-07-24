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

// @tc.name: ut_reason_enum_values
// @tc.desc: Test the repr values of Reason enum
// @tc.precon: NA
// @tc.step: 1. Check the repr value of each Reason enum variant
// @tc.expect: Each Reason variant has the correct repr value
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_reason_enum_values() {
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

// @tc.name: ut_reason_from_u8_valid_values
// @tc.desc: Test From<u8> conversion with valid Reason values
// @tc.precon: NA
// @tc.step: 1. Convert u8 values to Reason using From trait
//           2. Verify correct Reason variant is returned
// @tc.expect: All valid u8 values map to correct Reason variants
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_reason_from_u8_valid_values() {
    assert_eq!(Reason::from(0), Reason::Default);
    assert_eq!(Reason::from(1), Reason::TaskSurvivalOneMonth);
    assert_eq!(Reason::from(4), Reason::RunningTaskMeetLimits);
    assert_eq!(Reason::from(5), Reason::UserOperation);
    assert_eq!(Reason::from(6), Reason::AppBackgroundOrTerminate);
    assert_eq!(Reason::from(7), Reason::NetworkOffline);
    assert_eq!(Reason::from(8), Reason::UnsupportedNetworkType);
    assert_eq!(Reason::from(10), Reason::BuildRequestFailed);
    assert_eq!(Reason::from(11), Reason::GetFileSizeFailed);
    assert_eq!(Reason::from(12), Reason::ContinuousTaskTimeout);
    assert_eq!(Reason::from(14), Reason::RequestError);
    assert_eq!(Reason::from(15), Reason::UploadFileError);
    assert_eq!(Reason::from(16), Reason::RedirectError);
    assert_eq!(Reason::from(17), Reason::ProtocolError);
    assert_eq!(Reason::from(18), Reason::IoError);
    assert_eq!(Reason::from(19), Reason::UnsupportedRangeRequest);
    assert_eq!(Reason::from(20), Reason::OthersError);
    assert_eq!(Reason::from(21), Reason::AccountStopped);
    assert_eq!(Reason::from(23), Reason::Dns);
    assert_eq!(Reason::from(24), Reason::Tcp);
    assert_eq!(Reason::from(25), Reason::Ssl);
    assert_eq!(Reason::from(26), Reason::InsufficientSpace);
    assert_eq!(Reason::from(27), Reason::NetworkApp);
    assert_eq!(Reason::from(28), Reason::NetworkAccount);
    assert_eq!(Reason::from(29), Reason::AppAccount);
    assert_eq!(Reason::from(30), Reason::NetworkAppAccount);
    assert_eq!(Reason::from(31), Reason::LowSpeed);
}

// @tc.name: ut_reason_from_u8_invalid_values
// @tc.desc: Test From<u8> conversion with invalid Reason values
// @tc.precon: NA
// @tc.step: 1. Convert invalid u8 values to Reason
//           2. Verify OthersError is returned for invalid values
// @tc.expect: All invalid u8 values map to OthersError
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 2
#[test]
fn ut_reason_from_u8_invalid_values() {
    let invalid_values = vec![2, 3, 9, 13, 22, 32, 100, 200, 255];
    for value in invalid_values {
        assert_eq!(Reason::from(value), Reason::OthersError);
    }
}

// @tc.name: ut_reason_to_str_all_variants
// @tc.desc: Test to_str method for all Reason variants
// @tc.precon: NA
// @tc.step: 1. Call to_str on each Reason variant
//           2. Verify correct string is returned
// @tc.expect: All Reason variants return correct descriptive strings
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_reason_to_str_all_variants() {
    assert_eq!(Reason::Default.to_str(), "");
    assert_eq!(Reason::TaskSurvivalOneMonth.to_str(), "The task has not been completed for a month yet");
    assert_eq!(Reason::RunningTaskMeetLimits.to_str(), "Too many task in running state");
    assert_eq!(Reason::UserOperation.to_str(), "User operation");
    assert_eq!(Reason::AppBackgroundOrTerminate.to_str(), "The app is background or terminate");
    assert_eq!(Reason::NetworkOffline.to_str(), "NetWork is offline");
    assert_eq!(Reason::UnsupportedNetworkType.to_str(), "NetWork type not meet the task config");
    assert_eq!(Reason::BuildRequestFailed.to_str(), "Build request error");
    assert_eq!(Reason::GetFileSizeFailed.to_str(), "Failed because cannot get the file size from the server and the precise is setted true by user");
    assert_eq!(Reason::ContinuousTaskTimeout.to_str(), "Continuous processing task time out");
    assert_eq!(Reason::RequestError.to_str(), "Request error");
    assert_eq!(Reason::UploadFileError.to_str(), "There are some files upload failed");
    assert_eq!(Reason::RedirectError.to_str(), "Redirect error");
    assert_eq!(Reason::ProtocolError.to_str(), "Http protocol error");
    assert_eq!(Reason::IoError.to_str(), "Io Error");
    assert_eq!(Reason::UnsupportedRangeRequest.to_str(), "The server is not support range request");
    assert_eq!(Reason::OthersError.to_str(), "Some other error occured");
    assert_eq!(Reason::AccountStopped.to_str(), "Account stopped");
    assert_eq!(Reason::Dns.to_str(), "DNS error");
    assert_eq!(Reason::Tcp.to_str(), "TCP error");
    assert_eq!(Reason::Ssl.to_str(), "TSL/SSL error");
    assert_eq!(Reason::InsufficientSpace.to_str(), "Insufficient space");
    assert_eq!(Reason::NetworkApp.to_str(), "NetWork is offline and the app is background or terminate");
    assert_eq!(Reason::NetworkAccount.to_str(), "NetWork is offline and the account is stopped");
    assert_eq!(Reason::AppAccount.to_str(), "The app is background or terminate and the account is stopped");
    assert_eq!(Reason::NetworkAppAccount.to_str(), "NetWork is offline and the app is background or terminate and the account is stopped");
    assert_eq!(Reason::LowSpeed.to_str(), "Below low speed limit");
}

// @tc.name: ut_reason_partial_eq
// @tc.desc: Test Reason enum PartialEq implementation
// @tc.precon: NA
// @tc.step: 1. Create multiple Reason instances
//           2. Compare for equality
// @tc.expect: Same variants are equal, different variants are not
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_reason_partial_eq() {
    let reason1 = Reason::IoError;
    let reason2 = Reason::IoError;
    let reason3 = Reason::NetworkOffline;

    assert_eq!(reason1, reason2);
    assert_ne!(reason1, reason3);
    assert_ne!(reason2, reason3);
}

// @tc.name: ut_reason_debug_format
// @tc.desc: Test Reason enum Debug formatting
// @tc.precon: NA
// @tc.step: 1. Format Reason variants using Debug
//           2. Verify format contains variant name
// @tc.expect: Debug format shows correct variant names
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 2
#[test]
fn ut_reason_debug_format() {
    let reason = Reason::IoError;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("IoError"));

    let reason = Reason::NetworkOffline;
    let debug_str = format!("{:?}", reason);
    assert!(debug_str.contains("NetworkOffline"));
}