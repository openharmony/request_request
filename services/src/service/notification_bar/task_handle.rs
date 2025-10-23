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

//! Task handling utilities for notification bar management.
//! 
//! This module provides functionality for handling notification-related tasks,
//! including task cancellation, group attachment, notification checking, and
//! subscription management for the notification bar.

use std::sync::atomic::Ordering;

use super::database::NotificationDb;
use super::ffi::{self, SubscribeNotification};
use super::NotificationDispatcher;
use crate::config::{Mode, Version};
use crate::error::ErrorCode;
use crate::info::{State, TaskInfo};
use crate::manage::database::RequestDb;
use crate::manage::events::TaskManagerEvent;
use crate::manage::task_manager::TaskManagerTx;
use crate::manage::TaskManager;
use crate::task::request_task::RequestTask;
use crate::utils::Recv;

/// Cancels a notification for a specific task.
/// 
/// Sends a request to the FFI layer to cancel a notification with the given request ID.
/// 
/// # Arguments
/// 
/// * `request_id` - ID of the task notification to cancel
pub(super) fn cancel_notification(request_id: u32) {
    info!("cancel notification {}", request_id);
    let ret = ffi::CancelNotification(request_id);
    if ret != 0 {
        error!("cancel notification failed {}", ret);
    }
}

impl TaskManager {
    /// Attaches multiple tasks to a notification group.
    /// 
    /// Validates that all tasks meet the requirements for group attachment and then
    /// attaches them to the specified notification group.
    /// 
    /// # Arguments
    /// 
    /// * `uid` - User ID associated with the tasks
    /// * `task_ids` - List of task IDs to attach to the group
    /// * `group_id` - ID of the target notification group
    /// 
    /// # Returns
    /// 
    /// `ErrorCode::ErrOk` if successful, or an appropriate error code if validation fails
    pub(crate) fn attach_group(&self, uid: u64, task_ids: Vec<u32>, group_id: u32) -> ErrorCode {
        // Validate all tasks meet requirements for group attachment
        for task_id in task_ids.iter().copied() {
            // Check if task exists
            let Some(mode) = RequestDb::get_instance().query_task_mode(task_id) else {
                return ErrorCode::TaskNotFound;
            };
            
            // Verify task is in background mode
            if mode != Mode::BackGround {
                return ErrorCode::TaskModeErr;
            }
            
            // Verify task is in initialized state
            let Some(state) = RequestDb::get_instance().query_task_state(task_id) else {
                return ErrorCode::TaskNotFound;
            };
            if state != State::Initialized.repr {
                return ErrorCode::TaskStateErr;
            }
        }
        
        // Attempt to attach tasks to group
        if !NotificationDispatcher::get_instance().attach_group(task_ids, group_id, uid) {
            return ErrorCode::GroupNotFound;
        }
        
        ErrorCode::ErrOk
    }
}

/// Trait for checking if a task is eligible for notifications.
/// 
/// Implemented for types that need to determine notification availability
/// based on their configuration and state.
pub(crate) trait NotificationCheck {
    /// Checks if notifications are enabled and available for this object.
    /// 
    /// # Arguments
    /// 
    /// * `db` - Reference to the notification database for configuration lookups
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether notifications should be shown
    fn notification_check(&self, db: &NotificationDb) -> bool;
}

impl NotificationCheck for RequestTask {
    /// Checks if notifications should be shown for this RequestTask.
    /// 
    /// Combines configuration checks with database configuration lookup to determine
    /// notification eligibility.
    /// 
    /// # Arguments
    /// 
    /// * `db` - Reference to the notification database
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether notifications should be shown
    fn notification_check(&self, db: &NotificationDb) -> bool {
        let mode = self.mode.load(Ordering::Acquire);
        notification_check_common(
            self.conf.version,
            true,
            Mode::from(mode),
            self.conf.common_data.background,
            false,
        ) && db.check_task_notification_available(&self.conf.common_data.task_id)
    }
}

impl NotificationCheck for TaskInfo {
    /// Checks if notifications should be shown for this TaskInfo.
    /// 
    /// Determines notification eligibility based on task version, gauge setting,
    /// mode, background status, and database configuration.
    /// 
    /// # Arguments
    /// 
    /// * `db` - Reference to the notification database
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether notifications should be shown
    fn notification_check(&self, db: &NotificationDb) -> bool {
        notification_check_common(
            Version::from(self.common_data.version),
            self.common_data.gauge,
            Mode::from(self.common_data.mode),
            RequestDb::get_instance().query_task_background(self.common_data.task_id),
            true,
        ) && db.check_task_notification_available(&self.common_data.task_id)
    }
}

