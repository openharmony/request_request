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
    enumration::*, log::LOG_LABEL, progress::*, request_binding, task_config::TaskConfig,
    task_info::TaskInfo, task_manager::*,
};
use hilog_rust::*;
use ipc_rust::{
    get_calling_token_id, get_calling_uid, FileDesc, IRemoteBroker, IRemoteObj, InterfaceToken,
    IpcResult, IpcStatusCode, MsgParcel, RemoteObj,
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
        TaskManager::get_instance().register_callback(Box::new(RequestAbility::notify_client));
        thread::spawn(|| {
            monitor_network();
            monitor_app_state();
            monitor_task();
        });
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
        let bundle = config.bundle.clone();
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

    pub fn start_task(&self, task_id: u32) -> ErrorCode {
        debug!(LOG_LABEL, "start_task");
        TaskManager::get_instance().start(get_calling_uid(), task_id)
    }

    pub fn stop_task(&self, task_id: u32) -> ErrorCode {
        debug!(LOG_LABEL, "stop_task");
        TaskManager::get_instance().stop(get_calling_uid(), task_id)
    }

    pub fn search_task(&self, task_id: u32) -> bool {
        true
    }

    pub fn show_task(&self, task_id: u32) -> Option<TaskInfo> {
        debug!(LOG_LABEL, "show_task");
        TaskManager::get_instance().show(get_calling_uid(), task_id)
    }

    pub fn touch_task(&self, task_id: u32) -> bool {
        true
    }

    pub fn clear_task(&self, task_id: u32) -> bool {
        true
    }

    pub fn add_unregister_notify(&self, task_id: u32, reg_type: String) {
        debug!(LOG_LABEL, "add_unregister_notify taskId: {} event: {}",  @public(task_id),  @public(reg_type));
        match reg_type.as_str() {
            "complete" | "fail" | "progress" | "pause" | "remove" => {
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
        match self.show_task(task_id) {
            Some(df) => {
                let key = reg_type.clone() + &String::from("-") + &task_id.to_string();
                let notify = self.unregistered_notify.lock().unwrap().clone();
                debug!(LOG_LABEL, "notify {:?}",  @public(notify));
                if notify.contains_key(&key) {
                    debug!(LOG_LABEL, "notify taskId: {} event: {}",  @public(task_id),  @public(reg_type));
                    let mut each_file_status = Vec::<(String, Reason, String)>::new();
                    for item in df.file_specs.iter() {
                        each_file_status.push((
                            item.path.clone(),
                            df.reason,
                            String::new(),
                        ));
                    }
                    let notify_data = NotifyData {
                        progress: df.progress,
                        action: df.common_data.action,
                        version: Version::API9,
                        each_file_status,
                        task_id: df.task_id,
                        uid: df.uid,
                        bundle: df.bundle,
                    };
                    RequestAbility::notify_client(reg_type, &notify_data);
                    self.unregistered_notify.lock().unwrap().remove(&key);
                }
            }
            None => {
                error!(LOG_LABEL, "not find task Api9 complete or fail event");
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
                    client_data.write(&(item.0)).ok();
                    client_data.write(&(item.1 as u32)).ok();
                    client_data.write(&(item.2)).ok();
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
}
