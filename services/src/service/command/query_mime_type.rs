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
use crate::service::permission::PermissionChecker;

pub(crate) struct QueryMimeType;

impl QueryMimeType {
    pub(crate) fn execute(
        data: &BorrowedMsgParcel,
        reply: &mut BorrowedMsgParcel,
    ) -> IpcResult<()> {
        info!("Service query mime type");
        if !PermissionChecker::check_internet() {
            error!("Service query mime type: no INTERNET permission");
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        let id: String = data.read()?;
        debug!("Service query mime type: task_id is {}", id);
        match id.parse::<u32>() {
            Ok(id) => {
                debug!("Service query mime type: u32 task_id is {}", id);

                let uid = get_calling_uid();
                debug!("Service query mime type: uid is {}", uid);

                let (event, rx) = EventMessage::query_mime_type(uid, id);
                if !RequestAbility::task_manager().send_event(event) {
                    return Err(IpcStatusCode::Failed);
                }
                let mime = match rx.get() {
                    Some(mime) => mime,
                    None => {
                        error!("Service query mime type: receives mime failed");
                        return Err(IpcStatusCode::Failed);
                    }
                };
                debug!("Service query mime type: {}", mime);
                reply.write(&(ErrorCode::ErrOk as i32))?;
                reply.write(&mime)?;
                Ok(())
            }
            _ => {
                error!("Service query mime type: task_id not valid");
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
