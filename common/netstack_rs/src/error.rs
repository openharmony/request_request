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

//! Error handling for HTTP client operations.
//! 
//! This module defines error types and codes used throughout the HTTP client implementation,
//! providing a consistent error handling mechanism across the library.

use crate::wrapper::ffi;

/// Represents an error that occurred during an HTTP request.
///
/// Contains both an error code for programmatic handling and
/// a human-readable error message.
#[derive(Clone)]
pub struct HttpClientError {
    /// The specific error code categorizing this error
    code: HttpErrorCode,
    /// Human-readable description of the error
    msg: String,
}

impl HttpClientError {
    /// Creates an `HttpClientError` from a raw FFI error object.
    ///
    /// # Arguments
    /// * `inner` - The FFI error object to convert
    pub(crate) fn from_ffi(inner: &ffi::HttpClientError) -> Self {
        let code = HttpErrorCode::try_from(inner.GetErrorCode()).unwrap_or_default();
        let msg = inner.GetErrorMessage().to_string();
        Self { code, msg }
    }

    /// Creates a new `HttpClientError` with the given code and message.
    ///
    /// # Arguments
    /// * `code` - The error code
    /// * `msg` - Human-readable error message
    pub fn new(code: HttpErrorCode, msg: String) -> Self {
        Self { code, msg }
    }

    /// Gets the error code for this error.
    pub fn code(&self) -> &HttpErrorCode {
        &self.code
    }

    /// Gets the human-readable error message.
    pub fn msg(&self) -> &str {
        &self.msg
    }
}

/// Enumeration of possible HTTP client error codes.
///
/// These codes correspond to common HTTP client errors and are compatible
/// with the underlying C++ implementation through `#[repr(i32)]`.
#[derive(Default, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum HttpErrorCode {
    /// No specific error occurred.
    HttpNoneErr,
    
    /// Permission denied when attempting to perform an operation (201).
    /// 
    /// Typically indicates lack of necessary permissions for network access.
    HttpPermissionDeniedCode = 201,
    
    /// Error parsing data (401).
    /// 
    /// Indicates failure to parse HTTP headers, JSON, or other structured data.
    HttpParseErrorCode = 401,
    
    /// Base value for all HTTP error codes (2300000).
    /// 
    /// Used as an offset for generating specific error codes within the system.
    HttpErrorCodeBase = 2300000,
    
    /// The requested protocol is not supported.
    /// 
    /// Occurs when attempting to use a protocol that the client doesn't support.
    HttpUnsupportedProtocol,
    
    /// Failed to initialize the HTTP client.
    /// 
    /// Indicates a problem during client creation or initialization.
    HttpFailedInit,
    
    /// URL format is invalid or malformed.
    /// 
    /// The provided URL string doesn't conform to URL syntax rules.
    HttpUrlMalformat,
    
    /// Failed to resolve the proxy server hostname (2300005).
    HttpCouldntResolveProxy = 2300005,
    
    /// Failed to resolve the target host hostname.
    /// 
    /// May indicate DNS resolution failure or network connectivity issues.
    HttpCouldntResolveHost,
    
    /// Failed to establish a connection to the target host.
    /// 
    /// Occurs when connection attempts time out or are rejected.
    HttpCouldntConnect,
    
    /// Received an unexpected or malformed response from the server.
    HttpWeirdServerReply,
    
    /// The server denied access to the requested resource.
    HttpRemoteAccessDenied,
    
    /// HTTP/2 protocol-specific error occurred (2300016).
    HttpHttp2Error = 2300016,
    
    /// File transfer completed only partially (2300018).
    /// 
    /// The connection was closed before the full file could be transferred.
    HttpPartialFile = 2300018,
    
    /// Error occurred while writing data to a file or buffer (2300023).
    HttpWriteError = 2300023,
    
    /// File upload operation failed (2300025).
    HttpUploadFailed = 2300025,
    
    /// Error occurred while reading data from a file or stream (2300026).
    HttpReadError = 2300026,
    
    /// Insufficient memory to complete the operation.
    HttpOutOfMemory,
    
    /// Operation timed out before completion.
    /// 
    /// The operation took longer than the specified timeout period.
    HttpOperationTimedout,
    
    /// Error during HTTP POST operation (2300034).
    HttpPostError = 2300034,
    
    /// The HTTP task was canceled before completion (2300042).
    HttpTaskCanceled = 2300042,
    
    /// Exceeded the maximum number of allowed redirects (2300047).
    HttpTooManyRedirects = 2300047,
    
    /// Received an empty response from the server (2300052).
    HttpGotNothing = 2300052,
    
    /// Error occurred while sending data to the server (2300055).
    HttpSendError = 2300055,
    
    /// Error occurred while receiving data from the server.
    HttpRecvError,
    
    /// Problem with the SSL certificate validation (2300058).
    HttpSslCertproblem = 2300058,
    
    /// SSL cipher selection or handshake failed.
    HttpSslCipher,
    
    /// Peer verification failed during SSL/TLS handshake.
    HttpPeerFailedVerification,
    
    /// The content encoding of the response is invalid or unsupported.
    HttpBadContentEncoding,
    
    /// File size exceeds the configured limit (2300063).
    HttpFilesizeExceeded = 2300063,
    
    /// Remote server reported that its disk is full (2300070).
    HttpRemoteDiskFull = 2300070,
    
    /// Operation failed because the remote file already exists (2300073).
    HttpRemoteFileExists = 2300073,
    
    /// CA certificate file is invalid or corrupted (2300077).
    HttpSslCacertBadfile = 2300077,
    
    /// The requested remote file was not found on the server.
    HttpRemoteFileNotFound,
    
    /// SSL public key pinning validation failed (2300090).
    /// 
    /// The server's public key does not match the expected pinned key.
    HttpSslPinnedpubkeynotmatch = 2300090,
    
    /// Authentication with the server failed (2300094).
    HttpAuthError = 2300094,
    
    /// Catch-all for unknown or uncategorized errors (2300999).
    /// 
    /// Used when an error occurs that doesn't match any specific error code.
    #[default]
    HttpUnknownOtherError = 2300999,
}
