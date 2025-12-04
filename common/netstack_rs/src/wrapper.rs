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

//! Module for FFI wrapper and callback handling.
//!
//! This module provides a bridge between Rust code and C++ FFI components,
//! managing HTTP request callbacks and task lifecycle.

use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Weak};

use cxx::SharedPtr;
use ffi::{
    GetHttpAddress, GetPerformanceInfo, GetResolvConf, HttpClientRequest, HttpClientTask,
    NewHttpClientTask, OnCallback,
};
use ffrt_rs::{ffrt_sleep, ffrt_spawn};
use request_utils::error;
use request_utils::task_id::TaskId;

use crate::error::{HttpClientError, HttpErrorCode};
use crate::info::{DownloadInfo, DownloadInfoMgr, RustPerformanceInfo};
use crate::request::RequestCallback;
use crate::response::{Response, ResponseCode};
use crate::task::{RequestTask, TaskStatus};

/// Result type for task creation operations.
///
/// Used internally to handle the result of creating a new HTTP client task.
enum NewTaskResult {
    /// Task creation succeeded with the new task and callback wrapper
    Success(SharedPtr<HttpClientTask>, Box<CallbackWrapper>),
    /// Task creation failed, returning the original callback
    Failed(Box<dyn RequestCallback>),
}

/// Wrapper for handling HTTP client callbacks.
///
/// Manages the lifecycle of HTTP requests, including success/failure handling,
/// progress tracking, and automatic retry logic.
pub struct CallbackWrapper {
    /// The user-provided callback implementation
    inner: Option<Box<dyn RequestCallback>>,
    // TODO This reset flag has never been assigned to true. Does it look useless?
    /// Flag indicating if the task should be reset
    reset: Arc<AtomicBool>,
    /// Weak reference to the task to avoid memory leaks
    task: Weak<Mutex<SharedPtr<HttpClientTask>>>,
    /// Unique identifier for the task
    task_id: TaskId,
    /// Performance and status information for the download
    info: DownloadInfo,
    /// Manager for storing download performance information
    info_mgr: Arc<DownloadInfoMgr>,
    /// Number of retry attempts made
    tries: usize,
    /// Current progress in bytes
    current: u64,
}

impl CallbackWrapper {
    /// Creates a new callback wrapper from a request callback.
    ///
    /// # Arguments
    ///
    /// * `inner` - The user-provided callback implementation
    /// * `reset` - Flag indicating if the task should be reset
    /// * `task` - Weak reference to the task
    /// * `task_id` - Unique identifier for the task
    /// * `info_mgr` - Manager for storing download information
    /// * `current` - Current progress in bytes
    ///
    /// # Returns
    ///
    /// A new `CallbackWrapper` instance configured with the provided parameters
    pub(crate) fn from_callback(
        inner: Box<dyn RequestCallback + 'static>,
        reset: Arc<AtomicBool>,
        task: Weak<Mutex<SharedPtr<HttpClientTask>>>,
        task_id: TaskId,
        info_mgr: Arc<DownloadInfoMgr>,
        current: u64,
    ) -> Self {
        // Create new download info and set network DNS configuration
        let mut info = DownloadInfo::new();
        let dns = GetResolvConf();
        info.set_network_dns(dns);
        Self {
            inner: Some(inner),
            reset,
            task,
            task_id,
            info,
            info_mgr,
            tries: 0,
            current,
        }
    }
}

impl CallbackWrapper {
    /// Handles successful HTTP request completion.
    ///
    /// Collects performance metrics and calls the appropriate user callback based on
    /// the HTTP response status code.
    ///
    /// # Arguments
    ///
    /// * `_request` - The HTTP request that completed
    /// * `response` - The HTTP response received
    fn on_success(&mut self, _request: &HttpClientRequest, response: &ffi::HttpClientResponse) {
        // Collect performance metrics from the response
        let mut performance = RustPerformanceInfo::default();
        GetPerformanceInfo(response, Pin::new(&mut performance));
        let addr = GetHttpAddress(response);
        self.info.set_performance(performance);
        self.info.set_ip_address(addr);
        self.info.set_size(self.current as i64);
        // Store download information for future reference
        self.info_mgr
            .insert_download_info(self.task_id.clone(), self.info.clone());

        // Take the user callback if available
        let Some(mut callback) = self.inner.take() else {
            return;
        };
        // Convert FFI response to Rust response
        let response = Response::from_ffi(response);
        // Check if the status code indicates success (200-299 range)
        if (response.status().clone() as u32 >= 300) || (response.status().clone() as u32) < 200 {
            // For non-success codes, create an error
            let error = HttpClientError::new(
                HttpErrorCode::HttpNoneErr,
                (response.status() as u32).to_string(),
            );
            callback.on_fail(error, self.info.clone());
        } else {
            // For success codes, call the success callback
            callback.on_success(response);
        }
    }

