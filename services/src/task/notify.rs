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

use super::config::{Action, Version};
use super::info::State;
use super::reason::Reason;
use crate::FileSpec;

/// Types of events that can be subscribed to for task notifications.
/// 
/// Used to specify which events a client wants to receive callbacks for.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum SubscribeType {
    /// Task has completed successfully.
    Complete = 0,
    /// Task has failed to complete.
    Fail,
    /// Response headers have been received.
    HeaderReceive,
    /// Task has been paused.
    Pause,
    /// Task progress has updated.
    Progress,
    /// Task has been removed.
    Remove,
    /// Task has been resumed.
    Resume,
    /// System fault has occurred.
    FaultOccur = 8,
}

/// Reasons why a task might be waiting to run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WaitingCause {
    /// Task is waiting in the queue for its turn.
    TaskQueue = 0,
    /// Task is waiting for network connectivity.
    Network,
    /// Task is waiting due to application state constraints.
    AppState,
    /// Task is waiting due to user state constraints.
    UserState,
}

/// Contains task notification data sent to subscribers.
#[derive(Debug, Clone)]
pub(crate) struct NotifyData {
    /// Bundle name of the application that created the task.
    pub(crate) bundle: String,
    /// Current progress information.
    pub(crate) progress: Progress,
    /// Action type (download/upload).
    pub(crate) action: Action,
    /// API version used for this task.
    pub(crate) version: Version,
    /// Status of each file in the task.
    pub(crate) each_file_status: Vec<EachFileStatus>,
    /// Unique task identifier.
    pub(crate) task_id: u32,
    /// User ID that owns the task.
    pub(crate) uid: u64,
}

/// Core progress information shared across different components.
#[repr(C)]
#[derive(Clone, Debug)]
pub(crate) struct CommonProgress {
    /// Current state of the task as a raw byte value.
    pub(crate) state: u8,
    /// Index of the current file being processed.
    pub(crate) index: usize,
    /// Total number of bytes processed across all files.
    pub(crate) total_processed: usize,
}

/// Comprehensive progress information for a task.
#[derive(Debug, Clone)]
pub(crate) struct Progress {
    /// Core progress metadata.
    pub(crate) common_data: CommonProgress,
    /// Total size of each file in bytes.
    /// A value of -1 indicates unknown size.
    pub(crate) sizes: Vec<i64>,
    /// Number of bytes processed for each file.
    pub(crate) processed: Vec<usize>,
    /// Additional progress-related parameters.
    pub(crate) extras: HashMap<String, String>,
}

/// Status information for an individual file in a multi-file task.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub(crate) struct EachFileStatus {
    /// Path to the file.
    pub(crate) path: String,
    /// Reason code for the file's current status.
    pub(crate) reason: Reason,
    /// Human-readable status message.
    pub(crate) message: String,
}

impl EachFileStatus {
    /// Creates a list of `EachFileStatus` objects for a set of file specifications.
    /// 
    /// Assigns the provided reason to files at or after the specified index, and
    /// `Reason::Default` to files before the index.
    /// 
    /// # Parameters
    /// - `file_specs`: List of file specifications to create statuses for
    /// - `index`: Starting index for applying the provided reason
    /// - `reason`: Reason to apply to files at or after the index
    pub(crate) fn create_each_file_status(
        file_specs: &[FileSpec],
        index: usize,
        reason: Reason,
    ) -> Vec<EachFileStatus> {
        let mut vec = Vec::new();
        for (i, file_spec) in file_specs.iter().enumerate() {
            // Only apply the provided reason to files we're actively processing
            let code = if i >= index { reason } else { Reason::Default };
            let each_file_status = EachFileStatus {
                path: file_spec.path.clone(),
                reason: code,
                message: code.to_str().into(),
            };
            vec.push(each_file_status);
        }
        vec
    }
}

impl Progress {
    /// Creates a new `Progress` instance with the specified file sizes.
    /// 
    /// Initializes all files to have processed 0 bytes and sets the state to
    /// `State::Initialized`.
    /// 
    /// # Parameters
    /// - `sizes`: List of file sizes in bytes (-1 indicates unknown size)
    pub(crate) fn new(sizes: Vec<i64>) -> Self {
        let len = sizes.len();
        Progress {
            common_data: CommonProgress {
                state: State::Initialized.repr,
                index: 0,
                total_processed: 0,
            },
            sizes,
            processed: vec![0; len],
            extras: HashMap::<String, String>::new(),
        }
    }

    /// Checks if the task has finished processing all files.
    /// 
    /// Returns `true` only if:
    /// 1. All files have known sizes (no -1 values)
    /// 2. The sum of processed bytes equals the sum of total sizes
    pub(crate) fn is_finish(&self) -> bool {
        self.sizes.iter().all(|a| *a != -1)
            && self.processed.iter().sum::<usize>() == self.sizes.iter().sum::<i64>() as usize
    }
}
