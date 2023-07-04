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
//! This create implement the request server
#![allow(unused_variables, clippy::vec_init_then_push)]
extern crate ipc_rust;
extern crate system_ability_fwk_rust;

use ipc_rust::{
    get_calling_uid, BorrowedMsgParcel, FileDesc, IMsgParcel, IRemoteBroker, IRemoteObj,
    InterfaceToken, IpcResult, IpcStatusCode, MsgParcel, RemoteObj,
};
use std::ffi::{c_char, CString};
use std::{
    collections::HashMap,
    fs::File,
    option::Option,
    string::String,
    sync::{Arc, Mutex},
};

use super::{
    enumration::*, form_item::*, log::LOG_LABEL, request_service_ability::*,
    task_config::*, task_info::*, RequestServiceInterface, filter::*, request_binding,
};
use hilog_rust::*;

static INTERNET_PERMISSION: &str = "ohos.permission.INTERNET";
/// RequestService type
pub struct RequestService;

impl RequestServiceInterface for RequestService {
    fn construct(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        if !RequestAbility::get_ability_instance().check_permission(INTERNET_PERMISSION.to_string()) {
            error!(LOG_LABEL, "permission denied");
            reply.write(&(ErrorCode::Permission as i32));
            return Err(IpcStatusCode::Failed);
        }
        debug!(LOG_LABEL, "construct");
        let action: u32 = data.read()?;
        let action: Action = Action::from(action as u8);
        let version: u32 = data.read()?;
        let version: Version = Version::from(version as u8);
        let mode: u32 = data.read()?;
        let mode: Mode = Mode::from(mode as u8);
        let cover: bool = data.read()?;
        let network: u32 = data.read()?;
        let network: Network = Network::from(network as u8);
        let metered: bool = data.read()?;
        let roaming: bool = data.read()?;
        let retry: bool = data.read()?;
        let redirect: bool = data.read()?;
        let background: bool = data.read()?;
        let index: u32 = data.read()?;
        let begins: i64 = data.read()?;
        let ends: i64 = data.read()?;
        let gauge: bool = data.read()?;
        let precise: bool = data.read()?;
        let url: String = data.read()?;
        let title: String = data.read()?;
        let method: String = data.read()?;
        let token: String = data.read()?;
        let description: String = data.read()?;
        let data_base: String = data.read()?;

        let mut form_items = Vec::<FormItem>::new();
        let form_size: u32 = data.read()?;
        if form_size > data.get_readable_bytes() {
            error!(LOG_LABEL, "size is too large");
            reply.write(&(ErrorCode::Ipc_size_too_large as i32));
            return Err(IpcStatusCode::Failed);
        }
        for i in 0..form_size {
            let name: String = data.read()?;
            let value: String = data.read()?;
            form_items.push(FormItem { name, value });
        }

        let mut files = Vec::<File>::new();
        let mut file_specs: Vec<FileSpec> = Vec::new();
        let file_size: u32 = data.read()?;
        if file_size > data.get_readable_bytes() {
            error!(LOG_LABEL, "size is too large");
            reply.write(&(ErrorCode::Ipc_size_too_large as i32));
            return Err(IpcStatusCode::Failed);
        }
        for i in 0..file_size {
            let name: String = data.read()?;
            let path: String = data.read()?;
            let file_name: String = data.read()?;
            let mime_type: String = data.read()?;
            let fd = data.read::<FileDesc>()?;
            files.push(File::from(fd));
            let fd_error: i32 = data.read()?;
            file_specs.push(FileSpec {
                name,
                path,
                file_name,
                mime_type,
            });
        }

        let header_size: u32 = data.read()?;
        if header_size > data.get_readable_bytes() {
            error!(LOG_LABEL, "size is too large");
            reply.write(&(ErrorCode::Ipc_size_too_large as i32));
            return Err(IpcStatusCode::Failed);
        }
        let mut headers: HashMap<String, String> = HashMap::new();
        for i in 0..header_size {
            let key: String = data.read()?;
            let value: String = data.read()?;
            headers.insert(key, value);
        }

        let extras_size: u32 = data.read()?;
        if extras_size > data.get_readable_bytes() {
            error!(LOG_LABEL, "size is too large");
            reply.write(&(ErrorCode::Ipc_size_too_large as i32));
            return Err(IpcStatusCode::Failed);
        }
        let mut extras: HashMap<String, String> = HashMap::new();
        for i in 0..extras_size {
            let key: String = data.read()?;
            let value: String = data.read()?;
            extras.insert(key, value);
        }
        let bundle = RequestAbility::get_ability_instance().get_calling_bundle();
        let task_config = TaskConfig {
            bundle,
            url,
            title,
            description,
            method,
            headers,
            data: data_base,
            token,
            extras,
            version,
            form_items,
            file_specs,
            common_data: CommonTaskConfig {
                action,
                mode,
                cover,
                network,
                metered,
                roaming,
                retry,
                redirect,
                index,
                begins: begins as u64,
                ends,
                gauge,
                precise,
                background,
            },
        };
        debug!(LOG_LABEL, "files {:?}", @public(files));
        let mut task_id: u32 = 0;
        let ret =
            RequestAbility::get_ability_instance().construct(task_config, files, &mut task_id);
        let remote_object: RemoteObj = data.read::<RemoteObj>()?;
        RequestAbility::get_ability_instance().on(task_id, "done".to_string(), remote_object);
        reply.write(&(ret as i32))?;
        if ret != ErrorCode::ErrOk {
            return Err(IpcStatusCode::Failed);
        }
        debug!(LOG_LABEL, "task id {}",  @public(task_id));
        reply.write(&(task_id as i32))?;
        Ok(())
    }

