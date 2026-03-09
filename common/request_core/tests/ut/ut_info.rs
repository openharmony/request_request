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

use request_core::info::{State, SubscribeType, Faults, Reason, WaitingReason};

// @tc.name: ut_state_from_u32
// @tc.desc: Test State From<u32> conversion
// @tc.precon: NA
// @tc.step: 1. Convert u32 values to State
//           2. Verify correct variants
// @tc.expect: All conversions produce correct State variants
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_state_from_u32() {
    assert_eq!(State::from(0x00), State::Initialized);
    assert_eq!(State::from(0x10), State::Waiting);
    assert_eq!(State::from(0x20), State::Running);
    assert_eq!(State::from(0x21), State::Retrying);
    assert_eq!(State::from(0x30), State::Paused);
    assert_eq!(State::from(0x31), State::Stopped);
    assert_eq!(State::from(0x40), State::Completed);
    assert_eq!(State::from(0x41), State::Failed);
    assert_eq!(State::from(0x50), State::Removed);
}

// @tc.name: ut_state_from_unknown
// @tc.desc: Test State From<u32> with unknown value
// @tc.precon: NA
// @tc.step: 1. Convert unknown u32 to State
//           2. Verify it returns Any
// @tc.expect: Unknown values return State::Any
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_state_from_unknown() {
    assert_eq!(State::from(0x99), State::Any);
    assert_eq!(State::from(0xFF), State::Any);
    assert_eq!(State::from(99999), State::Any);
}

// @tc.name: ut_state_clone
// @tc.desc: Test State Clone trait
// @tc.precon: NA
// @tc.step: 1. Create State variant
//           2. Clone it
//           3. Verify clone is equal
// @tc.expect: Clone works correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_state_clone() {
    let state = State::Running;
    let cloned = state.clone();
    assert_eq!(state, cloned);
}

// @tc.name: ut_state_debug
// @tc.desc: Test State Debug trait
// @tc.precon: NA
// @tc.step: 1. Create State variant
//           2. Format with Debug
//           3. Verify output
// @tc.expect: Debug output is correct
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_state_debug() {
    let state = State::Completed;
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("Completed"));
}

// @tc.name: ut_subscribe_type_from_u32
// @tc.desc: Test SubscribeType From<u32> conversion
// @tc.precon: NA
// @tc.step: 1. Convert u32 values to SubscribeType
//           2. Verify correct variants
// @tc.expect: All conversions produce correct variants
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_subscribe_type_from_u32() {
    assert_eq!(SubscribeType::from(0), SubscribeType::Completed);
    assert_eq!(SubscribeType::from(1), SubscribeType::Failed);
    assert_eq!(SubscribeType::from(2), SubscribeType::HeaderReceive);
    assert_eq!(SubscribeType::from(3), SubscribeType::Pause);
    assert_eq!(SubscribeType::from(4), SubscribeType::Progress);
    assert_eq!(SubscribeType::from(5), SubscribeType::Remove);
    assert_eq!(SubscribeType::from(6), SubscribeType::Resume);
    assert_eq!(SubscribeType::from(7), SubscribeType::Response);
    assert_eq!(SubscribeType::from(8), SubscribeType::FaultOccur);
    assert_eq!(SubscribeType::from(9), SubscribeType::Wait);
    assert_eq!(SubscribeType::from(10), SubscribeType::Butt);
}

// @tc.name: ut_faults_from_u32
// @tc.desc: Test Faults From<u32> conversion
// @tc.precon: NA
// @tc.step: 1. Convert u32 values to Faults
//           2. Verify correct variants
// @tc.expect: All conversions produce correct variants
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_faults_from_u32() {
    assert_eq!(Faults::from(0xFF), Faults::Others);
    assert_eq!(Faults::from(0x00), Faults::Disconnected);
    assert_eq!(Faults::from(0x10), Faults::Timeout);
    assert_eq!(Faults::from(0x20), Faults::Protocol);
    assert_eq!(Faults::from(0x30), Faults::Param);
    assert_eq!(Faults::from(0x40), Faults::Fsio);
    assert_eq!(Faults::from(0x50), Faults::Dns);
    assert_eq!(Faults::from(0x60), Faults::Tcp);
    assert_eq!(Faults::from(0x70), Faults::Ssl);
    assert_eq!(Faults::from(0x80), Faults::Redirect);
    assert_eq!(Faults::from(0x90), Faults::LowSpeed);
}

