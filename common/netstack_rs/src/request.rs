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

//! Module for constructing and managing HTTP requests.
//!
//! This module provides a builder pattern implementation for creating HTTP requests
//! with configurable options and callbacks for handling various request events.

use std::sync::Arc;

use cxx::{let_cxx_string, UniquePtr};
use request_utils::task_id::TaskId;

use crate::error::HttpClientError;
use crate::info::{DownloadInfo, DownloadInfoMgr};
use crate::response::Response;
use crate::task::RequestTask;
use crate::wrapper::ffi::{HttpClientRequest, NewHttpClientRequest, SetBody, SetRequestSslType};
/// Builder for creating HTTP requests with configurable options.
///
/// Provides a fluent interface for configuring and building HTTP requests
/// with various options including URL, method, headers, timeouts, and event callbacks.
///
/// # Examples
///
/// ```
/// use netstack_rs::{Request, RequestCallback};
///
/// struct MyCallback;
/// impl RequestCallback for MyCallback {}
///
/// // Create a simple GET request
/// let request = Request::<MyCallback>::new()
///     .url("https://example.com")
///     .method("GET")
///     .timeout(30000)
///     .build();
/// ```
pub struct Request<C: RequestCallback + 'static> {
    /// Underlying HTTP request object (FFI wrapper)
    inner: UniquePtr<HttpClientRequest>,
    /// Optional callback to handle request events
    callback: Option<C>,
    /// Optional download information manager for tracking performance metrics
    info_mgr: Option<Arc<DownloadInfoMgr>>,
    /// Optional task identifier for request tracking
    task_id: Option<TaskId>,
}

impl<C: RequestCallback> Request<C> {
    /// Creates a new HTTP request builder with default settings.
    ///
    /// Initializes with an empty URL, default method (usually GET),
    /// no headers, default timeouts, and no callback or tracking configured.
    pub fn new() -> Self {
        Self {
            inner: NewHttpClientRequest(),
            callback: None,
            info_mgr: None,
            task_id: None,
        }
    }

    /// Sets the URL for the request.
    ///
    /// # Arguments
    ///
    /// * `url` - The target URL for the HTTP request
    ///
    /// # Returns
    ///
    /// A mutable reference to `self` for method chaining
    pub fn url(&mut self, url: &str) -> &mut Self {
        let_cxx_string!(url = url);
        self.inner.pin_mut().SetURL(&url);
        self
    }

    /// Sets the HTTP method for the request.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method (e.g., "GET", "POST", "PUT")
    ///
    /// # Returns
    ///
    /// A mutable reference to `self` for method chaining
    pub fn method(&mut self, method: &str) -> &mut Self {
        let_cxx_string!(method = method);
        self.inner.pin_mut().SetMethod(&method);
        self
    }

    /// Adds or sets a header for the request.
    ///
    /// Multiple calls with the same key will typically overwrite previous values
    /// depending on the underlying implementation.
    ///
    /// # Arguments
    ///
    /// * `key` - The header name
    /// * `value` - The header value
    ///
    /// # Returns
    ///
    /// A mutable reference to `self` for method chaining
    pub fn header(&mut self, key: &str, value: &str) -> &mut Self {
        let_cxx_string!(key = key);
        let_cxx_string!(value = value);
        self.inner.pin_mut().SetHeader(&key, &value);
        self
    }

    /// Sets the SSL/TLS type for the request.
    ///
    /// # Arguments
    ///
    /// * `ssl_type` - The type of SSL/TLS configuration to use (e.g., "tlsv1.2")
    ///
    /// # Returns
    ///
    /// A mutable reference to `self` for method chaining
    pub fn ssl_type(&mut self, ssl_type: &str) -> &mut Self {
        let_cxx_string!(ssl_type = ssl_type);
        SetRequestSslType(self.inner.pin_mut(), &ssl_type);
        self
    }

    /// Sets the CA certificate path for SSL/TLS verification.
    ///
    /// # Arguments
    ///
    /// * `ca_path` - Path to the CA certificate file
    ///
    /// # Returns
    ///
    /// A mutable reference to `self` for method chaining
    pub fn ca_path(&mut self, ca_path: &str) -> &mut Self {
        let_cxx_string!(ca_path = ca_path);
        self.inner.pin_mut().SetCaPath(&ca_path);
        self
    }

    /// Sets the request body as raw bytes.
    ///
    /// # Arguments
    ///
    /// * `body` - The request body content as a byte slice
    ///
    /// # Returns
    ///
    /// A mutable reference to `self` for method chaining
    ///
    /// # Safety
    ///
    /// This function uses an unsafe FFI call to transfer the body to the underlying
    /// C++ implementation. The implementation must handle the pointer correctly
    /// and not use it after the request is built or the body slice is dropped.
    pub fn body(&mut self, body: &[u8]) -> &mut Self {
        unsafe { SetBody(self.inner.pin_mut(), body.as_ptr(), body.len()) };
        self
    }

    /// Sets the total timeout for the entire request in milliseconds.
    ///
    /// # Arguments
    ///
    /// * `timeout` - Maximum time in milliseconds to wait for the request to complete
    ///
    /// # Returns
    ///
    /// A mutable reference to `self` for method chaining
    pub fn timeout(&mut self, timeout: u32) -> &mut Self {
        self.inner.pin_mut().SetTimeout(timeout);
        self
    }

