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
use crate::manage::events::TaskManagerEvent;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    pub(crate) fn subscribe(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        let task_id: String = data.read()?;
        info!("Service subscribe: tid: {}", task_id);

        let Ok(task_id) = task_id.parse::<u32>() else {
            error!("End Service subscribe, failed: task_id not valid");
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        };
        let uid = ipc::Skeleton::calling_uid();

        if !self.check_task_uid(task_id, uid) {
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let pid = ipc::Skeleton::calling_pid();
        let token_id = ipc::Skeleton::calling_full_token_id();

        let (event, rx) = TaskManagerEvent::subscribe(task_id, token_id);
        if !self.task_manager.lock().unwrap().send_event(event) {
            reply.write(&(ErrorCode::Other as i32))?;
            error!(
                "End Service subscribe, tid: {}, failed: send event failed",
                task_id
            );
            return Err(IpcStatusCode::Failed);
        }
        let ret = match rx.get() {
            Some(ret) => ret,
            None => {
                error!(
                    "End Service subscribe, tid: {}, failed: receives ret failed",
                    task_id
                );
                reply.write(&(ErrorCode::Other as i32))?;
                return Err(IpcStatusCode::Failed);
            }
        };

        if ret != ErrorCode::ErrOk {
            error!("End Service subscribe, tid: {}, failed: {:?}", task_id, ret);
            reply.write(&(ret as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let ret = self.client_manager.subscribe(task_id, pid, uid, token_id);
        if ret == ErrorCode::ErrOk {
            reply.write(&(ErrorCode::ErrOk as i32))?;
            debug!("End Service subscribe ok: tid: {}", task_id);
            Ok(())
        } else {
            error!("End Service subscribe, tid: {}, failed: {:?}", task_id, ret);
            reply.write(&(ret as i32))?;
            Err(IpcStatusCode::Failed)
        }
    }
}
