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
use crate::task::config::Action;
use crate::task::ffi::{CTaskInfo, DeleteCTaskInfo};
use crate::task::info::TaskInfo;

impl TaskManager {
    pub(crate) fn query(&self, task_id: u32, query_action: Action) -> Option<TaskInfo> {
        debug!(
            "TaskManager query, task_id:{}, query_action:{:?}",
            task_id, query_action
        );

        if let Some(task) = self.tasks.get(&task_id) {
            if task.conf.common_data.action == query_action || query_action == Action::Any {
                debug!("query task info by memory");
                let mut task_info = task.show();
                task_info.data = "".to_string();
                task_info.url = "".to_string();
                debug!("query task info is {:?}", task_info);
                return Some(task_info);
            }
        }

        debug!("query task info by database");
        let c_task_info = unsafe { Query(task_id, query_action) };
        if c_task_info.is_null() {
            return None;
        }
        let c_task_info = unsafe { &*c_task_info };
        let task_info = TaskInfo::from_c_struct(c_task_info);
        debug!("query task info is {:?}", task_info);
        unsafe { DeleteCTaskInfo(c_task_info) };
        Some(task_info)
    }
}

#[cfg(feature = "oh")]
#[link(name = "request_service_c")]
extern "C" {
    pub(crate) fn Query(taskId: u32, queryAction: Action) -> *const CTaskInfo;
}
