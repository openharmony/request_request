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
use std::fmt::Display;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr::null_mut;
use std::slice;
use std::sync::{Arc, Mutex, Once};

pub(crate) use ffi::*;

use super::app_state::AppStateManagerTx;
use crate::error::ErrorCode;
use crate::manage::{Network, SystemConfig};
use crate::service::client::ClientManagerEntry;
use crate::task::config::{Mode, TaskConfig};
use crate::task::ffi::{CTaskConfig, CTaskInfo, CUpdateInfo, CUpdateStateInfo};
use crate::task::info::{ApplicationState, State, TaskInfo};
use crate::task::request_task::RequestTask;
use crate::utils::hashmap_to_string;

pub(crate) struct Database {
    user_file_tasks: Mutex<HashMap<u32, Arc<RequestTask>>>,
}

impl Database {
    pub(crate) fn get_instance() -> &'static Self {
        static mut DATABASE: MaybeUninit<Database> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        ONCE.call_once(|| unsafe {
            DATABASE.write(Database {
                user_file_tasks: Mutex::new(HashMap::new()),
            });
        });

        unsafe { DATABASE.assume_init_ref() }
    }

    pub(crate) fn contains_task(&self, task_id: u32) -> bool {
        unsafe { HasRequestTaskRecord(task_id) }
    }

    pub(crate) fn insert_task(&self, task: RequestTask) {
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

        if !unsafe { RecordRequestTask(&c_task_info, &c_task_config) } {
            info!("Insert task {} to database failed", task_id);
        }

        // For some tasks contains user_file, we must save it to map first.
        if task.conf.contains_user_file() {
            self.user_file_tasks
                .lock()
                .unwrap()
                .insert(task.task_id(), Arc::new(task));
        }
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

    pub(crate) fn update_task_state(&self, task_id: u32, info: &CUpdateStateInfo) -> bool {
        if !self.contains_task(task_id) {
            return false;
        }
        let ret: bool = unsafe { UpdateRequestTaskState(task_id, info) };
        debug!("Update task state in database, ret is {}", ret);
        ret
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

    pub(crate) fn app_uncompleted_tasks_num(&self, uid: u64, mode: Mode) -> usize {
        let result = unsafe { QueryAppUncompletedTasksNum(uid, mode.repr) as usize };
        debug!(
            "App uid {} uncompleted tasks in mode {:?} number is {}",
            uid, mode, result
        );
        result
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

    pub(crate) fn get_task_qos_info(&self, uid: u64, task_id: u32) -> Option<TaskQosInfo> {
        let mut info: *mut TaskQosInfo = null_mut::<TaskQosInfo>();
        unsafe { GetTaskQosInfo(uid, task_id, &mut info as *mut *mut TaskQosInfo) };
        if info.is_null() {
            return None;
        }

        let res = unsafe {
            TaskQosInfo {
                uid: (*info).uid,
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

    pub(crate) async fn get_task(
        &self,
        task_id: u32,
        system: SystemConfig,
        network: Network,
        app_state_manager: &AppStateManagerTx,
        client_manager: &ClientManagerEntry,
    ) -> Result<Arc<RequestTask>, ErrorCode> {
        // If this task exists in `user_file_map`，get it from this map.
        if let Some(task) = self.user_file_tasks.lock().unwrap().get(&task_id) {
            return Ok(task.clone());
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
                    return Err(ErrorCode::TaskStateErr);
                }

                let app_state = app_state_manager.get_app_state(uid).await;
                match RequestTask::new(
                    config,
                    system,
                    app_state,
                    Some(task_info),
                    client_manager.clone(),
                    network.clone(),
                ) {
                    Ok(task) => {
                        return Ok(Arc::new(task));
                    }
                    Err(e) => {
                        error!("new RequestTask failed {}, err: {:?}", task_id, e);
                        return Err(e);
                    }
                }
            }
        }
        Err(ErrorCode::TaskNotFound)
    }
}

#[derive(Clone, Debug, Copy)]
#[repr(C)]
pub(crate) struct TaskQosInfo {
    pub(crate) uid: u64,
    pub(crate) task_id: u32,
    pub(crate) action: u8,
    pub(crate) mode: u8,
    pub(crate) state: u8,
    pub(crate) priority: u32,
}

#[link(name = "download_server_cxx", kind = "static")]
extern "C" {
    fn DeleteCTaskConfig(ptr: *const CTaskConfig);
    fn DeleteCTaskInfo(ptr: *const CTaskInfo);
    fn GetTaskInfo(task_id: u32) -> *const CTaskInfo;
    fn HasRequestTaskRecord(id: u32) -> bool;
    fn QueryAppUncompletedTasksNum(uid: u64, mode: u8) -> u32;
    fn QueryTaskConfig(task_id: u32) -> *const CTaskConfig;
    fn QueryTaskTokenId(task_id: u32, token_id: *mut u64) -> bool;
    fn RecordRequestTask(info: *const CTaskInfo, config: *const CTaskConfig) -> bool;
    fn RequestDBRemoveRecordsFromTime(time: u64);
    fn UpdateRequestTask(id: u32, info: *const CUpdateInfo) -> bool;
    fn UpdateRequestTaskState(id: u32, info: *const CUpdateStateInfo) -> bool;
    fn UpdateTaskStateOnAppStateChange(uid: u64, app_state: u8) -> c_void;
    fn GetTaskQosInfo(uid: u64, task_id: u32, info: *mut *mut TaskQosInfo) -> c_void;
    fn GetAppTaskQosInfos(uid: u64, array: *mut *mut TaskQosInfo, len: *mut usize) -> c_void;
    fn DeleteTaskQosInfo(ptr: *const TaskQosInfo) -> c_void;
}

pub(crate) struct RequestDb {
    pub(crate) inner: *mut RequestDataBase,
}

impl RequestDb {
    pub(crate) fn get_instance() -> Self {
        let path = if cfg!(test) {
            "/data/test/request.db"
        } else {
            "/data/service/el1/public/database/request/request.db"
        };

        let inner = GetDatabaseInstance(path);
        Self { inner }
    }

    pub(crate) fn execute_sql(&mut self, sql: &str) -> Result<(), i32> {
        let ret = unsafe { Pin::new_unchecked(&mut *self.inner).ExecuteSql(sql) };
        if ret == 0 {
            Ok(())
        } else {
            Err(ret)
        }
    }

    pub(crate) fn query_integer<T: TryFrom<i64> + Default>(
        &mut self,
        sql: &str,
    ) -> Result<Vec<T>, (Vec<T>, i32)>
    where
        T::Error: Display,
    {
        let mut v = vec![];
        let ret = unsafe { Pin::new_unchecked(&mut *self.inner).QueryInteger(sql, &mut v) };
        let v = v
            .into_iter()
            .map(|a| {
                a.try_into().unwrap_or_else(|e| {
                    error!("query_integer failed, value: {}", e);
                    Default::default()
                })
            })
            .collect();

        if ret == 0 {
            Ok(v)
        } else {
            Err((v, ret))
        }
    }

    #[allow(unused)]
    pub(crate) fn query_text(&mut self, sql: &str) -> Result<Vec<String>, i32> {
        let mut v = vec![];
        let ret = unsafe { Pin::new_unchecked(&mut *self.inner).QueryText(sql, &mut v) };
        if ret == 0 {
            Ok(v)
        } else {
            Err(ret)
        }
    }
}

#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    unsafe extern "C++" {
        include!("c_request_database.h");
        type RequestDataBase;
        fn GetDatabaseInstance(path: &str) -> *mut RequestDataBase;
        fn ExecuteSql(self: Pin<&mut RequestDataBase>, sql: &str) -> i32;
        fn QueryInteger(self: Pin<&mut RequestDataBase>, sql: &str, v: &mut Vec<i64>) -> i32;
        fn QueryText(self: Pin<&mut RequestDataBase>, sql: &str, v: &mut Vec<String>) -> i32;
    }
}

#[cfg(test)]
mod test {
    use super::RequestDb;
    use crate::tests::test_init;
    use crate::utils::task_id_generator::TaskIdGenerator;

    #[test]
    fn ut_database_base() {
        test_init();
        let task_id = TaskIdGenerator::generate();
        let mut db = RequestDb::get_instance();
        db.execute_sql(&format!(
            "INSERT INTO request_task (task_id, bundle) VALUES ({}, 'example_bundle')",
            task_id
        ))
        .unwrap();

        let tasks = db
            .query_integer("SELECT task_id FROM request_task WHERE bundle = 'example_bundle'")
            .unwrap();
        assert!(tasks.contains(&task_id));
    }
}
