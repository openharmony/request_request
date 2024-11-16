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

use std::collections::HashMap;
use std::pin::Pin;

use cxx::SharedPtr;

use crate::request::RequestCallback;
use crate::response::Response;
use crate::wrapper::ffi::{HttpClientRequest, HttpClientTask, NewHttpClientTask, OnCallback};
use crate::wrapper::CallbackWrapper;

/// RequestTask
#[derive(Clone)]
pub struct RequestTask {
    inner: SharedPtr<HttpClientTask>,
}

unsafe impl Send for RequestTask {}
unsafe impl Sync for RequestTask {}

/// RequestTask status
#[derive(Debug, Default)]
pub enum TaskStatus {
    /// idle
    Idle,
    /// running
    #[default]
    Running,
}

impl RequestTask {
    pub(crate) fn from_http_request(request: &HttpClientRequest) -> Self {
        Self {
            inner: NewHttpClientTask(request),
        }
    }

    pub(crate) fn from_ffi(inner: SharedPtr<HttpClientTask>) -> Self {
        Self { inner }
    }

    /// start the request task
    pub fn start(&mut self) -> bool {
        self.pin_mut().Start()
    }

    /// cancel the request task
    pub fn cancel(&self) {
        self.pin_mut().Cancel();
    }

    /// get the request task status
    pub fn status(&mut self) -> TaskStatus {
        self.pin_mut()
            .GetStatus()
            .try_into()
            .map_err(|e| {})
            .unwrap_or_default()
    }

    pub fn response(&mut self) -> Response {
        Response::from_ffi(self.pin_mut().GetResponse().into_ref().get_ref())
    }

    pub fn headers(&mut self) -> HashMap<String, String> {
        self.response().headers()
    }

    pub(crate) fn callback(&mut self, callback: impl RequestCallback + 'static) {
        OnCallback(
            self.inner.clone(),
            Box::new(CallbackWrapper::from_callback(callback)),
        );
    }

    fn pin_mut(&self) -> Pin<&mut HttpClientTask> {
        let ptr = self.inner.as_ref().unwrap() as *const HttpClientTask as *mut HttpClientTask;
        unsafe { Pin::new_unchecked(ptr.as_mut().unwrap()) }
    }
}
