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

// @tc.name: ut_load_certs_uid_zero
// @tc.desc: Test load_certificates_from_paths with uid=0 (system process)
// @tc.precon: NA
// @tc.step: 1. Call load_certificates_from_paths with uid=0
//           2. Verify function returns without panic
// @tc.expect: Function completes without panic, returns Vec<Certificate>
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_load_certs_uid_zero() {
    // uid=0 -> user_id=0, so no current-user certs loaded even if developermode on
    let certs = load_certificates_from_paths(0, 0);
    assert_eq!(certs.len(), 2); // Should load system certs and user 0 certs
}

// @tc.name: ut_load_certs_user_id_zero_skips_current_user
// @tc.desc: Test that uid in user-0 range does not load current-user certs
// @tc.precon: NA
// @tc.step: 1. Use uid < 200000 (user_id=0)
//           2. Call load_certificates_from_paths
//           3. Verify no panic
// @tc.expect: Current-user cert loading is skipped when user_id=0
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_load_certs_user_id_zero_skips_current_user() {
    let certs = load_certificates_from_paths(100000, 0); // user_id = 0
    assert_eq!(certs.len(), 2); // Should load system certs and user 0 certs, but not current-user certs
}

// @tc.name: ut_load_certs_user_id_nonzero
// @tc.desc: Test load_certificates_from_paths with uid yielding user_id > 0
// @tc.precon: NA
// @tc.step: 1. Use uid=200000 (user_id=1)
//           2. Call load_certificates_from_paths
//           3. Verify no panic
// @tc.expect: Function completes, current-user path attempted (may fail gracefully if absent)
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_load_certs_user_id_nonzero() {
    let certs = load_certificates_from_paths(200000, 0); // user_id = 1
    assert!(certs.len() >= 2); // Should load system certs and user 0 certs, current-user certs may or may not load based on developermode
}

// @tc.name: ut_load_certs_typical_app_uid
// @tc.desc: Test load_certificates_from_paths with a typical third-party app uid
// @tc.precon: NA
// @tc.step: 1. Use uid=20000000 (typical app on user 100)
//           2. Call load_certificates_from_paths
//           3. Verify no panic
// @tc.expect: Function handles typical app uid correctly without panic
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_load_certs_typical_app_uid() {
    let certs = load_certificates_from_paths(20000000, 0);
    if is_developermode_enabled() {
        assert_eq!(certs.len(), 3); // Should load system certs, user 0 certs, and current-user certs
    } else {
        assert_eq!(certs.len(), 2); // Should load system certs and user 0 certs, but not current-user certs
    }
}

// @tc.name: ut_load_certs_max_uid
// @tc.desc: Test load_certificates_from_paths with u64::MAX uid
// @tc.precon: NA
// @tc.step: 1. Call load_certificates_from_paths with u64::MAX
//           2. Verify no panic or overflow
// @tc.expect: Function handles extreme uid without panic or arithmetic overflow
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_load_certs_max_uid() {
    let certs = load_certificates_from_paths(u64::MAX, 0);
    assert!(certs.len() >= 2); // Should load system certs and user 0 certs, current-user certs may or may not load based on developermode
}

// @tc.name: ut_load_certs_returns_vec
// @tc.desc: Test that load_certificates_from_paths always returns a Vec for various uids
// @tc.precon: NA
// @tc.step: 1. Call with multiple uid values
//           2. Verify no panic for each
// @tc.expect: Return is always Vec<Certificate> (possibly empty), never panics
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_load_certs_returns_vec() {
    let uids = [0u64, 1, 100000, 200000, 400000, u64::MAX / 2];
    for uid in uids {
        let certs = load_certificates_from_paths(uid, 0);
        let cert_count = certs.len();
        assert!((2..=3).contains(&cert_count)); // Should load system certs and user 0 certs, current-user certs may or may not load based on developermode
    }
}

// @tc.name: ut_uid_divisor_is_200000
// @tc.desc: Test UID_TO_USER_ID_DIVISOR constant is 200000
// @tc.precon: NA
// @tc.step: 1. Verify divisor value equals 200000
// @tc.expect: Divisor matches OpenHarmony multi-user uid allocation spec
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_uid_divisor_is_200000() {
    assert_eq!(UID_TO_USER_ID_DIVISOR, 200000);
}
