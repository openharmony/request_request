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
#![allow(unused)]
#![allow(missing_docs)]
#![cfg(feature = "oh")]

use std::collections::HashMap;
use std::ffi::{c_char, CString};
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd};
use std::os::unix::net::UnixDatagram;
use std::sync::{Arc, Mutex, Once};
use std::thread;

use download_server::config::{Action, Mode, TaskConfig};
use download_server::info::State;
use download_server::interface;
use ipc::parcel::{Deserialize, MsgParcel};
use ipc::remote::RemoteObj;
use once_cell::sync::{Lazy, OnceCell};
use samgr::definition::DOWNLOAD_SERVICE_ID;
use samgr::manage::SystemAbilityManager;

const SERVICE_TOKEN: &str = "OHOS.Download.RequestServiceInterface";
pub const CHANNEL_MAGIC_NUM: u32 = 0x43434646;

#[allow(clippy::type_complexity)]
static MESSAGES: OnceCell<Arc<Mutex<HashMap<u32, Vec<MessageInfo>>>>> = OnceCell::new();

pub fn test_init() -> RequestAgent {
    let remote = remote();
    RequestAgent::new(remote)
}

pub struct RequestAgent {
    remote: RemoteObj,
    messages: Arc<Mutex<HashMap<u32, Vec<MessageInfo>>>>,
}

impl RequestAgent {
    fn new(remote: RemoteObj) -> Self {
        static ONCE: Once = Once::new();
        let messages = MESSAGES.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
        ONCE.call_once(|| {
            let mut data = MsgParcel::new();
            data.write_interface_token(SERVICE_TOKEN).unwrap();
            let mut reply = remote
                .send_request(interface::OPEN_CHANNEL, &mut data)
                .unwrap();
            let ret: i32 = reply.read().unwrap();
            assert_eq!(0, ret);
            let file = reply.read_file().unwrap();
            let channel = unsafe { UnixDatagram::from_raw_fd(file.into_raw_fd()) };
            thread::spawn(move || loop {
                let mut buf = [0u8; 4096];
                let Ok(length) = channel.recv(&mut buf) else {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    continue;
                };
                channel
                    .send((length as u32).to_le_bytes().as_slice())
                    .unwrap();
                let (task_id, info) = deserialize(&buf);
                let mut map = messages.lock().unwrap();
                match map.get_mut(&task_id) {
                    Some(v) => v.push(info),
                    None => {
                        map.insert(task_id, vec![info]);
                    }
                };
            });
        });

        RequestAgent {
            remote,
            messages: messages.clone(),
        }
    }

    pub fn construct(&self, config: TaskConfig) -> u32 {
        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();
        data.write(&config).unwrap();
        let mut reply = self
            .remote
            .send_request(interface::CONSTRUCT, &mut data)
            .unwrap();
        let ret: i32 = reply.read().unwrap();
        assert_eq!(0, ret);
        reply.read::<i32>().unwrap() as u32
    }

    pub fn start(&self, task_id: u32) {
        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();
        data.write(&format!("{}", task_id)).unwrap();
        let mut reply = self
            .remote
            .send_request(interface::START, &mut data)
            .unwrap();
        let ret: i32 = reply.read().unwrap();
        assert_eq!(ret, 0);
    }

    pub fn pause(&self, task_id: u32) {
        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();
        data.write(&0u32);
        data.write(&format!("{}", task_id)).unwrap();
        let mut reply = self
            .remote
            .send_request(interface::PAUSE, &mut data)
            .unwrap();
        let ret: i32 = reply.read().unwrap();
        assert_eq!(ret, 0);
    }

    pub fn resume(&self, task_id: u32) {
        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();
        data.write(&format!("{}", task_id)).unwrap();
        let mut reply = self
            .remote
            .send_request(interface::RESUME, &mut data)
            .unwrap();
        let ret: i32 = reply.read().unwrap();
        assert_eq!(ret, 0);
    }

