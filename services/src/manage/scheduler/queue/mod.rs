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

//! Task queue management for download and upload operations.
//! 
//! This module implements a queue system for managing and scheduling network tasks,
//! with support for QoS-based prioritization, task lifecycle management,
//! and resource optimization through the service ability keeper.

mod keeper;
mod running_task;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use keeper::SAKeeper;

cfg_oh! {
    use crate::ability::SYSTEM_CONFIG_MANAGER;
}
use ylong_runtime::task::JoinHandle;

use crate::config::Mode;
use crate::error::ErrorCode;
use crate::manage::database::RequestDb;
use crate::manage::events::{TaskEvent, TaskManagerEvent};
use crate::manage::scheduler::qos::{QosChanges, QosDirection};
use crate::manage::scheduler::queue::running_task::RunningTask;
use crate::manage::task_manager::TaskManagerTx;
use crate::service::active_counter::ActiveCounter;
use crate::service::client::ClientManagerEntry;
use crate::service::run_count::RunCountManagerEntry;
use crate::task::config::Action;
use crate::task::info::State;
use crate::task::reason::Reason;
use crate::task::request_task::RequestTask;
use crate::utils::runtime_spawn;

/// Task queue manager for running download and upload operations.
///
/// This struct maintains separate queues for download and upload tasks,
/// manages their execution state, and handles task lifecycle events.
/// It coordinates with QoS mechanisms to adjust task priorities and rates.
pub(crate) struct RunningQueue {
    /// Map of currently running download tasks, keyed by (uid, task_id).
    download_queue: HashMap<(u64, u32), Arc<RequestTask>>,
    /// Map of currently running upload tasks, keyed by (uid, task_id).
    upload_queue: HashMap<(u64, u32), Arc<RequestTask>>,
    /// Map of abort handles for running tasks, allowing cancellation.
    running_tasks: HashMap<(u64, u32), Option<AbortHandle>>,
    /// Service ability keeper for managing idle timeout and resource cleanup.
    keeper: SAKeeper,
    /// Transmitter for sending task management events.
    tx: TaskManagerTx,
    /// Manager for tracking and notifying about running task counts.
    run_count_manager: RunCountManagerEntry,
    /// Manager for client-related operations and state.
    client_manager: ClientManagerEntry,
    /// Set of task IDs that need to resume uploads from breakpoints.
    pub(crate) upload_resume: HashSet<u32>,
}

impl RunningQueue {
    /// Creates a new RunningQueue instance with empty task collections.
    ///
    /// # Arguments
    ///
    /// * `tx` - Task manager transmitter for sending events.
    /// * `run_count_manager` - Manager for tracking running task counts.
    /// * `client_manager` - Manager for client-related operations.
    /// * `active_counter` - Counter for tracking active system tasks.
    ///
    /// # Returns
    ///
    /// A new `RunningQueue` with initialized components and empty queues.
    pub(crate) fn new(
        tx: TaskManagerTx,
        run_count_manager: RunCountManagerEntry,
        client_manager: ClientManagerEntry,
        active_counter: ActiveCounter,
    ) -> Self {
        Self {
            download_queue: HashMap::new(),
            upload_queue: HashMap::new(),
            keeper: SAKeeper::new(tx.clone(), active_counter),
            tx,
            running_tasks: HashMap::new(),
            run_count_manager,
            client_manager,
            upload_resume: HashSet::new(),
        }
    }

    /// Retrieves a reference to a task by its UID and task ID.
    ///
    /// # Arguments
    ///
    /// * `uid` - User ID associated with the task.
    /// * `task_id` - Unique identifier for the task.
    ///
    /// # Returns
    ///
    /// A reference to the task if found in either download or upload queue.
    pub(crate) fn get_task(&self, uid: u64, task_id: u32) -> Option<&Arc<RequestTask>> {
        self.download_queue
            .get(&(uid, task_id))
            .or_else(|| self.upload_queue.get(&(uid, task_id)))
    }

    /// Retrieves a cloned reference to a task by its UID and task ID.
    ///
    /// # Arguments
    ///
    /// * `uid` - User ID associated with the task.
    /// * `task_id` - Unique identifier for the task.
    ///
    /// # Returns
    ///
    /// A cloned reference to the task if found in either download or upload queue.
    pub(crate) fn get_task_clone(&self, uid: u64, task_id: u32) -> Option<Arc<RequestTask>> {
        self.download_queue
            .get(&(uid, task_id))
            .cloned()
            .or_else(|| self.upload_queue.get(&(uid, task_id)).cloned())
    }

    /// Marks a task as finished by removing it from the running tasks map.
    ///
    /// # Arguments
    ///
    /// * `uid` - User ID associated with the task.
    /// * `task_id` - Unique identifier for the task.
    pub(crate) fn task_finish(&mut self, uid: u64, task_id: u32) {
        self.running_tasks.remove(&(uid, task_id));
    }

