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
use crate::service::{serialize_task_config, RequestServiceStub};

impl RequestServiceStub {
    pub(crate) fn get_task(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        let task_id: String = data.read()?;
        info!("Service getTask tid {}", task_id);

        let Ok(task_id) = task_id.parse::<u32>() else {
            error!(
                "End Service getTask, tid: {}, failed: task_id or token not valid",
                task_id
            );
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        };

        let uid = ipc::Skeleton::calling_uid();

        if !self.check_task_uid(task_id, uid) {
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let token: String = data.read()?;
        let Some(config) = query::get_task(task_id, token) else {
            error!(
                "End Service getTask, tid: {}, failed: task_id or token not found",
                task_id
            );
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        };

        let token_id = ipc::Skeleton::calling_full_token_id();
        let pid = ipc::Skeleton::calling_pid();

        let ret = self.client_manager.subscribe(task_id, pid, uid, token_id);
        if ret != ErrorCode::ErrOk {
            error!("End Service subscribe, tid: {}, failed: {:?}", task_id, ret);
            reply.write(&(ret as i32))?;
            serialize_task_config(config, reply)?;
            return Ok(());
        }

        reply.write(&(ErrorCode::ErrOk as i32))?;
        serialize_task_config(config, reply)?;
        Ok(())
    }
}
