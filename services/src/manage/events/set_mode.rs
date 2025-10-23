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

//! Task mode configuration implementation for the task manager.
//! 
//! This module provides the implementation for changing the execution mode of tasks within the
//! `TaskManager`. It delegates the mode change operation to the scheduler component.

use crate::config::Mode;
use crate::error::ErrorCode;
use crate::manage::TaskManager;

impl TaskManager {
    /// Sets the execution mode for a task with the specified user ID and task ID.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID that owns the task.
    /// * `task_id` - The ID of the task to set mode for.
    /// * `mode` - The execution mode to set for the task.
    ///
    /// # Returns
    ///
    /// * `ErrorCode::ErrOk` - If the mode was successfully set.
    /// * Other `ErrorCode` values - If there was an error setting the mode.
    ///
    /// # Notes
    ///
    /// This method delegates the mode change operation to the scheduler component. If the scheduler
    /// encounters an error, that error is propagated back to the caller.
    pub(crate) fn set_mode(&mut self, uid: u64, task_id: u32, mode: Mode) -> ErrorCode {
        // Log the mode change operation for debugging purposes
        debug!("TaskManager change_mode, tid{} mode{:?}", task_id, mode);
        
        // Delegate to the scheduler to change the task mode
        match self.scheduler.task_set_mode(uid, task_id, mode) {
            Ok(_) => ErrorCode::ErrOk,
            Err(e) => e,
        }
    }
}
