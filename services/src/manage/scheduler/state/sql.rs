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

use crate::config::Mode;
use crate::info::State;
use crate::manage::network::{NetworkInfo, NetworkState, NetworkType};
use crate::task::reason::Reason;

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
                self.sqls.push(network_unavailable(info));
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
        "UPDATE request_task SET state = {}, reason = {} WHERE uid = {} AND mode = {} AND (state = {} AND reason = {} OR state = {} OR state = {})",
        State::Waiting.repr,
        Reason::AppBackgroundOrTerminate.repr,
        uid,
        Mode::FrontEnd.repr,
        State::Waiting.repr,
        Reason::RunningTaskMeetLimits.repr,
        State::Running.repr,
        State::Retrying.repr,
    )
}

pub(crate) fn app_state_available(top_uid: u64) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} WHERE uid = {} AND mode = {} AND state = {} AND reason = {}",
        State::Waiting.repr,
        Reason::RunningTaskMeetLimits.repr,
        top_uid,
        Mode::FrontEnd.repr,
        State::Waiting.repr,
        Reason::AppBackgroundOrTerminate.repr,
    )
}

pub(super) fn account_unavailable(active_accounts: &HashSet<u64>) -> String {
    let mut sql = format!(
        "UPDATE request_task SET state = {}, reason = {} WHERE (state = {} AND reason = {}  OR state = {} OR state = {}) AND uid/200000 NOT IN (",
        State::Waiting.repr,
        Reason::AccountStopped.repr,
        State::Waiting.repr,
        Reason::RunningTaskMeetLimits.repr,
        State::Running.repr,
        State::Retrying.repr,
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
        "UPDATE request_task SET reason = {} WHERE (state = {} AND reason = {}) AND uid/200000 IN (",
        Reason::RunningTaskMeetLimits.repr,
        State::Waiting.repr,
        Reason::AccountStopped.repr,
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
            "UPDATE request_task SET state = {}, reason = {} WHERE (state = {} AND reason = {}  OR state = {} OR state = {})",
            State::Waiting.repr,
            Reason::NetworkOffline.repr,
            State::Waiting.repr,
            Reason::RunningTaskMeetLimits.repr,
            State::Running.repr,
            State::Retrying.repr,
        )
}

pub(super) fn network_unavailable(info: &NetworkInfo) -> String {
    let mut sql = format!(
            "UPDATE request_task SET state = {}, reason = {} WHERE ((state = {} AND reason = {} ) OR state = {} OR state = {})",
            State::Waiting.repr,
            Reason::UnsupportedNetworkType.repr,
            State::Waiting.repr,
            Reason::RunningTaskMeetLimits.repr,
            State::Running.repr,
            State::Retrying.repr,
        );

    let mut sql_1 = String::new();
    if info.network_type != NetworkType::Other {
        sql_1.push_str(&format!(
            "(network != {} AND network != 0)",
            info.network_type.repr
        ));
    }

    if info.is_metered {
        if !sql_1.is_empty() {
            sql_1.push_str(" OR ");
        }
        sql_1.push_str("metered = 0");
    }

    if info.is_roaming {
        if !sql_1.is_empty() {
            sql_1.push_str(" OR ");
        }
        sql_1.push_str("roaming = 0");
    }
    if !sql_1.is_empty() {
        sql = format!("{} AND ({})", sql, sql_1);
    }
    sql
}

pub(super) fn network_available(info: &NetworkInfo) -> String {
    let mut sql = format!(
        "UPDATE request_task SET reason = {} WHERE state = {} AND (reason = {} OR reason = {})",
        Reason::RunningTaskMeetLimits.repr,
        State::Waiting.repr,
        Reason::UnsupportedNetworkType.repr,
        Reason::NetworkOffline.repr,
    );

    if info.network_type != NetworkType::Other {
        sql.push_str(&format!(
            " AND (network = {} OR network = 0)",
            info.network_type.repr
        ));
    }

    if info.is_metered {
        sql.push_str(" AND metered = 1");
    }

    if info.is_roaming {
        sql.push_str(" AND roaming = 1");
    }
    sql
}

#[cfg(all(not(feature = "oh"), test))]
mod test {
    use rusqlite::Connection;

    const CREATE: &'static str = "CREATE TABLE IF NOT EXISTS request_task (task_id INTEGER PRIMARY KEY, uid INTEGER, token_id INTEGER, action INTEGER, mode INTEGER, cover INTEGER, network INTEGER, metered INTEGER, roaming INTEGER, ctime INTEGER, mtime INTEGER, reason INTEGER, gauge INTEGER, retry INTEGER, redirect INTEGER, tries INTEGER, version INTEGER, config_idx INTEGER, begins INTEGER, ends INTEGER, precise INTEGER, priority INTEGER, background INTEGER, bundle TEXT, url TEXT, data TEXT, token TEXT, title TEXT, description TEXT, method TEXT, headers TEXT, config_extras TEXT, mime_type TEXT, state INTEGER, idx INTEGER, total_processed INTEGER, sizes TEXT, processed TEXT, extras TEXT, form_items BLOB, file_specs BLOB, each_file_status BLOB, body_file_names BLOB, certs_paths BLOB)";
    use super::*;
    use crate::info::State;
    use crate::manage::network::{NetworkInfo, NetworkType};
    use crate::task::reason::Reason;

