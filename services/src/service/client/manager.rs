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

//! Client management system for the request service.
//! 
//! This module provides components for managing client connections, handling subscriptions,
//! and sending notifications between the service and its clients through Unix domain sockets.

use std::collections::{hash_map, HashMap};
use std::sync::Arc;

use ylong_runtime::net::UnixDatagram;
use ylong_runtime::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use ylong_runtime::sync::oneshot::Sender;

use super::{Client, ClientEvent};

cfg_oh! {
    use crate::ability::PANIC_INFO;
}
use crate::error::ErrorCode;
use crate::utils::runtime_spawn;

/// Lightweight handle for sending events to the `ClientManager`.
///
/// This struct provides a thread-safe, cloneable entry point for sending events to the
/// client manager without direct access to its internal state.
#[derive(Clone)]
pub(crate) struct ClientManagerEntry {
    /// Channel for sending events to the client manager.
    tx: UnboundedSender<ClientEvent>,
}

impl ClientManagerEntry {
    /// Creates a new `ClientManagerEntry` with the provided event sender channel.
    ///
    /// # Arguments
    ///
    /// * `tx` - Unbounded sender channel for client events
    ///
    /// # Returns
    ///
    /// A new `ClientManagerEntry` instance.
    pub(crate) fn new(tx: UnboundedSender<ClientEvent>) -> Self {
        Self { tx }
    }

    /// Sends an event to the client manager.
    ///
    /// # Arguments
    ///
    /// * `event` - The client event to send
    ///
    /// # Returns
    ///
    /// `true` if the event was successfully sent, `false` if the client manager is no longer available.
    ///
    /// # Notes
    ///
    /// On OpenHarmony platforms, failure to send events will log detailed error information
    /// and trigger a system event for debugging purposes.
    pub(crate) fn send_event(&self, event: ClientEvent) -> bool {
        if self.tx.send(event).is_err() {
            // Log detailed error information on OpenHarmony platforms
            #[cfg(feature = "oh")]
            unsafe {
                if let Some(e) = PANIC_INFO.as_ref() {
                    error!("Sends ClientManager event failed {}", e);
                    sys_event!(
                        ExecFault,
                        DfxCode::UDS_FAULT_02,
                        &format!("Sends ClientManager event failed {}", e)
                    );
                } else {
                    info!("ClientManager is unloading");
                }
            }
            return false;
        }
        true
    }
}
/// Core client management system for tracking connections and handling event routing.
///
/// This struct maintains client connections, manages task-to-client mappings,
/// and routes events between the service and its clients.
pub(crate) struct ClientManager {
    // map from pid to client and fd
    /// Map of process IDs to client channels and socket connections.
    clients: HashMap<u64, (UnboundedSender<ClientEvent>, Arc<UnixDatagram>)>,
    /// Map of task IDs to process IDs for notification routing.
    pid_map: HashMap<u32, u64>,
    /// Receiver channel for incoming events to process.
    rx: UnboundedReceiver<ClientEvent>,
}

impl ClientManager {
    /// Initializes a new client manager and returns an entry point for sending events.
    ///
    /// This function creates a new client manager instance, spawns it in a new runtime task,
    /// and returns a lightweight entry point for sending events to it.
    ///
    /// # Returns
    ///
    /// A new `ClientManagerEntry` that can be used to communicate with the client manager.
    pub(crate) fn init() -> ClientManagerEntry {
        debug!("ClientManager init");
        let (tx, rx) = unbounded_channel();
        let client_manager = ClientManager {
            clients: HashMap::new(),
            pid_map: HashMap::new(),
            rx,
        };
        // Spawn the client manager's main loop in a separate task
        runtime_spawn(client_manager.run());
        ClientManagerEntry::new(tx)
    }

    /// Main event processing loop for the client manager.
    ///
    /// This async method continuously receives and processes client events,
    /// routing them to the appropriate handler methods.
    async fn run(mut self) {
        loop {
            let recv = match self.rx.recv().await {
                Ok(message) => message,
                Err(e) => {
                    // Log and report communication errors
                    error!("ClientManager recv error {:?}", e);
                    sys_event!(
                        ExecFault,
                        DfxCode::UDS_FAULT_03,
                        &format!("ClientManager recv error {:?}", e)
                    );
                    continue;
                }
            };

            // Route the received event to the appropriate handler
            match recv {
                ClientEvent::OpenChannel(pid, tx) => self.handle_open_channel(pid, tx),
                ClientEvent::Subscribe(tid, pid, uid, token_id, tx) => {
                    self.handle_subscribe(tid, pid, uid, token_id, tx)
                }
                ClientEvent::Unsubscribe(tid, tx) => self.handle_unsubscribe(tid, tx),
                ClientEvent::TaskFinished(tid) => self.handle_task_finished(tid),
                ClientEvent::Terminate(pid, tx) => self.handle_process_terminated(pid, tx),
                
                // Response event routing
                ClientEvent::SendResponse(tid, version, status_code, reason, headers) => {
                    if let Some(&pid) = self.pid_map.get(&tid) {
                        if let Some((tx, _fd)) = self.clients.get_mut(&pid) {
                            if let Err(err) = tx.send(ClientEvent::SendResponse(
                                tid,
                                version,
                                status_code,
                                reason,
                                headers,
                            )) {
                                error!("send response error, {}", err);
                                sys_event!(
                                    ExecFault,
                                    DfxCode::UDS_FAULT_02,
                                    &format!("send response error, {}", err)
                                );
                            }
                        } else {
                            debug!("response client not found");
                        }
                    } else {
                        debug!("response pid not found");
                    }
                }
                
                // Notification data routing
                ClientEvent::SendNotifyData(subscribe_type, notify_data) => {
                    if let Some(&pid) = self.pid_map.get(&(notify_data.task_id)) {
                        if let Some((tx, _fd)) = self.clients.get_mut(&pid) {
                            if let Err(err) = 
                                tx.send(ClientEvent::SendNotifyData(subscribe_type, notify_data))
                            {
                                error!("send notify data error, {}", err);
                                sys_event!(
                                    ExecFault,
                                    DfxCode::UDS_FAULT_02,
                                    &format!("send notify data error, {}", err)
                                );
                            }
                        } else {
                            debug!("response client not found");
                        }
                    } else {
                        debug!("notify data pid not found");
                    }
                }
                
                // Fault notification routing
                ClientEvent::SendFaults(tid, subscribe_type, reason) => {
                    if let Some(&pid) = self.pid_map.get(&tid) {
                        if let Some((tx, _fd)) = self.clients.get_mut(&pid) {
                            if let Err(err) = 
                                tx.send(ClientEvent::SendFaults(tid, subscribe_type, reason))
                            {
                                error!("send faults error, {}", err);
                                sys_event!(
                                    ExecFault,
                                    DfxCode::UDS_FAULT_02,
                                    &format!("send faults error, {}", err)
                                );
                            }
                        }
                    }
                }
                
                // Wait notification routing
                ClientEvent::SendWaitNotify(tid, reason) => {
                    if let Some(&pid) = self.pid_map.get(&tid) {
                        if let Some((tx, _fd)) = self.clients.get_mut(&pid) {
                            if let Err(err) = tx.send(ClientEvent::SendWaitNotify(tid, reason)) {
                                error!("send faults error, {}", err);
                                sys_event!(
                                    ExecFault,
                                    DfxCode::UDS_FAULT_02,
                                    &format!("send faults error, {}", err)
                                );
                            }
                        }
                    }
                }
                
                // Ignore unhandled events
                _ => {}
            }

            debug!("ClientManager handle message done");
        }
    }

