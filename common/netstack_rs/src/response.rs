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
use std::pin::Pin;

use cxx::SharedPtr;

use crate::task::RequestTask;
use crate::wrapper::ffi::{GetHeaders, HttpClientResponse, HttpClientTask};

/// Represents an HTTP response from the client.
///
/// This struct provides access to response status codes and headers.
/// The lifetime parameter `'a` ensures the response data remains valid
/// while this object is in use.
pub struct Response<'a> {
    /// Internal representation of the response (either borrowed or shared)
    inner: ResponseInner<'a>,
}

impl<'a> Response<'a> {
    /// Gets the HTTP status code of the response.
    ///
    /// # Returns
    /// The response code as a `ResponseCode` enum value.
    /// Returns `ResponseCode::None` if the status code cannot be determined.
    pub fn status(&self) -> ResponseCode {
        let response = self.inner.to_response();
        response.GetResponseCode().try_into().unwrap_or_default()
    }

    /// Gets all response headers as a case-insensitive HashMap.
    ///
    /// Header names are converted to lowercase for consistent access.
    ///
    /// # Returns
    /// A HashMap where keys are lowercase header names and values are header values.
    pub fn headers(&self) -> HashMap<String, String> {
        let ptr = self.inner.to_response() as *const HttpClientResponse as *mut HttpClientResponse;
        let p = unsafe { Pin::new_unchecked(ptr.as_mut().unwrap()) };

        let mut headers = GetHeaders(p).into_iter();
        let mut ret = HashMap::new();
        loop {
            if let Some(key) = headers.next() {
                if let Some(value) = headers.next() {
                    ret.insert(key.to_lowercase(), value);
                    continue;
                }
            }
            break;
        }
        ret
    }

    /// Creates a Response from a raw FFI HttpClientResponse reference.
    ///
    /// # Safety
    /// The caller must ensure the reference remains valid for the lifetime 'a.
    pub(crate) fn from_ffi(inner: &'a HttpClientResponse) -> Self {
        Self {
            inner: ResponseInner::Ref(inner),
        }
    }

    /// Creates a Response from a shared pointer to HttpClientTask.
    pub(crate) fn from_shared(inner: SharedPtr<HttpClientTask>) -> Self {
        Self {
            inner: ResponseInner::Shared(inner),
        }
    }
}

/// Internal representation of an HTTP response.
///
/// Can either be a borrowed reference or an owned shared pointer.
enum ResponseInner<'a> {
    /// Borrowed reference to a response
    Ref(&'a HttpClientResponse),
    /// Owned shared pointer to a task containing the response
    Shared(SharedPtr<HttpClientTask>),
}

impl<'a> ResponseInner<'a> {
    /// Converts the inner representation to a reference to HttpClientResponse.
    ///
    /// For shared pointers, this accesses the response through the task object.
    fn to_response(&self) -> &HttpClientResponse {
        match self {
            ResponseInner::Ref(inner) => inner,
            ResponseInner::Shared(inner) => RequestTask::pin_mut(inner)
                .GetResponse()
                .into_ref()
                .get_ref(),
        }
    }
}

/// Standard HTTP response codes with Rust-style naming.
///
/// Each variant corresponds to an HTTP status code as defined in RFC 2616.
#[derive(Clone, Default, PartialEq, Eq)]
pub enum ResponseCode {
    #[default]
    /// No response code available (0)
    None = 0,
    /// OK (200)
    Ok = 200,
    /// Created (201)
    Created,
    /// Accepted (202)
    Accepted,
    /// Non-Authoritative Information (203)
    NotAuthoritative,
    /// No Content (204)
    NoContent,
    /// Reset Content (205)
    Reset,
    /// Partial Content (206)
    Partial,
    /// Multiple Choices (300)
    MultChoice = 300,
    /// Moved Permanently (301)
    MovedPerm,
    /// Found (302)
    MovedTemp,
    /// See Other (303)
    SeeOther,
    /// Not Modified (304)
    NotModified,
    /// Use Proxy (305)
    UseProxy,
    /// Bad Request (400)
    BadRequest = 400,
    /// Unauthorized (401)
    Unauthorized,
    /// Payment Required (402)
    PaymentRequired,
    /// Forbidden (403)
    Forbidden,
    /// Not Found (404)
    NotFound,
    /// Method Not Allowed (405)
    BadMethod,
    /// Not Acceptable (406)
    NotAcceptable,
    /// Proxy Authentication Required (407)
    ProxyAuth,
    /// Request Timeout (408)
    ClientTimeout,
    /// Conflict (409)
    Conflict,
    /// Gone (410)
    Gone,
    /// Length Required (411)
    LengthRequired,
    /// Precondition Failed (412)
    PreconFailed,
    /// Payload Too Large (413)
    EntityTooLarge,
    /// URI Too Long (414)
    ReqTooLong,
    /// Unsupported Media Type (415)
    UnsupportedType,
    /// Internal Server Error (500)
    InternalError = 500,
    /// Not Implemented (501)
    NotImplemented,
    /// Bad Gateway (502)
    BadGateway,
    /// Service Unavailable (503)
    Unavailable,
    /// Gateway Timeout (504)
    GatewayTimeout,
    /// HTTP Version Not Supported (505)
    Version,
}
