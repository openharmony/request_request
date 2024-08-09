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
#![allow(unused)]
use std::collections::HashSet;

use super::sql::SqlList;
use crate::manage::network::NetworkState;

pub(super) struct StateRecord {
    pub(super) top_uid: Option<u64>,
    pub(super) top_user: u64,
    pub(super) network: NetworkState,
    pub(super) active_accounts: HashSet<u64>,
}

impl StateRecord {
    pub(crate) fn new() -> Self {
        StateRecord {
            top_uid: None,
            top_user: 0,
            network: NetworkState::Offline,
            active_accounts: HashSet::new(),
        }
    }

    pub(super) fn init(
        &mut self,
        network: NetworkState,
        top_uid: Option<u64>,
        foreground_account: u64,
        active_accounts: HashSet<u64>,
    ) -> SqlList {
        let mut sql_list = SqlList::new();
        sql_list.add_network_change(&network);
        sql_list.add_account_change(&active_accounts);
        if let Some(top_uid) = top_uid {
            sql_list.add_app_state_available(top_uid);
        }
        self.top_user = foreground_account;
        self.top_uid = top_uid;
        self.active_accounts = active_accounts;
        self.network = network;
        sql_list
    }

    pub(crate) fn update_network(&mut self, info: NetworkState) -> Option<SqlList> {
        if info == self.network {
            return None;
        }
        info!("update network state to {:?}", info);
        let mut sql_list = SqlList::new();
        sql_list.add_network_change(&info);
        self.network = info;
        Some(sql_list)
    }

    pub(crate) fn update_accounts(
        &mut self,
        foreground_account: u64,
        active_accounts: HashSet<u64>,
    ) -> Option<SqlList> {
        if self.active_accounts == active_accounts {
            return None;
        }
        info!("update active accounts : {:?}", active_accounts);
        let mut sql_list = SqlList::new();
        sql_list.add_account_change(&active_accounts);
        self.active_accounts = active_accounts;
        Some(sql_list)
    }

    pub(crate) fn update_top_uid(&mut self, uid: u64) -> Option<SqlList> {
        info!("update top uid to {}", uid);
        let mut sql_list = SqlList::new();
        sql_list.add_app_state_available(uid);
        self.top_uid = Some(uid);
        Some(sql_list)
    }

    pub(crate) fn update_background(&mut self) {
        if let Some(uid) = self.top_uid {
            info!("{} turn to background", uid);
        }
        self.top_uid = None;
    }

    pub(crate) fn update_background_timeout(&self, uid: u64) -> Option<SqlList> {
        if self.top_uid == Some(uid) {
            return None;
        }
        info!("{} background timeout", uid);
        let mut sql_list = SqlList::new();
        sql_list.add_app_state_unavailable(uid);
        Some(sql_list)
    }
}
