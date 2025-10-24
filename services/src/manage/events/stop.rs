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

//! Task stop implementation for the task manager.
//! 
//! This module provides the implementation for stopping tasks within the `TaskManager`. It handles
//! task count decrementation for active tasks and delegates the actual task termination to the
//! scheduler component.

use crate::error::ErrorCode;
use crate::info::State;
use crate::manage::database::RequestDb;
use crate::manage::TaskManager;

impl TaskManager {
    /// Stops a task with the specified user ID and task ID.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID that owns the task.
    /// * `task_id` - The ID of the task to stop.
    ///
    /// # Returns
    ///
    /// * `ErrorCode::ErrOk` - If the task was successfully stopped.
    /// * Other `ErrorCode` values - If there was an error stopping the task.
    ///
    /// # Notes
    ///
    /// This method decrements the task count only for tasks in active states (Running, Retrying,
    /// or Waiting). The count is decremented from the appropriate slot based on the task's mode.
    /// After updating the count, it delegates the actual task termination to the scheduler.
    pub(crate) fn stop(&mut self, uid: u64, task_id: u32) -> ErrorCode {
        // Log the task stop operation for debugging purposes
        debug!("TaskManager stop, tid{}", task_id);
        
        // Get database instance to check task status
        let db = RequestDb::get_instance();
        
        // Check if task exists and update task count if in active state
        if let Some(info) = db.get_task_qos_info(task_id) {
            // Only decrement count for tasks in active states
            if info.state == State::Running.repr
                || info.state == State::Retrying.repr
                || info.state == State::Waiting.repr
            {
                // Update the task count for the user
                if let Some(count) = self.task_count.get_mut(&uid) {
                    // Select the appropriate count slot based on task mode
                    let count = match info.mode {
                        1 => &mut count.0, // Mode 1 uses the first count slot
                        _ => &mut count.1, // All other modes use the second slot
                    };
                    
                    // Ensure count doesn't go negative
                    if *count > 0 {
                        *count -= 1;
                    }
                }
            }
        }

        // Delegate to the scheduler to actually stop the task execution
        match self.scheduler.stop_task(uid, task_id) {
            Ok(_) => ErrorCode::ErrOk,
            Err(e) => e,
        }
    }
}
