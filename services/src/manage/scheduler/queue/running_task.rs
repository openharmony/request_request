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

//! Running task execution and lifecycle management.
//! 
//! This module defines the `RunningTask` struct that handles the execution of
//! download and upload tasks, manages their lifecycle events, and ensures proper
//! cleanup and notification when tasks complete or fail.

use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::config::Mode;
use crate::manage::database::RequestDb;
use crate::manage::events::{TaskEvent, TaskManagerEvent};
use crate::manage::notifier::Notifier;
use crate::manage::scheduler::queue::keeper::SAKeeper;
use crate::manage::task_manager::TaskManagerTx;
use crate::service::notification_bar::NotificationDispatcher;
use crate::task::config::Action;
use crate::task::download::download;
use crate::task::reason::Reason;
use crate::task::request_task::RequestTask;
use crate::task::upload::upload;
use crate::utils::get_current_duration;

/// A task in the process of being executed.
///
/// This struct wraps a `RequestTask` and provides the runtime context for
/// executing the task and handling its lifecycle events.
pub(crate) struct RunningTask {
    /// The underlying request task being executed.
    task: Arc<RequestTask>,
    /// Transmitter for sending task management events.
    tx: TaskManagerTx,
    /// Service ability keeper reference to prevent service from unloading
    /// while tasks are running.
    // `_keeper` is never used when executing the task.
    _keeper: SAKeeper,
}

impl RunningTask {
    /// Creates a new RunningTask instance with the given task and context.
    ///
    /// # Arguments
    ///
    /// * `task` - The request task to execute.
    /// * `tx` - Task manager transmitter for sending events.
    /// * `keeper` - Service ability keeper to maintain service state.
    ///
    /// # Returns
    ///
    /// A new `RunningTask` ready to be executed.
    pub(crate) fn new(task: Arc<RequestTask>, tx: TaskManagerTx, keeper: SAKeeper) -> Self {
        Self {
            task,
            tx,
            _keeper: keeper,
        }
    }

    /// Executes the task based on its action type.
    ///
    /// # Arguments
    ///
    /// * `abort_flag` - Atomic flag that can be set to signal task cancellation.
    ///
    /// # Notes
    ///
    /// This method dispatches to either the download or upload implementation
    /// based on the task's action type. It consumes the `RunningTask` instance.
    pub(crate) async fn run(self, abort_flag: Arc<AtomicBool>) {
        match self.conf.common_data.action {
            Action::Download => {
                download(self.task.clone(), abort_flag).await;
            }
            Action::Upload => {
                upload(self.task.clone(), abort_flag).await;
            }
            _ => {}
        }
    }

    /// Checks if a download task has completed.
    ///
    /// # Returns
    ///
    /// `true` if the task is a download and has processed its total size,
    /// `false` otherwise.
    ///
    /// # Notes
    ///
    /// Returns `false` if the task is not a download or if the total size
    /// is unknown or incomplete.
    fn check_download_complete(&self) -> bool {
        if self.action() != Action::Download {
            return false;
        }
        let mutex_guard = self.task.progress.lock().unwrap();
        if let Some(total) = mutex_guard.sizes.first() {
            // -1 indicates unknown size, so task cannot be considered complete
            if *total == -1 {
                return false;
            }
            return mutex_guard.common_data.total_processed == (*total as usize);
        }
        false
    }

    /// Sends completion notifications for a finished task.
    ///
    /// # Arguments
    ///
    /// * `task_id` - Unique identifier for the task.
    /// * `uid` - User ID associated with the task.
    /// * `mode` - Execution mode of the task.
    ///
    /// # Notes
    ///
    /// Publishes a notification if background notification is enabled,
    /// and sends a completion event to the task manager.
    fn send_complete(&self, task_id: u32, uid: u64, mode: Mode) {
        // Check if background notifications are enabled
        if self.task.background_notify.load(Ordering::Acquire) {
            NotificationDispatcher::get_instance().publish_progress_notification(self);
        }
        // Notify task manager of completion
        self.tx
            .send_event(TaskManagerEvent::Task(TaskEvent::Completed(
                task_id, uid, mode,
            )));
    }
}

/// Implements deref to allow direct access to the underlying RequestTask methods.
///
/// This enables convenient access to the RequestTask methods and properties
/// without explicitly accessing the task field.
impl Deref for RunningTask {
    type Target = Arc<RequestTask>;

    /// Returns a reference to the underlying RequestTask.
    fn deref(&self) -> &Self::Target {
        &self.task
    }
}

/// Implements cleanup and finalization for a RunningTask when it's dropped.
///
/// Handles task completion reporting, progress updates, and time tracking
/// when a task finishes execution or is dropped prematurely.
impl Drop for RunningTask {
    /// Cleans up task resources and reports final state.
    ///
    /// Updates task timing information, saves progress to the database,
    /// notifies observers, and sends the appropriate task completion event
    /// based on the task's result status.
    fn drop(&mut self) {
        // Calculate and update task timing information
        let task_end_time = get_current_duration().as_secs();
        let start_time = self.task.start_time.load(Ordering::SeqCst);
        self.task.start_time.store(task_end_time, Ordering::SeqCst);
        let total_task_time = self.task.task_time.load(Ordering::SeqCst);
        let current_task_time = task_end_time - start_time;
        let task_time = total_task_time + current_task_time;
        self.task
            .task_time
            .store(task_time as u64, Ordering::SeqCst);
        
        // Save final progress to database
        self.task.update_progress_in_database();
        RequestDb::get_instance().update_task_time(self.task_id(), task_time);
        
        // Notify observers of final progress
        Notifier::progress(&self.client_manager, self.build_notify_data());
        
        // Get task metadata for event reporting
        let task_id = self.task_id();
        let uid = self.uid();
        let mode = Mode::from(self.mode.load(Ordering::Acquire));
        
        // Determine and report the final task state
        match *self.task.running_result.lock().unwrap() {
            Some(res) => match res {
                // Task completed successfully
                Ok(()) => {
                    self.send_complete(task_id, uid, mode);
                }
                // Special handling for network offline errors
                Err(e) if e == Reason::NetworkOffline => {
                    self.tx
                        .send_event(TaskManagerEvent::Task(TaskEvent::Offline(
                            task_id, uid, mode,
                        )));
                }
                // Report other failures
                Err(e) => {
                    self.tx.send_event(TaskManagerEvent::Task(TaskEvent::Failed(
                        task_id, uid, e, mode,
                    )));
                }
            },
            // No explicit result - check if download completed successfully
            None => {
                if self.check_download_complete() {
                    self.send_complete(task_id, uid, mode);
                } else {
                    // Task was possibly cancelled or interrupted
                    self.tx
                        .send_event(TaskManagerEvent::Task(TaskEvent::Running(
                            task_id, uid, mode,
                        )));
                }
            }
        }
    }
}
