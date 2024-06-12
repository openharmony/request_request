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

mod keeper;
mod notify_task;
mod running_task;

use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use keeper::SAKeeper;
pub(crate) use notify_task::NotifyTask;
use running_task::RunningTask;

use crate::error::ErrorCode;
use crate::init::SYSTEM_CONFIG_MANAGER;
use crate::manage::app_state::AppStateManagerTx;
use crate::manage::database::Database;
use crate::manage::notifier::Notifier;
use crate::manage::scheduler::qos::{QosChanges, QosDirection};
use crate::manage::task_manager::TaskManagerTx;
use crate::service::client::ClientManagerEntry;
use crate::service::runcount::RunCountManagerEntry;
use crate::task::config::Action;
use crate::task::info::State;
use crate::task::reason::Reason;
use crate::task::request_task::RequestTask;
use crate::utils::get_current_timestamp;

const MILLISECONDS_IN_ONE_MONTH: u64 = 30 * 24 * 60 * 60 * 1000;

pub(crate) struct RunningQueue {
    download_queue: HashMap<u32, Arc<RequestTask>>,
    upload_queue: HashMap<u32, Arc<RequestTask>>,
    keeper: SAKeeper,
    tx: TaskManagerTx,
    app_state_manager: AppStateManagerTx,
    runcount_manager: RunCountManagerEntry,
    client_manager: ClientManagerEntry,
}

impl RunningQueue {
    pub(crate) fn new(
        tx: TaskManagerTx,
        runcount_manager: RunCountManagerEntry,
        app_state_manager: AppStateManagerTx,
        client_manager: ClientManagerEntry,
    ) -> Self {
        Self {
            download_queue: HashMap::new(),
            upload_queue: HashMap::new(),
            keeper: SAKeeper::new(tx.clone()),
            tx,
            app_state_manager,
            runcount_manager,
            client_manager,
        }
    }

    pub(crate) fn tasks(&self) -> impl Iterator<Item = &Arc<RequestTask>> {
        self.download_queue
            .values()
            .chain(self.upload_queue.values())
    }

    pub(crate) fn get_task(&self, task_id: u32) -> Option<&Arc<RequestTask>> {
        self.download_queue
            .get(&task_id)
            .or(self.upload_queue.get(&task_id))
    }

    pub(crate) fn running_tasks(&self) -> usize {
        self.download_queue.len() + self.upload_queue.len()
    }

    pub(crate) fn dump_tasks(&self) {
        info!(
            "dump all task info, running tasks count: {}",
            self.running_tasks()
        );

        for (task_id, task) in self.download_queue.iter().chain(self.upload_queue.iter()) {
            let task_status = task.status.lock().unwrap();
            info!("dump task message, task_id:{}, action:{}, mode:{}, bundle name:{}, task_status:{:?}",
                task_id, task.action() as u8, task.mode() as u8, task.bundle(), *task_status);
        }
    }

    pub(crate) fn clear_timeout_tasks(&mut self) {
        let current_time = get_current_timestamp();

        for task in self.tasks() {
            if current_time - task.ctime > MILLISECONDS_IN_ONE_MONTH {
                task.set_status(State::Stopped, Reason::TaskSurvivalOneMonth);
                continue;
            }
        }
    }

    pub(crate) async fn reschedule(&mut self, qos: QosChanges) {
        if let Some(vec) = qos.download {
            self.download_queue = self.reschedule_inner(Action::Download, vec).await;
        }
        if let Some(vec) = qos.upload {
            self.upload_queue = self.reschedule_inner(Action::Upload, vec).await;
        }
    }

    pub(crate) async fn reschedule_inner(
        &mut self,
        action: Action,
        qos_vec: Vec<QosDirection>,
    ) -> HashMap<u32, Arc<RequestTask>> {
        let mut satisfied_tasks = HashMap::new();

        let queue = if action == Action::Download {
            &mut self.download_queue
        } else {
            &mut self.upload_queue
        };

        // We need to decide which tasks need to continue running based on `QosChanges`.
        for qos_direction in qos_vec.iter() {
            let task_id = qos_direction.task_id();

            if let Some(task) = queue.remove(&task_id) {
                // If we can find that the task is running in `running_tasks`,
                // we just need to adjust its rate.
                task.speed_limit(qos_direction.direction() as u8);
                // Then we put it into `satisfied_tasks`.
                satisfied_tasks.insert(task_id, task);
                continue;
            }

            // If the task is not in the current running queue, retrieve
            // the corresponding task from the database and start it.
            let system_config = unsafe { SYSTEM_CONFIG_MANAGER.assume_init_ref().system_config() };
            let task = match Database::get_instance()
                .get_task(
                    task_id,
                    system_config,
                    &self.app_state_manager,
                    &self.client_manager,
                )
                .await
            {
                Some(task) => task,
                None => continue,
            };

            let keeper = self.keeper.clone();
            let tx = self.tx.clone();
            let runcount_manager = self.runcount_manager.clone();
            task.speed_limit(qos_direction.direction() as u8);
            satisfied_tasks.insert(task_id, task.clone());
            let task = RunningTask::new(runcount_manager, task.clone(), tx, keeper);
            if !task.satisfied() {
                continue;
            }
            ylong_runtime::spawn(async move {
                loop {
                    task.run().await;
                    let (state, reason) = {
                        let status = task.status.lock().unwrap();
                        (status.state, status.reason)
                    };
                    if state == State::Waiting
                        && reason == Reason::NetworkChanged
                        && task.satisfied()
                    {
                        task.retry.store(true, Ordering::SeqCst);
                        task.tries.fetch_add(1, Ordering::SeqCst);
                        task.set_status(State::Retrying, Reason::Default);
                    } else {
                        break;
                    }
                }
            });
        }
        // every satisfied tasks in running has been moved, set left tasks to Waiting
        for task in queue.values_mut() {
            let state = task.status.lock().unwrap().state;
            if state == State::Running {
                task.set_status(State::Waiting, Reason::RunningTaskMeetLimits);
            }
        }
        satisfied_tasks
    }

    pub(crate) fn modify_task_state_by_user(&mut self, task_id: u32, state: State) -> ErrorCode {
        if let Some(task) = self
            .download_queue
            .remove(&task_id)
            .or(self.upload_queue.remove(&task_id))
        {
            set_task_state_by_user(&self.client_manager, task, state)
        } else {
            ErrorCode::TaskNotFound
        }
    }
}

fn set_task_state_by_user(
    client_manager: &ClientManagerEntry,
    task: Arc<RequestTask>,
    state: State,
) -> ErrorCode {
    if !task.set_status(state, Reason::UserOperation) {
        return ErrorCode::TaskStateErr;
    }
    if state == State::Removed {
        Notifier::remove(client_manager, task.build_notify_data());
    }
    task.resume.store(false, Ordering::SeqCst);
    ErrorCode::ErrOk
}
