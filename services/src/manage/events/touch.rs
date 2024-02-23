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
use crate::utils::c_wrapper::CStringWrapper;

impl TaskManager {
    pub(crate) fn touch(&self, uid: u64, task_id: u32, token: String) -> Option<TaskInfo> {
        debug!("TaskManager touch a task, uid:{}, task_id:{}", uid, task_id);

        match self.get_task(uid, task_id) {
            Some(value) => {
                debug!("touch task info by memory");
                if value.conf.token.eq(token.as_str()) {
                    let mut task_info = value.show();
                    task_info.bundle = "".to_string();
                    return Some(task_info);
                }
                None
            }
            None => {
                debug!("TaskManger touch: touch task_info from database");
                let c_task_info = unsafe { Touch(task_id, uid, CStringWrapper::from(&token)) };
                if c_task_info.is_null() {
                    info!("TaskManger touch: no task found in database");
                    return None;
                }
                let c_task_info = unsafe { &*c_task_info };
                let task_info = TaskInfo::from_c_struct(c_task_info);
                debug!("TaskManger touch: task info is {:?}", task_info);
                unsafe { DeleteCTaskInfo(c_task_info) };
                Some(task_info)
            }
        }
    }
}

#[cfg(feature = "oh")]
#[link(name = "request_service_c")]
extern "C" {
    pub(crate) fn Touch(taskId: u32, uid: u64, token: CStringWrapper) -> *const CTaskInfo;
}
