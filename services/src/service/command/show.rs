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
use crate::manage::database::RequestDb;
use crate::service::command::{set_code_with_index_other, GET_INFO_MAX};
use crate::service::permission::PermissionChecker;
use crate::service::{serialize_task_info, RequestServiceStub};
use crate::task::files::check_current_account;

impl RequestServiceStub {
    pub(crate) fn show(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        info!("Service show");
        let permission = PermissionChecker::check_down_permission();
        let len: u32 = data.read()?;
        let len = len as usize;
        let mut vec = vec![(ErrorCode::Other, TaskInfo::new()); len];

        if len > GET_INFO_MAX {
            info!("Service show: out of size: {}", len);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let ipc_uid = ipc::Skeleton::calling_uid();
        for i in 0..len {
            let task_id: String = data.read()?;
            info!("Service show tid {}", task_id);

            let Ok(task_id) = task_id.parse::<u32>() else {
                error!("Service show, failed: tid not valid: {}", task_id);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A18,
                    &format!("Service show, failed: tid not valid: {}", task_id)
                );
                set_code_with_index_other(&mut vec, i, ErrorCode::TaskNotFound);
                continue;
            };

            let task_uid = match RequestDb::get_instance().query_task_uid(task_id) {
                Some(uid) => uid,
                None => {
                    set_code_with_index_other(&mut vec, i, ErrorCode::TaskNotFound);
                    continue;
                }
            };

            if !check_current_account(task_uid) {
                set_code_with_index_other(&mut vec, i, ErrorCode::TaskNotFound);
                continue;
            }

            if (task_uid != ipc_uid) && !permission {
                set_code_with_index_other(&mut vec, i, ErrorCode::TaskNotFound);
                error!(
                    "Service show, failed: check task uid. tid: {}, uid: {}",
                    task_id, ipc_uid
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A18,
                    &format!(
                        "Service show, failed: check task uid. tid: {}, uid: {}",
                        task_id, ipc_uid
                    )
                );
                continue;
            }

            let info = self.task_manager.lock().unwrap().show(task_uid, task_id);
            match info {
                Some(task_info) => {
                    if let Some((c, info)) = vec.get_mut(i) {
                        *c = ErrorCode::ErrOk;
                        *info = task_info;
                    }
                }
                None => {
                    error!("Service show, failed: task_id not found, tid: {}", task_id);
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A18,
                        &format!("Service show, failed: task_id not found, tid: {}", task_id)
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
