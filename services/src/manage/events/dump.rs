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

//! Task information query and dump functionality for the request service.
//! 
//! This module implements methods for querying task information from the `TaskManager`,
//! providing functionality to retrieve details about single tasks or all active tasks
//! in the system.

use crate::manage::TaskManager;
use crate::task::info::{DumpAllEachInfo, DumpAllInfo, DumpOneInfo};

impl TaskManager {
    /// Queries information for a single task by its task ID.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The unique identifier of the task to query.
    ///
    /// # Returns
    ///
    /// * `Some(DumpOneInfo)` - Task information if a task with the given ID exists.
    /// * `None` - If no task with the given ID is found.
    ///
    /// # Notes
    ///
    /// This method locks the task's status to ensure thread-safe access to its
    /// current state and reason.
    pub(crate) fn query_one_task(&self, task_id: u32) -> Option<DumpOneInfo> {
        // Search for the task with matching ID in the scheduler
        self.scheduler
            .tasks()
            .find(|task| task.task_id() == task_id)
            .map(|task| {
                // Lock the task status to ensure thread-safe read
                let status = task.status.lock().unwrap();
                DumpOneInfo {
                    task_id: task.conf.common_data.task_id,
                    action: task.conf.common_data.action,
                    state: status.state,
                    reason: status.reason,
                }
            })
    }

    /// Queries information for all currently active tasks.
    ///
    /// # Returns
    ///
    /// A `DumpAllInfo` containing details for each active task in the system.
    ///
    /// # Notes
    ///
    /// This method locks each task's status individually to ensure thread-safe access
    /// to their current state and reason information.
    pub(crate) fn query_all_task(&self) -> DumpAllInfo {
        DumpAllInfo {
            // Map each task to its dump information
            vec: self
                .scheduler
                .tasks()
                .map(|task| {
                    // Lock each task's status individually for thread-safe read
                    let status = task.status.lock().unwrap();
                    DumpAllEachInfo {
                        task_id: task.conf.common_data.task_id,
                        action: task.conf.common_data.action,
                        state: status.state,
                        reason: status.reason,
                    }
                })
                .collect(),
        }
    }
}
