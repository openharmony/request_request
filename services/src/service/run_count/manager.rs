// Copyright (C) 2024 Huawei Device Co., Ltd.
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

//! Provides a manager for tracking and notifying about running task counts.
//! 
//! This module implements a system to maintain the number of running tasks and
//! notify interested clients when this count changes. It uses a manager-worker pattern
//! where the `RunCountManagerEntry` provides an API for clients to interact with the
//! background `RunCountManager` that handles event processing.

use std::collections::HashMap;

use ylong_runtime::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use ylong_runtime::sync::oneshot::{self, Sender};
cfg_oh! {
    use ipc::remote::RemoteObj;
    use crate::ability::PANIC_INFO;
}

use super::{Client, RunCountEvent};
use crate::error::ErrorCode;
use crate::utils::runtime_spawn;

/// Entry point for interacting with the run count manager.
/// 
/// This struct provides a client-facing API to send events to the background
/// `RunCountManager` without blocking the caller.
#[derive(Clone)]
pub(crate) struct RunCountManagerEntry {
    /// Channel for sending events to the RunCountManager worker
    tx: UnboundedSender<RunCountEvent>,
}

impl RunCountManagerEntry {
    /// Creates a new entry point for the run count manager.
    /// 
    /// # Arguments
    /// 
    /// * `tx` - Sender channel for communicating with the RunCountManager
    /// 
    /// # Returns
    /// 
    /// A new `RunCountManagerEntry` instance
    pub(crate) fn new(tx: UnboundedSender<RunCountEvent>) -> Self {
        Self { tx }
    }

    /// Sends an event to the run count manager.
    /// 
    /// # Arguments
    /// 
    /// * `event` - The event to send
    /// 
    /// # Returns
    /// 
    /// `true` if the event was sent successfully, `false` if the manager is down
    pub(crate) fn send_event(&self, event: RunCountEvent) -> bool {
        if self.tx.send(event).is_err() {
            #[cfg(feature = "oh")]
            unsafe {
                if let Some(e) = PANIC_INFO.as_ref() {
                    error!("Sends RunCountManager event failed {}", e);
                    sys_event!(
                        ExecFault,
                        DfxCode::UDS_FAULT_02,
                        &format!("Sends RunCountManager event failed {}", e)
                    );
                } else {
                    info!("RunCountManager is unloading");
                }
            }
            return false;
        }
        true
    }
    #[cfg(feature = "oh")]
    /// Subscribes to run count updates.
    /// 
    /// Registers a client to receive notifications when the run count changes.
    /// 
    /// # Arguments
    /// 
    /// * `pid` - Process ID of the client
    /// * `obj` - Remote object for IPC communication
    /// 
    /// # Returns
    /// 
    /// Error code indicating success or failure
    pub(crate) fn subscribe_run_count(&self, pid: u64, obj: RemoteObj) -> ErrorCode {
        let (tx, rx) = oneshot::channel::<ErrorCode>();
        let event = RunCountEvent::Subscribe(pid, obj, tx);
        self.send_event(event);
        match ylong_runtime::block_on(rx) {
            Ok(error_code) => error_code,
            Err(error) => {
                error!("In `subscribe_run_count`, block on failed, err {}", error);
                // todo: may be another error code
                ErrorCode::Other
            }
        }
    }

    /// Unsubscribes from run count updates.
    /// 
    /// Removes a client's registration for run count notifications.
    /// 
    /// # Arguments
    /// 
    /// * `pid` - Process ID of the client
    /// 
    /// # Returns
    /// 
    /// Error code indicating success or failure
    pub(crate) fn unsubscribe_run_count(&self, pid: u64) -> ErrorCode {
        let (tx, rx) = oneshot::channel::<ErrorCode>();
        let event = RunCountEvent::Unsubscribe(pid, tx);
        self.send_event(event);
        ylong_runtime::block_on(rx).unwrap()
    }

    #[cfg(feature = "oh")]
    /// Notifies all subscribers of a run count change.
    /// 
    /// Triggers an update to broadcast the new run count to all registered clients.
    /// 
    /// # Arguments
    /// 
    /// * `new_count` - The new number of running tasks
    pub(crate) fn notify_run_count(&self, new_count: usize) {
        let event = RunCountEvent::Change(new_count);
        self.send_event(event);
    }
}

