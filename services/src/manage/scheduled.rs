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

use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use ylong_runtime::sync::mpsc::UnboundedSender;

use super::events::{EventMessage, ScheduledMessage};
use super::TaskManager;
use crate::task::config::Version;
use crate::task::info::State;
use crate::task::reason::Reason;
use crate::task::RequestTask;
use crate::utils::get_current_timestamp;

const MILLISECONDS_IN_ONE_DAY: u64 = 24 * 60 * 60 * 1000;
const MILLISECONDS_IN_ONE_MONTH: u64 = 30 * 24 * 60 * 60 * 1000;
const CLEAR_INTERVAL: u64 = 30 * 60;
const LOG_INTERVAL: u64 = 5 * 60;
const UNLOAD_WAITING: u64 = 60;
const BACKGROUND_TASK_STOP_INTERVAL: u64 = 60;
const RESTORE_ALL_TASKS_INTERVAL: u64 = 10;

// monitor tasks, tasks in waiting state turn to stop after one day, tasks in
// other state turn to stop after one month.
pub(crate) async fn clear_timeout_tasks(tx: UnboundedSender<EventMessage>) {
    loop {
        ylong_runtime::time::sleep(Duration::from_secs(CLEAR_INTERVAL)).await;
        let _ = tx.send(EventMessage::Scheduled(ScheduledMessage::ClearTimeoutTasks));
    }
}

pub(crate) async fn log_all_task_info(tx: UnboundedSender<EventMessage>) {
    loop {
        ylong_runtime::time::sleep(Duration::from_secs(LOG_INTERVAL)).await;
        let _ = tx.send(EventMessage::Scheduled(ScheduledMessage::LogTasks));
    }
}

pub(crate) async fn unload_sa(tx: UnboundedSender<EventMessage>) {
    ylong_runtime::time::sleep(Duration::from_secs(UNLOAD_WAITING)).await;
    let _ = tx.send(EventMessage::Scheduled(ScheduledMessage::Unload));
}

pub(crate) async fn update_background_app(uid: u64, tx: UnboundedSender<EventMessage>) {
    ylong_runtime::time::sleep(Duration::from_secs(BACKGROUND_TASK_STOP_INTERVAL)).await;
    let _ = tx.send(EventMessage::Scheduled(
        ScheduledMessage::UpdateBackgroundApp(uid),
    ));
}

pub(crate) async fn restore_all_tasks(tx: UnboundedSender<EventMessage>) {
    ylong_runtime::time::sleep(Duration::from_secs(RESTORE_ALL_TASKS_INTERVAL)).await;
    let _ = tx.send(EventMessage::Scheduled(ScheduledMessage::RestoreAllTasks));
}

impl TaskManager {
    pub(crate) fn clear_timeout_tasks(&mut self) {
        let mut remove_tasks = Vec::<Arc<RequestTask>>::new();

        for task in self.tasks.values() {
            let current_time = get_current_timestamp();
            let (state, time) = {
                let guard = task.status.lock().unwrap();
                (guard.state, guard.waitting_network_time)
            };
            if state == State::Waiting {
                if let Some(t) = time {
                    if current_time - t > MILLISECONDS_IN_ONE_DAY {
                        task.set_status(State::Stopped, Reason::WaittingNetWorkOneday);
                        remove_tasks.push(task.clone());
                    }
                }
            }
            if task.conf.version == Version::API9 {
                continue;
            }
            if current_time - task.ctime > MILLISECONDS_IN_ONE_MONTH {
                task.set_status(State::Stopped, Reason::TaskSurvivalOneMonth);
                remove_tasks.push(task.clone());
                continue;
            }
        }

        for task in remove_tasks {
            self.after_task_processed(&task);
        }
    }

    pub(crate) fn log_all_task_info(&self) {
        let api10_background_task_count = self.api10_background_task_count;
        let recording_rdb_num = self.recording_rdb_num.load(Ordering::SeqCst);
        info!(
            "dump all task info, api10_background_task_count:{}, recording_rdb_num:{}",
            api10_background_task_count, recording_rdb_num
        );
        for (task_id, task) in self.tasks.iter() {
            let task_status = task.status.lock().unwrap();
            info!("dump task message, task_id:{}, action:{}, mode:{}, bundle name:{}, task_status:{:?}",
                task_id, task.conf.common_data.action as u8, task.conf.common_data.mode as u8, task.conf.bundle, *task_status);
        }
    }

    pub(crate) fn schedule_unload_sa(&mut self) {
        debug!("TaskManage clock 60s to close sa");
        let tx = self.tx.clone();
        match self.unload_handle.take() {
            Some(handle) => {
                handle.cancel();
                self.unload_handle = Some(ylong_runtime::spawn(unload_sa(tx)));
            }
            None => {
                self.unload_handle = Some(ylong_runtime::spawn(unload_sa(tx)));
            }
        }
    }
}