    /// Handles client channel opening requests.
    ///
    /// This method either returns an existing channel for a process or creates a new one.
    ///
    /// # Arguments
    ///
    /// * `pid` - Process ID of the client requesting the channel
    /// * `tx` - One-shot sender to return the result (socket or error)
    fn handle_open_channel(&mut self, pid: u64, tx: Sender<Result<Arc<UnixDatagram>, ErrorCode>>) {
        match self.clients.entry(pid) {
            // Reuse existing connection for the process
            hash_map::Entry::Occupied(o) => {
                let (_, fd) = o.get();
                let _ = tx.send(Ok(fd.clone()));
            }
            // Create new connection if none exists
            hash_map::Entry::Vacant(v) => match Client::constructor(pid) {
                Some((client, ud_fd)) => {
                    let _ = tx.send(Ok(ud_fd.clone()));
                    v.insert((client, ud_fd));
                }
                None => {
                    let _ = tx.send(Err(ErrorCode::Other));
                }
            },
        }
    }

    /// Handles task subscription requests from clients.
    ///
    /// Maps a task ID to a client process for future event routing.
    ///
    /// # Arguments
    ///
    /// * `tid` - Task ID being subscribed to
    /// * `pid` - Process ID of the subscribing client
    /// * `_uid` - User ID (currently unused)
    /// * `_token_id` - Token ID (currently unused)
    /// * `tx` - One-shot sender to confirm subscription status
    fn handle_subscribe(
        &mut self,
        tid: u32,
        pid: u64,
        _uid: u64,
        _token_id: u64,
        tx: Sender<ErrorCode>,
    ) {
        if let Some(_client) = self.clients.get_mut(&pid) {
            // Map task ID to process ID for future notifications
            self.pid_map.insert(tid, pid);
            let _ = tx.send(ErrorCode::ErrOk);
        } else {
            info!("channel not open, pid {}", pid);
            let _ = tx.send(ErrorCode::ChannelNotOpen);
        }
    }

    /// Handles task unsubscription requests.
    ///
    /// Removes the mapping between a task ID and client process.
    ///
    /// # Arguments
    ///
    /// * `tid` - Task ID being unsubscribed from
    /// * `tx` - One-shot sender to confirm unsubscription status
    fn handle_unsubscribe(&mut self, tid: u32, tx: Sender<ErrorCode>) {
        if let Some(&pid) = self.pid_map.get(&tid) {
            self.pid_map.remove(&tid);
            if let Some(_client) = self.clients.get_mut(&pid) {
                let _ = tx.send(ErrorCode::ErrOk);
                return;
            } else {
                debug!("client not found");
            }
        } else {
            debug!("unsubscribe tid not found");
        }
        let _ = tx.send(ErrorCode::Other);
    }

    /// Handles task completion notifications.
    ///
    /// Automatically unsubscribes the client when a task is finished.
    ///
    /// # Arguments
    ///
    /// * `tid` - Task ID that has finished
    fn handle_task_finished(&mut self, tid: u32) {
        if self.pid_map.remove(&tid).is_some() {
            debug!("unsubscribe tid {:?}", tid);
        } else {
            debug!("unsubscribe tid not found");
        }
    }

    /// Handles process termination notifications.
    ///
    /// Cleans up resources associated with a terminated process.
    ///
    /// # Arguments
    ///
    /// * `pid` - Process ID that has terminated
    /// * `tx` - One-shot sender to confirm termination handling
    fn handle_process_terminated(&mut self, pid: u64, tx: Sender<ErrorCode>) {
        if let Some((tx, _)) = self.clients.get_mut(&pid) {
            // Send shutdown signal to the client handler
            let _ = tx.send(ClientEvent::Shutdown);
            // Remove all traces of the client
            self.clients.remove(&pid);
        } else {
            debug!("terminate pid not found");
        }
        let _ = tx.send(ErrorCode::ErrOk);
    }
}
