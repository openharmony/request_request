// Copyright (C) 2025 Huawei Device Co., Ltd.
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

use std::fs::File;
use std::io;
use std::os::fd::{FromRawFd, IntoRawFd};
use std::os::unix;

use request_core::info::{NotifyData, Response};
use ylong_runtime::net::UnixDatagram;

use crate::listen::ser::UdsSer;

const MAGIC_NUM: i32 = 0x43434646;
const HTTP_RESPONSE: i16 = 0;
const NOTIFY_DATA: i16 = 1;

pub struct UdsListener {
    socket: UnixDatagram,

    message_id: i32,
}

impl UdsListener {
    pub fn new(file: File) -> Self {
        let socket = unsafe { unix::net::UnixDatagram::from_raw_fd(file.into_raw_fd()) };

        let socket = ylong_runtime::block_on(async { UnixDatagram::from_std(socket).unwrap() });

        Self {
            socket,
            message_id: 1,
        }
    }

    pub async fn recv(&mut self) -> Result<Message, io::Error> {
        let mut buf = [0u8; 4096];
        let size = self.socket.recv(&mut buf).await?;
        let ret = (size as u32).to_ne_bytes();
        self.socket.send(&ret).await?;

        let mut uds = UdsSer::new(&buf[..size]);

        let mut msg_type: i16 = 0;

        if !message_check(&mut uds, size as i16, self.message_id, &mut msg_type) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Message check failed",
            ));
        }

        self.message_id += 1;

        info!("Message ID: {}, Type: {}", self.message_id, msg_type);

        if msg_type == HTTP_RESPONSE {
            let response: Response = uds.read();
            Ok(Message::HttpResponse(response))
        } else if msg_type == NOTIFY_DATA {
            let notify_data: NotifyData = uds.read();
            Ok(Message::NotifyData(notify_data))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown message type: {}", msg_type),
            ))
        }
    }
}

pub enum Message {
    HttpResponse(Response),
    NotifyData(NotifyData),
}

fn message_check(uds: &mut UdsSer, size: i16, message_id: i32, msg_type: &mut i16) -> bool {
    let magic_num: i32 = uds.read();
    if magic_num != MAGIC_NUM as i32 {
        error!("Invalid magic number: {}", magic_num);
        return false;
    }

    let msg_id: i32 = uds.read();
    if msg_id != message_id {
        error!(
            "Message ID mismatch: expected {}, got {}",
            message_id, msg_id
        );
    }

    *msg_type = uds.read();

    let body_size: i16 = uds.read();
    if body_size != size as i16 {
        error!("Body size mismatch: expected {}, got {}", size, body_size);
        return false;
    }
    true
}
