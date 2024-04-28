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

use std::collections::HashMap;

use ylong_runtime::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use ylong_runtime::sync::oneshot::Sender;

use super::{Client, ClientEvent};
use crate::error::ErrorCode;
use crate::init::PANIC_INFO;

#[derive(Clone)]
pub(crate) struct ClientManagerEntry {
    tx: UnboundedSender<ClientEvent>,
}

impl ClientManagerEntry {
    fn new(tx: UnboundedSender<ClientEvent>) -> Self {
        Self { tx }
    }

    pub(crate) fn send_event(&self, event: ClientEvent) -> bool {
        if self.tx.send(event).is_err() {
            unsafe {
                if let Some(e) = PANIC_INFO.as_ref() {
                    error!("Sends ClientManager event failed {}", e);
                } else {
                    info!("ClientManager is unloading");
                }
            }
            return false;
        }
        true
    }
}
pub(crate) struct ClientManager {
    // map from pid to client and fd
    clients: HashMap<u64, (UnboundedSender<ClientEvent>, i32)>,
    pid_map: HashMap<u32, u64>,
    rx: UnboundedReceiver<ClientEvent>,
}

impl ClientManager {
    pub(crate) fn init() -> ClientManagerEntry {
        debug!("ClientManager init");
        let (tx, rx) = unbounded_channel();
        let client_manager = ClientManager {
            clients: HashMap::new(),
            pid_map: HashMap::new(),
            rx,
        };
        ylong_runtime::spawn(client_manager.run());
        ClientManagerEntry::new(tx)
    }

    async fn run(mut self) {
        loop {
            let recv = match self.rx.recv().await {
                Ok(message) => message,
                Err(e) => {
                    error!("ClientManager recv error {:?}", e);
                    continue;
                }
            };

            match recv {
                ClientEvent::OpenChannel(pid, uid, token_id, tx) => {
                    self.handle_open_channel(pid, uid, token_id, tx)
                }
                ClientEvent::Subscribe(tid, pid, uid, token_id, tx) => {
                    self.handle_subscribe(tid, pid, uid, token_id, tx)
                }
                ClientEvent::Unsubscribe(tid, tx) => self.handle_unsubscribe(tid, tx),
                ClientEvent::TaskFinished(tid) => self.handle_task_finished(tid),
                ClientEvent::Terminate(pid, tx) => self.handle_process_terminated(pid, tx),
                ClientEvent::SendResponse(tid, version, status_code, reason, headers) => {
                    if let Some(&pid) = self.pid_map.get(&tid) {
                        if let Some((rx, _fd)) = self.clients.get_mut(&pid) {
                            let _ = rx.send(ClientEvent::SendResponse(
                                tid,
                                version,
                                status_code,
                                reason,
                                headers,
                            ));
                        } else {
                            debug!("response client not found");
                        }
                    } else {
                        debug!("response pid not found");
                    }
                }
                ClientEvent::SendNotifyData(subscribe_type, notify_data) => {
                    if let Some(&pid) = self.pid_map.get(&(notify_data.task_id)) {
                        if let Some((rx, _fd)) = self.clients.get_mut(&pid) {
                            let _ =
                                rx.send(ClientEvent::SendNotifyData(subscribe_type, notify_data));
                        } else {
                            debug!("response client not found");
                        }
                    } else {
                        debug!("notify data pid not found");
                    }
                }
                _ => {}
            }

            debug!("ClientManager handle message done");
        }
    }

    fn handle_open_channel(
        &mut self,
        pid: u64,
        uid: u64,
        token_id: u64,
        tx: Sender<Result<i32, ErrorCode>>,
    ) {
        match self.clients.entry(pid) {
            std::collections::hash_map::Entry::Occupied(o) => {
                let (_, fd) = o.get();
                let _ = tx.send(Ok(*fd));
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                match Client::constructor(pid, uid, token_id) {
                    Some((client, fd)) => {
                        let _ = tx.send(Ok(fd));
                        v.insert((client, fd));
                    }
                    None => {
                        let _ = tx.send(Err(ErrorCode::Other));
                    }
                }
            }
        }
    }

    fn handle_subscribe(
        &mut self,
        tid: u32,
        pid: u64,
        _uid: u64,
        _token_id: u64,
        tx: Sender<ErrorCode>,
    ) {
        if let Some(_client) = self.clients.get_mut(&pid) {
            self.pid_map.insert(tid, pid);
            let _ = tx.send(ErrorCode::ErrOk);
        } else {
            info!("channel not open, pid: {}", pid);
            let _ = tx.send(ErrorCode::ChannelNotOpen);
        }
    }

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

    fn handle_task_finished(&mut self, tid: u32) {
        if self.pid_map.contains_key(&tid) {
            self.pid_map.remove(&tid);
            debug!("unsubscribe tid {:?}", tid);
        } else {
            debug!("unsubscribe tid not found");
        }
    }

    fn handle_process_terminated(&mut self, pid: u64, tx: Sender<ErrorCode>) {
        if let Some((tx, _)) = self.clients.get_mut(&pid) {
            let _ = tx.send(ClientEvent::Shutdown);
            self.clients.remove(&pid);
        } else {
            debug!("terminate pid not found");
        }
        let _ = tx.send(ErrorCode::ErrOk);
    }
}
