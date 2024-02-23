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

use super::show::Show;
use crate::manage::TaskManager;
use crate::task::ffi::DeleteCTaskInfo;
use crate::task::info::TaskInfo;

impl TaskManager {
    pub(crate) fn query_mime_type(&self, uid: u64, task_id: u32) -> String {
        debug!(
            "TaskManager query mime type, uid:{}, task_id:{}",
            uid, task_id
        );

        let task = self.get_task(uid, task_id);
        match task {
            Some(value) => {
                debug!("TaskManager query mime type by memory");
                value.query_mime_type()
            }
            None => {
                debug!("TaskManager query mime type: show mime type from database");
                let c_task_info = unsafe { Show(task_id, uid) };
                if c_task_info.is_null() {
                    info!("TaskManger query mime type: no task found in database");
                    return "".into();
                }
                let c_task_info = unsafe { &*c_task_info };
                let task_info = TaskInfo::from_c_struct(c_task_info);
                let mime_type = task_info.mime_type;
                debug!("TaskManager query mime type: mime type is {:?}", mime_type);
                unsafe { DeleteCTaskInfo(c_task_info) };
                mime_type
            }
        }
    }
}
