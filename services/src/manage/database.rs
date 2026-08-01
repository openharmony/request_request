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
use std::fmt::Display;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::sync::{Arc, Mutex, Once};

pub(crate) use ffi::*;

cfg_oh! {
    use crate::manage::SystemConfig;
}

cfg_not_oh! {
    use rusqlite::Connection;
    const CREATE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS request_task (task_id INTEGER PRIMARY KEY, uid INTEGER, token_id INTEGER, action INTEGER, mode INTEGER, cover INTEGER, network INTEGER, metered INTEGER, roaming INTEGER, ctime INTEGER, mtime INTEGER, reason INTEGER, gauge INTEGER, retry INTEGER, redirect INTEGER, tries INTEGER, version INTEGER, config_idx INTEGER, begins INTEGER, ends INTEGER, precise INTEGER, priority INTEGER, background INTEGER, bundle TEXT, url TEXT, data TEXT, token TEXT, title TEXT, description TEXT, method TEXT, headers TEXT, config_extras TEXT, mime_type TEXT, state INTEGER, idx INTEGER, total_processed INTEGER, sizes TEXT, processed TEXT, extras TEXT, form_items BLOB, file_specs BLOB, each_file_status BLOB, body_file_names BLOB, certs_paths BLOB)";
}
use crate::config::Action;
use crate::error::ErrorCode;
use crate::service::client::ClientManagerEntry;
use crate::task::config::TaskConfig;
use crate::task::ffi::{CTaskConfig, CTaskInfo, CUpdateInfo};
use crate::task::info::{State, TaskInfo, UpdateInfo};
use crate::task::reason::Reason;
use crate::task::request_task::RequestTask;
use crate::utils::{call_once, get_current_timestamp, hashmap_to_string};

pub(crate) struct RequestDb {
    user_file_tasks: Mutex<HashMap<u32, Arc<RequestTask>>>,
    #[cfg(feature = "oh")]
    pub(crate) inner: *mut RequestDataBase,
    #[cfg(not(feature = "oh"))]
    pub(crate) inner: Connection,
}