    /// Handles failed HTTP requests.
    ///
    /// Collects performance metrics and implements retry logic for failed requests.
    ///
    /// # Arguments
    ///
    /// * `request` - The HTTP request that failed
    /// * `response` - The partial or error response
    /// * `error` - The error information from the FFI layer
    fn on_fail(
        &mut self,
        request: &HttpClientRequest,
        response: &ffi::HttpClientResponse,
        error: &ffi::HttpClientError,
    ) {
        // Collect performance metrics from the response
        let mut performance = RustPerformanceInfo::default();
        GetPerformanceInfo(response, Pin::new(&mut performance));
        let addr = GetHttpAddress(response);
        self.info.set_performance(performance);
        self.info.set_ip_address(addr);
        self.info.set_size(self.current as i64);
        // Store download information for future reference
        self.info_mgr
            .insert_download_info(self.task_id.clone(), self.info.clone());

        // Convert FFI error to Rust error
        let error = HttpClientError::from_ffi(error);

        // Handle write errors as cancellations
        if *error.code() == HttpErrorCode::HttpWriteError {
            self.on_cancel(request, response);
            return;
        }

        // Take the user callback if available
        let Some(callback) = self.inner.take() else {
            return;
        };

        let info = self.info.clone();
        // Attempt to create a new task for retrying
        let (new_task, mut new_callback) = match self.create_new_task(callback, request) {
            NewTaskResult::Success(new_task, new_callback) => (new_task, new_callback),
            NewTaskResult::Failed(mut callback) => {
                // If task creation failed, call the fail callback
                callback.on_fail(error, info);
                return;
            }
        };

        // Retry immediately if we haven't exceeded the retry limit
        if self.tries < 3 {
            self.tries += 1;
            new_callback.tries = self.tries;
            Self::start_new_task(new_task, new_callback);
            return;
        }

        // For final attempt, use a spawned task to wait for potential reset signal
        let reset = self.reset.clone();
        ffrt_spawn(move || {
            // Wait up to 20 seconds for a reset signal
            for _ in 0..20 {
                ffrt_sleep(1000);
                if reset.load(Ordering::SeqCst) {
                    // If reset is signaled, start the new task
                    Self::start_new_task(new_task, new_callback);
                    reset.store(false, Ordering::SeqCst);
                    return;
                }
            }
            // If no reset signal after waiting, call the fail callback
            if let Some(mut callback) = new_callback.inner {
                callback.on_fail(error, info);
            }
        });
    }

    /// Handles task cancellation.
    ///
    /// Either restarts the task if a reset is requested or calls the cancel callback.
    ///
    /// # Arguments
    ///
    /// * `request` - The HTTP request that was cancelled
    /// * `_response` - The partial or incomplete response
    fn on_cancel(&mut self, request: &HttpClientRequest, _response: &ffi::HttpClientResponse) {
        // Take the user callback if available
        let Some(mut callback) = self.inner.take() else {
            return;
        };

        // Check if a reset is requested
        if self.reset.load(Ordering::SeqCst) {
            // Attempt to create a new task for restarting
            let (new_task, new_callback) = match self.create_new_task(callback, request) {
                NewTaskResult::Success(new_task, new_callback) => (new_task, new_callback),
                NewTaskResult::Failed(mut callback) => {
                    // If task creation failed, call the cancel callback
                    callback.on_cancel();
                    return;
                }
            };
            // Start the new task and clear the reset flag
            Self::start_new_task(new_task, new_callback);
            self.reset.store(false, Ordering::SeqCst);
        } else {
            // No reset requested, just call the cancel callback
            callback.on_cancel();
        }
    }

    fn on_data_receive(
        &mut self,
        task: SharedPtr<ffi::HttpClientTask>,
        data: *const u8,
        size: usize,
    ) {
        // Check if user callback is available
        let Some(callback) = self.inner.as_mut() else {
            return;
        };
        // Update progress counter
        self.current += size as u64;
        let data = unsafe { std::slice::from_raw_parts(data, size) };
        let task = RequestTask::from_ffi(task);
        // Forward data to user callback
        callback.on_data_receive(data, task);
    }

    /// Handles progress updates for HTTP transfers.
    ///
    /// Forwards progress information to the user callback.
    ///
    /// # Arguments
    ///
    /// * `dl_total` - Total bytes to download
    /// * `dl_now` - Bytes downloaded so far
    /// * `ul_total` - Total bytes to upload
    /// * `ul_now` - Bytes uploaded so far
    fn on_progress(&mut self, dl_total: u64, dl_now: u64, ul_total: u64, ul_now: u64) {
        // Check if user callback is available
        let Some(callback) = self.inner.as_mut() else {
            return;
        };
        // Forward progress to user callback
        callback.on_progress(dl_total, dl_now, ul_total, ul_now);
    }

