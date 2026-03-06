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

//! Exception error codes for the request ANI module.
//!
//! This module defines all error codes used throughout the request service
//! ANI implementation, following the OpenHarmony error code conventions.
//! Error codes are organized by category: success, permission, parameter,
//! file I/O, service, and task-related errors.

/// Exception error codes used by the request ANI module.
///
/// These error codes are returned to callers when operations fail, providing
/// detailed information about the nature of the failure.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionErrorCode {
    /// Operation completed successfully.
    E_OK = 0,
    /// Service ability is unloading.
    E_UNLOADING_SA = 1,
    /// IPC message size exceeds the limit.
    E_IPC_SIZE_TOO_LARGE = 2,
    /// MIME type not found for the resource.
    E_MIMETYPE_NOT_FOUND = 3,
    /// Task index exceeds the maximum allowed value.
    E_TASK_INDEX_TOO_LARGE = 4,
    /// Channel is not open for communication.
    E_CHANNEL_NOT_OPEN = 5,
    /// Permission denied for the operation.
    E_PERMISSION = 201,
    /// Operation requires system application privileges.
    E_NOT_SYSTEM_APP = 202,
    /// Parameter validation failed.
    E_PARAMETER_CHECK = 401,
    /// Feature or operation is not supported.
    E_UNSUPPORTED = 801,
    /// File I/O error occurred.
    E_FILE_IO = 13400001,
    /// Invalid file path specified.
    E_FILE_PATH = 13400002,
    /// Internal service error occurred.
    E_SERVICE_ERROR = 13400003,
    /// Other unspecified error occurred.
    E_OTHER = 13499999,
    /// Task queue is full.
    E_TASK_QUEUE = 21900004,
    /// Task mode is invalid or incompatible.
    E_TASK_MODE = 21900005,
    /// Task with the specified ID was not found.
    E_TASK_NOT_FOUND = 21900006,
    /// Task is in an invalid state for the operation.
    E_TASK_STATE = 21900007,
    /// Task group with the specified ID was not found.
    E_GROUP_NOT_FOUND = 21900008,
}
