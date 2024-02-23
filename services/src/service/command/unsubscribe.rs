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

use ipc_rust::{BorrowedMsgParcel, IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::service::ability::RequestAbility;

pub(crate) struct Unsubscribe;

impl Unsubscribe {
    pub(crate) fn execute(
        data: &BorrowedMsgParcel,
        reply: &mut BorrowedMsgParcel,
    ) -> IpcResult<()> {
        info!("subscribe");
        let tid: String = data.read()?;
        debug!("Service unsubscribe: task_id is {}", tid);
        match tid.parse::<u32>() {
            Ok(tid) => {
                if RequestAbility::client_manager().unsubscribe(tid) == ErrorCode::ErrOk {
                    reply.write(&(ErrorCode::ErrOk as i32))?;
                    Ok(())
                } else {
                    error!("unsubscribe failed");
                    reply.write(&(ErrorCode::TaskNotFound as i32))?; // 错误码待统一处理
                    Err(IpcStatusCode::Failed)
                }
            }
            _ => {
                error!("Service unsubscribe: task_id not valid");
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