impl RequestDb {
    #[cfg(feature = "oh")]
    pub(crate) fn get_instance() -> &'static Self {
        static mut DB: MaybeUninit<RequestDb> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        call_once(&ONCE, || {
            let (path, encrypt) = if cfg!(test) {
                ("/data/test/request.db", false)
            } else {
                ("/data/service/el1/public/database/request/request.db", true)
            };

            let inner = GetDatabaseInstance(path, encrypt);
            unsafe {
                DB.write(RequestDb {
                    inner,
                    user_file_tasks: Mutex::new(HashMap::new()),
                });
            }
        });
        unsafe { DB.assume_init_mut() }
    }

    #[cfg(not(feature = "oh"))]
    pub(crate) fn get_instance() -> &'static Self {
        static mut DATABASE: MaybeUninit<RequestDb> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        call_once(&ONCE, || {
            let inner = Connection::open_in_memory().unwrap();
            inner.execute(&CREATE_TABLE, ()).unwrap();
            unsafe {
                DATABASE.write(RequestDb {
                    inner,
                    user_file_tasks: Mutex::new(HashMap::new()),
                })
            };
        });

        unsafe { DATABASE.assume_init_ref() }
    }

    #[cfg(feature = "oh")]
    pub(crate) fn execute(&self, sql: &str) -> Result<(), i32> {
        let ret = unsafe { Pin::new_unchecked(&mut *self.inner).ExecuteSql(sql) };
        if ret == 0 {
            Ok(())
        } else {
            error!("execute sql failed: {}", ret);
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_04,
                &format!("execute sql failed: {}", ret)
            );
            Err(ret)
        }
    }

    #[cfg(not(feature = "oh"))]
    pub(crate) fn execute(&self, sql: &str) -> Result<(), i32> {
        let res = self.inner.execute(sql, ());

        self.inner.execute(sql, ()).map(|_| ()).map_err(|e| {
            error!("execute sql failed: {}", e);
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_04,
                &format!("execute {} failed: {}", sql, ret)
            );
            e.sqlite_error_code().unwrap() as u32 as i32
        })
    }

    #[cfg(feature = "oh")]
    pub(crate) fn query_integer<T: TryFrom<i64> + Default>(&self, sql: &str) -> Vec<T>
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
                    sys_event!(
                        ExecFault,
                        DfxCode::RDB_FAULT_06,
                        &format!("query_integer failed, value: {}", e)
                    );
                    Default::default()
                })
            })
            .collect();

        if ret != 0 {
            error!("query integer err:{}", ret);
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_06,
                &format!("query integer err:{}", ret)
            );
        }
        v
    }

    #[cfg(not(feature = "oh"))]
    pub(crate) fn query_integer<T: TryFrom<i64> + Default>(&self, sql: &str) -> Vec<T>
    where
        T::Error: Display,
    {
        let mut stmt = self.inner.prepare(sql).unwrap();
        let rows = stmt.query_map([], |row| Ok(row.get(0).unwrap())).unwrap();
        let v: Vec<i64> = rows.into_iter().map(|a| a.unwrap()).collect();
        v.into_iter()
            .map(|a| a.try_into().unwrap_or_else(|_| Default::default()))
            .collect()
    }

    pub(crate) fn contains_task(&self, task_id: u32) -> bool {
        let sql = format!(
            "SELECT COUNT(*) FROM request_task WHERE task_id = {}",
            task_id
        );
        let v = self.query_integer::<u32>(&sql);
        if v.is_empty() {
            error!("contains_task check failed, empty result");
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_06,
                "contains_task check failed, empty result"
            );
            false
        } else {
            v[0] == 1
        }
    }

    pub(crate) fn query_task_token_id(&self, task_id: u32) -> Result<u64, i32> {
        let sql = format!(
            "SELECT token_id FROM request_task WHERE task_id = {}",
            task_id
        );
        let v = self.query_integer::<u64>(&sql);
        if v.is_empty() {
            error!("query_task_token_id failed, empty result");
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_06,
                "query_task_token_id failed, empty result"
            );
            Err(-1)
        } else {
            Ok(v[0])
        }
    }

    #[cfg(feature = "oh")]
    pub(crate) fn insert_task(&self, task: RequestTask) -> bool {
        let task_id = task.task_id();
        let uid = task.uid();

        debug!("Insert task to database, uid: {}, tid: {}", uid, task_id);

        if self.contains_task(task_id) {
            return false;
        }

        let task_config = task.config();
        let config_set = task_config.build_config_set();
        let c_task_config = task_config.to_c_struct(task_id, uid, &config_set);

        let task_info = &task.info();
        let info_set = task_info.build_info_set();
        let c_task_info = task_info.to_c_struct(&info_set);

        if !unsafe { RecordRequestTask(&c_task_info, &c_task_config) } {
            info!("task {} insert database fail", task_id);
        }

        // For some tasks contains user_file, we must save it to map first.
        if task.conf.contains_user_file() {
            self.user_file_tasks
                .lock()
                .unwrap()
                .insert(task.task_id(), Arc::new(task));
        };
        true
    }

    #[cfg(not(feature = "oh"))]
    pub(crate) fn insert_task(&self, task: RequestTask) -> bool {
        use crate::task::reason::Reason;
        use crate::utils::get_current_timestamp;

        let task_id = task.task_id();
        let uid = task.uid();
        info!("insert database, uid {} tid {}", uid, task_id);
        if self.contains_task(task_id) {
            return false;
        }

        let config = task.config();
        let sql = format!(
            "INSERT OR REPLACE INTO request_task (task_id, uid, token_id, action, mode, cover, network, metered, roaming, ctime, gauge, retry, redirect, version, config_idx, begins, ends, precise, priority, background, bundle, url, data, token, title, description, method, headers, config_extras, mtime, reason, tries, state)
            VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', {}, {}, {}, {})",
            config.common_data.task_id,
            config.common_data.uid,
            config.common_data.token_id,
            config.common_data.action.repr,
            config.common_data.mode.repr,
            config.common_data.cover,
            config.common_data.network_config as u8,
            config.common_data.metered as u8,
            config.common_data.roaming as u8,
            get_current_timestamp(),
            config.common_data.gauge,
            config.common_data.retry,
            config.common_data.redirect,
            config.version as u8,
            config.common_data.index,
            config.common_data.begins,
            config.common_data.ends,
            config.common_data.precise,
            config.common_data.priority,
            config.common_data.background as u8,
            config.bundle,
            config.url,
            config.data,
            config.token,
            config.title,
            config.description,
            config.method,
            hashmap_to_string(&config.headers),
            hashmap_to_string(&config.extras),
            get_current_timestamp(),
            Reason::Default.repr,
            0,
            State::Initialized.repr,
        );
        self.execute(&sql).unwrap();

        // For some tasks contains user_file, we must save it to map first.
        if task.conf.contains_user_file() {
            self.user_file_tasks
                .lock()
                .unwrap()
                .insert(task.task_id(), Arc::new(task));
        };
        true
    }

    pub(crate) fn remove_user_file_task(&self, task_id: u32) {
        let mut task_map = self.user_file_tasks.lock().unwrap();
        task_map.remove(&task_id);
        debug!("Remove completed user file task, task_id: {}", task_id);
    }

    #[cfg(feature = "oh")]
    pub(crate) fn update_task(&self, task_id: u32, update_info: UpdateInfo) {
        debug!("Update task in database, task_id: {}", task_id);
        if !self.contains_task(task_id) {
            return;
        }
        let sizes = format!("{:?}", update_info.progress.sizes);
        let processed = format!("{:?}", update_info.progress.processed);
        let extras = hashmap_to_string(&update_info.progress.extras);
        let c_update_info = update_info.to_c_struct(&sizes, &processed, &extras);
        let ret = unsafe { UpdateRequestTask(task_id, &c_update_info) };
        debug!("Update task in database, ret is {}", ret);
    }

    pub(crate) fn update_task_time(&self, task_id: u32, task_time: u64) {
        let ret = unsafe { UpdateRequestTaskTime(task_id, task_time) };
        debug!("Update task time in database, ret is {}", ret);
    }

    pub(crate) fn clear_invalid_records(&self) {
        let sql = format!(
            "UPDATE request_task SET state = {} WHERE state = {} AND reason = {}",
            State::Failed.repr,
            State::Waiting.repr,
            Reason::Default.repr,
        );
        let _ = self.execute(&sql);
    }

    pub(crate) fn query_task_uid(&self, task_id: u32) -> Option<u64> {
        let sql = format!("SELECT uid FROM request_task WHERE task_id = {}", task_id);
        self.query_integer(&sql).first().copied()
    }

    pub(crate) fn query_task_action(&self, task_id: u32) -> Option<Action> {
        let sql = format!(
            "SELECT action FROM request_task WHERE task_id = {}",
            task_id
        );
        self.query_integer(&sql).first().map(|action: &i32| Action {
            repr: *action as u8,
        })
    }

    #[cfg(not(feature = "oh"))]
    pub(crate) fn update_task(&self, task_id: u32, update_info: UpdateInfo) {
        if !self.contains_task(task_id) {
            return;
        }
        let sql = format!(
            "UPDATE request_task SET sizes = {:?}, processed = {:?}, extras = {} WHERE task_id = {}",
            update_info.progress.sizes, update_info.progress.processed, hashmap_to_string(&update_info.progress.extras),
            task_id,
        );
        self.execute(&sql).unwrap();
    }

    pub(crate) fn update_task_state(&self, task_id: u32, state: State, reason: Reason) {
        let sql = format!(
            "UPDATE request_task SET state = {}, mtime = {}, reason = {} WHERE task_id = {}",
            state.repr,
            get_current_timestamp(),
            reason.repr,
            task_id
        );
        let _ = self.execute(&sql);
    }

    pub(crate) fn update_task_max_speed(&self, task_id: u32, max_speed: i64) {
        let sql = format!(
            "UPDATE request_task SET max_speed = {} WHERE task_id = {}",
            max_speed, task_id
        );
        let _ = self.execute(&sql);
    }

    pub(crate) fn update_task_sizes(&self, task_id: u32, sizes: &Vec<i64>) {
        let sql = format!(
            "UPDATE request_task SET sizes = '{:?}' WHERE task_id = {}",
            sizes, task_id
        );
        let _ = self.execute(&sql);
    }

    #[cfg(feature = "oh")]
    pub(crate) fn get_task_info(&self, task_id: u32) -> Option<TaskInfo> {
        debug!("Get task info from database");
        let c_task_info = unsafe { GetTaskInfo(task_id) };
        if c_task_info.is_null() {
            info!("No task found in database");
            return None;
        }
        let c_task_info = unsafe { &*c_task_info };
        let task_info = TaskInfo::from_c_struct(c_task_info);
        unsafe { DeleteCTaskInfo(c_task_info) };
        Some(task_info)
    }

    pub(crate) fn query_task_total_processed(&self, task_id: u32) -> Option<i64> {
        let sql = format!(
            "SELECT total_processed FROM request_task WHERE task_id = {}",
            task_id
        );
        self.query_integer(&sql).first().copied()
    }

    pub(crate) fn query_task_state(&self, task_id: u32) -> Option<u8> {
        let sql = format!("SELECT state FROM request_task WHERE task_id = {}", task_id);
        self.query_integer(&sql)
            .first()
            .map(|state: &i32| *state as u8)
    }

    #[cfg(not(feature = "oh"))]
    pub(crate) fn get_task_info(&self, task_id: u32) -> Option<TaskInfo> {
        use crate::info::CommonTaskInfo;
        use crate::task::notify::Progress;

        let sql = format!("SELECT task_id, uid, action, mode, mtime, reason, gauge, retry, version, priority, ctime, tries, url, data, token, state, idx from request_task where task_id = {}", task_id);
        let mut stmt = self.inner.prepare(&sql).unwrap();
        let mut row = stmt
            .query_map([], |row| {
                Ok(TaskInfo {
                    common_data: CommonTaskInfo {
                        task_id: row.get(0).unwrap(),
                        uid: row.get(1).unwrap(),
                        action: row.get(2).unwrap(),
                        mode: row.get(3).unwrap(),
                        mtime: row.get(4).unwrap(),
                        reason: row.get(5).unwrap(),
                        gauge: row.get(6).unwrap(),
                        retry: row.get(7).unwrap(),
                        version: row.get(8).unwrap(),
                        priority: row.get(9).unwrap(),
                        ctime: row.get(10).unwrap(),
                        tries: row.get(11).unwrap(),
                    },
                    url: row.get(12).unwrap(),
                    data: row.get(13).unwrap(),
                    token: row.get(14).unwrap(),
                    bundle: "".to_string(),
                    title: "".to_string(),
                    description: "".to_string(),
                    mime_type: "".to_string(),
                    extras: HashMap::new(),
                    each_file_status: vec![],
                    form_items: vec![],
                    file_specs: vec![],
                    progress: Progress::new(vec![]),
                })
            })
            .unwrap();
        row.next().map(|info| info.unwrap())
    }

    #[cfg(feature = "oh")]
    pub(crate) fn get_task_config(&self, task_id: u32) -> Option<TaskConfig> {
        debug!("query single task config in database");
        let c_task_config = unsafe { QueryTaskConfig(task_id) };
        if c_task_config.is_null() {
            error!("can not find task in database, task id: {}", task_id);
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_06,
                &format!("can not find task in database, task id: {}", task_id)
            );
            None
        } else {
            let task_config = TaskConfig::from_c_struct(unsafe { &*c_task_config });
            unsafe { DeleteCTaskConfig(c_task_config) };
            Some(task_config)
        }
    }

    #[cfg(not(feature = "oh"))]
    pub(crate) fn get_task_config(&self, task_id: u32) -> Option<TaskConfig> {
        use crate::config::{Action, CommonTaskConfig, NetworkConfig};

        debug!("query single task config in database");
        let sql = format!("SELECT url, title, description, method, data, token, version from request_task where task_id = {}", task_id);
        let mut stmt = self.inner.prepare(&sql).unwrap();
        let mut row = stmt
            .query_map([], |row| {
                let version: u8 = row.get(6).unwrap();
                Ok(TaskConfig {
                    url: row.get(0).unwrap(),
                    title: row.get(1).unwrap(),
                    description: row.get(2).unwrap(),
                    method: row.get(3).unwrap(),
                    data: row.get(4).unwrap(),
                    token: row.get(5).unwrap(),
                    version: version.into(),
                    common_data: CommonTaskConfig {
                        task_id,
                        uid: 0,
                        token_id: 0,
                        action: Action::Download,
                        mode: Mode::BackGround,
                        cover: true,
                        network_config: NetworkConfig::Any,
                        metered: true,
                        roaming: true,
                        gauge: true,
                        retry: true,
                        redirect: true,
                        index: 0,
                        begins: 0,
                        ends: 0,
                        precise: true,
                        priority: 0,
                        background: true,
                        multipart: false,
                    },
                    headers: Default::default(),
                    extras: Default::default(),
                    form_items: Default::default(),
                    file_specs: Default::default(),
                    bundle: Default::default(),
                    bundle_type: 0,
                    body_file_paths: vec![],
                    certs_path: vec![],
                    proxy: Default::default(),
                    certificate_pins: Default::default(),
                    atomic_account: Default::default(),
                })
            })
            .unwrap();
        row.next().map(|config| config.unwrap())
    }

    #[cfg(feature = "oh")]
    pub(crate) fn get_task_qos_info(&self, task_id: u32) -> Option<TaskQosInfo> {
        #[cfg(feature = "oh")]
        {
            let mut info = TaskQosInfo {
                task_id,
                action: 0,
                mode: 0,
                state: 0,
                priority: 0,
            };
            let sql = format!(
                "SELECT action, mode, state, priority FROM request_task WHERE task_id = {}",
                task_id
            );
            let ret =
                unsafe { Pin::new_unchecked(&mut *self.inner).GetTaskQosInfo(&sql, &mut info) };
            if ret == 0 {
                Some(info)
            } else {
                None
            }
        }
    }

    #[cfg(not(feature = "oh"))]
    pub(crate) fn get_task_qos_info(&self, task_id: u32) -> Option<TaskQosInfo> {
        let sql = format!(
            "SELECT action, mode, state, priority FROM request_task WHERE task_id = {}",
            task_id,
        );
        let mut stmt = self.inner.prepare(&sql).unwrap();
        let mut rows = stmt
            .query_map([], |row| {
                Ok(TaskQosInfo {
                    task_id: task_id,
                    action: row.get::<_, u8>(0).unwrap().into(),
                    mode: row.get::<_, u8>(1).unwrap().into(),
                    state: row.get(2).unwrap(),
                    priority: row.get(3).unwrap(),
                })
            })
            .unwrap();
        rows.next().map(|info| info.unwrap())
    }

    pub(crate) fn get_app_task_qos_infos_inner(&self, sql: &str) -> Vec<TaskQosInfo> {
        #[cfg(feature = "oh")]
        {
            let mut v = vec![];
            let _ = unsafe { Pin::new_unchecked(&mut *self.inner).GetAppTaskQosInfos(sql, &mut v) };
            v
        }
        #[cfg(not(feature = "oh"))]
        {
            let mut stmt = self.inner.prepare(&sql).unwrap();
            let rows = stmt
                .query_map([], |row| {
                    Ok(TaskQosInfo {
                        task_id: row.get(0).unwrap(),
                        action: row.get::<_, u8>(1).unwrap().into(),
                        mode: row.get::<_, u8>(2).unwrap().into(),
                        state: row.get(3).unwrap(),
                        priority: row.get(4).unwrap(),
                    })
                })
                .unwrap();
            rows.into_iter().map(|info| info.unwrap()).collect()
        }
    }

    pub(crate) fn get_app_task_qos_infos(&self, uid: u64) -> Vec<TaskQosInfo> {
        let sql = format!(
            "SELECT task_id, action, mode, state, priority FROM request_task WHERE uid = {} AND ((state = {} AND reason = {}) OR state = {} OR state = {})",
            uid,
            State::Waiting.repr,
            Reason::RunningTaskMeetLimits.repr,
            State::Running.repr,
            State::Retrying.repr,
        );
        self.get_app_task_qos_infos_inner(&sql)
    }

    pub(crate) fn get_task(
        &self,
        task_id: u32,
        #[cfg(feature = "oh")] system: SystemConfig,
        client_manager: &ClientManagerEntry,
        upload_resume: bool,
    ) -> Result<Arc<RequestTask>, ErrorCode> {
        // If this task exists in `user_file_map`，get it from this map.
        if let Some(task) = self.user_file_tasks.lock().unwrap().get(&task_id) {
            return Ok(task.clone());
        }

        // 此处需要根据 task_id 从数据库构造指定的任务。
        let config = match self.get_task_config(task_id) {
            Some(config) => config,
            None => return Err(ErrorCode::TaskNotFound),
        };
        let task_id = config.common_data.task_id;

        let task_info = match self.get_task_info(task_id) {
            Some(info) => info,
            None => return Err(ErrorCode::TaskNotFound),
        };

        let state = State::from(task_info.progress.common_data.state);
        debug!("get_task {} state is {:?}", task_id, state);
        if state == State::Removed {
            error!("get_task state is Removed, {}", task_id);
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_06,
                &format!("get_task state is Removed, {}", task_id)
            );
            return Err(ErrorCode::TaskStateErr);
        }

        match RequestTask::new_by_info(
            config,
            #[cfg(feature = "oh")]
            system,
            task_info,
            client_manager.clone(),
            upload_resume,
        ) {
            Ok(task) => Ok(Arc::new(task)),
            Err(e) => {
                error!("new RequestTask failed {}, err: {:?}", task_id, e);
                sys_event!(
                    ExecFault,
                    DfxCode::RDB_FAULT_06,
                    &format!("new RequestTask failed {}, err: {:?}", task_id, e)
                );
                Err(e)
            }
        }
    }
}

