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

use crate::config::Action;
use crate::error::ErrorCode;
use crate::manage::database::RequestDb;
use crate::manage::events::TaskManagerEvent;
use crate::service::notification_bar::NotificationDispatcher;
use crate::service::permission::{ManagerPermission, PermissionChecker};
use crate::service::RequestServiceStub;
use crate::utils::{check_permission, is_system_api};

impl RequestServiceStub {
    pub(crate) fn create_group(
        &self,
        data: &mut MsgParcel,
        reply: &mut MsgParcel,
    ) -> IpcResult<()> {
        let gauge: bool = data.read()?;

        let title = if data.read::<bool>()? {
            Some(data.read()?)
        } else {
            None
        };

        let text = if data.read::<bool>()? {
            Some(data.read()?)
        } else {
            None
        };

        let want_agent = if data.read::<bool>()? {
            Some(data.read()?)
        } else {
            None
        };

        let mut disable:bool = data.read()?;
        if disable && (!is_system_api() || !check_permission("ohos.permission.REQUEST_DISABLE_NOTIFICATION")){
            disable = false;
        }

        let visibility = data.read()?;

        let new_group_id = NotificationDispatcher::get_instance().create_group(
            gauge, title, text, want_agent, disable, visibility);
        reply.write(&new_group_id.to_string())?;
        Ok(())
    }

    pub(crate) fn attach_group(
        &self,
        data: &mut MsgParcel,
        reply: &mut MsgParcel,
    ) -> IpcResult<()> {
        let Ok(group_id) = data.read::<String>()?.parse::<u32>() else {
            error!("End Service attach_group, group_id, failed: group_id not valid",);
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A38,
                "End Service attach_group, group_id, failed: group_id not valid"
            );
            reply.write(&(ErrorCode::GroupNotFound as i32))?;
            return Ok(());
        };
        let task_ids = data.read::<Vec<String>>()?;

        let uid = ipc::Skeleton::calling_uid();

        let mut parse_ids = Vec::with_capacity(task_ids.len());

        for task_id in task_ids.iter() {
            let Ok(task_id) = task_id.parse::<u32>() else {
                error!("End Service attach_group, task_id, failed: task_id not valid");
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A38,
                    "End Service attach_group, task_id, failed: task_id not valid"
                );
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                return Ok(());
            };
            if !self.check_task_uid(task_id, uid) {
                error!(
                    "End Service attach_group, task_id: {}, failed: task_id not belong to uid",
                    task_id
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A38,
                    &format!("End Service attach_group, task_id: {}, failed: task_id not belong to uid", task_id)
                );
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                return Ok(());
            }
            parse_ids.push(task_id);
        }
        let (event, rx) = TaskManagerEvent::attach_group(uid, parse_ids, group_id);
        if !self.task_manager.lock().unwrap().send_event(event) {
            return Err(IpcStatusCode::Failed);
        }

        let ret = match rx.get() {
            Some(ret) => ret,
            None => {
                error!(
                    "End Service attach_group, task_id: {:?}, group_id: {}, failed: receives ret failed",
                    task_ids, group_id
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A38, 
                    &format!("End Service attach_group, task_id: {:?}, group_id: {}, failed: receives ret failed",task_ids, group_id)
                );
                ErrorCode::Other
            }
        };
        if ret != ErrorCode::ErrOk {
            error!(
                "End Service attach_group, task_id: {:?}, group_id: {}, failed: ret is not ErrOk",
                task_ids, group_id
            );
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A38,
                &format!("End Service attach_group, task_id: {:?}, group_id: {}, failed: ret is not ErrOk",task_ids, group_id)
            );
        }
        reply.write(&(ret as i32))?;
        Ok(())
    }

    pub(crate) fn delete_group(
        &self,
        data: &mut MsgParcel,
        reply: &mut MsgParcel,
    ) -> IpcResult<()> {
        let Ok(group_id) = data.read::<String>()?.parse::<u32>() else {
            reply.write(&(ErrorCode::GroupNotFound as i32))?;
            return Ok(());
        };
        let mut ret = ErrorCode::ErrOk;
        let uid = ipc::Skeleton::calling_uid();
        if !NotificationDispatcher::get_instance().delete_group(group_id, uid) {
            ret = ErrorCode::GroupNotFound;
        }
        reply.write(&(ret as i32))?;
        Ok(())
    }

    pub(crate) fn disable_task_notifications(
        &self,
        data: &mut MsgParcel,
        reply: &mut MsgParcel,
    ) -> IpcResult<()> {
        let mut permission = None;
        let task_ids = data.read::<Vec<String>>()?;
        let calling_uid = ipc::Skeleton::calling_uid();

        for task_id in task_ids.iter() {
            match self.disable_task_notification_inner(calling_uid, task_id, &mut permission) {
                Ok(()) => reply.write(&(ErrorCode::ErrOk as i32)),
                Err(e) => {
                    error!("End Service disable_task_notifications, failed: {:?}", e);
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A46,
                        &format!("End Service disable_task_notifications, failed: {:?}", e)
                    );
                    reply.write(&(e as i32))
                }
            }?;
        }
        Ok(())
    }

    fn disable_task_notification_inner(
        &self,
        calling_uid: u64,
        task_id: &str,
        permission: &mut Option<ManagerPermission>,
    ) -> Result<(), ErrorCode> {
        let Ok(task_id) = task_id.parse::<u32>() else {
            return Err(ErrorCode::TaskNotFound);
        };
        let Some(task_uid) = RequestDb::get_instance().query_task_uid(task_id) else {
            return Err(ErrorCode::TaskNotFound);
        };
        if task_uid != calling_uid {
            let permission = match permission {
                Some(permission) => *permission,
                None => {
                    *permission = Some(PermissionChecker::check_manager());
                    permission.unwrap()
                }
            };
            match permission {
                ManagerPermission::ManagerAll => {}
                ManagerPermission::ManagerDownLoad => {
                    let Some(action) = RequestDb::get_instance().query_task_action(task_id) else {
                        return Err(ErrorCode::TaskNotFound);
                    };
                    if action != Action::Download {
                        return Err(ErrorCode::Permission);
                    }
                }
                ManagerPermission::ManagerUpload => {
                    let Some(action) = RequestDb::get_instance().query_task_action(task_id) else {
                        return Err(ErrorCode::TaskNotFound);
                    };
                    if action != Action::Upload {
                        return Err(ErrorCode::Permission);
                    }
                }
                ManagerPermission::NoPermission => {
                    return Err(ErrorCode::Permission);
                }
            }
        }
        NotificationDispatcher::get_instance().disable_task_notification(task_uid, task_id);
        Ok(())
    }
}
