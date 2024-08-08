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

use crate::manage::network::{Network, NetworkState};
use crate::manage::task_manager::TaskManagerTx;
#[cfg(feature = "oh")]
use crate::utils::GetTopUid;

mod recorder;
pub(crate) mod sql;
mod updater;

pub(crate) struct Handler {
    recorder: recorder::StateRecord,
    updater: updater::StateUpdater,
    background_timeout: HashMap<u64, JoinHandle<()>>,
    task_manager: TaskManagerTx,
}

impl Handler {
    pub(crate) fn new(network: Network, task_manager: TaskManagerTx) -> Self {
        Handler {
            recorder: recorder::StateRecord::new(),
            updater: updater::StateUpdater::new(network),
            background_timeout: HashMap::new(),
            task_manager,
        }
    }

    pub(crate) fn init(&mut self) -> SqlList {
        let network_info = self.updater.query_network();
        let (foreground_account, active_accounts) = self.updater.query_active_accounts();
        let mut top_uid = 0;
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
        self.recorder.init(
            network_info,
            top_uid as u64,
            foreground_account,
            active_accounts,
        )
    }

    pub(crate) fn update_network(&mut self, _a: ()) -> Option<SqlList> {
        let network_info = self.updater.query_network();
        self.recorder.update_network(network_info)
    }

    pub(crate) fn update_account(&mut self, _a: ()) -> Option<SqlList> {
        let (foreground_account, active_accounts) = self.updater.query_active_accounts();
        self.recorder
            .update_accounts(foreground_account, active_accounts)
    }

    pub(crate) fn update_top_uid(&mut self, top_uid: u64) -> Option<SqlList> {
        let old_top_uid = self.top_uid();
        if old_top_uid == top_uid {
            return None;
        }

        if let Some(handle) = self.background_timeout.remove(&top_uid) {
            handle.cancel();
        }
        let task_manager = self.task_manager.clone();
        self.background_timeout.insert(
            self.top_uid(),
            ylong_runtime::spawn(async move {
                ylong_runtime::time::sleep(Duration::from_secs(60)).await;
                task_manager.trigger_background_timeout(old_top_uid);
            }),
        );
        self.recorder.update_top_uid(top_uid)
    }

    pub(crate) fn update_background_timeout(&mut self, uid: u64) -> Option<SqlList> {
        self.recorder.update_background_timeout(uid)
    }

    pub(crate) fn top_uid(&self) -> u64 {
        self.recorder.top_uid
    }

    pub(crate) fn top_user(&self) -> u64 {
        self.recorder.top_user
    }

    pub(crate) fn network(&self) -> &NetworkState {
        &self.recorder.network
    }
}