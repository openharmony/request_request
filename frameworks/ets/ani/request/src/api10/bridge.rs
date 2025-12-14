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

//! Bridge module for API 10.
//!
//! This module provides bridge types and conversion utilities between the ETS interface
//! and the request core functionality for API 10.

use std::collections::HashMap;

use request_core::config::{self, CommonTaskConfig, NetworkConfig, TaskConfig, Version, MinSpeed, Timeout};
use serde::{Deserialize, Serialize};

/// Defines the type of action for a request task.
#[derive(Clone)]
#[ani_rs::ani(path = "L@ohos/request/request/agent/Action")]
pub enum Action {
    /// Download action type.
    Download,
    /// Upload action type.
    Upload,
}

/// Converts from API Action to core Action.
impl From<Action> for request_core::config::Action {
    fn from(value: Action) -> Self {
        match value {
            Action::Download => config::Action::Download,
            Action::Upload => config::Action::Upload,
        }
    }
}

/// Converts from core Action to API Action.
impl From<config::Action> for Action {
    fn from(value: config::Action) -> Self {
        match value {
            config::Action::Download => Action::Download,
            config::Action::Upload => Action::Upload,
        }
    }
}

/// Converts from u8 to Action (0 for Download, 1 for Upload).
impl From<u8> for Action {
    fn from(value: u8) -> Self {
        match value {
            0 => Action::Download,
            1 => Action::Upload,
            _ => unimplemented!(),
        }
    }
}

/// Defines the execution mode for a request task.
#[derive(Clone)]
#[ani_rs::ani(path = "L@ohos/request/request/agent/Mode")]
pub enum Mode {
    /// Background execution mode.
    Background,
    /// Foreground execution mode.
    Foreground,
}

/// Converts from API Mode to core Mode.
impl From<Mode> for config::Mode {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Background => config::Mode::BackGround,
            Mode::Foreground => config::Mode::FrontEnd,
        }
    }
}

/// Converts from core Mode to API Mode.
impl From<config::Mode> for Mode {
    fn from(value: config::Mode) -> Self {
        match value {
            config::Mode::BackGround => Mode::Background,
            config::Mode::FrontEnd => Mode::Foreground,
        }
    }
}

/// Converts from u8 to Mode (0 for Background, 1 for Foreground).
impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        match value {
            0 => Mode::Background,
            1 => Mode::Foreground,
            _ => unimplemented!(),
        }
    }
}

/// Defines network preferences for a request task.
#[derive(Clone)]
#[ani_rs::ani(path = "L@ohos/request/request/agent/Network")]
pub enum Network {
    /// Any network type is acceptable.
    Any,
    /// Only WiFi networks are allowed.
    Wifi,
    /// Only cellular networks are allowed.
    Cellular,
}

/// Converts from API Network to core NetworkConfig.
impl From<Network> for NetworkConfig {
    fn from(value: Network) -> Self {
        match value {
            Network::Any => NetworkConfig::Any,
            Network::Wifi => NetworkConfig::Wifi,
            Network::Cellular => NetworkConfig::Cellular,
        }
    }
}

/// Converts from core NetworkConfig to API Network.
impl From<NetworkConfig> for Network {
    fn from(value: NetworkConfig) -> Self {
        match value {
            NetworkConfig::Any => Network::Any,
            NetworkConfig::Wifi => Network::Wifi,
            NetworkConfig::Cellular => Network::Cellular,
        }
    }
}

/// Defines broadcast event types for request tasks.
#[ani_rs::ani(path = "L@ohos/request/request/agent/BroadcastEvent")]
pub enum BroadcastEvent {
    /// Event emitted when a task completes.
    Complete,
}

/// Represents file specifications for upload or download operations.
#[derive(Clone)]
#[ani_rs::ani(path = "L@ohos/request/request/agent/FileSpecInner")]
pub struct FileSpec {
    /// Path to the file.
    path: String,
    /// Optional content type of the file.
    content_type: Option<String>,
    /// Optional filename for the file.
    filename: Option<String>,
    /// Optional extra parameters associated with the file.
    extras: Option<HashMap<String, String>>,
}

