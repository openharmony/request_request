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

pub(crate) mod client;
pub(crate) mod command;
mod interface;
pub(crate) mod permission;
pub(crate) mod runcount;

use std::fs::File;

use ipc::parcel::MsgParcel;
use ipc::remote::RemoteStub;
use ipc::{IpcResult, IpcStatusCode};

use self::client::ClientManagerEntry;
use self::runcount::RunCountManagerEntry;
use crate::manage::task_manager::TaskManagerTx;
use crate::task::config::TaskConfig;
use crate::task::info::TaskInfo;
use crate::utils::c_wrapper::CStringWrapper;

pub(crate) struct RequestServiceStub {
    task_manager: TaskManagerTx,
    client_manager: ClientManagerEntry,
    runcount_manager: RunCountManagerEntry,
}

impl RequestServiceStub {
    pub(crate) fn new(
        task_manager: TaskManagerTx,
        client_manager: ClientManagerEntry,
        runcount_manager: RunCountManagerEntry,
    ) -> Self {
        Self {
            task_manager,
            client_manager,
            runcount_manager,
        }
    }
}

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
            interface::CONSTRUCT => self.construct(data, reply),
            interface::PAUSE => self.pause(data, reply),

            interface::QUERY => self.query(data, reply),
            interface::QUERY_MIME_TYPE => self.query_mime_type(data, reply),
            interface::REMOVE => self.remove(data, reply),
            interface::RESUME => self.resume(data, reply),
            interface::START => self.start(data, reply),
            interface::STOP => self.stop(data, reply),
            interface::SHOW => self.show(data, reply),
            interface::TOUCH => self.touch(data, reply),
            interface::SEARCH => self.search(data, reply),
            interface::GET_TASK => self.get_task(data, reply),
            interface::CLEAR => Ok(()),
            interface::OPEN_CHANNEL => self.open_channel(reply),
            interface::SUBSCRIBE => self.subscribe(data, reply),
            interface::UNSUBSCRIBE => self.unsubscribe(data, reply),
            interface::SUB_RUN_COUNT => self.sub_runcount(data, reply),
            interface::UNSUB_RUN_COUNT => self.unsub_runcount(reply),
            _ => return IpcStatusCode::Failed as i32,
        };

        match res {
            Ok(_) => 0,
            Err(e) => e as i32,
        }
    }

    fn dump(&self, file: File, args: Vec<String>) -> i32 {
        match self.dump(file, args) {
            Ok(()) => 0,
            Err(e) => e as i32,
        }
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

#[link(name = "download_server_cxx", kind = "static")]
extern "C" {
    pub(crate) fn GetCallingBundle(token_id: u64) -> CStringWrapper;
    pub(crate) fn RequestIsSystemAPI(token_id: u64) -> bool;
}
