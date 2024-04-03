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

use std::sync::atomic::Ordering;

use crate::error::ErrorCode;
use crate::manage::TaskManager;
use crate::task::info::State;
use crate::task::reason::Reason;

impl TaskManager {
    pub(crate) fn stop(&mut self, uid: u64, task_id: u32) -> ErrorCode {
        if let Some(task) = self.get_task(uid, task_id) {
            if !task.set_status(State::Stopped, Reason::UserOperation) {
                let state = task.status.lock().unwrap().state;
                error!(
                    "TaskManager can not stop task_id: {} that state is {:?}",
                    task_id, state
                );
                return ErrorCode::TaskStateErr;
            }
            debug!(
                "TaskManager stop a task, uid: {}, task_id:{} success",
                uid, task_id
            );
            task.resume.store(false, Ordering::SeqCst);
            ErrorCode::ErrOk
        } else {
            if self.tasks.contains_key(&task_id) {
                error!("TaskManager stop a task, task_id:{} exist, but not found in app_task_map, uid:{}", task_id, uid);
            } else {
                error!(
                    "TaskManager stop a task, uid:{}, task_id:{} not exist",
                    uid, task_id
                );
            }
            ErrorCode::TaskStateErr
        }
    }
}
