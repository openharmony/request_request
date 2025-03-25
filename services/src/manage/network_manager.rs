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
use std::mem::MaybeUninit;
use std::sync::atomic::Ordering;
use std::sync::{Mutex, Once};

use super::account::{BACKGROUND_ACCOUNTS, FOREGROUND_ACCOUNT};
use super::network::{NetworkInner, NetworkState};
use super::task_manager::TaskManagerTx;
use crate::manage::network::Network;
use crate::utils::call_once;

pub(crate) struct NetworkManager {
    pub(crate) network: Network,
    pub(crate) tx: Option<TaskManagerTx>,
}

impl NetworkManager {
    pub(crate) fn get_instance() -> &'static Mutex<NetworkManager> {
        static mut NETWORK_MANAGER: MaybeUninit<Mutex<NetworkManager>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        unsafe {
            call_once(&ONCE, || {
                let inner = NetworkInner::new();
                let network = Network {
                    inner,
                    _registry: None,
                };
                let network_manager = NetworkManager { network, tx: None };
                NETWORK_MANAGER.write(Mutex::new(network_manager));
            });
            &*NETWORK_MANAGER.as_ptr()
        }
    }

    pub(crate) fn is_online() -> bool {
        let network_manager = NetworkManager::get_instance().lock().unwrap();
        matches!(network_manager.network.state(), NetworkState::Online(_))
    }

    pub(super) fn query_network() -> NetworkState {
        let network_manager = NetworkManager::get_instance().lock().unwrap();
        network_manager.network.state()
    }

    pub(super) fn query_active_accounts() -> (u64, HashSet<u64>) {
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
