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

//! Task removal functionality for download tasks.
//! 
//! This module provides methods to remove tasks in bulk,
//! including permission verification, input validation,
//! and ownership checks before sending removal events.

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::manage::database::RequestDb;
use crate::manage::events::TaskManagerEvent;
use crate::service::command::{set_code_with_index, CONTROL_MAX};
use crate::service::permission::PermissionChecker;
use crate::service::RequestServiceStub;
use crate::task::config::Version;
use crate::task::files::check_current_account;

impl RequestServiceStub {
    /// Removes multiple tasks in bulk.
    ///
    /// # Arguments
    ///
    /// * `data` - Message parcel containing the API version and task IDs to remove
    /// * `reply` - Message parcel to write the results to
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the remove operation completed successfully
    /// * `Err(IpcStatusCode::Failed)` - If there was an error in the process
    ///
    /// # Errors
    ///
    /// * `ErrorCode::Permission` - When the caller lacks required permissions
    /// * `ErrorCode::Other` - When the input size exceeds limits
    /// * `ErrorCode::TaskNotFound` - When a task ID is invalid or not accessible
    ///
    /// # Notes
    ///
    /// Results are returned in the same order as the input task IDs.
    /// For API9, requires either INTERNET permission or download manager permission.
    pub(crate) fn remove(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        // Check for download manager permissions
        let permission = PermissionChecker::check_down_permission();

        // Read API version and perform version-specific permission checks
        let version: u32 = data.read()?;
        if Version::from(version as u8) == Version::API9
            && !PermissionChecker::check_internet()
            && !permission
        {
            error!("Service remove: no INTERNET permission");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A09,
                "Service pause: no INTERNET permission"
            );
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        // Read and validate the number of tasks to remove
        let len: u32 = data.read()?;
        let len = len as usize;

        // Enforce size limits to prevent resource exhaustion
        if len > CONTROL_MAX {
            info!("Service remove: out of size: {}", len);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        // Get calling UID for ownership verification
        let ipc_uid = ipc::Skeleton::calling_uid();
        
        // Initialize results vector with default error codes
        let mut vec = vec![ErrorCode::Other; len];
        
        // Process each task ID individually
        for i in 0..len {
            let task_id: String = data.read()?;
            info!("Service remove tid {}", task_id);
            
            // Validate and convert task ID format
            let Ok(task_id) = task_id.parse::<u32>() else {
                error!("Service remove, failed: tid not valid: {}", task_id);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A10,
                    &format!("Service remove, failed: tid not valid: {}", task_id)
                );
                set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                continue;
            };

            // Check if task exists and get its UID
            let task_uid = match RequestDb::get_instance().query_task_uid(task_id) {
                Some(uid) => uid,
                None => {
                    set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                    continue;
                }
            };

            // Verify task belongs to the current account
            if !check_current_account(task_uid) {
                set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                continue;
            }

            // Check task ownership or manager permissions
            if (task_uid != ipc_uid) && !permission {
                set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                error!(
                    "Service remove, failed: check task uid. tid: {}, uid: {}",
                    task_id, ipc_uid
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A10,
                    &format!(
                        "Service remove, failed: check task uid. tid: {}, uid: {}",
                        task_id, ipc_uid
                    )
                );
                continue;
            }

            // Create and send removal event to task manager
            let (event, rx) = TaskManagerEvent::remove(task_uid, task_id);
            if !self.task_manager.lock().unwrap().send_event(event) {
                error!("Service remove, failed: task_manager err: {}", task_id);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A10,
                    &format!("Service remove, failed: task_manager err: {}", task_id)
                );
                set_code_with_index(&mut vec, i, ErrorCode::Other);
                continue;
            }
            
            // Get result from task manager
            let ret = match rx.get() {
                Some(ret) => ret,
                None => {
                    error!(
                        "Service remove, tid: {}, failed: receives ret failed",
                        task_id
                    );
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A10,
                        &format!(
                            "Service remove, tid: {}, failed: receives ret failed",
                            task_id
                        )
                    );
                    set_code_with_index(&mut vec, i, ErrorCode::Other);
                    continue;
                }
            };
            
            // Store the result and log any failures
            set_code_with_index(&mut vec, i, ret);
            if ret != ErrorCode::ErrOk {
                error!("Service remove, tid: {}, failed: {}", task_id, ret as i32);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A10,
                    &format!("Service remove, tid: {}, failed: {}", task_id, ret as i32)
                );
            }
        }
        
        // Send successful operation status
        reply.write(&(ErrorCode::ErrOk as i32))?;
        
        // Return individual results for each task
        for ret in vec {
            reply.write(&(ret as i32))?;
        }
        Ok(())
    }
}
