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

//! Task subscription functionality for request service.
//! 
//! This module provides methods to subscribe to download task notifications,
//! with validation, permission checking, and event management.

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::manage::events::TaskManagerEvent;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    /// Subscribes a client to notifications for a specific download task.
    ///
    /// # Arguments
    ///
    /// * `data` - Message parcel containing task ID string
    /// * `reply` - Message parcel to write operation result to
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If subscription was successful
    /// * `Err(IpcStatusCode::Failed)` - If subscription failed due to validation error
    ///   or permission issues
    ///
    /// # Errors
    ///
    /// Returns error codes in the reply parcel:
    /// * `ErrOk` - Subscription successful
    /// * `TaskNotFound` - Task ID is invalid or doesn't belong to caller
    /// * `Other` - Other subscription failure
    ///
    /// # Notes
    ///
    /// * Validates that the task ID belongs to the calling user
    /// * Registers the client with both the task manager and client manager
    /// * Uses full token ID for secure client identification
    pub(crate) fn subscribe(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        // Read task ID from parcel
        let task_id: String = data.read()?;
        debug!("Service subscribe tid {}", task_id);

        // Validate and parse task ID format
        let Ok(task_id) = task_id.parse::<u32>() else {
            error!("End Service subscribe, failed: task_id not valid");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A28,
                "End Service subscribe, failed: task_id not valid"
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

        // Get caller's process and token information for subscription tracking
        let pid = ipc::Skeleton::calling_pid();
        let token_id = ipc::Skeleton::calling_full_token_id();

        // Create subscription event for the task manager
        let (event, rx) = TaskManagerEvent::subscribe(task_id, token_id);
        
        // Send event to task manager
        if !self.task_manager.lock().unwrap().send_event(event) {
            reply.write(&(ErrorCode::Other as i32))?;
            error!(
                "End Service subscribe, tid: {}, failed: send event failed",
                task_id
            );
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A28,
                &format!(
                    "End Service subscribe, tid: {}, failed: send event failed",
                    task_id
                )
            );
            return Err(IpcStatusCode::Failed);
        }
        
        // Wait for task manager's response
        let ret = match rx.get() {
            Some(ret) => ret,
            None => {
                error!(
                    "End Service subscribe, tid: {}, failed: receives ret failed",
                    task_id
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A28,
                    &format!(
                        "End Service subscribe, tid: {}, failed: receives ret failed",
                        task_id
                    )
                );
                reply.write(&(ErrorCode::Other as i32))?;
                return Err(IpcStatusCode::Failed);
            }
        };

        // Handle task manager subscription failure
        if ret != ErrorCode::ErrOk {
            error!("End Service subscribe, tid: {}, failed: {:?}", task_id, ret);
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A28,
                &format!("End Service subscribe, tid: {}, failed: {:?}", task_id, ret)
            );
            reply.write(&(ret as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        // Register client with client manager for notification delivery
        let ret = self.client_manager.subscribe(task_id, pid, uid, token_id);
        if ret == ErrorCode::ErrOk {
            reply.write(&(ErrorCode::ErrOk as i32))?;
            debug!("End Service subscribe ok: tid: {}", task_id);
            Ok(())
        } else {
            error!("End Service subscribe, tid: {}, failed: {:?}", task_id, ret);
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A28,
                &format!("End Service subscribe, tid: {}, failed: {:?}", task_id, ret)
            );
            reply.write(&(ret as i32))?;
            Err(IpcStatusCode::Failed)
        }
    }
}
