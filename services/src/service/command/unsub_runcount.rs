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
use crate::service::runcount::RunCountEvent;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    pub(crate) fn unsub_runcount(&self, reply: &mut MsgParcel) -> IpcResult<()> {
        let pid = ipc::Skeleton::calling_pid();
        // let uid = ipc::Skeleton::calling_uid();
        info!("Process Service runcount unsubscribe: pid is {}", pid);

        let (event, rx) = RunCountEvent::unsub_runcount(pid);
        self.runcount_manager.send_event(event);

        let ret = match rx.get() {
            Some(ret) => ret,
            None => {
                error!("End Service runcount unsubscribe, failed with reason: receives ret failed");
                return Err(IpcStatusCode::Failed);
            }
        };
        reply.write(&(ret as i32))?;
        if ret != ErrorCode::ErrOk {
            error!(
                "End Service runcount unsubscribe, failed with reason: {}",
                ret as i32
            );
            return Err(IpcStatusCode::Failed);
        }
        info!("End Service runcount successfully: pid is {}", pid);
        Ok(())
    }
}
