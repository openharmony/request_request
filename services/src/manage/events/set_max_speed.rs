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

//! Task speed limit implementation for the task manager.
//! 
//! This module provides the implementation for setting maximum speed limits on tasks within the
//! `TaskManager`. It handles database updates and delegates the actual speed limit enforcement
//! to the scheduler component.

use crate::error::ErrorCode;
use crate::info::State;
use crate::manage::database::RequestDb;
use crate::manage::TaskManager;

impl TaskManager {
    /// Sets a maximum speed limit for a task with the specified user ID and task ID.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID that owns the task.
    /// * `task_id` - The ID of the task to set speed limit for.
    /// * `max_speed` - The maximum speed limit to set, in bytes per second.
    ///
    /// # Returns
    ///
    /// * `ErrorCode::ErrOk` - If the speed limit was successfully set.
    /// * `ErrorCode::TaskStateErr` - If the task is in a removed state or does not exist.
    /// * Other `ErrorCode` values - If there was an error setting the speed limit.
    ///
    /// # Notes
    ///
    /// This method first checks if the task exists and is not in a removed state. If valid,
    /// it updates the task's maximum speed in the database and delegates to the scheduler
    /// to apply the speed limit.
    pub(crate) fn set_max_speed(&mut self, uid: u64, task_id: u32, max_speed: i64) -> ErrorCode {
        // Log the set_max_speed operation for debugging purposes
        debug!(
            "TaskManager set_max_speed, uid{}, tid{}, max_speed{}",
            uid, task_id, max_speed
        );

        // Get database instance to check task status and update speed limit
        let db = RequestDb::get_instance();
        
        // Check if task exists and is in a valid state for speed limit changes
        if let Some(info) = db.get_task_qos_info(task_id) {
            // Reject speed limit changes for removed tasks
            if info.state == State::Removed.repr {
                return ErrorCode::TaskStateErr;
            }
            
            // Update the speed limit in the database
            db.update_task_max_speed(task_id, max_speed);
        } else {
            // Return error if task doesn't exist
            return ErrorCode::TaskStateErr;
        }

        // Delegate to the scheduler to apply the speed limit
        match self.scheduler.set_max_speed(uid, task_id, max_speed) {
            Ok(_) => ErrorCode::ErrOk,
            Err(e) => e,
        }
    }
}
