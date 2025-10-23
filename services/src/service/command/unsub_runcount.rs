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

//! Running count unsubscription functionality for request service.
//! 
//! This module provides methods to unsubscribe from download task running count updates,
//! with security checks and process identification.

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::service::RequestServiceStub;
use crate::utils::is_called_by_hap;

impl RequestServiceStub {
    /// Unsubscribes from task running count updates for the calling process.
    ///
    /// # Arguments
    ///
    /// * `reply` - Message parcel to write operation result to
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If unsubscription was successful
    /// * `Err(IpcStatusCode::Failed)` - If caller is unauthorized or unsubscription failed
    ///
    /// # Errors
    ///
    /// Returns error codes in the reply parcel:
    /// * `ErrOk` - Unsubscription successful
    /// * Various error codes - Depending on unsubscription failure reason
    ///
    /// # Notes
    ///
    /// * Only system processes are allowed to unsubscribe (not HAPs)
    /// * Uses calling process ID to identify the subscription to remove
    pub(crate) fn unsubscribe_run_count(&self, reply: &mut MsgParcel) -> IpcResult<()> {
        // Verify caller is not a HAP (only system processes allowed)
        if is_called_by_hap() {
            error!("Service run_count unsubscribe called by hap");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A34,
                "Service run_count unsubscribe called by hap"
            );
            return Err(IpcStatusCode::Failed);
        }

        // Get caller's process ID for identifying the subscription
        let pid = ipc::Skeleton::calling_pid();
        info!("Service run_count unsubscribe pid {}", pid);

        // Request unsubscription from run count manager
        let ret = self.run_count_manager.unsubscribe_run_count(pid);
        reply.write(&(ret as i32))?;
        
        // Handle unsubscription failure
        if ret != ErrorCode::ErrOk {
            error!("End Service run_count unsubscribe, failed: {}", ret as i32);
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A34,
                &format!("End Service run_count unsubscribe, failed: {}", ret as i32)
            );
            return Err(IpcStatusCode::Failed);
        }
        Ok(())
    }
}
