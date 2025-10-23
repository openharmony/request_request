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

//! QoS app and task management for the scheduler.
//! 
//! This module implements a priority-based scheduling system for tasks across different applications.
//! It manages sorted collections of applications and their tasks, with sorting based on application
//! priority (foreground vs background) and user focus. Tasks within applications are further sorted
//! by their mode and priority.

use std::cmp;
use std::collections::HashSet;
use std::ops::Deref;

use crate::manage::database::{RequestDb, TaskQosInfo};
use crate::task::config::{Action, Mode};

/// A collection of applications sorted by priority.
///
/// This struct maintains a list of applications that can be dynamically sorted based on
/// application state (foreground/background) and user focus. It provides methods for managing
/// tasks within these applications.
pub(crate) struct SortedApps {
    /// The inner list of applications.
    inner: Vec<App>,
}

impl SortedApps {
    /// Creates a new `SortedApps` instance and loads all applications from the database.
    ///
    /// Returns a `SortedApps` with all applications and their tasks loaded from persistent storage.
    pub(crate) fn init() -> Self {
        Self {
            inner: reload_all_app_from_database(),
        }
    }

    /// Sorts applications based on user focus and foreground status.
    ///
    /// # Arguments
    ///
    /// * `foreground_abilities` - A set of UIDs representing foreground applications.
    /// * `top_user` - The user ID that currently has focus.
    ///
    /// # Notes
    ///
    /// Applications are sorted first by whether they belong to the top user (user ID divided by 200000),
    /// and then by whether they are in the foreground.
    pub(crate) fn sort(&mut self, foreground_abilities: &HashSet<u64>, top_user: u64) {
        self.inner.sort_by(|a, b| {
            // First sort by top user status
            (a.uid / 200000 == top_user)
                .cmp(&(b.uid / 200000 == top_user))
                .then(
                    // Then sort by foreground status
                    foreground_abilities
                        .contains(&a.uid)
                        .cmp(&(foreground_abilities.contains(&b.uid))),
                )
        })
    }

    /// Reloads all tasks from the database.
    ///
    /// This replaces the current application and task data with fresh data from persistent storage.
    pub(crate) fn reload_all_tasks(&mut self) {
        self.inner = reload_all_app_from_database();
    }

    /// Inserts a new task into the appropriate application.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application.
    /// * `task` - The task information from the database.
    ///
    /// # Notes
    ///
    /// If the application doesn't exist, a new one is created and added to the list.
    pub(crate) fn insert_task(&mut self, uid: u64, task: TaskQosInfo) {
        // Convert database task info to internal task representation
        let task = Task {
            uid,
            task_id: task.task_id,
            mode: Mode::from(task.mode),
            action: Action::from(task.action),
            priority: task.priority,
        };

        // Check if the app already exists and add the task
        if let Some(app) = self.inner.iter_mut().find(|app| app.uid == uid) {
            app.insert(task);
            return;
        }

        // Create a new app with the task if it doesn't exist
        let mut app = App::new(uid);
        app.insert(task);
        self.inner.push(app);
    }

    /// Gets a mutable reference to an application by its UID.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application.
    ///
    /// # Returns
    ///
    /// An `Option` containing a mutable reference to the application if found, otherwise `None`.
    fn get_app_mut(&mut self, uid: u64) -> Option<&mut App> {
        self.inner.iter_mut().find(|app| app.uid == uid)
    }

    /// Removes a task from an application.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application.
    /// * `task_id` - The ID of the task to remove.
    ///
    /// # Returns
    ///
    /// `true` if the task was successfully removed, `false` if either the application or task wasn't found.
    pub(crate) fn remove_task(&mut self, uid: u64, task_id: u32) -> bool {
        match self.get_app_mut(uid) {
            Some(app) => app.remove(task_id),
            None => false,
        }
    }

    /// Changes the mode of a task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application.
    /// * `task_id` - The ID of the task to modify.
    /// * `mode` - The new mode to set for the task.
    ///
    /// # Returns
    ///
    /// `true` if the task's mode was successfully changed, `false` if either the application or task wasn't found.
    pub(crate) fn task_set_mode(&mut self, uid: u64, task_id: u32, mode: Mode) -> bool {
        match self.get_app_mut(uid) {
            Some(app) => app.task_set_mode(task_id, mode),
            None => false,
        }
    }
}

impl Deref for SortedApps {
    type Target = Vec<App>;

    /// Returns a reference to the underlying vector of applications.
    ///
    /// This allows for convenient iteration and access to the applications.
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Represents an application with its associated tasks.
///
/// This struct manages a collection of tasks belonging to a specific application (identified by UID).
pub(crate) struct App {
    /// The application's user ID.
    pub(crate) uid: u64,
    /// The list of tasks associated with this application, sorted by priority.
    pub(crate) tasks: Vec<Task>,
}

impl App {
    /// Creates a new empty application with the given UID.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application.
    fn new(uid: u64) -> Self {
        Self {
            uid,
            tasks: Vec::new(),
        }
    }

    /// Creates a new application with the given UID and tasks.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application.
    /// * `tasks` - The initial list of tasks for the application.
    fn from_raw(uid: u64, tasks: Vec<Task>) -> Self {
        Self { uid, tasks }
    }

    /// Inserts a task into the application in sorted order.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to insert.
    ///
    /// # Notes
    ///
    /// Uses binary search to maintain sorted order based on task priority.
    fn insert(&mut self, task: Task) {
        self.tasks.binary_insert(task)
    }

    /// Finds a task by its ID.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of the task to find.
    ///
    /// # Returns
    ///
    /// An `Option` containing the index and a mutable reference to the task if found, otherwise `None`.
    fn get_task_mut(&mut self, task_id: u32) -> Option<(usize, &mut Task)> {
        self.tasks
            .iter_mut()
            .enumerate()
            .find(|(_, task)| task.task_id == task_id)
    }

