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

//! System state management for task scheduling.
//! 
//! This module handles the tracking and management of system state information
//! that affects task scheduling decisions, including network status, account
//! activity, foreground processes, and resource availability.

use std::collections::{HashMap, HashSet};
use std::time::Duration;

use sql::SqlList;
use ylong_runtime::task::JoinHandle;

use super::qos::RssCapacity;
use crate::manage::account;
use crate::manage::network::NetworkState;
use crate::manage::network_manager::NetworkManager;
use crate::manage::task_manager::TaskManagerTx;
use crate::utils::runtime_spawn;
#[cfg(feature = "oh")]
#[cfg(not(test))]
use crate::utils::GetForegroundAbilities;

mod recorder;
pub(crate) mod sql;

/// Handler for managing and responding to system state changes.
///
/// This struct coordinates system state information and triggers appropriate
/// scheduling actions when the system state changes.
pub(crate) struct Handler {
    /// Record keeping component that tracks and maintains system state.
    recorder: recorder::StateRecord,
    /// Map of background timeout handles, keyed by UID.
    background_timeout: HashMap<u64, JoinHandle<()>>,
    /// Transmitter for sending events to the task manager.
    task_manager: TaskManagerTx,
}

impl Handler {
    /// Creates a new state handler with empty state tracking collections.
    ///
    /// # Arguments
    ///
    /// * `task_manager` - Task manager transmitter for sending state change events.
    ///
    /// # Returns
    ///
    /// A new `Handler` with initialized components and empty collections.
    pub(crate) fn new(task_manager: TaskManagerTx) -> Self {
        Handler {
            recorder: recorder::StateRecord::new(),
            background_timeout: HashMap::new(),
            task_manager,
        }
    }

    /// Initializes the state handler with current system information.
    ///
    /// Queries and initializes system state including network information,
    /// active accounts, and foreground applications.
    ///
    /// # Returns
    ///
    /// SQL statements to update the database based on initial system state.
    pub(crate) fn init(&mut self) -> SqlList {
        // Get current network information
        let network_info = NetworkManager::query_network();
        // Get account information
        let (foreground_account, active_accounts) = account::query_active_accounts();

        #[allow(unused_mut)]
        let mut foreground_abilities = vec![];

        #[cfg(not(test))]
        #[cfg(feature = "oh")]
        {
            // Retry up to 10 times to get foreground abilities
            for _ in 0..10 {
                let res = GetForegroundAbilities(&mut foreground_abilities);
                if res != 0 {
                    error!("Get top uid failed, res: {}", res);
                    // Wait 500ms before retry
                    std::thread::sleep(Duration::from_millis(500));
                } else {
                    break;
                }
            }
        }
        info!("foreground_abilities: {:?}", foreground_abilities);
        // Convert to Option<HashSet> for recorder
        let foreground_abilities = if foreground_abilities.is_empty() {
            None
        } else {
            Some(
                foreground_abilities
                    .into_iter()
                    .map(|a: i32| a as u64)
                    .collect(),
            )
        };
        // Initialize the state recorder with collected information
        self.recorder.init(
            network_info,
            foreground_abilities,
            foreground_account,
            active_accounts,
        )
    }

    /// Updates the RSS (Resource Scheduling Service) level.
    ///
    /// # Arguments
    ///
    /// * `level` - The new RSS level.
    ///
    /// # Returns
    ///
    /// Updated RSS capacity information if the level changed.
    pub(crate) fn update_rss_level(&mut self, level: i32) -> Option<RssCapacity> {
        self.recorder.update_rss_level(level)
    }

    /// Updates the network state information.
    ///
    /// # Arguments
    ///
    /// * `_a` - Unused parameter, placeholder for API consistency.
    ///
    /// # Returns
    ///
    /// SQL statements to update the database if network state changed.
    pub(crate) fn update_network(&mut self, _a: ()) -> Option<SqlList> {
        // Query current network state
        let network_info = NetworkManager::query_network();
        self.recorder.update_network(network_info)
    }

