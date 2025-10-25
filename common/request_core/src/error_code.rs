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

pub const EXCEPTION_SERVICE: i32 = 13400003;

// General status codes
/// Operation completed successfully.
pub const ERR_OK: i32 = 0;

// IPC-related error codes
/// IPC message size exceeds the maximum allowed limit.
pub const IPC_SIZE_TOO_LARGE: i32 = 2;

/// IPC communication channel is not open.
pub const CHANNEL_NOT_OPEN: i32 = 5;

// Permission and access error codes
/// Permission denied to perform the requested operation.
pub const PERMISSION: i32 = 201;

/// System API error occurred.
pub const SYSTEM_API: i32 = 202;

// Validation and parameter error codes
/// Invalid or missing required parameters.
pub const PARAMETER_CHECK: i32 = 401;

// File operation error codes
/// File operation failed (reading, writing, accessing, etc.).
pub const FILE_OPERATION_ERR: i32 = 13400001;
pub const OTHER: i32 = 13499999;
pub const TASK_ENQUEUE_ERR: i32 = 21900004;

/// Invalid task mode specified.
pub const TASK_MODE_ERR: i32 = 21900005;

/// Requested task not found in the system.
pub const TASK_NOT_FOUND: i32 = 21900006;

/// Operation cannot be performed due to invalid task state.
pub const TASK_STATE_ERR: i32 = 21900007;

/// Requested task group not found.
pub const GROUP_NOT_FOUND: i32 = 21900008;
