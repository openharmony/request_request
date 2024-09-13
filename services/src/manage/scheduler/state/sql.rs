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

use std::collections::HashSet;

use crate::config::{Action, Mode, Version};
use crate::info::State;
use crate::manage::network::{NetworkInfo, NetworkState, NetworkType};
use crate::task::reason::Reason;

const RUNNING: u8 = State::Running.repr;
const RETRYING: u8 = State::Retrying.repr;
const WAITING: u8 = State::Waiting.repr;
const FAILED: u8 = State::Failed.repr;

const APP_BACKGROUND_OR_TERMINATE: u8 = Reason::AppBackgroundOrTerminate.repr;
const RUNNING_TASK_MEET_LIMITS: u8 = Reason::RunningTaskMeetLimits.repr;
const ACCOUNT_STOPPED: u8 = Reason::AccountStopped.repr;
const NETWORK_OFFLINE: u8 = Reason::NetworkOffline.repr;
const UNSUPPORTED_NETWORK_TYPE: u8 = Reason::UnsupportedNetworkType.repr;
const NETWORK_APP: u8 = Reason::NetworkApp.repr;
const NETWORK_ACCOUNT: u8 = Reason::NetworkAccount.repr;
const APP_ACCOUNT: u8 = Reason::AppAccount.repr;
const NETWORK_APP_ACCOUNT: u8 = Reason::NetworkAppAccount.repr;

const DOWNLOAD: u8 = Action::Download.repr;
const UPLOAD: u8 = Action::Upload.repr;

const BACKGROUND: u8 = Mode::BackGround.repr;
const FRONTEND: u8 = Mode::FrontEnd.repr;

const API9: u8 = Version::API9 as u8;
const API10: u8 = Version::API10 as u8;

pub(crate) struct SqlList {
    sqls: Vec<String>,
}

impl SqlList {
    pub(crate) fn new() -> Self {
        SqlList { sqls: Vec::new() }
    }

    pub(crate) fn add_network_change(&mut self, info: &NetworkState) {
        match info {
            NetworkState::Online(info) => {
                self.sqls.push(network_available(info));
                if let Some(sql) = network_unavailable(info) {
                    self.sqls.push(sql);
                }
            }
            NetworkState::Offline => {
                self.sqls.push(network_offline());
            }
        }
    }

    pub(crate) fn add_account_change(&mut self, active_accounts: &HashSet<u64>) {
        self.sqls.push(account_available(active_accounts));
        self.sqls.push(account_unavailable(active_accounts));
    }

    pub(crate) fn add_app_state_available(&mut self, top_uid: u64) {
        self.sqls.push(app_state_available(top_uid));
    }

    pub(crate) fn add_app_state_unavailable(&mut self, uid: u64) {
        self.sqls.push(app_state_unavailable(uid));
    }
}

impl Iterator for SqlList {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.sqls.pop()
    }
}

pub(crate) fn app_state_unavailable(uid: u64) -> String {
    format!(
        "UPDATE request_task SET 
            state = CASE
                WHEN (state = {RUNNING} OR state = {RETRYING}) AND action = {DOWNLOAD} THEN {WAITING}
                WHEN (state = {RUNNING} OR state = {RETRYING}) AND action = {UPLOAD} THEN {FAILED}
                ELSE state
            END,
            reason = CASE 
                WHEN (state = {RUNNING} OR state = {RETRYING}) THEN {APP_BACKGROUND_OR_TERMINATE} 
                WHEN state = {WAITING} THEN
                    CASE reason
                        WHEN {RUNNING_TASK_MEET_LIMITS} THEN {APP_BACKGROUND_OR_TERMINATE}
                        WHEN {NETWORK_OFFLINE} THEN {NETWORK_APP}
                        WHEN {UNSUPPORTED_NETWORK_TYPE} THEN {NETWORK_APP}
                        WHEN {ACCOUNT_STOPPED} THEN {APP_ACCOUNT}
                        WHEN {NETWORK_ACCOUNT} THEN {NETWORK_APP_ACCOUNT}
                        ELSE reason
                    END
                ELSE reason 
            END
        WHERE 
            uid = {uid} AND mode = {FRONTEND}",
    )
}

