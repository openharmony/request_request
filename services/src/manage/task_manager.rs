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

use std::ops::{Deref, DerefMut};
use std::time::Duration;

use samgr::manage::SystemAbilityManager;
use ylong_runtime::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use ylong_runtime::time::sleep;

use super::app_state::AppStateManagerTx;
// use super::app_state::AppStateManager;
use super::events::{ScheduleEvent, ServiceEvent, StateEvent, TaskEvent, TaskManagerEvent};
use crate::error::ErrorCode;
use crate::init::PANIC_INFO;
use crate::manage::app_state::AppStateManager;
use crate::manage::database::Database;
use crate::manage::network::NetworkManager;
use crate::manage::scheduler::Scheduler;
use crate::service::client::ClientManagerEntry;
use crate::service::runcount::RunCountManagerEntry;
use crate::task::ffi::NetworkInfo;
use crate::task::info::ApplicationState;

const CLEAR_INTERVAL: u64 = 30 * 60;
const LOG_INTERVAL: u64 = 5 * 60;
#[allow(dead_code)]
const MILLISECONDS_IN_ONE_DAY: u64 = 24 * 60 * 60 * 1000;
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
    pub(crate) scheduler: Scheduler,
    pub(crate) rx: TaskManagerRx,
    pub(crate) app_state_manager: AppStateManagerTx,
    pub(crate) client_manager: ClientManagerEntry,
}

impl TaskManager {
    pub(crate) fn init(
        runcount_manager: RunCountManagerEntry,
        client_manager: ClientManagerEntry,
    ) -> TaskManagerTx {
        debug!("TaskManager init");

        let (tx, rx) = unbounded_channel();
        let tx = TaskManagerTx::new(tx);
        let rx = TaskManagerRx::new(rx);

        let app_state_manager = AppStateManager::init(client_manager.clone(), tx.clone());

        let task_manager = Self::new(
            tx.clone(),
            rx,
            runcount_manager,
            app_state_manager,
            client_manager,
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
        ylong_runtime::spawn(restore_all_tasks(tx.clone()));

        ylong_runtime::spawn(clear_timeout_tasks(tx.clone()));
        ylong_runtime::spawn(log_all_task_info(tx.clone()));
        ylong_runtime::spawn(task_manager.run());
        tx
    }

    pub(crate) fn new(
        tx: TaskManagerTx,
        rx: TaskManagerRx,
        runcount_manager: RunCountManagerEntry,
        app_state_manager: AppStateManagerTx,
        client_manager: ClientManagerEntry,
    ) -> Self {
        Self {
            scheduler: Scheduler::init(
                tx,
                runcount_manager,
                app_state_manager.clone(),
                client_manager.clone(),
            ),
            rx,
            app_state_manager,
            client_manager,
        }
    }

    async fn run(mut self) {
        loop {
            let event = match self.rx.recv().await {
                Ok(event) => event,
                Err(e) => {
                    error!("TaskManager receives error {:?}", e);
                    continue;
                }
            };

            match event {
                TaskManagerEvent::Service(event) => self.handle_service_event(event).await,
                TaskManagerEvent::State(event) => self.handle_state_event(event).await,
                TaskManagerEvent::Task(event) => self.handle_task_event(event).await,
                TaskManagerEvent::Schedule(event) => {
                    if self.handle_schedule_event(event).await {
                        info!("TaskManager unload succeed");
                        // If unload_sa success, breaks this loop.
                        return;
                    }
                }
                TaskManagerEvent::Device(level) => {
                    self.scheduler.on_rss_change(level).await;
                }
            }

            debug!("TaskManager handles events finished");
        }
    }

    async fn handle_service_event(&mut self, event: ServiceEvent) {
        debug!("TaskManager handles service event {:?}", event);

        match event {
            ServiceEvent::Construct(msg, tx) => {
                let _ = tx.send(self.create(msg.config).await);
            }
            ServiceEvent::Start(uid, task_id, tx) => {
                let _ = tx.send(self.start(uid, task_id).await);
            }
            ServiceEvent::Stop(uid, task_id, tx) => {
                let _ = tx.send(self.stop(uid, task_id).await);
            }
            ServiceEvent::Pause(uid, task_id, tx) => {
                let _ = tx.send(self.pause(uid, task_id).await);
            }
            ServiceEvent::Resume(uid, task_id, tx) => {
                let _ = tx.send(self.resume(uid, task_id).await);
            }
            ServiceEvent::Remove(uid, task_id, tx) => {
                let _ = tx.send(self.remove(uid, task_id).await);
            }
            ServiceEvent::Show(uid, task_id, tx) => {
                let _ = tx.send(self.show(uid, task_id));
            }
            ServiceEvent::Touch(uid, task_id, token, tx) => {
                let _ = tx.send(self.touch(uid, task_id, token));
            }
            ServiceEvent::Query(task_id, action, tx) => {
                let _ = tx.send(self.query(task_id, action));
            }
            ServiceEvent::QueryMimeType(uid, task_id, tx) => {
                let _ = tx.send(self.query_mime_type(uid, task_id));
            }
            ServiceEvent::Search(filter, tx) => {
                let _ = tx.send(self.search(filter));
            }
            ServiceEvent::GetTask(uid, task_id, token, tx) => {
                let _ = tx.send(self.get_task(uid, task_id, token));
            }
            ServiceEvent::DumpAll(tx) => {
                let _ = tx.send(self.query_all_task());
            }
            ServiceEvent::DumpOne(task_id, tx) => {
                let _ = tx.send(self.query_one_task(task_id));
            }
        }
    }

    async fn handle_state_event(&mut self, event: StateEvent) {
        debug!("TaskManager handles state event {:?}", event);

        match event {
            StateEvent::NetworkChange => {
                self.handle_network_change().await;
            }
            StateEvent::AppStateChange(uid, state) => {
                self.handle_app_state_change(uid, state).await;
            }
        }
    }

    async fn handle_task_event(&mut self, event: TaskEvent) {
        debug!("TaskManager handles task event {:?}", event);

        match event {
            TaskEvent::Finished(task_id, uid, _version) => {
                self.handle_finished_task(uid, task_id).await;
            }
            TaskEvent::Subscribe(task_id, token_id, tx) => {
                let _ = tx.send(self.check_subscriber(task_id, token_id));
            }
        }
    }

    async fn handle_schedule_event(&mut self, message: ScheduleEvent) -> bool {
        debug!("TaskManager handle scheduled_message {:?}", message);

        match message {
            ScheduleEvent::ClearTimeoutTasks => self.clear_timeout_tasks(),
            ScheduleEvent::LogTasks => self.log_all_task_info(),
            ScheduleEvent::RestoreAllTasks => self.restore_all_tasks().await,
            ScheduleEvent::Unload => return self.unload_sa(),
        }
        false
    }

    async fn handle_network_change(&mut self) {
        static mut NETWORK_INFO: Option<NetworkInfo> = None;
        let manager = NetworkManager::new();
        manager.update_network_info();
        if let Some(info) = manager.get_network_info() {
            unsafe {
                if Some(info) != NETWORK_INFO {
                    NETWORK_INFO = Some(info);
                    self.scheduler.on_network_change(info).await;
                }
            }
        }
    }

    async fn handle_app_state_change(&mut self, uid: u64, state: ApplicationState) {
        self.scheduler.on_app_state_change(uid, state).await;
    }

    async fn handle_finished_task(&mut self, uid: u64, task_id: u32) {
        self.scheduler.finish_task(uid, task_id).await;
    }

    fn check_subscriber(&self, task_id: u32, token_id: u64) -> ErrorCode {
        match Database::get_instance().get_token_id(task_id) {
            Some(id) if id == token_id => ErrorCode::ErrOk,
            Some(_) => ErrorCode::Permission,
            None => ErrorCode::TaskNotFound,
        }
    }

    fn clear_timeout_tasks(&mut self) {
        self.scheduler.clear_timeout_tasks();
    }

    fn log_all_task_info(&self) {
        self.scheduler.dump_tasks();
    }

    async fn restore_all_tasks(&mut self) {
        self.scheduler.restore_all_tasks().await;
    }

    fn unload_sa(&mut self) -> bool {
        const REQUEST_SERVICE_ID: i32 = 3706;

        if !self.rx.is_empty() {
            return false;
        }

        let running_tasks = self.scheduler.running_tasks();
        if running_tasks != 0 {
            info!(
                "Running tasks num is {} when trying to unload SA",
                running_tasks,
            );
            return false;
        }

        Database::get_instance().delete_early_records();

        // check rx again for there may be new message arrive.
        if !self.rx.is_empty() {
            return false;
        }

        self.rx.close();

        info!("unload SA");

        // failed logic?
        let _ = SystemAbilityManager::unload_system_ability(REQUEST_SERVICE_ID);

        true
    }
}

#[derive(Clone)]
pub(crate) struct TaskManagerTx {
    tx: UnboundedSender<TaskManagerEvent>,
}

impl TaskManagerTx {
    fn new(tx: UnboundedSender<TaskManagerEvent>) -> Self {
        Self { tx }
    }

