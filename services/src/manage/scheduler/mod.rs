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

use super::events::TaskManagerEvent;
use super::network::Network;
use crate::config::{Mode, Version};
use crate::error::ErrorCode;
use crate::info::TaskInfo;
use crate::manage::database::RequestDb;
use crate::manage::notifier::Notifier;
use crate::manage::task_manager::TaskManagerTx;
use crate::service::client::ClientManagerEntry;
use crate::service::run_count::RunCountManagerEntry;
use crate::task::config::Action;
use crate::task::info::State;
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
    qos: Qos,
    running_queue: RunningQueue,
    client_manager: ClientManagerEntry,
    state_handler: state::Handler,
    pub(crate) resort_scheduled: bool,
    task_manager: TaskManagerTx,
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
            resort_scheduled: false,
            task_manager: tx,
        }
    }

    pub(crate) fn get_task(&self, uid: u64, task_id: u32) -> Option<&Arc<RequestTask>> {
        self.running_queue.get_task(uid, task_id)
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
        self.schedule_if_not_scheduled();
    }

    pub(crate) fn start_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        self.start_inner(uid, task_id, false)
    }

    pub(crate) fn resume_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        self.start_inner(uid, task_id, true)
    }

    fn start_inner(&mut self, uid: u64, task_id: u32, is_resume: bool) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        let info = RequestDb::get_instance()
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;

        if (is_resume && info.progress.common_data.state != State::Paused.repr)
            || (!is_resume && info.progress.common_data.state == State::Paused.repr)
        {
            return Err(ErrorCode::TaskStateErr);
        }
        database.change_status(task_id, State::Waiting)?;

        let info = RequestDb::get_instance()
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;
        if is_resume {
            Notifier::resume(&self.client_manager, info.build_notify_data());
        }

        if info.progress.is_finish() {
            database.update_task_state(task_id, State::Completed, Reason::Default);
            if let Some(info) = database.get_task_info(task_id) {
                Notifier::complete(&self.client_manager, info.build_notify_data());
            }
        }

        if !self.check_config_satisfy(task_id)? {
            return Ok(());
        };
        let qos_info = database
            .get_task_qos_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;
        self.qos.start_task(uid, qos_info);
        self.schedule_if_not_scheduled();
        Ok(())
    }

    pub(crate) fn pause_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        database.change_status(task_id, State::Paused)?;

        if let Some(info) = database.get_task_info(task_id) {
            Notifier::pause(&self.client_manager, info.build_notify_data());
        }
        if self.qos.remove_task(uid, task_id) {
            self.schedule_if_not_scheduled();
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

        if self.qos.remove_task(uid, task_id) {
            self.schedule_if_not_scheduled();
        }
        Ok(())
    }

    pub(crate) fn stop_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        database.change_status(task_id, State::Stopped)?;

        if self.qos.remove_task(uid, task_id) {
            self.schedule_if_not_scheduled();
        }
        Ok(())
    }

    pub(crate) fn task_completed(&mut self, uid: u64, task_id: u32) {
        info!("Scheduler notify task {} completed", task_id);
        self.running_queue.task_finish(uid, task_id);

        let database = RequestDb::get_instance();
        if self.qos.remove_task(uid, task_id) {
            self.schedule_if_not_scheduled();
        }

        if let Some(info) = database.get_task_qos_info(task_id) {
            if info.state != State::Running.repr && info.state != State::Waiting.repr {
                return;
            }
        }

        database.update_task_state(task_id, State::Completed, Reason::Default);
        if let Some(info) = database.get_task_info(task_id) {
            Notifier::complete(&self.client_manager, info.build_notify_data());
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
            State::Running | State::Retrying => {
                if !self.running_queue.try_restart(uid, task_id) {
                    info!("task {} waiting for task limits", task_id);
                    RequestDb::get_instance().update_task_state(
                        task_id,
                        State::Waiting,
                        Reason::RunningTaskMeetLimits,
                    );
                }
            }
            State::Failed => {
                info!("task {} cancel with state Failed", task_id);
                Notifier::fail(&self.client_manager, info.build_notify_data());
                #[cfg(feature = "oh")]
                {
                    let reason = Reason::from(info.common_data.reason);
                    Self::sys_event(info, reason);
                }
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

        if self.qos.remove_task(uid, task_id) {
            self.schedule_if_not_scheduled();
        }

        if let Some(info) = database.get_task_qos_info(task_id) {
            if info.state != State::Running.repr && info.state != State::Waiting.repr {
                return;
            }
        }

        database.update_task_state(task_id, State::Failed, reason);

        if let Some(info) = database.get_task_info(task_id) {
            Notifier::fail(&self.client_manager, info.build_notify_data());
            #[cfg(feature = "oh")]
            Self::sys_event(info, reason);
        }
    }
    #[cfg(feature = "oh")]
    pub(crate) fn sys_event(info: TaskInfo, reason: Reason) {
        use hisysevent::{build_number_param, build_str_param};

        use crate::sys_event::SysEvent;

        let index = info.progress.common_data.index;
        let size = info.file_specs.len();
        let action = match info.action() {
            Action::Download => "DOWNLOAD",
            Action::Upload => "UPLOAD",
            _ => "UNKNOWN",
        };

        SysEvent::task_fault()
            .param(build_str_param!(crate::sys_event::TASKS_TYPE, action))
            .param(build_number_param!(
                crate::sys_event::TOTAL_FILE_NUM,
                size as i32
            ))
            .param(build_number_param!(
                crate::sys_event::FAIL_FILE_NUM,
                (size - index) as i32
            ))
            .param(build_number_param!(
                crate::sys_event::SUCCESS_FILE_NUM,
                index as i32
            ))
            .param(build_number_param!(
                crate::sys_event::ERROR_INFO,
                reason.repr as i32
            ))
            .write();
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
        self.qos.reload_all_tasks();
        self.schedule_if_not_scheduled();
    }

    pub(crate) fn on_rss_change(&mut self, level: i32) {
        if let Some(new_rss) = self.state_handler.update_rss_level(level) {
            self.qos.change_rss(new_rss);
            self.schedule_if_not_scheduled();
        }
    }

    fn schedule_if_not_scheduled(&mut self) {
        if self.resort_scheduled {
            return;
        }
        self.resort_scheduled = true;
        let task_manager = self.task_manager.clone();
        task_manager.send_event(TaskManagerEvent::Reschedule);
    }

    pub(crate) fn reschedule(&mut self) {
        self.resort_scheduled = false;
        let changes = self.qos.reschedule(&self.state_handler);
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

        if let Err(reason) = config.satisfy_network(self.state_handler.network()) {
            info!(
                "task {} not satisfy network {:?}",
                task_id,
                self.state_handler.network()
            );
            let state = match config.version {
                Version::API9 => match config.common_data.action {
                    Action::Download => State::Waiting,
                    Action::Upload => State::Failed,
                    _ => unreachable!(),
                },
                Version::API10 => {
                    if config.common_data.mode == Mode::BackGround && config.common_data.retry {
                        State::Waiting
                    } else {
                        State::Failed
                    }
                }
            };
            database.update_task_state(task_id, state, reason);
            if state == State::Failed {
                if let Some(info) = database.get_task_info(task_id) {
                    Notifier::fail(&self.client_manager, info.build_notify_data());
                }
            }
            return Ok(false);
        }

        if !config.satisfy_foreground(self.state_handler.top_uid()) {
            info!("task {} started, waiting for app state", task_id);
            database.update_task_state(task_id, State::Waiting, Reason::AppBackgroundOrTerminate);
            return Ok(false);
        }
        Ok(true)
    }

    pub(crate) fn clear_timeout_tasks(&mut self) {
        let current_time = get_current_timestamp();
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
            if database
                .change_status(task.task_id(), State::Stopped)
                .is_ok()
            {
                self.qos.apps.remove_task(task.uid(), task.task_id());
            }
        }
        self.schedule_if_not_scheduled();
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
            State::Waiting => sql::start_task(task_id),
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
