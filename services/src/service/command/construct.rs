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

use crate::config::Mode;
use crate::error::ErrorCode;
use crate::manage::events::TaskManagerEvent;
use crate::service::command::{set_code_with_index_other, CONSTRUCT_MAX};
use crate::service::notification_bar::NotificationDispatcher;
use crate::service::permission::PermissionChecker;
use crate::service::RequestServiceStub;
use crate::task::config::TaskConfig;
use crate::utils::{check_permission, is_system_api};

impl RequestServiceStub {
    pub(crate) fn construct(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        info!("Service construct");
        let permission = PermissionChecker::check_down_permission();
        if !PermissionChecker::check_internet() && !permission {
            error!("Service start: no INTERNET permission.");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A01,
                "Service start: no INTERNET permission."
            );
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        let len: u32 = data.read()?;
        let len = len as usize;

        if len > CONSTRUCT_MAX {
            info!("Service construct: out of size: {}", len);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let uid = ipc::Skeleton::calling_uid();
        let token_id = ipc::Skeleton::calling_full_token_id();
        let pid = ipc::Skeleton::calling_pid();
        let mut vec = vec![(ErrorCode::Other, 0u32); len];

        for i in 0..len {
            let task_config: TaskConfig = match data.read() {
                Ok(config) => config,
                Err(_e) => {
                    set_code_with_index_other(&mut vec, i, ErrorCode::IpcSizeTooLarge);
                    continue;
                }
            };
            debug!("Service construct: task_config constructed");
            let mode = task_config.common_data.mode;
            let (event, rx) = TaskManagerEvent::construct(task_config);
            if !self.task_manager.lock().unwrap().send_event(event) {
                set_code_with_index_other(&mut vec, i, ErrorCode::Other);
                continue;
            }
            let ret = match rx.get() {
                Some(ret) => ret,
                None => {
                    error!("End Service construct, failed: receives ret failed");
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A02,
                        "End Service construct, failed: receives ret failed"
                    );
                    set_code_with_index_other(&mut vec, i, ErrorCode::Other);
                    continue;
                }
            };

            let task_id = match ret {
                Ok(id) => id,
                Err(err_code) => {
                    error!("End Service construct, failed: {:?}", err_code);
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A02,
                        &format!("End Service construct, failed: {:?}", err_code)
                    );
                    set_code_with_index_other(&mut vec, i, err_code);
                    continue;
                }
            };

            let title = if data.read::<bool>()? {
                Some(data.read::<String>()?)
            } else {
                None
            };

            let text = if data.read::<bool>()? {
                Some(data.read::<String>()?)
            } else {
                None
            };
            if title.is_some() || text.is_some() {
                NotificationDispatcher::get_instance()
                    .update_task_customized_notification(task_id, title, text);
            }
            let notification_disable = data.read::<bool>()? && is_system_api();
            if notification_disable {
                if !check_permission("ohos.permission.REQUEST_DISABLE_NOTIFICATION") {
                    if let Some((c, tid)) = vec.get_mut(i) {
                        *c = ErrorCode::Permission;
                        *tid = task_id;
                    }
                    continue;
                }
                if matches!(mode, Mode::BackGround) {
                    NotificationDispatcher::get_instance().disable_task_notification(uid, task_id);
                }
            }

            debug!("Service construct: construct event sent to manager");

            let ret = self.client_manager.subscribe(task_id, pid, uid, token_id);
            if ret != ErrorCode::ErrOk {
                error!("End Service subscribe, tid: {}, failed: {:?}", task_id, ret);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A02,
                    &format!("End Service subscribe, tid: {}, failed: {:?}", task_id, ret)
                );
            }
            if let Some((c, tid)) = vec.get_mut(i) {
                *c = ret;
                *tid = task_id;
            }
            debug!("End Service construct, succeed with tid: {}", task_id);
        }
        reply.write(&(ErrorCode::ErrOk as i32))?;
        for (c, tid) in vec {
            reply.write(&(c as i32))?;
            reply.write(&tid)?;
        }
        Ok(())
    }
}
