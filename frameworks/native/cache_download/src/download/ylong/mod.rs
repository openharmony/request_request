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

//! Ylong HTTP client-based download implementation.
//! 
//! This module provides a download implementation using the Ylong HTTP client, including:
//! - A download task runner
//! - Progress reporting and cancellation mechanisms
//! - Response handling and error conversion
//! - Integration with the common download interface

mod client;

use std::collections::HashMap;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

use client::client;
use ylong_http_client::async_impl::{
    Body, DownloadOperator, Downloader, PercentEncoder, RequestBuilder,
};
use ylong_http_client::{ErrorKind, HttpClientError, StatusCode};

use super::callback::PrimeCallback;
use super::common::{CommonHandle, CommonError, CommonResponse};
use crate::services::DownloadRequest;

/// Implements the `CommonError` trait for the `HttpClientError` type.
///
/// Converts HTTP client errors to the common error format used by the download system,
/// providing error code and message information.
impl CommonError for HttpClientError {
    fn code(&self) -> i32 {
        self.error_kind() as i32
    }

    fn msg(&self) -> String {
        self.to_string()
    }
}

/// Implements the `CommonResponse` trait for the `Response` type.
///
/// Provides access to the HTTP status code from the response.
impl CommonResponse for Response {
    fn code(&self) -> u32 {
        self.status().as_u16() as u32
    }
}

/// Download operator that processes download events and reports progress.
///
/// Handles data reception and progress updates during a download operation,
/// forwarding them to the provided callback. Also supports download cancellation.
struct Operator<'a> {
    /// Callback to report download events to
    callback: &'a mut PrimeCallback,
    /// Flag used to signal download cancellation
    abort_flag: Arc<AtomicBool>,
    /// HTTP headers received from the response
    headers: HashMap<String, String>,
}

/// Implements the `DownloadOperator` trait for processing download events.
///
/// Handles both data reception and progress updates during a download operation.
impl<'a> DownloadOperator for Operator<'a> {
    /// Processes downloaded data chunks and reports them to the callback.
    fn poll_download(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        data: &[u8],
    ) -> Poll<Result<usize, HttpClientError>> {
        let me = self.get_mut();
        me.callback.common_data_receive(data, || {
            me.headers.get("content-length").and_then(|v| v.parse().ok())
        });
        Poll::Ready(Ok(data.len()))
    }

    /// Updates download progress and checks for cancellation requests.
    fn poll_progress(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        downloaded: u64,
        total: Option<u64>,
    ) -> Poll<Result<(), HttpClientError>> {
        let me = self.get_mut();
        me.callback
            .common_progress(total.unwrap_or_default(), downloaded, 0, 0);
        
        // Check if download has been requested to abort
        if me.abort_flag.load(Ordering::Acquire) {
            Poll::Ready(Err(HttpClientError::user_aborted()))
        } else {
            Poll::Ready(Ok(()))
        }
    }
}

/// Task handler for managing Ylong HTTP client downloads.
///
/// Provides functionality to run download tasks with progress reporting and cancellation support.
pub struct DownloadTask;

impl DownloadTask {
    /// Runs a download task with the specified request and callback.
    ///
    /// Encodes the URL, sets up the download context, and spawns an asynchronous task to perform
    /// the actual download. Returns a handle that can be used to cancel the download.
    ///
    /// # Parameters
    /// - `request`: Download request containing URL and headers
    /// - `callback`: Callback to receive download events
    ///
    /// # Returns
    /// A handle that implements `CommonHandle` for download cancellation
    pub(super) fn run(
        request: DownloadRequest,
        mut callback: PrimeCallback,
    ) -> Arc<dyn CommonHandle> {
        // Encode the URL to handle special characters
        let url = match PercentEncoder::encode(request.url) {
            Ok(url) => url,
            Err(e) => {
                callback.common_fail(e);
                return Arc::new(CancelHandle::new(Arc::new(AtomicBool::new(false))));
            }
        };
        
        // Signal that the download is starting
        callback.set_running();
        
        // Create a cancellation flag and handle
        let flag = Arc::new(AtomicBool::new(false));
        let handle = Arc::new(CancelHandle::new(flag.clone()));
        
        // Process request headers if provided
        let mut headers = None;
        if let Some(h) = request.headers {
            headers = Some(
                h.iter()
                    .map(|a| (a.0.to_string(), a.1.to_string()))
                    .collect(),
            );
        }
        
        // Spawn an asynchronous task to perform the download
        ylong_runtime::spawn(async move {
            if let Err(e) = download(url, headers, &mut callback, flag).await {
                // Handle errors based on their type
                if e.error_kind() == ErrorKind::UserAborted {
                    callback.common_cancel();
                } else {
                    callback.common_fail(e);
                }
            }
        });
        
        handle
    }
}

