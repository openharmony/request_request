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

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::info::TaskInfo;
use crate::service::command::{set_code_with_index_other, GET_INFO_MAX};
use crate::service::permission::PermissionChecker;
use crate::service::{serialize_task_info, RequestServiceStub};
use crate::utils::is_system_api;

impl RequestServiceStub {
    pub(crate) fn query(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        if !is_system_api() {
            error!("Service query: not system api");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A05,
                "Service query: not system api"
            );
            reply.write(&(ErrorCode::SystemApi as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        let permission = PermissionChecker::check_manager();
        let action = match permission.get_action() {
            Some(a) => a,
            None => {
                error!("Service query: no QUERY permission");
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A05,
                    "Service query: no QUERY permission"
                );
                reply.write(&(ErrorCode::Permission as i32))?;
                return Err(IpcStatusCode::Failed);
            }
        };

        let len: u32 = data.read()?;
        let len = len as usize;
        let mut vec = vec![(ErrorCode::Other, TaskInfo::new()); len];

        if len > GET_INFO_MAX {
            info!("Service query: out of size: {}", len);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        for i in 0..len {
            let task_id: String = data.read()?;
            info!("Service query tid {}", task_id);

            let Ok(task_id) = task_id.parse::<u32>() else {
                error!("Service query, failed: tid not valid: {}", task_id);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A06,
                    &format!("Service query, failed: tid not valid: {}", task_id)
                );
                set_code_with_index_other(&mut vec, i, ErrorCode::TaskNotFound);
                continue;
            };

            let info = self.task_manager.lock().unwrap().query(task_id, action);
            match info {
                Some(task_info) => {
                    if let Some((c, info)) = vec.get_mut(i) {
                        *c = ErrorCode::ErrOk;
                        *info = task_info;
                    }
                }
                None => {
                    error!("Service query, failed: task_id not found, tid: {}", task_id);
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A06,
                        &format!("Service query, failed: task_id not found, tid: {}", task_id)
                    );
                    set_code_with_index_other(&mut vec, i, ErrorCode::TaskNotFound);
                }
            };
        }
        reply.write(&(ErrorCode::ErrOk as i32))?;
        for (c, info) in vec {
            reply.write(&(c as i32))?;
            // TODO: Sends info only when ErrOk.
            serialize_task_info(info, reply)?;
        }
        Ok(())
    }
}
