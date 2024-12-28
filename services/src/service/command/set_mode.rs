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

use crate::config::Mode;
use crate::error::ErrorCode;
use crate::manage::database::RequestDb;
use crate::manage::events::TaskManagerEvent;
use crate::service::permission::PermissionChecker;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    pub(crate) fn set_mode(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        let permission = PermissionChecker::check_down_permission();
        if !permission {
            error!("Service change_mode: no DOWNLOAD_SESSION_MANAGER permission.");
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let task_id: String = data.read()?;
        info!("Service change_mode tid {}", task_id);
        let Ok(task_id) = task_id.parse::<u32>() else {
            error!("Service change_mode, failed: tid not valid: {}", task_id);
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        };

        let mode: u32 = data.read()?;
        let mode = Mode::from(mode as u8);

        let old_mode = match RequestDb::get_instance().query_task_mode(task_id) {
            Some(m) => m,
            None => {
                error!(
                    "Service change_mode, failed: old_mode not valid: {}",
                    task_id
                );
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                return Err(IpcStatusCode::Failed);
            }
        };

        if old_mode == mode || mode == Mode::Any {
            error!("Service change_mode, failed: mod state error: {}", task_id);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let uid = match RequestDb::get_instance().query_task_uid(task_id) {
            Some(id) => id,
            None => {
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                return Err(IpcStatusCode::Failed);
            }
        };

        let (event, rx) = TaskManagerEvent::set_mode(uid, task_id, mode);
        if !self.task_manager.lock().unwrap().send_event(event) {
            error!("Service change_mode, failed: task_manager err: {}", task_id);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        let ret = match rx.get() {
            Some(ret) => ret,
            None => {
                error!(
                    "Service change_mode, tid: {}, failed: receives ret failed",
                    task_id
                );
                reply.write(&(ErrorCode::Other as i32))?;
                return Err(IpcStatusCode::Failed);
            }
        };
        reply.write(&(ret as i32))?;
        Ok(())
    }
}
