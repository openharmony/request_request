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

use ipc_rust::{IRemoteObj, InterfaceToken, IpcResult, MsgParcel, RemoteObj};
pub(crate) use manager::{RunCountManager, RunCountManagerEntry};
use ylong_runtime::sync::oneshot::{channel, Sender};

use crate::error::ErrorCode;
use crate::service::RequestNotifyInterfaceCode;
use crate::utils::Recv;

pub(crate) enum RunCountEvent {
    SubRunCount(SubKey, RemoteObj, Sender<ErrorCode>),
    UnsubRunCount(SubKey, Sender<ErrorCode>),
    ChangeRunCount(i64),
    Shutdown,
}

impl RunCountEvent {
    pub(crate) fn sub_runcount(pid: u64, obj: RemoteObj) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (Self::SubRunCount(SubKey::new(pid), obj, tx), Recv::new(rx))
    }

    pub(crate) fn unsub_runcount(pid: u64) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::UnsubRunCount(SubKey::new(pid), tx),
            Recv::new(rx),
        )
    }

    pub(crate) fn change_runcount(change: i64) -> Self {
        Self::ChangeRunCount(change)
    }

    pub(crate) fn shutdown() -> Self {
        Self::Shutdown
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

#[derive(Clone)]
struct SubClient {
    obj: RemoteObj,
}

impl SubClient {
    fn new(obj: RemoteObj) -> Self {
        Self { obj }
    }

    fn notify_runcount(&self, runcount: i64) {
        let mut parcel = match MsgParcel::new() {
            Some(parcel) => parcel,
            None => {
                error!("During notify_runcount: create MsgParcel failed");
                return;
            }
        };

        if self.write_parcel_runcount(&mut parcel, runcount).is_err() {
            error!("During notify_runcount: ipc write failed");
            return;
        }

        debug!("During notify_runcount: send request");
        if let Err(e) = self.obj.send_request(
            RequestNotifyInterfaceCode::NotifyRunCount as u32,
            &parcel,
            false,
        ) {
            error!("During notify_runcount: send request failed {:?}", e);
        }
        debug!("During notify_runcount: send request success");
    }

    fn write_parcel_runcount(&self, parcel: &mut MsgParcel, runcount: i64) -> IpcResult<()> {
        parcel.write(&InterfaceToken::new("OHOS.Download.NotifyInterface"))?;
        parcel.write(&(runcount))?;
        Ok(())
    }
}