/// Performs an asynchronous HTTP download operation.
///
/// Creates and sends an HTTP GET request, processes the response, and streams the
/// downloaded data through the provided callback. Supports cancellation through
/// the abort flag.
///
/// # Parameters
/// - `url`: URL to download from
/// - `headers`: Optional HTTP headers to include in the request
/// - `callback`: Callback to receive download events
/// - `abort_flag`: Flag to signal download cancellation
///
/// # Returns
/// `Ok(())` if the download completed successfully, otherwise an error
pub async fn download(
    url: String,
    headers: Option<Vec<(String, String)>>,
    callback: &mut PrimeCallback,
    abort_flag: Arc<AtomicBool>,
) -> Result<(), HttpClientError> {
    // Create a GET request for the specified URL
    let mut request = RequestBuilder::new().url(url.as_str()).method("GET");

    // Add headers if provided
    if let Some(header) = headers {
        for (k, v) in header {
            request = request.append_header(k.as_str(), v.as_str());
        }
    }
    
    // Set empty body for GET request
    let request = request.body(Body::empty())?;

    // Send the request using the configured client
    let response = client().request(request).await?;
    let status = response.status();

    // Create download operator with the callback and headers
    let operator = Operator {
        callback: callback,
        abort_flag: abort_flag,
        headers: response
            .headers()
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string().unwrap()))
            .collect(),
    };
    
    // Build and run the downloader
    let mut downloader = Downloader::builder()
        .body(response)
        .operator(operator)
        .build();
    downloader.download().await?;

    // Notify the callback of successful completion
    let response = Response { status: status };
    callback.common_success(response);
    Ok(())
}

/// HTTP response wrapper containing status code information.
///
/// Provides a simplified view of the HTTP response for the download system.
pub struct Response {
    status: StatusCode,
}

impl Response {
    /// Returns the HTTP status code of the response.
    pub fn status(&self) -> StatusCode {
        self.status
    }
}

/// Handle for canceling a download operation.
///
/// Implements `CommonHandle` to provide download cancellation functionality with
/// reference counting to ensure proper resource management.
pub struct CancelHandle {
    /// Atomic flag used to signal cancellation
    inner: Arc<AtomicBool>,
    /// Reference count to track active handles
    count: AtomicUsize,
}

impl CancelHandle {
    /// Creates a new cancel handle with the specified atomic flag.
    ///
    /// # Parameters
    /// - `inner`: Atomic flag used to signal cancellation
    fn new(inner: Arc<AtomicBool>) -> Self {
        Self {
            inner,
            count: (AtomicUsize::new(1)),
        }
    }
}

/// Implements `CommonHandle` for canceling downloads with reference counting.
impl CommonHandle for CancelHandle {
    /// Attempts to cancel the download operation.
    ///
    /// Only cancels the download if this is the last active reference to the handle.
    /// This ensures that the download isn't canceled prematurely if the handle is
    /// shared across multiple consumers.
    ///
    /// # Returns
    /// `true` if the download was canceled, `false` if there are still active references
    fn cancel(&self) -> bool {
        // Only cancel if this is the last reference
        if self.count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst) == 1 {
            self.inner.store(true, Ordering::Release);
            true
        } else {
            false
        }
    }

    /// Increments the reference count for this handle.
    ///
    /// Allows sharing the handle across multiple consumers without prematurely canceling
    /// the download when one consumer calls `cancel`.
    fn add_count(&self) {
        self.count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}
