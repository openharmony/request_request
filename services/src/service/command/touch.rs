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
use crate::service::{serialize_task_info, RequestServiceStub};

impl RequestServiceStub {
    pub(crate) fn touch(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        let task_id: String = data.read()?;
        info!("Service touch: tid: {}", task_id);

        let Ok(task_id) = task_id.parse::<u32>() else {
            error!("End Service touch, failed: task_id or token not valid");
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        };

        let token: String = data.read()?;
        let uid = ipc::Skeleton::calling_uid();

        if !self.check_task_uid(task_id, uid) {
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        let info = self.task_manager.lock().unwrap().touch(uid, task_id, token);

        match info {
            Some(info) => {
                reply.write(&(ErrorCode::ErrOk as i32))?;
                serialize_task_info(info, reply)?;
                Ok(())
            }
            None => {
                error!(
                    "End Service touch, tid: {}, failed: task_id or token not found",
                    task_id
                );
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
