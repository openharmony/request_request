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

//! SQL statement generation for task state management.
//! 
//! This module provides functionality to generate SQL statements that update
//! task states in the database based on system state changes, including network
//! status, account activity, and application foreground/background transitions.

use std::collections::HashSet;

use crate::config::{Action, Mode, Version};
use crate::info::State;
use crate::manage::network::{NetworkInfo, NetworkState, NetworkType};
use crate::task::reason::Reason;

// State constants for SQL statements
const INITIALIZED: u8 = State::Initialized.repr;
const RUNNING: u8 = State::Running.repr;
const RETRYING: u8 = State::Retrying.repr;
const WAITING: u8 = State::Waiting.repr;
const PAUSED: u8 = State::Paused.repr;
const STOPPED: u8 = State::Stopped.repr;
const FAILED: u8 = State::Failed.repr;

// Reason constants for SQL statements
const APP_BACKGROUND_OR_TERMINATE: u8 = Reason::AppBackgroundOrTerminate.repr;
const RUNNING_TASK_MEET_LIMITS: u8 = Reason::RunningTaskMeetLimits.repr;
const ACCOUNT_STOPPED: u8 = Reason::AccountStopped.repr;
const NETWORK_OFFLINE: u8 = Reason::NetworkOffline.repr;
const UNSUPPORTED_NETWORK_TYPE: u8 = Reason::UnsupportedNetworkType.repr;
const NETWORK_APP: u8 = Reason::NetworkApp.repr;
const NETWORK_ACCOUNT: u8 = Reason::NetworkAccount.repr;
const APP_ACCOUNT: u8 = Reason::AppAccount.repr;
const NETWORK_APP_ACCOUNT: u8 = Reason::NetworkAppAccount.repr;

// Action constants for SQL statements
const DOWNLOAD: u8 = Action::Download.repr;
const UPLOAD: u8 = Action::Upload.repr;

// Mode constants for SQL statements
const BACKGROUND: u8 = Mode::BackGround.repr;
const FRONTEND: u8 = Mode::FrontEnd.repr;

// Version constants for SQL statements
const API9: u8 = Version::API9 as u8;
const API10: u8 = Version::API10 as u8;

/// Collection of SQL statements for database updates.
///
/// This struct provides methods to generate and store SQL statements that update
/// task states based on system state changes.
pub(crate) struct SqlList {
    /// Internal storage for SQL statements.
    sqls: Vec<String>,
}

impl SqlList {
    /// Creates a new empty collection of SQL statements.
    ///
    /// # Returns
    ///
    /// A new `SqlList` with an empty statement collection.
    pub(crate) fn new() -> Self {
        SqlList { sqls: Vec::new() }
    }

    /// Adds SQL statements for network state changes.
    ///
    /// # Arguments
    ///
    /// * `info` - The new network state information.
    pub(crate) fn add_network_change(&mut self, info: &NetworkState) {
        match info {
            NetworkState::Online(info) => {
                // Add SQL for tasks that can run on available network
                self.sqls.push(network_available(info));
                // Add SQL for tasks that cannot run on this network if applicable
                if let Some(sql) = network_unavailable(info) {
                    self.sqls.push(sql);
                }
            }
            NetworkState::Offline => {
                // Add SQL for offline network state
                self.sqls.push(network_offline());
            }
        }
    }

    /// Adds SQL statements for account state changes.
    ///
    /// # Arguments
    ///
    /// * `active_accounts` - Set of currently active user accounts.
    pub(crate) fn add_account_change(&mut self, active_accounts: &HashSet<u64>) {
        // Add SQL for tasks belonging to active accounts
        self.sqls.push(account_available(active_accounts));
        // Add SQL for tasks belonging to inactive accounts
        self.sqls.push(account_unavailable(active_accounts));
    }

    /// Adds SQL statement for when an application becomes available (foreground).
    ///
    /// # Arguments
    ///
    /// * `top_uid` - The UID of the application that moved to foreground.
    pub(crate) fn add_app_state_available(&mut self, top_uid: u64) {
        self.sqls.push(app_state_available(top_uid));
    }

    /// Adds SQL statement for when an application becomes unavailable (background/timeout).
    ///
    /// # Arguments
    ///
    /// * `uid` - The UID of the application that moved to background or timed out.
    pub(crate) fn add_app_state_unavailable(&mut self, uid: u64) {
        self.sqls.push(app_state_unavailable(uid));
    }

    /// Adds SQL statement for application uninstallation.
    ///
    /// # Arguments
    ///
    /// * `uid` - The UID of the uninstalled application.
    pub(crate) fn add_app_uninstall(&mut self, uid: u64) {
        self.sqls.push(app_uninstall(uid));
    }
    
