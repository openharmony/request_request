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

//! Netstack client integration for cache download operations.
//!
//! This module provides integration with the netstack HTTP client library for performing
//! download operations. It implements required traits and provides task management
//! functionality.

use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use netstack_rs::error::HttpClientError;
use netstack_rs::info::{DownloadInfo, DownloadInfoMgr};
use netstack_rs::request::{Request, RequestCallback};
use netstack_rs::response::Response;
use netstack_rs::task::RequestTask;

use super::callback::PrimeCallback;
use super::common::{CommonError, CommonHandle, CommonResponse};
use crate::services::DownloadRequest;

impl<'a> CommonResponse for Response<'a> {
    /// Returns the HTTP status code of the response.
    ///
    /// # Returns
    /// The HTTP status code as an unsigned 32-bit integer.
    fn code(&self) -> u32 {
        self.status() as u32
    }
}

impl CommonError for HttpClientError {
    /// Returns the error code of the HTTP client error.
    ///
    /// # Returns
    /// The error code as a signed 32-bit integer.
    fn code(&self) -> i32 {
        self.code().clone() as i32
    }

    /// Returns the error message of the HTTP client error.
    ///
    /// # Returns
    /// A string containing the error message.
    fn msg(&self) -> String {
        self.msg().to_string()
    }
}

impl RequestCallback for PrimeCallback {
    /// Handles successful response from the HTTP client.
    ///
    /// # Parameters
    /// - `response`: The successful HTTP response object.
    fn on_success(&mut self, response: Response) {
        self.common_success(response);
    }

    /// Handles failure response from the HTTP client.
    ///
    /// # Parameters
    /// - `error`: The HTTP client error object.
    fn on_fail(&mut self, error: HttpClientError, info: DownloadInfo) {
        self.common_fail(error, info);
    }

    /// Handles cancellation notification from the HTTP client.
    fn on_cancel(&mut self) {
        self.common_cancel();
    }

    /// Handles data received notification from the HTTP client.
    ///
    /// Extracts content length information from headers if available and not chunked encoding.
    ///
    /// # Parameters
    /// - `data`: The received data buffer.
    /// - `task`: The request task containing response metadata.
    fn on_data_receive(&mut self, data: &[u8], mut task: RequestTask) {
        let f = || {
            let headers = task.headers();
            let is_chunked = headers
                .get("transfer-encoding")
                .map(|s| s == "chunked")
                .unwrap_or(false);
            if is_chunked {
                None
            } else {
                headers
                    .get("content-length")
                    .and_then(|s| s.parse::<usize>().ok())
            }
        };

        self.common_data_receive(data, f)
    }

    /// Handles progress update notification from the HTTP client.
    ///
    /// # Parameters
    /// - `dl_total`: Total bytes to download.
    /// - `dl_now`: Bytes downloaded so far.
    /// - `ul_total`: Total bytes to upload.
    /// - `ul_now`: Bytes uploaded so far.
    fn on_progress(&mut self, dl_total: u64, dl_now: u64, ul_total: u64, ul_now: u64) {
        self.common_progress(dl_total, dl_now, ul_total, ul_now);
    }

    /// Handles restart notification from the HTTP client.
    fn on_restart(&mut self) {
        self.common_restart();
    }
}

/// Task handler for netstack-based download operations.
///
/// Provides functionality to create and execute download tasks using the netstack HTTP client.
pub(crate) struct DownloadTask;

impl DownloadTask {
    /// Creates and starts a new download task using the netstack HTTP client.
    ///
    /// # Parameters
    /// - `input`: The download request configuration.
    /// - `callback`: The callback handler for download events.
    /// - `info_mgr`: Manager for download information.
    ///
    /// # Returns
    /// An `Arc<dyn CommonHandle>` for controlling the download task if successful,
    /// otherwise `None`.
    pub(super) fn run(
        input: DownloadRequest,
        callback: PrimeCallback,
        info_mgr: Arc<DownloadInfoMgr>,
    ) -> Option<Arc<dyn CommonHandle>> {
        let mut request = Request::new();
        request.url(input.url);
        if let Some(headers) = input.headers {
            for (key, value) in headers {
                request.header(key, value);
            }
        }
        if let Some(ssl_type) = input.ssl_type {
            request.ssl_type(ssl_type);
        }
        if let Some(ca_path) = input.ca_path {
            request.ca_path(ca_path);
        }
        callback.set_running();
        request.task_id(callback.task_id());
        let task_id = callback.task_id();
        request.callback(callback);
        request.info_mgr(info_mgr);
        match request.build() {
            Some(mut task) => {
                if task.start() {
                    Some(Arc::new(CancelHandle::new(task)))
                } else {
                    error!(
                        "Netstack HttpClientTask start task {:?} failed.",
                        task_id.brief()
                    );
                    None
                }
            }
            None => None,
        }
    }
}

/// Handle for managing and canceling netstack download tasks.
///
/// Provides reference counting and cancellation functionality for download operations.
#[derive(Clone)]
pub struct CancelHandle {
    /// The underlying netstack request task.
    inner: RequestTask,
    /// Reference counter for tracking active handles.
    count: Arc<AtomicUsize>,
}

impl CancelHandle {
    /// Creates a new cancel handle for a netstack request task.
    ///
    /// # Parameters
    /// - `inner`: The netstack request task to wrap.
    fn new(inner: RequestTask) -> Self {
        Self {
            inner,
            count: Arc::new(AtomicUsize::new(1)),
        }
    }
}

impl CommonHandle for CancelHandle {
    /// Cancels the download task when the last reference is released.
    ///
    /// Uses atomic operations to ensure thread-safe reference counting.
    ///
    /// # Returns
    /// `true` if cancellation was performed (last reference), `false` otherwise.
    fn cancel(&self) -> bool {
        // Only cancel when the last reference is released
        if self.count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst) == 1 {
            self.inner.cancel();
            true
        } else {
            false
        }
    }

    /// Increments the reference count for this handle.
    ///
    /// Uses atomic operations to ensure thread-safe reference counting.
    fn add_count(&self) {
        self.count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// Resets the underlying download task.
    fn reset(&self) {
        self.inner.reset();
    }
}
