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
use ipc::remote::RemoteObj;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    pub(crate) fn subscribe_run_count(
        &self,
        data: &mut MsgParcel,
        reply: &mut MsgParcel,
    ) -> IpcResult<()> {
        let pid = ipc::Skeleton::calling_pid();
        info!("Service run_count subscribe pid {}", pid);

        let obj: RemoteObj = data.read_remote()?;
        let ret = self.run_count_manager.subscribe_run_count(pid, obj);

        reply.write(&(ret as i32))?;
        if ret != ErrorCode::ErrOk {
            error!("End Service run_count subscribe, failed:{}", ret as i32);
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A32,
                &format!("End Service run_count subscribe, failed:{}", ret as i32)
            );
            return Err(IpcStatusCode::Failed);
        }
        Ok(())
    }
}
