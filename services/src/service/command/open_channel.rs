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

use std::fs::File;
use std::os::unix::io::FromRawFd;

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    pub(crate) fn open_channel(&self, reply: &mut MsgParcel) -> IpcResult<()> {
        let pid = ipc::Skeleton::calling_pid();
        info!("Process Service open channel, pid: {}", pid);
        let uid = ipc::Skeleton::calling_uid();
        let token_id = ipc::Skeleton::calling_full_token_id();
        match self.client_manager.open_channel(pid, uid, token_id) {
            Ok(fd) => {
                info!("End Service open channel successfully, fd is {}", fd);
                let file = unsafe { File::from_raw_fd(fd) };
                reply.write(&(ErrorCode::ErrOk as i32))?;
                reply.write_file(file)?;
                Ok(())
            }
            Err(err) => {
                error!("End Service open channel, failed with reason: {:?}", err);
                reply.write(&(ErrorCode::ParameterCheck as i32))?;
                Err(IpcStatusCode::Failed)
            }
        }
    }
}
