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

//! Task removal implementation for the task manager.
//! 
//! This module provides the implementation for removing tasks within the `TaskManager`. It handles
//! task count management and delegates the actual removal operation to the scheduler component.

use crate::error::ErrorCode;
use crate::info::State;
use crate::manage::database::RequestDb;
use crate::manage::TaskManager;

impl TaskManager {
    /// Removes a task with the specified user ID and task ID.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID that owns the task.
    /// * `task_id` - The ID of the task to remove.
    ///
    /// # Returns
    ///
    /// * `ErrorCode::ErrOk` - If the task was successfully removed.
    /// * Other `ErrorCode` values - If there was an error removing the task.
    ///
    /// # Notes
    ///
    /// This method handles task count management by decrementing the appropriate task count
    /// if the task is not already in a terminal state (Failed, Completed, Removed). The actual
    /// task removal is delegated to the scheduler component.
    pub(crate) fn remove(&mut self, uid: u64, task_id: u32) -> ErrorCode {
        // Log the remove operation for debugging purposes
        debug!("TaskManager remove,uid{} tid{}", uid, task_id);
        
        // Get database instance to check task status
        let db = RequestDb::get_instance();
        
        // Check if task exists and update task count if necessary
        if let Some(info) = db.get_task_qos_info(task_id) {
            // Only update count for non-terminal states
            if info.state != State::Failed.repr
                && info.state != State::Completed.repr
                && info.state != State::Removed.repr
            {
                // Get the task count for this user and decrement based on mode
                if let Some(count) = self.task_count.get_mut(&uid) {
                    let count = match info.mode {
                        1 => &mut count.0, // First count slot for mode 1
                        _ => &mut count.1, // Second count slot for other modes
                    };
                    // Ensure we don't go below zero
                    if *count > 0 {
                        *count -= 1;
                    }
                }
            }
        }

        // Delegate to the scheduler to remove the task
        match self.scheduler.remove_task(uid, task_id) {
            Ok(_) => ErrorCode::ErrOk,
            Err(e) => e,
        }
    }
}
