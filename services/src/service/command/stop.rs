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
use crate::manage::database::RequestDb;
use crate::manage::events::TaskManagerEvent;
use crate::service::permission::PermissionChecker;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    pub(crate) fn stop(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        let task_id: String = data.read()?;
        info!("Service stop tid {}", task_id);

        let Ok(task_id) = task_id.parse::<u32>() else {
            error!("End Service stop, failed: task_id not valid");
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        };

        let mut uid = ipc::Skeleton::calling_uid();
        if PermissionChecker::check_down_permission() {
            //skip uid check if task used by innerkits
            info!("task permission inner");
            match RequestDb::get_instance().query_task_uid(task_id) {
                Some(res) => uid = res ,
                None => {
                    reply.write(&(ErrorCode::TaskNotFound as i32))?;
                    return Err(IpcStatusCode::Failed);
                },
            };
        } else if !self.check_task_uid(task_id, uid) {
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        let (event, rx) = TaskManagerEvent::stop(uid, task_id);
        if !self.task_manager.lock().unwrap().send_event(event) {
            return Err(IpcStatusCode::Failed);
        }
        let ret = match rx.get() {
            Some(ret) => ret,
            None => {
                error!(
                    "End Service stop, tid: {}, failed: receives ret failed",
                    task_id
                );
                return Err(IpcStatusCode::Failed);
            }
        };
        reply.write(&(ret as i32))?;
        if ret != ErrorCode::ErrOk {
            error!("End Service stop, tid: {}, failed: {}", task_id, ret as i32);
            return Err(IpcStatusCode::Failed);
        }
        Ok(())
    }
}
