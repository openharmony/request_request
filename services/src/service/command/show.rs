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
use crate::service::permission::PermissionChecker;
use crate::service::{serialize_task_info, RequestServiceStub};

impl RequestServiceStub {
    pub(crate) fn show(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        if !PermissionChecker::check_internet() {
            error!("Service show: no INTERNET permission");
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        let task_id: String = data.read()?;
        info!("Service show tid {}", task_id);

        let Ok(task_id) = task_id.parse::<u32>() else {
            error!("End Service show, failed: task_id not valid");
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

        let info = self.task_manager.lock().unwrap().show(uid, task_id);
        match info {
            Some(info) => {
                reply.write(&(ErrorCode::ErrOk as i32))?;
                serialize_task_info(info, reply)?;
                Ok(())
            }
            None => {
                error!(
                    "End Service show, failed: task_id not found, tid: {}",
                    task_id
                );
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
