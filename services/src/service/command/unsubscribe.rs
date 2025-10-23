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

//! Task unsubscription functionality for request service.
//! 
//! This module provides methods to unsubscribe from download task notifications,
//! with validation, permission checking, and client management.

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    /// Unsubscribes from notifications for a specific download task.
    ///
    /// # Arguments
    ///
    /// * `data` - Message parcel containing the task ID to unsubscribe from
    /// * `reply` - Message parcel to write operation result to
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If unsubscription was successful
    /// * `Err(IpcStatusCode::Failed)` - If task ID is invalid, task not found, or
    ///   caller lacks ownership permissions
    ///
    /// # Errors
    ///
    /// Returns error codes in the reply parcel:
    /// * `ErrOk` - Unsubscription successful
    /// * `TaskNotFound` - Invalid task ID, task not found, or permission denied
    ///
    /// # Notes
    ///
    /// * Validates task ID format and caller's ownership permissions
    /// * Removes notification subscription through the client manager
    pub(crate) fn unsubscribe(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        // Read task ID from input parcel
        let task_id: String = data.read()?;
        info!("Service unsubscribe tid {}", task_id);

        // Validate and parse task ID format
        let Ok(task_id) = task_id.parse::<u32>() else {
            error!("End Service unsubscribe, failed: task_id not valid");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A30,
                "End Service unsubscribe, failed: task_id not valid"
            );
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        };
        
        // Get caller's UID for permission validation
        let uid = ipc::Skeleton::calling_uid();

        // Verify task ownership to prevent unauthorized access
        if !self.check_task_uid(task_id, uid) {
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        // Attempt to unsubscribe from task notifications
        if self.client_manager.unsubscribe(task_id) == ErrorCode::ErrOk {
            reply.write(&(ErrorCode::ErrOk as i32))?;
            Ok(())
        } else {
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            Err(IpcStatusCode::Failed)
        }
    }
}
