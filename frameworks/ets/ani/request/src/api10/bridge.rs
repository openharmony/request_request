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

use request_core::config::{self, CommonTaskConfig, NetworkConfig, TaskConfig, Version};
use serde::{Deserialize, Serialize};

#[ani_rs::ani(path = "L@ohos/request/request/agent/Action")]
pub enum Action {
    Download,
    Upload,
}

impl From<Action> for request_core::config::Action {
    fn from(value: Action) -> Self {
        match value {
            Action::Download => config::Action::Download,
            Action::Upload => config::Action::Upload,
        }
    }
}

impl From<config::Action> for Action {
    fn from(value: config::Action) -> Self {
        match value {
            config::Action::Download => Action::Download,
            config::Action::Upload => Action::Upload,
        }
    }
}

impl From<u8> for Action {
    fn from(value: u8) -> Self {
        match value {
            0 => Action::Download,
            1 => Action::Upload,
            _ => unimplemented!(),
        }
    }
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/Mode")]
pub enum Mode {
    Background,
    Foreground,
}

impl From<Mode> for config::Mode {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Background => config::Mode::BackGround,
            Mode::Foreground => config::Mode::FrontEnd,
        }
    }
}

impl From<config::Mode> for Mode {
    fn from(value: config::Mode) -> Self {
        match value {
            config::Mode::BackGround => Mode::Background,
            config::Mode::FrontEnd => Mode::Foreground,
        }
    }
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        match value {
            0 => Mode::Background,
            1 => Mode::Foreground,
            _ => unimplemented!(),
        }
    }
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/Network")]
pub enum Network {
    Any,
    Wifi,
    Cellular,
}

impl From<Network> for NetworkConfig {
    fn from(value: Network) -> Self {
        match value {
            Network::Any => NetworkConfig::Any,
            Network::Wifi => NetworkConfig::Wifi,
            Network::Cellular => NetworkConfig::Cellular,
        }
    }
}

