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

//! Core task management functionality.
//! 
//! This module defines the `TaskManager` and related types that handle task lifecycle management,
//! scheduling, and event processing for the request service. It coordinates task operations
//! including creation, starting, pausing, resuming, stopping, and monitoring of tasks.

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::time::Duration;

use samgr::definition::COMM_NET_CONN_MANAGER_SYS_ABILITY_ID;
use ylong_runtime::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use ylong_runtime::sync::oneshot;
use ylong_runtime::time::sleep;

cfg_oh! {
    use samgr::manage::SystemAbilityManager;
    use crate::ability::PANIC_INFO;
    use crate::manage::account::registry_account_subscribe;
}
use super::account::{remove_account_tasks, AccountEvent};
use super::database::RequestDb;
use super::events::{
    QueryEvent, ScheduleEvent, ServiceEvent, StateEvent, TaskEvent, TaskManagerEvent,
};
use crate::config::{Action, Mode};
use crate::database::clear_database_part;
use crate::error::ErrorCode;
use crate::info::{State, TaskInfo};
use crate::manage::app_state::AppUninstallSubscriber;
use crate::manage::network::register_network_change;
use crate::manage::network_manager::NetworkManager;
use crate::manage::query::TaskFilter;
use crate::manage::scheduler::state::Handler;
use crate::manage::scheduler::Scheduler;
use crate::service::active_counter::ActiveCounter;
use crate::service::client::ClientManagerEntry;
use crate::service::notification_bar::{subscribe_notification_bar, NotificationDispatcher};
use crate::service::run_count::RunCountManagerEntry;
use crate::utils::task_event_count::{task_complete_add, task_fail_add, task_unload};
use crate::utils::{get_current_timestamp, runtime_spawn, subscribe_common_event, update_policy};

/// Interval (in seconds) for clearing timeout tasks.
const CLEAR_INTERVAL: u64 = 30 * 60;

/// Interval (in seconds) before restoring all tasks after service initialization.
const RESTORE_ALL_TASKS_INTERVAL: u64 = 10;

// TaskManager 的初始化逻辑：
//
// 首先确定任务的来源：1）来自应用的任务 2）数据库中未完成的任务。
// 其次确定 SA 拉起的时机：1）WIFI 连接拉起 SA 2）应用拉起 SA

// Qos schedule 逻辑步骤：
// 1. SA 启动时，从数据库中将存在 Waiting + QosWaiting 的任务（Qos
//    信息）及应用信息取出，存放到 Qos 结构中排序，此时触发一次初始的任务加载。
// 2. 当新任务添加到 SA 侧\网络状态变化\前后台状态变化时，更新并排序
//    Qos，触发任务加载，把可执行任务加载到内存中处理，
//    或是把不可执行任务返回数据库中。

pub(crate) struct TaskManager {
    /// Handles task scheduling and execution
    pub(crate) scheduler: Scheduler,
    /// Channel receiver for task manager events
    pub(crate) rx: TaskManagerRx,
    /// Manages client connections and permissions
    pub(crate) client_manager: ClientManagerEntry,
    /// Tracks task counts per user ID (foreground, background)
    pub(crate) task_count: HashMap<u64, (usize, usize)>,
}

