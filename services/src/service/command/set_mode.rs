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

//! Task mode configuration functionality for download tasks.
//! 
//! This module provides methods to change the operational mode of tasks,
//! with permission checking, validation, and event-based task management.

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::config::Mode;
use crate::error::ErrorCode;
use crate::manage::database::RequestDb;
use crate::manage::events::TaskManagerEvent;
use crate::service::permission::PermissionChecker;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    /// Changes the operational mode of a download task.
    ///
    /// # Arguments
    ///
    /// * `data` - Message parcel containing task ID and new mode
    /// * `reply` - Message parcel to write operation result to
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the mode change operation completed successfully
    /// * `Err(IpcStatusCode::Failed)` - If there was a permission issue, validation failure,
    ///   or task manager error
    ///
    /// # Errors
    ///
    /// Returns error codes in the reply parcel:
    /// * `ErrOk` - Mode changed successfully or mode was already correct
    /// * `Permission` - Caller lacks required download permission
    /// * `TaskNotFound` - Invalid task ID or task does not exist
    /// * `Other` - General failure in task manager or result retrieval
    ///
    /// # Notes
    ///
    /// * Requires `DOWNLOAD_SESSION_MANAGER` permission
    /// * Mode change is skipped if new mode equals current mode or is `Mode::Any`
    pub(crate) fn set_mode(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        // Check if caller has required download permission
        let permission = PermissionChecker::check_down_permission();
        if !permission {
            error!("Service change_mode: no DOWNLOAD_SESSION_MANAGER permission.");
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A43,
                "Service change_mode: no DOWNLOAD_SESSION_MANAGER permission."
            );
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        // Read and parse task ID
        let task_id: String = data.read()?;
        info!("Service change_mode tid {}", task_id);
        let Ok(task_id) = task_id.parse::<u32>() else {
            error!("Service change_mode, failed: tid not valid: {}", task_id);
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A44,
                &format!("Service change_mode, failed: tid not valid: {}", task_id)
            );
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        };

        // Read and convert mode value
        let mode: u32 = data.read()?;
        let mode = Mode::from(mode as u8);

        // Get current mode from database to check if change is needed
        let old_mode = match RequestDb::get_instance().query_task_mode(task_id) {
            Some(m) => m,
            None => {
                error!(
                    "Service change_mode, failed: old_mode not valid: {}",
                    task_id
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A44,
                    &format!(
                        "Service change_mode, failed: old_mode not valid: {}",
                        task_id
                    )
                );
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                return Err(IpcStatusCode::Failed);
            }
        };

        // Skip if modes are already the same or new mode is Any
        if old_mode == mode || mode == Mode::Any {
            error!("Service change_mode, mod state is ok: {}", task_id);
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A44,
                &format!("Service change_mode, mod state is ok: {}", task_id)
            );
            reply.write(&(ErrorCode::ErrOk as i32))?;
            return Ok(());
        }

        // Get task owner UID from database
        let uid = match RequestDb::get_instance().query_task_uid(task_id) {
            Some(id) => id,
            None => {
                reply.write(&(ErrorCode::TaskNotFound as i32))?;
                return Err(IpcStatusCode::Failed);
            }
        };

        // Create and send mode change event to task manager
        let (event, rx) = TaskManagerEvent::set_mode(uid, task_id, mode);
        if !self.task_manager.lock().unwrap().send_event(event) {
            error!("Service change_mode, failed: task_manager err: {}", task_id);
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A44,
                &format!("Service change_mode, failed: task_manager err: {}", task_id)
            );
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        
        // Receive result from task manager
        let ret = match rx.get() {
            Some(ret) => ret,
            None => {
                error!(
                    "Service change_mode, tid: {}, failed: receives ret failed",
                    task_id
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A44,
                    &format!("Service change_mode, mod state is ok: {}", task_id)
                );
                reply.write(&(ErrorCode::Other as i32))?;
                return Err(IpcStatusCode::Failed);
            }
        };
        
        // Send the operation result
        reply.write(&(ret as i32))?;
        Ok(())
    }
}
