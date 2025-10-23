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

//! QoS direction and level definitions for task scheduling.
//! 
//! This module defines structures and enums for managing Quality of Service (QoS) changes
//! for network tasks, distinguishing between download and upload operations and providing
//! different priority levels with associated speed constraints.

/// Container for QoS changes for both download and upload operations.
///
/// This struct holds optional lists of QoS direction changes for download and upload tasks,
/// allowing for batch processing of QoS updates.
pub(crate) struct QosChanges {
    /// QoS direction changes for download tasks, if any.
    pub(crate) download: Option<Vec<QosDirection>>,
    /// QoS direction changes for upload tasks, if any.
    pub(crate) upload: Option<Vec<QosDirection>>,
}

impl QosChanges {
    /// Creates a new empty `QosChanges` instance.
    ///
    /// Returns a `QosChanges` with both download and upload fields initialized to `None`.
    pub(crate) fn new() -> Self {
        Self {
            upload: None,
            download: None,
        }
    }
}
/// Represents a QoS level change for a specific task.
///
/// This struct associates a task with a new QoS level, allowing the scheduler to
/// adjust the task's priority and resource allocation.
#[derive(Debug)]
pub(crate) struct QosDirection {
    /// The user ID of the application that owns the task.
    uid: u64,
    /// The unique identifier of the task.
    task_id: u32,
    /// The new QoS level to apply to the task.
    direction: QosLevel,
}

impl QosDirection {
    /// Returns the task's owning user ID.
    pub(crate) fn uid(&self) -> u64 {
        self.uid
    }

    /// Returns the task's unique identifier.
    pub(crate) fn task_id(&self) -> u32 {
        self.task_id
    }

    /// Returns the QoS level assigned to the task.
    pub(crate) fn direction(&self) -> QosLevel {
        self.direction
    }

    /// Creates a new `QosDirection` instance.
    ///
    /// # Arguments
    ///
    /// * `uid` - The user ID of the application that owns the task.
    /// * `task_id` - The unique identifier of the task.
    /// * `direction` - The QoS level to apply to the task.
    pub(crate) fn new(uid: u64, task_id: u32, direction: QosLevel) -> Self {
        Self {
            uid,
            task_id,
            direction,
        }
    }
}

/// Quality of Service levels with associated maximum speeds.
///
/// Each enum variant represents a different priority level with a corresponding
/// maximum speed in bytes per second. A value of 0 indicates no speed limit.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum QosLevel {
    /// High priority with no speed limit (0 B/s indicates unlimited).
    High = 0,
    /// Low priority with a maximum speed of 400 KB/s.
    Low = 400 * 1024,
    /// Medium priority with a maximum speed of 800 KB/s.
    Middle = 800 * 1024,
}
