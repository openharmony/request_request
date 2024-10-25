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
use ipc::IpcResult;

use crate::manage::query::{self, SearchMethod, TaskFilter};
use crate::service::RequestServiceStub;
use crate::utils::is_system_api;

impl RequestServiceStub {
    pub(crate) fn search(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        info!("Service search");
        let bundle: String = data.read()?;

        let method = if is_system_api() {
            debug!("Service system api search: bundle name is {}", bundle);
            SearchMethod::System(bundle)
        } else {
            let uid = ipc::Skeleton::calling_uid();
            debug!("Service user search: uid is {}", uid);
            SearchMethod::User(uid)
        };

        let before: i64 = data.read()?;
        debug!("Service search: before is {}", before);
        let after: i64 = data.read()?;
        debug!("Service search: after is {}", after);
        let state: u32 = data.read()?;
        debug!("Service search: state is {}", state);
        let action: u32 = data.read()?;
        debug!("Service search: action is {}", action);
        let mode: u32 = data.read()?;
        debug!("Service search: mode is {}", mode);

        let filter = TaskFilter {
            before,
            after,
            state: state as u8,
            action: action as u8,
            mode: mode as u8,
        };

        let ids = query::search(filter, method);
        debug!("End Service search ok: search task ids is {:?}", ids);
        reply.write(&(ids.len() as u32))?;
        for it in ids.iter() {
            reply.write(&(it.to_string()))?;
        }
        Ok(())
    }
}
