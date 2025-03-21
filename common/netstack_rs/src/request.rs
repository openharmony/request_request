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

use cxx::{let_cxx_string, UniquePtr};

use crate::error::HttpClientError;
use crate::response::Response;
use crate::task::RequestTask;
use crate::wrapper::ffi::{HttpClientRequest, NewHttpClientRequest, SetBody};
/// Builder for creating a Request.
pub struct Request<C: RequestCallback + 'static> {
    inner: UniquePtr<HttpClientRequest>,
    callback: Option<C>,
}

impl<C: RequestCallback> Request<C> {
    /// Create a new Request.
    pub fn new() -> Self {
        Self {
            inner: NewHttpClientRequest(),
            callback: None,
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

    /// Build the RequestTask.
    pub fn build(mut self) -> RequestTask {
        let mut task = RequestTask::from_http_request(&self.inner);
        if let Some(callback) = self.callback.take() {
            task.set_callback(Box::new(callback));
        }
        task
    }
}

/// RequestCallback
#[allow(unused_variables)]
pub trait RequestCallback {
    /// Called when the request is successful.
    fn on_success(&mut self, response: Response) {}
    /// Called when the request fails.
    fn on_fail(&mut self, error: HttpClientError) {}
    /// Called when the request is canceled.
    fn on_cancel(&mut self) {}
    /// Called when data is received.
    fn on_data_receive(&mut self, data: &[u8], task: RequestTask) {}
    /// Called when progress is made.
    fn on_progress(&mut self, dl_total: u64, dl_now: u64, ul_total: u64, ul_now: u64) {}
    /// Called when the task is restarted.
    fn on_restart(&mut self) {}
}

impl<C: RequestCallback> Default for Request<C> {
    fn default() -> Self {
        Self::new()
    }
}