    pub fn search(
        &self,
        before: i64,
        after: i64,
        state: State,
        action: Action,
        mode: Mode,
    ) -> Vec<u32> {
        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();
        data.write("com.example.app").unwrap();
        data.write(&before).unwrap();
        data.write(&after).unwrap();
        data.write(&state.repr).unwrap();
        data.write(&action.repr).unwrap();
        data.write(&mode.repr).unwrap();

        let mut reply = self
            .remote
            .send_request(interface::SEARCH, &mut data)
            .unwrap();
        let len = reply.read::<u32>().unwrap();
        let mut ans = vec![];
        for _ in 0..len {
            let id: String = reply.read().unwrap();
            ans.push(id.parse::<u32>().unwrap());
        }
        ans
    }

    pub fn subscribe(&self, task_id: u32) {
        let mut data = MsgParcel::new();
        data.write_interface_token(SERVICE_TOKEN).unwrap();
        data.write(&format!("{}", task_id)).unwrap();
        let mut reply = self
            .remote
            .send_request(interface::SUBSCRIBE, &mut data)
            .unwrap();
        let ret: i32 = reply.read().unwrap();
        assert_eq!(0, ret);
    }

    pub fn pop_task_info(&self, task_id: u32) -> Vec<MessageInfo> {
        self.messages
            .lock()
            .unwrap()
            .remove(&task_id)
            .unwrap_or_default()
    }
}

/// test init
fn remote() -> RemoteObj {
    unsafe { SetAccessTokenPermission() };
    let mut count = 0;
    loop {
        if let Some(download_server) =
            SystemAbilityManager::check_system_ability(DOWNLOAD_SERVICE_ID)
        {
            return download_server;
        }
        SystemAbilityManager::load_system_ability(DOWNLOAD_SERVICE_ID, 15000).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        count += 1;
        println!("load download service {} seconds", count);
    }
}

#[derive(Debug)]
pub enum MessageInfo {
    Http(ResponseInfo),
    Notify(NotifyInfo),
}

impl MessageInfo {
    pub fn is_finished(&self) -> bool {
        match self {
            MessageInfo::Http(info) => false,
            MessageInfo::Notify(info) => {
                info.state == State::Completed || info.state == State::Failed
            }
        }
    }

    pub fn check_correct(&self) {
        match self {
            MessageInfo::Http(info) => {
                if info.status != 200 && info.status != 206 {
                    panic!("http status code is {}", info.status);
                }
            }
            MessageInfo::Notify(info) => {
                assert_ne!(info.state, State::Removed);
                assert_ne!(info.state, State::Failed);
            }
        }
    }
}

#[derive(Debug)]
pub struct NotifyInfo {
    notify_type: SubscribeType,
    state: State,
    index: u32,
    processed: u64,
    total_processed: u64,
    sizes: Vec<u64>,
    extras: HashMap<String, String>,
    action: Action,
    task_states: Vec<TaskState>,
}

#[derive(Debug)]
pub struct ResponseInfo {
    pub version: String,
    pub status: u32,
    pub reason: String,
}

fn deserialize(mut input: &[u8]) -> (u32, MessageInfo) {
    static mut MESSAGE_ID: usize = 1;

    let magic_num: u32 = input.take_value();
    assert_eq!(magic_num, CHANNEL_MAGIC_NUM);

    let message_id: u32 = input.take_value();
    assert_eq!(message_id as usize, unsafe { MESSAGE_ID });

    let msg_type: u16 = input.take_value();
    let body_size: u16 = input.take_value();

    unsafe {
        MESSAGE_ID += 1;
    }

    if msg_type == 0 {
        let task_id = input.take_value();
        let version = input.take_value();
        let status = input.take_value();
        let reason = input.take_value();
        (
            task_id,
            MessageInfo::Http(ResponseInfo {
                version,
                status,
                reason,
            }),
        )
    } else {
        let notify_type = input.take_value();
        let task_id = input.take_value();
        let state = input.take_value();
        let index = input.take_value();
        let processed = input.take_value();
        let total_processed = input.take_value();
        let sizes = input.take_value();
        let extras = input.take_value();
        let action = input.take_value();
        // Currently, it is not necessary to add to NotifyInfo
        let _version: u32 = input.take_value();
        let task_states = input.take_value();
        (
            task_id,
            MessageInfo::Notify(NotifyInfo {
                notify_type,
                state,
                index,
                processed,
                total_processed,
                sizes,
                extras,
                action,
                task_states,
            }),
        )
    }
}

