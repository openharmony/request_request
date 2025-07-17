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
use crate::config::NetworkConfig;
use crate::manage::database::RequestDb;
use crate::tests::{lock_database, test_init};
use crate::utils::get_current_timestamp;
use crate::utils::task_id_generator::TaskIdGenerator;

const COMPLETED: u8 = State::Completed.repr;
const PAUSED: u8 = State::Paused.repr;
const INIT: u8 = State::Initialized.repr;
const WIFI: u8 = NetworkConfig::Wifi as u8;
const CELLULAR: u8 = NetworkConfig::Cellular as u8;

fn query_state_and_reason(task_id: u32) -> (u8, u8) {
    let db = RequestDb::get_instance();
    (
        db.query_integer(&format!(
            "SELECT state FROM request_task where task_id = {task_id}"
        ))[0],
        db.query_integer(&format!(
            "SELECT reason FROM request_task where task_id = {task_id}"
        ))[0],
    )
}

fn network(sql: &str, change_reason: u8) {
    let db = RequestDb::get_instance();
    let task_id = TaskIdGenerator::generate();
    let fail_reason = get_current_timestamp() as u8;

    // running
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, state, reason, network, version, mode, retry) VALUES ({task_id}, {RUNNING}, {fail_reason}, {WIFI}, {API10}, {BACKGROUND}, 1)",
    ))
    .unwrap();
    db.execute(sql).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, change_reason);

    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, state, reason, network, version, action) VALUES ({task_id}, {RUNNING}, {fail_reason}, {WIFI}, {API9}, {DOWNLOAD})",
    ))
    .unwrap();
    db.execute(sql).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, change_reason);

    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, state, reason, network, version, action) VALUES ({task_id}, {RUNNING}, {fail_reason}, {WIFI}, {API9}, {UPLOAD})",
    ))
    .unwrap();
    db.execute(sql).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, FAILED);
    assert_eq!(reason, change_reason);

    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, state, reason, network, version, mode, retry) VALUES ({task_id}, {RUNNING}, {fail_reason}, {WIFI}, {API10}, {FRONTEND}, 1)",
    ))
    .unwrap();
    db.execute(sql).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, FAILED);
    assert_eq!(reason, change_reason);

    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, state, reason, network, version, mode, retry) VALUES ({task_id}, {RUNNING}, {fail_reason}, {WIFI}, {API10}, {BACKGROUND}, 0)",
    ))
    .unwrap();
    db.execute(sql).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, FAILED);
    assert_eq!(reason, change_reason);

    // other state
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, state, reason, network) VALUES ({task_id}, {FAILED}, {fail_reason}, {WIFI})",
    ))
    .unwrap();
    db.execute(sql).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, FAILED);
    assert_eq!(reason, fail_reason);

    // waiting
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, state, reason, network) VALUES ({task_id}, {WAITING}, {RUNNING_TASK_MEET_LIMITS}, {WIFI})",
    ))
    .unwrap();
    db.execute(sql).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, change_reason);

    // api9 + download
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, state, version, action, network, metered, roaming) VALUES ({task_id}, {RUNNING}, {API9}, {DOWNLOAD}, {CELLULAR}, 1, 0)",
    ))
    .unwrap();
    db.execute(sql).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, change_reason);

    // api9 + upload
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, state, version, action, network, metered, roaming) VALUES ({task_id}, {RUNNING}, {API9}, {UPLOAD}, {CELLULAR}, 0, 1)",
    ))
    .unwrap();
    db.execute(sql).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, FAILED);
    assert_eq!(reason, change_reason);

    // api10 + background + retry
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, state, version, mode, retry, network, metered, roaming) VALUES ({task_id}, {RUNNING}, {API10}, {BACKGROUND}, 1, {CELLULAR}, 0, 0)",
    ))
    .unwrap();
    db.execute(sql).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, change_reason);

    // api10 + frontEnd + retry
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, state, version, mode, retry, network) VALUES ({task_id}, {RUNNING}, {API10}, {FRONTEND}, 1, {WIFI})",
    ))
    .unwrap();
    db.execute(sql).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, FAILED);
    assert_eq!(reason, change_reason);

    // api10 + Background
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, state, version, mode, retry, network) VALUES ({task_id}, {RUNNING}, {API10}, {BACKGROUND}, 0, {WIFI})",
    ))
    .unwrap();
    db.execute(sql).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, FAILED);
    assert_eq!(reason, change_reason);
}

