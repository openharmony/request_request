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

use request_core::config::{self, CommonTaskConfig, NetworkConfig, TaskConfig, Version};
use serde::{Deserialize, Serialize};

/// Defines the type of action for a request task.
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
#[derive(Serialize, Deserialize)]
pub enum Value {
    /// String value type.
    S(String),
    /// File specification type.
    #[serde(rename = "L@ohos/request/request/agent/FileSpecInner;")]
    FileSpec(FileSpec),
    /// Array of file specifications.
    Array(Vec<FileSpec>),
}

/// Represents an item in a form for data submission.
#[ani_rs::ani(path = "L@ohos/request/request/agent/FormItemInner")]
pub struct FormItem {
    /// Name of the form item.
    name: String,
    /// Value of the form item.
    value: Value,
}

/// Represents notification details for a request task.
#[ani_rs::ani]
pub struct Notification {
    /// Optional title for the notification.
    title: Option<String>,
    /// Optional text content for the notification.
    text: Option<String>,
}

/// Represents different data types for request body content.
#[derive(Serialize, Deserialize)]
pub enum Data {
    /// String data type.
    S(String),
    /// Array of form items.
    Array(Vec<FormItem>),
}

/// Represents configuration for a request task.
#[ani_rs::ani]
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

/// Represents search filter criteria for tasks.
#[ani_rs::ani]
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
}

/// Represents configuration for a task group.
#[ani_rs::ani]
pub struct GroupConfig {
    /// Optional gauge flag for the group.
    pub gauge: Option<bool>,
    /// Notification details for the group.
    pub notification: Notification,
}

/// Converts from API Config to core TaskConfig.
/// 
/// Maps API configuration options to the corresponding core task configuration,
/// providing default values for unspecified fields.
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
