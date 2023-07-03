/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
//! ipc and task manger for service ability
#![allow(unused_variables, clippy::vec_init_then_push)]
extern crate ipc_rust;
extern crate system_ability_fwk_rust;
use super::{
    enumration::*, log::LOG_LABEL, progress::*, request_binding, task_config::*,
    task_info::*, task_manager::*, download_server_ipc_interface_code::*, filter::*,
};
use hilog_rust::*;
use ipc_rust::{
    get_calling_token_id, get_calling_uid, FileDesc, IRemoteBroker, IRemoteObj, InterfaceToken,
    IpcResult, IpcStatusCode, MsgParcel, RemoteObj, BorrowedMsgParcel, get_self_token_id,
};
use std::ffi::{c_char, CString};
use std::{
    collections::HashMap,
    fmt::Debug,
    fs::File,
    mem::MaybeUninit,
    result::Result,
    string::String,
    sync::{Arc, Mutex, Once},
    thread,
};

#[derive(PartialEq, Debug)]
pub enum ServerRunState {
    NoStart,
    Running,
}

pub struct RequestAbility {
    pub server_state: ServerRunState,
    pub reg_remote_obj: Mutex<HashMap<String, RemoteObj>>,
    unregistered_notify: Mutex<HashMap<String, u32>>,
}

impl RequestAbility {
    fn new(state: ServerRunState, obj: Mutex<HashMap<String, RemoteObj>>) -> Self {
        RequestAbility {
            server_state: state,
            reg_remote_obj: obj,
            unregistered_notify: Mutex::new(HashMap::new()),
        }
    }

    pub fn init(&mut self) -> i32 {
        debug!(LOG_LABEL, "init");
        TaskManager::get_instance().register_callback(Box::new(RequestAbility::notify_client),
                                                      Box::new(RequestAbility::notify_task_info));
        monitor_network();
        monitor_app_state();
        monitor_task();
        TaskManager::get_instance().rt.spawn(unload_sa());
        0
    }

    pub fn start(&mut self) {
        debug!(LOG_LABEL, "start");
        if self.server_state == ServerRunState::Running {
            info!(LOG_LABEL, "DownloadServiceAbility is already running");
            return;
        }
        unsafe {
            request_binding::InitServiceHandler();
        }
        let ret = self.init();
        if ret != 0 {
            unsafe {
                extern "C" fn ability_init() {
                    RequestAbility::get_ability_instance().init();
                }
                request_binding::PostTask(ability_init);
            }
        }
        self.server_state = ServerRunState::Running;
    }

    pub fn stop(&mut self) {
        debug!(LOG_LABEL, "stop");
        if ServerRunState::NoStart == self.server_state {
            return;
        }
        self.server_state = ServerRunState::NoStart;
    }

    pub fn construct(&self, config: TaskConfig, files: Vec<File>, task_id: &mut u32) -> ErrorCode {
        debug!(LOG_LABEL, "construct");
        let uid = get_calling_uid();
        let version = config.version.clone();
        let error = TaskManager::get_instance().construct_task(
            Arc::new(config),
            get_calling_uid(),
            task_id,
            files,
        );
        if version != Version::API10 {
            TaskManager::get_instance().start(get_calling_uid(), *task_id);
        }
        error
    }

    pub fn pause(&self, task_id: u32) -> ErrorCode {
        debug!(LOG_LABEL, "pause");
        TaskManager::get_instance().pause(get_calling_uid(), task_id)
    }

    pub fn query_mime_type(&self, task_id: u32, mime: &mut String) -> ErrorCode {
        *mime = TaskManager::get_instance().query_mime_type(get_calling_uid(), task_id);
        if mime.is_empty() {
            return ErrorCode::MimeType_not_found;
        }
        ErrorCode::ErrOk
    }

    pub fn remove(&self, task_id: u32) -> ErrorCode {
        debug!(LOG_LABEL, "remove");
        TaskManager::get_instance().remove(get_calling_uid(), task_id)
    }

    pub fn resume(&self, task_id: u32) -> ErrorCode {
        debug!(LOG_LABEL, "resume");
        TaskManager::get_instance().resume(get_calling_uid(), task_id)
    }

    pub fn on(&self, task_id: u32, on_type: String, obj: RemoteObj) -> ErrorCode {
        let key = on_type.clone() + &String::from("-") + &task_id.to_string();
        debug!(LOG_LABEL, "on key {}", @public(key));
        self.reg_remote_obj.lock().unwrap().insert(key, obj);
        RequestAbility::get_ability_instance().do_unregistered_notify(task_id, on_type);
        ErrorCode::ErrOk
    }

