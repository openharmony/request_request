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
use crate::task::ffi::{CTaskInfo, DeleteCTaskInfo};
use crate::task::info::TaskInfo;

impl TaskManager {
    pub(crate) fn show(&self, uid: u64, task_id: u32) -> Option<TaskInfo> {
        match self.get_task(uid, task_id) {
            Some(value) => {
                debug!("TaskManager show, uid:{}, task_id:{} success", uid, task_id);
                let task_info = value.show();
                Some(task_info)
            }
            None => {
                debug!("TaskManager show: show task info from database");
                let c_task_info = unsafe { Show(task_id, uid) };
                if c_task_info.is_null() {
                    info!(
                        "TaskManger show: no task found in database, task_id: {}",
                        task_id
                    );
                    return None;
                }
                let c_task_info = unsafe { &*c_task_info };
                let task_info = TaskInfo::from_c_struct(c_task_info);
                debug!("TaskManager show: task info is {:?}", task_info);
                unsafe { DeleteCTaskInfo(c_task_info) };
                Some(task_info)
            }
        }
    }
}

#[cfg(feature = "oh")]
#[link(name = "request_service_c")]
extern "C" {
    pub(crate) fn Show(task_id: u32, uid: u64) -> *const CTaskInfo;
}
