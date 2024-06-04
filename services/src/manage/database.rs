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

use std::collections::HashMap;
use std::ffi::c_void;
use std::ptr::null_mut;
use std::slice;
use std::sync::{Arc, Once};

use super::app_state::AppStateManagerTx;
use crate::manage::SystemConfig;
use crate::service::client::ClientManagerEntry;
use crate::task::config::TaskConfig;
use crate::task::ffi::{CTaskConfig, CTaskInfo, CUpdateInfo, NetworkInfo};
use crate::task::info::{ApplicationState, Mode, State, TaskInfo};
use crate::task::reason::Reason;
use crate::task::request_task::RequestTask;
use crate::utils::c_wrapper::{CFilter, CStringWrapper, CVectorWrapper};
use crate::utils::filter::Filter;
use crate::utils::hashmap_to_string;

#[derive(Clone)]
pub(crate) struct Database {
    user_file_tasks: HashMap<u32, Arc<RequestTask>>,
}

impl Database {
    pub(crate) fn get_instance() -> &'static mut Self {
        static mut DATABASE: Option<Database> = None;
        static ONCE: Once = Once::new();

        ONCE.call_once(|| unsafe {
            DATABASE = Some(Database {
                user_file_tasks: HashMap::new(),
            });
        });

        unsafe { DATABASE.as_mut().unwrap() }
    }

    pub(crate) fn contains_task(&self, task_id: u32) -> bool {
        unsafe { HasRequestTaskRecord(task_id) }
    }

    pub(crate) fn insert_task(&mut self, task: RequestTask) {
        let task_id = task.task_id();
        let uid = task.uid();

        debug!(
            "Insert task to database, uid: {}, task_id: {}",
            uid, task_id
        );

        if self.contains_task(task_id) {
            return;
        }

        let task_config = task.config();
        let config_set = task_config.build_config_set();
        let c_task_config = task_config.to_c_struct(task_id, uid, &config_set);

        let task_info = &task.info();
        let info_set = task_info.build_info_set();
        let c_task_info = task_info.to_c_struct(&info_set);

        let ret = unsafe { RecordRequestTask(&c_task_info, &c_task_config) };

        // For some tasks contains user_file, we must save it to map first.
        if task.conf.contains_user_file() {
            self.user_file_tasks.insert(task.task_id(), Arc::new(task));
        }

        info!("Insert task to database, ret is {}", ret);
    }

    pub(crate) fn update_task(&self, task: &RequestTask) {
        let task_id = task.task_id();
        let uid = task.uid();

        debug!(
            "Update task in database, uid: {}, task_id: {}",
            uid, task_id
        );

        if !self.contains_task(task_id) {
            return;
        }

        let update_info = task.update_info();
        let sizes = format!("{:?}", update_info.progress.sizes);
        let processed = format!("{:?}", update_info.progress.processed);
        let extras = hashmap_to_string(&update_info.progress.extras);
        let each_file_status = update_info
            .each_file_status
            .iter()
            .map(|x| x.to_c_struct())
            .collect();
        let c_update_info = update_info.to_c_struct(&sizes, &processed, &extras, &each_file_status);
        let ret = unsafe { UpdateRequestTask(task_id, &c_update_info) };
        debug!("Update task in database, ret is {}", ret);
    }

    pub(crate) fn change_task_state(&self, task_id: u32, uid: u64, state: State, reason: Reason) {
        unsafe { ChangeRequestTaskState(task_id, uid, state, reason) };
    }

    pub(crate) fn get_task_info(&self, task_id: u32) -> Option<TaskInfo> {
        debug!("Get task info from database");
        let c_task_info = unsafe { GetTaskInfo(task_id) };
        if c_task_info.is_null() {
            info!("No task found in database");
            return None;
        }
        let c_task_info = unsafe { &*c_task_info };
        let task_info = TaskInfo::from_c_struct(c_task_info);
        debug!("Task info is {:?}", task_info);
        unsafe { DeleteCTaskInfo(c_task_info) };
        Some(task_info)
    }

    pub(crate) fn search_tasks(&self, filter: Filter) -> Vec<u32> {
        debug!("Search tasks, filter: {:?}", filter);

        let wrapper = unsafe { Search(filter.to_c_struct()) };
        if wrapper.ptr.is_null() || wrapper.len == 0 {
            error!("c_vector_wrapper is null");
            return Vec::new();
        }
        let slice = unsafe { std::slice::from_raw_parts(wrapper.ptr, wrapper.len as usize) };
        let vec = slice.to_vec();
        debug!("c_vector_wrapper is not null");
        unsafe { DeleteCVectorWrapper(wrapper.ptr) };
        vec
    }

    pub(crate) fn app_uncompleted_tasks_num(&self, uid: u64, mode: Mode) -> usize {
        let result = unsafe { QueryAppUncompletedTasksNum(uid, mode as u8) as usize };
        debug!(
            "App uid {} uncompleted tasks in mode {:?} number is {}",
            uid, mode, result
        );
        result
    }

    #[allow(dead_code)]
    pub(crate) fn get_all_uncompleted_task_config(&self) -> Option<HashMap<u32, TaskConfig>> {
        debug!("query all task config in database");
        let c_config_list_len = unsafe { QueryTaskConfigLen() };
        if c_config_list_len <= 0 {
            debug!("no task config in database");
            return None;
        }
        let c_task_config_list = unsafe { QueryAllTaskConfigs() };
        if c_task_config_list.is_null() {
            debug!("no task config in database");
            return None;
        }
        let c_task_config_ptrs =
            unsafe { slice::from_raw_parts(c_task_config_list, c_config_list_len as usize) };

        let mut task_config_map = HashMap::new();
        for c_task_config in c_task_config_ptrs.iter() {
            let task_config = TaskConfig::from_c_struct(unsafe { &**c_task_config });
            task_config_map.insert(task_config.common_data.task_id, task_config);
            unsafe { DeleteCTaskConfig(*c_task_config) };
        }
        unsafe { DeleteCTaskConfigs(c_task_config_list) };
        Some(task_config_map)
    }

    pub(crate) fn get_task_config(&self, task_id: u32) -> Option<TaskConfig> {
        debug!("query single task config in database");
        let c_task_config = unsafe { QueryTaskConfig(task_id) };
        if c_task_config.is_null() {
            error!("can not find task in database, task id: {}", task_id);
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

    pub(crate) fn get_token_id(&self, task_id: u32) -> Option<u64> {
        let mut token_id = 0;
        unsafe { QueryTaskTokenId(task_id, &mut token_id as *mut u64) }.then_some(token_id)
    }

    pub(crate) fn update_on_app_state_change(&self, uid: u64, state: ApplicationState) {
        unsafe { UpdateTaskStateOnAppStateChange(uid, state as u8) };
    }

    pub(crate) fn update_on_network_change(&self, network: NetworkInfo) {
        unsafe { UpdateTaskStateOnNetworkChange(network) };
    }

    pub(crate) fn get_task_qos_info(&self, uid: u64, task_id: u32) -> Option<TaskQosInfo> {
        let mut info: *mut TaskQosInfo = null_mut::<TaskQosInfo>();
        unsafe { GetTaskQosInfo(uid, task_id, &mut info as *mut *mut TaskQosInfo) };
        if info.is_null() {
            return None;
        }

        let res = unsafe {
            TaskQosInfo {
                task_id: (*info).task_id,
                action: (*info).action,
                mode: (*info).mode,
                state: (*info).state,
                priority: (*info).priority,
            }
        };
        unsafe { DeleteTaskQosInfo(info) };
        Some(res)
    }

    pub(crate) fn get_app_task_qos_infos(&self, uid: u64) -> Vec<TaskQosInfo> {
        let mut array = null_mut::<TaskQosInfo>();
        let mut len = 0;
        unsafe {
            GetAppTaskQosInfos(
                uid,
                &mut array as *mut *mut TaskQosInfo,
                &mut len as *mut usize,
            )
        };

        if array.is_null() {
            return Vec::new();
        }

        let res = unsafe { slice::from_raw_parts(array as *const TaskQosInfo, len) }.to_vec();
        unsafe { DeleteTaskQosInfo(array) };
        res
    }

    pub(crate) fn get_app_infos(&self) -> Vec<(u64, String)> {
        let mut array = null_mut::<AppInfo>();
        let mut len = 0;
        unsafe { GetAppArray(&mut array as *mut *mut AppInfo, &mut len as *mut usize) };

        let mut vec = Vec::new();
        if array.is_null() {
            return vec;
        }

        for info in unsafe { slice::from_raw_parts(array as *const AppInfo, len) } {
            vec.push((info.uid, info.bundle.to_string()));
        }
        unsafe { DeleteAppInfo(array) };
        vec
    }

    pub(crate) fn get_app_bundle(&self, uid: u64) -> Option<String> {
        let str = unsafe { GetAppBundle(uid) }.to_string();
        if str.is_empty() {
            None
        } else {
            Some(str)
        }
    }

    pub(crate) async fn get_task(
        &self,
        task_id: u32,
        system: SystemConfig,
        app_state_manager: &AppStateManagerTx,
        client_manager: &ClientManagerEntry,
    ) -> Option<Arc<RequestTask>> {
        // If this task exists in `user_file_map`，get it from this map.
        if let Some(task) = self.user_file_tasks.get(&task_id) {
            return Some(task.clone());
        }

        // 此处需要根据 task_id 从数据库构造指定的任务。
        if let Some(config) = self.get_task_config(task_id) {
            let uid = config.common_data.uid;
            let task_id = config.common_data.task_id;
            if let Some(task_info) = self.get_task_info(task_id) {
                let state = State::from(task_info.progress.common_data.state);
                debug!("get_task {} state is {:?}", task_id, state);
                if state == State::Removed {
                    error!("get_task state is Removed, {}", task_id);
                    return None;
                }

                let app_state = app_state_manager.get_app_state(uid).await;
                match RequestTask::new(
                    config,
                    system,
                    app_state,
                    Some(task_info),
                    client_manager.clone(),
                ) {
                    Ok(task) => {
                        return Some(Arc::new(task));
                    }
                    Err(_) => {
                        error!("new RequestTask failed");
                        return None;
                    }
                }
            }
        }
        None
    }
}

#[derive(Clone, Debug, Copy)]
#[repr(C)]
pub(crate) struct TaskQosInfo {
    pub(crate) task_id: u32,
    pub(crate) action: u8,
    pub(crate) mode: u8,
    pub(crate) state: u8,
    pub(crate) priority: u32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub(crate) struct AppInfo {
    pub(crate) uid: u64,
    pub(crate) bundle: CStringWrapper,
}

#[link(name = "download_server_cxx", kind = "static")]
extern "C" {
    fn ChangeRequestTaskState(task_id: u32, uid: u64, state: State, reason: Reason) -> bool;
    fn DeleteCTaskConfig(ptr: *const CTaskConfig);
    fn DeleteCTaskConfigs(ptr: *const *const CTaskConfig);
    fn DeleteCTaskInfo(ptr: *const CTaskInfo);
    fn DeleteCVectorWrapper(ptr: *const u32);
    fn GetTaskInfo(task_id: u32) -> *const CTaskInfo;
    fn HasRequestTaskRecord(id: u32) -> bool;
    fn QueryAllTaskConfigs() -> *const *const CTaskConfig;
    fn QueryAppUncompletedTasksNum(uid: u64, mode: u8) -> u32;
    fn QueryTaskConfig(task_id: u32) -> *const CTaskConfig;
    fn QueryTaskConfigLen() -> i32;
    fn QueryTaskTokenId(task_id: u32, token_id: *mut u64) -> bool;
    fn RecordRequestTask(info: *const CTaskInfo, config: *const CTaskConfig) -> bool;
    fn RequestDBRemoveRecordsFromTime(time: u64);
    fn Search(filter: CFilter) -> CVectorWrapper;
    fn UpdateRequestTask(id: u32, info: *const CUpdateInfo) -> bool;
    fn UpdateTaskStateOnAppStateChange(uid: u64, app_state: u8) -> c_void;
    fn UpdateTaskStateOnNetworkChange(info: NetworkInfo) -> c_void;
    fn GetTaskQosInfo(uid: u64, task_id: u32, info: *mut *mut TaskQosInfo) -> c_void;
    fn GetAppTaskQosInfos(uid: u64, array: *mut *mut TaskQosInfo, len: *mut usize) -> c_void;
    fn GetAppArray(apps: *mut *mut AppInfo, len: *mut usize) -> c_void;
    fn DeleteTaskQosInfo(ptr: *const TaskQosInfo) -> c_void;
    fn DeleteAppInfo(ptr: *const AppInfo) -> c_void;
    fn GetAppBundle(uid: u64) -> CStringWrapper;
}
