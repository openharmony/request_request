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

use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use netstack_rs::error::HttpClientError;
use netstack_rs::request::{Request, RequestCallback};
use netstack_rs::response::Response;
use netstack_rs::task::RequestTask;

use super::callback::PrimeCallback;
use super::common::{CommonCancel, CommonError, CommonResponse};
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

    fn on_fail(&mut self, error: HttpClientError) {
        self.common_fail(error);
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
}

pub(crate) struct DownloadTask;

impl DownloadTask {
    pub(super) fn run(input: DownloadRequest, callback: PrimeCallback) -> Arc<dyn CommonCancel> {
        let mut request = Request::new();
        request.url(input.url);
        if let Some(headers) = input.headers {
            for (key, value) in headers {
                request.header(key, value);
            }
        }
        callback.set_running();
        request.callback(callback);
        let mut task = request.build();
        task.start();
        Arc::new(CancelHandle::new(task))
    }
}

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

impl CommonCancel for CancelHandle {
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
}
