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

use std::collections::HashMap;
use std::time::Duration;

use sql::SqlList;
use ylong_runtime::task::JoinHandle;

use super::qos::RssCapacity;
use crate::manage::network::NetworkState;
use crate::manage::network_manager::NetworkManager;
use crate::manage::task_manager::TaskManagerTx;
#[cfg(feature = "oh")]
#[cfg(not(test))]
use crate::utils::GetTopUid;

mod recorder;
pub(crate) mod sql;

pub(crate) struct Handler {
    recorder: recorder::StateRecord,
    background_timeout: HashMap<u64, JoinHandle<()>>,
    task_manager: TaskManagerTx,
}

impl Handler {
    pub(crate) fn new(task_manager: TaskManagerTx) -> Self {
        Handler {
            recorder: recorder::StateRecord::new(),
            background_timeout: HashMap::new(),
            task_manager,
        }
    }

    pub(crate) fn init(&mut self) -> SqlList {
        let network_info = NetworkManager::query_network();
        let (foreground_account, active_accounts) = NetworkManager::query_active_accounts();

        #[allow(unused_mut)]
        let mut top_uid = 0;

        #[cfg(not(test))]
        #[cfg(feature = "oh")]
        {
            for _ in 0..10 {
                let res = GetTopUid(&mut top_uid);
                if res != 0 || top_uid == 0 {
                    error!("Get top uid failed, res: {}", top_uid);
                    std::thread::sleep(Duration::from_millis(500));
                }
            }
        }
        let top_uid = if top_uid == 0 {
            None
        } else {
            Some(top_uid as u64)
        };
        self.recorder
            .init(network_info, top_uid, foreground_account, active_accounts)
    }

    pub(crate) fn update_rss_level(&mut self, level: i32) -> Option<RssCapacity> {
        self.recorder.update_rss_level(level)
    }

    pub(crate) fn update_network(&mut self, _a: ()) -> Option<SqlList> {
        let network_info = NetworkManager::query_network();
        self.recorder.update_network(network_info)
    }

    pub(crate) fn update_account(&mut self, _a: ()) -> Option<SqlList> {
        let (foreground_account, active_accounts) = NetworkManager::query_active_accounts();
        self.recorder
            .update_accounts(foreground_account, active_accounts)
    }

    pub(crate) fn update_top_uid(&mut self, top_uid: u64) -> Option<SqlList> {
        if self.top_uid() == Some(top_uid) {
            return None;
        }
        if let Some(uid) = self.top_uid() {
            self.update_background(uid);
        }
        if let Some(handle) = self.background_timeout.remove(&top_uid) {
            handle.cancel();
        }
        self.recorder.update_top_uid(top_uid)
    }

    pub(crate) fn update_background(&mut self, uid: u64) -> Option<SqlList> {
        if Some(uid) != self.top_uid() {
            return None;
        }
        let task_manager = self.task_manager.clone();
        self.background_timeout.insert(
            uid,
            ylong_runtime::spawn(async move {
                ylong_runtime::time::sleep(Duration::from_secs(60)).await;
                task_manager.trigger_background_timeout(uid);
            }),
        );
        self.recorder.update_background();
        None
    }

    pub(crate) fn update_background_timeout(&mut self, uid: u64) -> Option<SqlList> {
        self.recorder.update_background_timeout(uid)
    }

    pub(crate) fn app_uninstall(&mut self, uid: u64) -> Option<SqlList> {
        let mut sql_list = SqlList::new();
        sql_list.add_app_uninstall(uid);
        Some(sql_list)
    }

    pub(crate) fn special_process_terminate(&mut self, uid: u64) -> Option<SqlList> {
        info!("hiviewx terminate handle. {:?}", uid);
        let mut sql_list = SqlList::new();
        sql_list.add_special_process_terminate(uid);
        Some(sql_list)
    }

    pub(crate) fn top_uid(&self) -> Option<u64> {
        self.recorder.top_uid
    }

    pub(crate) fn top_user(&self) -> u64 {
        self.recorder.top_user
    }

    pub(crate) fn network(&self) -> &NetworkState {
        &self.recorder.network
    }
}
