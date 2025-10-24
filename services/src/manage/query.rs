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

//! Task querying and searching functionality.
//! 
//! This module provides various methods for retrieving and searching task information,
//! including filtering tasks by different criteria and handling query-related events.

pub(crate) use ffi::TaskFilter;

use super::events::QueryEvent;
use super::TaskManager;
use crate::config::{Action, Mode};
use crate::manage::database::RequestDb;
use crate::service::permission::ManagerPermission;
use crate::task::config::TaskConfig;
use crate::task::info::{State, TaskInfo};

/// Retrieves a task configuration by ID and token.
/// 
/// # Arguments
/// 
/// * `task_id` - The ID of the task to retrieve
/// * `token` - The authentication token for the task
/// 
/// # Returns
/// 
/// Returns `Some(TaskConfig)` if a task with the given ID exists and the token matches,
/// otherwise `None`.
pub(crate) fn get_task(task_id: u32, token: String) -> Option<TaskConfig> {
    if let Some(config) = RequestDb::get_instance().get_task_config(task_id) {
        if config.token.eq(token.as_str()) {
            return Some(config);
        }
        return None;
    }
    None
}

/// Searches for tasks matching the given filter.
/// 
/// Supports both user-specific and system-wide searches.
/// 
/// # Arguments
/// 
/// * `filter` - The filter criteria for the search
/// * `method` - The search method to use (user-specific or system-wide)
/// 
/// # Returns
/// 
/// Returns a vector of task IDs that match the search criteria.
pub(crate) fn search(filter: TaskFilter, method: SearchMethod) -> Vec<u32> {
    let database = RequestDb::get_instance();

    match method {
        SearchMethod::User(uid) => database.search_task(filter, uid),
        SearchMethod::System(bundle_name) => database.system_search_task(filter, bundle_name),
    }
}

impl TaskManager {
    /// Handles a query event by processing the appropriate query operation.
    /// 
    /// Processes different types of query events (Show, Query, Touch) and sends the result
    /// back through the provided channel.
    /// 
    /// # Arguments
    /// 
    /// * `event` - The query event to handle
    pub(crate) fn handle_query_event(&self, event: QueryEvent) {
        let (info, tx) = match event {
            QueryEvent::Show(task_id, uid, tx) => {
                let info = self.show(uid, task_id);
                (info, tx)
            }
            QueryEvent::Query(task_id, action, tx) => {
                let info = self.query(task_id, action);
                (info, tx)
            }
            QueryEvent::Touch(task_id, uid, token, tx) => {
                let info = self.touch(uid, task_id, token);
                (info, tx)
            }
        };
        let _ = tx.send(info);
    }

    /// Retrieves task information for a specific user.
    /// 
    /// Updates the task's progress in the database if the task is currently running,
    /// then retrieves the task information if the UIDs match.
    /// 
    /// # Arguments
    /// 
    /// * `uid` - The user ID to verify ownership
    /// * `task_id` - The ID of the task to retrieve
    /// 
    /// # Returns
    /// 
    /// Returns `Some(TaskInfo)` if the task exists and is owned by the specified user,
    /// otherwise `None`.
    pub(crate) fn show(&self, uid: u64, task_id: u32) -> Option<TaskInfo> {
        if let Some(task) = self.scheduler.get_task(uid, task_id) {
            task.update_progress_in_database()
        }

        match RequestDb::get_instance().get_task_info(task_id) {
            Some(info) if info.uid() == uid => Some(info),
            _ => {
                info!("TaskManger Show: no task found");
                None
            }
        }
    }

    /// Retrieves task information with token authentication.
    /// 
    /// Updates the task's progress in the database if the task is currently running,
    /// then retrieves and sanitizes the task information if the UIDs and token match.
    /// 
    /// # Arguments
    /// 
    /// * `uid` - The user ID to verify ownership
    /// * `task_id` - The ID of the task to retrieve
    /// * `token` - The authentication token for the task
    /// 
    /// # Returns
    /// 
    /// Returns `Some(TaskInfo)` with the bundle name sanitized if the task exists,
    /// is owned by the specified user, and the token matches, otherwise `None`.
    pub(crate) fn touch(&self, uid: u64, task_id: u32, token: String) -> Option<TaskInfo> {
        if let Some(task) = self.scheduler.get_task(uid, task_id) {
            task.update_progress_in_database()
        }

        let mut info = match RequestDb::get_instance().get_task_info(task_id) {
            Some(info) => info,
            None => {
                info!("TaskManger Touch: no task found");
                return None;
            }
        };

        if info.uid() == uid && info.token() == token {
            info.bundle = "".to_string();
            Some(info)
        } else {
            info!("TaskManger Touch: no task found");
            None
        }
    }

