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

//! Module for handling HTTP responses.
//! 
//! This module provides types and functionality for working with HTTP responses,
//! including status codes, headers, and response data.

use std::collections::HashMap;
use std::pin::Pin;

use cxx::SharedPtr;

use crate::task::RequestTask;
use crate::wrapper::ffi::{GetHeaders, HttpClientResponse, HttpClientTask};

/// Represents an HTTP response from the client.
///
/// Provides access to response status codes and headers. The lifetime parameter `'a`
/// ensures the response data remains valid while this object is in use.
///
/// # Examples
///
/// ```
/// use netstack_rs::{RequestCallback, Response, ResponseCode};
///
/// struct MyCallback;
///
/// impl RequestCallback for MyCallback {
///     fn on_success(&mut self, response: Response) {
///         // Get the status code
///         let status = response.status();
///         println!("Response status: {:?}", status);
///         
///         // Check if the response was successful
///         if status == ResponseCode::Ok {
///             println!("Request succeeded!");
///         }
///         
///         // Access response headers
///         let headers = response.headers();
///         if let Some(content_type) = headers.get("content-type") {
///             println!("Content-Type: {}", content_type);
///         }
///     }
/// }
/// ```
pub struct Response<'a> {
    /// Internal representation of the response (either borrowed or shared)
    inner: ResponseInner<'a>,
}

impl<'a> Response<'a> {
    /// Gets the HTTP status code of the response.
    ///
    /// # Returns
    ///
    /// The response code as a `ResponseCode` enum value.
    /// Returns `ResponseCode::None` if the status code cannot be determined.
    pub fn status(&self) -> ResponseCode {
        let response = self.inner.to_response();
        // Attempt to convert the status code, defaulting to None if conversion fails
        response.GetResponseCode().try_into().unwrap_or_default()
    }

