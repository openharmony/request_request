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
use crate::service::ability::RequestAbility;
use crate::service::runcount::RunCountEvent;

pub(crate) struct SubRunCount;

impl SubRunCount {
    pub(crate) fn execute(data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        info!("Service runcount subscribe");
        let pid = ipc::Skeleton::calling_pid();
        debug!("Service runcount subscribe: pid is {}", pid);

        let obj: RemoteObj = data.read_remote()?;
        debug!("read obj from data success!");
        let (event, rx) = RunCountEvent::sub_runcount(pid, obj);
        RequestAbility::runcount_manager().send_event(event);
        debug!("send event sub runcount success!");

        let ret = match rx.get() {
            Some(ret) => ret,
            None => {
                error!("Service runcount subscribe: receives ret failed");
                return Err(IpcStatusCode::Failed);
            }
        };
        reply.write(&(ret as i32))?;
        if ret != ErrorCode::ErrOk {
            error!(
                "Service runcount subscribe: on failed for ret is {}",
                ret as i32
            );
            return Err(IpcStatusCode::Failed);
        }
        Ok(())
    }
}
