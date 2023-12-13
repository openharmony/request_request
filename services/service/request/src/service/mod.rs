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

//! This crate implement the request server service.

pub(crate) mod ability;
pub(crate) mod command;
#[allow(unused)]
pub(crate) mod interface;
pub(crate) mod listener;
pub(crate) mod notify;
pub(crate) mod permission;

use std::fs::{File, OpenOptions};

pub(crate) use interface::{RequestInterfaceCode, RequestNotifyInterfaceCode};
use ipc_rust::{
    define_remote_object, get_calling_token_id, BorrowedMsgParcel, FileDesc, IRemoteBroker,
    InterfaceToken, IpcResult, IpcStatusCode, RemoteObj, RemoteStub, String16,
};

use crate::task::info::TaskInfo;
use crate::task::config::TaskConfig;
use crate::utils::c_wrapper::CStringWrapper;

define_remote_object!(
    RequestServiceInterface["ohos.request.service"] {
        stub: RequestServiceStub(on_remote_request),
        proxy: RequestServiceProxy,
    }
);

fn on_remote_request(
    stub: &dyn RequestServiceInterface,
    code: u32,
    data: &BorrowedMsgParcel,
    reply: &mut BorrowedMsgParcel,
) -> IpcResult<()> {
    const SERVICE_TOKEN: &str = "OHOS.Download.RequestServiceInterface";

    info!("Processes on_remote_request, code: {}", code);
    match data.read::<InterfaceToken>().map(|token| token.get_token()) {
        Ok(token) if token == SERVICE_TOKEN => {}
        _ => {
            error!("Gets invalid token");
            return Err(IpcStatusCode::Failed);
        }
    };
    match code.try_into()? {
        RequestInterfaceCode::Construct => stub.construct(data, reply),
        RequestInterfaceCode::Pause => stub.pause(data, reply),
        RequestInterfaceCode::Query => stub.query(data, reply),
        RequestInterfaceCode::QueryMimeType => stub.query_mime_type(data, reply),
        RequestInterfaceCode::Remove => stub.remove(data, reply),
        RequestInterfaceCode::Resume => stub.resume(data, reply),
        RequestInterfaceCode::On => stub.on(data, reply),
        RequestInterfaceCode::Off => stub.off(data, reply),
        RequestInterfaceCode::Start => stub.start(data, reply),
        RequestInterfaceCode::Stop => stub.stop(data, reply),
        RequestInterfaceCode::Show => stub.show(data, reply),
        RequestInterfaceCode::Touch => stub.touch(data, reply),
        RequestInterfaceCode::Search => stub.search(data, reply),
        RequestInterfaceCode::GetTask => stub.get_task(data, reply),
        RequestInterfaceCode::Clear => Ok(()),
    }
}

impl TryFrom<u32> for RequestInterfaceCode {
    type Error = IpcStatusCode;

    fn try_from(code: u32) -> IpcResult<Self> {
        match code {
            _ if code == Self::Construct as u32 => Ok(Self::Construct),
            _ if code == Self::Pause as u32 => Ok(Self::Pause),
            _ if code == Self::Query as u32 => Ok(Self::Query),
            _ if code == Self::QueryMimeType as u32 => Ok(Self::QueryMimeType),
            _ if code == Self::Remove as u32 => Ok(Self::Remove),
            _ if code == Self::Resume as u32 => Ok(Self::Resume),
            _ if code == Self::On as u32 => Ok(Self::On),
            _ if code == Self::Off as u32 => Ok(Self::Off),
            _ if code == Self::Start as u32 => Ok(Self::Start),
            _ if code == Self::Stop as u32 => Ok(Self::Stop),
            _ if code == Self::Show as u32 => Ok(Self::Show),
            _ if code == Self::Touch as u32 => Ok(Self::Touch),
            _ if code == Self::Search as u32 => Ok(Self::Search),
            _ if code == Self::GetTask as u32 => Ok(Self::GetTask),
            _ if code == Self::Clear as u32 => Ok(Self::Clear),
            _ => Err(IpcStatusCode::Failed),
        }
    }
}