impl TaskManager {
    /// Initializes the task manager and starts its event processing loop.
    /// 
    /// Sets up subscriptions for system events, network changes, and notifications,
    /// then initializes and starts the task manager's main processing loop.
    /// 
    /// # Arguments
    /// 
    /// * `runcount_manager` - Manager for tracking task execution counts
    /// * `client_manager` - Manager for client connections and permissions
    /// * `active_counter` - Counter for tracking active tasks
    /// * `network` - Network state tracker (non-OH feature only)
    /// 
    /// # Returns
    /// 
    /// Returns a `TaskManagerTx` for sending events to the task manager
    pub(crate) fn init(
        runcount_manager: RunCountManagerEntry,
        client_manager: ClientManagerEntry,
        active_counter: ActiveCounter,
        #[cfg(not(feature = "oh"))] network: Network,
    ) -> TaskManagerTx {
        debug!("TaskManager init");

        let (tx, rx) = unbounded_channel();
        let tx = TaskManagerTx::new(tx);
        let rx = TaskManagerRx::new(rx);

        #[cfg(feature = "oh")]
        registry_account_subscribe(tx.clone());

        #[cfg(feature = "oh")]
        {
            let mut network_manager = NetworkManager::get_instance().lock().unwrap();
            network_manager.tx = Some(tx.clone());
            SystemAbilityManager::subscribe_system_ability(
                COMM_NET_CONN_MANAGER_SYS_ABILITY_ID,
                |_, _| {
                    register_network_change();
                },
                |_, _| {
                    info!("network service died");
                },
            );
        }
        #[cfg(feature = "oh")]
        register_network_change();
        subscribe_notification_bar(tx.clone());

        if let Err(e) = subscribe_common_event(
            vec![
                "usual.event.PACKAGE_REMOVED",
                "usual.event.BUNDLE_REMOVED",
                "usual.event.PACKAGE_FULLY_REMOVED",
            ],
            AppUninstallSubscriber::new(tx.clone()),
        ) {
            error!("Subscribe app uninstall event failed: {}", e);
            sys_event!(
                ExecFault,
                DfxCode::EVENT_FAULT_01,
                &format!("Subscribe app uninstall event failed: {}", e)
            );
        }

        let task_manager = Self::new(
            tx.clone(),
            rx,
            runcount_manager,
            client_manager,
            active_counter,
        );

        // Performance optimization tips for task restoring:
        //
        // When SA is initializing, it will create and initialize an app sorting
        // queue in `scheduler.QoS`, but there is no task rescheduling or
        // execution at this time.
        //
        // After SA initialization, we will start a coroutine to recover all
        // tasks, which is used to notify `TaskManager` to recover waiting tasks
        // in the database.
        //
        // If a new task is started at this time, this future can
        // be removed because the scheduler will also be rearranged in the
        // startup logic of the new task.
        runtime_spawn(restore_all_tasks(tx.clone()));

        runtime_spawn(clear_timeout_tasks(tx.clone()));
        runtime_spawn(task_manager.run());
        tx
    }

    /// Creates a new task manager instance.
    /// 
    /// # Arguments
    /// 
    /// * `tx` - Channel for sending events to the task manager
    /// * `rx` - Channel for receiving events from the task manager
    /// * `run_count_manager` - Manager for tracking task execution counts
    /// * `client_manager` - Manager for client connections and permissions
    /// * `active_counter` - Counter for tracking active tasks
    /// 
    /// # Returns
    /// 
    /// Returns a new `TaskManager` instance
    pub(crate) fn new(
        tx: TaskManagerTx,
        rx: TaskManagerRx,
        run_count_manager: RunCountManagerEntry,
        client_manager: ClientManagerEntry,
        active_counter: ActiveCounter,
    ) -> Self {
        Self {
            scheduler: Scheduler::init(
                tx.clone(),
                run_count_manager,
                client_manager.clone(),
                active_counter,
            ),
            rx,
            client_manager,
            task_count: HashMap::new(),
        }
    }

    /// Runs the task manager's main event processing loop.
    /// 
    /// Continuously receives and processes events, delegating to specialized
    /// handlers based on event type.
    async fn run(mut self) {
        let db = RequestDb::get_instance();
        db.clear_invalid_records();
        loop {
            let event = match self.rx.recv().await {
                Ok(event) => event,
                Err(e) => {
                    error!("TaskManager receives error {:?}", e);
                    continue;
                }
            };

            match event {
                TaskManagerEvent::Service(event) => self.handle_service_event(event),
                TaskManagerEvent::State(event) => self.handle_state_event(event),
                TaskManagerEvent::Task(event) => self.handle_task_event(event),
                TaskManagerEvent::Schedule(event) => {
                    if self.handle_schedule_event(event) {
                        info!("TaskManager unload ok");
                        // If unload_sa success, can not breaks this loop.
                    }
                }
                TaskManagerEvent::Device(level) => {
                    self.scheduler.on_rss_change(level);
                }
                TaskManagerEvent::Account(event) => self.handle_account_event(event),
                TaskManagerEvent::Query(query) => self.handle_query_event(query),
                TaskManagerEvent::Reschedule => self.scheduler.reschedule(),
            }

            debug!("TaskManager handles events finished");
        }
    }