// @tc.name: ut_network_offline
// @tc.desc: Test task state handling when network is offline
// @tc.precon: NA
// @tc.step: 1. Initialize test database
//           2. Lock database
//           3. Execute network offline test cases
//           4. Verify task state transitions and reasons
// @tc.expect: Tasks transition to correct states with appropriate reasons when network is offline
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_offline() {
    test_init();
    let _lock = lock_database();
    network(&network_offline(), NETWORK_OFFLINE);
}

// @tc.name: ut_network_unsupported
// @tc.desc: Test task state handling with unsupported network types
// @tc.precon: NA
// @tc.step: 1. Initialize test database
//           2. Lock database
//           3. Create network info with unsupported type
//           4. Execute network unavailable test cases
//           5. Verify task state transitions and reasons
// @tc.expect: Tasks transition to correct states with appropriate reasons for unsupported networks
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_unsupported() {
    test_init();
    let _lock = lock_database();
    let info = NetworkInfo {
        network_type: NetworkType::Cellular,
        is_metered: true,
        is_roaming: true,
    };
    network(
        &network_unavailable(&info).unwrap(),
        UNSUPPORTED_NETWORK_TYPE,
    );

    // network type matches
    let db = RequestDb::get_instance();
    let task_id = TaskIdGenerator::generate();
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, state, reason, network, metered, roaming) VALUES ({task_id}, {WAITING}, {RUNNING_TASK_MEET_LIMITS}, {CELLULAR}, 1, 1)",
    ))
    .unwrap();
    db.execute(&network_unavailable(&info).unwrap()).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, RUNNING_TASK_MEET_LIMITS);
}

// @tc.name: ut_network_online
// @tc.desc: Test task state handling when network is online
// @tc.precon: NA
// @tc.step: 1. Initialize test database
//           2. Lock database
//           3. Create network info with cellular type
//           4. Execute network available test cases
//           5. Verify task state transitions and reasons
// @tc.expect: Tasks transition to correct states with appropriate reasons when network is online
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_network_online() {
    test_init();
    let _lock = lock_database();
    let db = RequestDb::get_instance();
    let task_id = TaskIdGenerator::generate();

    let info = NetworkInfo {
        network_type: NetworkType::Cellular,
        is_metered: true,
        is_roaming: true,
    };

    // unsupported
    let unsupported_states = [
        (WIFI, 1, 1),
        (CELLULAR, 0, 0),
        (CELLULAR, 1, 0),
        (CELLULAR, 0, 1),
    ];
    for state in unsupported_states {
        db.execute(&format!(
            "INSERT OR REPLACE INTO request_task (task_id, state, reason, network, metered, roaming) VALUES ({task_id}, {WAITING}, {NETWORK_OFFLINE}, {}, {}, {})",state.0,state.1,state.2
        )).unwrap();

        db.execute(&network_available(&info)).unwrap();

        let state: u8 = db.query_integer(&format!(
            "SELECT state FROM request_task where task_id = {task_id}"
        ))[0];
        let reason: u8 = db.query_integer(&format!(
            "SELECT reason FROM request_task where task_id = {task_id}"
        ))[0];
        assert_eq!(state, WAITING);
        assert_eq!(reason, NETWORK_OFFLINE);
    }

    // support
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, state, reason, network, metered, roaming) VALUES ({task_id}, {WAITING}, {NETWORK_OFFLINE}, {CELLULAR}, 1, 1)"
    )).unwrap();
    db.execute(&network_available(&info)).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, RUNNING_TASK_MEET_LIMITS);
}