    fn pause(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        debug!(LOG_LABEL, "Pause");
        let version: u32 = data.read()?;
        if Version::from(version as u8) == Version::API9 {
            if !RequestAbility::get_ability_instance().check_permission(INTERNET_PERMISSION.to_string()) {
                error!(LOG_LABEL, "permission denied");
                reply.write(&(ErrorCode::Permission as i32));
                return Err(IpcStatusCode::Failed);
            }
        }

        let id: String = data.read()?;
        match id.parse::<u32>() {
            Ok(id) => {
                let ret = RequestAbility::get_ability_instance().pause(id);
                reply.write(&(ret as i32))?;
                if ret != ErrorCode::ErrOk {
                    error!(LOG_LABEL, "Pause fail ret {}",  @public(ret as u32));
                    return Err(IpcStatusCode::Failed);
                }
                Ok(())
            }
            _ => {
                error!(LOG_LABEL, "id is not a valid");
                reply.write(&(ErrorCode::TaskNotFound as i32));
                Err(IpcStatusCode::Failed)
            }
        }
    }

    fn query_mime_type(
        &self,
        data: &BorrowedMsgParcel,
        reply: &mut BorrowedMsgParcel,
    ) -> IpcResult<()> {
        if !RequestAbility::get_ability_instance().check_permission(INTERNET_PERMISSION.to_string()) {
            error!(LOG_LABEL, "permission denied");
            reply.write(&(ErrorCode::Permission as i32));
            return Err(IpcStatusCode::Failed);
        }
        debug!(LOG_LABEL, "QueryMimeType");
        let id: String = data.read()?;
        match id.parse::<u32>() {
            Ok(id) => {
                let mime = RequestAbility::get_ability_instance().query_mime_type(id);
                reply.write(&(ErrorCode::ErrOk as i32))?;
                reply.write(&mime)?;
                Ok(())
            }
            _ => {
                error!(LOG_LABEL, "id is not a valid");
                reply.write(&(ErrorCode::TaskNotFound as i32));
                Err(IpcStatusCode::Failed)
            }
        }
    }

    fn remove(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        debug!(LOG_LABEL, "remove");
        let version: u32 = data.read()?;
        if Version::from(version as u8) == Version::API9 {
            if !RequestAbility::get_ability_instance().check_permission(INTERNET_PERMISSION.to_string()) {
                error!(LOG_LABEL, "permission denied");
                reply.write(&(ErrorCode::Permission as i32));
                return Err(IpcStatusCode::Failed);
            }
        }

        let id: String = data.read()?;
        debug!(LOG_LABEL, "id {}",  @public(id));
        match id.parse::<u32>() {
            Ok(id) => {
                let ret = RequestAbility::get_ability_instance().remove(id);
                reply.write(&(ret as i32))?;
                if ret != ErrorCode::ErrOk {
                    error!(LOG_LABEL, "Remove fail ret {}",  @public(ret as i32));
                    return Err(IpcStatusCode::Failed);
                }
                Ok(())
            }
            _ => {
                error!(LOG_LABEL, "id is not a valid");
                reply.write(&(ErrorCode::TaskNotFound as i32));
                Err(IpcStatusCode::Failed)
            }
        }
    }

    fn resume(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        if !RequestAbility::get_ability_instance().check_permission(INTERNET_PERMISSION.to_string()) {
            error!(LOG_LABEL, "permission denied");
            reply.write(&(ErrorCode::Permission as i32));
            return Err(IpcStatusCode::Failed);
        }
        debug!(LOG_LABEL, "resume");
        let id: String = data.read()?;
        match id.parse::<u32>() {
            Ok(id) => {
                let ret = RequestAbility::get_ability_instance().resume(id);
                reply.write(&(ret as i32))?;
                if ret != ErrorCode::ErrOk {
                    error!(LOG_LABEL, "Resume fail ret {}",  @public(ret as i32));
                    return Err(IpcStatusCode::Failed);
                }
                Ok(())
            }
            _ => {
                error!(LOG_LABEL, "id is not a valid");
                reply.write(&(ErrorCode::TaskNotFound as i32));
                Err(IpcStatusCode::Failed)
            }
        }
    }

