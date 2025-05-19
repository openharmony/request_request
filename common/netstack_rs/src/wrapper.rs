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

use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Weak};

use cxx::{let_cxx_string, SharedPtr};
use ffi::{HttpClientRequest, HttpClientTask, NewHttpClientTask, OnCallback};
use ffrt_rs::{ffrt_sleep, ffrt_spawn};

use crate::error::{HttpClientError, HttpErrorCode};
use crate::request::RequestCallback;
use crate::response::{Response, ResponseCode};
use crate::task::{RequestTask, TaskStatus};

pub struct CallbackWrapper {
    inner: Option<Box<dyn RequestCallback>>,
    reset: Arc<AtomicBool>,
    task: Weak<Mutex<SharedPtr<HttpClientTask>>>,
    tries: usize,
    current: u64,
}

impl CallbackWrapper {
    pub(crate) fn from_callback(
        inner: Box<dyn RequestCallback + 'static>,
        reset: Arc<AtomicBool>,
        task: Weak<Mutex<SharedPtr<HttpClientTask>>>,
        current: u64,
    ) -> Self {
        Self {
            inner: Some(inner),
            reset,
            task,
            tries: 0,
            current,
        }
    }
}

impl CallbackWrapper {
    fn on_success(&mut self, _request: &HttpClientRequest, response: &ffi::HttpClientResponse) {
        let Some(mut callback) = self.inner.take() else {
            return;
        };
        let response = Response::from_ffi(response);
        if (response.status().clone() as u32 >= 300) || (response.status().clone() as u32) < 200 {
            let error = HttpClientError::new(
                HttpErrorCode::HttpNoneErr,
                (response.status() as u32).to_string(),
            );
            callback.on_fail(error);
        } else {
            callback.on_success(response);
        }
    }

    fn on_fail(
        &mut self,
        request: &HttpClientRequest,
        response: &ffi::HttpClientResponse,
        error: &ffi::HttpClientError,
    ) {
        let error = HttpClientError::from_ffi(error);
        if *error.code() == HttpErrorCode::HttpWriteError {
            self.on_cancel(request, response);
            return;
        }
        let Some(callback) = self.inner.take() else {
            return;
        };
        let (new_task, mut new_callback) = self.create_new_task(callback, request, response);
        if self.tries < 3 {
            self.tries += 1;
            new_callback.tries = self.tries;
            Self::start_new_task(new_task, new_callback);
            return;
        }

        let reset = self.reset.clone();
        ffrt_spawn(move || {
            for _ in 0..20 {
                ffrt_sleep(1000);
                if reset.load(Ordering::SeqCst) {
                    Self::start_new_task(new_task, new_callback);
                    reset.store(false, Ordering::SeqCst);
                    return;
                }
            }
            if let Some(mut callback) = new_callback.inner {
                callback.on_fail(error);
            }
        });
    }

    fn on_cancel(&mut self, request: &HttpClientRequest, response: &ffi::HttpClientResponse) {
        let Some(mut callback) = self.inner.take() else {
            return;
        };

        if self.reset.load(Ordering::SeqCst) {
            let (new_task, new_callback) = self.create_new_task(callback, request, response);
            Self::start_new_task(new_task, new_callback);
            self.reset.store(false, Ordering::SeqCst);
        } else {
            callback.on_cancel();
        }
    }

    fn on_data_receive(
        &mut self,
        task: SharedPtr<ffi::HttpClientTask>,
        data: *const u8,
        size: usize,
    ) {
        let Some(callback) = self.inner.as_mut() else {
            return;
        };
        self.current += size as u64;
        let data = unsafe { std::slice::from_raw_parts(data, size) };
        let task = RequestTask::from_ffi(task);
        callback.on_data_receive(data, task);
    }

    fn on_progress(&mut self, dl_total: u64, dl_now: u64, ul_total: u64, ul_now: u64) {
        let Some(callback) = self.inner.as_mut() else {
            return;
        };
        callback.on_progress(dl_total, dl_now, ul_total, ul_now);
    }

    fn create_new_task(
        &mut self,
        mut callback: Box<dyn RequestCallback>,
        request: &HttpClientRequest,
        response: &ffi::HttpClientResponse,
    ) -> (SharedPtr<HttpClientTask>, Box<CallbackWrapper>) {
        if self.current > 0 && !set_range(request, response, &mut self.current) {
            callback.on_restart();
        }

        let new_task = NewHttpClientTask(request);
        let new_callback = Box::new(CallbackWrapper::from_callback(
            callback,
            self.reset.clone(),
            self.task.clone(),
            self.current,
        ));
        (new_task, new_callback)
    }