// @tc.name: ut_app_state_unavailable
// @tc.desc: Test task state handling when application state is unavailable
// @tc.precon: NA
// @tc.step: 1. Initialize test database
//           2. Lock database
//           3. Execute application unavailable test cases
//           4. Verify task state transitions and reasons
// @tc.expect: Tasks transition to correct states with appropriate reasons when application is unavailable
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_app_state_unavailable() {
    test_init();
    let _lock = lock_database();
    let db = RequestDb::get_instance();
    let task_id = TaskIdGenerator::generate();
    let uid = get_current_timestamp();
    let fail_reason = get_current_timestamp() as u8;

    // running
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, mode, state, reason, action) VALUES ({task_id}, {uid}, {FRONTEND}, {RUNNING}, {fail_reason}, {DOWNLOAD})"
    )).unwrap();
    db.execute(&app_state_unavailable(uid)).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, APP_BACKGROUND_OR_TERMINATE);

    // upload
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, mode, state, reason, action) VALUES ({task_id}, {uid}, {FRONTEND}, {RUNNING}, {fail_reason}, {UPLOAD})"
    )).unwrap();
    db.execute(&app_state_unavailable(uid)).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, FAILED);
    assert_eq!(reason, APP_BACKGROUND_OR_TERMINATE);

    // retrying
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, mode, state, reason, action) VALUES ({task_id}, {uid}, {FRONTEND}, {RETRYING}, {fail_reason}, {DOWNLOAD})"
    )).unwrap();
    db.execute(&app_state_unavailable(uid)).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, APP_BACKGROUND_OR_TERMINATE);

    // other state
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, mode, state, reason) VALUES ({task_id}, {uid}, {FRONTEND}, {FAILED}, {fail_reason})"
    )).unwrap();
    db.execute(&app_state_unavailable(uid)).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, FAILED);
    assert_eq!(reason, fail_reason);

    // waiting
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, mode, state, reason) VALUES ({task_id}, {uid}, {FRONTEND}, {WAITING}, {RUNNING_TASK_MEET_LIMITS})"
    )).unwrap();
    db.execute(&app_state_unavailable(uid)).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, APP_BACKGROUND_OR_TERMINATE);

    // running + donwload
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, mode, state, action) VALUES ({task_id}, {uid}, {FRONTEND}, {RUNNING}, {DOWNLOAD})"
    )).unwrap();
    db.execute(&app_state_unavailable(uid)).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, APP_BACKGROUND_OR_TERMINATE);

    // running + upload
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, mode, state, action) VALUES ({task_id}, {uid}, {FRONTEND}, {RUNNING}, {UPLOAD})"
    )).unwrap();
    db.execute(&app_state_unavailable(uid)).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, FAILED);
    assert_eq!(reason, APP_BACKGROUND_OR_TERMINATE);

    // background
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, mode, state, action) VALUES ({task_id}, {uid}, {BACKGROUND}, {RUNNING}, {UPLOAD})"
    )).unwrap();
    db.execute(&app_state_unavailable(uid)).unwrap();

    let state: u8 = db.query_integer(&format!(
        "SELECT state FROM request_task where task_id = {task_id}"
    ))[0];
    assert_eq!(state, RUNNING);
}

// @tc.name: ut_app_state_available
// @tc.desc: Test task state handling when application state is available
// @tc.precon: NA
// @tc.step: 1. Initialize test database
//           2. Lock database
//           3. Execute application available test cases
//           4. Verify task state transitions and reasons
// @tc.expect: Tasks transition to correct states with appropriate reasons when application is available
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_app_state_available() {
    test_init();
    let _lock = lock_database();
    let db = RequestDb::get_instance();
    let task_id = TaskIdGenerator::generate();
    let uid = get_current_timestamp();

    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason) VALUES ({task_id}, {uid}, {WAITING}, {APP_BACKGROUND_OR_TERMINATE})"
    )).unwrap();
    db.execute(&app_state_available(uid)).unwrap();

    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, RUNNING_TASK_MEET_LIMITS);
}

