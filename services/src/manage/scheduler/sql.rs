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

//! SQL statement generation for task state management.
//! 
//! This module provides functions to generate SQL statements that update task states
//! in the database based on various operations and system events.

use crate::config::{Action, Mode};
use crate::info::State;
use crate::task::reason::Reason;

/// Generates SQL to start a task and transition it to the Waiting state.
///
/// # Arguments
///
/// * `task_id` - The unique identifier of the task to start.
///
/// # Returns
///
/// A SQL UPDATE statement that changes the task state to `Waiting` with
/// appropriate reason code, but only if the task is in an applicable state.
///
/// # Notes
///
/// This function creates SQL that allows starting tasks from:
/// - `Initialized` state
/// - `Paused` state
/// - For download tasks only: `Failed` or `Stopped` state
///
/// This conditional logic ensures tasks can only be started from valid states.
pub(super) fn start_task(task_id: u32) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id = {} AND (state = {} OR state = {} OR (action = {} AND (state = {} OR state = {} )))",
        State::Waiting.repr,
        Reason::RunningTaskMeetLimits.repr,
        task_id,
        State::Initialized.repr,
        State::Paused.repr,
        Action::Download.repr,
        State::Failed.repr,
        State::Stopped.repr,
    )
}

/// Generates SQL to pause a task and transition it to the Paused state.
///
/// # Arguments
///
/// * `task_id` - The unique identifier of the task to pause.
///
/// # Returns
///
/// A SQL UPDATE statement that changes the task state to `Paused` with
/// a user operation reason code.
///
/// # Notes
///
/// This function creates SQL that allows pausing tasks that are in:
/// - `Running` state
/// - `Retrying` state
/// - `Waiting` state
///
/// This ensures tasks can only be paused from active or waiting states.
pub(super) fn pause_task(task_id: u32) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id = {} AND (state = {} OR state = {} OR state = {})",
        State::Paused.repr,
        Reason::UserOperation.repr,
        task_id,
        State::Running.repr,
        State::Retrying.repr,
        State::Waiting.repr,
    )
}

/// Generates SQL to stop a task and transition it to the Stopped state.
///
/// # Arguments
///
/// * `task_id` - The unique identifier of the task to stop.
///
/// # Returns
///
/// A SQL UPDATE statement that changes the task state to `Stopped` with
/// a user operation reason code.
///
/// # Notes
///
/// This function creates SQL that allows stopping tasks that are in:
/// - `Running` state
/// - `Retrying` state
/// - `Waiting` state
///
/// Tasks can only be stopped from active or waiting states, similar to pause operations.
pub(super) fn stop_task(task_id: u32) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id = {} AND (state = {} OR state = {} OR state = {})",
        State::Stopped.repr,
        Reason::UserOperation.repr,
        task_id,
        State::Running.repr,
        State::Retrying.repr,
        State::Waiting.repr,
    )
}

/// Generates SQL to mark a task as removed in the database.
///
/// # Arguments
///
/// * `task_id` - The unique identifier of the task to remove.
///
/// # Returns
///
/// A SQL UPDATE statement that changes the task state to `Removed` with
/// a user operation reason code.
///
/// # Notes
///
/// Unlike other operations, this function allows removing tasks in any state.
/// This is because a removed task is typically scheduled for final cleanup and
/// data removal from the system.
pub(super) fn remove_task(task_id: u32) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id = {}",
        State::Removed.repr,
        Reason::UserOperation.repr,
        task_id,
    )
}

/// Generates SQL to update a task's operation mode.
///
/// # Arguments
///
/// * `task_id` - The unique identifier of the task to update.
/// * `mode` - The new operational mode to set for the task.
///
/// # Returns
///
/// A SQL UPDATE statement that changes the task's mode without affecting
/// its current state or reason.
///
/// # Notes
///
/// This function is specifically for updating the operational mode of a task,
/// which controls aspects like whether it runs only on Wi-Fi or can use mobile data.
pub(super) fn task_set_mode(task_id: u32, mode: Mode) -> String {
    format!(
        "UPDATE request_task SET mode = {} where task_id = {}\n",
        mode.repr, task_id,
    )
}

// Test module included conditionally for unit testing
#[cfg(all(not(feature = "oh"), test))]
mod ut_sql {
    include!("../../../tests/ut/manage/scheduler/ut_sql.rs");
}
