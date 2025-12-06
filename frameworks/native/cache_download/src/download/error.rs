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
use netstack_rs::error::HttpErrorCode;

/// DNS error codes using HttpErrorCode enum values
const DNS_ERROR_CODES: &[i32] = &[
    HttpErrorCode::HttpCouldntResolveProxy as i32,
    HttpErrorCode::HttpCouldntResolveHost as i32,
];

/// TCP error codes using HttpErrorCode enum values
const TCP_ERROR_CODES: &[i32] = &[
    HttpErrorCode::HttpCouldntConnect as i32,
    HttpErrorCode::HttpSendError as i32,
    HttpErrorCode::HttpRecvError as i32,
];

/// SSL error codes using HttpErrorCode enum values
const SSL_ERROR_CODES: &[i32] = &[
    HttpErrorCode::HttpSslCertproblem as i32,
    HttpErrorCode::HttpSslCipher as i32,
    HttpErrorCode::HttpPeerFailedVerification as i32,
    HttpErrorCode::HttpSslCacertBadfile as i32,
    HttpErrorCode::HttpSslPinnedpubkeynotmatch as i32,
];

/// HTTP error codes using HttpErrorCode enum values
const HTTP_ERROR_CODES: &[i32] = &[
    HttpErrorCode::HttpRemoteAccessDenied as i32,
    HttpErrorCode::HttpHttp2Error as i32,
    HttpErrorCode::HttpPostError as i32,
    HttpErrorCode::HttpTooManyRedirects as i32,
    HttpErrorCode::HttpRemoteDiskFull as i32,
    HttpErrorCode::HttpRemoteFileExists as i32,
    HttpErrorCode::HttpRemoteFileNotFound as i32,
    HttpErrorCode::HttpAuthError as i32,
    HttpErrorCode::HttpNoneErr as i32,
    HttpErrorCode::HttpPartialFile as i32,
    HttpErrorCode::HttpBadContentEncoding as i32,
];

/// OTHERS error codes using HttpErrorCode enum values
const OTHERS_ERROR_CODES: &[i32] = &[
    HttpErrorCode::HttpPermissionDeniedCode as i32,
    HttpErrorCode::HttpParseErrorCode as i32,
    HttpErrorCode::HttpUnsupportedProtocol as i32,
    HttpErrorCode::HttpFailedInit as i32,
    HttpErrorCode::HttpUrlMalformat as i32,
    HttpErrorCode::HttpOutOfMemory as i32,
    HttpErrorCode::HttpUnknownOtherError as i32,
    HttpErrorCode::HttpWeirdServerReply as i32,
    HttpErrorCode::HttpWriteError as i32,
    HttpErrorCode::HttpUploadFailed as i32,
    HttpErrorCode::HttpReadError as i32,
    HttpErrorCode::HttpOperationTimedout as i32,
    HttpErrorCode::HttpTaskCanceled as i32,
    HttpErrorCode::HttpGotNothing as i32,
    HttpErrorCode::HttpFilesizeExceeded as i32,
];

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
    /// DNS-related errors, typically from network operations
    Dns,
    /// TCP-related errors, typically from network operations
    Tcp,
    /// SSL-related errors, typically from network operations
    Ssl,
    /// Others errors, typically from network operations
    Others,
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
    /// Sets the error kind based on error code ranges and preserves both the error code and message.
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
        let code = err.code();
        let kind = match code {
            code if DNS_ERROR_CODES.contains(&code) => ErrorKind::Dns,
            code if TCP_ERROR_CODES.contains(&code) => ErrorKind::Tcp,
            code if SSL_ERROR_CODES.contains(&code) => ErrorKind::Ssl,
            code if HTTP_ERROR_CODES.contains(&code) => ErrorKind::Http,
            code if OTHERS_ERROR_CODES.contains(&code) => ErrorKind::Others,
            // default case for unknown error codes
            _ => ErrorKind::Others,
        };

        CacheDownloadError {
            code: Some(code),
            message: err.msg().to_string(),
            kind,
        }
    }
}