    fn start_new_task(task: SharedPtr<HttpClientTask>, callback: Box<CallbackWrapper>) {
        if let Some(r) = callback.task.upgrade() {
            *r.lock().unwrap() = task.clone();
        }
        OnCallback(&task, callback);
        RequestTask::pin_mut(&task).Start();
    }
}

fn set_range(
    request: &HttpClientRequest,
    response: &ffi::HttpClientResponse,
    current: &mut u64,
) -> bool {
    let response = Response::from_ffi(response);
    let headers = response.headers();
    let ptr = request as *const HttpClientRequest as *mut HttpClientRequest;
    let mut support_range = false;

    if let Some(etag) = headers.get("etag") {
        let_cxx_string!(key = "If-Range");
        let_cxx_string!(val = etag);
        unsafe {
            Pin::new_unchecked(ptr.as_mut().unwrap()).SetHeader(&key, &val);
        }
        support_range = true;
    } else if let Some(last_modified) = headers.get("last-modified") {
        let_cxx_string!(key = "If-Range");
        let_cxx_string!(val = last_modified);
        unsafe {
            Pin::new_unchecked(ptr.as_mut().unwrap()).SetHeader(&key, &val);
        }
        support_range = true;
    }

    if support_range {
        let_cxx_string!(key = "Range");
        let bytes = format!("bytes={}-", current);
        let_cxx_string!(val = bytes);
        unsafe {
            Pin::new_unchecked(ptr.as_mut().unwrap()).SetHeader(&key, &val);
        }
    } else {
        *current = 0;
    }
    support_range
}

unsafe impl Send for HttpClientTask {}
unsafe impl Sync for HttpClientTask {}

#[allow(unused_unsafe)]
#[cxx::bridge(namespace = "OHOS::Request")]
pub(crate) mod ffi {
    extern "Rust" {
        type CallbackWrapper;
        fn on_success(
            self: &mut CallbackWrapper,
            request: &HttpClientRequest,
            response: &HttpClientResponse,
        );
        fn on_fail(
            self: &mut CallbackWrapper,
            request: &HttpClientRequest,
            response: &HttpClientResponse,
            error: &HttpClientError,
        );
        fn on_cancel(
            self: &mut CallbackWrapper,
            request: &HttpClientRequest,
            response: &HttpClientResponse,
        );
        unsafe fn on_data_receive(
            self: &mut CallbackWrapper,
            task: SharedPtr<HttpClientTask>,
            data: *const u8,
            size: usize,
        );
        fn on_progress(
            self: &mut CallbackWrapper,
            dl_total: u64,
            dl_now: u64,
            ul_total: u64,
            ul_now: u64,
        );
    }

    unsafe extern "C++" {
        include!("http_client_request.h");
        include!("wrapper.h");
        include!("http_client_task.h");

        #[namespace = "OHOS::NetStack::HttpClient"]
        type TaskStatus;

        #[namespace = "OHOS::NetStack::HttpClient"]
        type ResponseCode;

        #[namespace = "OHOS::NetStack::HttpClient"]
        type HttpClientRequest;

        #[namespace = "OHOS::NetStack::HttpClient"]
        type HttpErrorCode;

        fn NewHttpClientRequest() -> UniquePtr<HttpClientRequest>;
        fn SetURL(self: Pin<&mut HttpClientRequest>, url: &CxxString);
        fn SetMethod(self: Pin<&mut HttpClientRequest>, method: &CxxString);
        fn SetHeader(self: Pin<&mut HttpClientRequest>, key: &CxxString, val: &CxxString);
        fn SetTimeout(self: Pin<&mut HttpClientRequest>, timeout: u32);
        fn SetConnectTimeout(self: Pin<&mut HttpClientRequest>, timeout: u32);
        unsafe fn SetBody(request: Pin<&mut HttpClientRequest>, data: *const u8, length: usize);

        #[namespace = "OHOS::NetStack::HttpClient"]
        type HttpClientTask;

        fn NewHttpClientTask(request: &HttpClientRequest) -> SharedPtr<HttpClientTask>;
        fn GetResponse(self: Pin<&mut HttpClientTask>) -> Pin<&mut HttpClientResponse>;
        fn Start(self: Pin<&mut HttpClientTask>) -> bool;
        fn Cancel(self: Pin<&mut HttpClientTask>);
        fn GetStatus(self: Pin<&mut HttpClientTask>) -> TaskStatus;
        fn OnCallback(task: &SharedPtr<HttpClientTask>, callback: Box<CallbackWrapper>);

        #[namespace = "OHOS::NetStack::HttpClient"]
        type HttpClientResponse;

        fn GetResponseCode(self: &HttpClientResponse) -> ResponseCode;
        fn GetHeaders(response: Pin<&mut HttpClientResponse>) -> Vec<String>;

        #[namespace = "OHOS::NetStack::HttpClient"]
        type HttpClientError;

        fn GetErrorCode(self: &HttpClientError) -> HttpErrorCode;
        fn GetErrorMessage(self: &HttpClientError) -> &CxxString;
    }

