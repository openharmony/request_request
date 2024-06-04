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
use crate::task::config::TaskConfig;

impl TaskManager {
    pub(crate) fn get_task(&self, uid: u64, task_id: u32, token: String) -> Option<TaskConfig> {
        debug!("TaskManager get a task, uid:{}, task_id:{}", uid, task_id);

        if let Some(config) = Database::get_instance().get_task_config(task_id) {
            debug!("found single task in database, task_id:{}", task_id);
            if config.token.eq(token.as_str()) {
                return Some(config);
            }
            debug!("get task token not equal");
            return None;
        }
        debug!("get task not found");
        None
    }
}