/// Functions between proxy and stub.
pub trait RequestServiceInterface: IRemoteBroker {
    /// Constructs or creates a task.
    fn construct(
        &self,
        _data: &BorrowedMsgParcel,
        _reply: &mut BorrowedMsgParcel,
    ) -> IpcResult<()> {
        Ok(())
    }

    /// Pauses a task.
    fn pause(&self, _data: &BorrowedMsgParcel, _reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    /// Queries tasks.
    fn query(&self, _data: &BorrowedMsgParcel, _reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    /// Queries the mime type of a task.
    fn query_mime_type(
        &self,
        _data: &BorrowedMsgParcel,
        _reply: &mut BorrowedMsgParcel,
    ) -> IpcResult<()> {
        Ok(())
    }

    /// Removes a task.
    fn remove(&self, _data: &BorrowedMsgParcel, _reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    /// Resumes a task.
    fn resume(&self, _data: &BorrowedMsgParcel, _reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    /// Sets the `on` callback of a task.
    fn on(&self, _data: &BorrowedMsgParcel, _reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    /// Sets the `off` callback of a task.
    fn off(&self, _data: &BorrowedMsgParcel, _reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    /// Starts a task.
    fn start(&self, _data: &BorrowedMsgParcel, _reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    /// Stops a task.
    fn stop(&self, _data: &BorrowedMsgParcel, _reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    /// Shows a specified task details which belongs to the caller.
    fn show(&self, _data: &BorrowedMsgParcel, _reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    /// Touches a specified task with token.
    fn touch(&self, _data: &BorrowedMsgParcel, _reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    /// Searches tasks of this system.
    fn search(&self, _data: &BorrowedMsgParcel, _reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }

    /// Get a task of this system.
    fn get_task(&self, _data: &BorrowedMsgParcel, _reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        Ok(())
    }
}

impl RequestServiceInterface for RequestServiceProxy {}

/// RequestService type
pub struct RequestService;

impl IRemoteBroker for RequestService {
    fn dump(&self, file: &FileDesc, args: &mut Vec<String16>) -> i32 {
        command::Dump::execute(file, args)
    }
}

impl RequestServiceInterface for RequestService {
    fn construct(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        command::Construct::execute(data, reply)
    }

    fn pause(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        command::Pause::execute(data, reply)
    }

    fn query(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        command::Query::execute(data, reply)
    }

    fn query_mime_type(
        &self,
        data: &BorrowedMsgParcel,
        reply: &mut BorrowedMsgParcel,
    ) -> IpcResult<()> {
        command::QueryMimeType::execute(data, reply)
    }

    fn remove(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        command::Remove::execute(data, reply)
    }

    fn resume(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        command::Resume::execute(data, reply)
    }

    fn on(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        command::On::execute(data, reply)
    }

    fn off(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        command::Off::execute(data, reply)
    }

    fn start(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        command::Start::execute(data, reply)
    }

    fn stop(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        command::Stop::execute(data, reply)
    }

    fn show(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        command::Show::execute(data, reply)
    }

    fn touch(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        command::Touch::execute(data, reply)
    }

    fn search(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        command::Search::execute(data, reply)
    }

    fn get_task(&self, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
        command::GetTask::execute(data, reply)
    }
}

pub(crate) fn serialize_task_info(tf: TaskInfo, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
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
    reply.write(&(tf.common_data.priority))?;

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

pub(crate) fn serialize_task_config(config: TaskConfig, reply: &mut BorrowedMsgParcel) -> IpcResult<()> {
    reply.write(&(config.common_data.action as u32))?;
    reply.write(&(config.common_data.mode as u32))?;
    reply.write(&(config.common_data.cover))?;
    reply.write(&(config.common_data.network as u32))?;
    reply.write(&(config.common_data.metered))?;
    reply.write(&(config.common_data.roaming))?;
    reply.write(&(config.common_data.retry))?;
    reply.write(&(config.common_data.redirect))?;
    reply.write(&(config.common_data.index))?;
    reply.write(&(config.common_data.begins))?;
    reply.write(&(config.common_data.ends))?;
    reply.write(&(config.common_data.gauge))?;
    reply.write(&(config.common_data.precise))?;
    reply.write(&(config.common_data.priority))?;
    reply.write(&(config.common_data.background))?;
    reply.write(&(config.bundle))?;
    reply.write(&(config.url))?;
    reply.write(&(config.title))?;
    reply.write(&(config.description))?;
    reply.write(&(config.method))?;
    // write config.headers
    reply.write(&(config.headers.len() as u32))?;
    for (k, v) in config.headers.iter() {
        reply.write(&(k))?;
        reply.write(&(v))?;
    }
    reply.write(&(config.data))?;
    reply.write(&(config.token))?;
    // write config.extras
    reply.write(&(config.extras.len() as u32))?;
    for (k, v) in config.extras.iter() {
        reply.write(&(k))?;
        reply.write(&(v))?;
    }
    reply.write(&(config.version as u32))?;
    // write config.form_items
    reply.write(&(config.form_items.len() as u32))?;
    for i in 0..config.form_items.len() {
        reply.write(&(config.form_items[i].name))?;
        reply.write(&(config.form_items[i].value))?;
    }
    // write config.file_specs
    reply.write(&(config.file_specs.len() as u32))?;
    for i in 0..config.file_specs.len() {
        reply.write(&(config.file_specs[i].name))?;
        reply.write(&(config.file_specs[i].path))?;
        reply.write(&(config.file_specs[i].file_name))?;
        reply.write(&(config.file_specs[i].mime_type))?;
    }
    // write config.body_file_names
    reply.write(&(config.body_file_names.len() as u32))?;
    for i in 0..config.body_file_names.len() {
        reply.write(&(config.body_file_names[i]))?;
    }
    Ok(())
}

pub(crate) fn get_calling_bundle() -> String {
    debug!("Gets calling bundle");
    let token_id = get_calling_token_id();
    debug!("Gets token id {}", &token_id);
    unsafe { GetCallingBundle(token_id).to_string() }
}

pub(crate) fn is_system_api() -> bool {
    debug!("Checks if the api is a system_api");
    let token_id = get_calling_token_id();
    debug!("Gets token id {}", &token_id);
    unsafe { RequestIsSystemAPI(token_id) }
}

pub(crate) fn open_file_readwrite(uid: u64, bundle: &str, path: &str) -> IpcResult<File> {
    match OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(convert_path(uid, bundle, path))
    {
        Ok(file) => Ok(file),
        Err(e) => {
            error!("open_file_readwrite failed, err is {:?}", e);
            Err(IpcStatusCode::Failed)
        }
    }
}

pub(crate) fn open_file_readonly(uid: u64, bundle: &str, path: &str) -> IpcResult<File> {
    match OpenOptions::new()
        .read(true)
        .open(convert_path(uid, bundle, path))
    {
        Ok(file) => Ok(file),
        Err(e) => {
            error!("open_file_readonly failed, err is {:?}", e);
            Err(IpcStatusCode::Failed)
        }
    }
}

fn convert_path(uid: u64, bundle: &str, path: &str) -> String {
    let uuid = uid / 200000;
    let base_replace = format!("{}/base/{}", uuid, bundle);
    let real_path = path
        .replacen("storage", "app", 1)
        .replacen("base", &base_replace, 1);
    debug!("convert to real_path: {}", real_path);
    real_path
}

extern "C" {
    pub(crate) fn GetCallingBundle(token_id: u64) -> CStringWrapper;
    pub(crate) fn RequestIsSystemAPI(token_id: u64) -> bool;
}
