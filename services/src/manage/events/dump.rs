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

use crate::manage::TaskManager;
use crate::task::info::{DumpAllEachInfo, DumpAllInfo, DumpOneInfo};

impl TaskManager {
    pub(crate) fn query_one_task(&self, task_id: u32) -> Option<DumpOneInfo> {
        self.scheduler
            .tasks()
            .find(|task| task.task_id() == task_id)
            .map(|task| {
                let status = task.status.lock().unwrap();
                DumpOneInfo {
                    task_id: task.conf.common_data.task_id,
                    action: task.conf.common_data.action,
                    state: status.state,
                    reason: status.reason,
                }
            })
    }

    pub(crate) fn query_all_task(&self) -> DumpAllInfo {
        DumpAllInfo {
            vec: self
                .scheduler
                .tasks()
                .map(|task| {
                    let status = task.status.lock().unwrap();
                    DumpAllEachInfo {
                        task_id: task.conf.common_data.task_id,
                        action: task.conf.common_data.action,
                        state: status.state,
                        reason: status.reason,
                    }
                })
                .collect(),
        }
    }
}
