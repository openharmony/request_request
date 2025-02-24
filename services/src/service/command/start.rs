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
use crate::manage::database::RequestDb;
use crate::manage::events::TaskManagerEvent;
use crate::service::command::{set_code_with_index, CONTROL_MAX};
use crate::service::permission::PermissionChecker;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    pub(crate) fn start(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        info!("Service start");
        let permission = PermissionChecker::check_down_permission();
        if !PermissionChecker::check_internet() && !permission {
            error!("Service start: no INTERNET permission.");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A13,
                "Service start: no INTERNET permission."
            );
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let len: u32 = data.read()?;
        let len = len as usize;
        let mut vec = vec![ErrorCode::Other; len];

        if len > CONTROL_MAX {
            info!("Service start: out of size: {}", len);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let uid = ipc::Skeleton::calling_uid();

        for i in 0..len {
            let task_id: String = data.read()?;
            info!("Service start tid {}", task_id);
            let Ok(task_id) = task_id.parse::<u32>() else {
                error!("Service start, failed: tid not valid: {}", task_id);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A14,
                    &format!("Service start, failed: tid not valid: {}", task_id)
                );
                set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                continue;
            };

            let mut uid = uid;
            if permission {
                // skip uid check if task used by innerkits
                info!("{} start permission inner", task_id);
                match RequestDb::get_instance().query_task_uid(task_id) {
                    Some(id) => uid = id,
                    None => {
                        set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                        continue;
                    }
                };
            } else if !self.check_task_uid(task_id, uid) {
                set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                error!(
                    "Service start, failed: check task uid. tid: {}, uid: {}",
                    task_id, uid
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A14,
                    &format!(
                        "Service start, failed: check task uid. tid: {}, uid: {}",
                        task_id, uid
                    )
                );
                continue;
            }

            let (event, rx) = TaskManagerEvent::start(uid, task_id);
            if !self.task_manager.lock().unwrap().send_event(event) {
                error!("Service start, failed: task_manager err: {}", task_id);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A14,
                    &format!("Service start, failed: task_manager err: {}", task_id)
                );
                set_code_with_index(&mut vec, i, ErrorCode::Other);
                continue;
            }
            let ret = match rx.get() {
                Some(ret) => ret,
                None => {
                    error!(
                        "Service start, tid: {}, failed: receives ret failed",
                        task_id
                    );
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A14,
                        &format!(
                            "Service start, tid: {}, failed: receives ret failed",
                            task_id
                        )
                    );
                    set_code_with_index(&mut vec, i, ErrorCode::Other);
                    continue;
                }
            };
            set_code_with_index(&mut vec, i, ret);
            if ret != ErrorCode::ErrOk {
                error!("Service start, tid: {}, failed: {}", task_id, ret as i32);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A14,
                    &format!("Service start, tid: {}, failed: {}", task_id, ret as i32)
                );
            }
        }

        reply.write(&(ErrorCode::ErrOk as i32))?;
        for ret in vec {
            reply.write(&(ret as i32))?;
        }
        Ok(())
    }
}
