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
use crate::service::permission::PermissionChecker;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    pub(crate) fn set_max_speed(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        if !PermissionChecker::check_internet() {
            error!("Service resume: no INTERNET permission");
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let task_id: String = data.read()?;
        let max_speed: i64 = data.read()?;
        let Ok(task_id) = task_id.parse::<u32>() else {
            error!("Service set_max_speed, failed: tid not valid: {}", task_id);
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        };

        let uid = ipc::Skeleton::calling_uid();

        let (event, rx) = TaskManagerEvent::set_max_speed(uid, task_id, max_speed);
        if !self.task_manager.lock().unwrap().send_event(event) {
            error!("Service set_max_speed, failed: task_manager err: {}", task_id);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let Some(ret) = rx.get() else {
            error!("Service set_max_speed, tid: {}, failed: receives ret failed", task_id);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        };

        reply.write(&(ret as i32))?;
        Ok(())
    }
}
