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
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    pub(crate) fn attach_group(
        &self,
        data: &mut MsgParcel,
        reply: &mut MsgParcel,
    ) -> IpcResult<()> {
        let Ok(group_id) = data.read::<String>()?.parse::<u32>() else {
            error!("End Service attach_group, group_id, failed: group_id not valid",);
            return Ok(());
        };
        let task_ids = data.read::<Vec<String>>()?;
        let mut ret = ErrorCode::ErrOk;
        for task_id in task_ids {
            let Ok(task_id) = task_id.parse::<u32>() else {
                error!("End Service attach_group, task_id, failed: task_id not valid");
                ret = ErrorCode::TaskNotFound;
                break;
            };

            let (event, rx) = TaskManagerEvent::attach_group(task_id, group_id);
            if !self.task_manager.lock().unwrap().send_event(event) {
                return Err(IpcStatusCode::Failed);
            }

            ret = match rx.get() {
                Some(ret) => ret,
                None => {
                    error!(
                    "End Service attach_group, task_id: {}, group_id: {}, failed: receives ret failed",
                    task_id, group_id
                );
                    ErrorCode::TaskNotFound
                }
            };
            if ret != ErrorCode::ErrOk {
                error!(
                    "End Service attach_group, task_id: {}, group_id: {}, failed: ret is not ErrOk",
                    task_id, group_id
                );
                break;
            }
        }
        reply.write(&(ret as i32))?;
        Ok(())
    }
}
