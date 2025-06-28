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

const INITIALIZED: u8 = State::Initialized.repr;
const RUNNING: u8 = State::Running.repr;
const RETRYING: u8 = State::Retrying.repr;
const WAITING: u8 = State::Waiting.repr;
const PAUSED: u8 = State::Paused.repr;
const STOPPED: u8 = State::Stopped.repr;
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

    pub(crate) fn add_app_uninstall(&mut self, uid: u64) {
        self.sqls.push(app_uninstall(uid));
    }
    pub(crate) fn add_special_process_terminate(&mut self, uid: u64) {
        self.sqls.push(special_process_terminate(uid));
    }
}

impl Iterator for SqlList {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.sqls.pop()
    }
}

pub(crate) fn app_uninstall(uid: u64) -> String {
    format!("DELETE FROM request_task WHERE uid = {}", uid)
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

pub(crate) fn special_process_terminate(uid: u64) -> String {
    format!(
        "UPDATE request_task
        SET
            state = {FAILED},
            reason = {APP_BACKGROUND_OR_TERMINATE}
        WHERE
            uid = {uid}
            AND (
                state = {INITIALIZED}
                OR state = {RUNNING}
                OR state = {RETRYING}
                OR state = {WAITING}
                OR state = {PAUSED}
                OR state = {STOPPED}
            );",
    )
}

#[cfg(feature = "oh")]
#[cfg(test)]
mod ut_sql {
    include!("../../../../tests/ut/manage/scheduler/state/ut_sql.rs");
}
