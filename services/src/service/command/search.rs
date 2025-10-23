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

//! Task search functionality for download tasks.
//! 
//! This module provides methods to search for tasks based on various criteria,
//! including time ranges, states, actions, and modes, with different permission levels.

use ipc::parcel::MsgParcel;
use ipc::IpcResult;

use crate::manage::query::{self, SearchMethod, TaskFilter};
use crate::service::RequestServiceStub;
use crate::utils::is_system_api;

impl RequestServiceStub {
    /// Searches for tasks based on specified filters and permission level.
    ///
    /// # Arguments
    ///
    /// * `data` - Message parcel containing search parameters: bundle name, time range,
    ///   state, action, and mode
    /// * `reply` - Message parcel to write the search results to
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the search operation completed successfully
    /// * `Err(_)` - If there was an error reading from or writing to the message parcels
    ///
    /// # Notes
    ///
    /// * System APIs search by bundle name, while user APIs search by UID
    /// * Returns a list of matching task IDs as strings
    pub(crate) fn search(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        debug!("Service search");
        // Read bundle name for system API or UID for user API
        let bundle: String = data.read()?;

        // Determine search method based on API type (system or user)
        let method = if is_system_api() {
            debug!("Service system api search: bundle name is {}", bundle);
            SearchMethod::System(bundle)
        } else {
            let uid = ipc::Skeleton::calling_uid();
            debug!("Service user search: uid is {}", uid);
            SearchMethod::User(uid)
        };

        // Read time range filters
        let before: i64 = data.read()?;
        debug!("Service search: before is {}", before);
        let after: i64 = data.read()?;
        debug!("Service search: after is {}", after);
        
        // Read task state filter
        let state: u32 = data.read()?;
        debug!("Service search: state is {}", state);
        
        // Read task action filter
        let action: u32 = data.read()?;
        debug!("Service search: action is {}", action);
        
        // Read task mode filter
        let mode: u32 = data.read()?;
        debug!("Service search: mode is {}", mode);

        // Construct task filter with all search criteria
        let filter = TaskFilter {
            before,
            after,
            state: state as u8,
            action: action as u8,
            mode: mode as u8,
        };

        // Perform the search operation
        let ids = query::search(filter, method);
        debug!("End Service search ok: search task ids is {:?}", ids);
        
        // Send the count of results first
        reply.write(&(ids.len() as u32))?;
        
        // Send each task ID as a string
        for it in ids.iter() {
            reply.write(&(it.to_string()))?;
        }
        Ok(())
    }
}
