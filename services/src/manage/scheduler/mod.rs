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

mod qos;
mod queue;

use std::sync::Arc;

use qos::Qos;
use queue::{NotifyTask, RunningQueue};

use super::app_state::AppStateManagerTx;
use crate::error::ErrorCode;
use crate::init::SYSTEM_CONFIG_MANAGER;
use crate::manage::database::Database;
use crate::manage::notifier::Notifier;
use crate::manage::scheduler::qos::{QosChanges, RssCapacity};
use crate::manage::task_manager::TaskManagerTx;
use crate::service::client::ClientManagerEntry;
use crate::service::runcount::RunCountManagerEntry;
use crate::task::config::Action;
use crate::task::ffi::NetworkInfo;
use crate::task::info::{ApplicationState, State};
use crate::task::reason::Reason;
use crate::task::request_task::RequestTask;

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
    qos: Qos,
    upload_queue: RunningQueue,
    download_queue: RunningQueue,
    app_state_manager: AppStateManagerTx,
    client_manager: ClientManagerEntry,
}

impl Scheduler {
    pub(crate) fn init(
        tx: TaskManagerTx,
        runcount_manager: RunCountManagerEntry,
        app_state_manager: AppStateManagerTx,
        client_manager: ClientManagerEntry,
    ) -> Scheduler {
        Self {
            qos: Qos::new(),
            upload_queue: RunningQueue::new(
                tx.clone(),
                runcount_manager.clone(),
                app_state_manager.clone(),
                client_manager.clone(),
            ),
            download_queue: RunningQueue::new(
                tx,
                runcount_manager,
                app_state_manager.clone(),
                client_manager.clone(),
            ),
            app_state_manager,
            client_manager,
        }
    }

    pub(crate) fn tasks(&self) -> impl Iterator<Item = &Arc<RequestTask>> {
        self.upload_queue.tasks().chain(self.download_queue.tasks())
    }
    
    pub(crate) fn get_task(&self, uid: u64, task_id: u32) -> Option<&Arc<RequestTask>> {
        self.upload_queue
            .get_task(uid, task_id)
            .or(self.download_queue.get_task(uid, task_id))
    }

    pub(crate) fn running_tasks(&self) -> usize {
        self.upload_queue.running_tasks() + self.download_queue.running_tasks()
    }

    pub(crate) fn dump_tasks(&self) {
        self.upload_queue.dump_tasks();
        self.download_queue.dump_tasks();
    }

    pub(crate) async fn start_task(&mut self, uid: u64, task_id: u32) -> ErrorCode {
        let database = Database::new();
        if let Some(task) = database.get_task_qos_info(uid, task_id) {
            let action = Action::from(task.action);
            let task_state = State::from(task.state);
            let app_state = self.app_state_manager.get_app_raw_state(uid).await;
            if task_state == State::Initialized
                || (task_state == State::Failed && action == Action::Download)
            {
                let changes = self.qos.start_task(uid, app_state, task);
                self.reschedule(changes).await;
                return ErrorCode::ErrOk;
            }
            return ErrorCode::TaskStateErr;
        }
        ErrorCode::TaskNotFound
    }

    pub(crate) async fn resume_task(
        &mut self,
        uid: u64,
        task_id: u32,
        app_state_manager: AppStateManagerTx,
    ) -> ErrorCode {
        let database = Database::new();
        if let Some(task) = database.get_task_qos_info(uid, task_id) {
            let task_state = State::from(task.state);
            let app_state = app_state_manager.get_app_raw_state(uid).await;
            if task_state == State::Paused {
                let changes = self.qos.start_task(uid, app_state, task);
                self.reschedule(changes).await;
                return ErrorCode::ErrOk;
            }
            return ErrorCode::TaskStateErr;
        }
        ErrorCode::TaskNotFound
    }

    pub(crate) async fn pause_task(&mut self, uid: u64, task_id: u32) -> ErrorCode {
        self.modify_task_state_by_user(uid, task_id, State::Paused)
            .await
    }

    pub(crate) async fn remove_task(&mut self, uid: u64, task_id: u32) -> ErrorCode {
        self.modify_task_state_by_user(uid, task_id, State::Removed)
            .await
    }

    pub(crate) async fn stop_task(&mut self, uid: u64, task_id: u32) -> ErrorCode {
        self.modify_task_state_by_user(uid, task_id, State::Stopped)
            .await
    }