    fn on(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        debug!(LOG_LABEL, "on");
        let on_type: String = data.read()?;
        if on_type.is_empty() {
            error!(LOG_LABEL, "id is not a valid");
            reply.write(&(ErrorCode::Parameter_check as i32));
            return Err(IpcStatusCode::Failed);
        }
        let id: String = data.read()?;
        match id.parse::<u32>() {
            Ok(id) => {
                let remote_object: RemoteObj = data.read::<RemoteObj>()?;
                let ret = RequestAbility::get_ability_instance().on(id, on_type, remote_object);
                reply.write(&(ret as i32))?;
                if ret != ErrorCode::ErrOk {
                    error!(LOG_LABEL, "on fail");
                    return Err(IpcStatusCode::Failed);
                }
                Ok(())
            }
            _ => {
                error!(LOG_LABEL, "id is not a valid");
                reply.write(&(ErrorCode::TaskNotFound as i32));
                Err(IpcStatusCode::Failed)
            }
        }
    }

    fn off(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        debug!(LOG_LABEL, "off");
        let off_type: String = data.read()?;
        debug!(LOG_LABEL, "off_type: {:?}",  @public(off_type));
        if off_type.is_empty() {
            error!(LOG_LABEL, "id is not a valid");
            reply.write(&(ErrorCode::Parameter_check as i32));
            return Err(IpcStatusCode::Failed);
        }
        let id: String = data.read()?;
        debug!(LOG_LABEL, "id {}",  @public(id));
        match id.parse::<u32>() {
            Ok(id) => {
                debug!(LOG_LABEL, "int id: {:?}",  @public(id));
                let ret = RequestAbility::get_ability_instance().off(id, off_type);
                reply.write(&(ret as i32))?;
                if ret != ErrorCode::ErrOk {
                    error!(LOG_LABEL, "Off fail ret {}",  @public(ret as i32));
                    return Err(IpcStatusCode::Failed);
                }
                Ok(())
            }
            _ => {
                error!(LOG_LABEL, "id is not a valid");
                reply.write(&(ErrorCode::TaskNotFound as i32));
                Err(IpcStatusCode::Failed)
            }
        }
    }

    fn start(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        if !RequestAbility::get_ability_instance().check_permission(INTERNET_PERMISSION.to_string()) {
            error!(LOG_LABEL, "permission denied");
            reply.write(&(ErrorCode::Permission as i32));
            return Err(IpcStatusCode::Failed);
        }
        debug!(LOG_LABEL, "start");
        let id: String = data.read()?;
        match id.parse::<u32>() {
            Ok(id) => {
                let ret = RequestAbility::get_ability_instance().start_task(id);
                reply.write(&(ret as i32))?;
                if ret != ErrorCode::ErrOk {
                    error!(LOG_LABEL, "start fail ret {}",  @public(ret as i32));
                    return Err(IpcStatusCode::Failed);
                }
                Ok(())
            }
            _ => {
                error!(LOG_LABEL, "id is not a valid");
                reply.write(&(ErrorCode::TaskNotFound as i32));
                Err(IpcStatusCode::Failed)
            }
        }
    }

    fn stop(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        debug!(LOG_LABEL, "stop");
        let id: String = data.read()?;
        match id.parse::<u32>() {
            Ok(id) => {
                let ret = RequestAbility::get_ability_instance().stop_task(id);
                reply.write(&(ret as i32))?;
                if ret != ErrorCode::ErrOk {
                    error!(LOG_LABEL, "stop fail ret {}",  @public(ret as i32));
                    return Err(IpcStatusCode::Failed);
                }
                Ok(())
            }
            _ => {
                error!(LOG_LABEL, "id is not a valid");
                reply.write(&(ErrorCode::TaskNotFound as i32));
                Err(IpcStatusCode::Failed)
            }
        }
    }

