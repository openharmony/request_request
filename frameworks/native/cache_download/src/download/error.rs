// Copyright (C) 2024 Huawei Device Co., Ltd.
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

//! Error types and handling for cache download operations.
//! 
//! This module defines the primary error type used throughout the cache download system,
//! along with conversion from common error sources.

use std::io;

use super::common::CommonError;

/// Primary error type for cache download operations.
///
/// Encapsulates error information including error code, message, and error kind.
#[derive(Debug)]
pub struct CacheDownloadError {
    /// Numeric error code, if available
    code: Option<i32>,
    /// Human-readable error message
    message: String,
    /// Categorizes the type of error that occurred
    kind: ErrorKind,
}

impl CacheDownloadError {
    /// Returns the error code.
    ///
    /// # Returns
    /// The error code if available, otherwise 0.
    pub fn code(&self) -> i32 {
        self.code.unwrap_or(0)
    }

    /// Returns the error message.
    ///
    /// # Returns
    /// A string slice containing the error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the error kind as an integer code for FFI compatibility.
    ///
    /// # Returns
    /// An integer representation of the error kind.
    pub fn ffi_kind(&self) -> i32 {
        self.kind.clone() as i32
    }
}

/// Categorizes the type of error that occurred.
#[derive(Debug, Clone)]
pub enum ErrorKind {
    /// HTTP-related errors, typically from network operations
    Http,
    /// I/O-related errors, typically from file operations
    Io,
}

impl From<io::Error> for CacheDownloadError {
    /// Converts an I/O error into a cache download error.
    ///
    /// Preserves the OS error code if available and sets the error kind to Io.
    ///
    /// # Parameters
    /// - `err`: The I/O error to convert
    ///
    /// # Returns
    /// A new `CacheDownloadError` with the I/O error information.
    fn from(err: io::Error) -> Self {
        CacheDownloadError {
            code: err.raw_os_error(),
            message: err.to_string(),
            kind: ErrorKind::Io,
        }
    }
}

impl<'a, E> From<&'a E> for CacheDownloadError
where
    E: CommonError,
{
    /// Converts a reference to any type implementing `CommonError` into a cache download error.
    ///
    /// Sets the error kind to Http and preserves both the error code and message.
    ///
    /// # Type Parameters
    /// - `E`: Type implementing `CommonError`
    ///
    /// # Parameters
    /// - `err`: Reference to the error object to convert
    ///
    /// # Returns
    /// A new `CacheDownloadError` with the converted error information.
    fn from(err: &'a E) -> Self {
        CacheDownloadError {
            code: Some(err.code()),
            message: err.msg().to_string(),
            kind: ErrorKind::Http,
        }
    }
}