impl From<NetworkConfig> for Network {
    fn from(value: NetworkConfig) -> Self {
        match value {
            NetworkConfig::Any => Network::Any,
            NetworkConfig::Wifi => Network::Wifi,
            NetworkConfig::Cellular => Network::Cellular,
        }
    }
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/BroadcastEvent")]
pub enum BroadcastEvent {
    Complete,
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/FileSpecInner")]
pub struct FileSpec {
    path: String,
    content_type: Option<String>,
    filename: Option<String>,
    extras: Option<HashMap<String, String>>,
}

impl From<FileSpec> for request_core::file::FileSpec {
    fn from(value: FileSpec) -> Self {
        request_core::file::FileSpec {
            name: "".to_string(),
            path: value.path,
            mime_type: value.content_type.unwrap_or("".to_string()),
            file_name: value.filename.unwrap_or("".to_string()),
            is_user_file: false,
            fd: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Value {
    S(String),
    #[serde(rename = "L@ohos/request/request/agent/FileSpecInner;")]
    FileSpec(FileSpec),
    Array(Vec<FileSpec>),
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/FormItemInner")]
pub struct FormItem {
    name: String,
    value: Value,
}

#[ani_rs::ani]
pub struct Notification {
    title: Option<String>,
    text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Data {
    S(String),
    Array(Vec<FormItem>),
}

#[ani_rs::ani]
pub struct Config {
    pub action: Action,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub mode: Option<Mode>,
    pub overwrite: Option<bool>,
    pub method: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub data: Option<Data>,
    pub saveas: Option<String>,
    pub network: Option<Network>,
    pub metered: Option<bool>,
    pub roaming: Option<bool>,
    pub retry: Option<bool>,
    pub redirect: Option<bool>,
    pub proxy: Option<String>,
    pub index: Option<i32>,
    pub begins: Option<i64>,
    pub ends: Option<i64>,
    pub gauge: Option<bool>,
    pub precise: Option<bool>,
    pub token: Option<String>,
    pub priority: Option<i32>,
    pub extras: Option<HashMap<String, String>>,
    pub multipart: Option<bool>,
    pub notification: Option<Notification>,
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/State")]
pub enum State {
    Initialized = 0x00,
    Waiting = 0x10,
    Running = 0x20,
    Retrying = 0x21,
    Paused = 0x30,
    Stopped = 0x31,
    Completed = 0x40,
    Failed = 0x41,
    Removed = 0x50,
}

impl From<request_core::info::State> for State {
    fn from(value: request_core::info::State) -> Self {
        match value {
            request_core::info::State::Initialized => State::Initialized,
            request_core::info::State::Waiting => State::Waiting,
            request_core::info::State::Running => State::Running,
            request_core::info::State::Retrying => State::Retrying,
            request_core::info::State::Paused => State::Paused,
            request_core::info::State::Stopped => State::Stopped,
            request_core::info::State::Completed => State::Completed,
            request_core::info::State::Failed => State::Failed,
            request_core::info::State::Removed => State::Removed,
            _ => unimplemented!(),
        }
    }
}

impl From<State> for request_core::info::State {
    fn from(value: State) -> Self {
        match value {
            State::Initialized => request_core::info::State::Initialized,
            State::Waiting => request_core::info::State::Waiting,
            State::Running => request_core::info::State::Running,
            State::Retrying => request_core::info::State::Retrying,
            State::Paused => request_core::info::State::Paused,
            State::Stopped => request_core::info::State::Stopped,
            State::Completed => request_core::info::State::Completed,
            State::Failed => request_core::info::State::Failed,
            State::Removed => request_core::info::State::Removed,
        }
    }
}

impl From<u8> for State {
    fn from(value: u8) -> Self {
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
            _ => unimplemented!(),
        }
    }
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/ProgressInner")]
pub struct Progress {
    state: State,
    index: i32,
    processed: i64,
    sizes: Vec<i64>,
    extras: Option<HashMap<String, String>>,
}

impl From<&request_core::info::Progress> for Progress {
    fn from(value: &request_core::info::Progress) -> Self {
        Progress {
            state: value.state.clone().into(),
            index: value.index as i32,
            processed: value.total_processed as i64,
            sizes: value.sizes.clone(),
            extras: None,
        }
    }
}

impl From<&request_core::info::InfoProgress> for Progress {
    fn from(value: &request_core::info::InfoProgress) -> Self {
        Progress {
            state: value.common_data.state.into(),
            index: value.common_data.index as i32,
            processed: value.common_data.total_processed as i64,
            sizes: value.sizes.clone(),
            extras: None,
        }
    }
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/Faults")]
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

#[ani_rs::ani]
pub struct Filter {
    pub bundle: Option<String>,
    pub before: Option<i64>,
    pub after: Option<i64>,
    pub state: Option<State>,
    pub action: Option<Action>,
    pub mode: Option<Mode>,
}

impl From<Filter> for request_core::filter::SearchFilter {
    fn from(value: Filter) -> Self {
        request_core::filter::SearchFilter {
            bundle_name: value.bundle,
            before: value.before,
            after: value.after,
            state: value.state.map(|s| s.into()),
            action: value.action.map(|a| a.into()),
            mode: value.mode.map(|m| m.into()),
        }
    }
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/TaskInfoInner")]
pub struct TaskInfo {
    pub uid: Option<String>,
    pub bundle: Option<String>,
    pub saveas: Option<String>,
    pub url: Option<String>,
    pub data: Option<Data>,
    pub tid: String,
    pub title: String,
    pub description: String,
    pub action: Action,
    pub mode: Mode,
    pub priority: i32,
    pub mime_type: String,
    pub progress: Progress,
    pub gauge: bool,
    pub ctime: i64,
    pub mtime: i64,
    pub retry: bool,
    pub tries: i32,
    pub faults: Faults,
    pub reason: String,
    pub extras: Option<HashMap<String, String>>,
}

impl From<request_core::info::TaskInfo> for TaskInfo {
    fn from(value: request_core::info::TaskInfo) -> Self {
        TaskInfo {
            uid: Some(value.common_data.uid.to_string()),
            bundle: Some(value.bundle),
            saveas: None,
            url: Some(value.url),
            data: None,
            tid: value.common_data.task_id.to_string(),
            title: value.title,
            description: value.description,
            action: value.common_data.action.into(),
            mode: value.common_data.mode.into(),
            priority: value.common_data.priority as i32,
            mime_type: value.mime_type,
            progress: Progress::from(&value.progress),
            gauge: value.common_data.gauge,
            ctime: value.common_data.ctime as i64,
            mtime: value.common_data.mtime as i64,
            retry: value.common_data.retry,
            tries: value.common_data.tries as i32,
            faults: Faults::Others,
            reason: value.common_data.reason.to_string(),
            extras: Some(value.extras.clone()),
        }
    }
}

#[ani_rs::ani]
pub struct HttpResponse {
    version: String,
    status_code: i32,
    reason: String,
    headers: HashMap<String, Vec<String>>,
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/TaskInner")]
pub struct Task {
    pub tid: String,
}

#[ani_rs::ani]
pub struct GroupConfig {
    pub gauge: Option<bool>,
    pub notification: Notification,
}

impl From<Config> for TaskConfig {
    fn from(value: Config) -> Self {
        TaskConfig {
            bundle: "".to_string(),
            bundle_type: 0,
            atomic_account: "".to_string(),
            url: value.url,
            title: value.title.unwrap_or("".to_string()),
            description: value.description.unwrap_or_default(),
            method: value.method.unwrap_or("GET".to_string()),
            headers: value.headers.unwrap_or_default(),
            data: "".to_string(),
            token: "".to_string(),
            proxy: "".to_string(),
            certificate_pins: "".to_string(),
            extras: HashMap::new(),
            version: Version::API9,
            form_items: vec![],
            file_specs: vec![],
            body_file_paths: vec![],
            certs_path: vec![],
            common_data: CommonTaskConfig {
                task_id: 0,
                uid: 0,
                token_id: 0,
                action: value.action.into(),
                mode: value.mode.unwrap_or(Mode::Background).into(),
                cover: false,
                network_config: NetworkConfig::Any,
                metered: false,
                roaming: false,
                retry: false,
                redirect: true,
                index: 0,
                begins: 0,
                ends: -1,
                gauge: false,
                precise: false,
                priority: 0,
                background: false,
                multipart: false,
            },
        }
    }
}
