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
        self.pin_mut().GetStatus().try_into().unwrap_or_default()
    }

    pub fn response(&mut self) -> Response {
        Response::from_ffi(self.pin_mut().GetResponse().into_ref().get_ref())
    }

    pub fn headers(&mut self) -> HashMap<String, String> {
        self.response().headers()
    }

    pub(crate) fn set_callback(&mut self, callback: impl RequestCallback + 'static) {
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

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
    use std::sync::Arc;

    use super::*;
    use crate::error::HttpClientError;
    use crate::wrapper::ffi::NewHttpClientRequest;
    const TEST_URL: &str = "https://www.w3cschool.cn/statics/demosource/movie.mp4";
    const LOCAL_URL: &str = "https://127.0.0.1";

    #[test]
    fn ut_task_from_http_request() {
        let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
            NewHttpClientRequest();
        cxx::let_cxx_string!(url = TEST_URL);
        request.pin_mut().SetURL(&url);
        cxx::let_cxx_string!(method = "GET");
        request.pin_mut().SetMethod(&method);
        let mut task = RequestTask::from_http_request(&request);
        assert!(matches!(task.status(), TaskStatus::Idle));
    }

    struct TestCallback {
        pub(crate) finished: Arc<AtomicBool>,
        pub(crate) response_code: Arc<AtomicU32>,
        pub(crate) error: Arc<AtomicU32>,
        pub(crate) result: Arc<AtomicU32>,
    }

    impl TestCallback {
        fn new(
            finished: Arc<AtomicBool>,
            response_code: Arc<AtomicU32>,
            error: Arc<AtomicU32>,
            result: Arc<AtomicU32>,
        ) -> Self {
            Self {
                finished,
                response_code,
                error,
                result,
            }
        }
    }

    impl RequestCallback for TestCallback {
        fn on_success(&mut self, response: Response) {
            self.response_code
                .store(response.status() as u32, Ordering::SeqCst);
            self.finished.store(true, Ordering::SeqCst);
        }

        fn on_fail(&mut self, error: HttpClientError) {
            self.error
                .store(error.code().clone() as u32, Ordering::SeqCst);
            self.finished.store(true, Ordering::SeqCst);
        }

        fn on_cancel(&mut self) {
            self.error.store(123456, Ordering::SeqCst);
            self.finished.store(true, Ordering::SeqCst);
        }

        fn on_data_receive(&mut self, data: &[u8], _task: RequestTask) {
            self.result.fetch_add(data.len() as u32, Ordering::SeqCst);
        }
    }

    #[test]
    fn ut_request_task_start_success() {
        let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
            NewHttpClientRequest();
        cxx::let_cxx_string!(url = TEST_URL);
        request.pin_mut().SetURL(&url);
        cxx::let_cxx_string!(method = "GET");
        request.pin_mut().SetMethod(&method);
        let mut task = RequestTask::from_http_request(&request);
        let finished = Arc::new(AtomicBool::new(false));
        let response_code = Arc::new(AtomicU32::new(0));
        let error = Arc::new(AtomicU32::new(0));
        let result = Arc::new(AtomicU32::new(0));
        let callback = TestCallback::new(
            finished.clone(),
            response_code.clone(),
            error.clone(),
            result.clone(),
        );
        task.set_callback(callback);
        task.start();
        while !finished.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert_eq!(response_code.load(Ordering::SeqCst), 200);
        assert_eq!(error.load(Ordering::SeqCst), 0);
        assert_eq!(
            result.load(Ordering::SeqCst),
            task.headers()
                .get("content-length")
                .unwrap()
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn ut_request_task_cancel() {
        let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
            NewHttpClientRequest();
        cxx::let_cxx_string!(url = TEST_URL);
        request.pin_mut().SetURL(&url);
        cxx::let_cxx_string!(method = "GET");
        request.pin_mut().SetMethod(&method);
        let mut task = RequestTask::from_http_request(&request);
        let finished = Arc::new(AtomicBool::new(false));
        let response_code = Arc::new(AtomicU32::new(0));
        let error = Arc::new(AtomicU32::new(0));
        let result = Arc::new(AtomicU32::new(0));
        let callback = TestCallback::new(
            finished.clone(),
            response_code.clone(),
            error.clone(),
            result.clone(),
        );
        task.set_callback(callback);
        task.start();
        std::thread::sleep(std::time::Duration::from_millis(1));
        task.cancel();
        while !finished.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert_eq!(error.load(Ordering::SeqCst), 123456);
    }

    #[test]
    fn ut_request_task_fail() {
        let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
            NewHttpClientRequest();
        cxx::let_cxx_string!(url = LOCAL_URL);
        request.pin_mut().SetURL(&url);
        cxx::let_cxx_string!(method = "GET");
        request.pin_mut().SetMethod(&method);
        let mut task = RequestTask::from_http_request(&request);
        let finished = Arc::new(AtomicBool::new(false));
        let response_code = Arc::new(AtomicU32::new(0));
        let error = Arc::new(AtomicU32::new(0));
        let result = Arc::new(AtomicU32::new(0));
        let callback = TestCallback::new(
            finished.clone(),
            response_code.clone(),
            error.clone(),
            result.clone(),
        );
        task.set_callback(callback);
        task.start();
        while !finished.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert_eq!(
            error.load(Ordering::SeqCst),
            crate::error::HttpErrorCode::HttpCouldntConnect as u32
        );
    }

    #[test]
    fn ut_request_task_connect_timeout() {
        let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
            NewHttpClientRequest();
        cxx::let_cxx_string!(url = "222.222.222.222");
        request.pin_mut().SetURL(&url);
        cxx::let_cxx_string!(method = "GET");
        request.pin_mut().SetMethod(&method);
        request.pin_mut().SetConnectTimeout(1);
        let mut task = RequestTask::from_http_request(&request);
        let finished = Arc::new(AtomicBool::new(false));
        let response_code = Arc::new(AtomicU32::new(0));
        let error = Arc::new(AtomicU32::new(0));
        let result = Arc::new(AtomicU32::new(0));
        let callback = TestCallback::new(
            finished.clone(),
            response_code.clone(),
            error.clone(),
            result.clone(),
        );
        task.set_callback(callback);
        task.start();
        while !finished.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert_eq!(
            error.load(Ordering::SeqCst),
            crate::error::HttpErrorCode::HttpOperationTimedout as u32
        );
    }

    #[test]
    fn ut_request_task_timeout() {
        let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
            NewHttpClientRequest();
        cxx::let_cxx_string!(url = TEST_URL);
        request.pin_mut().SetURL(&url);
        cxx::let_cxx_string!(method = "GET");
        request.pin_mut().SetMethod(&method);
        request.pin_mut().SetTimeout(1);
        let mut task = RequestTask::from_http_request(&request);
        let finished = Arc::new(AtomicBool::new(false));
        let response_code = Arc::new(AtomicU32::new(0));
        let error = Arc::new(AtomicU32::new(0));
        let result = Arc::new(AtomicU32::new(0));
        let callback = TestCallback::new(
            finished.clone(),
            response_code.clone(),
            error.clone(),
            result.clone(),
        );
        task.set_callback(callback);
        task.start();
        while !finished.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert_eq!(
            error.load(Ordering::SeqCst),
            crate::error::HttpErrorCode::HttpOperationTimedout as u32
        );
    }
}