// @tc.name: ut_faults_from_reason
// @tc.desc: Test Faults From<Reason> conversion
// @tc.precon: NA
// @tc.step: 1. Convert Reason values to Faults
//           2. Verify correct mappings
// @tc.expect: All conversions produce correct Faults
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_faults_from_reason() {
    assert_eq!(Faults::from(Reason::NetworkOffline), Faults::Disconnected);
    assert_eq!(Faults::from(Reason::NetworkApp), Faults::Disconnected);
    assert_eq!(Faults::from(Reason::BuildClientFailed), Faults::Param);
    assert_eq!(Faults::from(Reason::GetFilesizeFailed), Faults::Fsio);
    assert_eq!(Faults::from(Reason::IoError), Faults::Fsio);
    assert_eq!(Faults::from(Reason::ContinuousTaskTimeout), Faults::Timeout);
    assert_eq!(Faults::from(Reason::ConnectError), Faults::Tcp);
    assert_eq!(Faults::from(Reason::ProtocolError), Faults::Protocol);
    assert_eq!(Faults::from(Reason::RedirectError), Faults::Redirect);
    assert_eq!(Faults::from(Reason::DNS), Faults::Dns);
    assert_eq!(Faults::from(Reason::TCP), Faults::Tcp);
    assert_eq!(Faults::from(Reason::SSL), Faults::Ssl);
    assert_eq!(Faults::from(Reason::LowSpeed), Faults::LowSpeed);
}

// @tc.name: ut_reason_from_u32
// @tc.desc: Test Reason From<u32> conversion
// @tc.precon: NA
// @tc.step: 1. Convert u32 values to Reason
//           2. Verify correct variants
// @tc.expect: All conversions produce correct Reason
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_reason_from_u32() {
    assert_eq!(Reason::from(0), Reason::ReasonOk);
    assert_eq!(Reason::from(1), Reason::TaskSurvivalOneMonth);
    assert_eq!(Reason::from(7), Reason::NetworkOffline);
    assert_eq!(Reason::from(12), Reason::ContinuousTaskTimeout);
    assert_eq!(Reason::from(18), Reason::IoError);
    assert_eq!(Reason::from(23), Reason::DNS);
    assert_eq!(Reason::from(24), Reason::TCP);
    assert_eq!(Reason::from(25), Reason::SSL);
    assert_eq!(Reason::from(31), Reason::LowSpeed);
}

// @tc.name: ut_waiting_reason_from_u32
// @tc.desc: Test WaitingReason From<u32> conversion
// @tc.precon: NA
// @tc.step: 1. Convert u32 values to WaitingReason
//           2. Verify correct variants
// @tc.expect: All conversions produce correct WaitingReason
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_waiting_reason_from_u32() {
    assert_eq!(WaitingReason::from(0x00), WaitingReason::TASK_QUEUE_FULL);
    assert_eq!(WaitingReason::from(0x01), WaitingReason::NETWORK_NOT_MATCH);
    assert_eq!(WaitingReason::from(0x02), WaitingReason::APP_BACKGROUND);
    assert_eq!(WaitingReason::from(0x03), WaitingReason::USER_INACTIVATED);
}

// @tc.name: ut_faults_copy
// @tc.desc: Test Faults Copy trait
// @tc.precon: NA
// @tc.step: 1. Create Faults variant
//           2. Copy it
//           3. Verify copy works
// @tc.expect: Copy works correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_faults_copy() {
    let faults = Faults::Timeout;
    let copied = faults;
    assert_eq!(faults, copied);
}

// @tc.name: ut_reason_copy
// @tc.desc: Test Reason Copy trait
// @tc.precon: NA
// @tc.step: 1. Create Reason variant
//           2. Copy it
//           3. Verify copy works
// @tc.expect: Copy works correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_reason_copy() {
    let reason = Reason::IoError;
    let copied = reason;
    assert_eq!(reason, copied);
}

// @tc.name: ut_waiting_reason_copy
// @tc.desc: Test WaitingReason Copy trait
// @tc.precon: NA
// @tc.step: 1. Create WaitingReason variant
//           2. Copy it
//           3. Verify copy works
// @tc.expect: Copy works correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_waiting_reason_copy() {
    let reason = WaitingReason::NETWORK_NOT_MATCH;
    let copied = reason;
    assert_eq!(reason, copied);
}
