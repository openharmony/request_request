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

//! Task information retrieval functionality for download tasks.
//! 
//! This module provides methods to query detailed information about tasks,
//! with permission checking, validation, and bulk operation support.

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::info::TaskInfo;
use crate::manage::database::RequestDb;
use crate::service::command::{set_code_with_index_other, GET_INFO_MAX};
use crate::service::permission::PermissionChecker;
use crate::service::{serialize_task_info, RequestServiceStub};
use crate::task::files::check_current_account;

impl RequestServiceStub {
    /// Retrieves detailed information for multiple download tasks.
    ///
    /// # Arguments
    ///
    /// * `data` - Message parcel containing count and task IDs to query
    /// * `reply` - Message parcel to write operation results to
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the information retrieval operation completed
    /// * `Err(IpcStatusCode::Failed)` - If input validation failed
    ///
    /// # Errors
    ///
    /// Returns error codes in the reply parcel:
    /// * `ErrOk` - Task information retrieved successfully
    /// * `Other` - Input size exceeds maximum allowed or other system error
    /// * `TaskNotFound` - Invalid task ID, task does not exist, or permission denied
    ///
    /// # Notes
    ///
    /// * Requires `DOWNLOAD_SESSION_MANAGER` permission to view tasks belonging to other UIDs
    /// * Input is limited to `GET_INFO_MAX` number of tasks
    /// * Performs account and UID validation to ensure proper access control
    pub(crate) fn show(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        debug!("Service show");
        // Check if caller has download permission (needed for cross-UID access)
        let permission = PermissionChecker::check_down_permission();
        let len: u32 = data.read()?;
        let len = len as usize;

        // Validate input size against maximum allowed
        if len > GET_INFO_MAX {
            info!("Service show: out of size: {}", len);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        // Get caller's UID for permission validation
        let ipc_uid = ipc::Skeleton::calling_uid();
        // Pre-allocate results vector with default error values
        let mut vec = vec![(ErrorCode::Other, TaskInfo::new()); len];
        
        // Process each task individually
        for i in 0..len {
            // Read task ID from input parcel
            let task_id: String = data.read()?;
            info!("Service show tid {}", task_id);

            // Parse and validate task ID format
            let Ok(task_id) = task_id.parse::<u32>() else {
                error!("Service show, failed: tid not valid: {}", task_id);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A18,
                    &format!("Service show, failed: tid not valid: {}", task_id)
                );
                set_code_with_index_other(&mut vec, i, ErrorCode::TaskNotFound);
                continue;
            };

            // Get task owner UID from database
            let task_uid = match RequestDb::get_instance().query_task_uid(task_id) {
                Some(uid) => uid,
                None => {
                    set_code_with_index_other(&mut vec, i, ErrorCode::TaskNotFound);
                    continue;
                }
            };

            // Verify current account matches task owner's account
            if !check_current_account(task_uid) {
                set_code_with_index_other(&mut vec, i, ErrorCode::TaskNotFound);
                continue;
            }

            // Check permission for cross-UID access
            if (task_uid != ipc_uid) && !permission {
                set_code_with_index_other(&mut vec, i, ErrorCode::TaskNotFound);
                error!(
                    "Service show, failed: check task uid. tid: {}, uid: {}",
                    task_id, ipc_uid
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A18,
                    &format!(
                        "Service show, failed: check task uid. tid: {}, uid: {}",
                        task_id, ipc_uid
                    )
                );
                continue;
            }

            // Request task information from task manager
            let info = self.task_manager.lock().unwrap().show(task_uid, task_id);
            match info {
                Some(task_info) => {
                    // Update results with success code and task info
                    if let Some((c, info)) = vec.get_mut(i) {
                        *c = ErrorCode::ErrOk;
                        *info = task_info;
                    }
                }
                None => {
                    error!("Service show, failed: task_id not found, tid: {}", task_id);
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A18,
                        &format!("Service show, failed: task_id not found, tid: {}", task_id)
                    );
                    set_code_with_index_other(&mut vec, i, ErrorCode::TaskNotFound);
                }
            };
        }
        
        // Write overall operation success code
        reply.write(&(ErrorCode::ErrOk as i32))?;
        
        // Serialize each result to reply parcel
        for (c, info) in vec {
            reply.write(&(c as i32))?;
            // TODO: Sends info only when ErrOk.
            serialize_task_info(info, reply)?;
        }
        Ok(())
    }
}
