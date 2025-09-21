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
    FaultOccur,
    Wait,
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
            8 => SubscribeType::FaultOccur,
            9 => SubscribeType::Wait,
            10 => SubscribeType::Butt,
            _ => unimplemented!(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Faults {
    Others = 0xFF,
    Disconnected = 0x00,
    Timeout = 0x10,
    Protocol = 0x20,
    Param = 0x30,
    Fsio = 0x40,
    Dns = 0x50,
    Tcp = 0x60,
    Ssl = 0x70,
    Redirect = 0x80,
}

impl From<u32> for Faults {
    fn from(value: u32) -> Self {
        match value {
            0xFF => Faults::Others,
            0x00 => Faults::Disconnected,
            0x10 => Faults::Timeout,
            0x20 => Faults::Protocol,
            0x30 => Faults::Param,
            0x40 => Faults::Fsio,
            0x50 => Faults::Dns,
            0x60 => Faults::Tcp,
            0x70 => Faults::Ssl,
            0x80 => Faults::Redirect,
            _ => unimplemented!(),
        }
    }
}

impl From<Reason> for Faults {
    fn from(reason: Reason) -> Self {
        match reason {
            Reason::NetworkOffline | Reason::NetworkApp | Reason::NetworkAccount
            | Reason::NetworkAppAccount => Faults::Disconnected,
            Reason::BuildClientFailed | Reason::BuildRequestFailed => Faults::Param,
            Reason::GetFilesizeFailed | Reason::IoError => Faults::Fsio,
            Reason::ContinuousTaskTimeout => Faults::Timeout,
            Reason::ConnectError => Faults::Tcp,
            Reason::RequestError | Reason::ProtocolError | Reason::UnsupportRangeRequest => Faults::Protocol,
            Reason::RedirectError => Faults::Redirect,
            Reason::DNS => Faults::Dns,
            Reason::TCP => Faults::Tcp,
            Reason::SSL => Faults::Ssl,
            _ => Faults::Others,
        }
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Reason {
    ReasonOk = 0,
    TaskSurvivalOneMonth,
    WaittingNetworkOneDay,
    StoppedNewFrontTask,
    RunningTaskMeetLimits,
    UserOperation,
    AppBackgroundOrTerminate,
    NetworkOffline,
    UnsupportedNetworkType,
    BuildClientFailed,
    BuildRequestFailed,
    GetFilesizeFailed,
    ContinuousTaskTimeout,
    ConnectError,
    RequestError,
    UploadFileError,
    RedirectError,
    ProtocolError,
    IoError,
    UnsupportRangeRequest,
    OthersError,
    AccountStopped,
    NetworkChanged,
    DNS,
    TCP,
    SSL,
    InsufficientSpace,
    NetworkApp,
    NetworkAccount,
    AppAccount,
    NetworkAppAccount,
    LowSpeed,
}

impl From<u32> for Reason {
    fn from(value: u32) -> Self {
        match value {
            0 => Reason::ReasonOk,
            1 => Reason::TaskSurvivalOneMonth,
            2 => Reason::WaittingNetworkOneDay,
            3 => Reason::StoppedNewFrontTask,
            4 => Reason::RunningTaskMeetLimits,
            5 => Reason::UserOperation,
            6 => Reason::AppBackgroundOrTerminate,
            7 => Reason::NetworkOffline,
            8 => Reason::UnsupportedNetworkType,
            9 => Reason::BuildClientFailed,
            10 => Reason::BuildRequestFailed,
            11 => Reason::GetFilesizeFailed,
            12 => Reason::ContinuousTaskTimeout,
            13 => Reason::ConnectError,
            14 => Reason::RequestError,
            15 => Reason::UploadFileError,
            16 => Reason::RedirectError,
            17 => Reason::ProtocolError,
            18 => Reason::IoError,
            19 => Reason::UnsupportRangeRequest,
            20 => Reason::OthersError,
            21 => Reason::AccountStopped,
            22 => Reason::NetworkChanged,
            23 => Reason::DNS,
            24 => Reason::TCP,
            25 => Reason::SSL,
            26 => Reason::InsufficientSpace,
            27 => Reason::NetworkApp,
            28 => Reason::NetworkAccount,
            29 => Reason::AppAccount,
            30 => Reason::NetworkAppAccount,
            31 => Reason::LowSpeed,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct FaultOccur {
    pub task_id: i32,
    pub subscribe_type: SubscribeType,
    pub faults: Faults,
}

#[derive(Debug)]
pub struct Response {
    pub task_id: String,
    pub version: String,
    pub status_code: i32,
    pub reason: String,
    pub headers: HashMap<String, Vec<String>>,
}

#[derive(Clone)]
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
    // pub body_bytes: Vec<u8>,
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
pub struct InfoProgress {
    pub common_data: CommonProgress,
    /// Total size of the files.
    pub sizes: Vec<i64>,
    /// Each progress size of the files.
    pub processed: Vec<usize>,
    pub extras: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct CommonProgress {
    pub state: u8,
    pub index: usize,
    pub total_processed: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct CommonTaskInfo {
    pub task_id: u32,
    pub uid: u64,
    pub action: u8,
    pub mode: u8,
    pub ctime: u64,
    pub mtime: u64,
    pub reason: u8,
    pub gauge: bool,
    pub retry: bool,
    pub tries: u32,
    pub version: u8,
    pub priority: u32,
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub bundle: String,
    pub url: String,
    pub data: String,
    pub token: String,
    pub form_items: Vec<FormItem>,
    pub file_specs: Vec<FileSpec>,
    pub title: String,
    pub description: String,
    pub mime_type: String,
    pub progress: InfoProgress,
    pub extras: HashMap<String, String>,
    pub common_data: CommonTaskInfo,
    pub max_speed: i64,
}

impl Deserialize for TaskInfo {
    fn deserialize(parcel: &mut ipc::parcel::MsgParcel) -> ipc::IpcResult<Self> {
        let gauge = parcel.read::<bool>().unwrap();
        let retry = parcel.read::<bool>().unwrap();
        let action = parcel.read::<u32>().unwrap() as u8;
        let mode = parcel.read::<u32>().unwrap() as u8;
        let reason = parcel.read::<u32>().unwrap() as u8;
        let tries = parcel.read::<u32>().unwrap();
        let uid = parcel.read::<String>().unwrap().parse::<u64>().unwrap_or(0);
        let bundle = parcel.read::<String>().unwrap();
        let url = parcel.read::<String>().unwrap();
        let task_id = parcel.read::<String>().unwrap().parse::<u32>().unwrap_or(0);
        let title = parcel.read::<String>().unwrap();
        let mime_type = parcel.read::<String>().unwrap();
        let ctime = parcel.read::<u64>().unwrap();
        let mtime = parcel.read::<u64>().unwrap();
        let data = parcel.read::<String>().unwrap();
        let description = parcel.read::<String>().unwrap();
        let priority = parcel.read::<u32>().unwrap();
        let form_items_len = parcel.read::<u32>().unwrap() as usize;
        let mut form_items = Vec::with_capacity(form_items_len);
        for _ in 0..form_items_len {
            let name = parcel.read::<String>().unwrap();
            let value = parcel.read::<String>().unwrap();
            form_items.push(FormItem { name, value });
        }

        let file_specs_len = parcel.read::<u32>().unwrap() as usize;
        let mut file_specs = Vec::with_capacity(file_specs_len);
        for _ in 0..file_specs_len {
            let name = parcel.read::<String>().unwrap();
            let path = parcel.read::<String>().unwrap();
            let file_name = parcel.read::<String>().unwrap();
            let mime_type = parcel.read::<String>().unwrap();
            file_specs.push(FileSpec {
                name,
                path,
                file_name,
                mime_type,
                fd: None,
                is_user_file: false, // Assuming is_user_file is false by default
            });
        }

        let state = parcel.read::<u32>().unwrap() as u8;
        let index = parcel.read::<u32>().unwrap() as usize;
        let processed = parcel.read::<u64>().unwrap() as usize;
        let total_processed = parcel.read::<u64>().unwrap() as usize;
        let sizes = parcel.read::<Vec<i64>>().unwrap();

        let extras_len = parcel.read::<u32>().unwrap() as usize;
        let mut progress_extras = HashMap::with_capacity(extras_len);
        for _ in 0..extras_len {
            let key = parcel.read::<String>().unwrap();
            let value = parcel.read::<String>().unwrap();
            progress_extras.insert(key, value);
        }

        let extras_len = parcel.read::<u32>().unwrap() as usize;
        let mut extras = HashMap::with_capacity(extras_len);
        for _ in 0..extras_len {
            let key = parcel.read::<String>().unwrap();
            let value = parcel.read::<String>().unwrap();
            extras.insert(key, value);
        }

        let version = parcel.read::<u32>().unwrap() as u8;

        let each_file_status_len = parcel.read::<u32>().unwrap() as usize;
        let mut task_states = Vec::with_capacity(each_file_status_len);
        for _ in 0..each_file_status_len {
            let path = parcel.read::<String>().unwrap();
            let reason = parcel.read::<u32>().unwrap() as u8;
            let message = parcel.read::<String>().unwrap();
            task_states.push(TaskState {
                path,
                response_code: reason as u32,
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
            extras: progress_extras,
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
            extras, // Extras are not serialized in this context
            common_data,
            max_speed: 0, // Max speed is not serialized in this context
        })
    }
}
