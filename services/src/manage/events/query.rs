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

use crate::manage::database::Database;
use crate::manage::TaskManager;
use crate::task::config::Action;
use crate::task::info::TaskInfo;

impl TaskManager {
    pub(crate) fn query(&self, task_id: u32, action: Action) -> Option<TaskInfo> {
        debug!(
            "TaskManager Query, task_id: {}, query_action: {:?}",
            task_id, action
        );

        let mut info = match Database::get_instance().get_task_info(task_id) {
            Some(info) => info,
            None => {
                info!("TaskManger Query: no task found");
                return None;
            }
        };

        if info.action() == action || action == Action::Any {
            info.data = "".to_string();
            info.url = "".to_string();
            debug!("TaskManager Query, query task info is {:?}", info);
            Some(info)
        } else {
            info!("TaskManger Query: no task found");
            None
        }
    }
}
