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
use std::net::Shutdown;
use std::sync::Arc;
use std::time::Duration;

pub(crate) use manager::{ClientManager, ClientManagerEntry};
use ylong_http_client::Headers;
use ylong_runtime::net::UnixDatagram;
use ylong_runtime::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use ylong_runtime::sync::oneshot::{channel, Sender};

use crate::config::Version;
use crate::error::ErrorCode;
use crate::task::notify::{NotifyData, SubscribeType, WaitingCause};
use crate::task::reason::Reason;
use crate::utils::{runtime_spawn, Recv};

const REQUEST_MAGIC_NUM: u32 = 0x43434646;
const HEADERS_MAX_SIZE: u16 = 8 * 1024;
const POSITION_OF_LENGTH: u32 = 10;

#[derive(Debug)]
pub(crate) enum ClientEvent {
    OpenChannel(u64, Sender<Result<Arc<UnixDatagram>, ErrorCode>>),
    Subscribe(u32, u64, u64, u64, Sender<ErrorCode>),
    Unsubscribe(u32, Sender<ErrorCode>),
    TaskFinished(u32),
    Terminate(u64, Sender<ErrorCode>),
    SendResponse(u32, String, u32, String, Headers),
    SendNotifyData(SubscribeType, NotifyData),
    SendFaults(u32, SubscribeType, Reason),
    SendWaitNotify(u32, WaitingCause),
    Shutdown,
}

pub(crate) enum MessageType {
    HttpResponse = 0,
    NotifyData,
    Faults,
    Waiting,
}