/// Converts from API FileSpec to core FileSpec.
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

/// Represents different value types for form data.
#[derive(Serialize, Deserialize, Clone)]
pub enum Value {
    /// String value type.
    S(String),
    /// File specification type.
    #[serde(rename = "L@ohos/request/request/agent/FileSpec;")]
    FileSpec(FileSpec),
    /// Array of file specifications.
    Array(Vec<FileSpec>),
}

/// Represents an item in a form for data submission.
#[derive(Clone)]
#[ani_rs::ani(path = "L@ohos/request/request/agent/FormItemInner")]
pub struct FormItem {
    /// Name of the form item.
    name: String,
    /// Value of the form item.
    value: Value,
}

/// Represents notification details for a request task.
#[derive(Clone)]
#[ani_rs::ani(path = "L@ohos/request/request/agent/NotificationInner")]
pub struct Notification {
    /// Optional title for the notification.
    pub title: Option<String>,
    /// Optional text content for the notification.
    pub text: Option<String>,
    // pub disable: Option<bool>,
}

/// Represents different data types for request body content.
impl From<Notification> for request_core::config::Notification {
    fn from(value: Notification) -> Self {
        request_core::config::Notification {
            title: value.title,
            text: value.text,
        }
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub enum Data {
    /// String data type.
    S(String),
    /// Array of form items.
    Array(Vec<FormItem>),
}

/// Represents configuration for a request task.
#[derive(Clone)]
#[ani_rs::ani(path = "L@ohos/request/request/agent/ConfigInner")]
pub struct Config {
    /// Action type (download or upload).
    pub action: Action,
    /// URL to send the request to.
    pub url: String,
    /// Optional title for the task.
    pub title: Option<String>,
    /// Optional description for the task.
    pub description: Option<String>,
    /// Optional execution mode.
    pub mode: Option<Mode>,
    /// Optional flag to overwrite existing files.
    pub overwrite: Option<bool>,
    /// Optional HTTP method.
    pub method: Option<String>,
    /// Optional HTTP headers.
    pub headers: Option<HashMap<String, String>>,
    /// Optional request body data.
    pub data: Option<Data>,
    /// Optional save path for downloaded files.
    pub saveas: Option<String>,
    /// Optional network preference.
    pub network: Option<Network>,
    /// Optional flag for metered network usage.
    pub metered: Option<bool>,
    /// Optional flag for roaming network usage.
    pub roaming: Option<bool>,
    /// Optional retry flag.
    pub retry: Option<bool>,
    /// Optional redirect handling flag.
    pub redirect: Option<bool>,
    /// Optional proxy configuration.
    pub proxy: Option<String>,
    /// Optional index for the task.
    pub index: Option<i32>,
    /// Optional beginning range for resumable downloads.
    pub begins: Option<i64>,
    /// Optional ending range for resumable downloads.
    pub ends: Option<i64>,
    /// Optional gauge flag.
    pub gauge: Option<bool>,
    /// Optional precise flag.
    pub precise: Option<bool>,
    /// Optional authentication token.
    pub token: Option<String>,
    /// Optional priority level.
    pub priority: Option<i32>,
    /// Optional extra parameters.
    pub extras: Option<HashMap<String, String>>,
    /// Optional multipart flag.
    pub multipart: Option<bool>,
    /// Optional notification details.
    pub notification: Option<Notification>,
}

/// Represents the state of a request task.
#[ani_rs::ani(path = "L@ohos/request/request/agent/State")]
pub enum State {
    /// Task is initialized but not yet started.
    Initialized = 0x00,
    /// Task is waiting to be processed.
    Waiting = 0x10,
    /// Task is currently running.
    Running = 0x20,
    /// Task is retrying after a failure.
    Retrying = 0x21,
    /// Task is paused.
    Paused = 0x30,
    /// Task has been stopped.
    Stopped = 0x31,
    /// Task has completed successfully.
    Completed = 0x40,
    /// Task has failed.
    Failed = 0x41,
    /// Task has been removed.
    Removed = 0x50,
}

/// Converts from core State to API State.
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

/// Converts from API State to core State.
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

/// Converts from u8 to State based on predefined state codes.
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

/// Represents progress information for a request task.
#[ani_rs::ani(path = "L@ohos/request/request/agent/ProgressInner")]
pub struct Progress {
    /// Current state of the task.
    state: State,
    /// Index of the current part being processed (for multi-part tasks).
    index: i32,
    /// Total bytes processed.
    processed: i64,
    /// Sizes of individual parts.
    sizes: Vec<i64>,
    /// Optional extra progress information.
    extras: Option<HashMap<String, String>>,
}

/// Converts from core Progress to API Progress.
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

/// Converts from core InfoProgress to API Progress.
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

/// Represents error types for request tasks.
#[ani_rs::ani(path = "L@ohos/request/request/agent/Faults")]
pub enum Faults {
    /// Other or unspecified error.
    Others = 0xFF,
    /// Connection disconnected error.
    Disconnected = 0x00,
    /// Request timeout error.
    Timeout = 0x10,
    /// Protocol error.
    Protocol = 0x20,
    /// Parameter error.
    Param = 0x30,
    /// File system I/O error.
    Fsio = 0x40,
    /// DNS resolution error.
    Dns = 0x50,
    /// TCP connection error.
    Tcp = 0x60,
    /// SSL/TLS error.
    Ssl = 0x70,
    /// Redirect handling error.
    Redirect = 0x80,
}

impl From<request_core::info::Faults> for Faults {
    fn from(value: request_core::info::Faults) -> Self {
        match value {
            request_core::info::Faults::Others => Faults::Others,
            request_core::info::Faults::Disconnected => Faults::Disconnected,
            request_core::info::Faults::Timeout => Faults::Timeout,
            request_core::info::Faults::Protocol => Faults::Protocol,
            request_core::info::Faults::Param => Faults::Param,
            request_core::info::Faults::Fsio => Faults::Fsio,
            request_core::info::Faults::Dns => Faults::Dns,
            request_core::info::Faults::Tcp => Faults::Tcp,
            request_core::info::Faults::Ssl => Faults::Ssl,
            request_core::info::Faults::Redirect => Faults::Redirect,
            _ => unimplemented!(),
        }
    }
}

#[ani_rs::ani(path = "L@ohos/request/request/agent/FilterInner")]
pub struct Filter {
    /// Optional bundle name filter.
    pub bundle: Option<String>,
    /// Optional upper time limit (tasks created before this time).
    pub before: Option<i64>,
    /// Optional lower time limit (tasks created after this time).
    pub after: Option<i64>,
    /// Optional state filter.
    pub state: Option<State>,
    /// Optional action type filter.
    pub action: Option<Action>,
    /// Optional mode filter.
    pub mode: Option<Mode>,
}

/// Converts from API Filter to core SearchFilter.
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

/// Represents detailed information about a request task.
#[ani_rs::ani(path = "L@ohos/request/request/agent/TaskInfoInner")]
pub struct TaskInfo {
    /// Optional user ID.
    pub uid: Option<String>,
    /// Optional bundle name.
    pub bundle: Option<String>,
    /// Optional save path.
    pub saveas: Option<String>,
    /// Optional URL.
    pub url: Option<String>,
    /// Optional request data.
    pub data: Option<Data>,
    /// Task ID.
    pub tid: String,
    /// Task title.
    pub title: String,
    /// Task description.
    pub description: String,
    /// Action type.
    pub action: Action,
    /// Execution mode.
    pub mode: Mode,
    /// Priority level.
    pub priority: i32,
    /// MIME type of the content.
    pub mime_type: String,
    /// Progress information.
    pub progress: Progress,
    /// Whether gauge is enabled.
    pub gauge: bool,
    /// Creation time.
    pub ctime: i64,
    /// Modification time.
    pub mtime: i64,
    /// Whether retry is enabled.
    pub retry: bool,
    /// Number of retry attempts.
    pub tries: i32,
    /// Error type.
    pub faults: Faults,
    /// Reason for failure.
    pub reason: String,
    /// Optional extra parameters.
    pub extras: Option<HashMap<String, String>>,
}

/// Converts from core TaskInfo to API TaskInfo.
impl From<request_core::info::TaskInfo> for TaskInfo {
    fn from(value: request_core::info::TaskInfo) -> Self {
        let saveas = if value.common_data.action == Action::Upload as u8 {
            "".to_string()
        } else {
            value.file_specs.get(0).map(|x| x.path.clone()).unwrap_or("".to_string())
        };
        TaskInfo {
            uid: Some(value.common_data.uid.to_string()),
            bundle: Some(value.bundle),
            saveas: Some(saveas),
            url: Some(value.url),
            // todo
            data: Some(Data::S(value.data)),
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
            faults: request_core::info::Faults::from(request_core::info::Reason::from(value.common_data.reason as u32)).into(),
            reason: value.common_data.reason.to_string(),
            extras: Some(value.extras.clone()),
        }
    }
}

/// Represents an HTTP response.
#[ani_rs::ani(path = "L@ohos/request/request/agent/HttpResponseInner")]
pub struct HttpResponse {
    /// HTTP version.
    version: String,
    /// HTTP status code.
    status_code: i32,
    /// Reason phrase.
    reason: String,
    /// Response headers.
    headers: HashMap<String, Vec<String>>,
}

/// Converts from core Response to API HttpResponse.
impl From<&request_core::info::Response> for HttpResponse {
    fn from(value: &request_core::info::Response) -> Self {
        HttpResponse {
            version: value.version.clone(),
            status_code: value.status_code as i32,
            reason: value.reason.clone(),
            headers: value.headers.clone(),
        }
    }
}

/// Represents a request task.
#[ani_rs::ani(path = "L@ohos/request/request/agent/TaskInner")]
pub struct Task {
    /// Task ID.
    pub tid: String,
    pub config: Config,
}

/// Represents configuration for a task group.
#[ani_rs::ani(path = "L@ohos/request/request/agent/GroupConfigInner")]
pub struct GroupConfig {
    /// Optional gauge flag for the group.
    pub gauge: Option<bool>,
    /// Notification details for the group.
    pub notification: Notification,
}

impl From<request_core::config::TaskConfig> for Config {
    fn from(value: request_core::config::TaskConfig) -> Self {
        Config {
            action: Action::from(value.common_data.action),
            url: value.url,
            title: if value.title.is_empty() { None } else { Some(value.title) },
            description: if value.description.is_empty() { None } else { Some(value.description) },
            mode: Some(Mode::from(value.common_data.mode)),
            overwrite: None,
            method: if value.method == "GET" { None } else { Some(value.method) },
            headers: if value.headers.is_empty() { None } else { Some(value.headers) },
            data: Some(Data::S(value.data)),
            saveas: None,
            network: Some(value.common_data.network_config.into()),
            metered: Some(value.common_data.metered),
            roaming: Some(value.common_data.roaming),
            retry: Some(value.common_data.retry),
            redirect: Some(value.common_data.redirect),
            proxy: if value.proxy.is_empty() { None } else { Some(value.proxy) },
            index: Some(value.common_data.index as i32),
            begins: Some(value.common_data.begins as i64),
            ends: Some(value.common_data.ends),
            gauge: Some(value.common_data.gauge),
            precise: Some(value.common_data.precise),
            token: if value.token.is_empty() { None } else { Some(value.token) },
            priority: Some(value.common_data.priority as i32),
            extras: if value.extras.is_empty() { None } else { Some(value.extras) },
            multipart: Some(value.common_data.multipart),
            notification: None,
        }
    }
}

/// Converts from API Config to core TaskConfig.
///
/// Maps API configuration options to the corresponding core task configuration,
/// providing default values for unspecified fields.
impl From<Config> for TaskConfig {
    fn from(value: Config) -> Self {
        let mut form_items = vec![];
        let mut file_specs = vec![];
        let mut data = "".to_string();
        let method;
        // todo: error?
        if matches!(value.action, Action::Upload) {
            method = match value.method {
                Some(m) if m.to_uppercase() == "POST" => m,
                _ => "PUT".to_string(),
            };
            if let Some(Data::Array(form_items_data)) = value.data {
                for form_item in form_items_data {
                    match form_item.value {
                        Value::S(s) => {
                            // String 类型的 value，添加到 form_items
                            form_items.push(request_core::config::FormItem {
                                name: form_item.name,
                                value: s,
                            });
                        }
                        Value::FileSpec(file_spec) => {
                            let mut file_spec: request_core::file::FileSpec = file_spec.into();
                            file_spec.name = form_item.name;
                            file_specs.push(file_spec);
                        }
                        Value::Array(file_spec_array) => {
                            for file_spec in file_spec_array {
                                let mut file_spec: request_core::file::FileSpec = file_spec.into();
                                file_spec.name = form_item.name.clone();
                                file_specs.push(file_spec);
                            }
                        }
                    }
                }
            }
        } else {
            method = match value.method {
                Some(m) if m.to_uppercase() == "POST" => m,
                _ => "GET".to_string(),
            };
            if let Some(Data::S(s)) = value.data {
                data = s;
            }
        }
        // todo: cert pins
        TaskConfig {
            bundle: "".to_string(),
            bundle_type: 0,
            atomic_account: "".to_string(),
            url: value.url,
            title: value.title.unwrap_or("".to_string()),
            description: value.description.unwrap_or_default(),
            method: method,
            headers: value.headers.unwrap_or_default(),
            data,
            token: value.token.unwrap_or("".to_string()),
            proxy: value.proxy.unwrap_or("".to_string()),
            certificate_pins: "".to_string(),
            extras: value.extras.unwrap_or_default(),
            version: Version::API10,
            form_items,
            file_specs,
            body_file_paths: vec![],
            certs_path: vec![],
            common_data: CommonTaskConfig {
                task_id: 0,
                uid: 0,
                token_id: 0,
                action: value.action.into(),
                cover: false,
                network_config: value.network.map(|n| n.into()).unwrap_or(NetworkConfig::Any),
                metered: value.metered.unwrap_or(false),
                roaming: value.roaming.unwrap_or(true),
                retry: value.retry.unwrap_or(true),
                redirect: value.redirect.unwrap_or(true),
                index: value.index.map(|i| i as u32).unwrap_or(0u32),
                // todo
                begins: value.begins.map(|b| if b > 0 { b as u64 } else { 0u64 }).unwrap_or(0u64),
                ends: value.ends.unwrap_or(-1),
                gauge: value.gauge.unwrap_or(false),
                precise: value.precise.unwrap_or(false),
                priority: value.priority.map(|p| p as u32).unwrap_or(0u32),
                // todo
                background: !matches!(value.mode, Some(Mode::Foreground)),
                multipart: value.multipart.unwrap_or(false),
                mode: value.mode.unwrap_or(Mode::Background).into(),
                min_speed: MinSpeed {
                    speed: 0,
                    duration: 0,
                },
                timeout: Timeout {
                    connection_timeout: 0,
                    total_timeout: 0,
                },
            },
            saveas: value.saveas.unwrap_or_default(),
            overwrite: value.overwrite.unwrap_or(false),
            notification: value.notification.map(Into::into).unwrap_or(request_core::config::Notification {
                title: None,
                text: None,
            }),
        }
    }
}