    fn show(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        debug!(LOG_LABEL, "show");
        if !RequestAbility::get_ability_instance().check_permission(INTERNET_PERMISSION.to_string()) {
            error!(LOG_LABEL, "permission denied");
            reply.write(&(ErrorCode::Permission as i32));
            return Err(IpcStatusCode::Failed);
        }
        let id: String = data.read()?;
        debug!(LOG_LABEL, "id: {}", @public(id));
        match id.parse::<u32>() {
            Ok(id) => match RequestAbility::get_ability_instance().show_task(id) {
                Some(tf) => {
                    reply.write(&(ErrorCode::ErrOk as i32));
                    debug!(LOG_LABEL, "tf: {:?}",  @public(tf));
                    RequestAbility::get_ability_instance().serialize_task_info(tf, reply, false)?;
                    Ok(())
                }
                None => {
                    error!(LOG_LABEL, "id is not a valid");
                    reply.write(&(ErrorCode::TaskNotFound as i32));
                    Err(IpcStatusCode::Failed)
                }
            },
            _ => {
                error!(LOG_LABEL, "id is not a valid");
                reply.write(&(ErrorCode::TaskNotFound as i32));
                Err(IpcStatusCode::Failed)
            }
        }
    }

    fn touch(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        debug!(LOG_LABEL, "touch");
        let id: String = data.read()?;
        debug!(LOG_LABEL, "id: {}", @public(id));
        match id.parse::<u32>() {
            Ok(id) => {
                let token: String = data.read()?;
                match RequestAbility::get_ability_instance().touch_task(id, token) {
                    Some(tf) => {
                        reply.write(&(ErrorCode::ErrOk as i32));
                        debug!(LOG_LABEL, "tf: {:?}",  @public(tf));
                        RequestAbility::get_ability_instance().serialize_task_info(tf, reply, false)?;
                        return Ok(());
                    }
                    None => {
                        error!(LOG_LABEL, "id is not a valid");
                        reply.write(&(ErrorCode::TaskNotFound as i32));
                        Err(IpcStatusCode::Failed)
                    }
                }
            },
            _ => {
                error!(LOG_LABEL, "id or token is not a valid");
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                return Err(IpcStatusCode::Failed);
            }
        }
    }

    fn search(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        debug!(LOG_LABEL, "search");
        let mut bundle: String = data.read()?;
        if !RequestAbility::get_ability_instance().is_system_api() {
            bundle = RequestAbility::get_ability_instance().get_calling_bundle();
        }
        debug!(LOG_LABEL, "search bundle is {}", @public(bundle));
        let before: i64 = data.read()?;
        debug!(LOG_LABEL, "search before is {}", @public(before));
        let after: i64 = data.read()?;
        debug!(LOG_LABEL, "search after is {}", @public(after));
        let state: u32 = data.read()?;
        debug!(LOG_LABEL, "search state is {}", @public(state));
        let action: u32 = data.read()?;
        debug!(LOG_LABEL, "search action is {}", @public(action));
        let mode: u32 = data.read()?;
        debug!(LOG_LABEL, "search mode is {}", @public(mode));
        let common_data = CommonFilter {
            before,
            after,
            state: state as u8,
            action: action as u8,
            mode: mode as u8,
        };
        let filter = Filter {
            bundle,
            common_data,
        };
        let ids = RequestAbility::get_ability_instance().search_task(filter);
        debug!(LOG_LABEL, "search task ids is {:?}", @public(ids));
        reply.write(&(ids.len() as u32));
        for it in ids.iter() {
            reply.write(&(it.to_string()))?;
        }
        Ok(())
    }

    fn query(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        debug!(LOG_LABEL, "query");
        if !RequestAbility::get_ability_instance().is_system_api() {
            error!(LOG_LABEL, "not system api");
            reply.write(&(ErrorCode::SystemApi as i32));
            return Err(IpcStatusCode::Failed);
        }
        let ret = RequestAbility::get_ability_instance().get_query_permission();
        if ret == QueryPermission::NoPermisson {
            error!(LOG_LABEL, "no query permission");
            reply.write(&(ErrorCode::Permission as i32));
            return Err(IpcStatusCode::Failed);
        }
        let id: String = data.read()?;
        debug!(LOG_LABEL, "id: {}", @public(id));
        match id.parse::<u32>() {
            Ok(id) => match RequestAbility::get_ability_instance().query_task(id, ret) {
                Some(tf) => {
                    reply.write(&(ErrorCode::ErrOk as i32));
                    debug!(LOG_LABEL, "tf: {:?}",  @public(tf));
                    RequestAbility::get_ability_instance().serialize_task_info(tf, reply, true)?;
                    Ok(())
                }
                None => {
                    error!(LOG_LABEL, "id is not a valid");
                    reply.write(&(ErrorCode::TaskNotFound as i32));
                    Err(IpcStatusCode::Failed)
                }
            },
            _ => {
                error!(LOG_LABEL, "id is not a valid");
                reply.write(&(ErrorCode::TaskNotFound as i32));
                Err(IpcStatusCode::Failed)
            }
        }
    }
}

/// start
pub fn start() {
    RequestAbility::get_ability_instance().start();
}

/// stop
pub fn stop() {
    RequestAbility::get_ability_instance().stop();
}

impl IRemoteBroker for RequestService {}
