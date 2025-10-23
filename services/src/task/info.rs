// Copyright (C) 2023 Huawei Device Co., Ltd.
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

/// Task state enumeration.
pub use ffi::State;

use super::notify::{EachFileStatus, NotifyData, Progress};
use crate::task::config::{Action, Version};
use crate::task::reason::Reason;
use crate::utils::c_wrapper::{CFileSpec, CFormItem};
use crate::utils::form_item::{FileSpec, FormItem};
use crate::utils::hashmap_to_string;

/// Contains comprehensive information about a download/upload task.
#[derive(Debug, Clone)]
pub(crate) struct TaskInfo {
    /// Bundle name of the application that created the task.
    pub(crate) bundle: String,
    /// URL for the network request.
    pub(crate) url: String,
    /// Request payload data.
    pub(crate) data: String,
    /// Authentication token for the task.
    pub(crate) token: String,
    /// Form items to be included in the request.
    pub(crate) form_items: Vec<FormItem>,
    /// File specifications for download/upload.
    pub(crate) file_specs: Vec<FileSpec>,
    /// Title of the task.
    pub(crate) title: String,
    /// Description of the task.
    pub(crate) description: String,
    /// MIME type of the content.
    pub(crate) mime_type: String,
    /// Current progress of the task.
    pub(crate) progress: Progress,
    /// Additional task-specific parameters.
    pub(crate) extras: HashMap<String, String>,
    /// Common task metadata.
    pub(crate) common_data: CommonTaskInfo,
    /// Maximum speed limit in bytes per second.
    pub(crate) max_speed: i64,
    /// Time when the task was created.
    pub(crate) task_time: u64,
}

impl TaskInfo {
    /// Creates a new `TaskInfo` with default values.
    pub(crate) fn new() -> Self {
        Self {
            bundle: "".to_string(),
            url: "".to_string(),
            data: "".to_string(),
            token: "".to_string(),
            form_items: vec![],
            file_specs: vec![],
            title: "".to_string(),
            description: "".to_string(),
            mime_type: "".to_string(),
            // Has at least one progress size.
            progress: Progress::new(vec![0]),
            extras: HashMap::new(),
            common_data: CommonTaskInfo::new(),
            max_speed: 0,
            task_time: 0,
        }
    }

    /// Gets the user ID associated with this task.
    pub(crate) fn uid(&self) -> u64 {
        self.common_data.uid
    }

    /// Gets the MIME type of the task content.
    pub(crate) fn mime_type(&self) -> String {
        self.mime_type.clone()
    }

    /// Gets the action type (download/upload) for this task.
    pub(crate) fn action(&self) -> Action {
        Action::from(self.common_data.action)
    }

    /// Gets the authentication token for this task.
    pub(crate) fn token(&self) -> String {
        self.token.clone()
    }
}

/// Common metadata shared across different task representations.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct CommonTaskInfo {
    /// Unique identifier for the task.
    pub(crate) task_id: u32,
    /// User ID that owns the task.
    pub(crate) uid: u64,
    /// Action type encoded as a byte (0 for download, 1 for upload).
    pub(crate) action: u8,
    /// Operating mode encoded as a byte.
    pub(crate) mode: u8,
    /// Creation time in milliseconds since epoch.
    pub(crate) ctime: u64,
    /// Modification time in milliseconds since epoch.
    pub(crate) mtime: u64,
    /// Reason code for current state.
    pub(crate) reason: u8,
    /// Whether progress can be tracked accurately.
    pub(crate) gauge: bool,
    /// Whether automatic retries are enabled.
    pub(crate) retry: bool,
    /// Number of retry attempts made.
    pub(crate) tries: u32,
    /// API version used for this task.
    pub(crate) version: u8,
    /// Task priority level.
    pub(crate) priority: u32,
}

impl CommonTaskInfo {
    /// Creates a new `CommonTaskInfo` with default values.
    pub(crate) fn new() -> Self {
        Self {
            task_id: 0,
            uid: 0,
            action: 0,
            mode: 0,
            ctime: 0,
            mtime: 0,
            reason: 0,
            gauge: false,
            retry: false,
            tries: 0,
            version: 0,
            priority: 0,
        }
    }
}

/// Set of task information prepared for FFI calls.
pub(crate) struct InfoSet {
    /// Form items converted to C-compatible format.
    pub(crate) form_items: Vec<CFormItem>,
    /// File specifications converted to C-compatible format.
    pub(crate) file_specs: Vec<CFileSpec>,
    /// JSON string representation of file sizes.
    pub(crate) sizes: String,
    /// JSON string representation of processed bytes.
    pub(crate) processed: String,
    /// JSON string representation of extra parameters.
    pub(crate) extras: String,
}