pub(crate) fn app_state_available(uid: u64) -> String {
    format!(
        "UPDATE request_task SET 
            reason = CASE
                WHEN reason = {APP_BACKGROUND_OR_TERMINATE} THEN {RUNNING_TASK_MEET_LIMITS}
                WHEN reason = {NETWORK_APP} THEN {NETWORK_OFFLINE}
                WHEN reason = {APP_ACCOUNT} THEN {ACCOUNT_STOPPED}
                WHEN reason = {NETWORK_APP_ACCOUNT} THEN {NETWORK_ACCOUNT}
                ELSE reason
            END
        WHERE 
            state = {WAITING} AND uid = {uid}",
    )
}

pub(super) fn account_unavailable(active_accounts: &HashSet<u64>) -> String {
    let mut sql = format!(
        "UPDATE request_task SET 
            state = CASE
                WHEN state = {RUNNING} OR state = {RETRYING} THEN {WAITING}
                ELSE state
            END,
            reason = CASE
                WHEN (state = {RUNNING} OR state = {RETRYING}) THEN {ACCOUNT_STOPPED}
                WHEN state = {WAITING} THEN 
                    CASE reason
                        WHEN {RUNNING_TASK_MEET_LIMITS} THEN {ACCOUNT_STOPPED}
                        WHEN {NETWORK_OFFLINE} THEN {NETWORK_ACCOUNT}
                        WHEN {UNSUPPORTED_NETWORK_TYPE} THEN {NETWORK_ACCOUNT}
                        WHEN {APP_BACKGROUND_OR_TERMINATE} THEN {APP_ACCOUNT}
                        WHEN {NETWORK_APP} THEN {NETWORK_APP_ACCOUNT}
                        ELSE reason
                    END
                ELSE reason
            END  
        WHERE 
            uid/200000 NOT IN (",
    );

    for active_account in active_accounts {
        sql.push_str(&format!("{},", active_account));
    }
    if !active_accounts.is_empty() {
        sql.pop();
    }

    sql.push(')');
    sql
}

pub(super) fn account_available(active_accounts: &HashSet<u64>) -> String {
    let mut sql = format!(
        "UPDATE request_task SET 
            reason = CASE
                WHEN reason= {ACCOUNT_STOPPED} THEN {RUNNING_TASK_MEET_LIMITS}
                WHEN reason = {NETWORK_ACCOUNT} THEN {NETWORK_OFFLINE}
                WHEN reason = {APP_ACCOUNT} THEN {APP_BACKGROUND_OR_TERMINATE}
                WHEN reason = {NETWORK_APP_ACCOUNT} THEN {NETWORK_APP}
                ELSE reason
            END
        WHERE 
            state = {WAITING} AND uid/200000 IN (",
    );

    for active_account in active_accounts {
        sql.push_str(&format!("{},", active_account));
    }
    if !active_accounts.is_empty() {
        sql.pop();
    }
    sql.push(')');
    sql
}

pub(super) fn network_offline() -> String {
    format!(
        "UPDATE request_task SET 
            state = CASE 
                WHEN (state = {RUNNING} OR state = {RETRYING}) AND ((version = {API9} AND action = {DOWNLOAD}) OR (version = {API10} AND mode = {BACKGROUND} AND retry = 1)) THEN {WAITING}
                WHEN (state = {RUNNING} OR state = {RETRYING}) AND ((version = {API9} AND action = {UPLOAD}) OR (version = {API10} AND (mode = {FRONTEND} OR retry = 0))) THEN {FAILED}
                ELSE state
            END,
            reason = CASE 
                WHEN state = {RUNNING} OR state = {RETRYING} THEN {NETWORK_OFFLINE}
                WHEN state = {WAITING} THEN 
                    CASE reason
                        WHEN {RUNNING_TASK_MEET_LIMITS} THEN {NETWORK_OFFLINE}
                        WHEN {ACCOUNT_STOPPED} THEN {NETWORK_ACCOUNT}
                        WHEN {APP_BACKGROUND_OR_TERMINATE} THEN {NETWORK_APP}
                        WHEN {APP_ACCOUNT} THEN {NETWORK_APP_ACCOUNT}
                        ELSE reason
                    END
                ELSE reason
            END"
    )
}

pub(super) fn network_unavailable(info: &NetworkInfo) -> Option<String> {
    if info.network_type == NetworkType::Other {
        return None;
    }
    let mut unsupported_condition = format!("network != {}", info.network_type.repr);
    if info.is_metered {
        unsupported_condition.push_str(" OR metered = 0");
    }
    if info.is_roaming {
        unsupported_condition.push_str(" OR roaming = 0");
    }
    Some(format!(
        "UPDATE request_task SET 
            state = CASE 
                WHEN (state = {RUNNING} OR state = {RETRYING}) AND ((version = {API9} AND action = {DOWNLOAD}) OR (version = {API10} AND mode = {BACKGROUND} AND retry = 1)) THEN {WAITING}
                WHEN (state = {RUNNING} OR state = {RETRYING}) AND ((version = {API9} AND action = {UPLOAD}) OR (version = {API10} AND (mode = {FRONTEND} OR retry = 0))) THEN {FAILED}
                ELSE state
            END,
            reason = CASE 
                WHEN state = {RUNNING} OR state = {RETRYING} THEN {UNSUPPORTED_NETWORK_TYPE}
                WHEN state = {WAITING} THEN
                    CASE reason
                        WHEN {RUNNING_TASK_MEET_LIMITS} THEN {UNSUPPORTED_NETWORK_TYPE}
                        WHEN {ACCOUNT_STOPPED} THEN {NETWORK_ACCOUNT}
                        WHEN {APP_BACKGROUND_OR_TERMINATE} THEN {NETWORK_APP}
                        WHEN {APP_ACCOUNT} THEN {NETWORK_APP_ACCOUNT}
                        ELSE reason
                    END
                ELSE reason
            END
        WHERE 
            {unsupported_condition}"
    ))
}

pub(super) fn network_available(info: &NetworkInfo) -> String {
    let mut sql = format!(
        "UPDATE request_task SET 
            reason = CASE 
                WHEN reason = {NETWORK_OFFLINE} THEN {RUNNING_TASK_MEET_LIMITS}
                WHEN reason = {UNSUPPORTED_NETWORK_TYPE} THEN {RUNNING_TASK_MEET_LIMITS}
                WHEN reason = {NETWORK_ACCOUNT} THEN {ACCOUNT_STOPPED}
                WHEN reason = {NETWORK_APP} THEN {APP_BACKGROUND_OR_TERMINATE}
                WHEN reason = {NETWORK_APP_ACCOUNT} THEN {APP_ACCOUNT}
                ELSE reason
            END
        WHERE 
            state = {WAITING}",
    );

    if info.network_type == NetworkType::Other {
        return sql;
    }

    sql.push_str(&format!(
        " AND (network = 0 OR network = {}",
        info.network_type.repr
    ));
    if info.is_metered {
        sql.push_str(" AND metered = 1");
    }
    if info.is_roaming {
        sql.push_str(" AND roaming = 1");
    }
    sql.push(')');
    sql
}

#[cfg(feature = "oh")]
#[cfg(test)]
mod test {

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

    #[test]
    fn ut_network_offline() {
        test_init();
        let _lock = lock_database();
        network(&network_offline(), NETWORK_OFFLINE);
    }

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
}
