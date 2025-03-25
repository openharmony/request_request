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

#[derive(Clone)]
pub struct HttpClientError {
    code: HttpErrorCode,
    msg: String,
}

impl HttpClientError {
    pub(crate) fn from_ffi(inner: &ffi::HttpClientError) -> Self {
        let code = HttpErrorCode::try_from(inner.GetErrorCode()).unwrap_or_default();
        let msg = inner.GetErrorMessage().to_string();
        Self { code, msg }
    }

    pub fn new(code: HttpErrorCode, msg: String) -> Self {
        Self { code, msg }
    }

    pub fn code(&self) -> &HttpErrorCode {
        &self.code
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum HttpErrorCode {
    HttpNoneErr,
    HttpPermissionDeniedCode = 201,
    HttpParseErrorCode = 401,
    HttpErrorCodeBase = 2300000,
    HttpUnsupportedProtocol,
    HttpFailedInit,
    HttpUrlMalformat,
    HttpCouldntResolveProxy = 2300005,
    HttpCouldntResolveHost,
    HttpCouldntConnect,
    HttpWeirdServerReply,
    HttpRemoteAccessDenied,
    HttpHttp2Error = 2300016,
    HttpPartialFile = 2300018,
    HttpWriteError = 2300023,
    HttpUploadFailed = 2300025,
    HttpReadError = 2300026,
    HttpOutOfMemory,
    HttpOperationTimedout,
    HttpPostError = 2300034,
    HttpTaskCanceled = 2300042,
    HttpTooManyRedirects = 2300047,
    HttpGotNothing = 2300052,
    HttpSendError = 2300055,
    HttpRecvError,
    HttpSslCertproblem = 2300058,
    HttpSslCipher,
    HttpPeerFailedVerification,
    HttpBadContentEncoding,
    HttpFilesizeExceeded = 2300063,
    HttpRemoteDiskFull = 2300070,
    HttpRemoteFileExists = 2300073,
    HttpSslCacertBadfile = 2300077,
    HttpRemoteFileNotFound,
    HttpSslPinnedpubkeynotmatch = 2300090,
    HttpAuthError = 2300094,
    #[default]
    HttpUnknownOtherError = 2300999,
}
