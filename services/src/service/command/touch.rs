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

//! Task touch/refresh functionality for request service.
//! 
//! This module provides methods to touch/update multiple download tasks,
//! with validation, permission checking, and bulk operation support.

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
    /// Touches multiple download tasks to refresh their status.
    ///
    /// # Arguments
    ///
    /// * `data` - Message parcel containing task IDs and tokens
    /// * `reply` - Message parcel to write operation results to
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the operation completes, regardless of individual task results
    ///
    /// # Notes
    ///
    /// * Processes multiple tasks in bulk with individual result tracking
    /// * Validates task ownership and caller permissions
    /// * Returns error codes for each task individually
    pub(crate) fn touch(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        // Check if caller has download permission (needed for privileged operations)
        let permission = PermissionChecker::check_down_permission();
        
        // Read input count and convert to usize
        let len: u32 = data.read()?;
        let len = len as usize;

        // Validate input size against maximum allowed
        if len > GET_INFO_MAX {
            info!("Service touch: out of size: {}", len);
            reply.write(&(ErrorCode::Other as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        // Get caller's UID for permission validation
        let ipc_uid = ipc::Skeleton::calling_uid();
        
        // Pre-allocate result vector to avoid reallocations
        let mut vec = vec![(ErrorCode::Other, TaskInfo::new()); len];
        
        // Process each task individually
        for i in 0..len {
            // Read task ID and token from input parcel
            let task_id: String = data.read()?;
            info!("Service touch tid {}", task_id);

            let token: String = data.read()?;

            // Parse and validate task ID format
            let Ok(task_id) = task_id.parse::<u32>() else {
                error!("Service touch, failed: tid not valid: {}", task_id);
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A20,
                    &format!("Service touch, failed: tid not valid: {}", task_id)
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
                    "Service touch, failed: check task uid. tid: {}, uid: {}",
                    task_id, ipc_uid
                );
                sys_event!(
                    ExecError,
                    DfxCode::INVALID_IPC_MESSAGE_A20,
                    &format!(
                        "Service touch, failed: check task uid. tid: {}, uid: {}",
                        task_id, ipc_uid
                    )
                );
                continue;
            }

            // Attempt to touch the task with the provided token
            let info = self
                .task_manager
                .lock()
                .unwrap()
                .touch(task_uid, task_id, token);
                
            // Process touch result for this task
            match info {
                Some(task_info) => {
                    if let Some((c, info)) = vec.get_mut(i) {
                        *c = ErrorCode::ErrOk;
                        *info = task_info;
                    }
                }
                None => {
                    error!("Service touch, failed: task_id not found, tid: {}", task_id);
                    sys_event!(
                        ExecError,
                        DfxCode::INVALID_IPC_MESSAGE_A20,
                        &format!("Service touch, failed: task_id not found, tid: {}", task_id)
                    );
                    set_code_with_index_other(&mut vec, i, ErrorCode::TaskNotFound);
                }
            };
        }
        
        // Write overall operation success
        reply.write(&(ErrorCode::ErrOk as i32))?;
        
        // Write individual results for each task
        for (c, info) in vec {
            reply.write(&(c as i32))?;
            // TODO: Sends info only when ErrOk.
            serialize_task_info(info, reply)?;
        }
        Ok(())
    }
}
