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
use crate::task::info::TaskInfo;

impl TaskManager {
    pub(crate) fn touch(&self, uid: u64, task_id: u32, token: String) -> Option<TaskInfo> {
        debug!("TaskManager Touch, uid: {}, task_id: {}", uid, task_id);

        match self.scheduler.get_task(task_id) {
            Some(task) => {
                if task.uid() == uid && task.conf.token.eq(token.as_str()) {
                    let mut info = task.info();
                    info.bundle = "".to_string();
                    Some(info)
                } else {
                    None
                }
            }
            None => {
                let mut info = match Database::get_instance().get_task_info(task_id) {
                    Some(info) => info,
                    None => {
                        info!("TaskManger Touch: no task found");
                        return None;
                    }
                };

                if info.uid() == uid && info.token() == token {
                    info.bundle = "".to_string();
                    Some(info)
                } else {
                    info!("TaskManger Touch: no task found");
                    None
                }
            }
        }
    }
}
