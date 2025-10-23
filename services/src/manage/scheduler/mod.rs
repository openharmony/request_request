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

//! Task scheduler for network operations.
//! 
//! This module implements a comprehensive task scheduling system for managing network tasks
//! based on QoS (Quality of Service) priorities, system state changes, and resource constraints.
//! The scheduler coordinates task execution across multiple applications while respecting
//! network conditions, account states, and application foreground/background transitions.

mod qos;
mod queue;
pub(crate) mod state;
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::sync::Arc;

mod sql;
use qos::Qos;
use queue::RunningQueue;
use state::sql::SqlList;

use super::events::TaskManagerEvent;
use crate::config::Mode;
use crate::error::ErrorCode;
use crate::info::TaskInfo;
use crate::manage::database::RequestDb;
use crate::manage::notifier::Notifier;
use crate::manage::task_manager::TaskManagerTx;
use crate::service::active_counter::ActiveCounter;
use crate::service::client::ClientManagerEntry;
use crate::service::notification_bar::NotificationDispatcher;
use crate::service::run_count::RunCountManagerEntry;
use crate::task::config::Action;
use crate::task::info::State;
use crate::task::notify::WaitingCause;
use crate::task::reason::Reason;
use crate::task::request_task::RequestTask;
use crate::utils::get_current_timestamp;

const MILLISECONDS_IN_ONE_MONTH: u64 = 30 * 24 * 60 * 60 * 1000;

// Scheduler 的基本处理逻辑如下：
// 1. Scheduler 维护一个当前所有 运行中 和
//    待运行的任务优先级队列（scheduler.qos），
// 该队列仅保存任务的优先级信息和基础信息，当环境发生变化时，
// 将该优先级队列重新排序，并得到一系列优先级调节指令（QosChange），
// 这些指令的作用是指引运行队列将满足优先级排序的任务变为运行状态。
//
// 2. 得到指令后，将该指令作用于任务队列（scheduler.queue）。
// 任务队列保存当前正在运行的任务列表（scheduler.queue.running），
// 所以运行队列根据指令的内容， 将指令引导的那些任务置于运行任务列表，
// 并调节速率。对于那些当前正在执行，但此时又未得到运行权限的任务，
// 我们将其修改为Waiting状态，运行任务队列就更新完成了。
//
// 注意：未处于运行状态中的任务不会停留在内存中。

pub(crate) struct Scheduler {
    /// Quality of Service manager for task prioritization.
    qos: Qos,
    /// Queue managing currently executing tasks.
    running_queue: RunningQueue,
    /// Client manager for task notifications.
    client_manager: ClientManagerEntry,
    /// Handler for system state changes and their impact on tasks.
    state_handler: state::Handler,
    /// Flag indicating whether a reschedule operation is pending.
    pub(crate) resort_scheduled: bool,
    /// Transmitter for sending events to the task manager.
    task_manager: TaskManagerTx,
}

impl Scheduler {
    /// Initializes a new scheduler instance.
    ///
    /// # Arguments
    ///
    /// * `tx` - Transmitter for sending events to the task manager.
    /// * `runcount_manager` - Manager for tracking task run counts.
    /// * `client_manager` - Manager for client notifications.
    /// * `active_counter` - Counter for tracking active tasks.
    ///
    /// # Returns
    ///
    /// A new `Scheduler` instance initialized with the provided components.
    pub(crate) fn init(
        tx: TaskManagerTx,
        runcount_manager: RunCountManagerEntry,
        client_manager: ClientManagerEntry,
        active_counter: ActiveCounter,
    ) -> Scheduler {
        let mut state_handler = state::Handler::new(tx.clone());
        // Initialize state and update database with initial state
        let sql_list = state_handler.init();
        let db = RequestDb::get_instance();
        for sql in sql_list {
            if let Err(e) = db.execute(&sql) {
                error!("TaskManager update network failed {:?}", e);
            };
        }

        Self {
            qos: Qos::new(),
            running_queue: RunningQueue::new(
                tx.clone(),
                runcount_manager,
                client_manager.clone(),
                active_counter,
            ),
            client_manager,
            state_handler,
            resort_scheduled: false,
            task_manager: tx,
        }
    }