    pub(crate) fn clear_timeout_tasks(&mut self) {
        self.download_queue.clear_timeout_tasks();
        self.upload_queue.clear_timeout_tasks();
        // TODO: 考虑优化，在进行删除操作之后若队列没有变化，则不 reload。
        // TODO:任务结束时如何进行qos及queue操作的。
    }

    pub(crate) async fn restore_all_tasks(&mut self) {
        // Reschedule tasks based on the current `QOS` status.
        let changes = self.qos.reschedule(Action::Any);
        self.reschedule(changes).await;
    }

    pub(crate) async fn finish_task(&mut self, uid: u64, task_id: u32) {
        let changes = self.qos.finish_task(uid, task_id);
        self.reschedule(changes).await;
    }

    pub(crate) async fn on_network_change(&mut self, network: NetworkInfo) {
        let changes = self.qos.change_network(network);
        self.reschedule(changes).await;
    }

    pub(crate) async fn on_app_state_change(&mut self, uid: u64, state: ApplicationState) {
        let changes = self.qos.change_app_state(uid, state);
        self.reschedule(changes).await;
    }

    pub(crate) async fn on_rss_change(&mut self, level: i32) {
        let new_rss = RssCapacity::new(level);
        let changes = self.qos.change_rss(new_rss);
        self.reschedule(changes).await;
    }

    async fn reschedule(&mut self, changes: QosChanges) {
        if let Some(vec) = changes.download {
            self.download_queue.reschedule(vec).await;
        }
        if let Some(vec) = changes.upload {
            self.upload_queue.reschedule(vec).await;
        }
    }
}

impl Scheduler {
    // 如何调整任务状态并触发回调？
    // 任务所处的位置有三个：
    // 1）存在 running 队列中且存在 qos 队列中。 （Running \ Retrying）
    // 2）不存在 running 队列中但存在 qos 队列中。（Waiting &&
    // RunningTaskMeetsLimit） 3）既不存在 running 队列且也不存在 qos
    // 队列中。（Waiting && NetworkOffline \ Waiting && AppStateNotSatisfied）
    //
    // 针对场景 1），可以利用 running_task 的析构函数触发回调逻辑。
    // 针对场景 2），可以直接从数据库里取出对应的任务信息，触发回调。
    // 针对场景 3），可以直接从数据库里取出对应的任务信息，触发回调。
    async fn modify_task_state_by_user(
        &mut self,
        uid: u64,
        task_id: u32,
        state: State,
    ) -> ErrorCode {
        // If the task currently exists in the running queue, simply manipulate
        // the task status directly.
        if self
            .download_queue
            .modify_task_state_by_user(uid, task_id, state)
            == ErrorCode::ErrOk
        {
            return ErrorCode::ErrOk;
        }
        if self
            .upload_queue
            .modify_task_state_by_user(uid, task_id, state)
            == ErrorCode::ErrOk
        {
            return ErrorCode::ErrOk;
        }
        // If the task is not in the running queue but exists in qos, we need to
        // update task status in the database.
        if self.qos.contains_task(uid, task_id) {
            // If task is not running, we need not to reschedule the running queue,
            // also we need to delete it from qos.
            let _ = self.qos.finish_task(uid, task_id);
        }
        let database = Database::new();
        if state != State::Removed {
            let system_config = unsafe { SYSTEM_CONFIG_MANAGER.assume_init_ref().system_config() };
            if let Some(task) = database
                .get_task(
                    task_id,
                    system_config,
                    &self.app_state_manager,
                    &self.client_manager,
                )
                .await
            {
                return if task.set_status(state, Reason::UserOperation) {
                    // Here we use the `drop` method of `NotifyTask` to notify apps.
                    let _ = NotifyTask::new(Arc::new(task));
                    ErrorCode::ErrOk
                } else {
                    ErrorCode::TaskStateErr
                };
            }
        } else {
            // removed task can not be constructed, set state and send notify
            if let Some(mut info) = database.get_task_info(task_id) {
                if State::from(info.progress.common_data.state) == State::Removed {
                    error!(
                        "TaskManager remove a task, uid:{}, task_id:{} removed already",
                        uid, task_id
                    );
                } else {
                    debug!(
                        "TaskManager remove a task, uid:{}, task_id:{} success",
                        uid, task_id
                    );
                    database.change_task_state(task_id, uid, State::Removed);
                    info.progress.common_data.state = State::Removed as u8;
                    let notify_data = info.build_notify_data();
                    Notifier::remove(&self.client_manager, notify_data);
                    return ErrorCode::ErrOk;
                }
            }
        }
        ErrorCode::TaskNotFound
    }
}