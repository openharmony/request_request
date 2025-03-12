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
use crate::manage::query;
use crate::service::permission::PermissionChecker;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    pub(crate) fn query_mime_type(
        &self,
        data: &mut MsgParcel,
        reply: &mut MsgParcel,
    ) -> IpcResult<()> {
        let permission = PermissionChecker::check_down_permission();
        if !PermissionChecker::check_internet() && !permission {
            error!("Service query mime type: no INTERNET permission");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A07,
                "Service query mime type: no INTERNET permission"
            );
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        let task_id: String = data.read()?;
        info!("Service query mime type tid {}", task_id);

        let Ok(task_id) = task_id.parse::<u32>() else {
            error!("End Service query mime type, failed: task_id not valid");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A08,
                "End Service query mime type, failed: task_id not valid"
            );
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        };

        let uid = ipc::Skeleton::calling_uid();
        if !self.check_task_uid(task_id, uid) {
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let mime = query::query_mime_type(uid, task_id);

        reply.write(&(ErrorCode::ErrOk as i32))?;
        reply.write(&mime)?;
        Ok(())
    }
}
