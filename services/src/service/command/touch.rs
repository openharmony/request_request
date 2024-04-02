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

use ipc_rust::{get_calling_uid, BorrowedMsgParcel, IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::manage::events::EventMessage;
use crate::service::ability::RequestAbility;
use crate::service::serialize_task_info;

pub(crate) struct Touch;

impl Touch {
    pub(crate) fn execute(
        data: &BorrowedMsgParcel,
        reply: &mut BorrowedMsgParcel,
    ) -> IpcResult<()> {
        debug!("Service touch");
        let id: String = data.read()?;
        debug!("Service touch: task_id is {}", id);
        match id.parse::<u32>() {
            Ok(id) => {
                debug!("Service touch: u32 task_id is {}", id);
                let token: String = data.read()?;
                debug!("Service touch: token is {}", token);
                let uid = get_calling_uid();
                debug!("Service touch: uid is {}", uid);
                let (event, rx) = EventMessage::touch(uid, id, token);
                if !RequestAbility::task_manager().send_event(event) {
                    return Err(IpcStatusCode::Failed);
                }
                match rx.get() {
                    Some(Some(info)) => {
                        reply.write(&(ErrorCode::ErrOk as i32))?;
                        debug!("Service touch: task_info get");
                        serialize_task_info(info, reply)?;
                        Ok(())
                    }
                    Some(None) => {
                        error!("Service touch: task_id or token not found");
                        reply.write(&(ErrorCode::TaskNotFound as i32))?;
                        Err(IpcStatusCode::Failed)
                    }
                    None => {
                        error!("Service touch: receives task_info failed");
                        Err(IpcStatusCode::Failed)
                    }
                }
            }
            _ => {
                error!("Service touch: task_id or token not valid");
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}