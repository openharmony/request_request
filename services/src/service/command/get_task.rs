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

//! Implements task retrieval functionality for the request service.
//! 
//! This module provides methods to retrieve task configurations and subscribe clients
//! to receive updates about specific download/upload tasks.

use ipc::parcel::MsgParcel;
use ipc::{IpcResult, IpcStatusCode};

use crate::error::ErrorCode;
use crate::manage::query;
use crate::service::{serialize_task_config, RequestServiceStub};

impl RequestServiceStub {
    /// Retrieves task configuration and subscribes client to task updates.
    ///
    /// Reads a task ID and token from the input parcel, validates them,
    /// retrieves the corresponding task configuration, and subscribes the calling
    /// client to receive updates for this task.
    ///
    /// # Arguments
    ///
    /// * `data` - Input parcel containing the task ID and token.
    /// * `reply` - Output parcel to write the result code and task configuration.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the task was successfully retrieved and subscription completed.
    /// * `Err(IpcStatusCode::Failed)` - If the task ID is invalid or the task was not found.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The task ID cannot be parsed as a valid u32.
    /// * The task does not exist or the token is invalid.
    /// * The calling UID does not have permission to access the task.
    /// * Writing to the reply parcel fails.
    ///
    /// # Notes
    ///
    /// Even if subscription fails with a non-critical error, the task configuration
    /// is still returned to the client.
    pub(crate) fn get_task(&self, data: &mut MsgParcel, reply: &mut MsgParcel) -> IpcResult<()> {
        // Read task ID from input parcel
        let task_id: String = data.read()?;
        info!("Service getTask tid {}", task_id);

        // Validate and parse task ID as a 32-bit integer
        let Ok(task_id) = task_id.parse::<u32>() else {
            error!(
                "End Service getTask, tid: {}, failed: task_id or token not valid",
                task_id
            );
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A24,
                &format!("End Service getTask, tid: {}, failed: task_id or token not valid", task_id)
            );
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        };

        // Verify calling process has permission to access this task
        let uid = ipc::Skeleton::calling_uid();

        if !self.check_task_uid(task_id, uid) {
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        }

        // Read authentication token and retrieve task configuration
        let token: String = data.read()?;
        let Some(config) = query::get_task(task_id, token) else {
            error!(
                "End Service getTask, tid: {}, failed: task_id or token not found",
                task_id
            );
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A24,
                &format!("End Service getTask, tid: {}, failed: task_id or token not found", task_id)
            );
            reply.write(&(ErrorCode::TaskNotFound as i32))?;
            return Err(IpcStatusCode::Failed);
        };

        // Subscribe client to receive task updates
        let token_id = ipc::Skeleton::calling_full_token_id();
        let pid = ipc::Skeleton::calling_pid();

        // Register client subscription with task manager
        let ret = self.client_manager.subscribe(task_id, pid, uid, token_id);
        if ret != ErrorCode::ErrOk {
            error!("End Service subscribe, tid: {}, failed: {:?}", task_id, ret);
            sys_event!(
                ExecError,
                DfxCode::INVALID_IPC_MESSAGE_A24,
                &format!("End Service subscribe, tid: {}, failed: {:?}", task_id, ret)
            );
            reply.write(&(ret as i32))?;
            serialize_task_config(config, reply)?;
            // Even if subscription fails, return task configuration to client
            return Ok(());
        }

        // Return success code and serialize task configuration
        reply.write(&(ErrorCode::ErrOk as i32))?;
        serialize_task_config(config, reply)?;
        Ok(())
    }
}
