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

//! Database operations for notification bar management.
//! 
//! This module provides database functionality for storing and managing notification
//! configurations, group settings, and notification content for download tasks.
//! It handles creation, updates, queries, and cleanup operations.

use crate::database::REQUEST_DB;
use crate::service::notification_bar::NotificationConfig;
use super::NotificationDispatcher;

const CREATE_TASK_CONFIG_TABLE: &str = 
    "CREATE TABLE IF NOT EXISTS task_config (task_id INTEGER PRIMARY KEY, display BOOLEAN)";

const CREATE_GROUP_TABLE: &str = 
    "CREATE TABLE IF NOT EXISTS group_notification (task_id INTEGER PRIMARY KEY, group_id INTEGER)";

const CREATE_GROUP_CONFIG_TABLE: &str = 
    "CREATE TABLE IF NOT EXISTS group_notification_config (group_id INTEGER PRIMARY KEY, gauge BOOLEAN, attach_able BOOLEAN, ctime INTEGER)";

const CREATE_TASK_CONTENT_TABLE: &str = 
    "CREATE TABLE IF NOT EXISTS task_notification_content (task_id INTEGER PRIMARY KEY, title TEXT, text TEXT)";

const CREATE_GROUP_CONTENT_TABLE: &str = 
    "CREATE TABLE IF NOT EXISTS group_notification_content (group_id INTEGER PRIMARY KEY, title TEXT, text TEXT)";

const GROUP_CONFIG_TABLE_ADD_DISPLAY: &str = 
    "ALTER TABLE group_notification_config ADD COLUMN display BOOLEAN DEFAULT TRUE";

const GROUP_CONFIG_TABLE_ADD_VISIBILITY: &str = 
    "ALTER TABLE group_notification_config ADD COLUMN visibility INTEGER";

const TASK_CONTENT_TABLE_ADD_VISIBILITY: &str = 
    "ALTER TABLE task_notification_content ADD COLUMN visibility INTEGER";
    
const TASK_CONTENT_TABLE_ADD_WANT_AGENT: &str = 
    "ALTER TABLE task_notification_content ADD COLUMN want_agent TEXT";

const GROUP_CONTENT_TABLE_ADD_WANT_AGENT: &str = 
    "ALTER TABLE group_notification_content ADD COLUMN want_agent TEXT";

use std::time::{SystemTime, UNIX_EPOCH};

const MILLIS_IN_A_WEEK: u64 = 7 * 24 * 60 * 60 * 1000;

/// Notification database handler for managing notification configurations.
/// 
/// This struct provides methods for storing, retrieving, and modifying notification
/// settings for individual tasks and task groups, as well as managing the underlying
/// database schema.
pub(crate) struct NotificationDb {
    inner: &'static rdb::RdbStore<'static>,
}

/// Customized notification content for download tasks or groups.
/// 
/// Stores optional title, text, and want agent information that can be displayed
/// in notification bar items.
#[derive(Default, Clone)]
pub(crate) struct CustomizedNotification {
    pub title: Option<String>,
    pub text: Option<String>,
    pub want_agent: Option<String>,
}

impl NotificationDb {
    /// Creates a new notification database handler and initializes the database.
    /// 
    /// Initializes the database by creating required tables and updating schema
    /// if necessary. Logs errors if initialization fails.
    pub(crate) fn new() -> Self {
        let me = Self { inner: &REQUEST_DB };
        if let Err(e) = me.create_db() {
            error!("Failed to create notification database: {}", e);
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_04,
                &format!("Failed to create notification database: {}", e)
            );
        }

