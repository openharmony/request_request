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

//! Event system for the task manager.
//! 
//! This module defines the event system used for communication with and within the
//! `TaskManager`. It includes various event types, message structures, and factory methods
//! for creating events that trigger different task management operations.

use std::fmt::Debug;

use ylong_runtime::sync::oneshot::{channel, Sender};

use super::account::AccountEvent;
use crate::config::{Action, Mode};
use crate::error::ErrorCode;
use crate::info::TaskInfo;
use crate::task::config::TaskConfig;
use crate::task::info::{DumpAllInfo, DumpOneInfo};
use crate::task::reason::Reason;
use crate::utils::Recv;

// Event handling implementations for specific operations
mod construct;
mod dump;
mod pause;
mod remove;
mod resume;
mod set_max_speed;
mod set_mode;
mod start;
mod stop;

/// The main event type for the task manager.
///
/// Represents all possible events that can be processed by the `TaskManager`,
/// including service operations, state changes, scheduling events, and task events.
#[derive(Debug)]
pub(crate) enum TaskManagerEvent {
    /// Service-related events for task management operations.
    Service(ServiceEvent),
    /// System state change events.
    State(StateEvent),
    /// Task scheduling events.
    Schedule(ScheduleEvent),
    /// Task-specific events.
    Task(TaskEvent),
    /// Device-related events with event code.
    Device(i32),
    /// Account-related events.
    Account(AccountEvent),
    /// Task information query events.
    Query(QueryEvent),
    /// Trigger to reschedule all tasks.
    Reschedule,
}

impl TaskManagerEvent {
    /// Creates a new event to construct a task with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the new task.
    ///
    /// # Returns
    ///
    /// A tuple containing the event and a receiver for the task ID result.
    pub(crate) fn construct(config: TaskConfig) -> (Self, Recv<Result<u32, ErrorCode>>) {
        // Create channel for async response
        let (tx, rx) = channel::<Result<u32, ErrorCode>>();
        (
            Self::Service(ServiceEvent::Construct(
                Box::new(ConstructMessage { config }),
                tx,
            )),
            Recv::new(rx),
        )
    }

    /// Creates a new event to pause a specific task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID that owns the task.
    /// * `task_id` - The ID of the task to pause.
    ///
    /// # Returns
    ///
    /// A tuple containing the event and a receiver for the operation result.
    pub(crate) fn pause(uid: u64, task_id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::Pause(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    /// Creates a new event to start a specific task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID that owns the task.
    /// * `task_id` - The ID of the task to start.
    ///
    /// # Returns
    ///
    /// A tuple containing the event and a receiver for the operation result.
    pub(crate) fn start(uid: u64, task_id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::Start(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    /// Creates a new event to stop a specific task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID that owns the task.
    /// * `task_id` - The ID of the task to stop.
    ///
    /// # Returns
    ///
    /// A tuple containing the event and a receiver for the operation result.
    pub(crate) fn stop(uid: u64, task_id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::Stop(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    /// Creates a new event to remove a specific task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID that owns the task.
    /// * `task_id` - The ID of the task to remove.
    ///
    /// # Returns
    ///
    /// A tuple containing the event and a receiver for the operation result.
    pub(crate) fn remove(uid: u64, task_id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::Remove(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    /// Creates a new event to resume a specific task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID that owns the task.
    /// * `task_id` - The ID of the task to resume.
    ///
    /// # Returns
    ///
    /// A tuple containing the event and a receiver for the operation result.
    pub(crate) fn resume(uid: u64, task_id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::Resume(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    /// Creates a new event to dump information for all tasks.
    ///
    /// # Returns
    ///
    /// A tuple containing the event and a receiver for the dump information.
    pub(crate) fn dump_all() -> (Self, Recv<DumpAllInfo>) {
        let (tx, rx) = channel::<DumpAllInfo>();
        (Self::Service(ServiceEvent::DumpAll(tx)), Recv::new(rx))
    }

    /// Creates a new event to dump information for a specific task.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of the task to dump information for.
    ///
    /// # Returns
    ///
    /// A tuple containing the event and a receiver for the task information.
    pub(crate) fn dump_one(task_id: u32) -> (Self, Recv<Option<DumpOneInfo>>) {
        let (tx, rx) = channel::<Option<DumpOneInfo>>();
        (
            Self::Service(ServiceEvent::DumpOne(task_id, tx)),
            Recv::new(rx),
        )
    }

    /// Creates a new event to set the mode of a specific task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID that owns the task.
    /// * `task_id` - The ID of the task to modify.
    /// * `mode` - The new mode to set for the task.
    ///
    /// # Returns
    ///
    /// A tuple containing the event and a receiver for the operation result.
    pub(crate) fn set_mode(uid: u64, task_id: u32, mode: Mode) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::SetMode(uid, task_id, mode, tx)),
            Recv::new(rx),
        )
    }

    /// Creates a new event to notify about network state changes.
    ///
    /// # Returns
    ///
    /// The network state change event.
    pub(crate) fn network() -> Self {
        Self::State(StateEvent::Network)
    }

    /// Creates a new event to subscribe to updates for a specific task.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of the task to subscribe to.
    /// * `token_id` - The token ID for subscription identification.
    ///
    /// # Returns
    ///
    /// A tuple containing the event and a receiver for the subscription result.
    pub(crate) fn subscribe(task_id: u32, token_id: u64) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Task(TaskEvent::Subscribe(task_id, token_id, tx)),
            Recv::new(rx),
        )
    }

    /// Creates a new event to attach tasks to a group.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID that owns the tasks.
    /// * `task_ids` - The IDs of the tasks to attach to the group.
    /// * `group_id` - The ID of the group to attach tasks to.
    ///
    /// # Returns
    ///
    /// A tuple containing the event and a receiver for the operation result.
    pub(crate) fn attach_group(
        uid: u64,
        task_ids: Vec<u32>,
        group_id: u32,
    ) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::AttachGroup(uid, task_ids, group_id, tx)),
            Recv::new(rx),
        )
    }

    /// Creates a new event to set the maximum speed for a specific task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID that owns the task.
    /// * `task_id` - The ID of the task to modify.
    /// * `max_speed` - The maximum speed to set in bytes per second.
    ///
    /// # Returns
    ///
    /// A tuple containing the event and a receiver for the operation result.
    pub(crate) fn set_max_speed(uid: u64, task_id: u32, max_speed: i64) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::SetMaxSpeed(uid, task_id, max_speed, tx)),
            Recv::new(rx),
        )
    }
}

