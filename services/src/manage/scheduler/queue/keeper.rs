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

use std::sync::{Arc, Mutex};
use std::time::Duration;

use ylong_runtime::sync::mpsc::UnboundedSender;
use ylong_runtime::task::JoinHandle;

use crate::manage::events::{ScheduleEvent, TaskManagerEvent};
use crate::manage::task_manager::TaskManagerTx;
use crate::service::active_counter::ActiveCounter;
use crate::utils::runtime_spawn;

const UNLOAD_WAITING: u64 = 60;

pub(crate) struct SAKeeper {
    tx: UnboundedSender<TaskManagerEvent>,
    inner: Arc<Mutex<Inner>>,
    active_counter: ActiveCounter,
}

struct Inner {
    cnt: usize,
    handle: Option<JoinHandle<()>>,
}

impl SAKeeper {
    pub(crate) fn new(tx: TaskManagerTx, active_counter: ActiveCounter) -> Self {
        info!("Countdown 60s future started");
        let tx = &tx.tx;
        let handle = count_down(tx.clone());
        Self {
            tx: tx.clone(),
            inner: Arc::new(Mutex::new(Inner {
                cnt: 0,
                handle: Some(handle),
            })),
            active_counter,
        }
    }

    /// Stops repeatedly executing unload_sa.
    pub(crate) fn shutdown(&self) {
        let mut inner = self.inner.lock().unwrap();
        if let Some(handle) = inner.handle.take() {
            handle.cancel();
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
            if inner.cnt == 1 {
                self.active_counter.increment();
                if let Some(handle) = inner.handle.take() {
                    handle.cancel();
                    debug!("Countdown 60s future canceled");
                }
            }
        }
        Self {
            tx: self.tx.clone(),
            inner: self.inner.clone(),
            active_counter: self.active_counter.clone(),
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
            debug!("Countdown 60s future restarted");
            inner.handle = Some(count_down(self.tx.clone()));
            self.active_counter.decrement();
        }
    }
}

fn count_down(tx: UnboundedSender<TaskManagerEvent>) -> JoinHandle<()> {
    runtime_spawn(unload_sa(tx))
}

async fn unload_sa(tx: UnboundedSender<TaskManagerEvent>) {
    loop {
        ylong_runtime::time::sleep(Duration::from_secs(UNLOAD_WAITING)).await;
        let _ = tx.send(TaskManagerEvent::Schedule(ScheduleEvent::Unload));
    }
}
