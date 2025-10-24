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

//! Quality of Service (QoS) scheduling system for network tasks.
//! 
//! This module implements a priority-based scheduling system for managing network tasks across
//! different applications. It provides resource allocation based on application priority,
//! user focus, and system capacity constraints.
//! 
//! The QoS system categorizes tasks into multiple priority levels with different speed limits,
//! ensuring that foreground applications and high-priority tasks receive appropriate network
//! resources while maintaining overall system performance.

mod apps;
mod direction;
mod rss;

use apps::SortedApps;
pub(crate) use direction::{QosChanges, QosDirection, QosLevel};
pub(crate) use rss::RssCapacity;

use super::state;
use crate::config::Mode;
use crate::manage::database::TaskQosInfo;
use crate::task::config::Action;

/// Main QoS scheduler that manages task prioritization and resource allocation.
///
/// This struct coordinates the scheduling of network tasks across applications,
/// adjusting their QoS levels based on application priority, user focus, and
/// system resource capacity.
pub(crate) struct Qos {
    /// Sorted collection of applications and their tasks.
    pub(crate) apps: SortedApps,
    /// Current RSS memory capacity level that determines task allocation limits.
    capacity: RssCapacity,
}

impl Qos {
    /// Creates a new QoS scheduler with default initial state.
    ///
    /// Returns a `Qos` instance with an empty application collection and initial
    /// memory capacity set to `RssCapacity::LEVEL0`.
    pub(crate) fn new() -> Self {
        Self {
            apps: SortedApps::init(),
            capacity: RssCapacity::LEVEL0,
        }
    }

    /// Adds a task to the QoS scheduler for prioritization.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task` - The task information to add to the scheduling queue.
    ///
    /// # Notes
    ///
    /// Only tasks that can run automatically are added to the QoS queue. Both upload and
    /// download tasks are managed within the scheduler, with updates determined by checking
    /// for empty collections.
    pub(crate) fn start_task(&mut self, uid: u64, task: TaskQosInfo) {
        // Only tasks that can run automatically can be added to the qos queue.
        self.apps.insert_task(uid, task);
    }

    /// Removes a task from the QoS scheduler.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task_id` - The ID of the task to remove.
    ///
    /// # Returns
    ///
    /// `true` if the task was found and removed, `false` otherwise.
    pub(crate) fn remove_task(&mut self, uid: u64, task_id: u32) -> bool {
        self.apps.remove_task(uid, task_id)
    }

    /// Reloads all tasks from the database into the QoS scheduler.
    ///
    /// This method refreshes the entire task collection, updating the scheduling state
    /// based on the current database contents.
    pub(crate) fn reload_all_tasks(&mut self) {
        self.apps.reload_all_tasks();
    }

    /// Updates the RSS memory capacity level used for task allocation.
    ///
    /// # Arguments
    ///
    /// * `rss` - The new RSS capacity level to use for scheduling decisions.
    pub(crate) fn change_rss(&mut self, rss: RssCapacity) {
        self.capacity = rss;
    }

    /// Changes the execution mode of a specific task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task_id` - The ID of the task to modify.
    /// * `mode` - The new execution mode to apply to the task.
    ///
    /// # Returns
    ///
    /// `true` if the task was found and its mode was changed, `false` otherwise.
    pub(crate) fn task_set_mode(&mut self, uid: u64, task_id: u32, mode: Mode) -> bool {
        self.apps.task_set_mode(uid, task_id, mode)
    }

    /// Reschedules all tasks and generates QoS direction changes.
    ///
    /// # Arguments
    ///
    /// * `state` - The state handler providing information about foreground abilities and top user.
    ///
    /// # Returns
    ///
    /// A `QosChanges` object containing the updated QoS directions for both download and upload tasks.
    pub(crate) fn reschedule(&mut self, state: &state::Handler) -> QosChanges {
        // Only sort apps before assigning priorities
        self.apps
            .sort(state.foreground_abilities(), state.top_user());
        let mut changes = QosChanges::new();
        // Generate QoS directions for both download and upload tasks separately
        changes.download = Some(self.reschedule_inner(Action::Download));
        changes.upload = Some(self.reschedule_inner(Action::Upload));
        changes
    }

