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
use crate::manage::query;
use crate::service::{serialize_task_config, RequestServiceStub};

impl RequestServiceStub {
    pub(crate) fn get_task(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        let tid: String = data.read()?;
        info!("Service getTask, tid: {}", tid);

        let tid = tid.parse::<u32>().map_err(|_e| {
            error!(
                "End Service getTask, tid: {}, failed: task_id or token not valid",
                tid
            );
            let _ = reply.write(&(ErrorCode::TaskNotFound as i32));
            IpcStatusCode::Failed
        })?;

        debug!("Service getTask: u32 tid: {}", tid);
        let token: String = data.read()?;
        let config = query::get_task(tid, token).ok_or_else(|| {
            error!(
                "End Service getTask, tid: {}, failed: task_id or token not found",
                tid
            );
            let _ = reply.write(&(ErrorCode::TaskNotFound as i32));
            IpcStatusCode::Failed
        })?;
        
        debug!("End Service getTask ok: tid: {}", tid);

        let uid = ipc::Skeleton::calling_uid();
        let token_id = ipc::Skeleton::calling_full_token_id();
        let pid = ipc::Skeleton::calling_pid();

        let ret = self.client_manager.subscribe(tid, pid, uid, token_id);
        if ret != ErrorCode::ErrOk {
            error!("End Service subscribe, tid: {}, failed: {:?}", tid, ret);
            reply.write(&(ret as i32))?;
            serialize_task_config(config, reply)?;
            return Ok(());
        }

        reply.write(&(ErrorCode::ErrOk as i32))?;
        debug!("End Service construct, succeed with tid: {}", tid);
        serialize_task_config(config, reply)?;
        Ok(())
    }
}