    #[repr(i32)]
    enum TaskStatus {
        IDLE,
        RUNNING,
    }

    #[repr(i32)]
    enum ResponseCode {
        NONE = 0,
        OK = 200,
        CREATED,
        ACCEPTED,
        NOT_AUTHORITATIVE,
        NO_CONTENT,
        RESET,
        PARTIAL,
        MULT_CHOICE = 300,
        MOVED_PERM,
        MOVED_TEMP,
        SEE_OTHER,
        NOT_MODIFIED,
        USE_PROXY,
        BAD_REQUEST = 400,
        UNAUTHORIZED,
        PAYMENT_REQUIRED,
        FORBIDDEN,
        NOT_FOUND,
        BAD_METHOD,
        NOT_ACCEPTABLE,
        PROXY_AUTH,
        CLIENT_TIMEOUT,
        CONFLICT,
        GONE,
        LENGTH_REQUIRED,
        PRECON_FAILED,
        ENTITY_TOO_LARGE,
        REQ_TOO_LONG,
        UNSUPPORTED_TYPE,
        INTERNAL_ERROR = 500,
        NOT_IMPLEMENTED,
        BAD_GATEWAY,
        UNAVAILABLE,
        GATEWAY_TIMEOUT,
        VERSION,
    }

    #[repr(i32)]
    enum HttpErrorCode {
        HTTP_NONE_ERR,
        HTTP_PERMISSION_DENIED_CODE = 201,
        HTTP_PARSE_ERROR_CODE = 401,
        HTTP_ERROR_CODE_BASE = 2300000,
        HTTP_UNSUPPORTED_PROTOCOL,
        HTTP_FAILED_INIT,
        HTTP_URL_MALFORMAT,
        HTTP_COULDNT_RESOLVE_PROXY = 2300005,
        HTTP_COULDNT_RESOLVE_HOST,
        HTTP_COULDNT_CONNECT,
        HTTP_WEIRD_SERVER_REPLY,
        HTTP_REMOTE_ACCESS_DENIED,
        HTTP_HTTP2_ERROR = 2300016,
        HTTP_PARTIAL_FILE = 2300018,
        HTTP_WRITE_ERROR = 2300023,
        HTTP_UPLOAD_FAILED = 2300025,
        HTTP_READ_ERROR = 2300026,
        HTTP_OUT_OF_MEMORY,
        HTTP_OPERATION_TIMEDOUT,
        HTTP_POST_ERROR = 2300034,
        HTTP_TASK_CANCELED = 2300042,
        HTTP_TOO_MANY_REDIRECTS = 2300047,
        HTTP_GOT_NOTHING = 2300052,
        HTTP_SEND_ERROR = 2300055,
        HTTP_RECV_ERROR,
        HTTP_SSL_CERTPROBLEM = 2300058,
        HTTP_SSL_CIPHER,
        HTTP_PEER_FAILED_VERIFICATION,
        HTTP_BAD_CONTENT_ENCODING,
        HTTP_FILESIZE_EXCEEDED = 2300063,
        HTTP_REMOTE_DISK_FULL = 2300070,
        HTTP_REMOTE_FILE_EXISTS = 2300073,
        HTTP_SSL_CACERT_BADFILE = 2300077,
        HTTP_REMOTE_FILE_NOT_FOUND,
        HTTP_AUTH_ERROR = 2300094,
        HTTP_UNKNOWN_OTHER_ERROR = 2300999,
    }
}

impl TryFrom<ffi::TaskStatus> for TaskStatus {
    type Error = ffi::TaskStatus;
    fn try_from(status: ffi::TaskStatus) -> Result<Self, Self::Error> {
        let ret = match status {
            ffi::TaskStatus::IDLE => TaskStatus::Idle,
            ffi::TaskStatus::RUNNING => TaskStatus::Running,
            _ => {
                return Err(status);
            }
        };
        Ok(ret)
    }
}

