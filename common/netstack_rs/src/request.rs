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

use std::sync::Arc;

use cxx::{let_cxx_string, UniquePtr};
use request_utils::task_id::TaskId;

use crate::error::HttpClientError;
use crate::info::DownloadInfoMgr;
use crate::response::Response;
use crate::task::RequestTask;
use crate::wrapper::ffi::{HttpClientRequest, NewHttpClientRequest, SetBody, SetRequestSslType};
/// Builder for creating HTTP requests with configurable options.
///
/// This builder pattern allows for fluent configuration of HTTP requests
/// with various options like URL, method, headers, timeouts, and callbacks.
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
    /// Create a new Request.
    pub fn new() -> Self {
        Self {
            inner: NewHttpClientRequest(),
            callback: None,
            info_mgr: None,
            task_id: None,
        }
    }

    /// Set the URL for the request.
    pub fn url(&mut self, url: &str) -> &mut Self {
        let_cxx_string!(url = url);
        self.inner.pin_mut().SetURL(&url);
        self
    }

    /// Set the method for the request.
    pub fn method(&mut self, method: &str) -> &mut Self {
        let_cxx_string!(method = method);
        self.inner.pin_mut().SetMethod(&method);
        self
    }

    /// Set a header for the request.
    pub fn header(&mut self, key: &str, value: &str) -> &mut Self {
        let_cxx_string!(key = key);
        let_cxx_string!(value = value);
        self.inner.pin_mut().SetHeader(&key, &value);
        self
    }

    /// Sets the SSL/TLS type for the request.
    ///
    /// # Arguments
    /// * `ssl_type` - The type of SSL/TLS configuration to use (e.g., "tlsv1.2")
    pub fn ssl_type(&mut self, ssl_type: &str) -> &mut Self {
        let_cxx_string!(ssl_type = ssl_type);
        SetRequestSslType(self.inner.pin_mut(), &ssl_type);
        self
    }

    /// Sets the CA certificate path for SSL/TLS verification.
    ///
    /// # Arguments
    /// * `ca_path` - Path to the CA certificate file
    pub fn ca_path(&mut self, ca_path: &str) -> &mut Self {
        let_cxx_string!(ca_path = ca_path);
        self.inner.pin_mut().SetCaPath(&ca_path);
        self
    }

    /// Set the body for the request.
    pub fn body(&mut self, body: &[u8]) -> &mut Self {
        unsafe { SetBody(self.inner.pin_mut(), body.as_ptr(), body.len()) };
        self
    }

    /// Set a timeout for the request.
    pub fn timeout(&mut self, timeout: u32) -> &mut Self {
        self.inner.pin_mut().SetTimeout(timeout);
        self
    }

    /// Set a connect timeout for the request.
    pub fn connect_timeout(&mut self, timeout: u32) -> &mut Self {
        self.inner.pin_mut().SetConnectTimeout(timeout);
        self
    }

    /// Set a callback for the request.
    pub fn callback(&mut self, callback: C) -> &mut Self {
        self.callback = Some(callback);
        self
    }

    /// Sets the download information manager for tracking request metrics.
    ///
    /// # Arguments
    /// * `mgr` - Arc reference to the DownloadInfoMgr
    pub fn info_mgr(&mut self, mgr: Arc<DownloadInfoMgr>) -> &mut Self {
        self.info_mgr = Some(mgr);
        self
    }

    /// Sets the task identifier for this request.
    ///
    /// # Arguments
    /// * `task_id` - Unique identifier for the request task
    pub fn task_id(&mut self, task_id: TaskId) -> &mut Self {
        self.task_id = Some(task_id);
        self
    }

    /// Consumes the builder and creates a RequestTask.
    ///
    /// Returns `None` if the request could not be created.
    /// Transfers all configured callbacks and trackers to the new task.
    pub fn build(mut self) -> Option<RequestTask> {
        RequestTask::from_http_request(&self.inner).map(|mut task| {
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
/// All methods have default no-op implementations.
#[allow(unused_variables)]
pub trait RequestCallback {
    /// Called when the request completes successfully.
    ///
    /// # Arguments
    /// * `response` - The successful HTTP response
    fn on_success(&mut self, response: Response) {}

    /// Called when the request fails.
    ///
    /// # Arguments
    /// * `error` - The error that occurred
    fn on_fail(&mut self, error: HttpClientError) {}

    /// Called when the request is canceled by the user.
    fn on_cancel(&mut self) {}

    /// Called when new data is received in the response.
    ///
    /// # Arguments
    /// * `data` - The received data chunk
    /// * `task` - Reference to the ongoing request task
    fn on_data_receive(&mut self, data: &[u8], task: RequestTask) {}

    /// Called to report upload/download progress.
    ///
    /// # Arguments
    /// * `dl_total` - Total bytes to download (0 if unknown)
    /// * `dl_now` - Bytes downloaded so far
    /// * `ul_total` - Total bytes to upload (0 if unknown)
    /// * `ul_now` - Bytes uploaded so far
    fn on_progress(&mut self, dl_total: u64, dl_now: u64, ul_total: u64, ul_now: u64) {}

    /// Called when the task is being restarted (e.g., after a redirect).
    fn on_restart(&mut self) {}
}

impl<C: RequestCallback> Default for Request<C> {
    /// Creates a new Request with default settings.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod ut_request_set {
    include!("../tests/ut/ut_request_set.rs");
}
