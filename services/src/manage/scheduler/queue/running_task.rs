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

use std::io::SeekFrom;
use std::ops::Deref;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use ylong_runtime::io::AsyncSeekExt;

use crate::manage::events::{TaskEvent, TaskManagerEvent};
use crate::manage::notifier::Notifier;
use crate::manage::scheduler::queue::keeper::SAKeeper;
use crate::manage::scheduler::queue::notify_task::NotifyTask;
use crate::manage::task_manager::TaskManagerTx;
use crate::service::runcount::{RunCountEvent, RunCountManagerEntry};
use crate::task::config::Action;
use crate::task::download::download;
use crate::task::info::State;
use crate::task::reason::Reason;
use crate::task::request_task::RequestTask;
use crate::task::upload::upload;

pub(crate) struct RunningTask {
    runcount_manager: RunCountManagerEntry,
    task: NotifyTask,
    tx: TaskManagerTx,
    // `_keeper` is never used when executing the task.
    _keeper: SAKeeper,
}

impl RunningTask {
    pub(crate) fn new(
        runcount_manager: RunCountManagerEntry,
        task: Arc<RequestTask>,
        tx: TaskManagerTx,
        keeper: SAKeeper,
    ) -> Self {
        // Task start to run, then running count +1.
        runcount_manager.send_event(RunCountEvent::change_runcount(1));

        let (state, reason) = {
            let status = task.status.lock().unwrap();
            (status.state, status.reason)
        };
        if state == State::Waiting
            && (reason == Reason::NetworkOffline || reason == Reason::UnsupportedNetworkType)
        {
            info!(
                "Retry a waiting task with NetworkOffline/UnsupportedNetworkType, uid:{}, task_id:{}",
                task.conf.common_data.uid, task.conf.common_data.task_id
            );
            task.retry.store(true, Ordering::SeqCst);
            task.tries.fetch_add(1, Ordering::SeqCst);
            task.set_status(State::Retrying, Reason::Default);
        } else {
            if state == State::Paused {
                let notify_data = task.build_notify_data();
                Notifier::resume(&task.client_manager, notify_data);
            }
            task.set_status(State::Running, Reason::Default);
        }

        Self {
            runcount_manager,
            task: NotifyTask::new(task),
            tx,
            _keeper: keeper,
        }
    }

    pub(crate) async fn run(&self) {
        let task = self;
        info!("run the task which id is {}", task.conf.common_data.task_id);

        let action = task.conf.common_data.action;
        match action {
            Action::Download => loop {
                task.reset_code(0);

                download(task.task.clone()).await;

                let state = task.status.lock().unwrap().state;
                if state != State::Running && state != State::Retrying {
                    break;
                }
                let code = task.code.lock().unwrap()[0];
                if code != Reason::Default {
                    task.set_status(State::Failed, code);
                    break;
                }
            },
            Action::Upload => {
                let state = task.status.lock().unwrap().state;
                if state == State::Retrying {
                    let index = {
                        let mut progress_guard = task.progress.lock().unwrap();
                        let index = progress_guard.common_data.index;
                        progress_guard.common_data.total_processed -=
                            progress_guard.processed[index];
                        progress_guard.processed[index] = 0;
                        index
                    };
                    let file = task.files.get_mut(index).unwrap();
                    let mut begins = task.conf.common_data.begins;
                    let (is_partial_upload, _) = task.get_upload_info(index);
                    if !is_partial_upload {
                        begins = 0;
                    }
                    if let Err(e) = file.seek(SeekFrom::Start(begins)).await {
                        task.set_code(index, Reason::IoError);
                        error!("seek err is {:?}", e);
                    }
                }
                upload(task.task.clone()).await;
            }
            _ => {}
        }
        info!("task run end, task_id is {}", task.conf.common_data.task_id);
    }
}

impl Deref for RunningTask {
    type Target = Arc<RequestTask>;

    fn deref(&self) -> &Self::Target {
        &self.task
    }
}

impl Drop for RunningTask {
    fn drop(&mut self) {
        // Task finishes running, then running count -1.
        self.runcount_manager
            .send_event(RunCountEvent::change_runcount(-1));
        let (state, reason) = {
            let status = self.task.status.lock().unwrap();
            (status.state, status.reason)
        };
        // Only tasks that cannot run automatically need to be removed from QoS
        if state == State::Waiting && reason == Reason::RunningTaskMeetLimits {
            return;
        }

        // UserOperation tasks has been removed from qos in TaskManager
        if reason == Reason::UserOperation {
            return;
        }

        let _ = self
            .tx
            .send_event(TaskManagerEvent::Task(TaskEvent::Finished(
                self.task_id(),
                self.uid(),
                self.version(),
            )));
    }
}