    /// Handles account-related events.
    /// 
    /// Processes account removal and account change events by removing tasks
    /// associated with removed accounts or updating scheduler state.
    /// 
    /// # Arguments
    /// 
    /// * `event` - The account event to handle
    pub(crate) fn handle_account_event(&mut self, event: AccountEvent) {
        match event {
            AccountEvent::Remove(user_id) => remove_account_tasks(user_id),
            AccountEvent::Changed => self.scheduler.on_state_change(Handler::update_account, ()),
        }
    }

    /// Handles service-related events.
    /// 
    /// Processes various service events like constructing tasks, starting/stopping tasks,
    /// setting task properties, and querying task information.
    /// 
    /// # Arguments
    /// 
    /// * `event` - The service event to handle
    fn handle_service_event(&mut self, event: ServiceEvent) {
        debug!("TaskManager handles service event {:?}", event);

        match event {
            ServiceEvent::Construct(msg, tx) => {
                let _ = tx.send(self.create(msg.config));
            }
            ServiceEvent::Start(uid, task_id, tx) => {
                let _ = tx.send(self.start(uid, task_id));
            }
            ServiceEvent::Stop(uid, task_id, tx) => {
                let _ = tx.send(self.stop(uid, task_id));
            }
            ServiceEvent::Pause(uid, task_id, tx) => {
                let _ = tx.send(self.pause(uid, task_id));
            }
            ServiceEvent::Resume(uid, task_id, tx) => {
                let _ = tx.send(self.resume(uid, task_id));
            }
            ServiceEvent::Remove(uid, task_id, tx) => {
                let _ = tx.send(self.remove(uid, task_id));
            }
            ServiceEvent::SetMaxSpeed(uid, task_id, max_speed, tx) => {
                let _ = tx.send(self.set_max_speed(uid, task_id, max_speed));
            }
            ServiceEvent::DumpAll(tx) => {
                let _ = tx.send(self.query_all_task());
            }
            ServiceEvent::DumpOne(task_id, tx) => {
                let _ = tx.send(self.query_one_task(task_id));
            }
            ServiceEvent::AttachGroup(uid, task_ids, group, tx) => {
                let _ = tx.send(self.attach_group(uid, task_ids, group));
            }
            ServiceEvent::SetMode(uid, task_id, mode, tx) => {
                let _ = tx.send(self.set_mode(uid, task_id, mode));
            }
        }
    }

    /// Handles state-related events.
    /// 
    /// Processes system state changes like network changes, app foreground/background transitions,
    /// app uninstalls, and special process terminations.
    /// 
    /// # Arguments
    /// 
    /// * `event` - The state event to handle
    fn handle_state_event(&mut self, event: StateEvent) {
        debug!("TaskManager handles state event {:?}", event);

        match event {
            StateEvent::Network => {
                self.scheduler.retry_all_tasks();
                self.scheduler.on_state_change(Handler::update_network, ());
            }

            StateEvent::ForegroundApp(uid) => {
                self.scheduler.on_state_change(Handler::update_top_uid, uid);
            }
            StateEvent::Background(uid) => self
                .scheduler
                .on_state_change(Handler::update_background, uid),
            StateEvent::BackgroundTimeout(uid) => self
                .scheduler
                .on_state_change(Handler::update_background_timeout, uid),
            StateEvent::AppUninstall(uid) => {
                self.scheduler.on_state_change(Handler::app_uninstall, uid);
            }
            StateEvent::SpecialTerminate(uid) => {
                self.scheduler
                    .on_state_change(Handler::special_process_terminate, uid);
            }
        }
    }

