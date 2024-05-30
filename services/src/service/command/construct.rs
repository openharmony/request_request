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
use std::fs::File;
use std::os::fd::{IntoRawFd, RawFd};

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::manage::events::TaskManagerEvent;
use crate::service::permission::PermissionChecker;
use crate::service::{get_calling_bundle, RequestServiceStub};
use crate::task::config::{Action, CommonTaskConfig, Network, TaskConfig, Version};
use crate::task::info::Mode;
use crate::utils::form_item::{FileSpec, FormItem};

impl RequestServiceStub {
    pub(crate) fn construct(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        info!("Process Service construct");

        if !PermissionChecker::check_internet() {
            error!("End Service construct, failed with reason: no INTERNET permission");
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }

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

        let priority: u32 = data.read()?;

        let url: String = data.read()?;

        let title: String = data.read()?;

        let method: String = data.read()?;

        let token: String = data.read()?;

        let description: String = data.read()?;

        let data_base: String = data.read()?;

        let proxy: String = data.read()?;

        let certificate_pins: String = data.read()?;

        let bundle = get_calling_bundle();

        let uid = ipc::Skeleton::calling_uid();
        let token_id = ipc::Skeleton::calling_full_token_id();
        let pid = ipc::Skeleton::calling_pid();

        let certs_path_size: u32 = data.read()?;
        if certs_path_size > data.readable() as u32 {
            error!("End Service construct, failed with reason: certs_path_size too large");
            reply.write(&(ErrorCode::IpcSizeTooLarge as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        let mut certs_path = Vec::new();
        for _ in 0..certs_path_size {
            let cert_path: String = data.read()?;
            certs_path.push(cert_path);
        }

        let form_size: u32 = data.read()?;
        if form_size > data.readable() as u32 {
            error!("End Service construct, failed with reason: form_size too large");
            reply.write(&(ErrorCode::IpcSizeTooLarge as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        let mut form_items = Vec::new();
        for _ in 0..form_size {
            let name: String = data.read()?;
            let value: String = data.read()?;
            form_items.push(FormItem { name, value });
        }

        let file_size: u32 = data.read()?;
        if file_size > data.readable() as u32 {
            error!("End Service construct, failed with reason: file_specs size too large");
            reply.write(&(ErrorCode::IpcSizeTooLarge as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        let mut file_specs: Vec<FileSpec> = Vec::new();
        for _ in 0..file_size {
            let name: String = data.read()?;
            let path: String = data.read()?;
            let file_name: String = data.read()?;
            let mime_type: String = data.read()?;
            let is_user_file: bool = data.read()?;
            let mut fd: Option<RawFd> = None;
            if is_user_file {
                let ipc_fd: File = data.read_file()?;
                fd = Some(ipc_fd.into_raw_fd());
            }
            file_specs.push(FileSpec {
                name,
                path,
                file_name,
                mime_type,
                is_user_file,
                fd,
            });
        }

        // Response bodies fd.
        let body_file_size: u32 = data.read()?;
        if body_file_size > data.readable() as u32 {
            error!("End Service construct, failed with reason: body_file size too large");
            reply.write(&(ErrorCode::IpcSizeTooLarge as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let mut body_file_paths: Vec<String> = Vec::new();
        for _ in 0..body_file_size {
            let file_name: String = data.read()?;
            body_file_paths.push(file_name);
        }

        let header_size: u32 = data.read()?;
        if header_size > data.readable() as u32 {
            error!("End Service construct, failed with reason: header size too large");
            reply.write(&(ErrorCode::IpcSizeTooLarge as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        let mut headers: HashMap<String, String> = HashMap::new();
        for _ in 0..header_size {
            let key: String = data.read()?;
            let value: String = data.read()?;
            headers.insert(key, value);
        }

        let extras_size: u32 = data.read()?;
        if extras_size > data.readable() as u32 {
            error!("End Service construct, failed with reason: extras size too large");
            reply.write(&(ErrorCode::IpcSizeTooLarge as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        let mut extras: HashMap<String, String> = HashMap::new();
        for _ in 0..extras_size {
            let key: String = data.read()?;
            let value: String = data.read()?;
            extras.insert(key, value);
        }

        let task_config = TaskConfig {
            bundle,
            url,
            title,
            description,
            method,
            headers,
            data: data_base,
            token,
            proxy,
            certificate_pins,
            extras,
            version,
            form_items,
            file_specs,
            body_file_paths,
            certs_path,
            common_data: CommonTaskConfig {
                task_id: 0,
                uid,
                token_id,
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
                priority,
                background,
            },
        };

        debug!("Service construct: task_config constructed");

        let (event, rx) = TaskManagerEvent::construct(task_config);
        if !self.task_manager.send_event(event) {
            return Err(IpcStatusCode::Failed);
        }
        let ret = match rx.get() {
            Some(ret) => ret,
            None => {
                error!("End Service construct, failed with reason: receives ret failed");
                return Err(IpcStatusCode::Failed);
            }
        };

        let task_id = match ret {
            Ok(id) => id,
            Err(err_code) => {
                error!("End Service construct, failed with reason: {:?}", err_code);
                reply.write(&(err_code as i32))?;
                return Err(IpcStatusCode::Failed);
            }
        };

        debug!("Service construct: construct event sent to manager");

        let ret = self.client_manager.subscribe(task_id, pid, uid, token_id);
        if ret != ErrorCode::ErrOk {
            error!(
                "End Service subscribe, task_id is {}, failed with reason: {:?}",
                task_id, ret
            );
            reply.write(&(ret as i32))?;
            reply.write(&(task_id as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        reply.write(&(ErrorCode::ErrOk as i32))?;
        info!("End Service construct, succeed with tid: {}", task_id);
        reply.write(&(task_id as i32))?;
        Ok(())
    }
}
