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
use crate::task::config::Version;

pub(crate) struct Remove;

impl Remove {
    pub(crate) fn execute(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        info!("Service remove");
        let version: u32 = data.read()?;
        if Version::from(version as u8) == Version::API9 && !PermissionChecker::check_internet() {
            error!("Service remove: no INTERNET permission");
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let id: String = data.read()?;
        debug!("Service remove: task_id is {}", id);
        match id.parse::<u32>() {
            Ok(id) => {
                debug!("Service remove: u32 task_id is {}", id);
                let uid = ipc::Skeleton::calling_uid();
                debug!("Service remove: uid is {}", uid);
                let (event, rx) = EventMessage::remove(uid, id);
                if !RequestAbility::task_manager().send_event(event) {
                    return Err(IpcStatusCode::Failed);
                }
                let ret = match rx.get() {
                    Some(ret) => ret,
                    None => {
                        error!("Service remove: receives ret failed");
                        return Err(IpcStatusCode::Failed);
                    }
                };
                reply.write(&(ret as i32))?;
                if ret != ErrorCode::ErrOk {
                    error!("Service remove: remove failed for ret is {}", ret as i32);
                    return Err(IpcStatusCode::Failed);
                }
                Ok(())
            }
            _ => {
                error!("Service remove: task_id not valid");
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