    /// Handles task-related events.
    /// 
    /// Processes task lifecycle events like task subscription checks, completions,
    /// cancellations, failures, and offline status changes.
    /// 
    /// # Arguments
    /// 
    /// * `event` - The task event to handle
    fn handle_task_event(&mut self, event: TaskEvent) {
        debug!("TaskManager handles task event {:?}", event);

        match event {
            TaskEvent::Subscribe(task_id, token_id, tx) => {
                let _ = tx.send(self.check_subscriber(task_id, token_id));
            }
            TaskEvent::Completed(task_id, uid, mode) => {
                Scheduler::reduce_task_count(uid, mode, &mut self.task_count);
                task_complete_add();
                self.scheduler.task_completed(uid, task_id);
            }
            TaskEvent::Running(task_id, uid, mode) => {
                self.scheduler
                    .task_cancel(uid, task_id, mode, &mut self.task_count);
            }
            TaskEvent::Failed(task_id, uid, reason, mode) => {
                Scheduler::reduce_task_count(uid, mode, &mut self.task_count);
                task_fail_add();
                self.scheduler.task_failed(uid, task_id, reason);
            }
            TaskEvent::Offline(task_id, uid, mode) => {
                self.scheduler
                    .task_cancel(uid, task_id, mode, &mut self.task_count);
            }
        };
    }

    /// Handles scheduled events.
    /// 
    /// Processes scheduled operations like clearing timeout tasks, restoring tasks,
    /// unloading the service, and shutting down.
    /// 
    /// # Arguments
    /// 
    /// * `message` - The scheduled event to handle
    /// 
    /// # Returns
    /// 
    /// Returns `true` if the service was successfully unloaded, otherwise `false`
    fn handle_schedule_event(&mut self, message: ScheduleEvent) -> bool {
        debug!("TaskManager handle scheduled_message {:?}", message);

        match message {
            ScheduleEvent::ClearTimeoutTasks => self.clear_timeout_tasks(),
            ScheduleEvent::RestoreAllTasks => self.restore_all_tasks(),
            ScheduleEvent::Unload => return self.unload_sa(),
            ScheduleEvent::Shutdown => self.shutdown(),
        }
        false
    }

    /// Checks if a subscriber has permission to access a task.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task to check
    /// * `token_id` - The token ID of the subscriber
    /// 
    /// # Returns
    /// 
    /// Returns `ErrorCode::ErrOk` if the subscriber has permission, otherwise
    /// an appropriate error code
    fn check_subscriber(&self, task_id: u32, token_id: u64) -> ErrorCode {
        match RequestDb::get_instance().query_task_token_id(task_id) {
            Ok(id) if id == token_id => ErrorCode::ErrOk,
            Ok(_) => ErrorCode::Permission,
            Err(_) => ErrorCode::TaskNotFound,
        }
    }

    /// Shuts down the scheduler.
    /// 
    /// Terminates all ongoing tasks and prepares for service shutdown.
    fn shutdown(&mut self) {
        self.scheduler.shutdown();
    }

    /// Clears tasks that have timed out.
    /// 
    /// Delegates to the scheduler to identify and clean up tasks that have exceeded
    /// their allowed execution time.
    fn clear_timeout_tasks(&mut self) {
        self.scheduler.clear_timeout_tasks();
    }

    /// Restores all tasks from the database.
    /// 
    /// Delegates to the scheduler to reload and resume tasks that were saved in the database.
    fn restore_all_tasks(&mut self) {
        self.scheduler.restore_all_tasks();
    }

    /// Checks if there are any running tasks or pending events.
    /// 
    /// Used before unloading the service to ensure all tasks are completed and no new
    /// events are pending.
    /// 
    /// # Returns
    /// 
    /// Returns `true` if there are any running tasks or pending events, otherwise `false`
    fn check_any_tasks(&self) -> bool {
        let running_tasks = self.scheduler.running_tasks();
        if running_tasks != 0 {
            info!("running {} tasks when unload SA", running_tasks,);
            return true;
        }

        // check rx again for there may be new message arrive.
        if !self.rx.is_empty() {
            return true;
        }
        false
    }