    pub fn off(&self, task_id: u32, off_type: String) -> ErrorCode {
        debug!(LOG_LABEL, "off");
        let key = off_type + &String::from("-") + &task_id.to_string();
        debug!(LOG_LABEL, "off key {}",  @public(key));
        let reg_obj = self.reg_remote_obj.lock().unwrap().clone();
        if !reg_obj.contains_key(&key) {
            error!(LOG_LABEL, "off {} nonexistence",  @public(key));
            return ErrorCode::Other;
        }
        self.reg_remote_obj.lock().unwrap().remove(&key);
        debug!(LOG_LABEL, "off end {}",  @public(&key));
        ErrorCode::ErrOk
    }

    pub fn check_permission(&self) -> bool {
        debug!(LOG_LABEL, "check_permission");
        let token_id = get_calling_token_id();
        debug!(LOG_LABEL, "token_id {}",  @public(&token_id));
        unsafe { request_binding::CheckPermission(token_id) }
    }

    pub fn check_Session_manager_permission(&self) -> QueryPermission {
        debug!(LOG_LABEL, "check_Session_manager_permission");
        let token_id = get_calling_token_id();
        debug!(LOG_LABEL, "token_id {}",  @public(&token_id));
        unsafe { request_binding::CheckSessionManagerPermission(token_id) }
    }

    pub fn start_task(&self, task_id: u32) -> ErrorCode {
        debug!(LOG_LABEL, "start_task");
        TaskManager::get_instance().start(get_calling_uid(), task_id)
    }

    pub fn stop_task(&self, task_id: u32) -> ErrorCode {
        debug!(LOG_LABEL, "stop_task");
        TaskManager::get_instance().stop(get_calling_uid(), task_id)
    }

    pub fn show_task(&self, task_id: u32) -> Option<TaskInfo> {
        debug!(LOG_LABEL, "show_task");
        TaskManager::get_instance().show(get_calling_uid(), task_id)
    }

    pub fn touch_task(&self, task_id: u32, token: String) -> Option<TaskInfo> {
        debug!(LOG_LABEL, "touch_task");
        TaskManager::get_instance().touch(get_calling_uid(), task_id, token)
    }

    pub fn search_task(&self, filter: Filter) -> Option<Vec<u32>> {
        debug!(LOG_LABEL, "search_task");
        let vec = TaskManager::get_instance().search(filter);
        if vec.is_empty() {
            return None;
        }
        Some(vec)
    }

    pub fn is_system_api(&self) -> bool {
        debug!(LOG_LABEL, "is_system_api");
        let token_id = get_calling_token_id();
        debug!(LOG_LABEL, "token_id {}",  @public(&token_id));
        unsafe { request_binding::IsSystemAPI(token_id) }
    }

    pub fn get_calling_bundle(&self) -> String {
        debug!(LOG_LABEL, "get_calling_bundle");
        let token_id = get_calling_token_id();
        debug!(LOG_LABEL, "token_id {}",  @public(&token_id));
        unsafe { request_binding::GetCallingBundle(token_id).to_string() }
    }
    pub fn query_task(&self, task_id: u32, query_permission: QueryPermission) -> Option<TaskInfo> {
        debug!(LOG_LABEL, "touch_task");
        TaskManager::get_instance().query(task_id, query_permission)
    }

    pub fn add_unregister_notify(&self, task_id: u32, reg_type: String) {
        match reg_type.as_str() {
            "complete" | "fail" | "progress" | "remove" => {
                let key = reg_type.clone() + &String::from("-") + &task_id.to_string();
                let notify = self.unregistered_notify.lock().unwrap().clone();
                if notify.contains_key(&key) {
                    return;
                }
                self.unregistered_notify
                    .lock()
                    .unwrap()
                    .insert(key, task_id);
            }
            _ => {}
        }
    }

    fn do_unregistered_notify(&self, task_id: u32, reg_type: String) {
        match TaskManager::get_instance().query_one_task(task_id) {
            Some(task) => {
                let key = reg_type.clone() + &String::from("-") + &task_id.to_string();
                let notify = self.unregistered_notify.lock().unwrap().clone();
                if notify.contains_key(&key) {
                    debug!(LOG_LABEL, "notify taskId: {} event: {}",  @public(task_id),  @public(reg_type));
                    let notify_data = task.build_notify_data();
                    RequestAbility::notify_client(reg_type, &notify_data);
                    self.unregistered_notify.lock().unwrap().remove(&key);
                }
            }
            None => {
                error!(LOG_LABEL, "the task has been removed from the map");
            }
        }
    }

