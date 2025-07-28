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
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use cxx::SharedPtr;
use request_utils::error;
use request_utils::task_id::TaskId;

use crate::info::DownloadInfoMgr;
use crate::request::RequestCallback;
use crate::response::Response;
use crate::wrapper::ffi::{HttpClientRequest, HttpClientTask, NewHttpClientTask, OnCallback};
use crate::wrapper::CallbackWrapper;

/// RequestTask
#[derive(Clone)]
pub struct RequestTask {
    inner: Arc<Mutex<SharedPtr<HttpClientTask>>>,
    reset: Arc<AtomicBool>,
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
    pub(crate) fn from_http_request(request: &HttpClientRequest) -> Option<Self> {
        let http_task = NewHttpClientTask(request);
        if http_task.is_null() {
            error!("from_http_request NewHttpClientTask return null.");
            return None;
        }
        Some(Self {
            inner: Arc::new(Mutex::new(http_task)),
            reset: Arc::new(AtomicBool::new(false)),
        })
    }

    pub(crate) fn from_ffi(inner: SharedPtr<HttpClientTask>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
            reset: Arc::new(AtomicBool::new(false)),
        }
    }

    /// start the request task
    pub fn start(&mut self) -> bool {
        unsafe {
            let ptr = self.inner.lock().unwrap().as_ref().unwrap() as *const HttpClientTask
                as *mut HttpClientTask;
            Pin::new_unchecked(ptr.as_mut().unwrap()).Start()
        }
    }

    /// cancel the request task
    pub fn cancel(&self) {
        let task = self.inner.lock().unwrap().clone();
        Self::pin_mut(&task).Cancel();
    }

    /// reset the task
    pub fn reset(&self) {
        if self
            .reset
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            self.cancel();
        }
    }

    /// get the request task status
    pub fn status(&mut self) -> TaskStatus {
        let task = self.inner.lock().unwrap().clone();
        Self::pin_mut(&task)
            .GetStatus()
            .try_into()
            .unwrap_or_default()
    }

    pub fn response(&mut self) -> Response {
        let task = self.inner.lock().unwrap().clone();
        Response::from_shared(task)
    }

    pub fn headers(&mut self) -> HashMap<String, String> {
        self.response().headers()
    }

    pub(crate) fn set_callback(
        &mut self,
        callback: Box<dyn RequestCallback + 'static>,
        info_mgr: Arc<DownloadInfoMgr>,
        task_id: TaskId,
    ) {
        let task = self.inner.lock().unwrap();
        OnCallback(
            &task,
            Box::new(CallbackWrapper::from_callback(
                callback,
                self.reset.clone(),
                Arc::downgrade(&self.inner),
                task_id,
                info_mgr,
                0,
            )),
        );
    }

    pub(crate) fn pin_mut(ptr: &SharedPtr<HttpClientTask>) -> Pin<&mut HttpClientTask> {
        let ptr = ptr.as_ref().unwrap() as *const HttpClientTask as *mut HttpClientTask;
        unsafe { Pin::new_unchecked(ptr.as_mut().unwrap()) }
    }
}

#[cfg(test)]
mod ut_task {
    include!("../tests/ut/ut_task.rs");
}