    /// Gets all response headers as a case-insensitive HashMap.
    ///
    /// Header names are converted to lowercase for consistent access.
    ///
    /// # Returns
    ///
    /// A HashMap where keys are lowercase header names and values are header values.
    ///
    /// # Safety
    ///
    /// This method uses unsafe code to work with the FFI layer. It assumes the
    /// underlying response pointer is valid and properly aligned.
    pub fn headers(&self) -> HashMap<String, String> {
        // Convert to mutable pointer for FFI compatibility
        let ptr = self.inner.to_response() as *const HttpClientResponse as *mut HttpClientResponse;
        // Safety: Assuming the pointer is valid and properly aligned
        let p = unsafe { Pin::new_unchecked(ptr.as_mut().unwrap()) };

        // Get headers from FFI and iterate as key-value pairs
        let mut headers = GetHeaders(p).into_iter();
        let mut ret = HashMap::new();
        loop {
            if let Some(key) = headers.next() {
                if let Some(value) = headers.next() {
                    // Convert header names to lowercase for case-insensitive access
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
    ///
    /// The caller must ensure the reference remains valid for the lifetime 'a.
    pub(crate) fn from_ffi(inner: &'a HttpClientResponse) -> Self {
        Self {
            inner: ResponseInner::Ref(inner),
        }
    }

    /// Creates a Response from a shared pointer to HttpClientTask.
    ///
    /// This version takes ownership of the shared pointer to ensure the response
    /// data remains valid for the lifetime of the Response object.
    pub(crate) fn from_shared(inner: SharedPtr<HttpClientTask>) -> Self {
        Self {
            inner: ResponseInner::Shared(inner),
        }
    }
}

/// Internal representation of an HTTP response.
///
/// Provides flexibility by supporting both borrowed references and owned shared pointers,
/// allowing efficient response handling in different contexts.
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
    ///
    /// # Returns
    ///
    /// A reference to the underlying `HttpClientResponse`.
    fn to_response(&self) -> &HttpClientResponse {
        match self {
            // Direct access for borrowed references
            ResponseInner::Ref(inner) => inner,
            // For shared pointers, access through the task's response property
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
/// Provides a type-safe way to work with HTTP status codes.
///
/// # Examples
///
/// ```
/// use netstack_rs::ResponseCode;
///
/// fn handle_response(status: ResponseCode) -> bool {
///     match status {
///         // Success codes
///         ResponseCode::Ok | ResponseCode::Created | ResponseCode::Accepted => {
///             println!("Request succeeded!");
///             true
///         }
///         // Redirect codes
///         ResponseCode::MovedPerm | ResponseCode::MovedTemp => {
///             println!("Resource moved");
///             false
///         }
///         // Client error codes
///         ResponseCode::NotFound => {
///             println!("Resource not found");
///             false
///         }
///         // Server error codes
///         ResponseCode::InternalError => {
///             println!("Server error occurred");
///             false
///         }
///         // Default case for any other status
///         _ => {
///             println!("Unexpected status: {:?}", status);
///             false
///         }
///     }
/// }
/// ```
#[derive(Clone, Default, PartialEq, Eq)]
pub enum ResponseCode {
    #[default]
    /// No response code available (0)
    None = 0,
    /// OK (200) - The request has succeeded
    Ok = 200,
    /// Created (201) - The request has been fulfilled and a new resource has been created
    Created,
    /// Accepted (202) - The request has been accepted for processing but not completed
    Accepted,
    /// Non-Authoritative Information (203) - The returned metadata is not authoritative
    NotAuthoritative,
    /// No Content (204) - The server successfully processed the request and has no content to send
    NoContent,
    /// Reset Content (205) - The server successfully processed the request and asks to reset the document view
    Reset,
    /// Partial Content (206) - The server is delivering only part of the resource due to a range header
    Partial,
    /// Multiple Choices (300) - The request has multiple possible responses
    MultChoice = 300,
    /// Moved Permanently (301) - The resource has been moved permanently
    MovedPerm,
    /// Found (302) - The resource has been temporarily moved to a different URI
    MovedTemp,
    /// See Other (303) - The response to the request can be found under a different URI
    SeeOther,
    /// Not Modified (304) - The resource has not been modified since the last request
    NotModified,
    /// Use Proxy (305) - The requested resource must be accessed through the proxy given
    UseProxy,
    /// Bad Request (400) - The server cannot or will not process the request due to a client error
    BadRequest = 400,
    /// Unauthorized (401) - The client must authenticate itself to get the requested response
    Unauthorized,
    /// Payment Required (402) - Reserved for future use
    PaymentRequired,
    /// Forbidden (403) - The client does not have access rights to the content
    Forbidden,
    /// Not Found (404) - The server cannot find the requested resource
    NotFound,
    /// Method Not Allowed (405) - The request method is known by the server but not supported
    BadMethod,
    /// Not Acceptable (406) - The server cannot produce a response matching the list of acceptable values
    NotAcceptable,
    /// Proxy Authentication Required (407) - The client must first authenticate itself with the proxy
    ProxyAuth,
    /// Request Timeout (408) - The server would like to shut down this unused connection
    ClientTimeout,
    /// Conflict (409) - The request conflicts with the current state of the server
    Conflict,
    /// Gone (410) - The requested resource is no longer available and will not be available again
    Gone,
    /// Length Required (411) - The server requires the request to be valid before processing
    LengthRequired,
    /// Precondition Failed (412) - One or more conditions in the request header fields evaluated to false
    PreconFailed,
    /// Payload Too Large (413) - The request is larger than the server is willing or able to process
    EntityTooLarge,
    /// URI Too Long (414) - The URI provided was too long for the server to process
    ReqTooLong,
    /// Unsupported Media Type (415) - The media format of the requested data is not supported
    UnsupportedType,
    /// Internal Server Error (500) - The server encountered an unexpected condition
    InternalError = 500,
    /// Not Implemented (501) - The server does not support the functionality required
    NotImplemented,
    /// Bad Gateway (502) - The server got an invalid response from the upstream server
    BadGateway,
    /// Service Unavailable (503) - The server is not ready to handle the request
    Unavailable,
    /// Gateway Timeout (504) - The server did not get a timely response from an upstream server
    GatewayTimeout,
    /// HTTP Version Not Supported (505) - The server does not support the HTTP protocol version
    Version,
}
