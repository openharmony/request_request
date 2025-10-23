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

//! Module for managing and notifying about running task counts.
//! 
//! This module provides the core types and functionality for the run count management system,
//! including event definitions, client communication, and integration with the manager implementation.

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

/// Events for the run count management system.
/// 
/// This enum defines the different types of events that can be processed by the
/// `RunCountManager` to manage subscriptions and update run counts.
pub(crate) enum RunCountEvent {
    /// Subscribe to run count updates.
    #[cfg(feature = "oh")]
    Subscribe(u64, RemoteObj, Sender<ErrorCode>),
    /// Unsubscribe from run count updates.
    Unsubscribe(u64, Sender<ErrorCode>),
    /// Update the current run count.
    #[cfg(feature = "oh")]
    Change(usize),
}

/// Client for receiving run count notifications.
/// 
/// Handles IPC communication to notify clients when the run count changes.
struct Client {
    /// Remote object for IPC communication with the client
    #[cfg(feature = "oh")]
    obj: RemoteObj,
}

impl Client {
    /// Creates a new client instance.
    /// 
    /// # Arguments
    /// 
    /// * `obj` - Remote object for IPC communication
    fn new(#[cfg(feature = "oh")] obj: RemoteObj) -> Self {
        Self {
            #[cfg(feature = "oh")]
            obj,
        }
    }

    /// Sends a run count notification to the client.
    /// 
    /// # Arguments
    /// 
    /// * `run_count` - The current number of running tasks
    /// 
    /// # Returns
    /// 
    /// Result indicating success or failure of the IPC operation
    #[cfg(feature = "oh")]
    fn notify_run_count(&self, run_count: i64) -> IpcResult<()> {
        info!("run_count:{}", run_count);
        #[cfg(feature = "oh")]
        {
            let mut parcel = MsgParcel::new();
            
            // Write interface token and run count to the parcel
            parcel.write_interface_token("OHOS.Download.NotifyInterface")?;
            parcel.write(&(run_count))?;

            // Send notification request to the client
            self.obj
                .send_request(interface::NOTIFY_RUN_COUNT, &mut parcel)?;
            Ok(())
        }
    }
}
