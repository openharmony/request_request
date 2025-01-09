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

use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::config::Mode;
use crate::manage::events::{TaskEvent, TaskManagerEvent};
use crate::manage::notifier::Notifier;
use crate::manage::scheduler::queue::keeper::SAKeeper;
use crate::manage::task_manager::TaskManagerTx;
use crate::service::notification_bar::NotificationDispatcher;
use crate::task::config::Action;
use crate::task::download::download;
use crate::task::reason::Reason;
use crate::task::request_task::RequestTask;
use crate::task::upload::upload;

pub(crate) struct RunningTask {
    task: Arc<RequestTask>,
    tx: TaskManagerTx,
    // `_keeper` is never used when executing the task.
    _keeper: SAKeeper,
}

impl RunningTask {
    pub(crate) fn new(task: Arc<RequestTask>, tx: TaskManagerTx, keeper: SAKeeper) -> Self {
        Self {
            task,
            tx,
            _keeper: keeper,
        }
    }

    pub(crate) async fn run(self, abort_flag: Arc<AtomicBool>) {
        match self.conf.common_data.action {
            Action::Download => {
                download(self.task.clone(), abort_flag).await;
            }
            Action::Upload => {
                upload(self.task.clone(), abort_flag).await;
            }
            _ => {}
        }
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
        self.task.update_progress_in_database();
        Notifier::progress(&self.client_manager, self.build_notify_data());
        let task_id = self.task_id();
        let uid = self.uid();
        let mode = Mode::from(self.mode.load(Ordering::Acquire));
        match *self.task.running_result.lock().unwrap() {
            Some(res) => match res {
                Ok(()) => {
                    if self.task.background_notify.load(Ordering::Acquire) {
                        NotificationDispatcher::get_instance().publish_progress_notification(self);
                    }
                    self.tx
                        .send_event(TaskManagerEvent::Task(TaskEvent::Completed(
                            task_id, uid, mode,
                        )));
                }
                Err(e) if e == Reason::NetworkOffline => {
                    self.tx
                        .send_event(TaskManagerEvent::Task(TaskEvent::Offline(
                            task_id, uid, mode,
                        )));
                }
                Err(e) => {
                    self.tx.send_event(TaskManagerEvent::Task(TaskEvent::Failed(
                        task_id, uid, e, mode,
                    )));
                }
            },
            None => {
                self.tx
                    .send_event(TaskManagerEvent::Task(TaskEvent::Running(
                        task_id, uid, mode,
                    )));
            }
        }
    }
}
