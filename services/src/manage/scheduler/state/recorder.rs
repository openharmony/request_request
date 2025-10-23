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

//! System state recording and management.
//! 
//! This module provides functionality for recording and tracking system state
//! information, including foreground applications, user accounts, network status,
//! and resource levels.
use std::collections::HashSet;

use super::sql::SqlList;
use crate::manage::network::NetworkState;
use crate::manage::scheduler::qos::RssCapacity;

/// Records and maintains current system state information.
///
/// This struct stores system state details used for task scheduling decisions.
pub(super) struct StateRecord {
    /// Set of UIDs currently in the foreground.
    pub(super) foreground_abilities: HashSet<u64>,
    /// User ID currently in the foreground.
    pub(super) top_user: u64,
    /// Current network connection state.
    pub(super) network: NetworkState,
    /// Set of currently active user accounts.
    pub(super) active_accounts: HashSet<u64>,
    /// Current Resource Scheduling Service level.
    pub(super) rss_level: i32,
}

impl StateRecord {
    /// Creates a new state record with default values.
    ///
    /// # Returns
    ///
    /// A new `StateRecord` with empty collections and default state values.
    pub(crate) fn new() -> Self {
        StateRecord {
            foreground_abilities: HashSet::new(),
            top_user: 0,
            network: NetworkState::Offline,
            active_accounts: HashSet::new(),
            rss_level: 0,
        }
    }

    /// Initializes the state record with system information.
    ///
    /// # Arguments
    ///
    /// * `network` - Current network state.
    /// * `foreground_abilities` - Optional list of foreground application UIDs.
    /// * `foreground_account` - User ID currently in the foreground.
    /// * `active_accounts` - Set of currently active user accounts.
    ///
    /// # Returns
    ///
    /// SQL statements to update the database based on initial system state.
    pub(super) fn init(
        &mut self,
        network: NetworkState,
        foreground_abilities: Option<Vec<u64>>,
        foreground_account: u64,
        active_accounts: HashSet<u64>,
    ) -> SqlList {
        let mut sql_list = SqlList::new();
        // Add network change SQL statement
        sql_list.add_network_change(&network);
        // Add account change SQL statement
        sql_list.add_account_change(&active_accounts);
        
        // Process foreground applications if available
        if let Some(foreground_abilities) = foreground_abilities {
            for foreground_ability in foreground_abilities {
                sql_list.add_app_state_available(foreground_ability);
                self.foreground_abilities.insert(foreground_ability);
            }
        }
        
        // Update internal state
        self.top_user = foreground_account;
        self.active_accounts = active_accounts;
        self.network = network;
        
        sql_list
    }

    /// Updates the Resource Scheduling Service level.
    ///
    /// # Arguments
    ///
    /// * `rss_level` - The new RSS level.
    ///
    /// # Returns
    ///
    /// Updated RSS capacity if the level changed, or `None` if no change.
    pub(crate) fn update_rss_level(&mut self, rss_level: i32) -> Option<RssCapacity> {
        // Skip update if level hasn't changed
        if rss_level == self.rss_level {
            return None;
        }
        
        self.rss_level = rss_level;
        Some(RssCapacity::new(rss_level))
    }

    /// Updates the network state information.
    ///
    /// # Arguments
    ///
    /// * `info` - New network state information.
    ///
    /// # Returns
    ///
    /// SQL statements to update the database if network state changed, or `None` if no change.
    pub(crate) fn update_network(&mut self, info: NetworkState) -> Option<SqlList> {
        // Skip update if network state hasn't changed
        if info == self.network {
            return None;
        }
        
        info!("update network to {:?}", info);
        let mut sql_list = SqlList::new();
        sql_list.add_network_change(&info);
        self.network = info;
        Some(sql_list)
    }

    /// Updates account state information.
    ///
    /// # Arguments
    ///
    /// * `foreground_account` - User ID currently in the foreground.
    /// * `active_accounts` - Set of currently active user accounts.
    ///
    /// # Returns
    ///
    /// SQL statements to update the database if account state changed, or `None` if no change.
    pub(crate) fn update_accounts(
        &mut self,
        foreground_account: u64,
        active_accounts: HashSet<u64>,
    ) -> Option<SqlList> {
        // Skip update if active accounts haven't changed
        if self.active_accounts == active_accounts {
            return None;
        }
        
        info!("update active accounts {:?}", active_accounts);
        let mut sql_list = SqlList::new();
        sql_list.add_account_change(&active_accounts);
        
        // Update internal account state
        self.active_accounts = active_accounts;
        self.top_user = foreground_account;
        
        Some(sql_list)
    }

    /// Updates the top (foreground) UID.
    ///
    /// # Arguments
    ///
    /// * `uid` - The UID of the application that moved to foreground.
    ///
    /// # Returns
    ///
    /// SQL statements to update the database with the new foreground application state.
    pub(crate) fn update_top_uid(&mut self, uid: u64) -> Option<SqlList> {
        info!("update top uid {}", uid);
        let mut sql_list = SqlList::new();
        sql_list.add_app_state_available(uid);
        self.foreground_abilities.insert(uid);
        Some(sql_list)
    }

    /// Updates the background state for a UID.
    ///
    /// # Arguments
    ///
    /// * `uid` - The UID of the application that moved to background.
    pub(crate) fn update_background(&mut self, uid: u64) {
        // Only log if the UID was actually in the foreground set
        if self.foreground_abilities.remove(&uid) {
            info!("{} turn to background", uid);
        }
    }

    /// Updates the background timeout state for a UID.
    ///
    /// # Arguments
    ///
    /// * `uid` - The UID that has been in background for the timeout period.
    ///
    /// # Returns
    ///
    /// SQL statements to update the database if the UID is in background,
    /// or `None` if the UID is still in foreground.
    pub(crate) fn update_background_timeout(&self, uid: u64) -> Option<SqlList> {
        // Skip if the UID is still in foreground
        if self.foreground_abilities.contains(&uid) {
            return None;
        }
        
        info!("{} background timeout", uid);
        let mut sql_list = SqlList::new();
        sql_list.add_app_state_unavailable(uid);
        Some(sql_list)
    }
}