// C++ interoperability bridge for task state enumeration
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    #[derive(Clone, Copy, PartialEq, Debug)]
    #[repr(u8)]
    /// Represents the current state of a task.
    pub enum State {
        /// Task has been initialized but not yet started.
        Initialized = 0x00,
        /// Task is waiting to run.
        Waiting = 0x10,
        /// Task is actively running.
        Running = 0x20,
        /// Task is retrying after a failure.
        Retrying = 0x21,
        /// Task has been paused by the user or system.
        Paused = 0x30,
        /// Task has been stopped by the user or system.
        Stopped = 0x31,
        /// Task has completed successfully.
        Completed = 0x40,
        /// Task has failed to complete.
        Failed = 0x41,
        /// Task has been removed from the system.
        Removed = 0x50,
        /// Wildcard value used for filtering any state.
        Any = 0x61,
    }
}

/// Contains information needed to update a task's state.
#[derive(Debug)]
pub(crate) struct UpdateInfo {
    /// New modification time.
    pub(crate) mtime: u64,
    /// New reason code.
    pub(crate) reason: u8,
    /// Updated retry count.
    pub(crate) tries: u32,
    /// Updated MIME type.
    pub(crate) mime_type: String,
    /// Updated progress information.
    pub(crate) progress: Progress,
}

impl From<u8> for State {
    /// Converts a byte value to a `State` enum variant.
    /// 
    /// # Notes
    /// Values not explicitly mapped default to `State::Any`.
    fn from(value: u8) -> Self {
        match value {
            0 => State::Initialized,
            16 => State::Waiting,
            32 => State::Running,
            33 => State::Retrying,
            48 => State::Paused,
            49 => State::Stopped,
            64 => State::Completed,
            65 => State::Failed,
            80 => State::Removed,
            _ => State::Any,
        }
    }
}

impl TaskInfo {
    /// Builds an `InfoSet` for FFI communication from this `TaskInfo`.
    /// 
    /// Converts various components to C-compatible formats for interop.
    pub(crate) fn build_info_set(&self) -> InfoSet {
        InfoSet {
            form_items: self.form_items.iter().map(|x| x.to_c_struct()).collect(),
            file_specs: self.file_specs.iter().map(|x| x.to_c_struct()).collect(),
            sizes: format!("{:?}", self.progress.sizes),
            processed: format!("{:?}", self.progress.processed),
            extras: hashmap_to_string(&self.extras),
        }
    }

    /// Creates a list of `EachFileStatus` objects representing the status of each file.
    pub(crate) fn build_each_file_status(&self) -> Vec<EachFileStatus> {
        EachFileStatus::create_each_file_status(
            &self.file_specs,
            self.progress.common_data.index,
            self.common_data.reason.into(),
        )
    }

    /// Builds a `NotifyData` object for status notifications.
    pub(crate) fn build_notify_data(&self) -> NotifyData {
        NotifyData {
            bundle: self.bundle.clone(),
            progress: self.progress.clone(),
            action: Action::from(self.common_data.action),
            version: Version::from(self.common_data.version),
            each_file_status: self.build_each_file_status(),
            task_id: self.common_data.task_id,
            uid: self.common_data.uid,
        }
    }
}

/// Container for multiple task information dumps.
#[derive(Debug)]
pub(crate) struct DumpAllInfo {
    /// List of individual task dumps.
    pub(crate) vec: Vec<DumpAllEachInfo>,
}

/// Contains minimal information for a single task in a dump.
#[derive(Debug)]
pub(crate) struct DumpAllEachInfo {
    /// Task identifier.
    pub(crate) task_id: u32,
    /// Action type (download/upload).
    pub(crate) action: Action,
    /// Current task state.
    pub(crate) state: State,
    /// Reason for current state.
    pub(crate) reason: Reason,
}

/// Contains detailed information for a single task dump.
#[derive(Debug)]
pub(crate) struct DumpOneInfo {
    /// Task identifier.
    pub(crate) task_id: u32,
    /// Action type (download/upload).
    pub(crate) action: Action,
    /// Current task state.
    pub(crate) state: State,
    /// Reason for current state.
    pub(crate) reason: Reason,
}

#[cfg(test)]
mod ut_info {
    include!("../../tests/ut/task/ut_info.rs");
}
