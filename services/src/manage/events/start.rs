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

use crate::error::ErrorCode;
use crate::manage::events::{EventMessage, TaskMessage};
use crate::manage::TaskManager;
use crate::service::ability::RequestAbility;
use crate::service::runcount::RunCountEvent;
use crate::task::info::{ApplicationState, State};
use crate::task::reason::Reason;
use crate::task::request_task::{run, RequestTask, RunningTask};

impl TaskManager {
    pub(crate) fn start(&mut self, uid: u64, task_id: u32) -> ErrorCode {
        info!("start a task, which task id is {}", task_id);

        if let Some(task) = self.get_task(uid, task_id) {
            let task_state = task.status.lock().unwrap().state;
            if task_state != State::Initialized {
                error!("can not start a task which state is {}", task_state as u32);
                return ErrorCode::TaskStateErr;
            }
            self.start_inner(task);
            ErrorCode::ErrOk
        } else if self.has_task_config_record(task_id) {
            info!(
                "Task exists in database, task_id:{}, try to continue download",
                task_id
            );
            let err_code = self.continue_task_from_database(task_id);
            if err_code != ErrorCode::ErrOk {
                error!(
                    "continue task from database failed, task_id:{}, errCode:{:?}",
                    task_id, err_code
                );
            }
            return err_code;
        } else {
            if self.tasks.contains_key(&task_id) {
                info!("TaskManager start a task, task_id:{} exist, but not found in app_task_map, uid:{}", task_id, uid);
            } else {
                error!(
                    "TaskManager start a task, uid:{}, task_id:{} not exist",
                    uid, task_id
                );
            }
            ErrorCode::TaskStateErr
        }
    }

    pub(crate) fn start_inner(&mut self, task: Arc<RequestTask>) {
        if !task.net_work_online() || !task.check_net_work_status() {
            error!("check net work failed");
            self.after_task_processed(&task);
            return;
        }
        let state = task.status.lock().unwrap().state;
        if state != State::Initialized && state != State::Waiting && state != State::Paused {
            self.after_task_processed(&task);
            return;
        }

        // Everytime a task need to be started, a permit is required.
        let permit = match self
            .limit
            .get_permit(task.conf.common_data.uid, task.conf.version)
        {
            Some(permit) => permit,
            None => {
                debug!("Running task full, waiting for schedule");
                task.set_status(State::Waiting, Reason::RunningTaskMeetLimits);
                return;
            }
        };

        let (state, reason) = {
            let status = task.status.lock().unwrap();
            (status.state, status.reason)
        };
        if state == State::Waiting
            && (reason == Reason::NetworkOffline || reason == Reason::UnsupportedNetworkType)
        {
            task.retry.store(true, Ordering::SeqCst);
            task.tries.fetch_add(1, Ordering::SeqCst);
            task.set_status(State::Retrying, Reason::Default);
        } else {
            task.set_status(State::Running, Reason::Default);
        }

        let task_id = task.conf.common_data.task_id;

        let tx = self.tx.clone();

        let state = ApplicationState::from(
            self.app_state(task.conf.common_data.uid, &task.conf.bundle)
                .load(Ordering::Relaxed),
        );

        let qos_changes = self.qos.insert(&task, state);

        self.change_qos(qos_changes);

        let unloader = self.unloader.clone();

        ylong_runtime::spawn(async move {
            let running_task = RunningTask::new(task.clone(), unloader, permit);
            // Task start running, then runcount ++
            let event = RunCountEvent::change_runcount(1);
            RequestAbility::runcount_manager().send_event(event);

            run(running_task).await;

            // Task running finished, then runcount --
            let event = RunCountEvent::change_runcount(-1);
            RequestAbility::runcount_manager().send_event(event);
            tx.send(EventMessage::Task(TaskMessage::Finished(
                task.conf.common_data.task_id,
            )))
        });

        info!("task {} start success", task_id);
    }
}
