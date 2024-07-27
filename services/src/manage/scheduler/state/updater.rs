// Copyright (C) 2024 Huawei Device Co., Ltd.
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
use std::sync::atomic::Ordering;

use crate::manage::account::{BACKGROUND_ACCOUNTS, FOREGROUND_ACCOUNT};
use crate::manage::network::{Network, NetworkState};

pub(super) struct StateUpdater {
    network: Network,
}

impl StateUpdater {
    pub(super) fn new(network: Network) -> Self {
        StateUpdater { network }
    }

    pub(super) fn query_network(&self) -> NetworkState {
        self.network.state()
    }

    pub(super) fn query_active_accounts(&self) -> (u64, HashSet<u64>) {
        let mut active_accounts = HashSet::new();
        let foreground_account = FOREGROUND_ACCOUNT.load(Ordering::SeqCst) as u64;
        active_accounts.insert(foreground_account);
        if let Some(background_accounts) = BACKGROUND_ACCOUNTS.lock().unwrap().as_ref() {
            for account in background_accounts.iter() {
                active_accounts.insert(*account as u64);
            }
        }
        (foreground_account, active_accounts)
    }
}
