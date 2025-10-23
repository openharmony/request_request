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

//! Task speed limitation functionality for download tasks.
//! 
//! This module provides methods to set maximum download speed limits for tasks,
//! with validation, permission checking, and bulk operation support.

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::manage::database::RequestDb;
use crate::manage::events::TaskManagerEvent;
use crate::service::command::{set_code_with_index, CONTROL_MAX};
use crate::service::permission::PermissionChecker;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    /// Sets maximum download speed limits for multiple tasks.
    ///
    /// # Arguments
    ///
    /// * `data` - Message parcel containing task IDs and their corresponding speed limits
    /// * `reply` - Message parcel to write operation results to
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the operation completed successfully (though individual tasks may fail)
    /// * `Err(IpcStatusCode::Failed)` - If the input size exceeds maximum allowed
    /// * `Err(_)` - If there was an error reading from or writing to the message parcels
    ///
    /// # Errors
    ///
    /// Returns error codes for each task in the reply parcel:
    /// * `ErrOk` - Speed limit set successfully
    /// * `ParameterCheck` - Invalid speed value (must be >= 16KB/s)
    /// * `TaskNotFound` - Task ID invalid or permission denied
    /// * `Other` - General failure
    ///
    /// # Notes
    ///
    /// * Speed values must be at least 16KB/s (16 * 1024 bytes/s)
    /// * Performs permission checking to ensure tasks belong to the caller
    /// * Supports bulk operation with individual error handling for each task
    pub(crate) fn set_max_speed(
        &self,
        data: &mut MsgParcel,
        reply: &mut MsgParcel,
    ) -> IpcResult<()> {
        info!("Service set_max_speed");
        // Minimum speed limit: 16KB/s
        const MIN_SPEED_LIMIT: i64 = 16 * 1024;
        
        // Check if caller has download permission
        let permission = PermissionChecker::check_down_permission();

        // Read number of tasks to process
        let len: u32 = data.read()?;
        let len = len as usize;

        // Validate input size against maximum allowed
        if len > CONTROL_MAX {
            info!("Service set_max_speed: out of size: {}", len);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        // Get calling process UID for permission verification
        let uid = ipc::Skeleton::calling_uid();
        
        // Initialize result vector with default error values
        let mut vec = vec![ErrorCode::Other; len];
        
        // Process each task individually
        for i in 0..len {
            let task_id: String = data.read()?;
            let max_speed: i64 = data.read()?;
            
            // Validate speed limit is above minimum threshold
            if max_speed < MIN_SPEED_LIMIT {
                error!(
                    "Service set_max_speed, failed: speed not valid: {}",
                    max_speed
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A42,
                    &format!(
                        "Service set_max_speed, failed: speed not valid: {}",
                        max_speed
                    )
                );
                set_code_with_index(&mut vec, i, ErrorCode::ParameterCheck);
                continue;
            }
            
            // Parse and validate task ID format
            let Ok(task_id) = task_id.parse::<u32>() else {
                error!("Service set_max_speed, failed: tid not valid: {}", task_id);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A42,
                    &format!("Service set_max_speed, failed: tid not valid: {}", task_id)
                );
                set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                continue;
            };

            let mut uid = uid;

            // For privileged callers, get the actual task owner UID from database
            if permission {
                // skip uid check if task used by innerkits
                match RequestDb::get_instance().query_task_uid(task_id) {
                    Some(id) => uid = id,
                    None => {
                        set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                        continue;
                    }
                };
            } else if !self.check_task_uid(task_id, uid) {
                // Verify task ownership for non-privileged callers
                set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                error!(
                    "Service set_max_speed, failed: check task uid. tid: {}, uid: {}",
                    task_id, uid
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A42,
                    &format!(
                        "Service set_max_speed, failed: check task uid. tid: {}, uid: {}",
                        task_id, uid
                    )
                );
                continue;
            }

            // Create and send speed limit event to task manager
            let (event, rx) = TaskManagerEvent::set_max_speed(uid, task_id, max_speed);
            if !self.task_manager.lock().unwrap().send_event(event) {
                error!(
                    "Service set_max_speed, failed: task_manager err: {}",
                    task_id
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A42,
                    &format!(
                        "Service set_max_speed, failed: task_manager err: {}",
                        task_id
                    )
                );
                set_code_with_index(&mut vec, i, ErrorCode::Other);
                continue;
            }

            // Receive result from task manager
            let Some(ret) = rx.get() else {
                error!(
                    "Service set_max_speed, tid: {}, failed: receives ret failed",
                    task_id
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A42,
                    &format!(
                        "Service set_max_speed, tid: {}, failed: receives ret failed",
                        task_id
                    )
                );
                set_code_with_index(&mut vec, i, ErrorCode::Other);
                continue;
            };

            // Store result for this task
            set_code_with_index(&mut vec, i, ret);
            if ret != ErrorCode::ErrOk {
                error!(
                    "Service set_max_speed, tid: {}, failed: {}",
                    task_id, ret as i32
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A42,
                    &format!(
                        "Service set_max_speed, tid: {}, failed: {}",
                        task_id, ret as i32
                    )
                );
            }
        }

        // Send overall operation success code
        reply.write(&(ErrorCode::ErrOk as i32))?;
        
        // Send individual task results
        for ret in vec {
            reply.write(&(ret as i32))?;
        }
        Ok(())
    }
}
