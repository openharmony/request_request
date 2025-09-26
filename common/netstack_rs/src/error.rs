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
    /// No specific error code
    HttpNoneErr,
    /// Permission denied (201)
    HttpPermissionDeniedCode = 201,
    /// Parsing error (401)
    HttpParseErrorCode = 401,
    /// Base value for HTTP error codes (2300000)
    HttpErrorCodeBase = 2300000,
    /// Unsupported protocol
    HttpUnsupportedProtocol,
    /// Failed to initialize
    HttpFailedInit,
    /// Malformed URL
    HttpUrlMalformat,
    /// Could not resolve proxy (2300005)
    HttpCouldntResolveProxy = 2300005,
    /// Could not resolve host
    HttpCouldntResolveHost,
    /// Could not connect
    HttpCouldntConnect,
    /// Unexpected server reply
    HttpWeirdServerReply,
    /// Remote access denied
    HttpRemoteAccessDenied,
    /// HTTP/2 specific error (2300016)
    HttpHttp2Error = 2300016,
    /// Partial file transfer (2300018)
    HttpPartialFile = 2300018,
    /// Error writing data (2300023)
    HttpWriteError = 2300023,
    /// Upload failed (2300025)
    HttpUploadFailed = 2300025,
    /// Error reading data (2300026)
    HttpReadError = 2300026,
    /// Out of memory
    HttpOutOfMemory,
    /// Operation timed out
    HttpOperationTimedout,
    /// POST error (2300034)
    HttpPostError = 2300034,
    /// Task was canceled (2300042)
    HttpTaskCanceled = 2300042,
    /// Too many redirects (2300047)
    HttpTooManyRedirects = 2300047,
    /// Empty response (2300052)
    HttpGotNothing = 2300052,
    /// Error sending data (2300055)
    HttpSendError = 2300055,
    /// Error receiving data
    HttpRecvError,
    /// SSL certificate problem (2300058)
    HttpSslCertproblem = 2300058,
    /// SSL cipher error
    HttpSslCipher,
    /// Peer verification failed
    HttpPeerFailedVerification,
    /// Bad content encoding
    HttpBadContentEncoding,
    /// File size exceeded limit (2300063)
    HttpFilesizeExceeded = 2300063,
    /// Remote disk full (2300070)
    HttpRemoteDiskFull = 2300070,
    /// Remote file exists (2300073)
    HttpRemoteFileExists = 2300073,
    /// Bad CA certificate file (2300077)
    HttpSslCacertBadfile = 2300077,
    /// Remote file not found
    HttpRemoteFileNotFound,
    /// SSL pinned public key mismatch (2300090)
    HttpSslPinnedpubkeynotmatch = 2300090,
    /// Authentication error (2300094)
    HttpAuthError = 2300094,
    /// Catch-all for unknown errors (2300999)
    #[default]
    HttpUnknownOtherError = 2300999,
}
