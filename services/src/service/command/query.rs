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

use ipc_rust::{BorrowedMsgParcel, IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::manage::events::EventMessage;
use crate::service::ability::RequestAbility;
use crate::service::permission::{PermissionChecker, QueryPermission};
use crate::service::{is_system_api, serialize_task_info};
use crate::task::config::Action;

pub(crate) struct Query;

impl Query {
    pub(crate) fn execute(
        data: &BorrowedMsgParcel,
        reply: &mut BorrowedMsgParcel,
    ) -> IpcResult<()> {
        info!("Service query");
        if !is_system_api() {
            error!("Service query: not system api");
            reply.write(&(ErrorCode::SystemApi as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        let permission = PermissionChecker::check_query();
        let action = match permission {
            QueryPermission::NoPermission => {
                error!("Service query: no QUERY permission");
                reply.write(&(ErrorCode::Permission as i32))?;
                return Err(IpcStatusCode::Failed);
            }
            QueryPermission::QueryDownLoad => Action::Download,
            QueryPermission::QueryUpload => Action::Upload,
            QueryPermission::QueryAll => Action::Any,
        };

        let id: String = data.read()?;
        debug!("Service query: task_id is {}", id);
        match id.parse::<u32>() {
            Ok(id) => {
                debug!("Service query: u32 task_id is {}", id);
                let (event, rx) = EventMessage::query(id, action);
                if !RequestAbility::task_manager().send_event(event) {
                    return Err(IpcStatusCode::Failed);
                }
                match rx.get() {
                    Some(Some(info)) => {
                        reply.write(&(ErrorCode::ErrOk as i32))?;
                        debug!("Service query: task_info - {:?}", info);
                        serialize_task_info(info, reply)?;
                        Ok(())
                    }
                    Some(None) => {
                        error!("Service query: task_id not found");
                        reply.write(&(ErrorCode::TaskNotFound as i32))?;
                        Err(IpcStatusCode::Failed)
                    }
                    None => {
                        error!("Service query: receives task_info failed");
                        Err(IpcStatusCode::Failed)
                    }
                }
            }
            _ => {
                error!("Service query: task_id not valid");
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