impl ClientManagerEntry {
    pub(crate) fn open_channel(&self, pid: u64) -> Result<Arc<UnixDatagram>, ErrorCode> {
        let (tx, rx) = channel::<Result<Arc<UnixDatagram>, ErrorCode>>();
        let event = ClientEvent::OpenChannel(pid, tx);
        if !self.send_event(event) {
            return Err(ErrorCode::Other);
        }
        let rx = Recv::new(rx);
        match rx.get() {
            Some(ret) => ret,
            None => {
                error!("open channel fail, recv none");
                sys_event!(
                    ExecFault,
                    DfxCode::UDS_FAULT_03,
                    "open channel fail, recv none"
                );
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
                error!("subscribe fail, recv none");
                sys_event!(
                    ExecFault,
                    DfxCode::UDS_FAULT_03,
                    "subscribe fail, recv none"
                );
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
                sys_event!(ExecFault, DfxCode::UDS_FAULT_03, "unsubscribe failed");
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
                sys_event!(
                    ExecFault,
                    DfxCode::UDS_FAULT_03,
                    "notify_process_terminate failed"
                );
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

    pub(crate) fn send_notify_data(&self, subscribe_type: SubscribeType, notify_data: NotifyData) {
        let event = ClientEvent::SendNotifyData(subscribe_type, notify_data);
        let _ = self.send_event(event);
    }

    pub(crate) fn send_faults(&self, tid: u32, subscribe_type: SubscribeType, reason: Reason) {
        let event = ClientEvent::SendFaults(tid, subscribe_type, reason);
        let _ = self.send_event(event);
    }

    pub(crate) fn send_wait_reason(&self, tid: u32, reason: WaitingCause) {
        let event = ClientEvent::SendWaitNotify(tid, reason);
        let _ = self.send_event(event);
    }
}

// uid and token_id will be used later
pub(crate) struct Client {
    pub(crate) pid: u64,
    pub(crate) message_id: u32,
    pub(crate) server_sock_fd: UnixDatagram,
    pub(crate) client_sock_fd: Arc<UnixDatagram>,
    rx: UnboundedReceiver<ClientEvent>,
}

impl Client {
    pub(crate) fn constructor(
        pid: u64,
    ) -> Option<(UnboundedSender<ClientEvent>, Arc<UnixDatagram>)> {
        let (tx, rx) = unbounded_channel();
        let (server_sock_fd, client_sock_fd) = match UnixDatagram::pair() {
            Ok((server_sock_fd, client_sock_fd)) => (server_sock_fd, client_sock_fd),
            Err(err) => {
                error!("can't create a pair of sockets, {:?}", err);
                sys_event!(
                    ExecFault,
                    DfxCode::TASK_FAULT_09,
                    &format!("can't create a pair of sockets, {:?}", err)
                );
                return None;
            }
        };
        let client_sock_fd = Arc::new(client_sock_fd);
        let client = Client {
            pid,
            message_id: 1,
            server_sock_fd,
            client_sock_fd: client_sock_fd.clone(),
            rx,
        };

        runtime_spawn(client.run());
        Some((tx, client_sock_fd))
    }

    async fn run(mut self) {
        loop {
            // for one task, only send last progress message
            let mut progress_index = HashMap::new();
            let mut temp_notify_data: Vec<(SubscribeType, NotifyData)> = Vec::new();
            let mut len = self.rx.len();
            if len == 0 {
                len = 1;
            }
            for index in 0..len {
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
                    ClientEvent::Shutdown => {
                        let _ = self.client_sock_fd.shutdown(Shutdown::Both);
                        let _ = self.server_sock_fd.shutdown(Shutdown::Both);
                        self.rx.close();
                        info!("client terminate, pid {}", self.pid);
                        return;
                    }
                    ClientEvent::SendResponse(tid, version, status_code, reason, headers) => {
                        self.handle_send_response(tid, version, status_code, reason, headers)
                            .await;
                    }
                    ClientEvent::SendFaults(tid, subscribe_type, reason) => {
                        self.handle_send_faults(tid, subscribe_type, reason).await;
                    }
                    ClientEvent::SendNotifyData(subscribe_type, notify_data) => {
                        if subscribe_type == SubscribeType::Progress {
                            progress_index.insert(notify_data.task_id, index);
                        }
                        temp_notify_data.push((subscribe_type, notify_data));
                    }
                    ClientEvent::SendWaitNotify(task_id, waiting_reason) => {
                        self.handle_send_waiting_notify(task_id, waiting_reason)
                            .await;
                    }
                    _ => {}
                }
            }
            for (index, (subscribe_type, notify_data)) in temp_notify_data.into_iter().enumerate() {
                if subscribe_type != SubscribeType::Progress
                    || progress_index.get(&notify_data.task_id) == Some(&index)
                {
                    self.handle_send_notify_data(subscribe_type, notify_data)
                        .await;
                }
            }
            debug!("Client handle message done");
        }
    }

    async fn handle_send_faults(
        &mut self,
        tid: u32,
        subscribe_type: SubscribeType,
        reason: Reason,
    ) {
        let mut message = Vec::<u8>::new();
        message.extend_from_slice(&REQUEST_MAGIC_NUM.to_le_bytes());

        message.extend_from_slice(&self.message_id.to_le_bytes());
        self.message_id += 1;

        let message_type = MessageType::Faults as u16;
        message.extend_from_slice(&message_type.to_le_bytes());

        let message_body_size: u16 = 0;
        message.extend_from_slice(&message_body_size.to_le_bytes());

        message.extend_from_slice(&tid.to_le_bytes());

        message.extend_from_slice(&(subscribe_type as u32).to_le_bytes());

        message.extend_from_slice(&(reason.repr as u32).to_le_bytes());

        let size = message.len() as u16;
        info!("send faults size, {:?}", size);
        let size = size.to_le_bytes();
        message[POSITION_OF_LENGTH as usize] = size[0];
        message[(POSITION_OF_LENGTH + 1) as usize] = size[1];
        self.send_message(message).await;
    }

    async fn handle_send_waiting_notify(&mut self, task_id: u32, waiting_reason: WaitingCause) {
        let mut message = Vec::<u8>::new();

        message.extend_from_slice(&REQUEST_MAGIC_NUM.to_le_bytes());

        message.extend_from_slice(&self.message_id.to_le_bytes());
        self.message_id += 1;

        let message_type = MessageType::Waiting as u16;
        message.extend_from_slice(&message_type.to_le_bytes());

        let message_body_size: u16 = 0;
        message.extend_from_slice(&message_body_size.to_le_bytes());

        message.extend_from_slice(&task_id.to_le_bytes());

        message.extend_from_slice(&(waiting_reason.clone() as u32).to_le_bytes());

        let size = message.len() as u16;
        debug!(
            "send wait notify, tid {:?} reason {:?} size {:?}",
            task_id, waiting_reason, size
        );
        let size = size.to_le_bytes();
        message[POSITION_OF_LENGTH as usize] = size[0];
        message[(POSITION_OF_LENGTH + 1) as usize] = size[1];

        self.send_message(message).await;
    }

    async fn handle_send_response(
        &mut self,
        tid: u32,
        version: String,
        status_code: u32,
        reason: String,
        headers: Headers,
    ) {
        let mut response = Vec::<u8>::new();

        response.extend_from_slice(&REQUEST_MAGIC_NUM.to_le_bytes());

        response.extend_from_slice(&self.message_id.to_le_bytes());
        self.message_id += 1;

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

        // The maximum length of the headers in uds should not exceed 8192
        let mut buf_size = 0;
        for (k, v) in headers {
            buf_size += k.as_bytes().len() + v.iter().map(|f| f.len()).sum::<usize>();
            if buf_size > HEADERS_MAX_SIZE as usize {
                break;
            }

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
            info!("send response too long");
            response.truncate(HEADERS_MAX_SIZE as usize);
            size = HEADERS_MAX_SIZE;
        }
        debug!("send response size, {:?}", size);
        let size = size.to_le_bytes();
        response[POSITION_OF_LENGTH as usize] = size[0];
        response[(POSITION_OF_LENGTH + 1) as usize] = size[1];

        self.send_message(response).await;
    }

    async fn handle_send_notify_data(
        &mut self,
        subscribe_type: SubscribeType,
        notify_data: NotifyData,
    ) {
        let mut message = Vec::<u8>::new();

        message.extend_from_slice(&REQUEST_MAGIC_NUM.to_le_bytes());

        message.extend_from_slice(&self.message_id.to_le_bytes());
        self.message_id += 1;

        let message_type = MessageType::NotifyData as u16;
        message.extend_from_slice(&message_type.to_le_bytes());

        let message_body_size: u16 = 0;
        message.extend_from_slice(&message_body_size.to_le_bytes());

        message.extend_from_slice(&(subscribe_type as u32).to_le_bytes());

        message.extend_from_slice(&notify_data.task_id.to_le_bytes());

        message.extend_from_slice(&(notify_data.progress.common_data.state as u32).to_le_bytes());

        let index = notify_data.progress.common_data.index;
        message.extend_from_slice(&(index as u32).to_le_bytes());
        // for one task, only send last progress message
        message.extend_from_slice(&(notify_data.progress.processed[index] as u64).to_le_bytes());

        message.extend_from_slice(
            &(notify_data.progress.common_data.total_processed as u64).to_le_bytes(),
        );

        message.extend_from_slice(&(notify_data.progress.sizes.len() as u32).to_le_bytes());
        for size in notify_data.progress.sizes {
            message.extend_from_slice(&size.to_le_bytes());
        }

        // The maximum length of the headers in uds should not exceed 8192
        let mut buf_size = 0;
        let index = notify_data
            .progress
            .extras
            .iter()
            .take_while(|x| {
                buf_size += x.0.len() + x.1.len();
                buf_size < HEADERS_MAX_SIZE as usize
            })
            .count();

        message.extend_from_slice(&(index as u32).to_le_bytes());
        for (key, value) in notify_data.progress.extras.iter().take(index) {
            message.extend_from_slice(key.as_bytes());
            message.push(b'\0');
            message.extend_from_slice(value.as_bytes());
            message.push(b'\0');
        }

        message.extend_from_slice(&(notify_data.action.repr as u32).to_le_bytes());

        message.extend_from_slice(&(notify_data.version as u32).to_le_bytes());

        // Param taskstates used for UploadFile when complete or fail
        message.extend_from_slice(&(notify_data.each_file_status.len() as u32).to_le_bytes());
        for status in notify_data.each_file_status {
            if notify_data.version == Version::API9 {
                message.extend_from_slice(&status.path.into_bytes());
            }
            message.push(b'\0');
            message.extend_from_slice(&(status.reason.repr as u32).to_le_bytes());
            message.extend_from_slice(&status.message.into_bytes());
            message.push(b'\0');
        }

        let size = message.len() as u16;
        if subscribe_type == SubscribeType::Progress {
            debug!(
                "send tid {} {:?} size {}",
                notify_data.task_id, subscribe_type, size
            );
        } else {
            info!(
                "send tid {} {:?} size {}",
                notify_data.task_id, subscribe_type, size
            );
        }

        let size = size.to_le_bytes();
        message[POSITION_OF_LENGTH as usize] = size[0];
        message[(POSITION_OF_LENGTH + 1) as usize] = size[1];

        self.send_message(message).await;
    }

    async fn send_message(&mut self, message: Vec<u8>) {
        let ret = self.server_sock_fd.send(&message).await;
        match ret {
            Ok(size) => {
                debug!("send message ok, pid: {}, size: {}", self.pid, size);
                let mut buf: [u8; 4] = [0; 4];

                match ylong_runtime::time::timeout(
                    Duration::from_millis(500),
                    self.server_sock_fd.recv(&mut buf),
                )
                .await
                {
                    Ok(ret) => match ret {
                        Ok(len) => {
                            debug!("message recv len {:}", len);
                        }
                        Err(e) => {
                            debug!("message recv error: {:?}", e);
                        }
                    },
                    Err(e) => {
                        debug!("message recv {}", e);
                        return;
                    }
                };

                let len: u32 = u32::from_le_bytes(buf);
                if len != message.len() as u32 {
                    debug!("message len bad, send {:?}, recv {:?}", message.len(), len);
                } else {
                    debug!("notify done, pid: {}", self.pid);
                }
            }
            Err(err) => {
                error!("message send error: {:?}", err);
            }
        }
    }
}