    /// Adds SQL statement for special process termination.
    ///
    /// # Arguments
    ///
    /// * `uid` - The UID of the terminated special process.
    pub(crate) fn add_special_process_terminate(&mut self, uid: u64) {
        self.sqls.push(special_process_terminate(uid));
    }
}

impl Iterator for SqlList {
    type Item = String;

    /// Returns the next SQL statement in the collection.
    ///
    /// # Returns
    ///
    /// The next SQL statement as a string, or `None` if no more statements.
    ///
    /// # Note
    ///
    /// Statements are returned in reverse order (last added is returned first).
    fn next(&mut self) -> Option<Self::Item> {
        self.sqls.pop()
    }
}

/// Generates SQL to delete tasks for an uninstalled application.
///
/// # Arguments
///
/// * `uid` - The UID of the uninstalled application.
///
/// # Returns
///
/// SQL statement to delete all tasks belonging to the uninstalled application.
pub(crate) fn app_uninstall(uid: u64) -> String {
    format!("DELETE FROM request_task WHERE uid = {}", uid)
}

/// Generates SQL to update task states when an application becomes unavailable.
///
/// # Arguments
///
/// * `uid` - The UID of the application that moved to background or timed out.
///
/// # Returns
///
/// SQL statement to update task states and reasons based on application unavailability.
/// - Downloads are set to waiting state
/// - Uploads are set to failed state
/// - Existing waiting tasks have their reasons combined with app background reason
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

/// Generates SQL to update task states when an application becomes available.
///
/// # Arguments
///
/// * `uid` - The UID of the application that moved to foreground.
///
/// # Returns
///
/// SQL statement to restore original task reasons when an application becomes available again.
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

/// Generates SQL to update task states for inactive accounts.
///
/// # Arguments
///
/// * `active_accounts` - Set of currently active user accounts.
///
/// # Returns
///
/// SQL statement to update task states and reasons for tasks belonging to inactive accounts.
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

    // Add all active account IDs to the NOT IN clause
    for active_account in active_accounts {
        sql.push_str(&format!("{},", active_account));
    }
    // Remove trailing comma if accounts were added
    if !active_accounts.is_empty() {
        sql.pop();
    }

    sql.push(')');
    sql
}

/// Generates SQL to update task states for active accounts.
///
/// # Arguments
///
/// * `active_accounts` - Set of currently active user accounts.
///
/// # Returns
///
/// SQL statement to restore original task reasons for tasks belonging to active accounts.
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

    // Add all active account IDs to the IN clause
    for active_account in active_accounts {
        sql.push_str(&format!("{},", active_account));
    }
    // Remove trailing comma if accounts were added
    if !active_accounts.is_empty() {
        sql.pop();
    }
    sql.push(')');
    sql
}

/// Generates SQL to update task states when network goes offline.
///
/// # Returns
///
/// SQL statement to update task states and reasons based on network offline status.
/// Different handling for:
/// - API9 downloads (wait)
/// - API10 background downloads with retry (wait)
/// - API9 uploads (fail)
/// - API10 foreground downloads or no retry (fail)
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

/// Generates SQL to update task states for unsupported network conditions.
///
/// # Arguments
///
/// * `info` - Current network information.
///
/// # Returns
///
/// SQL statement to update task states and reasons for tasks that cannot run on the current network,
/// or `None` if network type is Other.
pub(super) fn network_unavailable(info: &NetworkInfo) -> Option<String> {
    // Skip if network type is Other
    if info.network_type == NetworkType::Other {
        return None;
    }
    
    // Build condition for tasks that can't run on this network
    let mut unsupported_condition = format!("network != {}", info.network_type.repr);
    
    // Add metered condition if current network is metered
    if info.is_metered {
        unsupported_condition.push_str(" OR metered = 0");
    }
    
    // Add roaming condition if current network is roaming
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

/// Generates SQL to update task states when network becomes available.
///
/// # Arguments
///
/// * `info` - Current network information.
///
/// # Returns
///
/// SQL statement to restore original task reasons for tasks that can run on the current network.
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

    // Skip network-specific conditions if network type is Other
    if info.network_type == NetworkType::Other {
        return sql;
    }

    // Add conditions for network type matching
    sql.push_str(&format!(
        " AND (network = 0 OR network = {}",
        info.network_type.repr
    ));
    
    // Add metered condition if current network is metered
    if info.is_metered {
        sql.push_str(" AND metered = 1");
    }
    
    // Add roaming condition if current network is roaming
    if info.is_roaming {
        sql.push_str(" AND roaming = 1");
    }
    
    sql.push(')');
    sql
}

/// Generates SQL to update task states when a special process terminates.
///
/// # Arguments
///
/// * `uid` - The UID of the terminated special process.
///
/// # Returns
///
/// SQL statement to set all tasks for the terminated process to failed state.
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