trait Take<T> {
    fn take_value(&mut self) -> T;
}

impl Take<u16> for &[u8] {
    fn take_value(&mut self) -> u16 {
        let (left, right) = self.split_at(std::mem::size_of::<u16>());
        *self = right;
        u16::from_le_bytes(left.try_into().unwrap())
    }
}

impl Take<u32> for &[u8] {
    fn take_value(&mut self) -> u32 {
        let (left, right) = self.split_at(std::mem::size_of::<u32>());
        *self = right;
        u32::from_le_bytes(left.try_into().unwrap())
    }
}

impl Take<u64> for &[u8] {
    fn take_value(&mut self) -> u64 {
        let (left, right) = self.split_at(std::mem::size_of::<u64>());
        *self = right;
        u64::from_le_bytes(left.try_into().unwrap())
    }
}

impl Take<Vec<u64>> for &[u8] {
    fn take_value(&mut self) -> Vec<u64> {
        let length: u32 = self.take_value();
        let mut v = Vec::with_capacity(length as usize);
        for _ in 0..length {
            v.push(self.take_value());
        }
        v
    }
}

impl Take<HashMap<String, String>> for &[u8] {
    fn take_value(&mut self) -> HashMap<String, String> {
        let length: u32 = self.take_value();
        let mut map = HashMap::with_capacity(length as usize);
        for _ in 0..length {
            let key = self.take_value();
            let value = self.take_value();
            map.insert(key, value);
        }
        map
    }
}

impl Take<String> for &[u8] {
    fn take_value(&mut self) -> String {
        let len = self.iter().position(|c| *c == b'\0').unwrap();
        let (left, right) = self.split_at(len + 1);
        *self = right;
        CString::from_vec_with_nul(left.to_vec())
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}

impl Take<SubscribeType> for &[u8] {
    fn take_value(&mut self) -> SubscribeType {
        let value: u32 = self.take_value();
        match value {
            0 => SubscribeType::Completed,
            1 => SubscribeType::Failed,
            2 => SubscribeType::HeaderReceive,
            3 => SubscribeType::Pause,
            4 => SubscribeType::Progress,
            5 => SubscribeType::Remove,
            6 => SubscribeType::Resume,
            7 => SubscribeType::Response,
            8 => SubscribeType::Butt,
            _ => panic!("Invalid SubscribeType value"),
        }
    }
}

impl Take<State> for &[u8] {
    fn take_value(&mut self) -> State {
        let value: u32 = self.take_value();
        State::from(value as u8)
    }
}

impl Take<Action> for &[u8] {
    fn take_value(&mut self) -> Action {
        let value: u32 = self.take_value();
        Action::from(value as u8)
    }
}

impl Take<Vec<TaskState>> for &[u8] {
    fn take_value(&mut self) -> Vec<TaskState> {
        let length: u32 = self.take_value();
        let mut v = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let path = self.take_value();
            let code = self.take_value();
            let message = self.take_value();
            v.push(TaskState {
                path,
                code,
                message,
            });
        }
        v
    }
}

#[derive(Debug)]
pub enum SubscribeType {
    Completed,
    Failed,
    HeaderReceive,
    Pause,
    Progress,
    Remove,
    Resume,
    Response,
    Butt,
}

#[derive(Debug)]
pub struct TaskState {
    path: String,
    // Reason
    code: u32,
    message: String,
}

extern "C" {
    pub fn SetAccessTokenPermission();
}
