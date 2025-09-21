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

use std::collections::HashMap;
use std::io::Read;

use request_core::config::{Action, Version};
use request_core::info::{NotifyData, Progress, Response, State, SubscribeType, TaskState, Faults, Reason, FaultOccur};

pub struct UdsSer<'a> {
    inner: &'a [u8],
}

impl UdsSer<'_> {
    pub fn new(inner: &[u8]) -> UdsSer {
        UdsSer { inner }
    }

    pub fn read<S: Serialize>(&mut self) -> S {
        S::read(self)
    }
}

pub trait Serialize {
    fn read(ser: &mut UdsSer) -> Self;
}

impl Serialize for i64 {
    fn read(ser: &mut UdsSer) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&ser.inner[..8]);
        ser.inner = &ser.inner[8..];
        i64::from_ne_bytes(bytes)
    }
}

impl Serialize for u64 {
    fn read(ser: &mut UdsSer) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&ser.inner[..8]);
        ser.inner = &ser.inner[8..];
        u64::from_ne_bytes(bytes)
    }
}

impl Serialize for i32 {
    fn read(ser: &mut UdsSer) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&ser.inner[..4]);
        ser.inner = &ser.inner[4..];
        i32::from_ne_bytes(bytes)
    }
}

impl Serialize for u32 {
    fn read(ser: &mut UdsSer) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&ser.inner[..4]);
        ser.inner = &ser.inner[4..];
        u32::from_ne_bytes(bytes)
    }
}

impl Serialize for i16 {
    fn read(ser: &mut UdsSer) -> Self {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&ser.inner[..2]);
        ser.inner = &ser.inner[2..];
        i16::from_ne_bytes(bytes)
    }
}

impl Serialize for State {
    fn read(ser: &mut UdsSer) -> Self {
        let state: u32 = ser.read();
        State::from(state)
    }
}

impl Serialize for Action {
    fn read(ser: &mut UdsSer) -> Self {
        let action: u32 = ser.read();
        Action::from(action)
    }
}

impl Serialize for Version {
    fn read(ser: &mut UdsSer) -> Self {
        let version: u32 = ser.read();
        Version::from(version)
    }
}

impl Serialize for SubscribeType {
    fn read(ser: &mut UdsSer) -> Self {
        let subscribe_type: u32 = ser.read();
        SubscribeType::from(subscribe_type)
    }
}

impl Serialize for FaultOccur {
    fn read(ser: &mut UdsSer) -> Self {
        // let task_id = ser.read::<i32>() as i64;
        let task_id = ser.read::<i32>();
        let subscribe_type = ser.read::<SubscribeType>();
        let faults: Faults = ser.read::<Reason>().into();
        FaultOccur {
            task_id,
            subscribe_type,
            faults,
        }
    }
}

impl Serialize for Reason {
    fn read(ser: &mut UdsSer) -> Self {
        let reason: u32 = ser.read();
        Reason::from(reason)
    }
}

impl Serialize for String {
    fn read(ser: &mut UdsSer) -> Self {
        if let Some(s) = ser.inner.split(|a| *a == b'\0').next() {
            ser.inner = &ser.inner[s.len() + 1..];
            String::from_utf8_lossy(s).to_string()
        } else {
            String::new()
        }
    }
}

impl Serialize for HashMap<String, Vec<String>> {
    fn read(ser: &mut UdsSer) -> Self {
        let mut map = HashMap::new();
        let mut s = String::new();
        let _ = ser.inner.read_to_string(&mut s);
        info!("headers {}", s);
        for line in s.lines() {
            let Some(index) = line.find(':') else {
                map.insert(line.to_string(), vec![]);
                continue;
            };
            let (key, value) = line.split_at(index);
            let value = &value[1..];
            let value: Vec<String> = value.split(',').map(String::from).collect();
            map.insert(key.to_string(), value);
        }
        map
    }
}

impl Serialize for HashMap<String, String> {
    fn read(ser: &mut UdsSer) -> Self {
        let mut map = HashMap::new();
        let length: u32 = ser.read();

        for _ in 0..length {
            let key = ser.read::<String>();
            let value = ser.read::<String>();
            map.insert(key, value);
        }
        map
    }
}

impl Serialize for Vec<i64> {
    fn read(ser: &mut UdsSer) -> Self {
        let length: u32 = ser.read();

        let mut vec = Vec::with_capacity(length as usize);
        for _ in 0..length {
            vec.push(ser.read());
        }
        vec
    }
}

impl Serialize for Response {
    fn read(ser: &mut UdsSer) -> Self {
        let task_id = ser.read::<i32>();
        let version = ser.read::<String>();
        let status_code: i32 = ser.read();

        let reason = ser.read::<String>();
        let headers: HashMap<String, Vec<String>> = ser.read();

        info!("headers {:?}", headers);

        Response {
            task_id: task_id.to_string(),
            version,
            status_code,
            reason,
            headers,
        }
    }
}

impl Serialize for Vec<TaskState> {
    fn read(ser: &mut UdsSer) -> Self {
        let length: u32 = ser.read();
        let mut vec = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let path = ser.read::<String>();

            let response_code: u32 = ser.read();
            let message = ser.read::<String>();

            vec.push(TaskState {
                path,
                response_code,
                message,
            });
        }
        vec
    }
}

impl Serialize for Progress {
    fn read(ser: &mut UdsSer) -> Self {
        let state: State = ser.read();
        let index: u32 = ser.read();
        let processed: u64 = ser.read();
        let total_processed: u64 = ser.read();
        let sizes: Vec<i64> = ser.read();
        let extras: HashMap<String, String> = ser.read();
        // let body_bytes: Vec<u8> = ser.read();

        Progress {
            state,
            index,
            processed,
            total_processed,
            sizes,
            extras,
            // body_bytes,
        }
    }
}

impl Serialize for NotifyData {
    fn read(ser: &mut UdsSer) -> Self {
        let subscribe_type: SubscribeType = ser.read();
        let task_id: u32 = ser.read();

        let progress: Progress = ser.read();

        let action: Action = ser.read();
        let version: Version = ser.read();

        let task_states = ser.read::<Vec<TaskState>>();

        NotifyData {
            subscribe_type,
            task_id,
            progress,
            action,
            version,
            task_states,
        }
    }
}
