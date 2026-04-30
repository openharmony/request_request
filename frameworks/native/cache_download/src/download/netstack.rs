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

use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use netstack_rs::error::HttpClientError;
use netstack_rs::info::{DownloadInfo, DownloadInfoMgr};
use netstack_rs::request::{Request, RequestCallback};
use netstack_rs::response::Response;
use netstack_rs::task::RequestTask;
use netstack_rs::{DEFAULT_MAX_RETRY_COUNT, DEFAULT_NETWORK_CHECK_TIMEOUT};
use request_utils::error;

use super::callback::PrimeCallback;
use super::common::{CommonError, CommonHandle, CommonResponse};
use crate::services::DownloadRequest;

impl<'a> CommonResponse for Response<'a> {
    fn code(&self) -> u32 {
        self.status() as u32
    }
}

impl CommonError for HttpClientError {
    fn code(&self) -> i32 {
        self.code().clone() as i32
    }

    fn msg(&self) -> String {
        self.msg().to_string()
    }
}

impl RequestCallback for PrimeCallback {
    fn on_success(&mut self, response: Response) {
        self.common_success(response);
    }

    fn on_fail(&mut self, error: HttpClientError, info: DownloadInfo) {
        self.common_fail(error, info);
    }

    fn on_cancel(&mut self) {
        self.common_cancel();
    }

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

    fn on_progress(&mut self, dl_total: u64, dl_now: u64, ul_total: u64, ul_now: u64) {
        self.common_progress(dl_total, dl_now, ul_total, ul_now);
    }

    fn on_restart(&mut self) {
        self.common_restart();
    }
}

/// Task handler for netstack-based download operations.
pub(crate) struct DownloadTask;

impl DownloadTask {
    pub(super) fn run(
        input: DownloadRequest,
        callback: PrimeCallback,
        info_mgr: Arc<DownloadInfoMgr>,
    ) -> Option<Arc<dyn CommonHandle>> {
        let network_check_timeout = callback
            .network_check_timeout()
            .unwrap_or(DEFAULT_NETWORK_CHECK_TIMEOUT);
        let max_retry = callback.max_retry().unwrap_or(DEFAULT_MAX_RETRY_COUNT);
        let http_total_timeout = callback.http_total_timeout();
        let task_id = callback.task_id();

        // Mark task as running
        callback.set_running();

        // Create and configure request
        let mut request: Request<PrimeCallback> = Request::new();
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
        if let Some(http_total_timeout) = http_total_timeout {
            request.timeout(http_total_timeout * 1000);
        }
        request.max_retry(max_retry);
        request.network_check_timeout(network_check_timeout);

        // Setup request with callback and info manager
        request.task_id(task_id.clone());
        request.callback(callback);
        request.info_mgr(info_mgr);

        // Start task directly without initial network check
        // Network check will be handled by retry mechanism if task fails
        Self::start_task(request)
    }

    /// Build and start the download task.
    fn start_task(request: Request<PrimeCallback>) -> Option<Arc<dyn CommonHandle>> {
        // Get task_id before building (for logging)
        let task_id = request.task_id_ref().cloned();
        match request.build() {
            Some(mut task) => {
                if task.start() {
                    Some(Arc::new(CancelHandle::new(task)))
                } else {
                    if let Some(task_id) = task_id {
                        error!(
                            "Netstack HttpClientTask start task {:?} failed.",
                            task_id.brief()
                        );
                    }
                    None
                }
            }
            None => None,
        }
    }
}

/// Handle for managing and canceling netstack download tasks.
#[derive(Clone)]
pub struct CancelHandle {
    inner: RequestTask,
    count: Arc<AtomicUsize>,
}

impl CancelHandle {
    fn new(inner: RequestTask) -> Self {
        Self {
            inner,
            count: Arc::new(AtomicUsize::new(1)),
        }
    }
}

impl CommonHandle for CancelHandle {
    fn cancel(&self) -> bool {
        if self.count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst) == 1 {
            self.inner.cancel();
            true
        } else {
            false
        }
    }

    fn add_count(&self) {
        self.count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    fn reset(&self) {
        self.inner.reset();
    }
}