    /// Removes a task by its ID.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of the task to remove.
    ///
    /// # Returns
    ///
    /// `true` if the task was successfully removed, `false` if the task wasn't found.
    fn remove(&mut self, task_id: u32) -> bool {
        match self.get_task_mut(task_id) {
            Some((index, _task)) => {
                self.tasks.remove(index);
                // Sorting isn't needed after removal as the remaining elements are already in order
                true
            }
            None => false,
        }
    }

    /// Re-sorts the tasks based on their priority.
    ///
    /// This should be called after modifying a task's properties that affect its sort order.
    fn resort_tasks(&mut self) {
        self.tasks.sort();
    }

    /// Changes the mode of a task and re-sorts the task list.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of the task to modify.
    /// * `mode` - The new mode to set for the task.
    ///
    /// # Returns
    ///
    /// `true` if the task's mode was successfully changed, `false` if the task wasn't found.
    fn task_set_mode(&mut self, task_id: u32, mode: Mode) -> bool {
        match self.get_task_mut(task_id) {
            Some((_index, task)) => {
                task.set_mode(mode);
                // Re-sort tasks since mode affects priority
                self.resort_tasks();
                true
            }
            None => false,
        }
    }
}

/// Represents a task with its scheduling parameters.
///
/// Tasks are sorted by mode and priority within their parent application.
pub(crate) struct Task {
    /// The user ID of the application that owns this task.
    uid: u64,
    /// The unique identifier for this task.
    task_id: u32,
    /// The execution mode of the task.
    mode: Mode,
    /// The action type of the task (e.g., download, upload).
    action: Action,
    /// The priority level of the task within its mode.
    priority: u32,
}

impl Task {
    /// Returns the task's owning user ID.
    pub(crate) fn uid(&self) -> u64 {
        self.uid
    }

    /// Returns the task's unique identifier.
    pub(crate) fn task_id(&self) -> u32 {
        self.task_id
    }

    /// Returns the task's action type.
    pub(crate) fn action(&self) -> Action {
        self.action
    }

    /// Updates the task's execution mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The new mode to set for the task.
    pub(crate) fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}

impl Eq for Task {}

impl Ord for Task {
    /// Compares tasks by mode first, then by priority.
    ///
    /// This ensures that tasks are sorted by their mode and then by their priority
    /// within the same mode, allowing for efficient prioritized task scheduling.
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.mode
            .cmp(&other.mode)
            .then(self.priority.cmp(&other.priority))
    }
}

impl PartialEq for Task {
    /// Checks if two tasks have the same mode and priority.
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode && self.priority == other.priority
    }
}

impl PartialOrd for Task {
    /// Provides a partial comparison between tasks.
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// A trait for binary insertion into sorted collections.
///
/// This trait provides a method to insert elements into a sorted collection while maintaining
/// the sorted order using binary search.
trait Binary<T: Ord> {
    /// Inserts a value into the collection in sorted order.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to insert.
    fn binary_insert(&mut self, value: T);
}

impl<T: Ord> Binary<T> for Vec<T> {
    /// Inserts a value into a sorted vector in the correct position.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to insert.
    ///
    /// # Notes
    ///
    /// Uses binary search to find the correct insertion point, maintaining the sorted order.
    /// Both equal and not found cases insert at the same position.
    fn binary_insert(&mut self, value: T) {
        match self.binary_search(&value) {
            Ok(n) => self.insert(n, value),
            Err(n) => self.insert(n, value),
        }
    }
}

/// Reloads all applications and their tasks from the database.
///
/// # Returns
///
/// A vector of `App` instances, each containing their sorted tasks.
fn reload_all_app_from_database() -> Vec<App> {
    let mut inner = Vec::new();
    // Get all application UIDs from the database
    for uid in reload_app_list_from_database() {
        // Load all tasks for this application
        let mut tasks = reload_tasks_of_app_from_database(uid);
        // Ensure tasks are sorted by priority
        tasks.sort();
        inner.push(App::from_raw(uid, tasks));
    }
    inner
}

/// Loads all tasks for a specific application from the database.
///
/// # Arguments
///
/// * `uid` - The user ID of the application.
///
/// # Returns
///
/// A vector of `Task` instances for the application.
fn reload_tasks_of_app_from_database(uid: u64) -> Vec<Task> {
    // Get task information from the database and convert to Task instances
    RequestDb::get_instance()
        .get_app_task_qos_infos(uid)
        .iter()
        .map(|info| Task {
            uid,
            task_id: info.task_id,
            mode: Mode::from(info.mode),
            action: Action::from(info.action),
            priority: info.priority,
        })
        .collect()
}

/// Loads the list of application UIDs from the database.
///
/// # Returns
///
/// A set of unique user IDs representing all applications with tasks.
fn reload_app_list_from_database() -> HashSet<u64> {
    // Get all unique application UIDs from the database
    RequestDb::get_instance()
        .get_app_infos()
        .into_iter()
        .collect()
}

impl RequestDb {
    /// Retrieves all unique application UIDs from the database.
    ///
    /// # Returns
    ///
    /// A vector of user IDs representing all applications that have tasks in the database.
    fn get_app_infos(&self) -> Vec<u64> {
        // SQL query to get distinct UIDs from the request_task table
        let sql = "SELECT DISTINCT uid FROM request_task";
        self.query_integer(sql)
    }
}

#[cfg(feature = "oh")]
#[cfg(test)]
mod ut_apps {
    include!("../../../../tests/ut/manage/scheduler/qos/ut_apps.rs");
}
