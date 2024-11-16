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

use std::collections::HashMap;
use std::fmt::Display;
use std::pin::Pin;

use crate::wrapper::ffi::{GetHeaders, HttpClientResponse};

/// http client response
pub struct Response<'a> {
    inner: &'a HttpClientResponse,
}

impl<'a> Response<'a> {
    /// Get Response Code
    pub fn status(&self) -> ResponseCode {
        self.inner
            .GetResponseCode()
            .try_into()
            .map_err(|e| {})
            .unwrap_or_default()
    }

    pub fn headers(&self) -> HashMap<String, String> {
        let ptr = self.inner as *const HttpClientResponse as *mut HttpClientResponse;
        let p = unsafe { Pin::new_unchecked(ptr.as_mut().unwrap()) };

        let mut headers = GetHeaders(p).into_iter();
        let mut ret = HashMap::new();
        loop {
            if let Some(key) = headers.next() {
                if let Some(value) = headers.next() {
                    ret.insert(key, value);
                    continue;
                }
            }
            break;
        }
        ret
    }

    pub(crate) fn from_ffi(inner: &'a HttpClientResponse) -> Self {
        Self { inner }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ResponseCode {
    #[default]
    None = 0,
    Ok = 200,
    Created,
    Accepted,
    NotAuthoritative,
    NoContent,
    Reset,
    Partial,
    MultChoice = 300,
    MovedPerm,
    MovedTemp,
    SeeOther,
    NotModified,
    UseProxy,
    BadRequest = 400,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    BadMethod,
    NotAcceptable,
    ProxyAuth,
    ClientTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconFailed,
    EntityTooLarge,
    ReqTooLong,
    UnsupportedType,
    InternalError = 500,
    NotImplemented,
    BadGateway,
    Unavailable,
    GatewayTimeout,
    Version,
}

impl Display for ResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.clone() as i32;
        write!(f, "{} {:?}", code, self)
    }
}
