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

use crate::manage::events::TaskManagerEvent;
use crate::service::{get_calling_bundle, is_system_api, RequestServiceStub};
use crate::utils::filter::{CommonFilter, Filter};

impl RequestServiceStub {
    pub(crate) fn search(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        info!("Process Service search");
        let mut bundle: String = data.read()?;
        if !is_system_api() {
            debug!("Service search: not system api");
            bundle = get_calling_bundle();
            debug!("Service search: bundle change: {}", bundle);
        }
        debug!("Service search: bundle is {}", bundle);
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
        let common_data = CommonFilter {
            before,
            after,
            state: state as u8,
            action: action as u8,
            mode: mode as u8,
        };
        let filter = Filter {
            bundle,
            common_data,
        };
        let (event, rx) = TaskManagerEvent::search(filter);
        if !self.task_manager.send_event(event) {
            return Err(IpcStatusCode::Failed);
        }
        let ids = match rx.get() {
            Some(ids) => ids,
            None => {
                error!("End Service search, failed with reason: receives ids failed");
                return Err(IpcStatusCode::Failed);
            }
        };
        info!(
            "End Service search successfully: search task ids is {:?}",
            ids
        );
        reply.write(&(ids.len() as u32))?;
        for it in ids.iter() {
            reply.write(&(it.to_string()))?;
        }
        Ok(())
    }
}
