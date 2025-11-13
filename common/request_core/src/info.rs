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

//! Task information and state management.
//!
//! This module defines structures and enums for representing task states, progress,
//! notifications, and detailed task information used throughout the request system.

use std::collections::HashMap;

use ipc::parcel::Deserialize;

use crate::config::{Action, FormItem, Mode, Version};
use crate::file::FileSpec;

/// Enumeration of possible task states.
///
/// Represents the lifecycle stages of a network task, from initialization through execution
/// to completion or failure.
#[derive(Clone, Debug)]
#[repr(u32)]
pub enum State {
    /// Task has been initialized but not yet scheduled.
    Initialized = 0x00,
    /// Task is waiting to be executed.
    Waiting = 0x10,
    /// Task is currently being executed.
    Running = 0x20,
    /// Task is retrying after a previous failure.
    Retrying = 0x21,
    /// Task execution has been temporarily paused.
    Paused = 0x30,
    /// Task execution has been permanently stopped.
    Stopped = 0x31,
    /// Task has completed successfully.
    Completed = 0x40,
    /// Task has failed to complete.
    Failed = 0x41,
    /// Task has been removed from the system.
    Removed = 0x50,
    /// Special value representing any state in filter operations.
    Any = 0x61,
}

impl From<u32> for State {
    /// Converts a u32 value to a `State` enum variant.
    ///
    /// # Notes
    ///
    /// Any value that doesn't match a defined state will be mapped to `State::Any`.
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

/// Types of notifications that can be subscribed to for task events.
#[repr(u32)]
#[derive(Debug)]
pub enum SubscribeType {
    /// Task has completed successfully.
    Completed = 0,
    /// Task has failed to complete.
    Failed,
    /// HTTP headers have been received.
    HeaderReceive,
    /// Task execution has been paused.
    Pause,
    /// Progress update for the task.
    Progress,
    /// Task has been removed.
    Remove,
    /// Task execution has been resumed.
    Resume,
    /// HTTP response has been received.
    Response,
    FaultOccur,
    Wait,
    /// Marker for the end of the enum.
    Butt,
}

impl From<u32> for SubscribeType {
    /// Converts a u32 value to a `SubscribeType` enum variant.
    ///
    /// # Panics
    ///
    /// Panics if the value is not a valid `SubscribeType` variant.
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
    /// Unique identifier of the task associated with this response.
    pub task_id: String,
    /// Version identifier of the response format.
    pub version: String,
    /// HTTP status code returned by the server.
    pub status_code: i32,
    /// Textual reason phrase associated with the status code.
    pub reason: String,
    /// HTTP headers returned by the server.
    pub headers: HashMap<String, Vec<String>>,
}

/// Status information for a specific task file.
#[derive(Clone, Debug)]
pub struct TaskState {
    /// Path to the file being processed.
    pub path: String,
    /// HTTP response code for this file.
    pub response_code: u32,
    /// Additional status message for this file.
    pub message: String,
}

/// Progress information for a task.
///
/// Contains current state, processed bytes, and other progress metrics.
#[derive(Debug)]
pub struct Progress {
    /// Current state of the task.
    pub state: State,
    /// Index of the current file being processed.
    pub index: u32,
    /// Number of bytes processed in the current file.
    pub processed: u64,
    /// Total number of bytes processed across all files.
    pub total_processed: u64,
    /// Sizes of all files in the task (in bytes).
    pub sizes: Vec<i64>,
    /// Additional progress-related metadata.
    pub extras: HashMap<String, String>,
    pub body_bytes: Vec<u8>,
}

/// Data structure for task notifications.
///
/// Combines subscription type, task identifier, progress information,
/// and additional task details.
#[derive(Debug)]
pub struct NotifyData {
    /// Type of notification being sent.
    pub subscribe_type: SubscribeType,
    /// Unique identifier of the task.
    pub task_id: u32,
    /// Current progress information for the task.
    pub progress: Progress,
    /// Action type of the task.
    pub action: Action,
    /// Version of the task protocol.
    pub version: Version,
    /// Status information for each file in the task.
    pub task_states: Vec<TaskState>,
}

/// Detailed progress information for a task.
#[derive(Clone, Debug)]
pub struct InfoProgress {
    /// Common progress data shared across different progress representations.
    pub common_data: CommonProgress,
    /// Total size of the files (in bytes).
    pub sizes: Vec<i64>,
    /// Processed size for each individual file (in bytes).
    pub processed: Vec<usize>,
    /// Additional progress-related metadata.
    pub extras: HashMap<String, String>,
}

/// Common progress data shared across different progress representations.
#[derive(Clone, Debug)]
pub struct CommonProgress {
    /// Current state of the task (as a numeric value).
    pub state: u8,
    /// Index of the current file being processed.
    pub index: usize,
    /// Total number of bytes processed across all files.
    pub total_processed: usize,
}

/// Core task information with minimal overhead.
///
/// Contains essential task metadata needed for quick task identification and management.
#[derive(Copy, Clone, Debug)]
pub struct CommonTaskInfo {
    /// Unique identifier of the task.
    pub task_id: u32,
    /// User ID of the task owner.
    pub uid: u64,
    /// Action type of the task (as a numeric value).
    pub action: u8,
    /// Operating mode of the task (as a numeric value).
    pub mode: u8,
    /// Creation time of the task (Unix timestamp).
    pub ctime: u64,
    /// Last modification time of the task (Unix timestamp).
    pub mtime: u64,
    /// Reason code for the current state.
    pub reason: u8,
    /// Whether the task progress can be accurately measured.
    pub gauge: bool,
    /// Whether the task will automatically retry on failure.
    pub retry: bool,
    /// Number of retry attempts made so far.
    pub tries: u32,
    /// Protocol version of the task.
    pub version: u8,
    /// Priority level of the task.
    pub priority: u32,
}

/// Comprehensive information about a network task.
///
/// Contains all details needed to represent and manage a network task,
/// including configuration, state, and progress information.
///
/// # Examples
///
/// ```rust
/// use request_core::{info::TaskInfo, file::FileSpec};
///
/// // Access task details
/// fn process_task_info(task_info: &TaskInfo) {
///     println!("Task ID: {}", task_info.common_data.task_id);
///     println!("URL: {}", task_info.url);
///     println!("Progress: {}/{}",
///              task_info.progress.common_data.total_processed,
///              task_info.progress.sizes.iter().sum::<i64>());
/// }
/// ```
#[derive(Debug, Clone)]
pub struct TaskInfo {
    /// Bundle name of the task owner.
    pub bundle: String,
    /// URL of the network request.
    pub url: String,
    /// Request body data.
    pub data: String,
    /// Authentication token.
    pub token: String,
    /// Form data items for the request.
    pub form_items: Vec<FormItem>,
    /// File specifications for uploads or downloads.
    pub file_specs: Vec<FileSpec>,
    /// User-visible title of the task.
    pub title: String,
    /// User-visible description of the task.
    pub description: String,
    /// MIME type of the request or response.
    pub mime_type: String,
    /// Progress information for the task.
    pub progress: InfoProgress,
    /// Additional task metadata.
    pub extras: HashMap<String, String>,
    /// Common task information.
    pub common_data: CommonTaskInfo,
    /// Maximum allowed transfer speed (bytes per second).
    pub max_speed: i64,
}

impl Deserialize for TaskInfo {
    /// Deserializes a `TaskInfo` from an IPC parcel.
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails for any reason.
    fn deserialize(parcel: &mut ipc::parcel::MsgParcel) -> ipc::IpcResult<Self> {
        let gauge = parcel.read::<bool>().unwrap();
        let retry = parcel.read::<bool>().unwrap();
        let action = parcel.read::<u32>().unwrap() as u8;
        let mode = parcel.read::<u32>().unwrap() as u8;
        let reason = parcel.read::<u32>().unwrap() as u8;
        let tries = parcel.read::<u32>().unwrap();

        // Parse user ID from string representation
        let uid = parcel.read::<String>().unwrap().parse::<u64>().unwrap_or(0);

        let bundle = parcel.read::<String>().unwrap();
        let url = parcel.read::<String>().unwrap();

        // Parse task ID from string representation
        let task_id = parcel.read::<String>().unwrap().parse::<u32>().unwrap_or(0);

        let title = parcel.read::<String>().unwrap();
        let mime_type = parcel.read::<String>().unwrap();
        let ctime = parcel.read::<u64>().unwrap();
        let mtime = parcel.read::<u64>().unwrap();
        let data = parcel.read::<String>().unwrap();
        let description = parcel.read::<String>().unwrap();
        let priority = parcel.read::<u32>().unwrap();

        // Read form items
        let form_items_len = parcel.read::<u32>().unwrap() as usize;
        let mut form_items = Vec::with_capacity(form_items_len);
        for _ in 0..form_items_len {
            let name = parcel.read::<String>().unwrap();
            let value = parcel.read::<String>().unwrap();
            form_items.push(FormItem { name, value });
        }

        // Read file specifications
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

        // Read progress information
        let state = parcel.read::<u32>().unwrap() as u8;
        let index = parcel.read::<u32>().unwrap() as usize;
        let processed = parcel.read::<u64>().unwrap() as usize;
        let total_processed = parcel.read::<u64>().unwrap() as usize;
        let sizes = parcel.read::<Vec<i64>>().unwrap();

        // Read progress extras
        let extras_len = parcel.read::<u32>().unwrap() as usize;
        let mut progress_extras = HashMap::with_capacity(extras_len);
        for _ in 0..extras_len {
            let key = parcel.read::<String>().unwrap();
            let value = parcel.read::<String>().unwrap();
            progress_extras.insert(key, value);
        }

        // Read task extras
        let extras_len = parcel.read::<u32>().unwrap() as usize;
        let mut extras = HashMap::with_capacity(extras_len);
        for _ in 0..extras_len {
            let key = parcel.read::<String>().unwrap();
            let value = parcel.read::<String>().unwrap();
            extras.insert(key, value);
        }

        // Read protocol version
        let version = parcel.read::<u32>().unwrap() as u8;

        // Read task states for individual files
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

        // Construct common task information
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

        // Construct progress information
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

        // Return constructed TaskInfo
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
