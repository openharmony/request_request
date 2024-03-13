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

use std::collections::HashSet;
use std::fs::File;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::error::ErrorCode;
use crate::manage::{SystemProxyManager, TaskManager};
use crate::task::config::{TaskConfig, Version};
use crate::task::ffi::{CTaskConfig, CTaskInfo};
use crate::task::info::State;
use crate::task::reason::Reason;
use crate::task::RequestTask;

impl TaskManager {
    pub(crate) fn construct_task(
        &mut self,
        config: TaskConfig,
        files: Vec<File>,
        body_files: Vec<File>,
        proxy_task: SystemProxyManager,
    ) -> ErrorCode {
        if files.is_empty() {
            return ErrorCode::FileOperationErr;
        }

        let uid = config.common_data.uid;
        let task_id = config.common_data.task_id;
        let version = config.version;

        debug!(
            "TaskManager Construct, uid:{}, task_id:{}, version:{:?}",
            uid, task_id, version
        );

        let app_state = self.app_state(uid, &config.bundle);

        let task = Arc::new(RequestTask::constructor(
            config,
            files,
            body_files,
            AtomicBool::new(false),
            app_state,
            proxy_task,
        ));

        match version {
            Version::API10 => {
                if !self.add_task_api10(task.clone()) {
                    return ErrorCode::TaskEnqueueErr;
                }
                self.api10_background_task_count += 1;
            }
            Version::API9 => {
                self.add_task_api9(task.clone());
            }
        }

        self.record_request_task(task.as_ref());

        ErrorCode::ErrOk
    }

    pub(crate) fn add_task_api9(&mut self, task: Arc<RequestTask>) {
        task.set_status(State::Initialized, Reason::Default);

        let task_id = task.conf.common_data.task_id;
        let uid = task.conf.common_data.uid;

        self.tasks.insert(task_id, task);

        match self.app_task_map.get_mut(&uid) {
            Some(set) => {
                set.insert(task_id);

                debug!(
                    "TaskManager app {} task count:{}, all task count {}",
                    uid,
                    set.len(),
                    self.tasks.len()
                );
            }
            None => {
                let mut set = HashSet::new();
                set.insert(task_id);
                self.app_task_map.insert(uid, set);
                debug!(
                    "TaskManager app {} task count:{}, all task count {}",
                    uid,
                    1,
                    self.tasks.len()
                );
            }
        }
    }

    pub(crate) fn add_task_api10(&mut self, task: Arc<RequestTask>) -> bool {
        let task_id = task.conf.common_data.task_id;
        let uid = task.conf.common_data.uid;

        match self.app_task_map.get_mut(&uid) {
            Some(set) => {
                set.insert(task_id);

                task.set_status(State::Initialized, Reason::Default);
                self.tasks.insert(task_id, task);

                debug!(
                    "TaskManager app {} task count:{}, all task count {}",
                    uid,
                    set.len(),
                    self.tasks.len()
                );
                true
            }
            None => {
                let mut set = HashSet::new();
                set.insert(task_id);
                self.app_task_map.insert(uid, set);

                task.set_status(State::Initialized, Reason::Default);
                self.tasks.insert(task_id, task);

                debug!(
                    "TaskManager app {} task count:{}, all task count {}",
                    uid,
                    1,
                    self.tasks.len()
                );
                true
            }
        }
    }

    pub(crate) fn record_request_task(&mut self, task: &RequestTask) {
        debug!("record request task into database");

        if unsafe { HasRequestTaskRecord(task.conf.common_data.task_id) } {
            return;
        }
        let task_config = &task.conf;
        let config_set = task_config.build_config_set();
        let c_task_config = task_config.to_c_struct(
            task.conf.common_data.task_id,
            task.conf.common_data.uid,
            &config_set,
        );
        let task_info = &task.show();
        let info_set = task_info.build_info_set();
        let c_task_info = task_info.to_c_struct(&info_set);
        let ret = unsafe { RecordRequestTask(&c_task_info, &c_task_config) };
        info!("insert request_task DB ret is {}", ret);
    }
}

#[cfg(feature = "oh")]
#[link(name = "request_service_c")]
extern "C" {
    pub(crate) fn HasRequestTaskRecord(taskId: u32) -> bool;
    pub(crate) fn RecordRequestTask(
        taskInfo: *const CTaskInfo,
        taskConfig: *const CTaskConfig,
    ) -> bool;
}