// @tc.name: ut_account_unavailable
// @tc.desc: Test task state handling when account is unavailable
// @tc.precon: NA
// @tc.step: 1. Initialize test database
//           2. Lock database
//           3. Execute account unavailable test cases
//           4. Verify task state transitions and reasons
// @tc.expect: Tasks transition to correct states with appropriate reasons when account is unavailable
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_account_unavailable() {
    test_init();
    let _lock = lock_database();
    let db = RequestDb::get_instance();
    let task_id = TaskIdGenerator::generate();
    let uid = get_current_timestamp();
    let user = uid / 200000;

    let mut hash_set = HashSet::new();
    let states = [RUNNING, RETRYING, WAITING];
    for (i, state) in states.into_iter().enumerate() {
        db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason) VALUES ({task_id}, {uid}, {state}, {RUNNING_TASK_MEET_LIMITS})"
    )).unwrap();
        db.execute(&account_unavailable(&hash_set)).unwrap();
        let state: u8 = db.query_integer(&format!(
            "SELECT state FROM request_task where task_id = {task_id}"
        ))[0];
        let reason: u8 = db.query_integer(&format!(
            "SELECT reason FROM request_task where task_id = {task_id}"
        ))[0];
        assert_eq!(state, WAITING);
        assert_eq!(reason, ACCOUNT_STOPPED);
        hash_set.insert(user + i as u64 + 1);
    }
    let states = [COMPLETED, FAILED, PAUSED, INIT];
    for state in states.into_iter() {
        db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason) VALUES ({task_id}, {uid}, {state}, {RUNNING_TASK_MEET_LIMITS})"
    )).unwrap();
        db.execute(&account_unavailable(&hash_set)).unwrap();
        let change_state: u8 = db.query_integer(&format!(
            "SELECT state FROM request_task where task_id = {task_id}"
        ))[0];

        assert_eq!(change_state, state);
        let reason: u8 = db.query_integer(&format!(
            "SELECT reason FROM request_task where task_id = {task_id}"
        ))[0];
        assert_eq!(reason, RUNNING_TASK_MEET_LIMITS);
    }
}

// @tc.name: ut_account_available
// @tc.desc: Test task state handling when account is available
// @tc.precon: NA
// @tc.step: 1. Initialize test database
//           2. Lock database
//           3. Execute account available test cases
//           4. Verify task state transitions and reasons
// @tc.expect: Tasks transition to correct states with appropriate reasons when account is available
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_account_available() {
    test_init();
    let _lock = lock_database();
    let db = RequestDb::get_instance();
    let task_id = TaskIdGenerator::generate();
    let uid = get_current_timestamp();
    let user = uid / 200000;

    let mut hash_set = HashSet::new();

    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason) VALUES ({task_id}, {uid}, {WAITING}, {ACCOUNT_STOPPED})"
    )).unwrap();
    db.execute(&account_available(&hash_set)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, ACCOUNT_STOPPED);
    hash_set.insert(user);
    db.execute(&account_available(&hash_set)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, RUNNING_TASK_MEET_LIMITS);
}

// @tc.name: ut_multi_reason_available
// @tc.desc: Test task state handling with multiple available reasons
// @tc.precon: NA
// @tc.step: 1. Initialize test database
//           2. Lock database
//           3. Execute multiple available reason combinations test cases
//           4. Verify task state transitions and reasons
// @tc.expect: Tasks transition to correct states with appropriate reasons for multiple available combinations
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_multi_reason_available() {
    test_init();
    let _lock = lock_database();
    let db = RequestDb::get_instance();
    let task_id = TaskIdGenerator::generate();
    let uid = get_current_timestamp();
    let user = uid / 200000;

    let hash_set = HashSet::from([user]);
    let info = NetworkInfo {
        network_type: NetworkType::Cellular,
        is_metered: true,
        is_roaming: true,
    };

    // account + network
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming) VALUES ({task_id}, {uid}, {WAITING}, {NETWORK_APP_ACCOUNT}, {CELLULAR}, 1, 1)"
    )).unwrap();

    db.execute(&account_available(&hash_set)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_APP);

    db.execute(&network_available(&info)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, APP_BACKGROUND_OR_TERMINATE);

    // account + app
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming) VALUES ({task_id}, {uid}, {WAITING}, {NETWORK_APP_ACCOUNT}, {CELLULAR}, 1, 1)"
    )).unwrap();

    db.execute(&account_available(&hash_set)).unwrap();
    db.execute(&app_state_available(uid)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_OFFLINE);

    // network + app
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming) VALUES ({task_id}, {uid}, {WAITING}, {NETWORK_APP_ACCOUNT}, {CELLULAR}, 1, 1)"
    )).unwrap();
    db.execute(&network_available(&info)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, APP_ACCOUNT);

    db.execute(&app_state_available(uid)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, ACCOUNT_STOPPED);

    // network + account
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming) VALUES ({task_id}, {uid}, {WAITING}, {NETWORK_APP_ACCOUNT}, {CELLULAR}, 1, 1)"
    )).unwrap();
    db.execute(&network_available(&info)).unwrap();
    db.execute(&account_available(&hash_set)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, APP_BACKGROUND_OR_TERMINATE);

    // app + network
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming) VALUES ({task_id}, {uid}, {WAITING}, {NETWORK_APP_ACCOUNT}, {CELLULAR}, 1, 1)"
    )).unwrap();
    db.execute(&app_state_available(uid)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_ACCOUNT);

    db.execute(&network_available(&info)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, ACCOUNT_STOPPED);

    // app + account
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming) VALUES ({task_id}, {uid}, {WAITING}, {NETWORK_APP_ACCOUNT}, {CELLULAR}, 1, 1)"
    )).unwrap();
    db.execute(&app_state_available(uid)).unwrap();
    db.execute(&account_available(&hash_set)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_OFFLINE);
}