    /// Retrieves a running task by its UID and task ID.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task_id` - The unique identifier of the task.
    ///
    /// # Returns
    ///
    /// A reference to the task if it exists in the running queue, `None` otherwise.
    pub(crate) fn get_task(&self, uid: u64, task_id: u32) -> Option<&Arc<RequestTask>> {
        self.running_queue.get_task(uid, task_id)
    }

    /// Returns an iterator over all currently running tasks.
    ///
    /// # Returns
    ///
    /// An iterator yielding references to running tasks.
    pub(crate) fn tasks(&self) -> impl Iterator<Item = &Arc<RequestTask>> {
        self.running_queue.tasks()
    }

    /// Returns the number of currently running tasks.
    ///
    /// # Returns
    ///
    /// The count of tasks currently in the running state.
    pub(crate) fn running_tasks(&self) -> usize {
        self.running_queue.running_tasks()
    }

    /// Restores all tasks and triggers a reschedule operation.
    ///
    /// This method schedules a reschedule operation to re-evaluate all tasks
    /// based on the current system state and QoS priorities.
    pub(crate) fn restore_all_tasks(&mut self) {
        info!("reschedule restore all tasks");
        // Reschedule tasks based on the current QoS status
        self.schedule_if_not_scheduled();
    }

    /// Starts a new task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task_id` - The unique identifier of the task.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the task was successfully started, or an error if the task
    /// could not be found or is in an invalid state.
    pub(crate) fn start_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        self.start_inner(uid, task_id, false)
    }

    /// Resumes a paused task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task_id` - The unique identifier of the task.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the task was successfully resumed, or an error if the task
    /// could not be found or is not in a paused state.
    pub(crate) fn resume_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        self.start_inner(uid, task_id, true)
    }

    /// Internal implementation for starting or resuming a task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task_id` - The unique identifier of the task.
    /// * `is_resume` - Boolean indicating whether this is a resume operation.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the task was successfully started or resumed, or an error if the task
    /// could not be found or is in an invalid state.
    fn start_inner(&mut self, uid: u64, task_id: u32, is_resume: bool) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        let info = RequestDb::get_instance()
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;

        // Validate task state for the requested operation
        if (is_resume && info.progress.common_data.state != State::Paused.repr)
            || (!is_resume && info.progress.common_data.state == State::Paused.repr)
        {
            return Err(ErrorCode::TaskStateErr);
        }
        // Change to Waiting state so the task can be scheduled
        database.change_status(task_id, State::Waiting)?;

        let info = RequestDb::get_instance()
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;
        if is_resume {
            Notifier::resume(&self.client_manager, info.build_notify_data());
        } else {
            // For new task starts, reset the task time
            database.update_task_time(task_id, 0);
        }

        // Check if task is already finished
        if info.progress.is_finish() {
            database.update_task_state(task_id, State::Completed, Reason::Default);
            if let Some(info) = database.get_task_info(task_id) {
                Notifier::complete(&self.client_manager, info.build_notify_data());
            }
        }

        // Check if task configuration requirements are satisfied
        if !self.check_config_satisfy(task_id)? {
            return Ok(());
        };
        
        // Add task to QoS system and trigger reschedule
        let qos_info = database
            .get_task_qos_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;
        self.qos.start_task(uid, qos_info);
        self.schedule_if_not_scheduled();
        Ok(())
    }

    /// Pauses a running task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task_id` - The unique identifier of the task.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the task was successfully paused, or an error if the task
    /// could not be found.
    pub(crate) fn pause_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        // Update task state in database
        database.change_status(task_id, State::Paused)?;
        // Remove from QoS system
        self.qos.remove_task(uid, task_id);

        // If the task was running, cancel it and schedule a reschedule
        if self.running_queue.cancel_task(task_id, uid) {
            // For upload tasks, mark for potential resume
            self.running_queue.upload_resume.insert(task_id);
            self.schedule_if_not_scheduled();
        }
        
        // Notify client of the pause
        let info = database
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;
        Notifier::pause(&self.client_manager, info.build_notify_data());
        Ok(())
    }

    /// Removes a task from the system.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task_id` - The unique identifier of the task.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the task was successfully removed, or an error if the task
    /// could not be found.
    pub(crate) fn remove_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        // Update task state in database
        database.change_status(task_id, State::Removed)?;
        // Remove from QoS system
        self.qos.remove_task(uid, task_id);

        // If the task was running, cancel it and schedule a reschedule
        if self.running_queue.cancel_task(task_id, uid) {
            self.schedule_if_not_scheduled();
        }
        
        // Clean up user file task association
        database.remove_user_file_task(task_id);
        
        // Notify client of the removal
        let info = database
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;
        Notifier::remove(&self.client_manager, info.build_notify_data());
        Ok(())
    }

    /// Stops a running task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task_id` - The unique identifier of the task.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the task was successfully stopped, or an error if the task
    /// could not be found.
    pub(crate) fn stop_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        // Update task state in database
        database.change_status(task_id, State::Stopped)?;
        // Remove from QoS system
        self.qos.remove_task(uid, task_id);

        // If the task was running, cancel it and schedule a reschedule
        if self.running_queue.cancel_task(task_id, uid) {
            self.schedule_if_not_scheduled();
        }
        Ok(())
    }

    /// Sets the maximum speed for a running task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task_id` - The unique identifier of the task.
    /// * `max_speed` - The maximum speed to set in bytes per second.
    ///
    /// # Returns
    ///
    /// `Ok(())` regardless of whether the task was found, as this operation is non-destructive.
    pub(crate) fn set_max_speed(
        &mut self,
        uid: u64,
        task_id: u32,
        max_speed: i64,
    ) -> Result<(), ErrorCode> {
        if let Some(task) = self.running_queue.get_task(uid, task_id) {
            // Use SeqCst ordering to ensure speed limit is visible to all threads immediately
            task.max_speed.store(max_speed, Ordering::SeqCst);
        }
        Ok(())
    }

    /// Changes the execution mode of a task.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task_id` - The unique identifier of the task.
    /// * `mode` - The new execution mode (foreground or background).
    ///
    /// # Returns
    ///
    /// `Ok(())` if the task mode was successfully changed, or an error if the task
    /// could not be found or is in an invalid state.
    pub(crate) fn task_set_mode(
        &mut self,
        uid: u64,
        task_id: u32,
        mode: Mode,
    ) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        // Update mode in database
        database.set_mode(task_id, mode)?;

        // Update QoS and trigger reschedule if needed
        if self.qos.task_set_mode(uid, task_id, mode) {
            self.schedule_if_not_scheduled();
        }
        
        // Update mode for running task
        if let Some(task) = self.running_queue.get_task_clone(uid, task_id) {
            task.mode.store(mode.repr, Ordering::Release);
        }
        
        // Update notification settings based on mode
        if mode == Mode::FrontEnd {
            NotificationDispatcher::get_instance().unregister_task(uid, task_id, false);
        } else if mode == Mode::BackGround {
            NotificationDispatcher::get_instance().enable_task_progress_notification(task_id);
        }
        Ok(())
    }

    /// Handles task completion.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task_id` - The unique identifier of the completed task.
    pub(crate) fn task_completed(&mut self, uid: u64, task_id: u32) {
        info!("task {} completed", task_id);
        // Mark task as finished in the running queue
        self.running_queue.task_finish(uid, task_id);

        let database = RequestDb::get_instance();
        // Remove from QoS system and trigger reschedule if needed
        if self.qos.remove_task(uid, task_id) {
            self.schedule_if_not_scheduled();
        }

        // Check if task state needs special handling
        if let Some(info) = database.get_task_qos_info(task_id) {
            // Handle failed state
            if info.state == State::Failed.repr {
                if let Some(task_info) = database.get_task_info(task_id) {
                    Scheduler::notify_fail(task_info, &self.client_manager, Reason::Default);
                    return;
                }
            }

            // Skip if task is not in a runnable state
            if info.state != State::Running.repr && info.state != State::Waiting.repr {
                return;
            }
        }

        // Mark as completed and clean up
        database.update_task_state(task_id, State::Completed, Reason::Default);
        database.remove_user_file_task(task_id);
        
        // Send completion notifications
        if let Some(info) = database.get_task_info(task_id) {
            Notifier::complete(&self.client_manager, info.build_notify_data());
            NotificationDispatcher::get_instance().publish_success_notification(&info);
        }
    }

    /// Handles task cancellation.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task_id` - The unique identifier of the task.
    /// * `mode` - The execution mode of the task.
    /// * `task_count` - Map tracking task counts by UID and mode.
    pub(crate) fn task_cancel(
        &mut self,
        uid: u64,
        task_id: u32,
        mode: Mode,
        task_count: &mut HashMap<u64, (usize, usize)>,
    ) {
        info!("task {} canceled", task_id);
        // Mark task as finished in the running queue
        self.running_queue.task_finish(uid, task_id);
        
        // Try to restart the task immediately if possible
        if self.running_queue.try_restart(uid, task_id) {
            return;
        }

        let database = RequestDb::get_instance();
        let Some(info) = database.get_task_info(task_id) else {
            error!("task {} not found in database", task_id);
            NotificationDispatcher::get_instance().unregister_task(uid, task_id, true);
            return;
        };
        
        // Handle different task states appropriately
        match State::from(info.progress.common_data.state) {
            // If running, move to waiting state due to task limits
            State::Running | State::Retrying => {
                info!("task {} waiting for task limits", task_id);
                RequestDb::get_instance().update_task_state(
                    task_id, 
                    State::Waiting, 
                    Reason::RunningTaskMeetLimits,
                );
                Notifier::waiting(&self.client_manager, task_id, WaitingCause::TaskQueue);
            }
            // If failed, notify client and reduce task count
            State::Failed => {
                info!("task {} cancel with state Failed", task_id);
                Scheduler::reduce_task_count(uid, mode, task_count);
                let reason = info.common_data.reason;
                Scheduler::notify_fail(info, &self.client_manager, Reason::from(reason));
            }
            // If stopped or removed, clean up and try restart
            State::Stopped | State::Removed => {
                info!("task {} cancel with state Stopped or Removed", task_id);
                NotificationDispatcher::get_instance().unregister_task(uid, task_id, true);
                self.running_queue.try_restart(uid, task_id);
            }
            // For waiting tasks, determine waiting cause based on reason
            State::Waiting => {
                info!("task {} cancel with state Waiting", task_id);
                let reason = match info.common_data.reason {
                    reason if reason == Reason::AppBackgroundOrTerminate.repr => {
                        WaitingCause::AppState
                    }
                    reason
                        if reason == Reason::NetworkOffline.repr
                            || reason == Reason::UnsupportedNetworkType.repr =>
                    {
                        WaitingCause::Network
                    }
                    reason if reason == Reason::RunningTaskMeetLimits.repr => {
                        WaitingCause::TaskQueue
                    }
                    reason if reason == Reason::AccountStopped.repr => WaitingCause::UserState,
                    reason => {
                        error!("task {} cancel with other reason {}", task_id, reason);
                        WaitingCause::TaskQueue
                    }
                };
                Notifier::waiting(&self.client_manager, task_id, reason);
            }
            // Log other states for debugging
            state => {
                info!(
                    "task {} cancel state {:?} reason {:?}",
                    task_id,
                    state,
                    Reason::from(info.common_data.reason)
                );
            }
        }
    }

    /// Handles task failure.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task_id` - The unique identifier of the task.
    /// * `reason` - The reason for the task failure.
    pub(crate) fn task_failed(&mut self, uid: u64, task_id: u32, reason: Reason) {
        info!("task {} failed", task_id);
        // Mark task as finished in the running queue
        self.running_queue.task_finish(uid, task_id);

        let database = RequestDb::get_instance();
        // Remove from QoS system and trigger reschedule if needed
        if self.qos.remove_task(uid, task_id) {
            self.schedule_if_not_scheduled();
        }

        // Check if task state needs updating
        if let Some(info) = database.get_task_qos_info(task_id) {
            // Skip if task is not in a runnable state
            if info.state != State::Running.repr && info.state != State::Waiting.repr {
                return;
            }
        }

        // Update task state to failed
        database.update_task_state(task_id, State::Failed, reason);
        
        // Send failure notifications
        if let Some(info) = database.get_task_info(task_id) {
            let reason = info.common_data.reason;
            Scheduler::notify_fail(info, &self.client_manager, Reason::from(reason));
        }
    }

    /// Sends notifications about task failure to various components.
    ///
    /// # Arguments
    ///
    /// * `info` - The task information for the failed task.
    /// * `client_manager` - Manager for client notifications.
    /// * `reason` - The reason for the task failure.
    fn notify_fail(info: TaskInfo, client_manager: &ClientManagerEntry, reason: Reason) {
        // Send failure notification to client
        Notifier::fail(client_manager, info.build_notify_data());
        // Log fault information
        Notifier::faults(info.common_data.task_id, client_manager, reason);
        // Show system notification
        NotificationDispatcher::get_instance().publish_failed_notification(&info);
        // Log system event on OpenHarmony
        #[cfg(feature = "oh")]
        Self::sys_event(info);
    }

    /// Decrements the task count for a specific UID and mode.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application.
    /// * `mode` - The task execution mode.
    /// * `task_count` - Map tracking task counts by UID and mode.
    pub(crate) fn reduce_task_count(
        uid: u64,
        mode: Mode,
        task_count: &mut HashMap<u64, (usize, usize)>,
    ) {
        if let Some((front, back)) = task_count.get_mut(&uid) {
            match mode {
                Mode::FrontEnd => {
                    if *front > 0 {
                        *front -= 1;
                    }
                }
                _ => {
                    if *back > 0 {
                        *back -= 1;
                    }
                }
            }
        }
    }

    /// Logs system events for failed tasks (OpenHarmony only).
    ///
    /// # Arguments
    ///
    /// * `info` - The task information for the failed task.
    #[cfg(feature = "oh")]
    pub(crate) fn sys_event(info: TaskInfo) {
        use crate::sys_event::sys_task_fault;

        let index = info.progress.common_data.index;
        let size = info.file_specs.len();
        let action = match info.action() {
            Action::Download => "DOWNLOAD",
            Action::Upload => "UPLOAD",
            _ => "UNKNOWN",
        };
        let reason = Reason::from(info.common_data.reason);

        // Log system event with task failure details
        sys_task_fault(
            action,
            size as i32,
            (size - index) as i32,
            index as i32,
            reason.repr as i32,
        );
    }

    /// Handles system state changes and updates tasks accordingly.
    ///
    /// # Arguments
    ///
    /// * `f` - Function that processes the state change and returns SQL statements.
    /// * `t` - State change payload.
    pub(crate) fn on_state_change<T, F>(&mut self, f: F, t: T)
    where
        F: FnOnce(&mut state::Handler, T) -> Option<SqlList>,
    {
        // Process the state change and get SQL statements
        let Some(sql_list) = f(&mut self.state_handler, t) else {
            return;
        };
        
        // Execute SQL statements to update database
        let db = RequestDb::get_instance();
        for sql in sql_list {
            if let Err(e) = db.execute(&sql) {
                error!("TaskManager update network failed {:?}", e);
            };
        }
        
        // Reload and reschedule all tasks based on new state
        self.reload_all_tasks();
    }

    /// Reloads all tasks and triggers a reschedule.
    ///
    /// This method reloads all tasks in the QoS system and schedules a reschedule
    /// operation to re-evaluate all tasks based on updated information.
    pub(crate) fn reload_all_tasks(&mut self) {
        self.qos.reload_all_tasks();
        self.schedule_if_not_scheduled();
    }

    /// Handles changes to the Resource Scheduling Service (RSS) level.
    ///
    /// # Arguments
    ///
    /// * `level` - The new RSS level.
    pub(crate) fn on_rss_change(&mut self, level: i32) {
        // Update RSS level and get new QoS settings
        if let Some(new_rss) = self.state_handler.update_rss_level(level) {
            // Apply new RSS settings to QoS system
            self.qos.change_rss(new_rss);
            // Trigger reschedule
            self.schedule_if_not_scheduled();
        }
    }

    /// Schedules a reschedule operation if one is not already pending.
    ///
    /// This method prevents multiple reschedule operations from being scheduled
    /// concurrently by setting a flag and sending a single reschedule event.
    fn schedule_if_not_scheduled(&mut self) {
        if self.resort_scheduled {
            return;
        }
        self.resort_scheduled = true;
        let task_manager = self.task_manager.clone();
        task_manager.send_event(TaskManagerEvent::Reschedule);
    }

    /// Performs the reschedule operation to update task priorities and execution.
    ///
    /// This method:
    /// 1. Clears the reschedule flag
    /// 2. Gets QoS changes based on current system state
    /// 3. Applies changes to the running queue
    /// 4. Removes tasks that should no longer be scheduled
    /// 5. Reloads tasks if any were removed
    pub(crate) fn reschedule(&mut self) {
        // Clear the reschedule flag
        self.resort_scheduled = false;
        
        // Get QoS changes based on current system state
        let changes = self.qos.reschedule(&self.state_handler);
        
        // Apply changes to running queue and collect tasks to remove
        let mut qos_remove_queue = vec![];
        self.running_queue
            .reschedule(changes, &mut qos_remove_queue);
        
        // Remove tasks that should no longer be in the QoS system
        for (uid, task_id) in qos_remove_queue.iter() {
            self.qos.apps.remove_task(*uid, *task_id);
        }
        
        // Reload all tasks if any were removed
        if !qos_remove_queue.is_empty() {
            self.reload_all_tasks();
        }
    }

    /// Checks if a task's configuration requirements are currently satisfied.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The unique identifier of the task.
    ///
    /// # Returns
    ///
    /// `Ok(true)` if all requirements are satisfied, `Ok(false)` if requirements
    /// are not met but the task can wait, or an error if the task could not be found.
    pub(crate) fn check_config_satisfy(&self, task_id: u32) -> Result<bool, ErrorCode> {
        let database = RequestDb::get_instance();
        let config = database
            .get_task_config(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;

        // Check if network requirements are satisfied
        if let Err(reason) = config.satisfy_network(self.state_handler.network()) {
            info!(
                "task {} started, waiting for network {:?}",
                task_id,
                self.state_handler.network()
            );
            // Put task in waiting state due to network
            database.update_task_state(task_id, State::Waiting, reason);
            Notifier::waiting(&self.client_manager, task_id, WaitingCause::Network);
            return Ok(false);
        }

        // Check if foreground requirements are satisfied
        if !config.satisfy_foreground(self.state_handler.foreground_abilities()) {
            info!(
                "task {} started, waiting for app {}",
                task_id, config.common_data.uid
            );
            // Put task in waiting state due to app state
            database.update_task_state(task_id, State::Waiting, Reason::AppBackgroundOrTerminate);
            Notifier::waiting(&self.client_manager, task_id, WaitingCause::AppState);
            return Ok(false);
        }
        
        // All requirements satisfied
        Ok(true)
    }

    /// Clears tasks that have been inactive for more than one month.
    ///
    /// This method identifies tasks that were created more than one month ago
    /// and stops them, removing them from the scheduling system.
    pub(crate) fn clear_timeout_tasks(&mut self) {
        let current_time = get_current_timestamp();
        // Identify tasks older than one month
        let timeout_tasks = self
            .tasks()
            .filter(|task| current_time - task.ctime > MILLISECONDS_IN_ONE_MONTH)
            .cloned()
            .collect::<Vec<_>>();
            
        if timeout_tasks.is_empty() {
            return;
        }
        
        let database = RequestDb::get_instance();
        for task in timeout_tasks {
            // Try to stop the task and remove from QoS if successful
            if database
                .change_status(task.task_id(), State::Stopped)
                .is_ok()
            {
                self.qos.apps.remove_task(task.uid(), task.task_id());
            }
        }
        
        // Schedule reschedule to update task execution
        self.schedule_if_not_scheduled();
    }

    /// Attempts to retry all tasks in the running queue.
    ///
    /// This method delegates to the running queue to retry any tasks that failed
    /// but are eligible for retry.
    pub(crate) fn retry_all_tasks(&mut self) {
        self.running_queue.retry_all_tasks();
    }

    /// Shuts down the scheduler and running queue.
    ///
    /// This method properly cleans up resources and stops all running tasks.
    pub(crate) fn shutdown(&mut self) {
        self.running_queue.shutdown();
    }
}