    /// Inner method that handles the core scheduling algorithm for a specific action type.
    ///
    /// # Arguments
    ///
    /// * `action` - The action type (Download or Upload) to schedule.
    ///
    /// # Returns
    ///
    /// A vector of `QosDirection` objects specifying the new QoS levels for tasks.
    ///
    /// # Notes
    ///
    /// This method implements a three-tier priority system (M1, M2, M3) with different speed limits.
    /// Tasks are assigned to tiers based on their application's priority and position in the sorted list.
    fn reschedule_inner(&mut self, action: Action) -> Vec<QosDirection> {
        // Get capacity limits and corresponding speed levels for each priority tier
        let m1 = self.capacity.m1();
        let m1_speed = self.capacity.m1_speed();
        let m2 = self.capacity.m2();
        let m2_speed = self.capacity.m2_speed();
        let m3 = self.capacity.m3();
        let m3_speed = self.capacity.m3_speed();

        // Track current task count and positions for fair distribution
        let mut count = 0;
        let mut app_i = 0;
        let mut task_i = 0;

        let mut qos_vec = Vec::new();

        // First pass: Assign highest priority (M1) and second priority (M2) tasks
        // Iterate through all tasks in sorted order by application
        for (i, task) in self.apps.iter().enumerate().flat_map(|(i, app)| {
            // Track the last non-empty application index
            if !app.tasks.is_empty() {
                app_i = i;
            }
            app.tasks.iter().enumerate()
        }) {
            // Skip tasks that don't match the current action type
            if task.action() != action {
                continue;
            }
            
            // Assign tasks to M1 (highest priority) or M2 (medium priority) based on count
            if count < m1 {
                qos_vec.push(QosDirection::new(task.uid(), task.task_id(), m1_speed));
            } else if count < m1 + m2 {
                qos_vec.push(QosDirection::new(task.uid(), task.task_id(), m2_speed));
            }
            count += 1;
            
            // Stop once we've filled all M1 and M2 slots
            if count == m1 + m2 {
                task_i = i;
                break;
            }
        }

        // If we didn't fill all M1 and M2 slots, we're done
        if count < m1 + m2 {
            return qos_vec;
        }

        // Second pass: Implement fair distribution algorithm for M3 (lowest priority) tasks
        // Each application gets one task in turn to ensure fair resource distribution
        let mut i = 0;

        loop {
            let mut no_tasks_left = true;

            // Iterate through remaining applications (after the last non-empty app)
            for tasks in self.apps.iter().skip(app_i + 1).map(|app| &app.tasks[..]) {
                let task = match tasks.get(i) {
                    Some(task) => {
                        no_tasks_left = false;
                        task
                    }
                    None => continue,
                };

                // Skip tasks that don't match the current action type
                if task.action() != action {
                    continue;
                }

                // Assign M3 priority if we haven't filled all slots
                if count < m1 + m2 + m3 {
                    qos_vec.push(QosDirection::new(task.uid(), task.task_id(), m3_speed));
                } else {
                    return qos_vec;
                }

                count += 1;
            }

            // Exit loop when there are no more tasks to process
            if no_tasks_left {
                break;
            }
            i += 1;
        }

        // Third pass: Fill any remaining M3 slots with tasks from the last non-empty application
        // This ensures we utilize all available capacity
        for task in self
            .apps
            .iter()
            .skip(app_i)
            .take(1)
            .flat_map(|app| app.tasks.iter().skip(task_i + 1))
        {
            // Skip tasks that don't match the current action type
            if task.action() != action {
                continue;
            }

            // Assign M3 priority if we haven't filled all slots
            if count < m1 + m2 + m3 {
                qos_vec.push(QosDirection::new(task.uid(), task.task_id(), m3_speed));
            } else {
                return qos_vec;
            }
            count += 1;
        }
        qos_vec
    }
}
