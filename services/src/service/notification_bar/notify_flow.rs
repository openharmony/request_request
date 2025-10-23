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

//! Notification flow management for download tasks.
//! 
//! This module handles the notification lifecycle for download tasks, including
//! progress updates, completion notifications, and group task management. It
//! coordinates between the task system and the notification publishing mechanism,
//! ensuring appropriate notifications are displayed based on task state changes.

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;

use ylong_runtime::sync::mpsc::{self, UnboundedReceiver};

use super::database::{CustomizedNotification, NotificationDb};
use super::ffi::{NotifyContent, PublishNotification};
use super::task_handle::cancel_notification;
use super::NotificationDispatcher;
use crate::config::Action;
use crate::info::State;
use crate::manage::database::RequestDb;
use crate::utils::{get_current_timestamp, runtime_spawn};
use crate::task::config::Version;

/// Minimum interval in milliseconds between progress notifications.
/// 
/// Prevents excessive notification updates for better performance and user experience.
/// Set to 1ms in test mode and 500ms in normal operation.
const NOTIFY_PROGRESS_INTERVAL: u64 = if cfg!(test) { 1 } else { 500 };

/// Manages the notification flow for download tasks and task groups.
/// 
/// Handles notification processing, updates, and publishing based on task events,
/// maintaining state information and visibility settings for all tracked tasks.
pub(crate) struct NotifyFlow {
    database: Arc<NotificationDb>,
    // Maps task IDs to their notification type (group or individual)
    notify_type_map: HashMap<u32, NotifyType>,

    // Tracks last notification time for rate limiting
    last_notify_map: HashMap<u32, u64>,

    // Progress tracking for group notifications
    group_notify_progress: HashMap<u32, GroupProgress>,
    // Customized notification content for groups
    group_customized_notify: HashMap<u32, Option<CustomizedNotification>>,
    // Customized notification content for individual tasks
    task_customized_notify: HashMap<u32, Option<CustomizedNotification>>,
    // Cached visibility settings
    group_progress_visibility: HashMap<u32, bool>,
    group_completion_visibility: HashMap<u32, bool>,
    progress_visibility: HashMap<u32, bool>,
    completion_visibility: HashMap<u32, bool>,
    // Channel for receiving notification events
    rx: mpsc::UnboundedReceiver<NotifyInfo>,
}

pub(crate) struct GroupProgress {
    // Individual task progress tracking
    task_progress: HashMap<u32, u64>,
    // Total processed bytes across all tasks
    total_progress: u64,
    // Current state of each task
    task_state: HashMap<u32, State>,
    // Count of successfully completed tasks
    successful: usize,
    // Count of failed tasks
    failed: usize,
}

impl GroupProgress {
    /// Creates a new empty group progress tracker.
    pub(crate) fn new() -> Self {
        Self {
            task_progress: HashMap::new(),
            total_progress: 0,
            task_state: HashMap::new(),
            successful: 0,
            failed: 0,
        }
    }

    /// Updates the progress for a specific task within the group.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task to update
    /// * `processed` - The new processed byte count
    pub(crate) fn update_task_progress(&mut self, task_id: u32, processed: u64) {
        let prev = match self.task_progress.entry(task_id) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(0),
        };
        // Update total progress by the delta between new and previous values
        self.total_progress += processed - *prev;
        *prev = processed;
    }

    /// Updates the state for a specific task within the group.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task to update
    /// * `state` - The new task state
    pub(crate) fn update_task_state(&mut self, task_id: u32, state: State) {
        let prev = match self.task_state.get_mut(&task_id) {
            Some(prev) => prev,
            None => {
                // First time tracking this task
                self.task_state.insert(task_id, state);
                // Update counters based on initial state
                if state == State::Completed {
                    self.successful += 1;
                } else if state == State::Failed {
                    self.failed += 1;
                }
                return;
            }
        };
        
        // Skip if state hasn't changed
        if *prev == state {
            return;
        }
        
        // Update success/failure counters based on state transition
        if *prev != State::Completed && *prev != State::Failed {
            // Transition from active to terminal state
            if state == State::Completed {
                self.successful += 1;
            } else if state == State::Failed {
                self.failed += 1;
            }
        } else if state == State::Completed {
            // Transition from failed to completed
            self.successful += 1;
            self.failed -= 1;
        } else if state == State::Failed {
            // Transition from completed to failed
            self.failed += 1;
            self.successful -= 1;
        }
        *prev = state;
    }

    /// Returns the number of successfully completed tasks.
    pub(crate) fn successful(&self) -> usize {
        self.successful
    }

    /// Returns the number of failed tasks.
    pub(crate) fn failed(&self) -> usize {
        self.failed
    }

    /// Returns the total number of tasks in the group.
    pub(crate) fn total(&self) -> usize {
        self.task_state.len()
    }
    
    /// Returns the total processed bytes across all tasks.
    pub(crate) fn processed(&self) -> u64 {
        self.total_progress
    }

    /// Checks if all tasks in the group have reached a terminal state.
    /// 
    /// # Returns
    /// 
    /// * `true` - If all tasks are either completed or failed
    /// * `false` - If there are still active tasks in the group
    pub(crate) fn is_finish(&self) -> bool {
        self.total() == self.successful + self.failed
    }
}

