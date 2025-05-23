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

#[derive(Clone)]
pub(crate) struct ClientManagerEntry {
    tx: UnboundedSender<ClientEvent>,
}

impl ClientManagerEntry {
    pub(crate) fn new(tx: UnboundedSender<ClientEvent>) -> Self {
        Self { tx }
    }

    pub(crate) fn send_event(&self, event: ClientEvent) -> bool {
        if self.tx.send(event).is_err() {
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
pub(crate) struct ClientManager {
    // map from pid to client and fd
    clients: HashMap<u64, (UnboundedSender<ClientEvent>, Arc<UnixDatagram>)>,
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
        runtime_spawn(client_manager.run());
        ClientManagerEntry::new(tx)
    }

    async fn run(mut self) {
        loop {
            let recv = match self.rx.recv().await {
                Ok(message) => message,
                Err(e) => {
                    error!("ClientManager recv error {:?}", e);
                    sys_event!(
                        ExecFault,
                        DfxCode::UDS_FAULT_03,
                        &format!("ClientManager recv error {:?}", e)
                    );
                    continue;
                }
            };

            match recv {
                ClientEvent::OpenChannel(pid, tx) => self.handle_open_channel(pid, tx),
                ClientEvent::Subscribe(tid, pid, uid, token_id, tx) => {
                    self.handle_subscribe(tid, pid, uid, token_id, tx)
                }
                ClientEvent::Unsubscribe(tid, tx) => self.handle_unsubscribe(tid, tx),
                ClientEvent::TaskFinished(tid) => self.handle_task_finished(tid),
                ClientEvent::Terminate(pid, tx) => self.handle_process_terminated(pid, tx),
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
                _ => {}
            }

            debug!("ClientManager handle message done");
        }
    }

    fn handle_open_channel(&mut self, pid: u64, tx: Sender<Result<Arc<UnixDatagram>, ErrorCode>>) {
        match self.clients.entry(pid) {
            hash_map::Entry::Occupied(o) => {
                let (_, fd) = o.get();
                let _ = tx.send(Ok(fd.clone()));
            }
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
            info!("channel not open, pid {}", pid);
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
        if self.pid_map.remove(&tid).is_some() {
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