    /// Sets the timeout for establishing the connection in milliseconds.
    ///
    /// # Arguments
    ///
    /// * `timeout` - Maximum time in milliseconds to wait for connection establishment
    ///
    /// # Returns
    ///
    /// A mutable reference to `self` for method chaining
    pub fn connect_timeout(&mut self, timeout: u32) -> &mut Self {
        self.inner.pin_mut().SetConnectTimeout(timeout);
        self
    }

    /// Sets the callback handler for request events.
    ///
    /// # Arguments
    ///
    /// * `callback` - The callback implementation to handle request events
    ///
    /// # Returns
    ///
    /// A mutable reference to `self` for method chaining
    pub fn callback(&mut self, callback: C) -> &mut Self {
        self.callback = Some(callback);
        self
    }

    /// Sets the download information manager for tracking request metrics.
    ///
    /// # Arguments
    ///
    /// * `mgr` - Arc reference to the DownloadInfoMgr
    ///
    /// # Returns
    ///
    /// A mutable reference to `self` for method chaining
    pub fn info_mgr(&mut self, mgr: Arc<DownloadInfoMgr>) -> &mut Self {
        self.info_mgr = Some(mgr);
        self
    }

    /// Sets the task identifier for this request.
    ///
    /// # Arguments
    ///
    /// * `task_id` - Unique identifier for the request task
    ///
    /// # Returns
    ///
    /// A mutable reference to `self` for method chaining
    pub fn task_id(&mut self, task_id: TaskId) -> &mut Self {
        self.task_id = Some(task_id);
        self
    }

    /// Consumes the builder and creates a RequestTask.
    ///
    /// # Returns
    ///
    /// * `Some(RequestTask)` if the task was successfully created
    /// * `None` if the request could not be created
    ///
    /// # Notes
    ///
    /// Transfers all configured callbacks and trackers to the new task. If a callback,
    /// info manager, and task ID are all provided, they are set together on the task.
    pub fn build(mut self) -> Option<RequestTask> {
        RequestTask::from_http_request(&self.inner).map(|mut task| {
            // Transfer ownership of callback, info_mgr, and task_id to the task if all are present
            if let (Some(callback), Some(mgr), Some(task_id)) = (
                self.callback.take(),
                self.info_mgr.take(),
                self.task_id.take(),
            ) {
                task.set_callback(Box::new(callback), mgr, task_id);
            }
            task
        })
    }
}

/// Trait defining callbacks for HTTP request events.
///
/// Implement this trait to handle various stages and outcomes of HTTP requests.
/// All methods have default no-op implementations, so you only need to override
/// the methods you're interested in.
///
/// # Examples
///
/// ```
/// use netstack_rs::{RequestCallback, Response, HttpClientError, DownloadInfo};
///
/// struct MyDownloadHandler {
///     bytes_received: u64,
/// }
///
/// impl RequestCallback for MyDownloadHandler {
///     fn on_success(&mut self, response: Response) {
///         println!("Download completed successfully with status: {}", response.status_code());
///     }
///     
///     fn on_fail(&mut self, error: HttpClientError, _info: DownloadInfo) {
///         println!("Download failed: {:?}", error);
///     }
///     
///     fn on_progress(&mut self, dl_total: u64, dl_now: u64, _ul_total: u64, _ul_now: u64) {
///         self.bytes_received = dl_now;
///         if dl_total > 0 {
///             let percent = (dl_now * 100) / dl_total;
///             println!("Progress: {}%", percent);
///         }
///     }
/// }
/// ```
#[allow(unused_variables)]
pub trait RequestCallback {
    /// Called when the request completes successfully.
    ///
    /// # Arguments
    ///
    /// * `response` - The successful HTTP response containing status code, headers, and body
    fn on_success(&mut self, response: Response) {}

    /// Called when the request fails.
    ///
    /// # Arguments
    ///
    /// * `error` - The error that occurred during the request
    fn on_fail(&mut self, error: HttpClientError, info: DownloadInfo) {}

    /// Called when the request is canceled by the user.
    ///
    /// This callback is triggered when the request is explicitly canceled,
    /// not when it times out or fails for other reasons.
    fn on_cancel(&mut self) {}

    /// Called when new data is received in the response.
    ///
    /// # Arguments
    ///
    /// * `data` - The received data chunk
    /// * `task` - Reference to the ongoing request task, which can be used to control
    ///   the request (e.g., cancel it)
    fn on_data_receive(&mut self, data: &[u8], task: RequestTask) {}

    /// Called to report upload/download progress.
    ///
    /// # Arguments
    ///
    /// * `dl_total` - Total bytes to download (0 if unknown)
    /// * `dl_now` - Bytes downloaded so far
    /// * `ul_total` - Total bytes to upload (0 if unknown)
    /// * `ul_now` - Bytes uploaded so far
    fn on_progress(&mut self, dl_total: u64, dl_now: u64, ul_total: u64, ul_now: u64) {}

    /// Called when the task is being restarted (e.g., after a redirect).
    ///
    /// This can be useful for resetting state before a request continues execution
    /// after being restarted due to a redirect or retry.
    fn on_restart(&mut self) {}
}

impl<C: RequestCallback> Default for Request<C> {
    /// Creates a new Request with default settings.
    ///
    /// Equivalent to calling `Request::new()`.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod ut_request_set {
    include!("../tests/ut/ut_request_set.rs");
}
