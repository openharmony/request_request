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

//! Error types for client-side operations.
//! 
//! This module defines error types used by the client API when creating download tasks,
//! providing a unified error interface while maintaining specific error information.

// Import the download path error type
use crate::check::file::DownloadPathError;

/// Error types that can occur when creating a download task.
///
/// Represents the possible error states encountered during task creation,
/// including path validation errors and generic error codes.
#[derive(Debug)]
pub enum CreateTaskError {
    /// Download path validation error
    DownloadPath(DownloadPathError),
    /// Generic error represented by an integer code
    Code(i32),
}

/// Converts a `DownloadPathError` into a `CreateTaskError`.
///
/// Enables the `?` operator to automatically convert path validation errors
/// into task creation errors during error propagation.
impl From<DownloadPathError> for CreateTaskError {
    fn from(error: DownloadPathError) -> Self {
        CreateTaskError::DownloadPath(error)
    }
}

/// Converts an integer error code into a `CreateTaskError`.
///
/// Allows for easy conversion from numeric error codes to the error enum,
/// enabling concise error handling with numeric error identifiers.
impl From<i32> for CreateTaskError {
    fn from(code: i32) -> Self {
        CreateTaskError::Code(code)
    }
}
