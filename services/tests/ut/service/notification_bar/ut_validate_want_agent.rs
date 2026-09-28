// Copyright (C) 2024 Huawei Device Co., Ltd.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

// @tc.name: ut_validate_want_agent_system_caller_allowed
// @tc.desc: A system API caller may set any want_agent (trusted).
// @tc.precon: NA
// @tc.step: 1. Call validate_want_agent_ownership with is_system_api=true
// @tc.expect: Returns true regardless of want_agent content
// @tc.type: FUNC
// @tc.require: issues#WantAgentOwnership
#[cfg(feature = "oh")]
#[test]
fn ut_validate_want_agent_system_caller_allowed() {
    assert!(validate_want_agent_ownership(
        "{\"wants\":[]}",
        "com.anything",
        true
    ));
}

// @tc.name: ut_validate_want_agent_empty_allowed
// @tc.desc: An empty want_agent is always allowed (no target to validate).
// @tc.precon: NA
// @tc.step: 1. Call validate_want_agent_ownership with an empty want_agent
// @tc.expect: Returns true (nothing to validate)
// @tc.type: FUNC
// @tc.require: issues#WantAgentOwnership
#[cfg(feature = "oh")]
#[test]
fn ut_validate_want_agent_empty_allowed() {
    assert!(validate_want_agent_ownership("", "com.caller", false));
}

// @tc.name: ut_check_bundle_ownership_empty_target_allowed
// @tc.desc: Empty target bundle (implicit start) is allowed.
// @tc.precon: NA
// @tc.step: 1. Call check_bundle_ownership with empty target_bundle
// @tc.expect: Returns true
// @tc.type: FUNC
// @tc.require: issues#WantAgentOwnership
#[cfg(feature = "oh")]
#[test]
fn ut_check_bundle_ownership_empty_target_allowed() {
    assert!(check_bundle_ownership("", "com.caller"));
}

// @tc.name: ut_check_bundle_ownership_empty_caller_rejected
// @tc.desc: Non-empty target with empty caller_bundle is rejected.
// @tc.precon: NA
// @tc.step: 1. Call check_bundle_ownership with non-empty target, empty caller
// @tc.expect: Returns false
// @tc.type: FUNC
// @tc.require: issues#WantAgentOwnership
#[cfg(feature = "oh")]
#[test]
fn ut_check_bundle_ownership_empty_caller_rejected() {
    assert!(!check_bundle_ownership("com.evil.target", ""));
}

// @tc.name: ut_check_bundle_ownership_match_allowed
// @tc.desc: Target bundle matching caller_bundle is allowed.
// @tc.precon: NA
// @tc.step: 1. Call check_bundle_ownership with matching bundles
// @tc.expect: Returns true
// @tc.type: FUNC
// @tc.require: issues#WantAgentOwnership
#[cfg(feature = "oh")]
#[test]
fn ut_check_bundle_ownership_match_allowed() {
    assert!(check_bundle_ownership("com.app", "com.app"));
}

// @tc.name: ut_check_bundle_ownership_mismatch_rejected
// @tc.desc: Target bundle differing from caller_bundle is rejected.
// @tc.precon: NA
// @tc.step: 1. Call check_bundle_ownership with mismatched bundles
// @tc.expect: Returns false
// @tc.type: FUNC
// @tc.require: issues#WantAgentOwnership
#[cfg(feature = "oh")]
#[test]
fn ut_check_bundle_ownership_mismatch_rejected() {
    assert!(!check_bundle_ownership("com.evil.target", "com.legit.app"));
}

// @tc.name: ut_check_want_agent_meta_send_common_event_rejected
// @tc.desc: A SEND_COMMON_EVENT want_agent is rejected even if bundle matches.
// @tc.precon: NA
// @tc.step: 1. Build WantAgentMeta with is_send_common_event=true and
//           caller-owned bundle
// @tc.step: 2. Call check_want_agent_meta
// @tc.expect: Returns false (would act under service identity at trigger time)
// @tc.type: FUNC
// @tc.require: issues#WantAgentOwnership
#[cfg(feature = "oh")]
#[test]
fn ut_check_want_agent_meta_send_common_event_rejected() {
    let meta = ffi::WantAgentMeta {
        bundle_name: String::from("com.caller"),
        is_send_common_event: true,
        valid: true,
    };
    assert!(!check_want_agent_meta(&meta, "com.caller"));
}

// @tc.name: ut_check_want_agent_meta_start_ability_match_allowed
// @tc.desc: A START_ABILITY agent targeting the caller's own bundle is allowed.
// @tc.precon: NA
// @tc.step: 1. Build WantAgentMeta with is_send_common_event=false and
//           caller-owned bundle
// @tc.step: 2. Call check_want_agent_meta
// @tc.expect: Returns true
// @tc.type: FUNC
// @tc.require: issues#WantAgentOwnership
#[cfg(feature = "oh")]
#[test]
fn ut_check_want_agent_meta_start_ability_match_allowed() {
    let meta = ffi::WantAgentMeta {
        bundle_name: String::from("com.caller"),
        is_send_common_event: false,
        valid: true,
    };
    assert!(check_want_agent_meta(&meta, "com.caller"));
}

// @tc.name: ut_check_want_agent_meta_invalid_allowed
// @tc.desc: An unparseable want_agent keeps the existing lenient behavior.
// @tc.precon: NA
// @tc.step: 1. Build WantAgentMeta with valid=false
// @tc.step: 2. Call check_want_agent_meta
// @tc.expect: Returns true
// @tc.type: FUNC
// @tc.require: issues#WantAgentOwnership
#[cfg(feature = "oh")]
#[test]
fn ut_check_want_agent_meta_invalid_allowed() {
    let meta = ffi::WantAgentMeta {
        bundle_name: String::from(""),
        is_send_common_event: false,
        valid: false,
    };
    assert!(check_want_agent_meta(&meta, "com.caller"));
}
