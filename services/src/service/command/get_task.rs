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
use crate::manage::events::EventMessage;
use crate::service::ability::RequestAbility;
use crate::service::serialize_task_config;

pub(crate) struct GetTask;

impl GetTask {
    pub(crate) fn execute(
        data: &BorrowedMsgParcel,
        reply: &mut BorrowedMsgParcel,
    ) -> IpcResult<()> {
        debug!("Service getTask");
        let tid: String = data.read()?;
        debug!("Service getTask: task_id is {}", tid);
        match tid.parse::<u32>() {
            Ok(tid) => {
                debug!("Service getTask: u32 task_id is {}", tid);
                let token: String = data.read()?;
                debug!("Service getTask: token is {}", token);
                let uid = get_calling_uid();
                debug!("Service getTask: uid is {}", uid);
                let (event, rx) = EventMessage::get_task(uid, tid, token);
                if !RequestAbility::task_manager().send_event(event) {
                    return Err(IpcStatusCode::Failed);
                }
                match rx.get() {
                    Some(Some(config)) => {
                        reply.write(&(ErrorCode::ErrOk as i32))?;
                        debug!("Service getTask: task_config get");
                        serialize_task_config(config, reply)?;
                        Ok(())
                    }
                    Some(None) => {
                        error!("Service getTask: task_id or token not found");
                        reply.write(&(ErrorCode::TaskNotFound as i32))?;
                        Err(IpcStatusCode::Failed)
                    }
                    None => {
                        error!("Service getTask: receives task_config failed");
                        Err(IpcStatusCode::Failed)
                    }
                }
            }
            _ => {
                error!("Service getTask: task_id or token not valid");
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