impl TryFrom<ffi::ResponseCode> for ResponseCode {
    type Error = ffi::ResponseCode;
    fn try_from(value: ffi::ResponseCode) -> Result<Self, Self::Error> {
        let ret = match value {
            ffi::ResponseCode::NONE => ResponseCode::None,
            ffi::ResponseCode::OK => ResponseCode::Ok,
            ffi::ResponseCode::CREATED => ResponseCode::Created,
            ffi::ResponseCode::ACCEPTED => ResponseCode::Accepted,
            ffi::ResponseCode::NOT_AUTHORITATIVE => ResponseCode::NotAuthoritative,
            ffi::ResponseCode::NO_CONTENT => ResponseCode::NoContent,
            ffi::ResponseCode::RESET => ResponseCode::Reset,
            ffi::ResponseCode::PARTIAL => ResponseCode::Partial,
            ffi::ResponseCode::MULT_CHOICE => ResponseCode::MultChoice,
            ffi::ResponseCode::MOVED_PERM => ResponseCode::MovedPerm,
            ffi::ResponseCode::MOVED_TEMP => ResponseCode::MovedTemp,
            ffi::ResponseCode::SEE_OTHER => ResponseCode::SeeOther,
            ffi::ResponseCode::NOT_MODIFIED => ResponseCode::NotModified,
            ffi::ResponseCode::USE_PROXY => ResponseCode::UseProxy,
            ffi::ResponseCode::BAD_REQUEST => ResponseCode::BadRequest,
            ffi::ResponseCode::UNAUTHORIZED => ResponseCode::Unauthorized,
            ffi::ResponseCode::PAYMENT_REQUIRED => ResponseCode::PaymentRequired,
            ffi::ResponseCode::FORBIDDEN => ResponseCode::Forbidden,
            ffi::ResponseCode::NOT_FOUND => ResponseCode::NotFound,
            ffi::ResponseCode::BAD_METHOD => ResponseCode::BadMethod,
            ffi::ResponseCode::NOT_ACCEPTABLE => ResponseCode::NotAcceptable,
            ffi::ResponseCode::PROXY_AUTH => ResponseCode::ProxyAuth,
            ffi::ResponseCode::CLIENT_TIMEOUT => ResponseCode::ClientTimeout,
            ffi::ResponseCode::CONFLICT => ResponseCode::Conflict,
            ffi::ResponseCode::GONE => ResponseCode::Gone,
            ffi::ResponseCode::LENGTH_REQUIRED => ResponseCode::LengthRequired,
            ffi::ResponseCode::PRECON_FAILED => ResponseCode::PreconFailed,
            ffi::ResponseCode::ENTITY_TOO_LARGE => ResponseCode::EntityTooLarge,
            ffi::ResponseCode::REQ_TOO_LONG => ResponseCode::ReqTooLong,
            ffi::ResponseCode::UNSUPPORTED_TYPE => ResponseCode::UnsupportedType,
            ffi::ResponseCode::INTERNAL_ERROR => ResponseCode::InternalError,
            ffi::ResponseCode::NOT_IMPLEMENTED => ResponseCode::NotImplemented,
            ffi::ResponseCode::BAD_GATEWAY => ResponseCode::BadGateway,
            ffi::ResponseCode::UNAVAILABLE => ResponseCode::Unavailable,
            ffi::ResponseCode::GATEWAY_TIMEOUT => ResponseCode::GatewayTimeout,
            ffi::ResponseCode::VERSION => ResponseCode::Version,
            _ => {
                return Err(value);
            }
        };
        Ok(ret)
    }
}