unsafe impl Send for RequestDb {}
unsafe impl Sync for RequestDb {}

#[cfg(feature = "oh")]

extern "C" {
    fn DeleteCTaskConfig(ptr: *const CTaskConfig);
    fn DeleteCTaskInfo(ptr: *const CTaskInfo);
    fn GetTaskInfo(task_id: u32) -> *const CTaskInfo;
    fn QueryTaskConfig(task_id: u32) -> *const CTaskConfig;
    fn RecordRequestTask(info: *const CTaskInfo, config: *const CTaskConfig) -> bool;
    fn UpdateRequestTask(id: u32, info: *const CUpdateInfo) -> bool;
    fn UpdateRequestTaskTime(task_id: u32, taskTime: u64) -> bool;
}

#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    #[derive(Clone, Debug, Copy)]
    pub(crate) struct TaskQosInfo {
        pub(crate) task_id: u32,
        pub(crate) action: u8,
        pub(crate) mode: u8,
        pub(crate) state: u8,
        pub(crate) priority: u32,
    }

    unsafe extern "C++" {
        include!("c_request_database.h");
        type RequestDataBase;
        fn GetDatabaseInstance(path: &str, encrypt: bool) -> *mut RequestDataBase;
        fn ExecuteSql(self: Pin<&mut RequestDataBase>, sql: &str) -> i32;
        fn QueryInteger(self: Pin<&mut RequestDataBase>, sql: &str, v: &mut Vec<i64>) -> i32;
        fn GetAppTaskQosInfos(
            self: Pin<&mut RequestDataBase>,
            sql: &str,
            v: &mut Vec<TaskQosInfo>,
        ) -> i32;
        fn GetTaskQosInfo(self: Pin<&mut RequestDataBase>, sql: &str, res: &mut TaskQosInfo)
            -> i32;
    }
}

#[cfg(feature = "oh")]
#[cfg(test)]
mod ut_database {
    include!("../../tests/ut/manage/ut_database.rs");
}