    /// Unloads the system ability.
    /// 
    /// Cleans up resources, removes old tasks from the database, and unloads the system ability
    /// if there are no running tasks or pending events.
    /// 
    /// # Returns
    /// 
    /// Returns `true` if the system ability was successfully unloaded, otherwise `false`
    fn unload_sa(&mut self) -> bool {
        if self.check_any_tasks() {
            return false;
        }

        const TIMES: usize = 10;
        const PRE_COUNT: usize = 1000;

        for _i in 0..TIMES {
            let remain = clear_database_part(PRE_COUNT).unwrap_or(false);
            if self.check_any_tasks() {
                return false;
            }
            if !remain {
                break;
            }
        }
        NotificationDispatcher::get_instance().clear_group_info();

        const REQUEST_SERVICE_ID: i32 = 3706;
        const ONE_MONTH: i64 = 30 * 24 * 60 * 60 * 1000;

        let db = RequestDb::get_instance();

        let filter = TaskFilter {
            before: get_current_timestamp() as i64,
            after: get_current_timestamp() as i64 - ONE_MONTH,
            state: State::Waiting.repr,
            action: Action::Any.repr,
            mode: Mode::Any.repr,
        };

        let bundle_name = "*".to_string();

        let task_ids = db.system_search_task(filter, bundle_name);

        info!("unload SA");
        task_unload();

        let any_tasks = task_ids.is_empty();
        let update_on_demand_policy = update_policy(any_tasks);
        if update_on_demand_policy != 0 {
            info!("Update on demand policy failed");
        }

        // failed logic?
        #[cfg(feature = "oh")]
        let _ = SystemAbilityManager::unload_system_ability(REQUEST_SERVICE_ID);

        true
    }
}

#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    // Task QoS information used for task scheduling priorities
    #[derive(Clone, Debug, Copy)]
    pub(crate) struct TaskQosInfo {
        pub(crate) task_id: u32,
        pub(crate) action: u8,
        pub(crate) mode: u8,
        pub(crate) state: u8,
        pub(crate) priority: u32,
    }

    // C++ interface includes
    unsafe extern "C++" {
        include!("system_ability_manager.h");
        include!("system_ability_on_demand_event.h");
    }
}

/// Sender for task manager events.
/// 
/// Provides methods for sending various types of events to the task manager
/// and for querying task information.
#[allow(unreachable_pub)]
#[derive(Clone)]
pub struct TaskManagerTx {
    /// Internal channel sender
    pub(crate) tx: UnboundedSender<TaskManagerEvent>,
}

impl TaskManagerTx {
    /// Creates a new task manager event sender.
    /// 
    /// # Arguments
    /// 
    /// * `tx` - The underlying channel sender
    /// 
    /// # Returns
    /// 
    /// Returns a new `TaskManagerTx` instance
    pub(crate) fn new(tx: UnboundedSender<TaskManagerEvent>) -> Self {
        Self { tx }
    }

    /// Sends an event to the task manager.
    /// 
    /// # Arguments
    /// 
    /// * `event` - The event to send
    /// 
    /// # Returns
    /// 
    /// Returns `true` if the event was successfully sent, otherwise `false`
    pub(crate) fn send_event(&self, event: TaskManagerEvent) -> bool {
        if self.tx.send(event).is_err() {
            #[cfg(feature = "oh")]
            unsafe {
                if let Some(e) = PANIC_INFO.as_ref() {
                    error!("Sends TaskManager event failed {}", e);
                } else {
                    info!("TaskManager is unloading");
                }
            }
            return false;
        }
        true
    }

    /// Notifies the task manager that an application has moved to the foreground.
    /// 
    /// # Arguments
    /// 
    /// * `uid` - The user ID of the application
    pub(crate) fn notify_foreground_app_change(&self, uid: u64) {
        let _ = self.send_event(TaskManagerEvent::State(StateEvent::ForegroundApp(uid)));
    }

    /// Notifies the task manager that an application has moved to the background.
    /// 
    /// # Arguments
    /// 
    /// * `uid` - The user ID of the application
    pub(crate) fn notify_app_background(&self, uid: u64) {
        let _ = self.send_event(TaskManagerEvent::State(StateEvent::Background(uid)));
    }

    /// Triggers a background timeout for an application.
    /// 
    /// # Arguments
    /// 
    /// * `uid` - The user ID of the application
    pub(crate) fn trigger_background_timeout(&self, uid: u64) {
        let _ = self.send_event(TaskManagerEvent::State(StateEvent::BackgroundTimeout(uid)));
    }

    /// Notifies the task manager that a special process has terminated.
    /// 
    /// # Arguments
    /// 
    /// * `uid` - The user ID of the process
    pub(crate) fn notify_special_process_terminate(&self, uid: u64) {
        let _ = self.send_event(TaskManagerEvent::State(StateEvent::SpecialTerminate(uid)));
    }