/// Events for querying task information.
#[derive(Debug)]
pub(crate) enum QueryEvent {
    /// Query task information by ID and action type.
    Query(u32, Action, Sender<Option<TaskInfo>>),
    /// Show task information by ID and user ID.
    Show(u32, u64, Sender<Option<TaskInfo>>),
    /// Touch (update last access time) and get task information.
    Touch(u32, u64, String, Sender<Option<TaskInfo>>),
}

/// Service operation events for task management.
#[derive(Debug)]
pub(crate) enum ServiceEvent {
    /// Construct a new task with the provided configuration.
    Construct(Box<ConstructMessage>, Sender<Result<u32, ErrorCode>>),
    /// Pause a specific task.
    Pause(u64, u32, Sender<ErrorCode>),
    /// Start a specific task.
    Start(u64, u32, Sender<ErrorCode>),
    /// Stop a specific task.
    Stop(u64, u32, Sender<ErrorCode>),
    /// Remove a specific task.
    Remove(u64, u32, Sender<ErrorCode>),
    /// Resume a specific task.
    Resume(u64, u32, Sender<ErrorCode>),
    /// Dump information for a specific task.
    DumpOne(u32, Sender<Option<DumpOneInfo>>),
    /// Dump information for all tasks.
    DumpAll(Sender<DumpAllInfo>),
    /// Attach multiple tasks to a group.
    AttachGroup(u64, Vec<u32>, u32, Sender<ErrorCode>),
    /// Set maximum speed limit for a specific task.
    SetMaxSpeed(u64, u32, i64, Sender<ErrorCode>),
    /// Set the execution mode for a specific task.
    SetMode(u64, u32, Mode, Sender<ErrorCode>),
}

/// Task state and lifecycle events.
#[derive(Debug)]
pub(crate) enum TaskEvent {
    /// Task has completed successfully.
    Completed(u32, u64, Mode),
    /// Task has failed with the specified reason.
    Failed(u32, u64, Reason, Mode),
    /// Task has gone offline.
    Offline(u32, u64, Mode),
    /// Task is currently running.
    Running(u32, u64, Mode),
    /// Subscribe to updates for a specific task.
    Subscribe(u32, u64, Sender<ErrorCode>),
}

/// System state change events that affect task execution.
#[derive(Debug)]
pub(crate) enum StateEvent {
    /// Network state has changed.
    Network,
    /// Application has moved to the foreground.
    ForegroundApp(u64),
    /// Application has moved to the background.
    Background(u64),
    /// Application has timed out in the background.
    BackgroundTimeout(u64),
    /// Application has been uninstalled.
    AppUninstall(u64),
    /// Application has been terminated specially.
    SpecialTerminate(u64),
}

/// Message containing task configuration for task construction.
pub(crate) struct ConstructMessage {
    /// Configuration details for the task to be constructed.
    pub(crate) config: TaskConfig,
}

impl Debug for ConstructMessage {
    /// Formats the task construction message for debugging.
    ///
    /// # Arguments
    ///
    /// * `f` - Formatter to write the debug representation to.
    ///
    /// # Returns
    ///
    /// Result of the formatting operation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format key task construction details for debugging
        f.debug_struct("Construct")
            .field("uid", &self.config.common_data.uid)
            .field("task_id", &self.config.common_data.task_id)
            .field("title", &self.config.title)
            .field("mode", &self.config.method)
            .field("version", &self.config.version)
            .finish()
    }
}

/// Task scheduling events for managing task lifecycle.
#[derive(Debug)]
pub(crate) enum ScheduleEvent {
    /// Clear tasks that have timed out.
    ClearTimeoutTasks,
    /// Restore all tasks from persistence.
    RestoreAllTasks,
    /// Unload resources but keep the service running.
    Unload,
    /// Shutdown the service completely.
    Shutdown,
}

#[cfg(not(feature = "oh"))]
#[cfg(test)]
mod ut_mod {
    include!("../../../tests/ut/manage/events/ut_mod.rs");
}
