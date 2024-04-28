// Copyright (C) 2024 Huawei Device Co., Ltd.
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
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    pub(crate) fn subscribe(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        let tid: String = data.read()?;
        info!("Process Service subscribe: task_id is {}", tid);
        let pid = ipc::Skeleton::calling_pid();
        let uid = ipc::Skeleton::calling_uid();
        let token_id = ipc::Skeleton::calling_full_token_id();
        match tid.parse::<u32>() {
            Ok(tid) => {
                let (event, rx) = TaskManagerEvent::subscribe(tid, token_id);
                if !self.task_manager.send_event(event) {
                    reply.write(&(ErrorCode::Other as i32))?;
                    error!(
                    "End Service subscribe, task_id is {}, failed with reason: send event failed",
                    tid
                );
                    return Err(IpcStatusCode::Failed);
                }
                let ret = match rx.get() {
                    Some(ret) => ret,
                    None => {
                        error!("End Service subscribe, task_id is {}, failed with reason: receives ret failed", tid);
                        reply.write(&(ErrorCode::Other as i32))?;
                        return Err(IpcStatusCode::Failed);
                    }
                };

                if ret != ErrorCode::ErrOk {
                    error!(
                        "End Service subscribe, task_id is {}, failed with reason: {:?}",
                        tid, ret
                    );
                    reply.write(&(ret as i32))?;
                    return Err(IpcStatusCode::Failed);
                }

                let ret = self.client_manager.subscribe(tid, pid, uid, token_id);
                if ret == ErrorCode::ErrOk {
                    reply.write(&(ErrorCode::ErrOk as i32))?;
                    info!("End Service subscribe successfully: task_id is {}", tid);
                    Ok(())
                } else {
                    error!(
                        "End Service subscribe, task_id is {}, failed with reason: {:?}",
                        tid, ret
                    );
                    reply.write(&(ret as i32))?;
                    Err(IpcStatusCode::Failed)
                }
            }
            _ => {
                error!("End Service subscribe, failed with reason: task_id not valid");
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