    /// Retrieves task information for a specific user.
    /// 
    /// # Arguments
    /// 
    /// * `uid` - The user ID to verify ownership
    /// * `task_id` - The ID of the task to retrieve
    /// 
    /// # Returns
    /// 
    /// Returns `Some(TaskInfo)` if the task exists and is owned by the specified user,
    /// otherwise `None`
    pub(crate) fn show(&self, uid: u64, task_id: u32) -> Option<TaskInfo> {
        let (tx, rx) = oneshot::channel();
        let event = QueryEvent::Show(task_id, uid, tx);
        let _ = self.send_event(TaskManagerEvent::Query(event));
        match ylong_runtime::block_on(rx) {
            Ok(task_info) => task_info,
            Err(error) => {
                error!("In `show`, block on failed, err {}", error);
                None
            }
        }
    }

    /// Queries task information with action permission checking.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task to retrieve
    /// * `action` - The action to check permissions against
    /// 
    /// # Returns
    /// 
    /// Returns `Some(TaskInfo)` with sensitive data sanitized if the task exists and
    /// the action has sufficient permissions, otherwise `None`
    pub(crate) fn query(&self, task_id: u32, action: Action) -> Option<TaskInfo> {
        let (tx, rx) = oneshot::channel();
        let event = QueryEvent::Query(task_id, action, tx);
        let _ = self.send_event(TaskManagerEvent::Query(event));
        match ylong_runtime::block_on(rx) {
            Ok(task_info) => task_info,
            Err(error) => {
                error!("In `query`, block on failed, err {}", error);
                None
            }
        }
    }

    /// Retrieves task information with token authentication.
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
    /// is owned by the specified user, and the token matches, otherwise `None`
    pub(crate) fn touch(&self, uid: u64, task_id: u32, token: String) -> Option<TaskInfo> {
        let (tx, rx) = oneshot::channel();
        let event = QueryEvent::Touch(task_id, uid, token, tx);
        let _ = self.send_event(TaskManagerEvent::Query(event));
        match ylong_runtime::block_on(rx) {
            Ok(task_info) => task_info,
            Err(error) => {
                error!("In `touch`, block on failed, err {}", error);
                None
            }
        }
    }
}

/// Receiver for task manager events.
/// 
/// Provides a wrapper around the unbounded receiver channel that allows
/// the task manager to receive and process events.
pub(crate) struct TaskManagerRx {
    rx: UnboundedReceiver<TaskManagerEvent>,
}

impl TaskManagerRx {
    /// Creates a new task manager event receiver.
    /// 
    /// # Arguments
    /// 
    /// * `rx` - The underlying channel receiver
    /// 
    /// # Returns
    /// 
    /// Returns a new `TaskManagerRx` instance
    pub(crate) fn new(rx: UnboundedReceiver<TaskManagerEvent>) -> Self {
        Self { rx }
    }
}

impl Deref for TaskManagerRx {
    type Target = UnboundedReceiver<TaskManagerEvent>;

    fn deref(&self) -> &Self::Target {
        &self.rx
    }
}

impl DerefMut for TaskManagerRx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rx
    }
}

/// Restores all tasks from the database after a delay.
/// 
/// Waits for a specified interval after service initialization, then triggers
/// the restoration of all tasks from the database.
/// 
/// # Arguments
/// 
/// * `tx` - The task manager event sender to use for triggering the restore
async fn restore_all_tasks(tx: TaskManagerTx) {
    sleep(Duration::from_secs(RESTORE_ALL_TASKS_INTERVAL)).await;
    let _ = tx.send_event(TaskManagerEvent::Schedule(ScheduleEvent::RestoreAllTasks));
}

/// Periodically clears timeout tasks.
/// 
/// Continuously runs at a specified interval, triggering the clearing of
/// timeout tasks each time.
/// 
/// # Arguments
/// 
/// * `tx` - The task manager event sender to use for triggering the clear
async fn clear_timeout_tasks(tx: TaskManagerTx) {
    loop {
        sleep(Duration::from_secs(CLEAR_INTERVAL)).await;
        let _ = tx.send_event(TaskManagerEvent::Schedule(ScheduleEvent::ClearTimeoutTasks));
    }
}
