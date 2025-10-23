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

//! Task information dumping utilities for the request service.
//! 
//! This module provides functionality to dump task information to a file, supporting both
//! summary views of all tasks and detailed views of specific tasks.

use std::fs::File;
use std::io::Write;

use ipc::IpcResult;

use crate::manage::events::TaskManagerEvent;
use crate::service::RequestServiceStub;

/// Help message displayed when the dump command is used incorrectly or with `-h` flag.
const HELP_MSG: &str = "usage:\n\
                         -h                    help text for the tool\n\
                         -t [taskid]           without taskid: display all task summary info; \
                         taskid: display one task detail info\n";
impl RequestServiceStub {
    /// Dumps task information to a file based on provided arguments.
    ///
    /// # Arguments
    ///
    /// * `file` - File to write the task information to.
    /// * `args` - Command-line arguments specifying what information to dump.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the dump operation completes successfully.
    ///
    /// # Notes
    ///
    /// Ignores all file I/O errors silently. Supports the following argument patterns:
    /// - `-h`: Display help message
    /// - `-t`: Dump summary information for all tasks
    /// - `-t [taskid]`: Dump detailed information for a specific task
    pub(crate) fn dump(&self, mut file: File, args: Vec<String>) -> IpcResult<()> {
        info!("Service dump");

        let len = args.len();
        // Display help message if no arguments or `-h` flag is provided
        if len == 0 || args[0] == "-h" {
            let _ = file.write(HELP_MSG.as_bytes());
            return Ok(());
        }

        // Validate that the first argument is `-t`
        if args[0] != "-t" {
            let _ = file.write("invalid args".as_bytes());
            return Ok(());
        }

        // Process based on the number of arguments
        match len {
            // Dump all task information when `-t` is provided without a task ID
            1 => self.dump_all_task_info(file),
            // Dump specific task information when `-t` is followed by a task ID
            2 => {
                let task_id = args[1].parse::<u32>();
                match task_id {
                    Ok(id) => self.dump_one_task_info(file, id),
                    Err(_) => {
                        let _ = file.write("-t accept a number".as_bytes());
                    }
                }
            }
            // Handle too many arguments error
            _ => {
                let _ = file.write("too many args, -t accept no arg or one arg".as_bytes());
            }
        }
        Ok(())
    }

    /// Dumps summary information for all tasks to the provided file.
    ///
    /// # Arguments
    ///
    /// * `file` - File to write the task summary information to.
    ///
    /// # Notes
    ///
    /// Writes a table with columns for task ID, action, state, and reason.
    fn dump_all_task_info(&self, mut file: File) {
        info!("Service dump all task info");

        // Create event to request all task information
        let (event, rx) = TaskManagerEvent::dump_all();
        // Send event to task manager and handle failure
        if !self.task_manager.lock().unwrap().send_event(event) {
            return;
        }

        // Receive task information response
        let infos = match rx.get() {
            Some(infos) => infos,
            None => {
                error!("Service dump: receives infos failed");
                sys_event!(ExecFault, DfxCode::UDS_FAULT_03, "Service dump: receives infos failed");
                return;
            }
        };
        // Write task count and formatted table of task information
        let len = infos.vec.len();
        let _ = file.write(format!("task num: {}\n", len).as_bytes());
        if len > 0 {
            // Write table header
            let _ = file.write(
                format!(
                    "{:<20}{:<12}{:<12}{:<12}\n",
                    "id", "action", "state", "reason"
                )
                .as_bytes(),
            );
            // Write each task's information in a formatted row
            for info in infos.vec.iter() {
                let _ = file.write(
                    format!(
                        "{:<20}{:<12}{:<12}{:<12}\n",
                        info.task_id, info.action.repr, info.state.repr, info.reason.repr
                    )
                    .as_bytes(),
                );
            }
        }
    }

    /// Dumps detailed information for a specific task to the provided file.
    ///
    /// # Arguments
    ///
    /// * `file` - File to write the task information to.
    /// * `task_id` - ID of the task to retrieve information for.
    fn dump_one_task_info(&self, mut file: File, task_id: u32) {
        info!("Service dump one task info");

        // Create event to request information for a specific task
        let (event, rx) = TaskManagerEvent::dump_one(task_id);
        // Send event to task manager and handle failure
        if !self.task_manager.lock().unwrap().send_event(event) {
            return;
        }
        // Receive task information response
        let task = match rx.get() {
            Some(task) => task,
            None => {
                error!("Service dump: receives task failed");
                sys_event!(ExecFault, DfxCode::UDS_FAULT_03, "Service dump: receives task failed");
                return;
            }
        };

        // Write task information if found, otherwise show error message
        if let Some(task) = task {
            // Write table header
            let _ = file.write(
                format!(
                    "{:<20}{:<12}{:<12}{:<12}\n",
                    "id", "action", "state", "reason"
                )
                .as_bytes(),
            );
            // Write the task's information in a formatted row
            let _ = file.write(
                format!(
                    "{:<20}{:<12}{:<12}{:<12}\n",
                    task.task_id, task.action.repr, task.state.repr, task.reason.repr
                )
                .as_bytes(),
            );
        } else {
            // Handle case where task ID is invalid
            let _ = file.write(format!("invalid task id {}", task_id).as_bytes());
        }
    }
}
