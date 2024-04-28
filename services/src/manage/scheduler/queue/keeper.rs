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
use std::sync::{Arc, Mutex};
use std::time::Duration;

use ylong_runtime::sync::mpsc::UnboundedSender;
use ylong_runtime::task::JoinHandle;

use crate::manage::events::{ScheduleEvent, TaskManagerEvent};
use crate::manage::task_manager::TaskManagerTx;

const UNLOAD_WAITING: u64 = 60;

pub(crate) struct SAKeeper {
    tx: UnboundedSender<TaskManagerEvent>,
    inner: Arc<Mutex<Inner>>,
}

struct Inner {
    cnt: usize,
    handle: Option<JoinHandle<()>>,
}

impl SAKeeper {
    pub(crate) fn new(tx: TaskManagerTx) -> Self {
        info!("Countdown 60s future started");
        let tx = tx.deref();
        let handle = count_down(tx.clone());
        Self {
            tx: tx.clone(),
            inner: Arc::new(Mutex::new(Inner {
                cnt: 0,
                handle: Some(handle),
            })),
        }
    }
}

impl Clone for SAKeeper {
    fn clone(&self) -> Self {
        // Everytime a new task becomes running state, cancel the `60s-countdown`
        // future.
        {
            let mut inner = self.inner.lock().unwrap();
            inner.cnt += 1;
            if inner.cnt != 0 {
                if let Some(handle) = inner.handle.take() {
                    handle.cancel();
                    info!("Countdown 60s future canceled");
                }
            }
        }
        Self {
            tx: self.tx.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl Drop for SAKeeper {
    // Everytime the last running task becomes finished, restart a `60s-countdown`
    // future.
    fn drop(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        inner.cnt -= 1;
        if inner.cnt == 0 {
            info!("Countdown 60s future restarted");
            inner.handle = Some(count_down(self.tx.clone()));
        }
    }
}

fn count_down(tx: UnboundedSender<TaskManagerEvent>) -> JoinHandle<()> {
    ylong_runtime::spawn(unload_sa(tx))
}

async fn unload_sa(tx: UnboundedSender<TaskManagerEvent>) {
    loop {
        ylong_runtime::time::sleep(Duration::from_secs(UNLOAD_WAITING)).await;
        let _ = tx.send(TaskManagerEvent::Schedule(ScheduleEvent::Unload));
    }
}
