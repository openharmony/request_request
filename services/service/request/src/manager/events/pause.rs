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

use crate::error::ErrorCode;
use crate::manager::TaskManager;
use crate::task::reason::Reason;

impl TaskManager {
    pub(crate) fn pause(&mut self, uid: u64, task_id: u32) -> ErrorCode {
        debug!("TaskManager pause, uid:{}, task_id:{}", uid, task_id);

        match self.get_task(uid, task_id) {
            Some(task) => self.pause_task(task, Reason::UserOperation),
            None => {
                if self.tasks.contains_key(&task_id) {
                    error!("TaskManager pause a task, task_id:{} exist, but not found in app_task_map, uid:{}", task_id, uid);
                } else {
                    error!(
                        "TaskManager pause a task, uid:{}, task_id:{} not exist",
                        uid, task_id
                    );
                }
                ErrorCode::TaskStateErr
            }
        }
    }
}
