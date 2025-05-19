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
    pub(crate) fn from_http_request(request: &HttpClientRequest) -> Self {
        Self {
            inner: Arc::new(Mutex::new(NewHttpClientTask(request))),
            reset: Arc::new(AtomicBool::new(false)),
        }
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

    pub(crate) fn set_callback(&mut self, callback: Box<dyn RequestCallback + 'static>) {
        let task = self.inner.lock().unwrap();
        OnCallback(
            &task,
            Box::new(CallbackWrapper::from_callback(
                callback,
                self.reset.clone(),
                Arc::downgrade(&self.inner),
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
mod test {
    use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
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
        let callback = Box::new(TestCallback::new(
            finished.clone(),
            response_code.clone(),
            error.clone(),
            result.clone(),
        ));
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
        let callback = Box::new(TestCallback::new(
            finished.clone(),
            response_code.clone(),
            error.clone(),
            result.clone(),
        ));
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
        let callback = Box::new(TestCallback::new(
            finished.clone(),
            response_code.clone(),
            error.clone(),
            result.clone(),
        ));
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
        let callback = Box::new(TestCallback::new(
            finished.clone(),
            response_code.clone(),
            error.clone(),
            result.clone(),
        ));
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
        let callback = Box::new(TestCallback::new(
            finished.clone(),
            response_code.clone(),
            error.clone(),
            result.clone(),
        ));
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
    fn ut_request_task_reset_range() {
        const RANGE_TEST_URL:&str = "https://vd4.bdstatic.com/mda-pm7bte3t6fs50rsh/sc/cae_h264/1702057792414494257/mda-pm7bte3t6fs50rsh.mp4?v_from_s=bdapp-author-nanjing";
        const LENGTH: usize = 1984562;
        struct RestartTest {
            finished: Arc<AtomicBool>,
            data_receive: Arc<AtomicBool>,
            failed: Arc<AtomicBool>,
            total: Arc<AtomicUsize>,
        }
        impl RequestCallback for RestartTest {
            fn on_success(&mut self, _response: Response) {
                self.finished.store(true, Ordering::SeqCst);
            }

            fn on_fail(&mut self, _error: HttpClientError) {
                self.finished.store(true, Ordering::SeqCst);
                self.failed.store(true, Ordering::SeqCst);
            }

            fn on_cancel(&mut self) {
                self.finished.store(true, Ordering::SeqCst);
                self.failed.store(true, Ordering::SeqCst);
            }

            fn on_data_receive(&mut self, data: &[u8], _task: RequestTask) {
                self.data_receive.store(true, Ordering::SeqCst);
                self.total.fetch_add(data.len(), Ordering::SeqCst);
            }
        }

        let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
            NewHttpClientRequest();
        cxx::let_cxx_string!(url = RANGE_TEST_URL);
        request.pin_mut().SetURL(&url);
        cxx::let_cxx_string!(method = "GET");
        request.pin_mut().SetMethod(&method);

        let mut task = RequestTask::from_http_request(&request);
        let finished = Arc::new(AtomicBool::new(false));
        let total = Arc::new(AtomicUsize::new(0));
        let failed = Arc::new(AtomicBool::new(false));
        let data_receive = Arc::new(AtomicBool::new(false));

        let callback = Box::new(RestartTest {
            finished: finished.clone(),
            data_receive: data_receive.clone(),
            failed: failed.clone(),
            total: total.clone(),
        });
        task.set_callback(callback);
        task.start();

        while !data_receive.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        task.reset();
        while !finished.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert_eq!(total.load(Ordering::SeqCst), LENGTH);
        assert!(!failed.load(Ordering::SeqCst));
    }

    #[test]
    fn ut_request_task_reset_not_range() {
        const NOT_SUPPORT_RANGE_TEST_URL: &str =
            "https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt";
        const LENGTH: usize = 1042003;
        struct RestartTest {
            finished: Arc<AtomicBool>,
            data_receive: Arc<AtomicBool>,
            failed: Arc<AtomicBool>,
            total: Arc<AtomicUsize>,
        }
        impl RequestCallback for RestartTest {
            fn on_success(&mut self, _response: Response) {
                self.finished.store(true, Ordering::SeqCst);
            }

            fn on_fail(&mut self, _error: HttpClientError) {
                self.finished.store(true, Ordering::SeqCst);
                self.failed.store(true, Ordering::SeqCst);
            }

            fn on_cancel(&mut self) {
                self.finished.store(true, Ordering::SeqCst);
                self.failed.store(true, Ordering::SeqCst);
            }

            fn on_data_receive(&mut self, data: &[u8], _task: RequestTask) {
                self.data_receive.store(true, Ordering::SeqCst);
                self.total.fetch_add(data.len(), Ordering::SeqCst);
            }

            fn on_restart(&mut self) {
                self.total.store(0, Ordering::SeqCst);
            }
        }

        let mut request: cxx::UniquePtr<crate::wrapper::ffi::HttpClientRequest> =
            NewHttpClientRequest();
        cxx::let_cxx_string!(url = NOT_SUPPORT_RANGE_TEST_URL);
        request.pin_mut().SetURL(&url);
        cxx::let_cxx_string!(method = "GET");
        request.pin_mut().SetMethod(&method);

        let mut task = RequestTask::from_http_request(&request);
        let finished = Arc::new(AtomicBool::new(false));
        let total = Arc::new(AtomicUsize::new(0));
        let failed = Arc::new(AtomicBool::new(false));
        let data_receive = Arc::new(AtomicBool::new(false));

        let callback = Box::new(RestartTest {
            finished: finished.clone(),
            data_receive: data_receive.clone(),
            failed: failed.clone(),
            total: total.clone(),
        });
        task.set_callback(callback);
        task.start();

        while !data_receive.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(2000));
        }
        task.reset();
        while !finished.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
        assert_eq!(total.load(Ordering::SeqCst), LENGTH);
        assert!(!failed.load(Ordering::SeqCst));
    }
}