#[derive(Clone, Debug)]
pub struct ProgressNotify {
    /// Action type (download or upload)
    pub(crate) action: Action,
    /// Task identifier
    pub(crate) task_id: u32,
    /// User identifier
    pub(crate) uid: u64,
    /// Number of bytes processed
    pub(crate) processed: u64,
    /// Total bytes to process (if available)
    pub(crate) total: Option<u64>,
    /// For multi-file uploads: (current_file_index, total_files)
    pub(crate) multi_upload: Option<(usize, usize)>,
    /// Name of the file being downloaded
    pub(crate) file_name: String,
    /// API version in use
    pub(crate) version: Version,
}

#[derive(Clone, Debug)]
pub(crate) struct EventualNotify {
    /// Action type (download or upload)
    pub(crate) action: Action,
    /// Task identifier
    pub(crate) task_id: u32,
    /// User identifier
    pub(crate) uid: u64,
    /// Total bytes processed
    pub(crate) processed: u64,
    /// Name of the file
    pub(crate) file_name: String,
    /// Whether the task completed successfully
    pub(crate) is_successful: bool,
}

#[derive(Debug)]
pub(crate) enum NotifyInfo {
    /// Task completion notification
    Eventual(EventualNotify),
    /// Progress update notification
    Progress(ProgressNotify),
    /// Attach tasks to a group notification
    AttachGroup(u32, u64, Vec<u32>),
    /// Unregister a task from notifications
    Unregister(u64, u32, u32),
    /// Group completion notification
    GroupEventual(u32, u64),
}

#[derive(Clone, Copy)]
enum NotifyType {
    /// Group notification with group ID
    Group(u32),
    /// Individual task notification
    Task,
}

impl NotifyFlow {
    /// Creates a new notification flow manager.
    /// 
    /// # Arguments
    /// 
    /// * `rx` - Receiver channel for notification events
    /// * `database` - Notification database handle
    /// 
    /// # Returns
    /// 
    /// A new `NotifyFlow` instance
    pub(crate) fn new(rx: UnboundedReceiver<NotifyInfo>, database: Arc<NotificationDb>) -> Self {
        Self {
            database,
            notify_type_map: HashMap::new(),
            last_notify_map: HashMap::new(),
            group_notify_progress: HashMap::new(),
            task_customized_notify: HashMap::new(),
            group_customized_notify: HashMap::new(),
            completion_visibility: HashMap::new(),
            progress_visibility: HashMap::new(),
            group_completion_visibility: HashMap::new(),
            group_progress_visibility: HashMap::new(),
            rx,
        }
    }

    /// Starts the notification flow processing loop.
    /// 
    /// Spawns an asynchronous task that processes incoming notification events
    /// and publishes notifications as needed.
    pub(crate) fn run(mut self) {
        runtime_spawn(async move {
            loop {
                let info = match self.rx.recv().await {
                    Ok(message) => message,
                    Err(e) => {
                        error!("Notification flow channel error: {:?}", e);
                        sys_event!(
                            ExecFault,
                            DfxCode::UDS_FAULT_03,
                            &format!("Notification flow channel error: {:?}", e)
                        );
                        continue;
                    }
                };

                if let Some(content) = match info {
                    NotifyInfo::Eventual(info) => self.publish_completed_notify(&info),
                    NotifyInfo::Progress(info) => self.publish_progress_notification(info),
                    NotifyInfo::GroupEventual(group_id, uid) => self.group_eventual(group_id, uid),
                    NotifyInfo::AttachGroup(group_id, uid, task_ids) => {
                        self.attach_group(group_id, task_ids, uid)
                    }
                    NotifyInfo::Unregister(uid, task_id, group_id) => {
                        self.unregister_task(uid, task_id, group_id)
                    }
                } {
                    PublishNotification(&content);
                }
            }
        });
    }

