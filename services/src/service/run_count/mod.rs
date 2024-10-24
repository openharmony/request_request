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

cfg_oh! {
    use ipc::parcel::MsgParcel;
    use ipc::remote::RemoteObj;
    use ipc::IpcResult;
}
pub(crate) use manager::{RunCountManager, RunCountManagerEntry};
use ylong_runtime::sync::oneshot::Sender;

use super::interface;
use crate::error::ErrorCode;

pub(crate) enum RunCountEvent {
    #[cfg(feature = "oh")]
    Subscribe(u64, RemoteObj, Sender<ErrorCode>),
    Unsubscribe(u64, Sender<ErrorCode>),
    #[cfg(feature = "oh")]
    Change(usize),
}

struct Client {
    #[cfg(feature = "oh")]
    obj: RemoteObj,
}

impl Client {
    fn new(#[cfg(feature = "oh")] obj: RemoteObj) -> Self {
        Self {
            #[cfg(feature = "oh")]
            obj,
        }
    }

    #[cfg(feature = "oh")]
    fn notify_run_count(&self, run_count: i64) -> IpcResult<()> {
        info!("notify run_count is {}", run_count);
        #[cfg(feature = "oh")]
        {
            let mut parcel = MsgParcel::new();

            parcel.write_interface_token("OHOS.Download.NotifyInterface")?;
            parcel.write(&(run_count))?;

            self.obj
                .send_request(interface::NOTIFY_RUN_COUNT, &mut parcel)?;
            Ok(())
        }
    }
}
