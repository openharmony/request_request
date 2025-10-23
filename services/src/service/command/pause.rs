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

//! Task pause functionality for request service.
//! 
//! This module implements methods to pause active download/upload tasks with
//! permission validation, ID verification, and proper error handling.
//! It supports batch pause operations for multiple tasks in a single call.

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
    /// Pauses one or more active tasks.
    ///
    /// Attempts to pause a batch of tasks specified by their IDs, performing
    /// permission validation, task ownership checks, and account verification
    /// for each task.
    ///
    /// # Arguments
    ///
    /// * `data` - Input parcel containing API version and list of task IDs to pause.
    /// * `reply` - Output parcel to write operation result code and individual task results.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the operation completed successfully, regardless of individual task results.
    /// * `Err(IpcStatusCode::Failed)` - If permission validation failed or the input size exceeded limits.
    ///
    /// # Errors
    ///
    /// Returns error codes in the reply parcel:
    /// * Overall status code (first value) - Always `ErrorCode::ErrOk` if the function returns `Ok(())`.
    /// * Individual task status codes - One per task ID, in the same order as input:
    ///   * `ErrorCode::ErrOk` - Task paused successfully.
    ///   * `ErrorCode::TaskNotFound` - Task ID invalid, doesn't exist, or caller lacks permission.
    ///   * `ErrorCode::Other` - Failed to communicate with task manager or other error.
    ///   * `ErrorCode::Permission` - Caller lacks required internet permission.
    ///
    /// # Notes
    ///
    /// This method supports batch operations with a maximum of `CONTROL_MAX` tasks per call.
    pub(crate) fn pause(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        // Check for download permission to manage tasks across applications
        let permission = PermissionChecker::check_down_permission();
        let version: u32 = data.read()?;
        
        // API9 requires internet permission for task operations
        if Version::from(version as u8) == Version::API9 && !PermissionChecker::check_internet() {
            error!("Service pause: no INTERNET permission");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A03,
                "Service pause: no INTERNET permission"
            );
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        // Read number of tasks to pause and validate against size limit
        let len: u32 = data.read()?;
        let len = len as usize;
        // Enforce maximum number of tasks per call to prevent resource exhaustion
        if len > CONTROL_MAX {
            info!("Service pause: out of size: {}", len);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        // Get caller's UID for permission checks
        let ipc_uid = ipc::Skeleton::calling_uid();
        // Initialize result vector with default error values
        let mut vec = vec![ErrorCode::Other; len];
        for i in 0..len {
            let task_id: String = data.read()?;
            info!("Service pause tid {}", task_id);

            let Ok(task_id) = task_id.parse::<u32>() else {
                error!("Service pause, failed: tid not valid: {}", task_id);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A04,
                    &format!("Service pause, failed: tid not valid: {}", task_id)
                );
                set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                continue;
            };

            // Get task owner's UID from database
            let task_uid = match RequestDb::get_instance().query_task_uid(task_id) {
                Some(uid) => uid,
                None => {
                    // Task doesn't exist
                    set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                    continue;
                }
            };

            // Verify task belongs to the current user account
            if !check_current_account(task_uid) {
                set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                continue;
            }

            // Ensure caller owns the task or has management permissions
            if (task_uid != ipc_uid) && !permission {
                set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                error!(
                    "Service pause, failed: check task uid. tid: {}, uid: {}",
                    task_id, ipc_uid
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A04,
                    &format!(
                        "Service pause, failed: check task uid. tid: {}, uid: {}",
                        task_id, ipc_uid
                    )
                );
                continue;
            }

            // Create and send pause event to task manager
            let (event, rx) = TaskManagerEvent::pause(task_uid, task_id);
            // Send the event to the task manager for processing
            if !self.task_manager.lock().unwrap().send_event(event) {
                error!("Service pause, failed: task_manager err: {}", task_id);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A04,
                    &format!("Service pause, failed: task_manager err: {}", task_id)
                );
                set_code_with_index(&mut vec, i, ErrorCode::Other);
                continue;
            }

            // Wait for pause operation result from task manager
            let ret = match rx.get() {
                Some(ret) => ret,
                None => {
                    error!(
                        "Service pause, tid: {}, failed: receives ret failed",
                        task_id
                    );
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A04,
                        &format!(
                            "Service pause, tid: {}, failed: receives ret failed",
                            task_id
                        )
                    );
                    set_code_with_index(&mut vec, i, ErrorCode::Other);
                    continue;
                }
            };
            set_code_with_index(&mut vec, i, ret);
            if ret != ErrorCode::ErrOk {
                error!("Service start, tid: {}, failed: {}", task_id, ret as i32);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A04,
                    &format!("Service start, tid: {}, failed: {}", task_id, ret as i32)
                );
            }
        }

        reply.write(&(ErrorCode::ErrOk as i32))?;
        for ret in vec {
            reply.write(&(ret as i32))?;
        }
        Ok(())
    }
}