        me.update();
        me
    }

    /// Creates the notification database tables if they don't exist.
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - If all tables are created successfully
    /// * `Err(i32)` - If any table creation fails, with the error code
    fn create_db(&self) -> Result<(), i32> {
        self.inner.execute(CREATE_TASK_CONFIG_TABLE, ())?;
        self.inner.execute(CREATE_GROUP_CONTENT_TABLE, ())?;
        self.inner.execute(CREATE_GROUP_TABLE, ())?;
        self.inner.execute(CREATE_TASK_CONTENT_TABLE, ())?;
        self.inner.execute(CREATE_GROUP_CONFIG_TABLE, ())?;
        Ok(())
    }

    /// Updates the database schema to the latest version.
    /// 
    /// Adds new columns to existing tables if they don't already exist. This ensures
    /// backward compatibility while allowing for schema evolution.
    fn update(&self) {
        // Add display column to group_notification_config table
        if let Err(e) = self.inner.execute(GROUP_CONFIG_TABLE_ADD_DISPLAY, ()) {
            error!("Failed to add display column to group_notification_config table: {}", e);
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_04,
                &format!("Failed to add display column to group_notification_config table: {}", e)
            );
        } else {
            debug!("Successfully added display column to group_notification_config table");
        }
        
        // Add visibility column to task_notification_content table
        if let Err(e) = self.inner.execute(TASK_CONTENT_TABLE_ADD_VISIBILITY, ()) {
            error!("Failed to add visibility column to task_notification_content table: {}", e);
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_04,
                &format!("Failed to add visibility column to task_notification_content table: {}", e)
            );
        } else {
            debug!("Successfully added visibility column to task_notification_content table");
        }
        
        // Add visibility column to group_notification_config table
        if let Err(e) = self.inner.execute(GROUP_CONFIG_TABLE_ADD_VISIBILITY, ()) {
            error!("Failed to add visibility column to group_notification_config table: {}", e);
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_04,
                &format!("Failed to add visibility column to group_notification_config table: {}", e)
            );
        } else {
            debug!("Successfully added visibility column to group_notification_config table");
        }

        // Add want_agent column to task_notification_content table
        if let Err(e) = self.inner.execute(TASK_CONTENT_TABLE_ADD_WANT_AGENT, ()) {
            error!("Failed to add want_agent column to task_notification_content table: {}", e);
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_04,
                &format!("Failed to add want_agent column to task_notification_content table: {}", e)
            );
        } else {
            debug!("Successfully added want_agent column to task_notification_content table");
        }

        // Add want_agent column to group_notification_content table
        if let Err(e) = self.inner.execute(GROUP_CONTENT_TABLE_ADD_WANT_AGENT, ()) {
            error!("Failed to add want_agent column to group_notification_content table: {}", e);
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_04,
                &format!("Failed to add want_agent column to group_notification_content table: {}", e)
            );
        } else {
            debug!("Successfully added want_agent column to group_notification_content table");
        }
    }

    /// Clears all notification information for a specific task.
    /// 
    /// Removes entries from task_config, task_notification_content, and group_notification
    /// tables related to the specified task ID. Logs errors if any deletion fails.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task whose notification information should be cleared
    pub(crate) fn clear_task_info(&self, task_id: u32) {
        let sqls = [
            "DELETE FROM task_config WHERE task_id = ?",
            "DELETE FROM task_notification_content WHERE task_id = ?",
            "DELETE FROM group_notification WHERE task_id = ?",
        ];
        // Execute each delete statement and log any errors
        for sql in sqls.iter() {
            if let Err(e) = self.inner.execute(sql, task_id) {
                error!("Failed to clear task {} notification info: {}", task_id, e);
            }
        }
    }

    /// Clears all notification information for a specific group.
    /// 
    /// Removes entries from group_notification, group_notification_content, and
    /// group_notification_config tables related to the specified group ID.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - The ID of the group whose notification information should be cleared
    pub(crate) fn clear_group_info(&self, group_id: u32) {
        let sqls = [
            "DELETE FROM group_notification WHERE group_id = ?",
            "DELETE FROM group_notification_content WHERE group_id = ?",
            "DELETE FROM group_notification_config WHERE group_id = ?",
        ];
        for sql in sqls.iter() {
            if let Err(e) = self.inner.execute(sql, group_id) {
                error!(
                    "Failed to clear group {} notification info: {}",
                    group_id, e
                );
            }
        }
    }

    /// Clears notification information for groups that haven't been used in over a week.
    /// 
    /// Identifies groups that were created more than one week ago and have no active tasks,
    /// then removes their notification information to free up database space.
    pub(crate) fn clear_group_info_a_week_ago(&self) {
        // Calculate timestamp for one week ago
        let current_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration,
            Err(e) => {
                error!("Failed to get current time: {}", e);
                return;
            }
        }
        .as_millis() as u64;
        
        // Find group IDs that are older than one week
        let group_ids = match self.inner.query::<u32>(
            "SELECT group_id FROM group_notification_config WHERE ctime < ?",
            current_time - MILLIS_IN_A_WEEK,
        ) {
            Ok(rows) => rows,
            Err(e) => {
                error!("Failed to clear group info: {}", e);
                return;
            }
        };
        
        // Only clear groups that have no associated tasks
        for group_id in group_ids {
            let mut count = match self.inner.query::<u32>(
                "SELECT COUNT(*) FROM group_notification WHERE group_id = ?",
                group_id,
            ) {
                Ok(rows) => rows,
                Err(e) => {
                    error!("Failed to clear group info: {}", e);
                    continue;
                }
            };
            
            // Skip groups that still have tasks
            if !count.next().is_some_and(|x| x == 0) {
                continue;
            }

            debug!(
                "clear group {} info for have been overdue for more than a week.",
                group_id
            );
            self.clear_group_info(group_id);
        }
    }

    /// Checks if notifications are enabled for a group.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - The ID of the group to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If group notifications are enabled or if the group doesn't exist
    /// * `false` - If group notifications are disabled
    pub(crate) fn check_group_notification_available(&self, group_id: &u32) -> bool {
        let mut set = match self.inner.query::<bool>(
            "SELECT display FROM group_notification_config WHERE group_id = ?",
            group_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query group {} notification: {}", group_id, e);
                return true;
            }
        };
        set.next().unwrap_or(true)
    }

    /// Checks if notifications are enabled for a task.
    /// 
    /// If the task belongs to a group, returns the group's notification setting.
    /// Otherwise, returns the task's individual notification setting.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If task notifications are enabled
    /// * `false` - If task notifications are disabled
    pub(crate) fn check_task_notification_available(&self, task_id: &u32) -> bool {
        // Check if task belongs to a group
        if let Some(group) = self.query_task_gid(*task_id) {
            return self.check_group_notification_available(&group);
        }

        // Check individual task notification setting
        let mut set = match self
            .inner
            .query::<bool>("SELECT display FROM task_config WHERE task_id = ?", task_id)
        {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query task {} notification: {}", task_id, e);
                return true;
            }
        };
        set.next().unwrap_or(true)
    }

    /// Disables notifications for a specific task.
    /// 
    /// Sets the display flag to false for the specified task ID, either inserting
    /// a new record or updating an existing one.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task for which notifications should be disabled
    pub(crate) fn disable_task_notification(&self, task_id: u32) {
        if let Err(e) = self.inner.execute(
            "INSERT INTO task_config (task_id, display) VALUES (?, ?) ON CONFLICT(task_id) DO UPDATE SET display = excluded.display",
            (task_id, false),
        ) {
            error!("Failed to update {} notification: {}", task_id, e);
            sys_event!(ExecFault, DfxCode::RDB_FAULT_04, &format!("Failed to update {} notification: {}", task_id, e));
        }
    }

    /// Associates a task with a notification group.
    /// 
    /// Inserts or updates the group association for the specified task ID.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task to associate with a group
    /// * `group_id` - The ID of the group to associate the task with
    pub(crate) fn update_task_group(&self, task_id: u32, group_id: u32) {
        if let Err(e) = self.inner.execute(
            "INSERT INTO group_notification (task_id, group_id) VALUES (?, ?) ON CONFLICT(task_id) DO UPDATE SET group_id = excluded.group_id",
            (task_id, group_id),
        ) {
            error!("Failed to update {} notification: {}", task_id, e);
            sys_event!(ExecFault, DfxCode::RDB_FAULT_04, &format!("Failed to update {} notification: {}", task_id, e));
        }
    }

    /// Retrieves all tasks belonging to a specific group.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - The ID of the group whose tasks to retrieve
    /// 
    /// # Returns
    /// 
    /// * A vector containing the IDs of all tasks in the specified group
    pub(crate) fn query_group_tasks(&self, group_id: u32) -> Vec<u32> {
        let set = match self.inner.query::<u32>(
            "SELECT task_id FROM group_notification WHERE group_id = ?",
            group_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query group tasks: {}", e);
                sys_event!(
                    ExecFault,
                    DfxCode::RDB_FAULT_04,
                    &format!("Failed to query group tasks: {}", e)
                );
                return Vec::new();
            }
        };
        set.collect()
    }

    /// Retrieves the group ID associated with a specific task.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task whose group to retrieve
    /// 
    /// # Returns
    /// 
    /// * `Some(u32)` - The group ID if the task belongs to a group
    /// * `None` - If the task doesn't belong to any group
    pub(crate) fn query_task_gid(&self, task_id: u32) -> Option<u32> {
        let mut set = match self.inner.query::<u32>(
            "SELECT group_id FROM group_notification WHERE task_id = ?",
            task_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query task group id: {}", e);
                sys_event!(
                    ExecFault,
                    DfxCode::RDB_FAULT_04,
                    &format!("Failed to query task group id: {}", e)
                );
                return None;
            }
        };
        set.next()
    }

    /// Retrieves customized notification content for a specific task.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task whose notification content to retrieve
    /// 
    /// # Returns
    /// 
    /// * `Some(CustomizedNotification)` - The customized notification content if available
    /// * `None` - If no customized notification content exists for the task
    pub(crate) fn query_task_customized_notification(
        &self,
        task_id: u32,
    ) -> Option<CustomizedNotification> {
        let mut set = match self.inner.query::<(Option<String>, Option<String>, Option<String>)>(
            "SELECT title, text, want_agent FROM task_notification_content WHERE task_id = ?",
            task_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query task customized notification: {}", e);
                sys_event!(
                    ExecFault,
                    DfxCode::RDB_FAULT_04,
                    &format!("Failed to query task customized notification: {}", e)
                );
                return None;
            }
        };
        set.next()
            .map(|(title, text, want_agent)| CustomizedNotification { title, text, want_agent })
    }

    /// Updates or inserts customized notification content for a task.
    /// 
    /// # Arguments
    /// 
    /// * `config` - Notification configuration containing task ID and content to update
    pub(crate) fn update_task_customized_notification(&self, config: &NotificationConfig) {
        if let Err(e) = self.inner.execute(
            "INSERT INTO task_notification_content (task_id, title, text, want_agent, visibility) VALUES (?, ?, ?, ?, ?) ON CONFLICT(task_id) DO UPDATE SET title = excluded.title, text = excluded.text, want_agent = excluded.want_agent, visibility = excluded.visibility",
            (config.task_id, config.title.clone(), config.text.clone(), config.want_agent.clone(), config.visibility),
        ) {
            error!("Failed to insert {} notification: {}", config.task_id, e);
            sys_event!(ExecFault, DfxCode::RDB_FAULT_04, &format!("Failed to insert {} notification: {}", config.task_id, e));
        }
    }

    /// Retrieves customized notification content for a specific group.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - The ID of the group whose notification content to retrieve
    /// 
    /// # Returns
    /// 
    /// * `Some(CustomizedNotification)` - The customized notification content if available
    /// * `None` - If no customized notification content exists for the group
    pub(crate) fn query_group_customized_notification(
        &self,
        group_id: u32,
    ) -> Option<CustomizedNotification> {
        let mut set = match self.inner.query::<(Option<String>, Option<String>, Option<String>)>(
            "SELECT title, text, want_agent FROM group_notification_content WHERE group_id = ?",
            group_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query task customized notification: {}", e);
                sys_event!(
                    ExecFault,
                    DfxCode::RDB_FAULT_04,
                    &format!("Failed to query task customized notification: {}", e)
                );
                return None;
            }
        };
        set.next()
            .map(|(title, text, want_agent)| CustomizedNotification { title, text, want_agent })
    }

    /// Updates or inserts customized notification content for a group.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - The ID of the group to update
    /// * `title` - Optional title for the group notification
    /// * `text` - Optional text for the group notification
    /// * `want_agent` - Optional want agent for the group notification
    pub(crate) fn update_group_customized_notification(
        &self,
        group_id: u32,
        title: Option<String>,
        text: Option<String>,
        want_agent: Option<String>,
    ) {
        if let Err(e) = self.inner.execute(
            "INSERT INTO group_notification_content (group_id, title, text, want_agent) VALUES (?, ?, ?, ?) ON CONFLICT(group_id) DO UPDATE SET title = excluded.title, text = excluded.text, want_agent = excluded.want_agent",
            (group_id, title, text, want_agent),
        ) {
            error!("Failed to insert {} notification: {}", group_id, e);
            sys_event!(ExecFault, DfxCode::RDB_FAULT_04, &format!("Failed to insert {} notification: {}", group_id, e));
        }
    }

    /// Updates or inserts configuration for a notification group.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - The ID of the group to update
    /// * `gauge` - Whether to display a progress gauge
    /// * `ctime` - Creation timestamp for the group
    /// * `display` - Whether to display notifications for this group
    /// * `visibility` - Visibility settings for the group notifications
    pub(crate) fn update_group_config(
        &self,
        group_id: u32,
        gauge: bool,
        ctime: u64,
        display: bool,
        visibility: u32,
    ) {
        if let Err(e) = self.inner.execute(
            "INSERT INTO group_notification_config (group_id, gauge, attach_able, ctime, display, visibility) VALUES (?, ?, ?, ?, ?, ?) ON CONFLICT(group_id) DO UPDATE SET gauge = excluded.gauge , ctime = excluded.ctime, display = excluded.display, visibility = excluded.visibility",
            (group_id, gauge, true, ctime, display, visibility),
        ) {
            error!("Failed to update {} notification: {}", group_id, e);
            sys_event!(ExecFault, DfxCode::RDB_FAULT_04, &format!("Failed to update {} notification: {}", group_id, e));
        }
    }

    /// Checks if a group exists in the database.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - The ID of the group to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If the group exists
    /// * `false` - If the group doesn't exist or an error occurs
    pub(crate) fn contains_group(&self, group_id: u32) -> bool {
        let mut set = match self.inner.query::<u32>(
            "SELECT group_id FROM group_notification_config where group_id = ?",
            group_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query group {} notification: {}", group_id, e);
                return false;
            }
        };
        set.row_count() == 1
    }

    /// Checks if tasks can be attached to a group.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - The ID of the group to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If tasks can be attached to the group
    /// * `false` - If tasks cannot be attached or the group doesn't exist
    pub(crate) fn attach_able(&self, group_id: u32) -> bool {
        let mut set = match self.inner.query::<bool>(
            "SELECT attach_able FROM group_notification_config where group_id = ?",
            group_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query group {} notification: {}", group_id, e);
                return false;
            }
        };
        set.next().unwrap_or(false)
    }

    /// Disables the ability to attach tasks to a group.
    /// 
    /// Sets the attach_able flag to false for the specified group ID.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - The ID of the group to modify
    pub(crate) fn disable_attach_group(&self, group_id: u32) {
        if let Err(e) = self.inner.execute(
            " UPDATE group_notification_config SET attach_able = ? where group_id = ?",
            (false, group_id),
        ) {
            error!("Failed to update {} notification: {}", group_id, e);
            sys_event!(
                ExecFault,
                DfxCode::RDB_FAULT_04,
                &format!("Failed to update {} notification: {}", group_id, e)
            );
        }
    }

    /// Checks if a group notification should display a progress gauge.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - The ID of the group to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If the group should display a progress gauge
    /// * `false` - If no progress gauge should be displayed or the group doesn't exist
    pub(crate) fn is_gauge(&self, group_id: u32) -> bool {
        let mut set = match self.inner.query::<bool>(
            "SELECT gauge FROM group_notification_config where group_id = ?",
            group_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query group {} notification: {}", group_id, e);
                return false;
            }
        };
        set.next().unwrap_or(false)
    }

    /// Checks if completion status should be visible in a task notification.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If completion status should be visible
    /// * `false` - If completion status should be hidden
    /// 
    /// # Notes
    /// 
    /// * Returns `true` if visibility is 0 or null (default behavior)
    /// * Otherwise checks the least significant bit of visibility (0b01)
    pub(crate) fn is_completion_visible(&self, task_id: u32) -> bool {
        let mut set = match self.inner.query::<i32>(
            "SELECT visibility FROM task_notification_content where task_id = ?",
            task_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query task {} notification: {}", task_id, e);
                return false;
            }
        };

        match set.next() {
            // If visibility is 0, completion_visible is true regardless of gauge setting
            Some(0) => true, 
            // Check the least significant bit (0b01) to determine visibility
            Some(visibility) => (visibility & 0b01) != 0,
            // If visibility is null, default to true
            None => true, 
        }
    }

    /// Checks if progress should be visible in a task notification.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If progress should be visible
    /// * `false` - If progress should be hidden
    /// 
    /// # Notes
    /// 
    /// * Returns the task's gauge setting if visibility is 0 or null
    /// * Otherwise checks the second bit of visibility (0b10)
    pub(crate) fn is_progress_visible(&self, task_id: u32) -> bool {
        let mut set = match self.inner.query::<i32>(
            "SELECT visibility FROM task_notification_content where task_id = ?",
            task_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query task {} notification: {}", task_id, e);
                return false;
            }
        };

        match set.next() {
            // If visibility is 0, use the task's gauge setting from the dispatcher
            Some(0) => NotificationDispatcher::get_instance()
                .get_task_gauge(task_id)
                .unwrap_or(false),
            // Check the second bit (0b10) to determine visibility
            Some(visibility) => (visibility & 0b10) != 0,
            // If visibility is null, use the task's gauge setting from the dispatcher
            None => NotificationDispatcher::get_instance()
                .get_task_gauge(task_id)
                .unwrap_or(false),
        }
    }

    /// Checks if completion status should be visible in a group notification.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - The ID of the group to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If completion status should be visible
    /// * `false` - If completion status should be hidden
    /// 
    /// # Notes
    /// 
    /// * Returns `true` if visibility is 0 or null (default behavior)
    /// * Otherwise checks the least significant bit of visibility (0b01)
    pub(crate) fn is_completion_visible_from_group(&self, group_id: u32) -> bool {
        let mut set = match self.inner.query::<i32>(
            "SELECT visibility FROM group_notification_config where group_id = ?",
            group_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query group {} notification: {}", group_id, e);
                return false;
            }
        };

        match set.next() {
            // If visibility is 0, completion_visible is true regardless of gauge setting
            Some(0) => true, 
            // Check the least significant bit (0b01) to determine visibility
            Some(visibility) => (visibility & 0b01) != 0,
            // If visibility is null, default to true
            None => true, 
        }
    }

    /// Checks if progress should be visible in a group notification.
    /// 
    /// # Arguments
    /// 
    /// * `group_id` - The ID of the group to check
    /// 
    /// # Returns
    /// 
    /// * `true` - If progress should be visible
    /// * `false` - If progress should be hidden
    /// 
    /// # Notes
    /// 
    /// * Returns the group's gauge setting if visibility is 0 or null
    /// * Otherwise checks the second bit of visibility (0b10)
    pub(crate) fn is_progress_visible_from_group(&self, group_id: u32) -> bool {
        let mut set = match self.inner.query::<i32>(
            "SELECT visibility FROM group_notification_config where group_id = ?",
            group_id,
        ) {
            Ok(set) => set,
            Err(e) => {
                error!("Failed to query group {} notification: {}", group_id, e);
                return false;
            }
        };

        match set.next() {
            // If visibility is 0, use the group's gauge setting
            Some(0) => self.is_gauge(group_id),
            // Check the second bit (0b10) to determine visibility
            Some(visibility) => (visibility & 0b10) != 0,
            // If visibility is null, use the group's gauge setting
            None => self.is_gauge(group_id),
        }
    }
}

#[cfg(test)]
mod ut_database {
    include!("../../../tests/ut/service/notification_bar/ut_database.rs");
}
