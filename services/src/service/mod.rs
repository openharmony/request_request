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
pub(crate) mod client;
pub(crate) mod command;
#[allow(unused)]
pub(crate) mod interface;
pub(crate) mod listener;
pub(crate) mod permission;
pub(crate) mod runcount;

use std::fs::File;

pub(crate) use interface::{RequestInterfaceCode, RequestNotifyInterfaceCode};
use ipc::parcel::MsgParcel;
use ipc::remote::RemoteStub;
use ipc::{IpcResult, IpcStatusCode};

use crate::task::config::TaskConfig;
use crate::task::info::TaskInfo;
use crate::utils::c_wrapper::CStringWrapper;

pub(crate) struct RequestServiceStub;

impl RemoteStub for RequestServiceStub {
    fn on_remote_request(&self, code: u32, data: &mut MsgParcel, reply: &mut MsgParcel) -> i32 {
        const SERVICE_TOKEN: &str = "OHOS.Download.RequestServiceInterface";

        debug!("Processes on_remote_request, code: {}", code);
        match data.read_interface_token() {
            Ok(token) if token == SERVICE_TOKEN => {}
            _ => {
                error!("Gets invalid token");
                return IpcStatusCode::Failed as i32;
            }
        };

        let res = match code {
            _ if code == RequestInterfaceCode::Construct as u32 => {
                RequestService::construct(data, reply)
            }
            _ if code == RequestInterfaceCode::Pause as u32 => RequestService::pause(data, reply),
            _ if code == RequestInterfaceCode::Query as u32 => RequestService::query(data, reply),
            _ if code == RequestInterfaceCode::QueryMimeType as u32 => {
                RequestService::query_mime_type(data, reply)
            }
            _ if code == RequestInterfaceCode::Remove as u32 => RequestService::remove(data, reply),
            _ if code == RequestInterfaceCode::Resume as u32 => RequestService::resume(data, reply),
            _ if code == RequestInterfaceCode::Start as u32 => RequestService::start(data, reply),
            _ if code == RequestInterfaceCode::Stop as u32 => RequestService::stop(data, reply),
            _ if code == RequestInterfaceCode::Show as u32 => RequestService::show(data, reply),
            _ if code == RequestInterfaceCode::Touch as u32 => RequestService::touch(data, reply),
            _ if code == RequestInterfaceCode::Search as u32 => RequestService::search(data, reply),
            _ if code == RequestInterfaceCode::GetTask as u32 => {
                RequestService::get_task(data, reply)
            }
            _ if code == RequestInterfaceCode::Clear as u32 => Ok(()),
            _ if code == RequestInterfaceCode::OpenChannel as u32 => {
                RequestService::open_channel(data, reply)
            }
            _ if code == RequestInterfaceCode::Subscribe as u32 => {
                RequestService::subscribe(data, reply)
            }
            _ if code == RequestInterfaceCode::Unsubscribe as u32 => {
                RequestService::unsubscribe(data, reply)
            }
            _ if code == RequestInterfaceCode::SubRunCount as u32 => {
                RequestService::sub_runcount(data, reply)
            }
            _ if code == RequestInterfaceCode::UnsubRunCount as u32 => {
                RequestService::unsub_runcount(data, reply)
            }
            _ => return IpcStatusCode::Failed as i32,
        };

        match res {
            Ok(_) => 0,
            Err(e) => e as i32,
        }
    }

    fn dump(&self, file: File, args: Vec<String>) -> i32 {
        match command::Dump::execute(file, args) {
            Ok(()) => 0,
            Err(e) => e as i32,
        }
    }
}

/// RequestService type
struct RequestService;

impl RequestService {
    fn construct(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::Construct::execute(data, reply)
    }

    fn pause(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::Pause::execute(data, reply)
    }

    fn query(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::Query::execute(data, reply)
    }

    fn query_mime_type(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::QueryMimeType::execute(data, reply)
    }

    fn remove(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::Remove::execute(data, reply)
    }

    fn resume(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::Resume::execute(data, reply)
    }

    fn start(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::Start::execute(data, reply)
    }

    fn stop(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::Stop::execute(data, reply)
    }

    fn show(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::Show::execute(data, reply)
    }

    fn touch(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::Touch::execute(data, reply)
    }

    fn search(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::Search::execute(data, reply)
    }

    fn get_task(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::GetTask::execute(data, reply)
    }

    fn open_channel(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::OpenChannel::execute(data, reply)
    }

    fn subscribe(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::Subscribe::execute(data, reply)
    }

    fn unsubscribe(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::Unsubscribe::execute(data, reply)
    }

    fn sub_runcount(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::SubRunCount::execute(data, reply)
    }

    fn unsub_runcount(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        command::UnsubRunCount::execute(data, reply)
    }
}

pub(crate) fn serialize_task_info(tf: TaskInfo, reply: &mut MsgParcel) -> IpcResult<()> {
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
        reply.write(k)?;
        reply.write(v)?;
    }

    reply.write(&(tf.extras.len() as u32))?;
    for (k, v) in tf.extras.iter() {
        reply.write(k)?;
        reply.write(v)?;
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

pub(crate) fn serialize_task_config(config: TaskConfig, reply: &mut MsgParcel) -> IpcResult<()> {
    reply.write(&(config.common_data.action as u32))?;
    reply.write(&(config.common_data.mode as u32))?;
    reply.write(&(config.bundle_type))?;
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
        reply.write(k)?;
        reply.write(v)?;
    }
    reply.write(&(config.data))?;
    reply.write(&(config.token))?;
    // write config.extras
    reply.write(&(config.extras.len() as u32))?;
    for (k, v) in config.extras.iter() {
        reply.write(k)?;
        reply.write(v)?;
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
    reply.write(&(config.body_file_paths.len() as u32))?;
    for i in 0..config.body_file_paths.len() {
        reply.write(&(config.body_file_paths[i]))?;
    }
    Ok(())
}

pub(crate) fn get_calling_bundle() -> String {
    debug!("Gets calling bundle");
    let token_id = ipc::Skeleton::calling_full_token_id();
    debug!("Gets token id {}", &token_id);
    unsafe { GetCallingBundle(token_id).to_string() }
}

pub(crate) fn is_system_api() -> bool {
    debug!("Checks if the api is a system_api");
    let token_id = ipc::Skeleton::calling_full_token_id();
    debug!("Gets token id {}", &token_id);
    unsafe { RequestIsSystemAPI(token_id) }
}

#[cfg(feature = "oh")]
#[link(name = "request_service_c")]
extern "C" {
    pub(crate) fn GetCallingBundle(token_id: u64) -> CStringWrapper;
    pub(crate) fn RequestIsSystemAPI(token_id: u64) -> bool;
}
