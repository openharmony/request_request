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
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

use super::TaskManager;
use crate::manager::monitor::IsOnline;
use crate::task::config::{TaskConfig, Version};
use crate::task::ffi::CTaskConfig;
use crate::task::info::State;
use crate::task::RequestTask;

impl TaskManager {
    pub(crate) fn check_unload_sa(&self) -> bool {
        if !self.rx.is_empty() {
            return false;
        }

        if !self.tasks.is_empty() && self.network_check_unload_sa() {
            return false;
        }

        if self.recording_rdb_num.load(Ordering::SeqCst) != 0 {
            return false;
        }

        if !self.rx.is_empty() {
            return false;
        }
        true
    }

    pub(crate) fn unload_sa(&mut self) -> bool {
        #[cfg(feature = "oh")]
        const REQUEST_SERVICE_ID: i32 = 3706;

        if !self.check_unload_sa() {
            debug!("Triggers unload sa, but cannot unload now");
            return false;
        }

        self.rx.close();

        info!("unload SA");

        if !self.tasks.is_empty() {
            self.record_all_task_config();
        }

        #[cfg(feature = "oh")]
        let samgr_proxy = rust_samgr::get_systemability_manager();

        // failed logic?
        #[cfg(feature = "oh")]
        let _ = samgr_proxy
            .unload_systemability(REQUEST_SERVICE_ID)
            .map_err(|e| error!("unload SA failed, err is {:?}", e));

        true
    }

    pub(crate) fn restore_all_tasks(&mut self, recording_rdb_num: Arc<AtomicU32>) {
        if self.restoring {
            return;
        }

        self.restoring = true;

        if let Some(config_list) = self.query_all_task_config() {
            info!(
                "RSA query task config list len: {} in database",
                config_list.len()
            );
            for config in config_list.into_iter() {
                debug!("RSA query task config is {:?}", config);
                let uid = config.common_data.uid;
                let task_id = config.common_data.task_id;
                let token = config.token.clone();
                if let Some(task_info) = self.touch(uid, task_id, token) {
                    let state = State::from(task_info.progress.common_data.state);
                    if state != State::Waiting && state != State::Paused {
                        continue;
                    }
                    let app_state = self.app_state(uid, &config.bundle);
                    let request_task = RequestTask::restore_task(
                        config,
                        task_info,
                        recording_rdb_num.clone(),
                        AtomicBool::new(false),
                        app_state,
                    );
                    let task = Arc::new(request_task);
                    self.restore_task(task.clone());
                    if unsafe { IsOnline() } {
                        self.resume_waiting_task(task.clone());
                    }
                }
                unsafe { CleanTaskConfigTable(task_id, uid) };
            }
        } else {
            self.schedule_unload_sa();
        }
        self.restoring = false;
    }

    fn record_all_task_config(&mut self) {
        debug!("record all task config into database");
        self.recording_rdb_num.fetch_add(1, Ordering::SeqCst);
        for task in self.tasks.values() {
            if unsafe { HasTaskConfigRecord(task.conf.common_data.task_id) } {
                continue;
            }
            let state = task.status.lock().unwrap().state;

            if state != State::Waiting && state != State::Paused {
                continue;
            }

            let task_config = &task.conf;

            let c_task_config =
                task_config.to_c_struct(task.conf.common_data.task_id, task.conf.common_data.uid);
            let ret = unsafe { RecordRequestTaskConfig(&c_task_config) };
            info!("insert taskConfig DB ret is {}", ret);
        }
        self.recording_rdb_num.fetch_sub(1, Ordering::SeqCst);
    }

    fn restore_task(&mut self, task: Arc<RequestTask>) {
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

    fn network_check_unload_sa(&self) -> bool {
        let mut need_unload = false;
        for task in self.tasks.values() {
            let state = task.status.lock().unwrap().state;
            if state == State::Completed
                || state == State::Failed
                || state == State::Removed
                || state == State::Stopped
                || ((state == State::Waiting || state == State::Paused)
                    && (!task.is_satisfied_configuration() || unsafe { !IsOnline() }))
            {
                need_unload = true;
            } else {
                return false;
            }
        }
        need_unload
    }

    pub(crate) fn query_all_task_config(&self) -> Option<Vec<TaskConfig>> {
        debug!("query all task config in database");
        let mut task_config_list: Vec<TaskConfig> = Vec::new();
        let c_config_list_len = unsafe { QueryTaskConfigLen() };
        if c_config_list_len <= 0 {
            debug!("no task config in database");
            return None;
        }
        let c_task_config_list = unsafe { QueryAllTaskConfig() };
        if c_task_config_list.is_null() {
            return None;
        }
        let c_task_config_ptrs =
            unsafe { std::slice::from_raw_parts(c_task_config_list, c_config_list_len as usize) };
        for c_task_config in c_task_config_ptrs.iter() {
            let task_config = TaskConfig::from_c_struct(unsafe { &**c_task_config });
            task_config_list.push(task_config);
            unsafe { DeleteCTaskConfig(*c_task_config) };
        }
        unsafe { DeleteCTaskConfigs(c_task_config_list) };
        Some(task_config_list)
    }
}

extern "C" {
    pub(crate) fn DeleteCTaskConfigs(ptr: *const *const CTaskConfig);
    pub(crate) fn QueryAllTaskConfig() -> *const *const CTaskConfig;
    pub(crate) fn QueryTaskConfigLen() -> i32;

    pub(crate) fn DeleteCTaskConfig(ptr: *const CTaskConfig);
    pub(crate) fn RecordRequestTaskConfig(taskConfig: *const CTaskConfig) -> bool;
    pub(crate) fn CleanTaskConfigTable(taskId: u32, uid: u64) -> bool;
    pub(crate) fn HasTaskConfigRecord(taskId: u32) -> bool;
}
