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
use crate::task::files::check_current_account;

impl RequestServiceStub {
    pub(crate) fn resume(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        let permission = PermissionChecker::check_down_permission();
        if !PermissionChecker::check_internet() && !permission {
            error!("Service resume: no INTERNET permission");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A11,
                "Service resume: no INTERNET permission"
            );
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let len: u32 = data.read()?;
        let len = len as usize;
        let mut vec = vec![ErrorCode::Other; len];

        if len > CONTROL_MAX {
            info!("Service resume: out of size: {}", len);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        if len == 1 {
            self.resume_one_task(data, permission, &mut vec)?;
        } else {
            self.resume_batch_tasks(data, permission, &mut vec, len)?;
        }

        reply.write(&(ErrorCode::ErrOk as i32))?;
        for ret in vec {
            reply.write(&(ret as i32))?;
        }
        Ok(())
    }

    fn resume_one_task(
        &self,
        data: &mut MsgParcel,
        permission: bool,
        rets: &mut [ErrorCode],
    ) -> IpcResult<()> {
        let ipc_uid = ipc::Skeleton::calling_uid();
        let task_id: String = data.read()?;
        info!("Service resume tid {}", task_id);

        let Ok(task_id) = task_id.parse::<u32>() else {
            error!("Service resume, failed: tid not valid: {}", task_id);
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A12,
                &format!("Service resume, failed: tid not valid: {}", task_id)
            );
            set_code_with_index(rets, 0, ErrorCode::TaskNotFound);
            return Ok(());
        };

        let task_uid = match RequestDb::get_instance().query_task_uid(task_id) {
            Some(uid) => uid,
            None => {
                set_code_with_index(rets, 0, ErrorCode::TaskNotFound);
                return Ok(());
            }
        };

        if !check_current_account(task_uid) {
            set_code_with_index(rets, 0, ErrorCode::TaskNotFound);
            return Ok(());
        }

        if (task_uid != ipc_uid) && !permission {
            set_code_with_index(rets, 0, ErrorCode::TaskNotFound);
            error!(
                "Service resume, failed: check task uid. tid: {}, uid: {}",
                task_id, ipc_uid
            );
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A12,
                &format!(
                    "Service resume, failed: check task uid. tid: {}, uid: {}",
                    task_id, ipc_uid
                )
            );
            return Ok(());
        }

        let (event, rx) = TaskManagerEvent::resume(task_uid, task_id);
        if !self.task_manager.lock().unwrap().send_event(event) {
            error!("Service resume, failed: task_manager err: {}", task_id);
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A12,
                &format!("Service resume, failed: task_manager err: {}", task_id)
            );
            set_code_with_index(rets, 0, ErrorCode::Other);
            return Ok(());
        }

        let ret = match rx.get() {
            Some(ret) => ret,
            None => {
                error!(
                    "Service resume, tid: {}, failed: receives ret failed",
                    task_id
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A12,
                    &format!(
                        "Service resume, tid: {}, failed: receives ret failed",
                        task_id
                    )
                );
                set_code_with_index(rets, 0, ErrorCode::Other);
                return Ok(());
            }
        };
        set_code_with_index(rets, 0, ret);
        if ret != ErrorCode::ErrOk {
            error!("Service resume, tid: {}, failed: {}", task_id, ret as i32);
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A12,
                &format!("Service resume, tid: {}, failed: {}", task_id, ret as i32)
            );
        }
        Ok(())
    }

    fn resume_batch_tasks(
        &self,
        data: &mut MsgParcel,
        permission: bool,
        rets: &mut [ErrorCode],
        len: usize,
    ) -> IpcResult<()> {
        let ipc_uid: u64 = ipc::Skeleton::calling_uid();
        let mut tasks = Vec::with_capacity(len);
        for i in 0..len {
            let task_id: String = data.read()?;
            info!("Service resume tid {}", task_id);
            let Ok(task_id) = task_id.parse::<u32>() else {
                error!("Service resume, failed: tid not valid: {}", task_id);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A12,
                    &format!("Service resume, failed: tid not valid: {}", task_id)
                );
                set_code_with_index(rets, i, ErrorCode::TaskNotFound);
                continue;
            };

            let task_uid = match RequestDb::get_instance().query_task_uid(task_id) {
                Some(uid) => uid,
                None => {
                    set_code_with_index(rets, i, ErrorCode::TaskNotFound);
                    continue;
                }
            };

            if !check_current_account(task_uid) {
                set_code_with_index(rets, i, ErrorCode::TaskNotFound);
                continue;
            }

            if (task_uid != ipc_uid) && !permission {
                set_code_with_index(rets, i, ErrorCode::TaskNotFound);
                error!(
                    "Service resume, failed: check task uid. tid: {}, uid: {}",
                    task_id, ipc_uid
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A12,
                    &format!(
                        "Service resume, failed: check task uid. tid: {}, uid: {}",
                        task_id, ipc_uid
                    )
                );
                continue;
            }
            tasks.push((task_uid, task_id));
        }

        let (event, rx) = TaskManagerEvent::resume_batch(tasks.clone());
        if !self.task_manager.lock().unwrap().send_event(event) {
            error!("Service resume, failed: task_manager err: {:?}", tasks);
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A12,
                &format!("Service resume, failed: task_manager err: {:?}", tasks)
            );
            return Err(IpcStatusCode::Failed);
        }
        let Some(error_map) = rx.get() else {
            error!("Service resume, failed: receives ret failed");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A12,
                "Service resume, tid: failed: receives ret failed"
            );
            return Err(IpcStatusCode::Failed);
        };

        // The error code is ErrorCode::Other, indicating that the task has been
        // dispatched to the TaskManager, and it is necessary to retrieve the
        // execution result of the TaskManager from the error_map.
        let mut index = 0;
        for ret in rets.iter_mut() {
            if matches!(*ret, ErrorCode::Other) {
                let Some((_, task_id)) = tasks.get(index) else {
                    error!("Service resume, failed: bad tasks index");
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A12,
                        "Service resume, failed: bad tasks index"
                    );
                    break;
                };
                *ret = *error_map.get(task_id).unwrap_or(&ErrorCode::Other);
                if !matches!(*ret, ErrorCode::ErrOk) {
                    error!("Service resume, tid: {}, failed: {}", task_id, *ret as i32);
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A12,
                        &format!("Service resume, tid: {}, failed: {}", task_id, *ret as i32)
                    );
                }
                index += 1;
            }
        }
        Ok(())
    }
}