    /// Attempts to restart a previously running task.
    ///
    /// # Arguments
    ///
    /// * `uid` - User ID associated with the task.
    /// * `task_id` - Unique identifier for the task.
    ///
    /// # Returns
    ///
    /// `true` if the task was successfully restarted, `false` if the task was not found
    /// or is already running.
    pub(crate) fn try_restart(&mut self, uid: u64, task_id: u32) -> bool {
        if let Some(task) = self
            .download_queue
            .get(&(uid, task_id))
            .or(self.upload_queue.get(&(uid, task_id)))
        {
            // Check if task is already running to prevent duplicate execution
            if self.running_tasks.contains_key(&(uid, task_id)) {
                return false;
            }
            info!("{} restart running", task_id);
            let running_task = RunningTask::new(task.clone(), self.tx.clone(), self.keeper.clone());
            let abort_flag = Arc::new(AtomicBool::new(false));
            let abort_flag_clone = abort_flag.clone();
            let join_handle = runtime_spawn(async move {
                running_task.run(abort_flag_clone.clone()).await;
            });
            let uid = task.uid();
            let task_id = task.task_id();
            self.running_tasks.insert(
                (uid, task_id),
                Some(AbortHandle::new(abort_flag, join_handle)),
            );
            true
        } else {
            false
        }
    }

    /// Returns an iterator over all tasks in both download and upload queues.
    ///
    /// # Returns
    ///
    /// An iterator yielding references to all currently queued tasks.
    pub(crate) fn tasks(&self) -> impl Iterator<Item = &Arc<RequestTask>> {
        self.download_queue
            .values()
            .chain(self.upload_queue.values())
    }

    /// Returns the total number of tasks currently in both queues.
    ///
    /// # Returns
    ///
    /// The sum of download and upload tasks currently being managed.
    pub(crate) fn running_tasks(&self) -> usize {
        self.download_queue.len() + self.upload_queue.len()
    }

    /// Reschedules tasks based on QoS changes for both download and upload operations.
    ///
    /// # Arguments
    ///
    /// * `qos` - Contains new QoS directions for download and upload tasks.
    /// * `qos_remove_queue` - Vector to collect tasks that need to be removed from QoS management.
    pub(crate) fn reschedule(&mut self, qos: QosChanges, qos_remove_queue: &mut Vec<(u64, u32)>) {
        if let Some(vec) = qos.download {
            self.reschedule_inner(Action::Download, vec, qos_remove_queue)
        }
        if let Some(vec) = qos.upload {
            self.reschedule_inner(Action::Upload, vec, qos_remove_queue)
        }
    }

    /// Internal implementation for rescheduling tasks based on QoS directions.
    ///
    /// # Arguments
    ///
    /// * `action` - The type of tasks to reschedule (Download or Upload).
    /// * `qos_vec` - List of QoS directions for specific tasks.
    /// * `qos_remove_queue` - Vector to collect tasks that need to be removed from QoS management.
    pub(crate) fn reschedule_inner(
        &mut self,
        action: Action,
        qos_vec: Vec<QosDirection>,
        qos_remove_queue: &mut Vec<(u64, u32)>,
    ) {
        // Create a new queue to hold tasks that should continue running
        let mut new_queue = HashMap::new();

        // Select the appropriate queue based on action type
        let queue = if action == Action::Download {
            &mut self.download_queue
        } else {
            &mut self.upload_queue
        };

        // Process each task according to its new QoS direction
        for qos_direction in qos_vec.iter() {
            let uid = qos_direction.uid();
            let task_id = qos_direction.task_id();

            if let Some(task) = queue.remove(&(uid, task_id)) {
                // Task exists in current queue - update its speed limit and keep it running
                task.speed_limit(qos_direction.direction() as u64);
                new_queue.insert((uid, task_id), task);
                continue;
            }

            // Task not in current queue - retrieve from database and start it
            #[cfg(feature = "oh")]
            let system_config = unsafe { SYSTEM_CONFIG_MANAGER.assume_init_ref().system_config() };
            let upload_resume = self.upload_resume.remove(&task_id);

            let task = match RequestDb::get_instance().get_task(
                task_id,
                #[cfg(feature = "oh")]
                system_config,
                &self.client_manager,
                upload_resume,
            ) {
                Ok(task) => task,
                Err(ErrorCode::TaskNotFound) => continue, // Skip if task doesn't exist
                Err(ErrorCode::TaskStateErr) => continue, // Skip if task state is invalid
                Err(e) => {
                    // Handle other errors by marking task as failed
                    info!("get task {} error:{:?}", task_id, e);
                    if let Some(info) = RequestDb::get_instance().get_task_qos_info(task_id) {
                        self.tx.send_event(TaskManagerEvent::Task(TaskEvent::Failed(
                            task_id,
                            uid,
                            Reason::OthersError,
                            Mode::from(info.mode),
                        )));
                    }
                    // Add to removal queue as it couldn't be processed
                    qos_remove_queue.push((uid, task_id));
                    continue;
                }
            };
            // Apply the new QoS speed limit
            task.speed_limit(qos_direction.direction() as u64);

            new_queue.insert((uid, task_id), task.clone());

            // Skip if task is already running
            if self.running_tasks.contains_key(&(uid, task_id)) {
                info!("task {} not finished", task_id);
                continue;
            }

            // Start the task execution
            info!("{} begin", task_id);
            let running_task = RunningTask::new(task.clone(), self.tx.clone(), self.keeper.clone());
            // Update task state in database
            RequestDb::get_instance().update_task_state(
                running_task.task_id(),
                State::Running,
                Reason::Default,
            );
            // Set up abort mechanism and spawn the task
            let abort_flag = Arc::new(AtomicBool::new(false));
            let abort_flag_clone = abort_flag.clone();
            let join_handle = runtime_spawn(async move {
                running_task.run(abort_flag_clone).await;
            });

            let uid = task.uid();
            let task_id = task.task_id();
            self.running_tasks.insert(
                (uid, task_id),
                Some(AbortHandle::new(abort_flag, join_handle)),
            );
        }
        // Cancel any tasks that weren't included in the new queue (no longer satisfy QoS)
        for task in queue.values() {
            if let Some(join_handle) = self.running_tasks.get_mut(&(task.uid(), task.task_id())) {
                if let Some(join_handle) = join_handle.take() {
                    join_handle.cancel();
                };
            }
        }
        // Replace the old queue with the new filtered queue
        *queue = new_queue;

        // Notify run count manager about the updated number of running tasks
        #[cfg(feature = "oh")]
        self.run_count_manager
            .notify_run_count(self.download_queue.len() + self.upload_queue.len());
    }

