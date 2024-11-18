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

use netstack_rs::request::Request;
use netstack_rs::task::RequestTask;

use super::common::DownloadCallback;
use crate::agent::DownloadRequest;

pub(crate) struct DownloadTask;

impl DownloadTask {
    pub(super) fn run(input: DownloadRequest, callback: DownloadCallback) -> CancelHandle {
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
        CancelHandle::new(task)
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

    pub(super) fn add_count(&self) {
        self.count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

impl CancelHandle {
    pub(super) fn cancel(&self) -> bool {
        if self.count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst) == 1 {
            self.inner.cancel();
            true
        } else {
            false
        }
    }
}
