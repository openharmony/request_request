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

use ipc_rust::{
    get_calling_pid, get_calling_token_id, get_calling_uid, BorrowedMsgParcel, IpcResult,
    IpcStatusCode,
};

use crate::error::ErrorCode;
use crate::manage::events::EventMessage;
use crate::service::ability::RequestAbility;

pub(crate) struct Subscribe;

impl Subscribe {
    pub(crate) fn execute(
        data: &BorrowedMsgParcel,
        reply: &mut BorrowedMsgParcel,
    ) -> IpcResult<()> {
        info!("subscribe");
        let tid: String = data.read()?;
        debug!("Service subscribe: task_id is {}", tid);
        let pid = get_calling_pid();
        let uid = get_calling_uid();
        let token_id = get_calling_token_id();
        match tid.parse::<u32>() {
            Ok(tid) => {
                let (event, rx) = EventMessage::subscribe(tid, token_id);
                if !RequestAbility::task_manager().send_event(event) {
                    reply.write(&(ErrorCode::Other as i32))?;
                    error!("send event failed");
                    return Err(IpcStatusCode::Failed);
                }
                let ret = match rx.get() {
                    Some(ret) => ret,
                    None => {
                        error!("Service construct: receives ret failed");
                        reply.write(&(ErrorCode::Other as i32))?;
                        return Err(IpcStatusCode::Failed);
                    }
                };

                if ret != ErrorCode::ErrOk {
                    error!("subscribe failed: {:?}", ret);
                    reply.write(&(ret as i32))?;
                    return Err(IpcStatusCode::Failed);
                }

                let ret = RequestAbility::client_manager().subscribe(tid, pid, uid, token_id);
                if ret == ErrorCode::ErrOk {
                    reply.write(&(ErrorCode::ErrOk as i32))?;
                    info!("subscribe ok");
                    Ok(())
                } else {
                    error!("subscribe failed");
                    reply.write(&(ret as i32))?;
                    Err(IpcStatusCode::Failed)
                }
            }
            _ => {
                error!("Service subscribe: task_id not valid");
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