/// Background manager that processes run count events and notifies subscribers.
/// 
/// Maintains the current count of running tasks and manages subscriptions,
/// running in a dedicated asynchronous task to handle events.
pub(crate) struct RunCountManager {
    /// Current number of running tasks
    count: usize,
    /// Map of subscribers with their process IDs as keys
    remotes: HashMap<u64, Client>,
    /// Channel for receiving events from RunCountManagerEntry
    rx: UnboundedReceiver<RunCountEvent>,
}

impl RunCountManager {
    /// Initializes the run count manager and starts its event loop.
    /// 
    /// Spawns a background task to process run count events and returns
    /// an entry point for clients to interact with the manager.
    /// 
    /// # Returns
    /// 
    /// A new `RunCountManagerEntry` instance for client interaction
    pub(crate) fn init() -> RunCountManagerEntry {
        debug!("RunCountManager init");
        let (tx, rx) = unbounded_channel();
        let run_count_manager = RunCountManager {
            count: 0,
            remotes: HashMap::new(),
            rx,
        };
        runtime_spawn(run_count_manager.run());
        RunCountManagerEntry::new(tx)
    }

    /// Main event processing loop for the run count manager.
    /// 
    /// Continuously receives and processes events from the RunCountManagerEntry,
    /// handling subscription, unsubscription, and run count change operations.
    async fn run(mut self) {
        loop {
            let recv = match self.rx.recv().await {
                Ok(message) => message,
                Err(e) => {
                    error!("RunCountManager recv error {:?}", e);
                    sys_event!(
                        ExecFault,
                        DfxCode::UDS_FAULT_03,
                        &format!("RunCountManager recv error {:?}", e)
                    );
                    continue;
                }
            };

            match recv {
                #[cfg(feature = "oh")]
                RunCountEvent::Subscribe(pid, obj, tx) => self.subscribe_run_count(pid, obj, tx),
                RunCountEvent::Unsubscribe(pid, tx) => self.unsubscribe_run_count(pid, tx),
                #[cfg(feature = "oh")]
                RunCountEvent::Change(change) => self.change_run_count(change),
            }

            debug!("RunCountManager handle message done");
        }
    }

    #[cfg(feature = "oh")]
    /// Handles a subscription request.
    /// 
    /// Registers a new client for run count notifications and immediately sends
    /// the current run count to the client.
    /// 
    /// # Arguments
    /// 
    /// * `pid` - Process ID of the client
    /// * `obj` - Remote object for IPC communication
    /// * `tx` - Sender channel to return the result
    fn subscribe_run_count(&mut self, pid: u64, obj: RemoteObj, tx: Sender<ErrorCode>) {
        let client = Client::new(obj);

        let _ = client.notify_run_count(self.count as i64);
        self.remotes.insert(pid, client);

        let _ = tx.send(ErrorCode::ErrOk);
    }

    /// Handles an unsubscription request.
    /// 
    /// Removes a client from the subscriber list if they were registered.
    /// 
    /// # Arguments
    /// 
    /// * `subscribe_pid` - Process ID of the client to unsubscribe
    /// * `tx` - Sender channel to return the result
    fn unsubscribe_run_count(&mut self, subscribe_pid: u64, tx: Sender<ErrorCode>) {
        if self.remotes.remove(&subscribe_pid).is_some() {
            let _ = tx.send(ErrorCode::ErrOk);
        } else {
            let _ = tx.send(ErrorCode::Other);
        }
    }

    #[cfg(feature = "oh")]
    /// Updates the run count and notifies all subscribers.
    /// 
    /// Updates the internal count and broadcasts the change to all registered clients.
    /// Removes any clients that fail to receive the update.
    /// 
    /// # Arguments
    /// 
    /// * `new_count` - The new number of running tasks
    fn change_run_count(&mut self, new_count: usize) {
        // Skip update if count hasn't changed to avoid unnecessary notifications
        if self.count == new_count {
            return;
        }
        self.count = new_count;
        // Notify all clients and automatically remove any that fail to receive the update
        self.remotes
            .retain(|_, remote| remote.notify_run_count(self.count as i64).is_ok());
    }
}