    /// Handles task unregistration from notifications.
    /// 
    /// # Arguments
    /// 
    /// * `uid` - User identifier
    /// * `task_id` - Task to unregister
    /// * `group_id` - Group the task belongs to
    /// 
    /// # Returns
    /// 
    /// * `Some(NotifyContent)` - If a notification should be published after unregistration
    /// * `None` - If no notification is needed
    fn unregister_task(&mut self, uid: u64, task_id: u32, group_id: u32) -> Option<NotifyContent> {
        info!(
            "Unregister task: uid: {}, task_id: {}, group_id: {}",
            uid, task_id, group_id
        );
        let customized = self.group_customized_notify(group_id);
        let is_completion_visible = self.check_completion_visibility_from_group(group_id);
        let progress = match self.group_notify_progress.entry(group_id) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let progress = Self::get_group_progress(&self.database, group_id);
                entry.insert(progress)
            }
        };
        if progress
            .task_state
            .get(&task_id)
            .is_some_and(|state| *state != State::Completed && *state != State::Failed)
        {
            progress.task_state.remove(&task_id);
        }
        if progress.task_state.is_empty() {
            cancel_notification(group_id);
            return None;
        }
        if !Self::group_eventual_check(&self.database, progress, group_id) {
            return None;
        }
        if !is_completion_visible {
            cancel_notification(group_id);
            return None;
        }
        Some(NotifyContent::group_eventual_notify(
            customized,
            Action::Download,
            group_id,
            uid as u32,
            progress.processed(),
            progress.successful() as i32,
            progress.failed() as i32,
        ))
    }

    /// Updates group progress from database for a specific task.
    /// 
    /// # Arguments
    /// 
    /// * `group_progress` - Group progress to update
    /// * `task_id` - Task to update progress for
    fn update_db_task_state_and_progress(group_progress: &mut GroupProgress, task_id: u32) {
        let Some(processed) = RequestDb::get_instance().query_task_total_processed(task_id) else {
            return;
        };
        let Some(state) = RequestDb::get_instance().query_task_state(task_id) else {
            return;
        };
        if state == State::Removed.repr {
            return;
        }
        group_progress.update_task_state(task_id, State::from(state));
        group_progress.update_task_progress(task_id, processed as u64);
    }

    /// Creates a group progress tracker initialized from database data.
    /// 
    /// # Arguments
    /// 
    /// * `database` - Notification database handle
    /// * `group_id` - Group ID to get progress for
    /// 
    /// # Returns
    /// 
    /// A `GroupProgress` instance with current state from database
    fn get_group_progress(database: &NotificationDb, group_id: u32) -> GroupProgress {
        let mut group_progress = GroupProgress::new();
        for task_id in database.query_group_tasks(group_id) {
            Self::update_db_task_state_and_progress(&mut group_progress, task_id);
        }
        group_progress
    }

    /// Attaches tasks to a group for notification tracking.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - Group ID to attach tasks to
    /// * `task_ids` - Tasks to attach to the group
    /// * `uid` - User identifier
    /// 
    /// # Returns
    /// 
    /// * `Some(NotifyContent)` - If a notification should be published after attachment
    /// * `None` - If no notification is needed
    fn attach_group(
        &mut self,
        group_id: u32,
        task_ids: Vec<u32>,
        uid: u64,
    ) -> Option<NotifyContent> {
        let is_progress_visibility_from_group = self.check_progress_visibility_from_group(group_id);
        let customized = self.group_customized_notify(group_id);
        let progress = match self.group_notify_progress.entry(group_id) {
            Entry::Occupied(entry) => {
                let progress = entry.into_mut();
                for task_id in task_ids {
                    Self::update_db_task_state_and_progress(progress, task_id);
                }
                progress
            }
            Entry::Vacant(entry) => {
                let progress = Self::get_group_progress(&self.database, group_id);
                entry.insert(progress)
            }
        };
        if !is_progress_visibility_from_group {
            return None;
        }
        Some(NotifyContent::group_progress_notify(
            customized,
            Action::Download,
            group_id,
            uid as u32,
            progress,
        ))
    }

    /// Checks if completion notifications are visible for a group.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - Group ID to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If completion notifications should be shown
    /// * `false` - If completion notifications should be hidden
    fn check_completion_visibility_from_group(&mut self, group_id: u32) -> bool {
        *self.group_completion_visibility
            .entry(group_id)
            .or_insert_with(|| self.database.is_completion_visible_from_group(group_id))
    }

    /// Checks if progress notifications are visible for a group.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - Group ID to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If progress notifications should be shown
    /// * `false` - If progress notifications should be hidden
    fn check_progress_visibility_from_group(&mut self, group_id: u32) -> bool {
        *self.group_progress_visibility
            .entry(group_id)
            .or_insert_with(|| self.database.is_progress_visible_from_group(group_id))
    }

    /// Checks if completion notifications are visible for a task.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - Task ID to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If completion notifications should be shown
    /// * `false` - If completion notifications should be hidden
    fn check_completion_visibility(&mut self, task_id: u32) -> bool {
        *self.completion_visibility
            .entry(task_id)
            .or_insert_with(|| self.database.is_completion_visible(task_id))
    }

    /// Checks if progress notifications are visible for a task.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - Task ID to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If progress notifications should be shown
    /// * `false` - If progress notifications should be hidden
    fn check_progress_visibility(&mut self, task_id: u32) -> bool {
        *self.progress_visibility
            .entry(task_id)
            .or_insert_with(|| self.database.is_progress_visible(task_id))
    }

    /// Gets customized notification content for a group.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - Group ID to get customized notifications for
    /// 
    /// # Returns
    /// 
    /// Optional customized notification content
    fn group_customized_notify(&mut self, group_id: u32) -> Option<CustomizedNotification> {
        match self.group_customized_notify.entry(group_id) {
            Entry::Occupied(entry) => entry.get().clone(),
            Entry::Vacant(entry) => {
                let customized = self.database.query_group_customized_notification(group_id);
                entry.insert(customized).clone()
            }
        }
    }

    /// Gets customized notification content for a task.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - Task ID to get customized notifications for
    /// 
    /// # Returns
    /// 
    /// Optional customized notification content
    fn task_customized_notify(&mut self, task_id: u32) -> Option<CustomizedNotification> {
        match self.task_customized_notify.entry(task_id) {
            Entry::Occupied(entry) => entry.get().clone(),
            Entry::Vacant(entry) => {
                let customized = self.database.query_task_customized_notification(task_id);
                entry.insert(customized).clone()
            }
        }
    }

    /// Publishes a progress notification for a task.
    /// 
    /// # Arguments
    /// 
    /// * `info` - Progress notification information
    /// 
    /// # Returns
    /// 
    /// * `Some(NotifyContent)` - If a notification should be published
    /// * `None` - If no notification is needed
    fn publish_progress_notification(&mut self, info: ProgressNotify) -> Option<NotifyContent> {
        let content = match self.get_request_id(info.task_id) {
            NotifyType::Group(group_id) => {
                if !self.check_progress_visibility_from_group(group_id) {
                    return None;
                }
                let progress_interval_check = self.progress_interval_check(group_id);

                let customized = self.group_customized_notify(group_id);
                let progress = match self.group_notify_progress.entry(group_id) {
                    Entry::Occupied(entry) => entry.into_mut(),
                    Entry::Vacant(entry) => {
                        let progress = Self::get_group_progress(&self.database, group_id);
                        entry.insert(progress)
                    }
                };
                progress.update_task_progress(info.task_id, info.processed);

                if !progress_interval_check {
                    return None;
                }
                NotifyContent::group_progress_notify(
                    customized,
                    info.action,
                    group_id,
                    info.uid as u32,
                    progress,
                )
            }
            NotifyType::Task => {
                if info.version == Version::API9 {
                    // Get gauge value and return notification content only when gauge is true
                    return NotificationDispatcher::get_instance()
                        .get_task_gauge(info.task_id)
                        .filter(|&gauge| gauge)
                        .map(|_| NotifyContent::task_progress_notify(
                            self.task_customized_notify(info.task_id),
                            &info,
                        ));
                }
                if !self.check_progress_visibility(info.task_id) {
                    return None;
                }
                NotifyContent::task_progress_notify(
                    self.task_customized_notify(info.task_id),
                    &info,
                )
            }
        };
        Some(content)
    }

    /// Checks if enough time has passed since the last notification.
    /// 
    /// # Arguments
    /// 
    /// * `request_id` - Task or group ID to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If enough time has passed (notification should be shown)
    /// * `false` - If not enough time has passed (notification should be skipped)
    fn progress_interval_check(&mut self, request_id: u32) -> bool {
        match self.last_notify_map.entry(request_id) {
            Entry::Occupied(mut entry) => {
                let last_notify = entry.get_mut();
                let current = get_current_timestamp();
                if current < NOTIFY_PROGRESS_INTERVAL + *last_notify {
                    return false;
                }
                *last_notify = current;
                true
            }
            Entry::Vacant(entry) => {
                let last_notify = get_current_timestamp();
                entry.insert(last_notify);
                true
            }
        }
    }

    /// Publishes a completion notification for a task.
    /// 
    /// # Arguments
    /// 
    /// * `info` - Completion notification information
    /// 
    /// # Returns
    /// 
    /// * `Some(NotifyContent)` - If a notification should be published
    /// * `None` - If no notification is needed
    fn publish_completed_notify(&mut self, info: &EventualNotify) -> Option<NotifyContent> {
        let content = match self.get_request_id(info.task_id) {
            NotifyType::Group(group_id) => {
                let is_progress_visible = self.check_progress_visibility_from_group(group_id);
                let is_completion_visible = self.check_completion_visibility_from_group(group_id);

                let customized = self.group_customized_notify(group_id);
                let group_progress = match self.group_notify_progress.entry(group_id) {
                    Entry::Occupied(entry) => {
                        let progress = entry.into_mut();
                        progress.update_task_progress(info.task_id, info.processed);
                        if info.is_successful {
                            progress.update_task_state(info.task_id, State::Completed);
                        } else {
                            progress.update_task_state(info.task_id, State::Failed);
                        }
                        progress
                    }
                    Entry::Vacant(entry) => {
                        let progress = Self::get_group_progress(&self.database, group_id);
                        entry.insert(progress)
                    }
                };

                let group_eventual =
                    Self::group_eventual_check(&self.database, group_progress, group_id);

                match (group_eventual, is_progress_visible) {
                    (false, true) => NotifyContent::group_progress_notify(
                        customized,
                        info.action,
                        group_id,
                        info.uid as u32,
                        group_progress,
                    ),
                    (false, false) => return None,
                    (true, _) => {
                        self.database.clear_group_info(group_id);
                        if !is_completion_visible {
                            cancel_notification(group_id);
                            return None;
                        }
                        NotifyContent::group_eventual_notify(
                            customized,
                            info.action,
                            group_id,
                            info.uid as u32,
                            group_progress.processed(),
                            group_progress.successful() as i32,
                            group_progress.failed() as i32,
                        )
                    }
                }
            }
            NotifyType::Task => {
                if !self.check_completion_visibility(info.task_id) {
                    cancel_notification(info.task_id);
                    return None;
                }
                let content = NotifyContent::task_eventual_notify(
                    self.task_customized_notify(info.task_id),
                    info.action,
                    info.task_id,
                    info.uid as u32,
                    info.file_name.clone(),
                    info.is_successful,
                );
                if info.is_successful {
                    self.database.clear_task_info(info.task_id);
                }
                content
            }
        };
        Some(content)
    }

    /// Handles group completion notification.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - Group ID to process
    /// * `uid` - User identifier
    /// 
    /// # Returns
    /// 
    /// * `Some(NotifyContent)` - If a notification should be published
    /// * `None` - If no notification is needed
    fn group_eventual(&mut self, group_id: u32, uid: u64) -> Option<NotifyContent> {
        let customized = self.group_customized_notify(group_id);
        let is_completion_visible = self.check_completion_visibility_from_group(group_id);
        let group_progress = match self.group_notify_progress.entry(group_id) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let progress = Self::get_group_progress(&self.database, group_id);
                entry.insert(progress)
            }
        };

        let group_eventual = Self::group_eventual_check(&self.database, group_progress, group_id);

        if !group_eventual {
            return None;
        }
        if !is_completion_visible {
            cancel_notification(group_id);
            return None;
        }
        Some(NotifyContent::group_eventual_notify(
            customized,
            Action::Download,
            group_id,
            uid as u32,
            group_progress.processed(),
            group_progress.successful() as i32,
            group_progress.failed() as i32,
        ))
    }

    /// Determines whether a task belongs to a group or is individual.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - Task ID to check
    /// 
    /// # Returns
    /// 
    /// The notification type for this task
    fn get_request_id(&mut self, task_id: u32) -> NotifyType {
        if let Some(n_type) = self.notify_type_map.get(&task_id) {
            return *n_type;
        }
        let n_type = match self.database.query_task_gid(task_id) {
            Some(group_id) => NotifyType::Group(group_id),
            None => NotifyType::Task,
        };

        self.notify_type_map.insert(task_id, n_type);
        n_type
    }

    /// Checks if a group should show a completion notification.
    /// 
    /// # Arguments
    /// 
    /// * `database` - Notification database handle
    /// * `group_progress` - Group progress to check
    /// * `group_id` - Group ID to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If the group should show a completion notification
    /// * `false` - If the group should not show a completion notification
    fn group_eventual_check(
        database: &NotificationDb,
        group_progress: &mut GroupProgress,
        group_id: u32,
    ) -> bool {
        !database.attach_able(group_id) && group_progress.is_finish()
    }
}

#[cfg(test)]
mod ut_notify_flow {
    include!("../../../tests/ut/service/notification_bar/ut_notify_flow.rs");
}