/// Common notification eligibility check for different task types.
/// 
/// Determines if notifications should be shown based on task version, mode,
/// gauge setting, background status, and completion status.
/// 
/// # Arguments
/// 
/// * `version` - API version of the task
/// * `gauge` - Whether the task should show progress gauge notifications
/// * `mode` - Current mode of the task
/// * `background` - Whether the task is running in background
/// * `completed_notify` - Whether the task should show completion notifications
/// 
/// # Returns
/// 
/// Boolean indicating whether notifications should be shown
fn notification_check_common(
    version: Version,
    gauge: bool,
    mode: Mode,
    background: bool,
    completed_notify: bool,
) -> bool {
    // API 10 requirements: background mode and either gauge or completed notification
    // API 9 requirement: background flag only
    version == Version::API10 && mode == Mode::BackGround && (gauge || completed_notify)
        || version == Version::API9 && background
}

/// Wrapper around TaskManager for notification-related task operations.
/// 
/// Provides a simplified interface for performing task operations triggered
/// by notification interactions.
pub struct TaskManagerWrapper {
    /// Channel for sending events to the task manager
    task_manager: TaskManagerTx,
}

impl TaskManagerWrapper {
    /// Creates a new TaskManagerWrapper instance.
    /// 
    /// # Arguments
    /// 
    /// * `task_manager` - Channel for sending events to the task manager
    fn new(task_manager: TaskManagerTx) -> Self {
        Self { task_manager }
    }

    /// Pauses a task.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - ID of the task to pause
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether the operation was successful
    pub(crate) fn pause_task(&self, task_id: u32) -> bool {
        self.event_inner(task_id, TaskManagerEvent::pause)
    }

    /// Resumes a paused task.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - ID of the task to resume
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether the operation was successful
    pub(crate) fn resume_task(&self, task_id: u32) -> bool {
        self.event_inner(task_id, TaskManagerEvent::resume)
    }

    /// Stops a running task.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - ID of the task to stop
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether the operation was successful
    pub(crate) fn stop_task(&self, task_id: u32) -> bool {
        self.event_inner(task_id, TaskManagerEvent::stop)
    }

    /// Internal function for sending task events and handling responses.
    /// 
    /// # Type Parameters
    /// 
    /// * `F` - Function that creates a task event
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - ID of the task to operate on
    /// * `f` - Function that creates a task event with a receive channel for response
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether the operation was successful
    fn event_inner<F>(&self, task_id: u32, f: F) -> bool
    where
        F: Fn(u64, u32) -> (TaskManagerEvent, Recv<ErrorCode>),
    {
        // Get task owner's UID
        let Some(uid) = RequestDb::get_instance().query_task_uid(task_id) else {
            return false;
        };
        
        // Create and send event
        let (event, rx) = f(uid, task_id);
        self.task_manager.send_event(event);
        
        // Wait for and process response
        let Some(ret) = rx.get() else {
            return false;
        };
        if ret != ErrorCode::ErrOk {
            error!("notification_bar {} failed: {}", task_id, ret as u32);
            return false;
        }
        true
    }
}

/// Subscribes to notification bar events and connects them to task management.
/// 
/// Creates a TaskManagerWrapper and registers it with the notification system
/// to handle user interactions with notifications.
/// 
/// # Arguments
/// 
/// * `task_manager` - Channel for sending task management events
pub(crate) fn subscribe_notification_bar(task_manager: TaskManagerTx) {
    SubscribeNotification(Box::new(TaskManagerWrapper::new(task_manager)));
}

impl RequestDb {
    /// Queries whether a task is running in background mode.
    /// 
    /// Executes a SQL query to check the task's background status from the database.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - ID of the task to query
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether the task is running in background
    fn query_task_background(&self, task_id: u32) -> bool {
        let sql = format!(
            "SELECT background FROM request_task WHERE task_id = {}",
            task_id
        );
        self.query_integer(&sql)
            .first()
            .map(|background: &i32| *background == 1)
            .unwrap_or(false)
    }

    /// Queries the mode of a specific task.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - ID of the task to query
    /// 
    /// # Returns
    /// 
    /// `Some(Mode)` with the task's mode if found, `None` otherwise
    pub(crate) fn query_task_mode(&self, task_id: u32) -> Option<Mode> {
        let sql = format!("SELECT mode FROM request_task WHERE task_id = {}", task_id);
        self.query_integer(&sql)
            .first()
            .map(|mode: &i32| Mode::from(*mode as u8))
    }
}
