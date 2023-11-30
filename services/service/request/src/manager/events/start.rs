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
use crate::manager::events::{EventMessage, TaskMessage};
use crate::manager::TaskManager;
use crate::task::config::Version;
use crate::task::info::{ApplicationState, State};
use crate::task::reason::Reason;
use crate::task::request_task::run;
use crate::task::RequestTask;
const MAX_RUNNING_TASK_COUNT_EACH_APP: u32 = 5; // api10
const MAX_RUNNING_TASK_COUNT_API9: u32 = 4;

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
        } else {
            if self.tasks.contains_key(&task_id) {
                error!("TaskManager start a task, task_id:{} exist, but not found in app_task_map, uid:{}", task_id, uid);
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

        if self.reach_maximum_running_limit(task.conf.common_data.uid, task.conf.version) {
            info!("too many task in running state");
            task.set_status(State::Waiting, Reason::RunningTaskMeetLimits);
            self.after_task_processed(&task);
            return;
        }

        let (state, reason) = {
            let status = task.status.lock().unwrap();
            (status.state, status.reason)
        };
        if state == State::Waiting
            && (reason == Reason::NetWorkOffline || reason == Reason::UnSupportedNetWorkType)
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

        ylong_runtime::spawn(async move {
            run(task.clone()).await;
            tx.send(EventMessage::Task(TaskMessage::Finished(
                task.conf.common_data.task_id,
            )))
        });

        info!("task {} start success", task_id);
    }

    fn reach_maximum_running_limit(&self, uid: u64, version: Version) -> bool {
        match version {
            Version::API10 => {
                let mut count = 0;
                let tasks = match self.app_task_map.get(&uid) {
                    Some(v) => v,
                    None => return false,
                };
                for task in tasks {
                    let request_task = match self.tasks.get(task) {
                        Some(task) => task,
                        None => {
                            error!("TaskManager reach_maximum_running_limit task_id:{} not found in uid:{}", task, uid);
                            continue;
                        }
                    };
                    if request_task.conf.version == Version::API10 {
                        let state = request_task.status.lock().unwrap().state;
                        if state == State::Retrying || state == State::Running {
                            count += 1;
                        }
                        if count >= MAX_RUNNING_TASK_COUNT_EACH_APP {
                            return true;
                        }
                    }
                }
            }
            Version::API9 => {
                let mut count = 0;
                for request_task in self.tasks.values() {
                    if request_task.conf.version == Version::API9 {
                        let state = request_task.status.lock().unwrap().state;
                        if state == State::Retrying || state == State::Running {
                            count += 1;
                        }
                        if count >= MAX_RUNNING_TASK_COUNT_API9 {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}