impl TryFrom<ffi::HttpErrorCode> for HttpErrorCode {
    type Error = ffi::HttpErrorCode;
    fn try_from(value: ffi::HttpErrorCode) -> Result<Self, Self::Error> {
        let ret = match value {
            ffi::HttpErrorCode::HTTP_NONE_ERR => HttpErrorCode::HttpNoneErr,
            ffi::HttpErrorCode::HTTP_PERMISSION_DENIED_CODE => {
                HttpErrorCode::HttpPermissionDeniedCode
            }
            ffi::HttpErrorCode::HTTP_PARSE_ERROR_CODE => HttpErrorCode::HttpParseErrorCode,
            ffi::HttpErrorCode::HTTP_ERROR_CODE_BASE => HttpErrorCode::HttpErrorCodeBase,
            ffi::HttpErrorCode::HTTP_UNSUPPORTED_PROTOCOL => HttpErrorCode::HttpUnsupportedProtocol,
            ffi::HttpErrorCode::HTTP_FAILED_INIT => HttpErrorCode::HttpFailedInit,
            ffi::HttpErrorCode::HTTP_URL_MALFORMAT => HttpErrorCode::HttpUrlMalformat,
            ffi::HttpErrorCode::HTTP_COULDNT_RESOLVE_PROXY => {
                HttpErrorCode::HttpCouldntResolveProxy
            }
            ffi::HttpErrorCode::HTTP_COULDNT_RESOLVE_HOST => HttpErrorCode::HttpCouldntResolveHost,
            ffi::HttpErrorCode::HTTP_COULDNT_CONNECT => HttpErrorCode::HttpCouldntConnect,
            ffi::HttpErrorCode::HTTP_WEIRD_SERVER_REPLY => HttpErrorCode::HttpWeirdServerReply,
            ffi::HttpErrorCode::HTTP_REMOTE_ACCESS_DENIED => HttpErrorCode::HttpRemoteAccessDenied,
            ffi::HttpErrorCode::HTTP_HTTP2_ERROR => HttpErrorCode::HttpHttp2Error,
            ffi::HttpErrorCode::HTTP_PARTIAL_FILE => HttpErrorCode::HttpPartialFile,
            ffi::HttpErrorCode::HTTP_WRITE_ERROR => HttpErrorCode::HttpWriteError,
            ffi::HttpErrorCode::HTTP_UPLOAD_FAILED => HttpErrorCode::HttpUploadFailed,
            ffi::HttpErrorCode::HTTP_READ_ERROR => HttpErrorCode::HttpReadError,
            ffi::HttpErrorCode::HTTP_OUT_OF_MEMORY => HttpErrorCode::HttpOutOfMemory,
            ffi::HttpErrorCode::HTTP_OPERATION_TIMEDOUT => HttpErrorCode::HttpOperationTimedout,
            ffi::HttpErrorCode::HTTP_POST_ERROR => HttpErrorCode::HttpPostError,
            ffi::HttpErrorCode::HTTP_TASK_CANCELED => HttpErrorCode::HttpTaskCanceled,
            ffi::HttpErrorCode::HTTP_TOO_MANY_REDIRECTS => HttpErrorCode::HttpTooManyRedirects,
            ffi::HttpErrorCode::HTTP_GOT_NOTHING => HttpErrorCode::HttpGotNothing,
            ffi::HttpErrorCode::HTTP_SEND_ERROR => HttpErrorCode::HttpSendError,
            ffi::HttpErrorCode::HTTP_RECV_ERROR => HttpErrorCode::HttpRecvError,
            ffi::HttpErrorCode::HTTP_SSL_CERTPROBLEM => HttpErrorCode::HttpSslCertproblem,
            ffi::HttpErrorCode::HTTP_SSL_CIPHER => HttpErrorCode::HttpSslCipher,
            ffi::HttpErrorCode::HTTP_PEER_FAILED_VERIFICATION => {
                HttpErrorCode::HttpPeerFailedVerification
            }
            ffi::HttpErrorCode::HTTP_BAD_CONTENT_ENCODING => HttpErrorCode::HttpBadContentEncoding,
            ffi::HttpErrorCode::HTTP_FILESIZE_EXCEEDED => HttpErrorCode::HttpFilesizeExceeded,
            ffi::HttpErrorCode::HTTP_REMOTE_DISK_FULL => HttpErrorCode::HttpRemoteDiskFull,
            ffi::HttpErrorCode::HTTP_REMOTE_FILE_EXISTS => HttpErrorCode::HttpRemoteFileExists,
            ffi::HttpErrorCode::HTTP_SSL_CACERT_BADFILE => HttpErrorCode::HttpSslCacertBadfile,
            ffi::HttpErrorCode::HTTP_REMOTE_FILE_NOT_FOUND => HttpErrorCode::HttpRemoteFileNotFound,
            ffi::HttpErrorCode::HTTP_AUTH_ERROR => HttpErrorCode::HttpAuthError,
            ffi::HttpErrorCode::HTTP_UNKNOWN_OTHER_ERROR => HttpErrorCode::HttpUnknownOtherError,
            _ => {
                return Err(value);
            }
        };
        Ok(ret)
    }
}
