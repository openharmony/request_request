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

//! Task resume implementation for the task manager.
//! 
//! This module provides the implementation for resuming paused tasks within the `TaskManager`. It handles
//! the resume operation by delegating to the scheduler component.

use crate::error::ErrorCode;
use crate::manage::TaskManager;

impl TaskManager {
    /// Resumes a paused task with the specified user ID and task ID.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID that owns the task.
    /// * `task_id` - The ID of the task to resume.
    ///
    /// # Returns
    ///
    /// * `ErrorCode::ErrOk` - If the task was successfully resumed.
    /// * Other `ErrorCode` values - If there was an error resuming the task.
    ///
    /// # Notes
    ///
    /// This method delegates the resume operation to the scheduler component. If the scheduler
    /// encounters an error, that error is propagated back to the caller.
    pub(crate) fn resume(&mut self, uid: u64, task_id: u32) -> ErrorCode {
        // Log the resume operation for debugging purposes
        debug!("TaskManager resume, uid{} tid{}", uid, task_id);
        
        // Delegate to the scheduler to resume the task and handle any errors
        match self.scheduler.resume_task(uid, task_id) {
            Ok(_) => ErrorCode::ErrOk,
            Err(e) => e,
        }
    }
}
