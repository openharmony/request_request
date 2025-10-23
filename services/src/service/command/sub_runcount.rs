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

//! Running count subscription functionality for request service.
//! 
//! This module provides methods to subscribe to download task running count updates,
//! with security checks and remote object handling.

use ipc::parcel::MsgParcel;
use ipc::remote::RemoteObj;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::service::RequestServiceStub;
use crate::utils::is_called_by_hap;

impl RequestServiceStub {
    /// Subscribes to task running count updates with remote callback object.
    ///
    /// # Arguments
    ///
    /// * `data` - Message parcel containing remote callback object
    /// * `reply` - Message parcel to write operation result to
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If subscription was successful
    /// * `Err(IpcStatusCode::Failed)` - If caller is unauthorized or subscription failed
    ///
    /// # Errors
    ///
    /// Returns error codes in the reply parcel:
    /// * `ErrOk` - Subscription successful
    /// * Various error codes - Depending on subscription failure reason
    ///
    /// # Notes
    ///
    /// * Only system processes are allowed to subscribe (not HAPs)
    /// * Uses calling process ID for subscription management
    /// * The remote callback object must be valid and properly initialized
    pub(crate) fn subscribe_run_count(
        &self,
        data: &mut MsgParcel,
        reply: &mut MsgParcel,
    ) -> IpcResult<()> {
        // Verify caller is not a HAP (only system processes allowed)
        if is_called_by_hap() {
            error!("Service run_count subscribe called by hap");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A32,
                "Service run_count subscribe called by hap"
            );
            return Err(IpcStatusCode::Failed);
        }

        // Get caller's process ID for tracking
        let pid = ipc::Skeleton::calling_pid();
        info!("Service run_count subscribe pid {}", pid);

        // Read remote callback object from parcel
        let obj: RemoteObj = data.read_remote()?;
        
        // Register subscription with run count manager
        let ret = self.run_count_manager.subscribe_run_count(pid, obj);

        // Write result code to reply parcel
        reply.write(&(ret as i32))?;
        
        // Handle subscription failure
        if ret != ErrorCode::ErrOk {
            error!("End Service run_count subscribe, failed:{}", ret as i32);
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A32,
                &format!("End Service run_count subscribe, failed:{}", ret as i32)
            );
            return Err(IpcStatusCode::Failed);
        }
        Ok(())
    }
}
