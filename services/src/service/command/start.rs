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

//! Task initiation functionality for download tasks.
//! 
//! This module provides methods to start download tasks,
//! with permission checking, validation, and bulk operation support.

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::manage::database::RequestDb;
use crate::manage::events::TaskManagerEvent;
use crate::service::command::{set_code_with_index, CONTROL_MAX};
use crate::service::permission::PermissionChecker;
use crate::service::RequestServiceStub;
use crate::task::files::check_current_account;

impl RequestServiceStub {
    /// Starts execution of multiple download tasks.
    ///
    /// # Arguments
    ///
    /// * `data` - Message parcel containing count and task IDs to start
    /// * `reply` - Message parcel to write operation results to
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the task start operation completed
    /// * `Err(IpcStatusCode::Failed)` - If input validation failed or permission denied
    ///
    /// # Errors
    ///
    /// Returns error codes in the reply parcel:
    /// * `ErrOk` - Task started successfully
    /// * `Permission` - Caller lacks required internet or download permission
    /// * `Other` - Input size exceeds maximum allowed or other system error
    /// * `TaskNotFound` - Invalid task ID, task does not exist, or permission denied
    ///
    /// # Notes
    ///
    /// * Requires `INTERNET` permission to start tasks, or `DOWNLOAD_SESSION_MANAGER` 
    ///   permission for privileged access
    /// * Input is limited to `CONTROL_MAX` number of tasks
    /// * Performs account and UID validation to ensure proper access control
    pub(crate) fn start(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        debug!("Service start");
        // Check if caller has download permission (needed for privileged operations)
        let permission = PermissionChecker::check_down_permission();
        
        // Verify internet permission unless caller has download permission
        if !PermissionChecker::check_internet() && !permission {
            error!("Service start: no INTERNET permission.");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A13,
                "Service start: no INTERNET permission."
            );
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        // Read input count and convert to usize
        let len: u32 = data.read()?;
        let len = len as usize;
        
        // Validate input size against maximum allowed
        if len > CONTROL_MAX {
            info!("Service start: out of size: {}", len);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        
        // Pre-allocate results vector with default error values
        let mut vec = vec![ErrorCode::Other; len];

        // Get caller's UID for permission validation
        let ipc_uid = ipc::Skeleton::calling_uid();

        // Process each task individually
        for i in 0..len {
            // Read task ID from input parcel
            let task_id: String = data.read()?;
            info!("Service start {}", task_id);
            
            // Parse and validate task ID format
            let Ok(task_id) = task_id.parse::<u32>() else {
                error!("Service start, failed: tid not valid: {}", task_id);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A14,
                    &format!("Service start, failed: tid not valid: {}", task_id)
                );
                set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                continue;
            };

            // Get task owner UID from database
            let task_uid = match RequestDb::get_instance().query_task_uid(task_id) {
                Some(uid) => uid,
                None => {
                    set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                    continue;
                }
            };

            // Verify current account matches task owner's account
            if !check_current_account(task_uid) {
                set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                continue;
            }

            // Check permission for cross-UID access
            if (task_uid != ipc_uid) && !permission {
                set_code_with_index(&mut vec, i, ErrorCode::TaskNotFound);
                error!(
                    "Service start, failed: check task uid. tid: {}, uid: {}",
                    task_id, ipc_uid
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A14,
                    &format!(
                        "Service start, failed: check task uid. tid: {}, uid: {}",
                        task_id, ipc_uid
                    )
                );
                continue;
            }

            // Create and send task start event to task manager
            let (event, rx) = TaskManagerEvent::start(task_uid, task_id);
            if !self.task_manager.lock().unwrap().send_event(event) {
                error!("Service start, failed: task_manager err: {}", task_id);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A14,
                    &format!("Service start, failed: task_manager err: {}", task_id)
                );
                set_code_with_index(&mut vec, i, ErrorCode::Other);
                continue;
            }
            
            // Receive result from task manager
            let ret = match rx.get() {
                Some(ret) => ret,
                None => {
                    error!(
                        "Service start, tid: {}, failed: receives ret failed",
                        task_id
                    );
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A14,
                        &format!(
                            "Service start, tid: {}, failed: receives ret failed",
                            task_id
                        )
                    );
                    set_code_with_index(&mut vec, i, ErrorCode::Other);
                    continue;
                }
            };
            
            // Update results with operation status
            set_code_with_index(&mut vec, i, ret);
            
            // Log error if task start failed
            if ret != ErrorCode::ErrOk {
                error!("Service start, tid: {}, failed: {}", task_id, ret as i32);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A14,
                    &format!("Service start, tid: {}, failed: {}", task_id, ret as i32)
                );
            }
        }

        // Write overall operation success code
        reply.write(&(ErrorCode::ErrOk as i32))?;
        
        // Serialize each result to reply parcel
        for ret in vec {
            reply.write(&(ret as i32))?;
        }
        Ok(())
    }
}