    /// Cancels all currently running tasks.
    ///
    /// This method cancels all tasks managed by this queue, clearing all abort handles.
    pub(crate) fn retry_all_tasks(&mut self) {
        for task in self.running_tasks.iter_mut() {
            if let Some(handle) = task.1.take() {
                handle.cancel();
            }
        }
    }

    /// Cancels a specific task by its ID and user ID.
    ///
    /// # Arguments
    ///
    /// * `task_id` - Unique identifier for the task to cancel.
    /// * `uid` - User ID associated with the task.
    ///
    /// # Returns
    ///
    /// `true` if the task was found and successfully canceled, `false` otherwise.
    ///
    /// # Notes
    ///
    /// This method acquires the task's progress lock before canceling to ensure
    /// consistent state updates.
    pub(crate) fn cancel_task(&mut self, task_id: u32, uid: u64) -> bool {
        let handle = match self
            .running_tasks
            .get_mut(&(uid, task_id))
            .and_then(|task| task.take())
        {
            Some(h) => h,
            None => return false,
        };
        let task = match self
            .upload_queue
            .get(&(uid, task_id))
            .or_else(|| self.download_queue.get(&(uid, task_id)))
        {
            Some(t) => t,
            None => {
                return false;
            }
        };

        // Acquire progress lock to ensure consistent state during cancellation
        let progress_lock = task.progress.lock().unwrap();
        handle.cancel();
        drop(progress_lock); // Release lock before database operation

        // Ensure task progress is saved to database
        task.update_progress_in_database();
        true
    }

    /// Shuts down the running queue and cancels any pending service unloading.
    ///
    /// This method calls shutdown on the service ability keeper to prevent any
    /// further idle timeout events from being triggered.
    pub(crate) fn shutdown(&self) {
        self.keeper.shutdown();
    }
}

/// Handle for canceling a running task with both flag and future cancellation.
struct AbortHandle {
    /// Atomic flag that can be checked by the running task to detect cancellation.
    abort_flag: Arc<AtomicBool>,
    /// Join handle for the spawned task future, allowing direct cancellation.
    join_handle: JoinHandle<()>,
}

impl AbortHandle {
    /// Creates a new AbortHandle with the given flag and join handle.
    ///
    /// # Arguments
    ///
    /// * `abort_flag` - Atomic boolean flag used to signal cancellation to the task.
    /// * `join_handle` - Join handle for the spawned task future.
    fn new(abort_flag: Arc<AtomicBool>, join_handle: JoinHandle<()>) -> Self {
        Self {
            abort_flag,
            join_handle,
        }
    }
    
    /// Cancels the associated task by setting the abort flag and canceling the future.
    ///
    /// Uses Release ordering to ensure the abort flag is visible to other threads.
    fn cancel(self) {
        // Set the abort flag for cooperative cancellation
        self.abort_flag.store(true, Ordering::Release);
        // Directly cancel the runtime future
        self.join_handle.cancel();
    }
}
