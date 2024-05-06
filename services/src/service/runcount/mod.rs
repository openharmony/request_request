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

mod manager;

use ipc::parcel::MsgParcel;
use ipc::remote::RemoteObj;
use ipc::IpcResult;
pub(crate) use manager::{RunCountManager, RunCountManagerEntry};
use ylong_runtime::sync::oneshot::{channel, Sender};

use super::interface;
use crate::error::ErrorCode;
use crate::utils::Recv;

pub(crate) enum RunCountEvent {
    Sub(SubKey, RemoteObj, Sender<ErrorCode>),
    Unsub(SubKey, Sender<ErrorCode>),
    Change(i64),
}

impl RunCountEvent {
    pub(crate) fn sub_runcount(pid: u64, obj: RemoteObj) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (Self::Sub(SubKey::new(pid), obj, tx), Recv::new(rx))
    }

    pub(crate) fn unsub_runcount(pid: u64) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (Self::Unsub(SubKey::new(pid), tx), Recv::new(rx))
    }

    pub(crate) fn change_runcount(change: i64) -> Self {
        Self::Change(change)
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) struct SubKey {
    pid: u64,
}

impl SubKey {
    fn new(pid: u64) -> Self {
        Self { pid }
    }
}

struct SubClient {
    obj: RemoteObj,
}

impl SubClient {
    fn new(obj: RemoteObj) -> Self {
        Self { obj }
    }

    fn notify_runcount(&self, runcount: i64) {
        debug!("notify runcount in");
        let mut parcel = MsgParcel::new();

        if self.write_parcel_runcount(&mut parcel, runcount).is_err() {
            error!("During notify_runcount: ipc write failed");
            return;
        }

        debug!("During notify_runcount: send request");
        if let Err(e) = self
            .obj
            .send_request(interface::NOTIFY_RUN_COUNT, &mut parcel)
        {
            error!("During notify_runcount: send request failed {:?}", e);
            return;
        }
        debug!("During notify_runcount: send request success");
    }

    fn write_parcel_runcount(&self, parcel: &mut MsgParcel, runcount: i64) -> IpcResult<()> {
        parcel.write_interface_token("OHOS.Download.NotifyInterface")?;
        parcel.write(&(runcount))?;
        Ok(())
    }
}