    #[test]
    fn ut_app_state_available() {
        let db = Connection::open_in_memory().unwrap();
        let uid: u32 = rand::random();
        let task_id: u32 = rand::random();
        db.execute(
            &CREATE,
            (), // empty list of parameters.
        )
        .unwrap();
        db.execute(
            &format!(
                "INSERT INTO request_task (task_id, uid, state, reason, mode) VALUES ({}, {}, {}, {}, {})",
                task_id,
                uid,
                State::Waiting.repr,
                Reason::AppBackgroundOrTerminate.repr,
                Mode::FrontEnd.repr,
            ),
            (),
        )
        .unwrap();
        db.execute(&app_state_available(uid as u64), ()).unwrap();

        let mut stmt = db
            .prepare(&format!(
                "SELECT state, reason from request_task WHERE task_id = {}",
                task_id
            ))
            .unwrap();
        let mut row = stmt
            .query_map([], |row| {
                Ok((row.get::<_, u8>(0).unwrap(), row.get::<_, u8>(1).unwrap()))
            })
            .unwrap();

        let (state, reason) = row.next().unwrap().unwrap();
        assert_eq!(state, State::Waiting.repr);
        assert_eq!(reason, Reason::RunningTaskMeetLimits.repr);
    }

    #[test]
    fn ut_app_state_unavailable() {
        let db = Connection::open_in_memory().unwrap();
        let uid: u64 = rand::random();
        let task_id: u32 = rand::random();
        db.execute(
            &CREATE,
            (), // empty list of parameters.
        )
        .unwrap();
        db.execute(
            &format!(
                "INSERT INTO request_task (task_id, uid, state, reason, mode) VALUES ({}, {}, {}, {}, {})",
                task_id,
                uid,
                State::Waiting.repr,
                Reason::RunningTaskMeetLimits.repr,
                Mode::FrontEnd.repr,
            ),
            (),
        )
        .unwrap();
        db.execute(&app_state_unavailable(uid), ()).unwrap();

        let mut stmt = db
            .prepare(&format!(
                "SELECT state, reason from request_task WHERE task_id = {}",
                task_id
            ))
            .unwrap();
        let mut row = stmt
            .query_map([], |row| {
                Ok((row.get::<_, u8>(0).unwrap(), row.get::<_, u8>(1).unwrap()))
            })
            .unwrap();

        let (state, reason) = row.next().unwrap().unwrap();
        assert_eq!(state, State::Waiting.repr);
        assert_eq!(reason, Reason::AppBackgroundOrTerminate.repr);
    }

    #[test]
    fn ut_account_available() {
        let task_id: u32 = rand::random();
        let db = Connection::open_in_memory().unwrap();
        db.execute(
            &CREATE,
            (), // empty list of parameters.
        )
        .unwrap();
        db.execute(
            &format!(
                "INSERT INTO request_task (task_id, uid, state, reason) VALUES ({}, {}, {}, {})",
                task_id,
                20010044,
                State::Waiting.repr,
                Reason::AccountStopped.repr,
            ),
            (),
        )
        .unwrap();
        let mut active_accounts = HashSet::new();
        active_accounts.insert(100);
        db.execute(&account_available(&active_accounts), ())
            .unwrap();
        let mut stmt = db
            .prepare(&format!(
                "SELECT state, reason from request_task WHERE task_id = {}",
                task_id
            ))
            .unwrap();
        let mut row = stmt
            .query_map([], |row| {
                Ok((row.get::<_, u8>(0).unwrap(), row.get::<_, u8>(1).unwrap()))
            })
            .unwrap();
        let (state, reason) = row.next().unwrap().unwrap();
        assert_eq!(state, State::Waiting.repr);
        assert_eq!(reason, Reason::RunningTaskMeetLimits.repr);
    }

