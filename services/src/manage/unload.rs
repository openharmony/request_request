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

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use samgr::manage::SystemAbilityManager;

use super::task_manager::GetTopBundleName;
use super::TaskManager;
use crate::error::ErrorCode;
use crate::task::config::{TaskConfig, Version};
use crate::task::ffi::{CTaskConfig, ChangeRequestTaskState};
use crate::task::info::{ApplicationState, State};
use crate::task::reason::Reason;
use crate::task::request_task::RequestTask;

impl TaskManager {
    pub(crate) fn keep_sa(&self) -> bool {
        if self.has_running_tasks() {
            info!(
                "Running tasks num is {} not 0, when trying to unload SA",
                self.tasks.len()
            );
            return true;
        }
        false
    }

    pub(crate) fn unload_sa(&mut self) -> bool {
        const REQUEST_SERVICE_ID: i32 = 3706;

        if !self.rx.is_empty() {
            return false;
        }

        if self.keep_sa() {
            return false;
        }

        self.delete_early_records();

        // check rx again for there may be new message arrive.
        if !self.rx.is_empty() {
            return false;
        }

        self.rx.close();

        info!("unload SA");

        // failed logic?
        let _ = SystemAbilityManager::unload_system_ability(REQUEST_SERVICE_ID);

        true
    }