    /// Updates account state information.
    ///
    /// # Arguments
    ///
    /// * `_a` - Unused parameter, placeholder for API consistency.
    ///
    /// # Returns
    ///
    /// SQL statements to update the database if account state changed.
    pub(crate) fn update_account(&mut self, _a: ()) -> Option<SqlList> {
        // Query current account information
        let (foreground_account, active_accounts) = account::query_active_accounts();
        self.recorder
            .update_accounts(foreground_account, active_accounts)
    }

    /// Updates the top (foreground) UID.
    ///
    /// # Arguments
    ///
    /// * `top_uid` - The UID of the application that moved to foreground.
    ///
    /// # Returns
    ///
    /// SQL statements to update the database if top UID changed.
    pub(crate) fn update_top_uid(&mut self, top_uid: u64) -> Option<SqlList> {
        // Skip if already tracked as foreground ability
        if self.foreground_abilities().contains(&top_uid) {
            return None;
        }
        // Cancel any pending background timeout for this UID
        if let Some(handle) = self.background_timeout.remove(&top_uid) {
            handle.cancel();
        }
        self.recorder.update_top_uid(top_uid)
    }

    /// Updates the background state for a UID.
    ///
    /// # Arguments
    ///
    /// * `uid` - The UID of the application that moved to background.
    ///
    /// # Returns
    ///
    /// SQL statements to update the database if background state changed.
    pub(crate) fn update_background(&mut self, uid: u64) -> Option<SqlList> {
        // Skip if not in foreground abilities list
        if !self.foreground_abilities().contains(&uid) {
            return None;
        }
        // Spawn a timer to handle background timeout after 60 seconds
        let task_manager = self.task_manager.clone();
        self.background_timeout.insert(
            uid,
            runtime_spawn(async move {
                // Wait 60 seconds before triggering timeout
                ylong_runtime::time::sleep(Duration::from_secs(60)).await;
                task_manager.trigger_background_timeout(uid);
            }),
        );
        // Update background state in recorder
        self.recorder.update_background(uid);
        None
    }

    /// Updates the background timeout state for a UID.
    ///
    /// # Arguments
    ///
    /// * `uid` - The UID that has been in background for the timeout period.
    ///
    /// # Returns
    ///
    /// SQL statements to update the database with background timeout state.
    pub(crate) fn update_background_timeout(&mut self, uid: u64) -> Option<SqlList> {
        self.recorder.update_background_timeout(uid)
    }

    /// Handles application uninstallation for a UID.
    ///
    /// # Arguments
    ///
    /// * `uid` - The UID of the uninstalled application.
    ///
    /// # Returns
    ///
    /// SQL statements to clean up data associated with the uninstalled application.
    pub(crate) fn app_uninstall(&mut self, uid: u64) -> Option<SqlList> {
        let mut sql_list = SqlList::new();
        sql_list.add_app_uninstall(uid);
        Some(sql_list)
    }

    /// Handles termination of special processes.
    ///
    /// # Arguments
    ///
    /// * `uid` - The UID of the terminated special process.
    ///
    /// # Returns
    ///
    /// SQL statements to clean up data associated with the terminated process.
    pub(crate) fn special_process_terminate(&mut self, uid: u64) -> Option<SqlList> {
        info!("hiviewx terminate handle. {:?}", uid);
        let mut sql_list = SqlList::new();
        sql_list.add_special_process_terminate(uid);
        Some(sql_list)
    }

    /// Gets the set of foreground application UIDs.
    ///
    /// # Returns
    ///
    /// A reference to the set of UIDs currently considered foreground abilities.
    pub(crate) fn foreground_abilities(&self) -> &HashSet<u64> {
        &self.recorder.foreground_abilities
    }

    /// Gets the top (foreground) user ID.
    ///
    /// # Returns
    ///
    /// The user ID currently in the foreground.
    pub(crate) fn top_user(&self) -> u64 {
        self.recorder.top_user
    }

    /// Gets the current network state.
    ///
    /// # Returns
    ///
    /// A reference to the current network state information.
    pub(crate) fn network(&self) -> &NetworkState {
        &self.recorder.network
    }
}
