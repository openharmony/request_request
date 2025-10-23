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

//! Notification publishing and dispatching module for download/upload tasks.
//! 
//! This module provides functionality for dispatching various types of notifications
//! related to download and upload tasks, including progress updates, success/failure
//! notifications, and group management. It uses a singleton pattern to maintain a
//! central notification dispatcher.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, Mutex};

use ylong_runtime::fastrand::fast_random;
use ylong_runtime::sync::mpsc::{self, unbounded_channel};

use super::database::NotificationDb;
use super::notify_flow::{EventualNotify, NotifyFlow, NotifyInfo, ProgressNotify};
use super::task_handle::{cancel_notification, NotificationCheck};
use crate::info::TaskInfo;
use crate::service::notification_bar::NotificationConfig;
use crate::task::request_task::RequestTask;
use crate::utils::get_current_duration;

/// Interval in milliseconds between progress notifications (500 ms).
pub(crate) const NOTIFY_PROGRESS_INTERVAL: u64 = 500;

/// Central dispatcher for managing and publishing task notifications.
/// 
/// This struct serves as a singleton and provides functionality to register tasks,
/// dispatch notifications, and manage notification groups for download/upload tasks.
pub struct NotificationDispatcher {
    /// Database for storing notification configuration and task information.
    database: Arc<NotificationDb>,
    /// Map of task IDs to notification visibility flags (gauge).
    task_gauge: Mutex<HashMap<u32, Arc<AtomicBool>>>,
    /// Channel for sending notification information to the notification flow.
    flow: mpsc::UnboundedSender<NotifyInfo>,
}

impl NotificationDispatcher {
    /// Creates a new NotificationDispatcher instance.
    /// 
    /// Initializes the database, creates a notification channel, and starts the
    /// notification flow for processing notifications asynchronously.
    fn new() -> Self {
        // Create notification database
        let database = Arc::new(NotificationDb::new());
        // Set up channel for notification messages
        let (tx, rx) = unbounded_channel();
        // Start notification flow processor
        NotifyFlow::new(rx, database.clone()).run();
        
        Self {
            database: database.clone(),
            task_gauge: Mutex::new(HashMap::new()),
            flow: tx,
        }
    }

