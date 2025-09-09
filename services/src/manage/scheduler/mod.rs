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
        active_counter: ActiveCounter,
    ) -> Scheduler {
        let mut state_handler = state::Handler::new(tx.clone());
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

    pub(crate) fn get_task(&self, uid: u64, task_id: u32) -> Option<&Arc<RequestTask>> {
        self.running_queue.get_task(uid, task_id)
    }

    pub(crate) fn tasks(&self) -> impl Iterator<Item = &Arc<RequestTask>> {
        self.running_queue.tasks()
    }

    pub(crate) fn running_tasks(&self) -> usize {
        self.running_queue.running_tasks()
    }

    pub(crate) fn restore_all_tasks(&mut self) {
        info!("reschedule restore all tasks");
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
        // Change `Waiting` so that it can be scheduled.
        database.change_status(task_id, State::Waiting)?;

        let info = RequestDb::get_instance()
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;
        if is_resume {
            Notifier::resume(&self.client_manager, info.build_notify_data());
        } else {
            // If the task is started, reset the task time.
            database.update_task_time(task_id, 0);
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
        self.qos.remove_task(uid, task_id);

        if self.running_queue.cancel_task(task_id, uid) {
            self.running_queue.upload_resume.insert(task_id);
            self.schedule_if_not_scheduled();
        }
        let info = database
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;

        Notifier::pause(&self.client_manager, info.build_notify_data());
        Ok(())
    }

    pub(crate) fn remove_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        database.change_status(task_id, State::Removed)?;
        self.qos.remove_task(uid, task_id);

        if self.running_queue.cancel_task(task_id, uid) {
            self.schedule_if_not_scheduled();
        }
        database.remove_user_file_task(task_id);
        let info = database
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;

        Notifier::remove(&self.client_manager, info.build_notify_data());
        Ok(())
    }

    pub(crate) fn stop_task(&mut self, uid: u64, task_id: u32) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        database.change_status(task_id, State::Stopped)?;
        self.qos.remove_task(uid, task_id);

        if self.running_queue.cancel_task(task_id, uid) {
            self.schedule_if_not_scheduled();
        }
        Ok(())
    }

    pub(crate) fn set_max_speed(
        &mut self,
        uid: u64,
        task_id: u32,
        max_speed: i64,
    ) -> Result<(), ErrorCode> {
        if let Some(task) = self.running_queue.get_task(uid, task_id) {
            task.max_speed.store(max_speed, Ordering::SeqCst);
        }
        Ok(())
    }

    pub(crate) fn task_set_mode(
        &mut self,
        uid: u64,
        task_id: u32,
        mode: Mode,
    ) -> Result<(), ErrorCode> {
        let database = RequestDb::get_instance();
        database.set_mode(task_id, mode)?;

        if self.qos.task_set_mode(uid, task_id, mode) {
            self.schedule_if_not_scheduled();
        }
        if let Some(task) = self.running_queue.get_task_clone(uid, task_id) {
            task.mode.store(mode.repr, Ordering::Release);
        }
        if mode == Mode::FrontEnd {
            NotificationDispatcher::get_instance().unregister_task(uid, task_id, false);
        } else if mode == Mode::BackGround {
            NotificationDispatcher::get_instance().enable_task_progress_notification(task_id);
        }
        Ok(())
    }

    pub(crate) fn task_completed(&mut self, uid: u64, task_id: u32) {
        info!("task {} completed", task_id);
        self.running_queue.task_finish(uid, task_id);

        let database = RequestDb::get_instance();
        if self.qos.remove_task(uid, task_id) {
            self.schedule_if_not_scheduled();
        }

        if let Some(info) = database.get_task_qos_info(task_id) {
            if info.state == State::Failed.repr {
                if let Some(task_info) = database.get_task_info(task_id) {
                    Scheduler::notify_fail(task_info, &self.client_manager, Reason::Default);
                    return;
                }
            }

            if info.state != State::Running.repr && info.state != State::Waiting.repr {
                return;
            }
        }

        database.update_task_state(task_id, State::Completed, Reason::Default);
        database.remove_user_file_task(task_id);
        if let Some(info) = database.get_task_info(task_id) {
            Notifier::complete(&self.client_manager, info.build_notify_data());
            NotificationDispatcher::get_instance().publish_success_notification(&info);
        }
    }

    pub(crate) fn task_cancel(
        &mut self,
        uid: u64,
        task_id: u32,
        mode: Mode,
        task_count: &mut HashMap<u64, (usize, usize)>,
    ) {
        info!("task {} canceled", task_id);
        self.running_queue.task_finish(uid, task_id);
        if self.running_queue.try_restart(uid, task_id) {
            return;
        }

        let database = RequestDb::get_instance();
        let Some(info) = database.get_task_info(task_id) else {
            error!("task {} not found in database", task_id);
            NotificationDispatcher::get_instance().unregister_task(uid, task_id, true);
            return;
        };
        match State::from(info.progress.common_data.state) {
            State::Running | State::Retrying => {
                info!("task {} waiting for task limits", task_id);
                RequestDb::get_instance().update_task_state(
                    task_id,
                    State::Waiting,
                    Reason::RunningTaskMeetLimits,
                );
                Notifier::waiting(&self.client_manager, task_id, WaitingCause::TaskQueue);
            }
            State::Failed => {
                info!("task {} cancel with state Failed", task_id);
                Scheduler::reduce_task_count(uid, mode, task_count);
                let reason = info.common_data.reason;
                Scheduler::notify_fail(info, &self.client_manager, Reason::from(reason));
            }
            State::Stopped | State::Removed => {
                info!("task {} cancel with state Stopped or Removed", task_id);
                NotificationDispatcher::get_instance().unregister_task(uid, task_id, true);
                self.running_queue.try_restart(uid, task_id);
            }
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

    pub(crate) fn task_failed(&mut self, uid: u64, task_id: u32, reason: Reason) {
        info!("task {} failed", task_id);
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
            let reason = info.common_data.reason;
            Scheduler::notify_fail(info, &self.client_manager, Reason::from(reason));
        }
    }

    fn notify_fail(info: TaskInfo, client_manager: &ClientManagerEntry, reason: Reason) {
        Notifier::fail(client_manager, info.build_notify_data());
        Notifier::faults(info.common_data.task_id, client_manager, reason);
        NotificationDispatcher::get_instance().publish_failed_notification(&info);
        #[cfg(feature = "oh")]
        Self::sys_event(info);
    }

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

        sys_task_fault(
            action,
            size as i32,
            (size - index) as i32,
            index as i32,
            reason.repr as i32,
        );
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
                "task {} started, waiting for network {:?}",
                task_id,
                self.state_handler.network()
            );

            database.update_task_state(task_id, State::Waiting, reason);
            Notifier::waiting(&self.client_manager, task_id, WaitingCause::Network);
            return Ok(false);
        }

        if !config.satisfy_foreground(self.state_handler.foreground_abilities()) {
            info!(
                "task {} started, waiting for app {}",
                task_id, config.common_data.uid
            );
            database.update_task_state(task_id, State::Waiting, Reason::AppBackgroundOrTerminate);
            Notifier::waiting(&self.client_manager, task_id, WaitingCause::AppState);
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

    pub(crate) fn retry_all_tasks(&mut self) {
        self.running_queue.retry_all_tasks();
    }

    pub(crate) fn shutdown(&mut self) {
        self.running_queue.shutdown();
    }
}

