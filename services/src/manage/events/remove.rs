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
use crate::manage::TaskManager;
use crate::task::ffi::{CTaskInfo, ChangeRequestTaskState, DeleteCTaskInfo};
use crate::task::info::{State, TaskInfo};
use crate::task::reason::Reason;
cfg_oh! {
    use crate::manage::notifier::Notifier;
}

impl TaskManager {
    pub(crate) fn remove(&mut self, uid: u64, task_id: u32) -> ErrorCode {
        if let Some(task) = self.get_task(uid, task_id) {
            task.set_status(State::Removed, Reason::UserOperation);
            #[cfg(feature = "oh")]
            let notify_data = task.build_notify_data();
            Notifier::remove_notify(notify_data);
            self.after_task_processed(&task);
            debug!(
                "TaskManager remove a task, uid:{}, task_id:{} success",
                uid, task_id
            );
            return ErrorCode::ErrOk;
        }
        let c_task_info = unsafe { Show(task_id, uid) };
        if !c_task_info.is_null() {
            let c_task_info = unsafe { &*c_task_info };
            let task_info = TaskInfo::from_c_struct(c_task_info);
            unsafe { DeleteCTaskInfo(c_task_info) };
            if State::from(task_info.progress.common_data.state) == State::Removed {
                error!(
                    "TaskManager remove a task, uid:{}, task_id:{} removed already",
                    uid, task_id
                );
                ErrorCode::TaskNotFound
            } else {
                let notify_data = task_info.build_notify_data();
                #[cfg(feature = "oh")]
                Notifier::remove_notify(notify_data);
                debug!(
                    "TaskManager remove a task, uid:{}, task_id:{} success",
                    uid, task_id
                );
                unsafe {
                    ChangeRequestTaskState(task_id, uid, State::Removed);
                }
                ErrorCode::ErrOk
            }
        } else {
            if self.tasks.contains_key(&task_id) {
                error!("TaskManager remove a task, task_id:{} exist, but not found in app_task_map, uid:{}", task_id, uid);
            } else {
                error!(
                    "TaskManager remove a task, uid:{}, task_id:{} not exist",
                    uid, task_id
                );
            }
            ErrorCode::TaskNotFound
        }
    }
}

#[cfg(feature = "oh")]
#[link(name = "request_service_c")]
extern "C" {
    pub(crate) fn Show(task_id: u32, uid: u64) -> *const CTaskInfo;
}
