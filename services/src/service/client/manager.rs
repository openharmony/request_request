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
use std::net::Shutdown;
use std::os::fd::AsRawFd;

use ylong_http_client::Headers;
use ylong_runtime::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use ylong_runtime::sync::oneshot::Sender;

use super::{Client, ClientEvent};
use crate::error::ErrorCode;
use crate::service::ability::PANIC_INFO;

const REQUEST_MAGIC_NUM: u32 = 0x43434646;
const HEADERS_MAX_SIZE: u16 = 8 * 1024;
const POSITION_OF_LENGTH: u32 = 10;

pub(crate) enum MessageType {
    HttpResponse = 0,
}

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
    clients: HashMap<u64, Client>,
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
                    self.handle_send_response(tid, version, status_code, reason, headers)
                        .await
                }
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
        let client: &Client;
        match self.clients.entry(pid) {
            std::collections::hash_map::Entry::Occupied(o) => {
                client = o.get();
                let _ = tx.send(Ok(client.client_sock_fd.as_raw_fd()));
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                match Client::constructor(pid, uid, token_id) {
                    Some(client) => {
                        let _ = tx.send(Ok(client.client_sock_fd.as_raw_fd()));
                        v.insert(client);
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
        uid: u64,
        token_id: u64,
        tx: Sender<ErrorCode>,
    ) {
        if let Some(client) = self.clients.get_mut(&pid) {
            let ret = client.handle_subscribe(tid, pid, uid, token_id);
            if ret == ErrorCode::ErrOk {
                self.pid_map.insert(tid, pid);
                let _ = tx.send(ErrorCode::ErrOk);
                return;
            }
        } else {
            error!("channel not open");
        }
        let _ = tx.send(ErrorCode::Other);
    }

    fn handle_unsubscribe(&mut self, tid: u32, tx: Sender<ErrorCode>) {
        if let Some(&pid) = self.pid_map.get(&tid) {
            self.pid_map.remove(&tid);
            if let Some(client) = self.clients.get_mut(&pid) {
                client.handle_unsubscribe(tid);
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
        if let Some(&pid) = self.pid_map.get(&tid) {
            self.pid_map.remove(&tid);
            if let Some(client) = self.clients.get_mut(&pid) {
                client.handle_unsubscribe(tid);
            } else {
                debug!("client not found");
            }
        } else {
            debug!("unsubscribe tid not found");
        }
    }

    fn handle_process_terminated(&mut self, pid: u64, tx: Sender<ErrorCode>) {
        if let Some(client) = self.clients.get_mut(&pid) {
            let _ = client.client_sock_fd.shutdown(Shutdown::Both);
            let _ = client.server_sock_fd.shutdown(Shutdown::Both);
            debug!("client terminate, pid: {}", pid);
            for (k, _v) in client.subscribed_map.iter() {
                self.pid_map.remove(k);
            }
            self.clients.remove(&pid);
        } else {
            debug!("terminate pid not found");
        }
        let _ = tx.send(ErrorCode::ErrOk);
    }

    async fn handle_send_response(
        &mut self,
        tid: u32,
        version: String,
        status_code: u32,
        reason: String,
        headers: Headers,
    ) {
        if let Some(&pid) = self.pid_map.get(&tid) {
            if let Some(client) = self.clients.get_mut(&pid) {
                let mut response = Vec::<u8>::new();

                response.extend_from_slice(&REQUEST_MAGIC_NUM.to_le_bytes());

                response.extend_from_slice(&client.message_id.to_le_bytes());
                client.message_id += 1;

                let message_type = MessageType::HttpResponse as u16;
                response.extend_from_slice(&message_type.to_le_bytes());

                let message_body_size: u16 = 0;
                response.extend_from_slice(&message_body_size.to_le_bytes());

                response.extend_from_slice(&tid.to_le_bytes());

                response.extend_from_slice(&version.into_bytes());
                response.push(b'\0');

                response.extend_from_slice(&status_code.to_le_bytes());

                response.extend_from_slice(&reason.into_bytes());
                response.push(b'\0');

                for (k, v) in headers {
                    response.extend_from_slice(k.as_bytes());
                    response.push(b':');
                    for (i, sub_value) in v.iter().enumerate() {
                        if i != 0 {
                            response.push(b',');
                        }
                        response.extend_from_slice(sub_value);
                    }
                    response.push(b'\n');
                }
                let mut size = response.len() as u16;
                if size > HEADERS_MAX_SIZE {
                    response.truncate(HEADERS_MAX_SIZE as usize);
                    size = HEADERS_MAX_SIZE;
                }
                debug!("send response size, {:?}", size);
                let size = size.to_le_bytes();
                response[POSITION_OF_LENGTH as usize] = size[0];
                response[(POSITION_OF_LENGTH + 1) as usize] = size[1];

                client.send_response(tid, response).await;
            } else {
                debug!("response client not found");
            }
        } else {
            debug!("response pid not found");
        }
    }
}