    pub fn get_ability_instance() -> &'static mut RequestAbility {
        static mut REQUESTABILITY: Option<RequestAbility> = None;
        static ONCE: Once = Once::new();
        unsafe {
            ONCE.call_once(|| {
                REQUESTABILITY = Some(RequestAbility::new(
                    ServerRunState::NoStart,
                    Mutex::new(HashMap::new()),
                ));
            });
            REQUESTABILITY.as_mut().unwrap()
        }
    }

    pub fn notify_client(cb_type: String, notify_data: &NotifyData) {
        debug!(LOG_LABEL, "notify_client");
        if notify_data.progress.common_data.index >= notify_data.progress.sizes.len() {
            error!(LOG_LABEL, "index out of range");
            return;
        }
        let common_data = notify_data.progress.common_data;
        if (common_data.state == State::RUNNING as u8 || common_data.state == State::RETRYING as u8) &&
            common_data.total_processed == 0 {
            return;
        }
        debug!(LOG_LABEL, "notify_data {:?}",  @public(notify_data));
        let key = cb_type.clone() + &String::from("-") + &notify_data.task_id.to_string();
        debug!(LOG_LABEL, "key {}",  @public(key));
        {
            let reg_obj = RequestAbility::get_ability_instance()
                .reg_remote_obj
                .lock()
                .unwrap()
                .clone();
            if reg_obj.contains_key(&key) {
                let obj = reg_obj.get(&key).unwrap().clone();
                let mut client_data = MsgParcel::new().expect("MsgParcel should success");
                let notify_token: InterfaceToken =
                    InterfaceToken::new("OHOS.Download.NotifyInterface");
                client_data.write::<InterfaceToken>(&notify_token).ok();
                client_data.write(&cb_type).ok();
                client_data.write(&(notify_data.task_id.to_string())).ok();
                client_data
                    .write(&(notify_data.progress.common_data.state as u32))
                    .ok();
                let index = notify_data.progress.common_data.index;
                client_data.write(&(index as u32)).ok();
                client_data
                    .write(&(notify_data.progress.processed[index] as u64))
                    .ok();
                client_data
                    .write(&(notify_data.progress.common_data.total_processed as u64))
                    .ok();
                client_data.write(&(notify_data.progress.sizes)).ok();
                client_data
                    .write(&(notify_data.progress.extras.len() as u32))
                    .ok();
                for (k, v) in notify_data.progress.extras.iter() {
                    client_data.write(&k).ok();
                    client_data.write(&v).ok();
                }
                client_data.write(&(notify_data.action as u32));
                client_data.write(&(notify_data.version as u32)).ok();

                client_data
                    .write(&(notify_data.each_file_status.len() as u32))
                    .ok();
                for item in notify_data.each_file_status.iter() {
                    client_data.write(&(item.path)).ok();
                    client_data.write(&(item.reason as u32)).ok();
                    client_data.write(&(item.message)).ok();
                }
                debug!(LOG_LABEL, "send_request");
                let reply = obj.send_request(0, &client_data, false).ok();
                return;
            }
            debug!(LOG_LABEL, "key not find");
        }
        if notify_data.version != Version::API10 {
            RequestAbility::get_ability_instance()
                .add_unregister_notify(notify_data.task_id, cb_type);
        }
    }

    pub fn notify_task_info(task_info: &TaskInfo) {
        debug!(LOG_LABEL, "notify_task_info");
        if task_info.progress.common_data.index >= task_info.progress.sizes.len() {
            error!(LOG_LABEL, "index is out of bounds");
            return ;
        }
        let key = String::from("done") + &String::from("-") + &task_info.common_data.task_id.to_string();
        debug!(LOG_LABEL, "key {}",  @public(key));
        let reg_obj = RequestAbility::get_ability_instance()
            .reg_remote_obj
            .lock()
            .unwrap()
            .clone();
        if reg_obj.contains_key(&key) {
            debug!(LOG_LABEL, "notify_task_info contain the key");
            let obj = reg_obj.get(&key).unwrap().clone();
            let mut reply = MsgParcel::new().expect("MsgParcel should success");
            let notify_token: InterfaceToken =
                InterfaceToken::new("OHOS.Download.NotifyInterface");
            reply.write::<InterfaceToken>(&notify_token).ok();
            reply.write(&(task_info.common_data.gauge)).ok();
            reply.write(&(task_info.common_data.retry)).ok();
            reply.write(&(task_info.common_data.action as u32)).ok();
            reply.write(&(task_info.common_data.mode as u32)).ok();
            reply.write(&(task_info.common_data.reason as u32)).ok();
            reply.write(&(task_info.common_data.tries)).ok();
            reply.write(&(task_info.common_data.uid.to_string())).ok();
            reply.write(&(task_info.bundle)).ok();
            reply.write(&task_info.url).ok();
            reply.write(&(task_info.common_data.task_id.to_string())).ok();
            reply.write(&task_info.title).ok();
            reply.write(&task_info.mime_type).ok();
            reply.write(&(task_info.common_data.ctime)).ok();
            reply.write(&(task_info.common_data.mtime)).ok();
            reply.write(&(task_info.data)).ok();
            reply.write(&(task_info.description)).ok();
            reply.write(&(task_info.form_items.len() as u32)).ok();
            for i in 0..task_info.form_items.len() {
                reply.write(&(task_info.form_items[i].name)).ok();
                reply.write(&(task_info.form_items[i].value)).ok();
            }
            reply.write(&(task_info.file_specs.len() as u32)).ok();
            for i in 0..task_info.file_specs.len() {
                reply.write(&(task_info.file_specs[i].name)).ok();
                reply.write(&(task_info.file_specs[i].path)).ok();
                reply.write(&(task_info.file_specs[i].file_name)).ok();
                reply.write(&(task_info.file_specs[i].mime_type)).ok();
            }
            reply.write(&(task_info.progress.common_data.state as u32)).ok();
            let index = task_info.progress.common_data.index;
            reply.write(&(index as u32)).ok();
            reply.write(&(task_info.progress.processed[index] as u64)).ok();
            reply.write(&(task_info.progress.common_data.total_processed as u64)).ok();
            reply.write(&(task_info.progress.sizes)).ok();
            reply.write(&(task_info.progress.extras.len() as u32)).ok();
            for (k, v) in task_info.progress.extras.iter() {
                reply.write(&(k)).ok();
                reply.write(&(v)).ok();
            }
            reply.write(&(task_info.extras.len() as u32)).ok();
            for (k, v) in task_info.extras.iter() {
                reply.write(&(k)).ok();
                reply.write(&(v)).ok();
            }
            reply.write(&(task_info.common_data.version as u32)).ok();
            reply.write(&(task_info.each_file_status.len() as u32)).ok();
            for item in task_info.each_file_status.iter() {
                reply.write(&(item.path)).ok();
                reply.write(&(item.reason as u32)).ok();
                reply.write(&(item.message)).ok();
            }
            debug!(LOG_LABEL, "send_request");
            let reply = obj.send_request(RequestNotifyInterfaceCode::DoneNotify as u32, &reply, false).ok();

            RequestAbility::get_ability_instance().off(task_info.common_data.task_id, String::from("done"));
        }
    }

    pub fn serialize_task_info(&self, tf: TaskInfo, reply: &mut BorrowedMsgParcel, is_system_api: bool) -> IpcResult<()> {
        reply.write(&(tf.common_data.gauge))?;
        reply.write(&(tf.common_data.retry))?;
        reply.write(&(tf.common_data.action as u32))?;
        reply.write(&(tf.common_data.mode as u32))?;
        reply.write(&(tf.common_data.reason as u32))?;
        reply.write(&(tf.common_data.tries))?;
        reply.write(&(tf.common_data.uid.to_string()))?;
        reply.write(&(tf.bundle))?;
        reply.write(&(tf.url))?;
        reply.write(&(tf.common_data.task_id.to_string()))?;
        reply.write(&tf.title)?;
        reply.write(&tf.mime_type)?;
        reply.write(&(tf.common_data.ctime))?;
        reply.write(&(tf.common_data.mtime))?;
        reply.write(&(tf.data))?;
        reply.write(&(tf.description))?;

        reply.write(&(tf.form_items.len() as u32))?;
        for i in 0..tf.form_items.len() {
            reply.write(&(tf.form_items[i].name))?;
            reply.write(&(tf.form_items[i].value))?;
        }

        reply.write(&(tf.file_specs.len() as u32))?;
        for i in 0..tf.file_specs.len() {
            reply.write(&(tf.file_specs[i].name))?;
            reply.write(&(tf.file_specs[i].path))?;
            reply.write(&(tf.file_specs[i].file_name))?;
            reply.write(&(tf.file_specs[i].mime_type))?;
        }

        reply.write(&(tf.progress.common_data.state as u32))?;
        let index = tf.progress.common_data.index;
        reply.write(&(index as u32))?;
        reply.write(&(tf.progress.processed[index] as u64))?;
        reply.write(&(tf.progress.common_data.total_processed as u64))?;
        reply.write(&(tf.progress.sizes))?;

        reply.write(&(tf.progress.extras.len() as u32))?;
        for (k, v) in tf.progress.extras.iter() {
            reply.write(&(k))?;
            reply.write(&(v))?;
        }

        reply.write(&(tf.extras.len() as u32))?;
        for (k, v) in tf.extras.iter() {
            reply.write(&(k))?;
            reply.write(&(v))?;
        }
        reply.write(&(tf.common_data.version as u32))?;
        reply.write(&(tf.each_file_status.len() as u32))?;
        for item in tf.each_file_status.iter() {
            reply.write(&(item.path))?;
            reply.write(&(item.reason as u32))?;
            reply.write(&(item.message))?;
        }
        Ok(())
    }
}
