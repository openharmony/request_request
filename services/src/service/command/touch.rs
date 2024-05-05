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
use crate::manage::events::TaskManagerEvent;
use crate::service::{serialize_task_info, RequestServiceStub};

impl RequestServiceStub {
    pub(crate) fn touch(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        let id: String = data.read()?;
        info!("Process Service touch: task_id is {}", id);
        match id.parse::<u32>() {
            Ok(id) => {
                debug!("Service touch: u32 task_id is {}", id);
                let token: String = data.read()?;
                let uid = ipc::Skeleton::calling_uid();
                debug!("Service touch: uid is {}", uid);
                let (event, rx) = TaskManagerEvent::touch(uid, id, token);
                if !self.task_manager.send_event(event) {
                    return Err(IpcStatusCode::Failed);
                }
                match rx.get() {
                    Some(Some(info)) => {
                        reply.write(&(ErrorCode::ErrOk as i32))?;
                        serialize_task_info(info, reply)?;
                        info!("End Service touch successfully: task_id is {}", id);
                        Ok(())
                    }
                    Some(None) => {
                        error!("End Service touch, task_id is {}, failed with reason: task_id or token not found", id);
                        reply.write(&(ErrorCode::TaskNotFound as i32))?;
                        Err(IpcStatusCode::Failed)
                    }
                    None => {
                        error!("End Service touch, task_id is {}, failed with reason: receives task_info failed", id);
                        Err(IpcStatusCode::Failed)
                    }
                }
            }
            _ => {
                error!("End Service touch, failed with reason: task_id or token not valid");
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