    pub(crate) fn restore_all_tasks(&mut self) {
        if let Some(config_map) = self.query_all_task_config() {
            info!(
                "RSA query task config list len: {} in database",
                config_map.len()
            );
            for (_, config) in config_map.into_iter() {
                debug!("RSA query task config is {:?}", config);
                let uid = config.common_data.uid;
                let task_id = config.common_data.task_id;
                let token = config.token.clone();
                if let Some(task_info) = self.touch(uid, task_id, token) {
                    let state = State::from(task_info.progress.common_data.state);
                    if state != State::Waiting
                        && state != State::Paused
                        && state != State::Initialized
                    {
                        continue;
                    }
                    let app_state = self.app_state(uid, &config.bundle);
                    match RequestTask::new(config, self.system_config(), app_state, Some(task_info))
                    {
                        Ok(task) => self.restoring_tasks.push(Arc::new(task)),
                        Err(_) => {
                            unsafe { ChangeRequestTaskState(task_id, uid, State::Failed, Reason::OthersError) };
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn has_task_config_record(&self, task_id: u32) -> bool {
        unsafe { HasTaskConfigRecord(task_id) }
    }

    pub(crate) fn continue_task_from_database(&mut self, task_id: u32) -> ErrorCode {
        if let Some(config) = self.query_single_task_config(task_id) {
            debug!("RSA query single task config is {:?}", config);
            let uid = config.common_data.uid;
            let task_id = config.common_data.task_id;
            let token = config.token.clone();
            if let Some(task_info) = self.touch(uid, task_id, token) {
                let state = State::from(task_info.progress.common_data.state);
                debug!("get continue task state is {:?}", state);
                if state != State::Failed && state != State::Stopped && state != State::Initialized
                {
                    error!("state of continue task is not Failed\\Stopped\\Initialized");
                    return ErrorCode::TaskStateErr;
                }
                let app_state = self.app_state(uid, &config.bundle);
                match RequestTask::new(config, self.system_config(), app_state, Some(task_info)) {
                    Ok(task) => {
                        task.set_status(State::Waiting, Reason::Default);
                        unsafe { ChangeRequestTaskState(task_id, uid, State::Waiting, Reason::Default) };
                        let arc_task = Arc::new(task);
                        self.restoring_tasks.push(arc_task);
                        // Adds tasks to task map and inits it.
                        self.insert_restore_tasks();
                        return ErrorCode::ErrOk;
                    }
                    Err(_) => {
                        error!("continue task failed");
                        return ErrorCode::Other;
                    }
                }
            }
        }
        ErrorCode::TaskNotFound
    }

    pub(crate) fn insert_restore_tasks(&mut self) {
        debug!("TaskManager inserts restore tasks");
        let top_bundle = unsafe { GetTopBundleName() }.to_string();
        for task in std::mem::take(&mut self.restoring_tasks) {
            self.restore_task(task, &top_bundle);
        }
    }

    fn restore_task(&mut self, task: Arc<RequestTask>, top_bundle: &str) {
        self.restore_task_inner(task.clone());

        if task.conf.bundle == top_bundle {
            self.update_app_state(task.conf.common_data.uid, ApplicationState::Foreground);
        }

        if unsafe { NetworkManager::new().is_online() } {
            self.resume_waiting_task(task);
        }
    }

    fn restore_task_inner(&mut self, task: Arc<RequestTask>) {
        if task.conf.version == Version::API10 {
            self.api10_background_task_count += 1;
        }
        let uid = task.conf.common_data.uid;
        let task_id = task.conf.common_data.task_id;
        if self.get_task(uid, task_id).is_some() {
            return;
        }

        self.tasks.insert(task_id, task);

        match self.app_task_map.get_mut(&uid) {
            Some(set) => {
                set.insert(task_id);
            }
            None => {
                let mut set = HashSet::new();
                set.insert(task_id);
                self.app_task_map.insert(uid, set);
            }
        }
    }

    fn has_running_tasks(&self) -> bool {
        if self.tasks.is_empty() {
            return false;
        }
        for task in self.tasks.values() {
            let (state, reason) = {
                let task = task.status.lock().unwrap();
                (task.state, task.reason)
            };
            if !(state == State::Completed
                || state == State::Failed
                || state == State::Removed
                || state == State::Stopped
                || (state == State::Waiting
                    && (reason == Reason::NetworkOffline
                        || reason == Reason::UnsupportedNetworkType))
                || state == State::Paused
                || state == State::Initialized
                || state == State::Created)
            {
                return true;
            }
        }
        false
    }

    pub(crate) fn query_all_task_config(&self) -> Option<HashMap<u32, TaskConfig>> {
        debug!("query all task config in database");
        let mut task_config_map: HashMap<u32, TaskConfig> = HashMap::new();
        let mut len = 0u32;
        let c_task_config_list = unsafe { QueryAllTaskConfig(&mut len as *mut u32) };
        debug!("query all task config in database, len: {}", len);
        if c_task_config_list.is_null() || len == 0 {
            return None;
        }
        let c_task_config_ptrs =
            unsafe { std::slice::from_raw_parts(c_task_config_list, len as usize) };
        for c_task_config in c_task_config_ptrs.iter() {
            let task_config = TaskConfig::from_c_struct(unsafe { &**c_task_config });
            task_config_map.insert(task_config.common_data.task_id, task_config);
            unsafe { DeleteCTaskConfig(*c_task_config) };
        }
        unsafe { DeleteCTaskConfigs(c_task_config_list) };
        Some(task_config_map)
    }

    pub(crate) fn query_single_task_config(&self, task_id: u32) -> Option<TaskConfig> {
        debug!("query single task config in database");
        let c_task_config = unsafe { QuerySingleTaskConfig(task_id) };
        if c_task_config.is_null() {
            error!(
                "can not find the failed task in database, which task id is {}",
                task_id
            );
            None
        } else {
            let task_config = TaskConfig::from_c_struct(unsafe { &*c_task_config });
            unsafe { DeleteCTaskConfig(c_task_config) };
            Some(task_config)
        }
    }

    /// Removes task records from a week ago before unloading.
    pub(crate) fn delete_early_records(&self) {
        use std::time::{SystemTime, UNIX_EPOCH};

        const MILLIS_IN_A_WEEK: u64 = 7 * 24 * 60 * 60 * 1000;

        debug!("Starts to delete early records");

        if let Ok(time) = SystemTime::now().duration_since(UNIX_EPOCH) {
            unsafe {
                RequestDBRemoveRecordsFromTime(time.as_millis() as u64 - MILLIS_IN_A_WEEK);
            }
        }

        debug!("Deletes early records end");
    }
}

#[link(name = "download_server_cxx", kind = "static")]
extern "C" {
    pub(crate) fn HasTaskConfigRecord(task_id: u32) -> bool;
    pub(crate) fn DeleteCTaskConfigs(ptr: *const *const CTaskConfig);
    pub(crate) fn QueryAllTaskConfig(len: *mut u32) -> *const *const CTaskConfig;
    pub(crate) fn QuerySingleTaskConfig(taskId: u32) -> *const CTaskConfig;
    pub(crate) fn DeleteCTaskConfig(ptr: *const CTaskConfig);
    pub(crate) fn RequestDBRemoveRecordsFromTime(time: u64);
}
