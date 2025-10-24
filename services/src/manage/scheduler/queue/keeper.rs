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

//! Service Ability (SA) unload scheduler for idle state detection.
//! 
//! This module implements a countdown-based mechanism that triggers service unloading
//! when no active tasks are running for a specified period. It tracks task activity
//! and manages cleanup timing to optimize resource usage.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use ylong_runtime::sync::mpsc::UnboundedSender;
use ylong_runtime::task::JoinHandle;

use crate::manage::events::{ScheduleEvent, TaskManagerEvent};
use crate::manage::task_manager::TaskManagerTx;
use crate::service::active_counter::ActiveCounter;
use crate::utils::runtime_spawn;

/// Number of seconds to wait before triggering service unload when idle.
const UNLOAD_WAITING: u64 = 60;

/// Service Ability keeper that manages idle timeout and unload scheduling.
///
/// This struct maintains a countdown timer that triggers service unloading
/// when there are no active tasks running for a specified period. It tracks
/// task activity through clone and drop operations to start/stop the timer.
pub(crate) struct SAKeeper {
    /// Sender for transmitting task management events.
    tx: UnboundedSender<TaskManagerEvent>,
    /// Shared internal state protected by a mutex.
    inner: Arc<Mutex<Inner>>,
    /// Counter for tracking active tasks.
    active_counter: ActiveCounter,
}

/// Internal state of the SAKeeper.
struct Inner {
    /// Count of active tasks using this keeper.
    cnt: usize,
    /// Join handle for the countdown task, if active.
    handle: Option<JoinHandle<()>>,
}

impl SAKeeper {
    /// Creates a new SAKeeper instance with initial countdown started.
    ///
    /// # Arguments
    ///
    /// * `tx` - Task manager sender for broadcasting schedule events.
    /// * `active_counter` - Counter for tracking active tasks across the system.
    ///
    /// # Returns
    ///
    /// A new `SAKeeper` instance with an initial countdown timer running.
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
    ///
    /// Cancels any active countdown timer and prevents further unload events
    /// from being triggered by this keeper instance.
    pub(crate) fn shutdown(&self) {
        let mut inner = self.inner.lock().unwrap();
        if let Some(handle) = inner.handle.take() {
            handle.cancel();
        }
    }
}

impl Clone for SAKeeper {
    /// Creates a clone of the SAKeeper and increments the active task count.
    ///
    /// # Notes
    ///
    /// Every time a new task enters running state and clones this keeper,
    /// the countdown timer is canceled if this is the first active task.
    /// This prevents service unloading while tasks are active.
    fn clone(&self) -> Self {
        {
            let mut inner = self.inner.lock().unwrap();
            inner.cnt += 1;
            // Only cancel countdown and increment active counter when transitioning from 0 to 1 tasks
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
    /// Decrements the active task count and restarts countdown when idle.
    ///
    /// # Notes
    ///
    /// When the last running task finishes and this keeper is dropped,
    /// a new countdown timer is started. After the timeout period,
    /// a service unload event will be triggered if no new tasks become active.
    fn drop(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        inner.cnt -= 1;
        // Only restart countdown and decrement active counter when transitioning from 1 to 0 tasks
        if inner.cnt == 0 {
            debug!("Countdown 60s future restarted");
            inner.handle = Some(count_down(self.tx.clone()));
            self.active_counter.decrement();
        }
    }
}

/// Spawns a background task that runs the countdown for service unloading.
///
/// # Arguments
///
/// * `tx` - Sender for transmitting the unload event when countdown completes.
///
/// # Returns
///
/// A `JoinHandle` for the spawned countdown task.
fn count_down(tx: UnboundedSender<TaskManagerEvent>) -> JoinHandle<()> {
    runtime_spawn(unload_sa(tx))
}

/// Async function that waits for the timeout period and then sends an unload event.
///
/// This function runs in an infinite loop, sleeping for the specified timeout period
/// and then sending an unload event. It continues until the task is canceled.
///
/// # Arguments
///
/// * `tx` - Sender for transmitting the unload event.
async fn unload_sa(tx: UnboundedSender<TaskManagerEvent>) {
    loop {
        // Wait for the configured timeout period
        ylong_runtime::time::sleep(Duration::from_secs(UNLOAD_WAITING)).await;
        // Send the unload event, ignoring any send errors
        let _ = tx.send(TaskManagerEvent::Schedule(ScheduleEvent::Unload));
    }
}
