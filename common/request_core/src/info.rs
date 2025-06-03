// Copyright (c) 2023 Huawei Device Co., Ltd.
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

use ipc::parcel::Deserialize;

use crate::config::{Action, FormItem, Mode, Version};
use crate::file::FileSpec;

#[derive(Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum State {
    /// Initialized
    Initialized = 0x00,
    /// Waiting
    Waiting = 0x10,
    /// Running
    Running = 0x20,
    /// Retrying
    Retrying = 0x21,
    /// Paused
    Paused = 0x30,
    /// Stopped
    Stopped = 0x31,
    /// Completed
    Completed = 0x40,
    /// Failed
    Failed = 0x41,
    /// Removed
    Removed = 0x50,
    /// Any
    Any = 0x61,
}

impl From<u32> for State {
    fn from(value: u32) -> Self {
        match value {
            0x00 => State::Initialized,
            0x10 => State::Waiting,
            0x20 => State::Running,
            0x21 => State::Retrying,
            0x30 => State::Paused,
            0x31 => State::Stopped,
            0x40 => State::Completed,
            0x41 => State::Failed,
            0x50 => State::Removed,
            _ => State::Any,
        }
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum SubscribeType {
    Completed = 0,
    Failed,
    HeaderReceive,
    Pause,
    Progress,
    Remove,
    Resume,
    Response,
    Butt,
}

impl From<u32> for SubscribeType {
    fn from(value: u32) -> Self {
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
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct Response {
    pub task_id: String,
    pub version: String,
    pub status_code: i32,
    pub reason: String,
    pub headers: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub struct TaskState {
    pub path: String,
    pub response_code: u32,
    pub message: String,
}

#[derive(Debug)]
pub struct Progress {
    pub state: State,
    pub index: u32,
    pub processed: u64,
    pub total_processed: u64,
    pub sizes: Vec<i64>,
    pub extras: HashMap<String, String>,
}

#[derive(Debug)]
pub struct NotifyData {
    pub subscribe_type: SubscribeType,
    pub task_id: u32,
    pub progress: Progress,

    pub action: Action,
    pub version: Version,
    pub task_states: Vec<TaskState>,
}

#[derive(Clone, Debug)]
pub(crate) struct InfoProgress {
    pub(crate) common_data: CommonProgress,
    /// Total size of the files.
    pub(crate) sizes: Vec<i64>,
    /// Each progress size of the files.
    pub(crate) processed: Vec<usize>,
    pub(crate) extras: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub(crate) struct CommonProgress {
    pub(crate) state: u8,
    pub(crate) index: usize,
    pub(crate) total_processed: usize,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct CommonTaskInfo {
    pub(crate) task_id: u32,
    pub(crate) uid: u64,
    pub(crate) action: u8,
    pub(crate) mode: u8,
    pub(crate) ctime: u64,
    pub(crate) mtime: u64,
    pub(crate) reason: u8,
    pub(crate) gauge: bool,
    pub(crate) retry: bool,
    pub(crate) tries: u32,
    pub(crate) version: u8,
    pub(crate) priority: u32,
}

#[derive(Debug, Clone)]
pub(crate) struct TaskInfo {
    pub(crate) bundle: String,
    pub(crate) url: String,
    pub(crate) data: String,
    pub(crate) token: String,
    pub(crate) form_items: Vec<FormItem>,
    pub(crate) file_specs: Vec<FileSpec>,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) mime_type: String,
    pub(crate) progress: InfoProgress,
    pub(crate) extras: HashMap<String, String>,
    pub(crate) common_data: CommonTaskInfo,
    pub(crate) max_speed: i64,
}

impl Deserialize for TaskInfo {
    fn deserialize(parcel: &mut ipc::parcel::MsgParcel) -> ipc::IpcResult<Self> {
        let gauge = parcel.read::<bool>()?;
        let retry = parcel.read::<bool>()?;
        let action = parcel.read::<u32>()? as u8;
        let mode = parcel.read::<u32>()? as u8;
        let reason = parcel.read::<u32>()? as u8;
        let tries = parcel.read::<u32>()?;
        let uid = parcel.read::<String>()?.parse::<u64>().unwrap_or(0);
        let bundle = parcel.read::<String>()?;
        let url = parcel.read::<String>()?;
        let task_id = parcel.read::<String>()?.parse::<u32>().unwrap_or(0);
        let title = parcel.read::<String>()?;
        let mime_type = parcel.read::<String>()?;
        let ctime = parcel.read::<u64>()?;
        let mtime = parcel.read::<u64>()?;
        let data = parcel.read::<String>()?;
        let description = parcel.read::<String>()?;
        let priority = parcel.read::<u32>()?;

        let form_items_len = parcel.read::<u32>()? as usize;
        let mut form_items = Vec::with_capacity(form_items_len);
        for _ in 0..form_items_len {
            let name = parcel.read::<String>()?;
            let value = parcel.read::<String>()?;
            form_items.push(FormItem { name, value });
        }

        let file_specs_len = parcel.read::<u32>()? as usize;
        let mut file_specs = Vec::with_capacity(file_specs_len);
        for _ in 0..file_specs_len {
            let name = parcel.read::<String>()?;
            let path = parcel.read::<String>()?;
            let file_name = parcel.read::<String>()?;
            let mime_type = parcel.read::<String>()?;
            file_specs.push(FileSpec {
                name,
                path,
                file_name,
                mime_type,
                fd: None,
                is_user_file: false, // Assuming is_user_file is false by default
            });
        }

        let state = parcel.read::<u32>()? as u8;
        let index = parcel.read::<u32>()? as usize;
        let processed = parcel.read::<u64>()? as usize;
        let total_processed = parcel.read::<u64>()? as usize;
        let sizes = parcel.read::<Vec<i64>>()?;
        let extras_len = parcel.read::<u32>()? as usize;
        let mut extras = HashMap::with_capacity(extras_len);
        for _ in 0..extras_len {
            let key = parcel.read::<String>()?;
            let value = parcel.read::<String>()?;
            extras.insert(key, value);
        }

        let version = parcel.read::<u32>()? as u8;

        let each_file_status_len = parcel.read::<u32>()? as usize;
        let mut each_file_status = Vec::with_capacity(each_file_status_len);
        for _ in 0..each_file_status_len {
            let path = parcel.read::<String>()?;
            let response_code = parcel.read::<u32>()?;
            let message = parcel.read::<String>()?;
            each_file_status.push(TaskState {
                path,
                response_code,
                message,
            });
        }
        let common_data = CommonTaskInfo {
            task_id,
            uid,
            action,
            mode,
            ctime,
            mtime,
            reason,
            gauge,
            retry,
            tries,
            version,
            priority,
        };
        let progress = InfoProgress {
            common_data: CommonProgress {
                state,
                index,
                total_processed,
            },
            sizes,
            processed: vec![processed; file_specs.len()],
            extras,
        };
        Ok(TaskInfo {
            bundle,
            url,
            data,
            token: String::new(), // Token is not serialized in this context
            form_items,
            file_specs,
            title,
            description,
            mime_type,
            progress,
            extras: HashMap::new(), // Extras are not serialized in this context
            common_data,
            max_speed: 0, // Max speed is not serialized in this context
        })
    }
}
