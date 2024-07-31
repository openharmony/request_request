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
pub(crate) mod state;
use std::sync::Arc;
mod sql;
use qos::Qos;
use queue::RunningQueue;
use state::sql::SqlList;

use super::network::Network;
use crate::error::ErrorCode;
use crate::manage::database::RequestDb;
use crate::manage::notifier::Notifier;
use crate::manage::scheduler::qos::{QosChanges, RssCapacity};
use crate::manage::task_manager::TaskManagerTx;
use crate::service::client::ClientManagerEntry;
use crate::service::run_count::RunCountManagerEntry;
use crate::task::config::Action;
use crate::task::ffi::CUpdateStateInfo;
use crate::task::info::State;
use crate::task::reason::Reason;
use crate::task::request_task::RequestTask;
#[cfg(feature = "oh")]
use crate::utils::publish_state_change_event;

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
    running_queue: RunningQueue,
    client_manager: ClientManagerEntry,
    state_handler: state::Handler,
}

impl Scheduler {
    pub(crate) fn init(
        tx: TaskManagerTx,
        runcount_manager: RunCountManagerEntry,
        client_manager: ClientManagerEntry,
        network: Network,
    ) -> Scheduler {
        let mut state_handler = state::Handler::new(network.clone(), tx.clone());
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
                network,
            ),
            client_manager,
            state_handler,
        }
    }

    pub(crate) fn tasks(&self) -> impl Iterator<Item = &Arc<RequestTask>> {
        self.running_queue.tasks()
    }

    pub(crate) fn running_tasks(&self) -> usize {
        self.running_queue.running_tasks()
    }

    pub(crate) fn dump_tasks(&self) {
        self.running_queue.dump_tasks();
    }

    pub(crate) fn restore_all_tasks(&mut self) {
        info!("Reschedule tasks restore all tasks");
        // Reschedule tasks based on the current `QOS` status.
        let changes = self.qos.reschedule(Action::Any, &self.state_handler);
        self.reschedule(changes);
    }

    pub(crate) fn start_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        database.change_status(task_id, State::Running)?;

        if !self.check_config_satisfy(task_id)? {
            return Ok(());
        };

        let qos_info = database
            .get_task_qos_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;
        let changes = self.qos.start_task(uid, qos_info, &self.state_handler);
        self.reschedule(changes);
        Ok(())
    }

    pub(crate) fn resume_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        database.change_status(task_id, State::Retrying)?;

        if !self.check_config_satisfy(task_id)? {
            return Ok(());
        };
        let info = database
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;

        let changes = self
            .qos
            .start_task(uid, info.qos_info(), &self.state_handler);

        self.reschedule(changes);

        Ok(())
    }

    pub(crate) fn pause_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        database.change_status(task_id, State::Paused)?;

        if let Some(qos_changes) = self.qos.remove_task(uid, task_id, &self.state_handler) {
            self.reschedule(qos_changes);
        }
        Ok(())
    }

    pub(crate) fn remove_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        database.change_status(task_id, State::Removed)?;
        let info = database
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;

        Notifier::remove(&self.client_manager, info.build_notify_data());

        if let Some(qos_changes) = self.qos.remove_task(uid, task_id, &self.state_handler) {
            self.reschedule(qos_changes);
        }
        Ok(())
    }

    pub(crate) fn stop_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        database.change_status(task_id, State::Stopped)?;

        if let Some(qos_changes) = self.qos.remove_task(uid, task_id, &self.state_handler) {
            self.reschedule(qos_changes);
        }
        Ok(())
    }

    pub(crate) fn task_completed(&mut self, uid: u64, task_id: u32) {
        info!("Scheduler notify task {} completed", task_id);
        self.running_queue.task_finish(uid, task_id);

        let database = RequestDb::get_instance();
        let _ = database.execute(&sql::task_completed(task_id));

        if let Some(info) = database.get_task_info(task_id) {
            Notifier::complete(&self.client_manager, info.build_notify_data());
            #[cfg(feature = "oh")]
            let _ = publish_state_change_event(
                info.bundle.as_str(),
                info.common_data.task_id,
                State::Completed.repr as i32,
            );
        }

        if let Some(changes) = self.qos.remove_task(uid, task_id, &self.state_handler) {
            self.reschedule(changes);
        }

        if let Some(changes) = self.qos.remove_task(uid, task_id, &self.state_handler) {
            self.reschedule(changes);
        }
    }

    pub(crate) fn task_cancel(&mut self, uid: u64, task_id: u32) {
        info!("Scheduler notify task {} canceled", task_id);
        self.running_queue.task_finish(uid, task_id);

        let database = RequestDb::get_instance();
        let Some(info) = database.get_task_info(task_id) else {
            error!("task {} not found in database", task_id);
            return;
        };
        match State::from(info.progress.common_data.state) {
            State::Paused => {
                Notifier::pause(&self.client_manager, info.build_notify_data());
            }
            State::Running | State::Retrying => {
                if !self.running_queue.try_restart(uid, task_id) {
                    info!("task {} waiting for task limits", task_id);
                    let state_info =
                        CUpdateStateInfo::new(State::Waiting, Reason::RunningTaskMeetLimits);
                    if !RequestDb::get_instance().update_task_state(task_id, &state_info) {
                        error!("{} update_task_state error", task_id);
                    }
                }
            }
            State::Failed => {
                info!("task {} cancel with state Failed", task_id);
                Notifier::fail(&self.client_manager, info.build_notify_data());
            }
            state => {
                info!("task {} cancel with state {:?}", task_id, state);
                self.running_queue.try_restart(uid, task_id);
            }
        }
    }

    pub(crate) fn task_failed(&mut self, uid: u64, task_id: u32, reason: Reason) {
        info!("Scheduler notify task {} failed", task_id);
        self.running_queue.task_finish(uid, task_id);

        let database = RequestDb::get_instance();
        let _ = database.execute(&sql::task_failed(task_id, reason));

        if let Some(info) = database.get_task_info(task_id) {
            Notifier::fail(&self.client_manager, info.build_notify_data());
            #[cfg(feature = "oh")]
            let _ = publish_state_change_event(
                info.bundle.as_str(),
                info.common_data.task_id,
                State::Failed.repr as i32,
            );
        }

        if let Some(changes) = self.qos.remove_task(uid, task_id, &self.state_handler) {
            self.reschedule(changes);
        }
    }

    pub(crate) fn on_state_change<T, F>(&mut self, f: F, t: T)
    where
        F: FnOnce(&mut state::Handler, T) -> Option<SqlList>,
    {
        let Some(sql_list) = f(&mut self.state_handler, t) else {
            return;
        };
        let db = RequestDb::get_instance();
        for sql in sql_list {
            if let Err(e) = db.execute(&sql) {
                error!("TaskManager update network failed {:?}", e);
            };
        }
        self.reload_all_tasks();
    }

    pub(crate) fn reload_all_tasks(&mut self) {
        let changes = self.qos.reload_all_tasks(&self.state_handler);
        self.reschedule(changes);
    }

    pub(crate) fn on_rss_change(&mut self, level: i32) {
        let new_rss = RssCapacity::new(level);
        let changes = self.qos.change_rss(new_rss, &self.state_handler);
        self.reschedule(changes);
    }

    fn reschedule(&mut self, changes: QosChanges) {
        info!("{:?}", changes.download);
        let mut qos_remove_queue = vec![];
        self.running_queue
            .reschedule(changes, &mut qos_remove_queue);
        for (uid, task_id) in qos_remove_queue.iter() {
            self.qos.apps.remove_task(*uid, *task_id);
        }
        if !qos_remove_queue.is_empty() {
            self.reload_all_tasks();
        }
    }

    pub(crate) fn check_config_satisfy(&self, task_id: u32) -> Result<bool, ErrorCode> {
        let database = RequestDb::get_instance();
        let config = database
            .get_task_config(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;

        if !config.satisfy_network(self.state_handler.network()) {
            info!("task {} started, waiting for network", task_id);
            let state_info = CUpdateStateInfo::new(State::Waiting, Reason::UnsupportedNetworkType);
            database.update_task_state(task_id, &state_info);
            return Ok(false);
        }

        if !config.satisfy_foreground(self.state_handler.top_uid()) {
            info!("task {} started, waiting for app state", task_id);
            let state_info =
                CUpdateStateInfo::new(State::Waiting, Reason::AppBackgroundOrTerminate);
            database.update_task_state(task_id, &state_info);
            return Ok(false);
        }
        Ok(true)
    }

    pub(crate) fn clear_timeout_tasks(&mut self) {
        self.running_queue.clear_timeout_tasks();
    }
}

impl RequestDb {
    fn change_status(&self, task_id: u32, state: State) -> Result<(), ErrorCode> {
        let info = RequestDb::get_instance()
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;
        if info.progress.common_data.state == state.repr {
            if state == State::Removed {
                return Err(ErrorCode::TaskNotFound);
            } else {
                return Err(ErrorCode::TaskStateErr);
            }
        }
        let sql = match state {
            State::Paused => sql::pause_task(task_id),
            State::Running => sql::start_task(task_id),
            State::Stopped => sql::stop_task(task_id),
            State::Removed => sql::remove_task(task_id),
            State::Retrying => sql::resume_task(task_id),
            _ => return Err(ErrorCode::Other),
        };

        RequestDb::get_instance()
            .execute(&sql)
            .map_err(|_| ErrorCode::SystemApi)?;

        let info = RequestDb::get_instance()
            .get_task_info(task_id)
            .ok_or(ErrorCode::SystemApi)?;
        if info.progress.common_data.state != state.repr {
            Err(ErrorCode::TaskStateErr)
        } else {
            Ok(())
        }
    }
}
