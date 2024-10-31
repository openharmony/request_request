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

use netstack_rs::request::Request;
use netstack_rs::task::RequestTask;

use super::common::DownloadCallback;

pub(crate) struct DownloadTask;

impl DownloadTask {
    pub(crate) fn run(url: &str, callback: DownloadCallback) -> CancelHandle {
        let mut request = Request::new();
        request.url(url);
        request.callback(callback);
        let mut task = request.build();
        task.start();
        CancelHandle { inner: task }
    }
}

#[derive(Clone)]
pub struct CancelHandle {
    inner: RequestTask,
}

impl CancelHandle {
    pub(crate) fn cancel(&mut self) {
        self.inner.cancel();
    }
}
