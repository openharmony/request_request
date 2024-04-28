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
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    pub(crate) fn unsubscribe(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        let tid: String = data.read()?;
        info!("Process Service unsubscribe: task_id is {}", tid);
        match tid.parse::<u32>() {
            Ok(tid) => {
                if self.client_manager.unsubscribe(tid) == ErrorCode::ErrOk {
                    reply.write(&(ErrorCode::ErrOk as i32))?;
                    info!("End Service unsubscribe successfully: task_id is {}", tid);
                    Ok(())
                } else {
                    debug!("unsubscribe failed, task_id is {}", tid);
                    reply.write(&(ErrorCode::TaskNotFound as i32))?;
                    Err(IpcStatusCode::Failed)
                }
            }
            _ => {
                error!("End Service unsubscribe, failed with reason: task_id not valid");
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
