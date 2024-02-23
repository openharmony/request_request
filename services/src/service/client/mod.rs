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

mod manager;

use std::collections::HashMap;

pub(crate) use manager::{ClientManager, ClientManagerEntry};
use ylong_http_client::Headers;
use ylong_runtime::net::UnixDatagram;
use ylong_runtime::sync::oneshot::{channel, Sender};

use crate::error::ErrorCode;
use crate::utils::Recv;

pub(crate) enum ClientEvent {
    OpenChannel(u64, u64, u64, Sender<Result<i32, ErrorCode>>),
    Subscribe(u32, u64, u64, u64, Sender<ErrorCode>),
    Unsubscribe(u32, Sender<ErrorCode>),
    TaskFinished(u32),
    Terminate(u64, Sender<ErrorCode>),
    SendResponse(u32, String, u32, String, Headers),
}

impl ClientManagerEntry {
    pub(crate) fn open_channel(&self, pid: u64, uid: u64, token_id: u64) -> Result<i32, ErrorCode> {
        let (tx, rx) = channel::<Result<i32, ErrorCode>>();
        let event = ClientEvent::OpenChannel(pid, uid, token_id, tx);
        if !self.send_event(event) {
            return Err(ErrorCode::Other);
        }
        let rx = Recv::new(rx);
        match rx.get() {
            Some(ret) => ret,
            None => {
                error!("open_channel failed");
                Err(ErrorCode::Other)
            }
        }
    }

    pub(crate) fn subscribe(&self, tid: u32, pid: u64, uid: u64, token_id: u64) -> ErrorCode {
        let (tx, rx) = channel::<ErrorCode>();
        let event = ClientEvent::Subscribe(tid, pid, uid, token_id, tx);
        if !self.send_event(event) {
            return ErrorCode::Other;
        }
        let rx = Recv::new(rx);
        match rx.get() {
            Some(ret) => ret,
            None => {
                error!("subscribe failed");
                ErrorCode::Other
            }
        }
    }

    pub(crate) fn unsubscribe(&self, tid: u32) -> ErrorCode {
        let (tx, rx) = channel::<ErrorCode>();
        let event = ClientEvent::Unsubscribe(tid, tx);
        if !self.send_event(event) {
            return ErrorCode::Other;
        }
        let rx = Recv::new(rx);
        match rx.get() {
            Some(ret) => ret,
            None => {
                error!("unsubscribe failed");
                ErrorCode::Other
            }
        }
    }

    pub(crate) fn notify_task_finished(&self, tid: u32) {
        let event = ClientEvent::TaskFinished(tid);
        self.send_event(event);
    }

    pub(crate) fn notify_process_terminate(&self, pid: u64) -> ErrorCode {
        let (tx, rx) = channel::<ErrorCode>();
        let event = ClientEvent::Terminate(pid, tx);
        if !self.send_event(event) {
            return ErrorCode::Other;
        }
        let rx = Recv::new(rx);
        match rx.get() {
            Some(ret) => ret,
            None => {
                error!("notify_process_terminate failed");
                ErrorCode::Other
            }
        }
    }

    pub(crate) fn send_response(
        &self,
        tid: u32,
        version: String,
        status_code: u32,
        reason: String,
        headers: Headers,
    ) {
        let event = ClientEvent::SendResponse(tid, version, status_code, reason, headers);
        let _ = self.send_event(event);
    }
}

// uid and token_id will be used later
#[allow(dead_code)]
pub(crate) struct Client {
    pub(crate) pid: u64,
    pub(crate) uid: u64,
    pub(crate) token_id: u64,
    pub(crate) message_id: u32,
    pub(crate) subscribed_map: HashMap<u32, bool>,
    pub(crate) server_sock_fd: UnixDatagram,
    pub(crate) client_sock_fd: UnixDatagram,
}

impl Client {
    pub(crate) fn constructor(pid: u64, uid: u64, token_id: u64) -> Option<Self> {
        let (server_sock_fd, client_sock_fd) = match UnixDatagram::pair() {
            Ok((server_sock_fd, client_sock_fd)) => (server_sock_fd, client_sock_fd),
            Err(err) => {
                error!("can't create a pair of sockets, {:?}", err);
                return None;
            }
        };
        Some(Client {
            pid,
            uid,
            token_id,
            message_id: 1,
            subscribed_map: HashMap::new(),
            server_sock_fd,
            client_sock_fd,
        })
    }

    pub(crate) fn handle_subscribe(
        &mut self,
        tid: u32,
        _pid: u64,
        _uid: u64,
        _token_id: u64,
    ) -> ErrorCode {
        if let Some(val) = self.subscribed_map.get_mut(&tid) {
            *val = true;
        } else {
            self.subscribed_map.insert(tid, true);
        }

        ErrorCode::ErrOk
    }

    pub(crate) fn handle_unsubscribe(&mut self, tid: u32) {
        if self.subscribed_map.remove(&tid).is_none() {
            error!("tid: {} not subscribed", tid);
        }
    }

    pub(crate) async fn send_response(&mut self, tid: u32, response: Vec<u8>) {
        if let Some(is_subscribed) = self.subscribed_map.get(&tid) {
            if *is_subscribed {
                let _ = self.server_sock_fd.send(&response).await;
            }
        }
    }
}
