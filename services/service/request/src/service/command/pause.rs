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
use crate::manager::events::EventMessage;
use crate::service::ability::RequestAbility;
use crate::service::permission::PermissionChecker;
use crate::task::config::Version;

pub(crate) struct Pause;

impl Pause {
    pub(crate) fn execute(
        data: &BorrowedMsgParcel,
        reply: &mut BorrowedMsgParcel,
    ) -> IpcResult<()> {
        info!("Service pause");
        let version: u32 = data.read()?;
        debug!("Service pause: version {}", version);
        if Version::from(version as u8) == Version::API9 && !PermissionChecker::check_internet() {
            error!("Service pause: no INTERNET permission");
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let id: String = data.read()?;
        debug!("Service pause: task_id is {}", id);
        match id.parse::<u32>() {
            Ok(id) => {
                debug!("Service pause: u32 task_id is {}", id);

                let uid = get_calling_uid();
                debug!("Service pause: uid is {}", uid);

                let (event, rx) = EventMessage::pause(uid, id);
                if !RequestAbility::task_manager().send_event(event) {
                    return Err(IpcStatusCode::Failed);
                }
                let ret = match rx.get() {
                    Some(ret) => ret,
                    None => {
                        error!("Service pause: receives ret failed");
                        return Err(IpcStatusCode::Failed);
                    }
                };
                reply.write(&(ret as i32))?;
                if ret != ErrorCode::ErrOk {
                    error!("Service pause: pause fail for ret is {}", ret as u32);
                    return Err(IpcStatusCode::Failed);
                }
                Ok(())
            }
            _ => {
                error!("Service pause: task_id not valid");
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}