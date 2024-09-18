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
use crate::service::permission::{PermissionChecker, QueryPermission};
use crate::service::{serialize_task_info, RequestServiceStub};
use crate::task::config::Action;
use crate::utils::is_system_api;

impl RequestServiceStub {
    pub(crate) fn query(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
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

        let task_id: String = data.read()?;
        info!("Service query: tid: {}", task_id);

        let Ok(task_id) = task_id.parse::<u32>() else {
            error!(
                "End Service query, tid: {}, failed: task_id not valid",
                task_id
            );
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        };

        let info = self.task_manager.lock().unwrap().query(task_id, action);
        match info {
            Some(info) => {
                reply.write(&(ErrorCode::ErrOk as i32))?;
                serialize_task_info(info, reply)?;
                Ok(())
            }
            None => {
                error!(
                    "End Service query, failed: task_id not found, tid: {}",
                    task_id
                );
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
