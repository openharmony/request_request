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
use std::os::fd::IntoRawFd;

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

#[ani_rs::ani(path = "L@ohos/request/request/agent/Mode")]
pub enum Mode {
    BackGround,
    ForeGround,
}

impl From<Mode> for config::Mode {
    fn from(value: Mode) -> Self {
        match value {
            Mode::BackGround => config::Mode::BackGround,
            Mode::ForeGround => config::Mode::FrontEnd,
        }
    }
}

impl From<config::Mode> for Mode {
    fn from(value: config::Mode) -> Self {
        match value {
            config::Mode::BackGround => Mode::BackGround,
            config::Mode::FrontEnd => Mode::ForeGround,
        }
    }
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/Network")]
pub enum Network {
    ANY,
    WIFI,
    CELLULAR,
}

impl From<Network> for NetworkConfig {
    fn from(value: Network) -> Self {
        match value {
            Network::ANY => NetworkConfig::Any,
            Network::WIFI => NetworkConfig::Wifi,
            Network::CELLULAR => NetworkConfig::Cellular,
        }
    }
}

impl From<NetworkConfig> for Network {
    fn from(value: NetworkConfig) -> Self {
        match value {
            NetworkConfig::Any => Network::ANY,
            NetworkConfig::Wifi => Network::WIFI,
            NetworkConfig::Cellular => Network::CELLULAR,
        }
    }
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/BroadcastEvent")]
pub enum BroadcastEvent {
    COMPLETE,
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
    INITIALIZED = 0x00,
    WAITING = 0x10,
    RUNNING = 0x20,
    RETRYING = 0x21,
    PAUSED = 0x30,
    STOPPED = 0x31,
    COMPLETED = 0x40,
    FAILED = 0x41,
    REMOVED = 0x50,
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/ProgressInner")]
pub struct Progress {
    state: State,
    index: i32,
    processed: i64,
    sizes: Vec<i64>,
    extras: Option<HashMap<String, String>>,
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/Faults")]
pub enum Faults {
    OTHERS = 0xFF,
    DISCONNECTED = 0x00,
    TIMEOUT = 0x10,
    PROTOCOL = 0x20,
    PARAM = 0x30,
    FSIO = 0x40,
    DNS = 0x50,
    TCP = 0x60,
    SSL = 0x70,
    REDIRECT = 0x80,
}

#[ani_rs::ani]
pub struct Filter {
    bundle: Option<String>,
    before: Option<i64>,
    after: Option<i64>,
    state: Option<State>,
    action: Option<Action>,
    mode: Option<Mode>,
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

#[ani_rs::ani]
pub struct HttpResponse {
    version: String,
    status_code: i32,
    reason: String,
    headers: HashMap<String, Vec<String>>,
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/TaskInner")]
pub struct Task {
    pub tid: i64,
}

#[ani_rs::ani]
pub struct GroupConfig {
    gauge: Option<bool>,
    notification: Notification,
}

impl From<Config> for TaskConfig {
    fn from(value: Config) -> Self {
        let file = std::fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .open("/data/test.txt")
            .unwrap();
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
            file_specs: vec![request_core::file::FileSpec {
                name: "".to_string(),
                mime_type: "".to_string(),
                path: "".to_string(),
                file_name: "".to_string(),
                is_user_file: true,
                fd: Some(file.into_raw_fd()),
            }],
            body_file_paths: vec![],
            certs_path: vec![],
            common_data: CommonTaskConfig {
                task_id: 0,
                uid: 0,
                token_id: 0,
                action: value.action.into(),
                mode: value.mode.unwrap_or(Mode::BackGround).into(),
                cover: false,
                network_config: NetworkConfig::Any,
                metered: false,
                roaming: false,
                retry: false,
                redirect: false,
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
