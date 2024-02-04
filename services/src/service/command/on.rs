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

use ipc_rust::{BorrowedMsgParcel, IpcResult, IpcStatusCode, RemoteObj};

use crate::error::ErrorCode;
use crate::service::ability::RequestAbility;
use crate::service::notify::{Event, NotifyEvent};
use crate::service::permission::PermissionChecker;
use crate::task::config::Version;

pub(crate) struct On;

impl On {
    pub(crate) fn execute(
        data: &BorrowedMsgParcel,
        reply: &mut BorrowedMsgParcel,
    ) -> IpcResult<()> {
        info!("Service on");
        let version: u32 = data.read()?;
        if Version::from(version as u8) == Version::API9 && !PermissionChecker::check_internet() {
            error!("Service on: no INTERNET permission");
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        let on_type: String = data.read()?;
        debug!("Service on: on_type is {:?}", on_type);

        let event = match Event::try_from(on_type) {
            Ok(event) => event,
            Err(_) => {
                error!("Service on: on_type not valid");
                reply.write(&(ErrorCode::ParameterCheck as i32))?;
                return Err(IpcStatusCode::Failed);
            }
        };
        let id: String = data.read()?;
        debug!("Service on: task_id is {}", id);
        match id.parse::<u32>() {
            Ok(id) => {
                debug!("Service on: u32 task_id is {}", id);
                let obj: RemoteObj = data.read::<RemoteObj>()?;
                let (event, rx) = NotifyEvent::on(event, id, obj);
                RequestAbility::notify().send_event(event);

                let ret = match rx.get() {
                    Some(ret) => ret,
                    None => {
                        error!("Service on: receives ret failed");
                        return Err(IpcStatusCode::Failed);
                    }
                };
                reply.write(&(ret as i32))?;
                if ret != ErrorCode::ErrOk {
                    error!("Service on: on failed for ret is {}", ret as i32);
                    return Err(IpcStatusCode::Failed);
                }
                Ok(())
            }
            _ => {
                error!("Service on: task_id not valid");
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
