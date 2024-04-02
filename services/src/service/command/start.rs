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
use crate::manage::events::EventMessage;
use crate::service::ability::RequestAbility;
use crate::service::permission::PermissionChecker;

pub(crate) struct Start;

impl Start {
    pub(crate) fn execute(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        info!("Service start");
        if !PermissionChecker::check_internet() {
            error!("Service start: no INTERNET permission.");
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let id: String = data.read()?;
        debug!("Service start: task_id is {}", id);
        match id.parse::<u32>() {
            Ok(id) => {
                debug!("Service start: u32 task_id is {}", id);
                let uid = ipc::Skeleton::calling_uid();
                debug!("Service start: uid is {}", uid);

                let (event, rx) = EventMessage::start(uid, id);
                if !RequestAbility::task_manager().send_event(event) {
                    return Err(IpcStatusCode::Failed);
                }
                let ret = match rx.get() {
                    Some(ret) => ret,
                    None => {
                        error!("Service start: receives ret failed");
                        return Err(IpcStatusCode::Failed);
                    }
                };
                reply.write(&(ret as i32))?;
                if ret != ErrorCode::ErrOk {
                    error!("Service start: start failed for ret is {}", ret as i32);
                    return Err(IpcStatusCode::Failed);
                }
                Ok(())
            }
            _ => {
                error!("Service start: task_id not valid");
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