impl RequestDb {
    fn change_status(&self, task_id: u32, new_state: State) -> Result<(), ErrorCode> {
        let info = RequestDb::get_instance()
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;

        let old_state = info.progress.common_data.state;
        if old_state == new_state.repr {
            if new_state == State::Removed {
                return Err(ErrorCode::TaskNotFound);
            } else {
                return Err(ErrorCode::TaskStateErr);
            }
        }
        let sql = match new_state {
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
        if info.progress.common_data.state != new_state.repr {
            return Err(ErrorCode::TaskStateErr);
        }

        if (old_state == State::Initialized.repr
            || old_state == State::Waiting.repr
            || old_state == State::Paused.repr)
            && (new_state == State::Stopped || new_state == State::Removed)
        {
            NotificationDispatcher::get_instance().unregister_task(info.uid(), task_id, true);
        }
        Ok(())
    }

    fn set_mode(&self, task_id: u32, mode: Mode) -> Result<(), ErrorCode> {
        let info = RequestDb::get_instance()
            .get_task_info(task_id)
            .ok_or(ErrorCode::TaskNotFound)?;
        let old_mode = info.common_data.mode;
        if old_mode == mode.repr {
            return Ok(());
        }
        let sql = sql::task_set_mode(task_id, mode);
        RequestDb::get_instance()
            .execute(&sql)
            .map_err(|_| ErrorCode::SystemApi)?;
        let info = RequestDb::get_instance()
            .get_task_info(task_id)
            .ok_or(ErrorCode::SystemApi)?;
        if info.common_data.mode != mode.repr {
            return Err(ErrorCode::TaskStateErr);
        }
        Ok(())
    }
}