    #[test]
    fn ut_account_unavailable() {
        let task_id: u32 = rand::random();
        let db = Connection::open_in_memory().unwrap();
        db.execute(
            &CREATE,
            (), // empty list of parameters.
        )
        .unwrap();
        db.execute(
            &format!(
                "INSERT INTO request_task (task_id, uid, state, reason) VALUES ({}, {}, {}, {})",
                task_id,
                20010044,
                State::Waiting.repr,
                Reason::RunningTaskMeetLimits.repr,
            ),
            (),
        )
        .unwrap();
        let mut active_accounts = HashSet::new();
        active_accounts.insert(103);
        db.execute(&account_unavailable(&active_accounts), ())
            .unwrap();
        let mut stmt = db
            .prepare(&format!(
                "SELECT state, reason from request_task WHERE task_id = {}",
                task_id
            ))
            .unwrap();
        let mut row = stmt
            .query_map([], |row| {
                Ok((row.get::<_, u8>(0).unwrap(), row.get::<_, u8>(1).unwrap()))
            })
            .unwrap();
        let (state, reason) = row.next().unwrap().unwrap();
        assert_eq!(state, State::Waiting.repr);
        assert_eq!(reason, Reason::AccountStopped.repr);
    }

    #[test]
    fn ut_network_database_available() {
        let task_id: u32 = rand::random();
        let db = Connection::open_in_memory().unwrap();
        db.execute(
            &CREATE,
            (), // empty list of parameters.
        )
        .unwrap();
        db.execute(&format!(
            "INSERT INTO request_task (task_id, state, reason, network,  metered, roaming) VALUES ({}, {}, {}, {}, 0, 0)",
            task_id,
            State::Waiting.repr,
            Reason::UnsupportedNetworkType.repr,
            NetworkType::Wifi.repr,
        ),())
        .unwrap();
        let info = NetworkInfo {
            network_type: NetworkType::Wifi,
            is_metered: false,
            is_roaming: false,
        };

        db.execute(&network_available(&info), ()).unwrap();

        let mut stmt = db
            .prepare(&format!(
                "SELECT task_id from request_task where state = {} AND reason = {}",
                State::Waiting.repr,
                Reason::RunningTaskMeetLimits.repr
            ))
            .unwrap();
        let mut rows = stmt.query_map([], |row| Ok(row.get(0).unwrap())).unwrap();
        assert_eq!(task_id, rows.next().unwrap().unwrap());
    }

    #[test]
    fn ut_network_database_unavailable() {
        let task_id: u32 = rand::random();
        let db = Connection::open_in_memory().unwrap();
        db.execute(
            &CREATE,
            (), // empty list of parameters.
        )
        .unwrap();
        db.execute(
            &format!(
                "INSERT INTO request_task (task_id, state, reason, network,
    metered, roaming) VALUES ({}, {}, {}, {}, 1, 1)",
                task_id,
                State::Waiting.repr,
                Reason::RunningTaskMeetLimits.repr,
                NetworkType::Wifi.repr,
            ),
            (),
        )
        .unwrap();

        let info = NetworkInfo {
            network_type: NetworkType::Wifi,
            is_metered: true,
            is_roaming: true,
        };
        db.execute(&network_unavailable(&info), ()).unwrap();

        let mut stmt = db
            .prepare(&format!(
                "SELECT task_id from request_task WHERE state = {} AND reason =
    {}",
                State::Waiting.repr,
                Reason::UnsupportedNetworkType.repr
            ))
            .unwrap();
        let mut rows = stmt
            .query_map([], |row| Ok(row.get::<_, u32>(0).unwrap()))
            .unwrap();
        assert!(rows.next().is_none());

        let info = NetworkInfo {
            network_type: NetworkType::Cellular,
            is_metered: true,
            is_roaming: true,
        };

        db.execute(&network_unavailable(&info), ()).unwrap();

        let mut stmt = db
            .prepare(&format!(
                "SELECT task_id from request_task WHERE state = {} AND reason = {}",
                State::Waiting.repr,
                Reason::UnsupportedNetworkType.repr
            ))
            .unwrap();
        let mut rows = stmt.query_map([], |row| Ok(row.get(0).unwrap())).unwrap();
        assert_eq!(task_id, rows.next().unwrap().unwrap());
    }

    #[test]
    fn ut_network_database_offline() {
        let task_id: u32 = rand::random();
        let db = Connection::open_in_memory().unwrap();
        db.execute(
            &CREATE,
            (), // empty list of parameters.
        )
        .unwrap();
        db.execute(
            &format!("INSERT INTO request_task (task_id, state, reason, network, metered, roaming) VALUES ({}, {}, {}, {}, 1, 1)",
                task_id,
                State::Waiting.repr,
                Reason::RunningTaskMeetLimits.repr,
                NetworkType::Wifi.repr,
            ),
            (),
        )
        .unwrap();

        db.execute(&network_offline(), ()).unwrap();

        let mut stmt = db
            .prepare(&format!(
                "SELECT task_id from request_task WHERE state = {} AND reason = {}",
                State::Waiting.repr,
                Reason::NetworkOffline.repr
            ))
            .unwrap();

        let mut rows = stmt.query_map([], |row| Ok(row.get(0).unwrap())).unwrap();
        assert_eq!(task_id, rows.next().unwrap().unwrap());
    }
}
