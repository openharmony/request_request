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

//! MIME type query functionality for download tasks.
//! 
//! This module provides methods to query the MIME type of a specific task,
//! including permission checks, task ownership verification, and MIME type retrieval.

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::manage::query;
use crate::service::permission::PermissionChecker;
use crate::service::RequestServiceStub;

impl RequestServiceStub {
    /// Queries the MIME type of a specified task.
    ///
    /// # Arguments
    ///
    /// * `data` - Message parcel containing the task ID to query
    /// * `reply` - Message parcel to write the result to
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the MIME type was successfully queried and returned
    /// * `Err(IpcStatusCode::Failed)` - If there was an error in the process
    ///
    /// # Errors
    ///
    /// * `ErrorCode::Permission` - When the caller lacks required permissions
    /// * `ErrorCode::TaskNotFound` - When the task ID is invalid or not owned by the caller
    ///
    /// # Notes
    ///
    /// This function requires either INTERNET permission or download manager permission.
    pub(crate) fn query_mime_type(
        &self,
        data: &mut MsgParcel,
        reply: &mut MsgParcel,
    ) -> IpcResult<()> {
        // Check for required permissions (INTERNET or download manager)
        let permission = PermissionChecker::check_down_permission();
        if !PermissionChecker::check_internet() && !permission {
            error!("Service query mime type: no INTERNET permission");
            // Log system event for permission failure
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A07,
                "Service query mime type: no INTERNET permission"
            );
            reply.write(&(ErrorCode::Permission as i32))?;
            return Err(IpcStatusCode::Failed);
        }
        
        // Read and log the task ID from the incoming parcel
        let task_id: String = data.read()?;
        info!("Service query mime type tid {}", task_id);

        // Validate and convert task ID to integer format
        let Ok(task_id) = task_id.parse::<u32>() else {
            error!("End Service query mime type, failed: task_id not valid");
            // Log system event for invalid task ID format
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A08,
                "End Service query mime type, failed: task_id not valid"
            );
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        };

        // Verify task ownership by checking UID
        let uid = ipc::Skeleton::calling_uid();
        if !self.check_task_uid(task_id, uid) {
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        // Retrieve MIME type from the query module
        let mime = query::query_mime_type(uid, task_id);

        // Send successful response with MIME type
        reply.write(&(ErrorCode::ErrOk as i32))?;
        reply.write(&mime)?;
        Ok(())
    }
}