// @tc.name: ut_multi_reason_unailable
// @tc.desc: Test task state handling with multiple unavailable reasons
// @tc.precon: NA
// @tc.step: 1. Initialize test database
//           2. Lock database
//           3. Execute multiple unavailable reason combinations test cases
//           4. Verify task state transitions and reasons
// @tc.expect: Tasks transition to correct states with appropriate reasons for multiple unavailable combinations
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_multi_reason_unailable() {
    test_init();
    let _lock = lock_database();
    let db = RequestDb::get_instance();
    let task_id = TaskIdGenerator::generate();
    let uid = get_current_timestamp();
    let hash_set = HashSet::new();
    let info = NetworkInfo {
        network_type: NetworkType::Wifi,
        is_metered: true,
        is_roaming: true,
    };

    // account + offline
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {ACCOUNT_STOPPED}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&network_offline()).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_ACCOUNT);

    // account + unsupported_network
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {ACCOUNT_STOPPED}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();

    db.execute(&network_unavailable(&info).unwrap()).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_ACCOUNT);

    // account + offline + app
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {NETWORK_ACCOUNT}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&app_state_unavailable(uid)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_APP_ACCOUNT);

    // account + app
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {ACCOUNT_STOPPED}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&app_state_unavailable(uid)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, APP_ACCOUNT);

    // account + app + offline
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {APP_ACCOUNT}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&network_offline()).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_APP_ACCOUNT);

    // account + app + unsupported_network
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {APP_ACCOUNT}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&network_unavailable(&info).unwrap()).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_APP_ACCOUNT);

    // network + account
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {NETWORK_OFFLINE}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&account_unavailable(&hash_set)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_ACCOUNT);

    // unsupported_network + account
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {UNSUPPORTED_NETWORK_TYPE}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&account_unavailable(&hash_set)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_ACCOUNT);

    // network + account + app
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {NETWORK_ACCOUNT}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&app_state_unavailable(uid)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_APP_ACCOUNT);

    // network + app
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {NETWORK_OFFLINE}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&app_state_unavailable(uid)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_APP);

    // unsupported_network + app
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {UNSUPPORTED_NETWORK_TYPE}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&app_state_unavailable(uid)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_APP);

    // network + app + account
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {NETWORK_APP}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&account_unavailable(&hash_set)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_APP_ACCOUNT);

    // app + offline
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {APP_BACKGROUND_OR_TERMINATE}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&network_offline()).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_APP);

    // app + unsupported_network
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {APP_BACKGROUND_OR_TERMINATE}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&network_unavailable(&info).unwrap()).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_APP);

    // app + network + account
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {NETWORK_APP}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&account_unavailable(&hash_set)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_APP_ACCOUNT);

    // app + account
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {APP_BACKGROUND_OR_TERMINATE}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&account_unavailable(&hash_set)).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, APP_ACCOUNT);

    // app + account + offline
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {APP_ACCOUNT}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&network_offline()).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_APP_ACCOUNT);

    // app + account + unsupported_network
    db.execute(&format!(
        "INSERT OR REPLACE INTO request_task (task_id, uid, state, reason, network, metered, roaming, mode) VALUES ({task_id}, {uid}, {WAITING}, {APP_ACCOUNT}, {CELLULAR}, 1, 1, {FRONTEND})"
    )).unwrap();
    db.execute(&network_unavailable(&info).unwrap()).unwrap();
    let (state, reason) = query_state_and_reason(task_id);
    assert_eq!(state, WAITING);
    assert_eq!(reason, NETWORK_APP_ACCOUNT);
}