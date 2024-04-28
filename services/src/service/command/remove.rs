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
use crate::manage::events::TaskManagerEvent;
use crate::service::permission::PermissionChecker;
use crate::service::RequestServiceStub;
use crate::task::config::Version;

impl RequestServiceStub {
    pub(crate) fn remove(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        let version: u32 = data.read()?;
        if Version::from(version as u8) == Version::API9 && !PermissionChecker::check_internet() {
            error!("Service remove: no INTERNET permission");
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let id: String = data.read()?;
        info!("Process Service remove: task_id is {}", id);
        match id.parse::<u32>() {
            Ok(id) => {
                debug!("Service remove: u32 task_id is {}", id);
                let uid = ipc::Skeleton::calling_uid();
                debug!("Service remove: uid is {}", uid);
                let (event, rx) = TaskManagerEvent::remove(uid, id);
                if !self.task_manager.send_event(event) {
                    return Err(IpcStatusCode::Failed);
                }
                let ret = match rx.get() {
                    Some(ret) => ret,
                    None => {
                        error!("End Service remove, task_id is {}, failed with reason: receives ret failed", id);
                        return Err(IpcStatusCode::Failed);
                    }
                };
                reply.write(&(ret as i32))?;
                if ret != ErrorCode::ErrOk {
                    error!(
                        "End Service remove, task_id is {}, failed with reason: {}",
                        id, ret as i32
                    );
                    return Err(IpcStatusCode::Failed);
                }
                info!("End Service remove successfully, task_id is {}", id);
                Ok(())
            }
            _ => {
                error!(
                    "End Service remove, task_id is {}, failed with reason: task_id not valid",
                    id
                );
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