    /// Creates a new HTTP task from an existing request.
    ///
    /// # Arguments
    ///
    /// * `callback` - The user callback to use with the new task
    /// * `request` - The HTTP request to create a new task for
    ///
    /// # Returns
    ///
    /// A `NewTaskResult` indicating success or failure
    fn create_new_task(
        &mut self,
        mut callback: Box<dyn RequestCallback>,
        request: &HttpClientRequest,
    ) -> NewTaskResult {
        // Notify the callback if we're restarting a partially completed request
        if self.current > 0 {
            callback.on_restart();
        }

        // Create a new HTTP task from the request
        let new_task = NewHttpClientTask(request);
        // Check if task creation failed
        if new_task.is_null() {
            error!("create_new_task NewHttpClientTask return null.");
            return NewTaskResult::Failed(callback);
        }
        // Create a new callback wrapper with the provided callback
        let new_callback = Box::new(CallbackWrapper::from_callback(
            callback,
            self.reset.clone(),
            self.task.clone(),
            self.task_id.clone(),
            self.info_mgr.clone(),
            0,
        ));
        // Return success with the new task and callback
        NewTaskResult::Success(new_task, new_callback)
    }

    /// Starts a new HTTP task with the provided callback.
    ///
    /// # Arguments
    ///
    /// * `task` - The HTTP task to start
    /// * `callback` - The callback to register with the task
    fn start_new_task(task: SharedPtr<HttpClientTask>, callback: Box<CallbackWrapper>) {
        // Update the weak reference to the task if possible
        if let Some(r) = callback.task.upgrade() {
            *r.lock().unwrap() = task.clone();
        }
        // Register the callback with the task
        OnCallback(&task, callback);
        // TODO start may return false. Not handling it may result in no callback, which
        // the caller cannot perceive
        // Start the task
        RequestTask::pin_mut(&task).Start();
    }
}

// SAFETY: HttpClientTask is thread-safe through its shared pointer implementation
unsafe impl Send for HttpClientTask {}
unsafe impl Sync for HttpClientTask {}

// C++ FFI bridge definitions - do not generate /// comments around this block
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

        type RustPerformanceInfo;
        fn set_dns_timing(self: &mut RustPerformanceInfo, time: f64);
        fn set_connect_timing(self: &mut RustPerformanceInfo, time: f64);
        fn set_tls_timing(self: &mut RustPerformanceInfo, time: f64);
        fn set_first_send_timing(self: &mut RustPerformanceInfo, time: f64);
        fn set_first_receive_timing(self: &mut RustPerformanceInfo, time: f64);
        fn set_total_timing(self: &mut RustPerformanceInfo, time: f64);
        fn set_redirect_timing(self: &mut RustPerformanceInfo, time: f64);
    }

    unsafe extern "C++" {
        include!("http_client_request.h");
        include!("wrapper.h");
        include!("http_client_task.h");
        include!("netstack.h");

        #[namespace = "OHOS::NetStack::HttpClient"]
        type TaskStatus;

        #[namespace = "OHOS::NetStack::HttpClient"]
        type ResponseCode;

        #[namespace = "OHOS::NetStack::HttpClient"]
        type HttpClientRequest;

        fn SetRequestSslType(request: Pin<&mut HttpClientRequest>, ssl_type: &CxxString);
        fn SetCaPath(self: Pin<&mut HttpClientRequest>, path: &CxxString);

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
        fn GetResolvConf() -> Vec<String>;
        fn GetPerformanceInfo(
            response: &HttpClientResponse,
            performance: Pin<&mut RustPerformanceInfo>,
        );
        fn GetHttpAddress(response: &HttpClientResponse) -> String;

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

    /// Converts an FFI task status to a Rust task status.
    ///
    /// # Arguments
    ///
    /// * `status` - The FFI task status to convert
    ///
    /// # Returns
    ///
    /// `Ok(Self)` if the conversion succeeded, `Err(status)` otherwise
    fn try_from(status: ffi::TaskStatus) -> Result<Self, Self::Error> {
        let ret = match status {
            ffi::TaskStatus::IDLE => TaskStatus::Idle,
            ffi::TaskStatus::RUNNING => TaskStatus::Running,
            _ => {
                // Return the original status if no mapping exists
                return Err(status);
            }
        };
        Ok(ret)
    }
}

impl TryFrom<ffi::ResponseCode> for ResponseCode {
    type Error = ffi::ResponseCode;

    /// Converts an FFI response code to a Rust response code.
    ///
    /// # Arguments
    ///
    /// * `value` - The FFI response code to convert
    ///
    /// # Returns
    ///
    /// `Ok(Self)` if the conversion succeeded, `Err(value)` otherwise
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
                // Return the original code if no mapping exists
                return Err(value);
            }
        };
        Ok(ret)
    }
}

impl TryFrom<ffi::HttpErrorCode> for HttpErrorCode {
    type Error = ffi::HttpErrorCode;

    /// Converts an FFI HTTP error code to a Rust HTTP error code.
    ///
    /// # Arguments
    ///
    /// * `value` - The FFI HTTP error code to convert
    ///
    /// # Returns
    ///
    /// `Ok(Self)` if the conversion succeeded, `Err(value)` otherwise
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
                // Return the original code if no mapping exists
                return Err(value);
            }
        };
        Ok(ret)
    }
}
