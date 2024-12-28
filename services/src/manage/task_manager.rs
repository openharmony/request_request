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
use crate::error::ErrorCode;
use crate::info::TaskInfo;
use crate::manage::app_state::AppUninstallSubscriber;
use crate::manage::network::register_network_change;
use crate::manage::network_manager::NetworkManager;
use crate::manage::scheduler::state::Handler;
use crate::manage::scheduler::Scheduler;
use crate::service::client::ClientManagerEntry;
use crate::service::notification_bar::subscribe_notification_bar;
use crate::service::run_count::RunCountManagerEntry;
use crate::utils::{runtime_spawn, subscribe_common_event};

const CLEAR_INTERVAL: u64 = 30 * 60;
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
    pub(crate) client_manager: ClientManagerEntry,
    // first usize for foreground , seconde for background
    pub(crate) task_count: HashMap<u64, (usize, usize)>,
}

impl TaskManager {
    pub(crate) fn init(
        runcount_manager: RunCountManagerEntry,
        client_manager: ClientManagerEntry,
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
        }

        let task_manager = Self::new(tx.clone(), rx, runcount_manager, client_manager);

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

    pub(crate) fn new(
        tx: TaskManagerTx,
        rx: TaskManagerRx,
        run_count_manager: RunCountManagerEntry,
        client_manager: ClientManagerEntry,
    ) -> Self {
        Self {
            scheduler: Scheduler::init(tx.clone(), run_count_manager, client_manager.clone()),
            rx,
            client_manager,
            task_count: HashMap::new(),
        }
    }

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

    pub(crate) fn handle_account_event(&mut self, event: AccountEvent) {
        match event {
            AccountEvent::Remove(user_id) => remove_account_tasks(user_id),
            AccountEvent::Changed => self.scheduler.on_state_change(Handler::update_account, ()),
        }
    }

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
            ServiceEvent::AttachGroup(task_id, group, tx) => {
                let _ = tx.send(self.attach_group(task_id, group));
            }
        }
    }

    fn handle_state_event(&mut self, event: StateEvent) {
        debug!("TaskManager handles state event {:?}", event);

        match event {
            StateEvent::Network => {
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

    fn handle_task_event(&mut self, event: TaskEvent) {
        debug!("TaskManager handles task event {:?}", event);

        match event {
            TaskEvent::Completed(task_id, uid, mode) => {
                if let Some((front, back)) = self.task_count.get_mut(&uid) {
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
                self.scheduler.task_completed(uid, task_id);
            }
            TaskEvent::Subscribe(task_id, token_id, tx) => {
                let _ = tx.send(self.check_subscriber(task_id, token_id));
            }
            TaskEvent::Running(task_id, uid, mode) => {
                self.scheduler
                    .task_cancel(uid, task_id, mode, &mut self.task_count);
            }
            TaskEvent::Failed(task_id, uid, reason, mode) => {
                if let Some((front, back)) = self.task_count.get_mut(&uid) {
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
                self.scheduler.task_failed(uid, task_id, reason);
            }
            TaskEvent::Offline(task_id, uid, mode) => {
                self.scheduler
                    .task_cancel(uid, task_id, mode, &mut self.task_count);
            }
        };
    }

    fn handle_schedule_event(&mut self, message: ScheduleEvent) -> bool {
        debug!("TaskManager handle scheduled_message {:?}", message);

        match message {
            ScheduleEvent::ClearTimeoutTasks => self.clear_timeout_tasks(),
            ScheduleEvent::RestoreAllTasks => self.restore_all_tasks(),
            ScheduleEvent::Unload => return self.unload_sa(),
        }
        false
    }

    fn check_subscriber(&self, task_id: u32, token_id: u64) -> ErrorCode {
        match RequestDb::get_instance().query_task_token_id(task_id) {
            Ok(id) if id == token_id => ErrorCode::ErrOk,
            Ok(_) => ErrorCode::Permission,
            Err(_) => ErrorCode::TaskNotFound,
        }
    }

    fn clear_timeout_tasks(&mut self) {
        self.scheduler.clear_timeout_tasks();
    }

    fn restore_all_tasks(&mut self) {
        self.scheduler.restore_all_tasks();
    }

    fn unload_sa(&mut self) -> bool {
        const REQUEST_SERVICE_ID: i32 = 3706;

        if !self.rx.is_empty() {
            return false;
        }

        let running_tasks = self.scheduler.running_tasks();
        if running_tasks != 0 {
            info!("running {} tasks when unload SA", running_tasks,);
            return false;
        }

        // check rx again for there may be new message arrive.
        if !self.rx.is_empty() {
            return false;
        }

        info!("unload SA");

        // failed logic?
        #[cfg(feature = "oh")]
        let _ = SystemAbilityManager::unload_system_ability(REQUEST_SERVICE_ID);

        true
    }
}

#[allow(unreachable_pub)]
#[derive(Clone)]
pub struct TaskManagerTx {
    pub(crate) tx: UnboundedSender<TaskManagerEvent>,
}

impl TaskManagerTx {
    pub(crate) fn new(tx: UnboundedSender<TaskManagerEvent>) -> Self {
        Self { tx }
    }

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

    pub(crate) fn notify_foreground_app_change(&self, uid: u64) {
        let _ = self.send_event(TaskManagerEvent::State(StateEvent::ForegroundApp(uid)));
    }

    pub(crate) fn notify_app_background(&self, uid: u64) {
        let _ = self.send_event(TaskManagerEvent::State(StateEvent::Background(uid)));
    }

    pub(crate) fn trigger_background_timeout(&self, uid: u64) {
        let _ = self.send_event(TaskManagerEvent::State(StateEvent::BackgroundTimeout(uid)));
    }

    pub(crate) fn notify_special_process_terminate(&self, uid: u64) {
        let _ = self.send_event(TaskManagerEvent::State(StateEvent::SpecialTerminate(uid)));
    }

    pub(crate) fn show(&self, uid: u64, task_id: u32) -> Option<TaskInfo> {
        let (tx, rx) = oneshot::channel();
        let event = QueryEvent::Show(task_id, uid, tx);
        let _ = self.send_event(TaskManagerEvent::Query(event));
        ylong_runtime::block_on(rx).unwrap()
    }

    pub(crate) fn query(&self, task_id: u32, action: Action) -> Option<TaskInfo> {
        let (tx, rx) = oneshot::channel();
        let event = QueryEvent::Query(task_id, action, tx);
        let _ = self.send_event(TaskManagerEvent::Query(event));
        ylong_runtime::block_on(rx).unwrap()
    }

    pub(crate) fn touch(&self, uid: u64, task_id: u32, token: String) -> Option<TaskInfo> {
        let (tx, rx) = oneshot::channel();
        let event = QueryEvent::Touch(task_id, uid, token, tx);
        let _ = self.send_event(TaskManagerEvent::Query(event));
        ylong_runtime::block_on(rx).unwrap()
    }
}

pub(crate) struct TaskManagerRx {
    rx: UnboundedReceiver<TaskManagerEvent>,
}

impl TaskManagerRx {
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