    /// Returns the singleton instance of NotificationDispatcher.
    /// 
    /// Uses LazyLock for thread-safe initialization of the singleton instance.
    /// 
    /// # Returns
    /// 
    /// Static reference to the singleton NotificationDispatcher instance
    pub(crate) fn get_instance() -> &'static Self {
        static INSTANCE: LazyLock<NotificationDispatcher> = 
            LazyLock::new(NotificationDispatcher::new);
        &INSTANCE
    }

    /// Clears notification information for a specific task.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - ID of the task whose notification information should be cleared
    pub(crate) fn clear_task_info(&self, task_id: u32) {
        self.database.clear_task_info(task_id);
    }

    /// Clears group notification information that is older than a week.
    /// 
    /// This method helps maintain database size by removing outdated group information
    /// automatically.
    pub(crate) fn clear_group_info(&self) {
        self.database.clear_group_info_a_week_ago();
    }

    /// Disables notifications for a specific task.
    /// 
    /// # Arguments
    /// 
    /// * `uid` - User ID associated with the task
    /// * `task_id` - ID of the task to disable notifications for
    pub(crate) fn disable_task_notification(&self, uid: u64, task_id: u32) {
        self.database.disable_task_notification(task_id);
        self.unregister_task(uid, task_id, true);
    }

    /// Enables progress notifications for a specific task.
    /// 
    /// Sets the notification gauge flag to true if the task is registered.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - ID of the task to enable progress notifications for
    pub(crate) fn enable_task_progress_notification(&self, task_id: u32) {
        if let Some(gauge) = self.task_gauge.lock().unwrap().get(&task_id) {
            gauge.store(true, Ordering::Release);
        }
    }

    /// Updates customized notification configuration for a task.
    /// 
    /// # Arguments
    /// 
    /// * `config` - New notification configuration to apply
    pub(crate) fn update_task_customized_notification(&self, config: &NotificationConfig) {
        self.database.update_task_customized_notification(config);
    }

    /// Checks if notifications are available for a specific task.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - ID of the task to check
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether notifications are available for the task
    pub(crate) fn check_task_notification_available(&self, task_id: u32) -> bool {
        self.database.check_task_notification_available(&task_id)
    }

    /// Gets the notification visibility flag (gauge) for a specific task.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - ID of the task to get the gauge for
    /// 
    /// # Returns
    /// 
    /// `Some(bool)` with the gauge value if the task exists, `None` otherwise
    pub(crate) fn get_task_gauge(&self, task_id: u32) -> Option<bool> {
        self.task_gauge.lock().ok()?.get(&task_id).map(|gauge| gauge.load(Ordering::Acquire))
    }

    /// Registers a task for notification tracking.
    /// 
    /// Creates a notification visibility gauge for the task based on its group configuration
    /// or individual settings.
    /// 
    /// # Arguments
    /// 
    /// * `task` - Reference to the task to register for notifications
    /// 
    /// # Returns
    /// 
    /// Arc<AtomicBool> representing the notification visibility gauge for the task
    pub(crate) fn register_task(&self, task: &RequestTask) -> Arc<AtomicBool> {
        // Determine gauge value based on group membership or individual task settings
        let gauge = if let Some(gid) = self.database.query_task_gid(task.task_id()) {
            if self.database.check_group_notification_available(&gid) {
                Arc::new(AtomicBool::new(true))
            } else {
                Arc::new(AtomicBool::new(false))
            }
        } else {
            let gauge = task.notification_check(&self.database);
            Arc::new(AtomicBool::new(gauge))
        };
        
        // Store gauge in the task map
        self.task_gauge
            .lock()
            .unwrap()
            .insert(task.task_id(), gauge.clone());
        gauge
    }

    /// Unregisters a task from notification tracking.
    /// 
    /// Stops notifications for the specified task and optionally affects group notifications.
    /// 
    /// # Arguments
    /// 
    /// * `uid` - User ID associated with the task
    /// * `task_id` - ID of the task to unregister
    /// * `affect_group` - Whether to update the task's group notification state
    pub(crate) fn unregister_task(&self, uid: u64, task_id: u32, affect_group: bool) {
        match (
            self.task_gauge.lock().unwrap().get(&task_id).cloned(),
            self.database.query_task_gid(task_id),
        ) {
            (Some(gauge), Some(gid)) => {
                if affect_group {
                    gauge.store(false, Ordering::Release);
                    let _ = self.flow.send(NotifyInfo::Unregister(uid, task_id, gid));
                }
            }
            (None, Some(gid)) => {
                if affect_group {
                    let _ = self.flow.send(NotifyInfo::Unregister(uid, task_id, gid));
                }
            }
            (Some(gauge), None) => {
                gauge.store(false, Ordering::Release);
                cancel_notification(task_id);
            }
            (None, None) => {}
        }
    }

    /// Publishes a progress notification for a task.
    /// 
    /// Creates and sends a progress notification with current task status information.
    /// Handles multi-file uploads and calculates total size when available.
    /// 
    /// # Arguments
    /// 
    /// * `task` - Reference to the task whose progress should be published
    pub(crate) fn publish_progress_notification(&self, task: &RequestTask) {
        let progress = task.progress.lock().unwrap();
        let mut total = Some(0);
        
        // Calculate total size if all sizes are non-negative
        for size in progress.sizes.iter() {
            if *size < 0 {
                total = None;  // Unknown total size
                break;
            }
            *total.as_mut().unwrap() += *size as u64;
        }
        
        // Determine if this is a multi-file upload
        let multi_upload = match progress.sizes.len() {
            0 | 1 => None,
            len => Some((progress.common_data.index, len)),
        };
        
        // Create progress notification
        let notify = ProgressNotify {
            action: task.action(),
            task_id: task.task_id(),
            uid: task.uid(),
            file_name: match task.conf.file_specs.first() {
                Some(spec) => spec.file_name.clone(),
                None => {
                    error!("Failed to get the first file_spec from an empty vector in TaskConfig");
                    String::new()
                }
            },
            processed: progress.common_data.total_processed as u64,
            total,
            multi_upload,
            version: task.conf.version,
        };
        
        // Send notification through the channel
        let _ = self.flow.send(NotifyInfo::Progress(notify));
    }

    /// Publishes a success notification when a task completes successfully.
    /// 
    /// Removes the task from notification tracking and sends a success notification
    /// if notifications are available for this task.
    /// 
    /// # Arguments
    /// 
    /// * `info` - Reference to the completed task information
    pub(crate) fn publish_success_notification(&self, info: &TaskInfo) {
        // Remove task from gauge map as it's completed
        self.task_gauge
            .lock()
            .unwrap()
            .remove(&info.common_data.task_id);
            
        // Only send notification if task notifications are available
        if !info.notification_check(&self.database) {
            return;
        }
        
        // Create success notification
        let notify = EventualNotify {
            action: info.action(),
            task_id: info.common_data.task_id,
            processed: info.progress.common_data.total_processed as u64,
            uid: info.uid(),
            file_name: match info.file_specs.first() {
                Some(spec) => spec.file_name.clone(),
                None => {
                    error!("Failed to get the first file_spec from an empty vector in TaskInfo");
                    String::new()
                }
            },
            is_successful: true,
        };
        
        // Send notification through the channel
        let _ = self.flow.send(NotifyInfo::Eventual(notify));
    }

    /// Publishes a failure notification when a task fails to complete.
    /// 
    /// Removes the task from notification tracking and sends a failure notification
    /// if notifications are available for this task.
    /// 
    /// # Arguments
    /// 
    /// * `info` - Reference to the failed task information
    pub(crate) fn publish_failed_notification(&self, info: &TaskInfo) {
        // Remove task from gauge map as it's completed (failed)
        self.task_gauge
            .lock()
            .unwrap()
            .remove(&info.common_data.task_id);
            
        // Only send notification if task notifications are available
        if !info.notification_check(&self.database) {
            return;
        }
        
        // Create failure notification
        let notify = EventualNotify {
            action: info.action(),
            task_id: info.common_data.task_id,
            processed: info.progress.common_data.total_processed as u64,
            uid: info.uid(),
            file_name: match info.file_specs.first() {
                Some(spec) => spec.file_name.clone(),
                None => {
                    error!("Failed to get the first file_spec from an empty vector in TaskInfo");
                    String::new()
                }
            },
            is_successful: false,
        };
        
        // Send notification through the channel
        let _ = self.flow.send(NotifyInfo::Eventual(notify));
    }

    /// Attaches multiple tasks to a notification group.
    /// 
    /// Updates the notification gauge for all specified tasks to match the group's visibility setting
    /// and sends a notification to update the group.
    /// 
    /// # Arguments
    /// 
    /// * `task_ids` - List of task IDs to attach to the group
    /// * `group_id` - ID of the target group
    /// * `uid` - User ID associated with the tasks
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether the operation was successful
    pub(crate) fn attach_group(&self, task_ids: Vec<u32>, group_id: u32, uid: u64) -> bool {
        if !self.database.attach_able(group_id) {
            return false;
        }
        info!("Attach task {:?} to group {}", task_ids, group_id);
        let is_gauge = self.database.is_gauge(group_id);
        
        // Update each task's group and gauge setting
        for task_id in task_ids.iter().copied() {
            self.database.update_task_group(task_id, group_id);
            if let Some(gauge) = self.task_gauge.lock().unwrap().get(&task_id) {
                gauge.store(is_gauge, std::sync::atomic::Ordering::Release);
            }
        }
        
        // Only send notification if group notifications are available
        if !self.database.check_group_notification_available(&group_id) {
            return true;
        }

        let _ = self
            .flow
            .send(NotifyInfo::AttachGroup(group_id, uid, task_ids));
        true
    }

    /// Deletes a notification group and disables further attachments.
    /// 
    /// Marks the group as disabled and sends a notification to update the UI.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - ID of the group to delete
    /// * `uid` - User ID associated with the group
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether the operation was successful
    pub(crate) fn delete_group(&self, group_id: u32, uid: u64) -> bool {
        info!("Delete group {}", group_id);
        // Check if group exists and can be deleted
        if !self.database.attach_able(group_id) {
            return false;
        }
        
        // Disable the group to prevent further attachments
        self.database.disable_attach_group(group_id);
        
        // Only send notification if group notifications are available
        if !self.database.check_group_notification_available(&group_id) {
            return true;
        }
        
        // Send group deletion notification
        let notify = NotifyInfo::GroupEventual(group_id, uid);
        let _ = self.flow.send(notify);
        true
    }

    /// Creates a new notification group with the specified configuration.
    /// 
    /// Generates a unique group ID, stores the group configuration in the database,
    /// and optionally sets up customized notification text.
    /// 
    /// # Arguments
    /// 
    /// * `gauge` - Whether to show progress gauge in notifications
    /// * `title` - Optional custom title for the group notification
    /// * `text` - Optional custom text for the group notification
    /// * `want_agent` - Optional agent identifier
    /// * `disable` - Whether to disable notifications for this group
    /// * `visibility` - Visibility level of the notifications
    /// 
    /// # Returns
    /// 
    /// The newly created group ID
    pub(crate) fn create_group(
        &self,
        gauge: bool,
        title: Option<String>,
        text: Option<String>,
        want_agent: Option<String>,
        disable: bool,
        visibility: u32,
    ) -> u32 {
        // Generate a unique group ID using random number generation
        let new_group_id = loop {
            let candidate = fast_random() as u32;
            if !self.database.contains_group(candidate) {
                break candidate;
            }
        };
        
        info!(
            "Create group {} gauge {} customized_title {:?} customized_text {:?} want_agent {:?} disable {} visibility {}",
            new_group_id, gauge, title, text, want_agent, disable, visibility
        );

        // Get current time for group creation timestamp
        let current_time = get_current_duration().as_millis() as u64;
        
        // Store group configuration
        self.database
            .update_group_config(new_group_id, gauge, current_time, !disable, visibility);
            
        // Set up customized notification if provided
        if title.is_some() || text.is_some() || want_agent.is_some() {
            self.database
                .update_group_customized_notification(new_group_id, title, text, want_agent);
        }
        
        new_group_id
    }
}
