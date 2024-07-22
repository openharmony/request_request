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
use std::sync::Arc;

use ylong_runtime::sync::oneshot;

use crate::manage::events::{TaskEvent, TaskManagerEvent};
use crate::manage::scheduler::queue::keeper::SAKeeper;
use crate::manage::scheduler::queue::notify_task::NotifyTask;
use crate::manage::task_manager::TaskManagerTx;
use crate::service::runcount::{RunCountEvent, RunCountManagerEntry};
use crate::task::config::Action;
use crate::task::download::download;
use crate::task::request_task::RequestTask;
use crate::task::upload::upload;

pub(crate) struct RunningTask {
    runcount_manager: RunCountManagerEntry,
    task: NotifyTask,
    tx: TaskManagerTx,
    lock: Option<oneshot::Sender<()>>,
    // `_keeper` is never used when executing the task.
    _keeper: SAKeeper,
}

impl RunningTask {
    pub(crate) fn new(
        runcount_manager: RunCountManagerEntry,
        task: Arc<RequestTask>,
        tx: TaskManagerTx,
        keeper: SAKeeper,
        lock: oneshot::Sender<()>,
    ) -> Self {
        // Task start to run, then running count +1.
        runcount_manager.send_event(RunCountEvent::change_runcount(1));
        Self {
            runcount_manager,
            task: NotifyTask::new(task),
            lock: Some(lock),
            tx,
            _keeper: keeper,
        }
    }

    pub(crate) async fn run(self) {
        let action = self.conf.common_data.action;
        match action {
            Action::Download => {
                download(self.task.clone()).await;
            }
            Action::Upload => {
                upload(self.task.clone()).await;
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
        info!("Task {} drop", self.task_id());

        self.tx
            .send_event(TaskManagerEvent::Task(TaskEvent::Finished(
                self.task_id(),
                self.uid(),
            )));
        self.lock.take().unwrap().send(()).unwrap();
        // Task finishes running, then running count -1.
        self.runcount_manager
            .send_event(RunCountEvent::change_runcount(-1));
    }
}