    /// Queries task information with action permission checking.
    /// 
    /// Updates the task's progress in the database if the task is currently running,
    /// then retrieves and sanitizes the task information if the action has sufficient
    /// permissions.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task to retrieve
    /// * `action` - The action to check permissions against
    /// 
    /// # Returns
    /// 
    /// Returns `Some(TaskInfo)` with sensitive data sanitized if the task exists and
    /// the action has sufficient permissions, otherwise `None`.
    pub(crate) fn query(&self, task_id: u32, action: Action) -> Option<TaskInfo> {
        if let Some(task) = self
            .scheduler
            .tasks()
            .find(|task| task.task_id() == task_id)
        {
            task.update_progress_in_database()
        }

        let mut info = match RequestDb::get_instance().get_task_info(task_id) {
            Some(info) => info,
            None => {
                info!("TaskManger Query: no task found");
                return None;
            }
        };

        let task_action = info.action();
        if ManagerPermission::check_action(action, task_action) {
            info.data = "".to_string();
            info.url = "".to_string();
            Some(info)
        } else {
            info!("TaskManger Query: no task found");
            None
        }
    }
}

impl RequestDb {
    /// Searches for tasks belonging to a specific user that match filter criteria.
    /// 
    /// # Arguments
    /// 
    /// * `filter` - The filter criteria for the search
    /// * `uid` - The user ID to filter by
    /// 
    /// # Returns
    /// 
    /// Returns a vector of task IDs that match the user and filter criteria.
    pub(crate) fn search_task(&self, filter: TaskFilter, uid: u64) -> Vec<u32> {
        let mut sql = format!("SELECT task_id from request_task WHERE uid = {} AND ", uid);
        Self::search_filter(&mut sql, &filter);
        self.query_integer(&sql)
    }

    /// Searches for tasks across the system with optional bundle filtering.
    /// 
    /// # Arguments
    /// 
    /// * `filter` - The filter criteria for the search
    /// * `bundle_name` - The bundle name to filter by, or "*" for all bundles
    /// 
    /// # Returns
    /// 
    /// Returns a vector of task IDs that match the bundle and filter criteria.
    pub(crate) fn system_search_task(&self, filter: TaskFilter, bundle_name: String) -> Vec<u32> {
        let mut sql = "SELECT task_id from request_task WHERE ".to_string();
        if bundle_name != "*" {
            sql.push_str(&format!("bundle = '{}' AND ", bundle_name));
        }
        Self::search_filter(&mut sql, &filter);
        self.query_integer(&sql)
    }

    /// Appends filter conditions to an SQL query string.
    /// 
    /// Adds conditions for time range, state, action, and mode to the provided SQL query.
    /// 
    /// # Arguments
    /// 
    /// * `sql` - The SQL query string to modify
    /// * `filter` - The filter criteria to apply
    fn search_filter(sql: &mut String, filter: &TaskFilter) {
        // Always include time range filtering
        sql.push_str(&format!(
            "ctime BETWEEN {} AND {} ",
            filter.after, filter.before
        ));
        
        // Only add state filter if not matching all states
        if filter.state != State::Any.repr {
            sql.push_str(&format!("AND state = {} ", filter.state));
        }
        
        // Only add action filter if not matching all actions
        if filter.action != Action::Any.repr {
            sql.push_str(&format!("AND action = {} ", filter.action));
        }
        
        // Only add mode filter if not matching all modes
        if filter.mode != Mode::Any.repr {
            sql.push_str(&format!("AND mode = {} ", filter.mode));
        }
    }
}

/// Retrieves the MIME type of a task belonging to a specific user.
/// 
/// # Arguments
/// 
/// * `uid` - The user ID to verify ownership
/// * `task_id` - The ID of the task to retrieve the MIME type for
/// 
/// # Returns
/// 
/// Returns the MIME type as a string if the task exists and is owned by the specified user,
/// otherwise an empty string.
pub(crate) fn query_mime_type(uid: u64, task_id: u32) -> String {
    match RequestDb::get_instance().get_task_info(task_id) {
        Some(info) if info.uid() == uid => info.mime_type(),
        _ => {
            info!("TaskManger QueryMimeType: no task found");
            "".into()
        }
    }
}

/// Method for searching tasks, either by user or system-wide.
/// 
/// Used to determine whether a search should be restricted to a specific user
/// or performed system-wide with optional bundle filtering.
#[derive(Debug)]
pub(crate) enum SearchMethod {
    /// Search tasks belonging to a specific user
    User(u64),
    /// Search tasks system-wide with bundle name filtering
    System(String),
}

#[allow(unreachable_pub)]
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    // Filter criteria for searching tasks
    #[derive(Debug)]
    struct TaskFilter {
        before: i64,
        after: i64,
        state: u8,
        action: u8,
        mode: u8,
    }
}

#[cfg(test)]
// Unit tests for the query module
mod ut_query {
    include!("../../tests/ut/manage/ut_query.rs");
}