impl RequestDb {
    /// Changes the status of a task in the database.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The unique identifier of the task.
    /// * `new_state` - The new state to set for the task.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the status was successfully changed, or an error if the task
    /// could not be found or if the state change failed.
    fn change_status(&self, task_id: u32, new_state: State) -> Result<(), ErrorCode> {
        // Get current task information
        let info = RequestDb::get_instance()
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;

        let old_state = info.progress.common_data.state;
        // Check if state is already the requested state
        if old_state == new_state.repr {
            if new_state == State::Removed {
                return Err(ErrorCode::TaskNotFound);
            } else {
                return Err(ErrorCode::TaskStateErr);
            }
        }
        
        // Generate appropriate SQL for the state change
        let sql = match new_state {
            State::Paused => sql::pause_task(task_id),
            State::Running => sql::start_task(task_id),
            State::Stopped => sql::stop_task(task_id),
            State::Removed => sql::remove_task(task_id),
            State::Waiting => sql::start_task(task_id),
            _ => return Err(ErrorCode::Other),
        };

        // Execute the SQL statement
        RequestDb::get_instance()
            .execute(&sql)
            .map_err(|_| ErrorCode::SystemApi)?;

        // Verify the state change was successful
        let info = RequestDb::get_instance()
            .get_task_info(task_id)
            .ok_or(ErrorCode::SystemApi)?;
        if info.progress.common_data.state != new_state.repr {
            return Err(ErrorCode::TaskStateErr);
        }

        // Unregister notifications for certain state transitions
        if (old_state == State::Initialized.repr
            || old_state == State::Waiting.repr
            || old_state == State::Paused.repr)
            && (new_state == State::Stopped || new_state == State::Removed)
        {
            NotificationDispatcher::get_instance().unregister_task(info.uid(), task_id, true);
        }
        Ok(())
    }

    /// Sets the execution mode of a task in the database.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The unique identifier of the task.
    /// * `mode` - The new execution mode to set.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the mode was successfully changed, or an error if the task
    /// could not be found or if the mode change failed.
    fn set_mode(&self, task_id: u32, mode: Mode) -> Result<(), ErrorCode> {
        // Get current task information
        let info = RequestDb::get_instance()
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;
        let old_mode = info.common_data.mode;
        
        // If already in the requested mode, no change needed
        if old_mode == mode.repr {
            return Ok(());
        }
        
        // Generate SQL for mode change
        let sql = sql::task_set_mode(task_id, mode);
        
        // Execute the SQL statement
        RequestDb::get_instance()
            .execute(&sql)
            .map_err(|_| ErrorCode::SystemApi)?;
            
        // Verify the mode change was successful
        let info = RequestDb::get_instance()
            .get_task_info(task_id)
            .ok_or(ErrorCode::SystemApi)?;
        if info.common_data.mode != mode.repr {
            return Err(ErrorCode::TaskStateErr);
        }
        Ok(())
    }
}