    pub(crate) fn send_event(&self, event: TaskManagerEvent) -> bool {
        if self.tx.send(event).is_err() {
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
}

impl Deref for TaskManagerTx {
    type Target = UnboundedSender<TaskManagerEvent>;

    fn deref(&self) -> &Self::Target {
        &self.tx
    }
}

pub(crate) struct TaskManagerRx {
    rx: UnboundedReceiver<TaskManagerEvent>,
}

impl TaskManagerRx {
    fn new(rx: UnboundedReceiver<TaskManagerEvent>) -> Self {
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

async fn restore_all_tasks(tx: TaskManagerTx) {
    sleep(Duration::from_secs(RESTORE_ALL_TASKS_INTERVAL)).await;
    let _ = tx.send_event(TaskManagerEvent::Schedule(ScheduleEvent::RestoreAllTasks));
}

async fn clear_timeout_tasks(tx: TaskManagerTx) {
    loop {
        sleep(Duration::from_secs(CLEAR_INTERVAL)).await;
        let _ = tx.send_event(TaskManagerEvent::Schedule(ScheduleEvent::ClearTimeoutTasks));
    }
}

async fn log_all_task_info(tx: TaskManagerTx) {
    loop {
        sleep(Duration::from_secs(LOG_INTERVAL)).await;
        let _ = tx.send_event(TaskManagerEvent::Schedule(ScheduleEvent::LogTasks));
    }
}
