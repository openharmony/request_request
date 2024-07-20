// Copyright (C) 2023 Huawei Device Co., Ltd.
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

use std::io::SeekFrom;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard};
use std::thread::sleep;
use std::time::Duration;

use ylong_http_client::async_impl::{Body, Client, Request, RequestBuilder, Response};
use ylong_http_client::{ErrorKind, HttpClientError};
use ylong_runtime::io::{AsyncSeekExt, AsyncWriteExt};

use super::config::{Mode, Version};
use super::info::{CommonTaskInfo, State, TaskInfo, UpdateInfo};
use super::notify::{EachFileStatus, NotifyData, Progress};
use super::reason::Reason;
use crate::error::ErrorCode;
use crate::manage::app_state::AppState;
use crate::manage::database::Database;
use crate::manage::notifier::Notifier;
use crate::manage::{self, SystemConfig};
use crate::service::client::ClientManagerEntry;
use crate::task::client::build_client;
use crate::task::config::{Action, TaskConfig};
use crate::task::files::{AttachedFiles, Files};
use crate::utils::form_item::FileSpec;
use crate::utils::{
    get_current_timestamp, publish_state_change_event, request_background_notify, RequestTaskMsg,
};
const RETRY_INTERVAL: u64 = 200;

pub(crate) struct RequestTask {
    pub(crate) conf: TaskConfig,
    pub(crate) app_state: AppState,
    pub(crate) client: Client,
    pub(crate) client_manager: ClientManagerEntry,
    pub(crate) files: Files,
    pub(crate) body_files: Files,
    pub(crate) ctime: u64,
    pub(crate) mime_type: Mutex<String>,
    pub(crate) progress: Mutex<Progress>,
    pub(crate) status: Mutex<TaskStatus>,
    pub(crate) code: Mutex<Vec<Reason>>,
    pub(crate) tries: AtomicU32,
    pub(crate) get_file_info: AtomicBool,
    pub(crate) retry: AtomicBool,
    pub(crate) retry_for_request: AtomicBool,
    pub(crate) background_notify_time: AtomicU64,
    pub(crate) file_total_size: AtomicI64,
    pub(crate) resume: AtomicBool,
    pub(crate) seek_flag: AtomicBool,
    pub(crate) range_request: AtomicBool,
    pub(crate) range_response: AtomicBool,
    pub(crate) restored: AtomicBool,
    pub(crate) skip_bytes: AtomicU64,
    pub(crate) upload_counts: AtomicUsize,
    pub(crate) rate_limiting: AtomicU64,
    pub(crate) last_notify: AtomicU64,
    pub(crate) network: manage::Network,
}

impl RequestTask {
    pub(crate) fn task_id(&self) -> u32 {
        self.conf.common_data.task_id
    }

    pub(crate) fn uid(&self) -> u64 {
        self.conf.common_data.uid
    }

    pub(crate) fn config(&self) -> &TaskConfig {
        &self.conf
    }

    // only use for download task
    pub(crate) fn mime_type(&self) -> String {
        self.mime_type.lock().unwrap().clone()
    }

    pub(crate) fn action(&self) -> Action {
        self.conf.common_data.action
    }

    pub(crate) fn mode(&self) -> Mode {
        self.conf.common_data.mode
    }

    pub(crate) fn bundle(&self) -> &str {
        self.conf.bundle.as_str()
    }

    pub(crate) fn speed_limit(&self, limit: u64) {
        let old = self.rate_limiting.swap(limit, Ordering::SeqCst);
        if old != limit {
            info!("task {} speed_limit {}", self.task_id(), limit);
        }
    }

    pub(crate) fn satisfied(&self) -> bool {
        if !self.network_online() || !self.check_network_status() {
            error!("check network failed, tid: {}", self.task_id());
            false
        } else {
            true
        }
    }
}

impl RequestTask {
    pub(crate) fn new(
        config: TaskConfig,
        app_state: AppState,
        files: AttachedFiles,
        client: Client,
        client_manager: ClientManagerEntry,
        network: manage::Network,
    ) -> RequestTask {
        let file_len = files.files.len();
        let action = config.common_data.action;

        let file_total_size = match action {
            Action::Upload => {
                let mut file_total_size = 0i64;
                // If the total size overflows, ignore it.
                for size in files.sizes.iter() {
                    file_total_size += *size;
                }
                file_total_size
            }
            Action::Download => -1,
            _ => unreachable!("Action::Any in RequestTask::new never reach"),
        };

        let time = get_current_timestamp();
        let status = TaskStatus::new(time);
        let progress = Progress::new(files.sizes);

        RequestTask {
            conf: config,
            client,
            files: files.files,
            body_files: files.body_files,
            ctime: time,
            mime_type: Mutex::new(String::new()),
            progress: Mutex::new(progress),
            tries: AtomicU32::new(0),
            status: Mutex::new(status),
            retry: AtomicBool::new(false),
            get_file_info: AtomicBool::new(false),
            retry_for_request: AtomicBool::new(false),
            code: Mutex::new(vec![Reason::Default; file_len]),
            background_notify_time: AtomicU64::new(time),
            file_total_size: AtomicI64::new(file_total_size),
            resume: AtomicBool::new(false),
            seek_flag: AtomicBool::new(false),
            range_request: AtomicBool::new(false),
            range_response: AtomicBool::new(false),
            restored: AtomicBool::new(false),
            skip_bytes: AtomicU64::new(0),
            upload_counts: AtomicUsize::new(0),
            rate_limiting: AtomicU64::new(0),
            app_state,
            last_notify: AtomicU64::new(time),
            client_manager,
            network,
        }
    }

    pub(crate) fn new_by_info(
        config: TaskConfig,
        system: SystemConfig,
        app_state: AppState,
        info: TaskInfo,
        client_manager: ClientManagerEntry,
        network: manage::Network,
    ) -> Result<RequestTask, ErrorCode> {
        let (files, client) = check_config(&config, system)?;

        let file_len = files.files.len();
        let action = config.common_data.action;
        let time = get_current_timestamp();

        let file_total_size = match action {
            Action::Upload => {
                let mut file_total_size = 0i64;
                // If the total size overflows, ignore it.
                for size in files.sizes.iter() {
                    file_total_size += *size;
                }
                file_total_size
            }
            Action::Download => *info.progress.sizes.first().unwrap_or(&-1),
            _ => unreachable!("Action::Any in RequestTask::new never reach"),
        };

        // If `TaskInfo` is provided, use data of it.
        let ctime = info.common_data.ctime;
        let mime_type = info.mime_type.clone();
        let tries = info.common_data.tries;
        let upload_counts = info.progress.common_data.index;
        let status = TaskStatus {
            waiting_network_time: None,
            mtime: time,
            state: State::from(info.progress.common_data.state),
            reason: Reason::from(info.common_data.reason),
        };
        let retry = info.common_data.retry;
        let progress = info.progress;

        Ok(RequestTask {
            conf: config,
            client,
            files: files.files,
            body_files: files.body_files,
            ctime,
            mime_type: Mutex::new(mime_type),
            progress: Mutex::new(progress),
            tries: AtomicU32::new(tries),
            status: Mutex::new(status),
            retry: AtomicBool::new(retry),
            get_file_info: AtomicBool::new(false),
            retry_for_request: AtomicBool::new(false),
            code: Mutex::new(vec![Reason::Default; file_len]),
            background_notify_time: AtomicU64::new(time),
            file_total_size: AtomicI64::new(file_total_size),
            resume: AtomicBool::new(false),
            seek_flag: AtomicBool::new(false),
            range_request: AtomicBool::new(false),
            range_response: AtomicBool::new(false),
            restored: AtomicBool::new(false),
            skip_bytes: AtomicU64::new(0),
            upload_counts: AtomicUsize::new(upload_counts),
            rate_limiting: AtomicU64::new(0),
            app_state,
            last_notify: AtomicU64::new(time),
            client_manager,
            network,
        })
    }

    pub(crate) fn build_notify_data(&self) -> NotifyData {
        let vec = self.get_each_file_status();
        NotifyData {
            bundle: self.conf.bundle.clone(),
            // `unwrap` for propagating panics among threads.
            progress: self.progress.lock().unwrap().clone(),
            action: self.conf.common_data.action,
            version: self.conf.version,
            each_file_status: vec,
            task_id: self.conf.common_data.task_id,
            _uid: self.conf.common_data.uid,
        }
    }

    pub(crate) fn check_network_status(&self) -> bool {
        if !self.is_satisfied_configuration() {
            if !(self.conf.version == Version::API10
                && self.conf.common_data.mode == Mode::BackGround
                && self.conf.common_data.retry)
            {
                self.change_task_status(State::Failed, Reason::UnsupportedNetworkType);
            }
            return false;
        }
        true
    }

    pub(crate) fn check_app_state(&self) -> bool {
        if self.conf.common_data.mode == Mode::FrontEnd && !self.app_state.is_foreground() {
            if self.conf.common_data.action == Action::Upload {
                self.change_task_status(State::Failed, Reason::AppBackgroundOrTerminate);
            } else if self.conf.common_data.action == Action::Download {
                self.change_task_status(State::Paused, Reason::AppBackgroundOrTerminate);
            }
            false
        } else {
            true
        }
    }

    pub(crate) fn network_online(&self) -> bool {
        if !self.network.is_online() {
            if self.conf.version == Version::API10
                && self.conf.common_data.mode == Mode::BackGround
                && self.conf.common_data.retry
            {
                self.change_task_status(State::Waiting, Reason::NetworkOffline);
            } else {
                let retry_times = 3;
                for _ in 0..retry_times {
                    if self.network.is_online() {
                        return true;
                    }
                    sleep(Duration::from_millis(RETRY_INTERVAL));
                }
                self.change_task_status(State::Failed, Reason::NetworkOffline);
            }
            return false;
        }
        true
    }

    pub(crate) fn build_request_builder(&self) -> Result<RequestBuilder, HttpClientError> {
        use ylong_http_client::async_impl::PercentEncoder;

        let url = self.conf.url.clone();
        let url = match PercentEncoder::encode(url.as_str()) {
            Ok(value) => value,
            Err(e) => {
                error!("url percent encoding error is {:?}", e);
                return Err(e);
            }
        };

        let method = match self.conf.method.to_uppercase().as_str() {
            "PUT" => "PUT",
            "POST" => "POST",
            "GET" => "GET",
            _ => match self.conf.common_data.action {
                Action::Upload => {
                    if self.conf.version == Version::API10 {
                        "PUT"
                    } else {
                        "POST"
                    }
                }
                Action::Download => "GET",
                _ => "",
            },
        };
        let mut request = RequestBuilder::new().method(method).url(url.as_str());
        for (key, value) in self.conf.headers.iter() {
            request = request.header(key.as_str(), value.as_str());
        }
        Ok(request)
    }

    async fn clear_downloaded_file(&self) -> bool {
        let file = self.files.get_mut(0).unwrap();
        let res = file.set_len(0).await;
        match res {
            Err(e) => {
                error!("clear download file error: {:?}", e);
                self.change_task_status(State::Failed, Reason::IoError);
                false
            }
            _ => {
                debug!("set len success");
                match file.seek(SeekFrom::Start(0)).await {
                    Err(e) => {
                        error!("seek err is {:?}", e);
                        self.change_task_status(State::Failed, Reason::IoError);
                        false
                    }
                    Ok(_) => {
                        debug!("seek success");
                        let mut progress_guard = self.progress.lock().unwrap();
                        progress_guard.common_data.total_processed = 0;
                        progress_guard.processed[0] = 0;
                        true
                    }
                }
            }
        }
    }

    pub(crate) async fn build_download_request(&self) -> Option<Request> {
        let mut request_builder = match self.build_request_builder() {
            Ok(builder) => builder,
            _ => {
                self.change_task_status(State::Failed, Reason::BuildRequestFailed);
                return None;
            }
        };

        let mut begins = self.conf.common_data.begins;
        let ends = self.conf.common_data.ends;
        self.range_response.store(false, Ordering::SeqCst);
        if self.resume.load(Ordering::SeqCst) || begins > 0 || ends >= 0 {
            self.range_request.store(true, Ordering::SeqCst);
            self.skip_bytes.store(0, Ordering::SeqCst);
            if self.resume.load(Ordering::SeqCst) {
                let if_range = {
                    let progress_guard = self.progress.lock().unwrap();
                    let etag = progress_guard.extras.get("etag");
                    let last_modified = progress_guard.extras.get("last-modified");
                    if let Some(etag) = etag {
                        request_builder = request_builder.header("If-Range", etag.as_str());
                        true
                    } else if let Some(last_modified) = last_modified {
                        request_builder =
                            request_builder.header("If-Range", last_modified.as_str());
                        true
                    } else {
                        false
                    }
                };
                if !if_range {
                    // unable to verify file consistency, need download again
                    if begins == 0 && ends < 0 {
                        self.range_request.store(false, Ordering::SeqCst);
                    }
                    if !self.clear_downloaded_file().await {
                        return None;
                    }
                }
            }
            let file = self.files.get_mut(0).unwrap();
            let current_len = file.metadata().await.unwrap().len();
            begins += current_len;
            // Modifys the progress to the current file size.
            // It will be recorded to the database later during download.
            let mut progress_guard = self.progress.lock().unwrap();
            progress_guard.processed[0] = current_len as usize;
            progress_guard.common_data.total_processed = current_len as usize;
            if self.range_request.load(Ordering::SeqCst) {
                let range = if ends < 0 {
                    format!("bytes={begins}-")
                } else {
                    format!("bytes={begins}-{ends}")
                };
                request_builder = request_builder.header("Range", range.as_str());
            }
        } else {
            self.range_request.store(false, Ordering::SeqCst);
        }
        let result = request_builder.body(Body::slice(self.conf.data.clone()));
        match result {
            Ok(value) => Some(value),
            Err(e) => {
                error!("build download request error is {:?}", e);
                self.change_task_status(State::Failed, Reason::BuildRequestFailed);
                None
            }
        }
    }

    pub(crate) fn get_file_info(&self, response: &Response) -> bool {
        if self.get_file_info.load(Ordering::SeqCst) {
            return true;
        }
        self.get_file_info.store(true, Ordering::SeqCst);
        let content_type = response.headers().get("content-type");
        if let Some(mime_type) = content_type {
            if let Ok(value) = mime_type.to_string() {
                *self.mime_type.lock().unwrap() = value;
            }
        }

        let content_length = response.headers().get("content-length");
        if let Some(len) = content_length {
            let length = len.to_string();
            match length {
                Ok(value) => {
                    let len = value.parse::<i64>();
                    match len {
                        Ok(v) => {
                            let mut guard = self.progress.lock().unwrap();
                            if !self.restored.load(Ordering::SeqCst) {
                                guard.sizes[0] = v + guard.processed[0] as i64;
                            }
                            self.file_total_size.store(v, Ordering::SeqCst);
                            debug!("the download task content-length is {}", v);
                        }
                        Err(e) => {
                            error!("convert string to i64 error: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("convert header value to string error: {:?}", e);
                }
            }
        } else {
            error!("cannot get content-length of the task");
            if self.conf.common_data.precise {
                self.change_task_status(State::Failed, Reason::GetFileSizeFailed);
                return false;
            }
        }
        true
    }

    async fn handle_body_transfer_error(&self) {
        if self.network.check_interval_online().await {
            self.change_task_status(State::Failed, Reason::OthersError);
        } else {
            match self.conf.version {
                Version::API9 => {
                    if self.conf.common_data.action == Action::Upload {
                        self.change_task_status(State::Failed, Reason::NetworkOffline);
                    }
                }
                Version::API10 => {
                    if self.conf.common_data.mode == Mode::FrontEnd || !self.conf.common_data.retry
                    {
                        self.change_task_status(State::Failed, Reason::NetworkOffline);
                    }
                }
            }
        }
    }

    pub(crate) async fn handle_download_error(&self, result: &Result<(), HttpClientError>) -> bool {
        match result {
            Ok(_) => true,
            Err(err) => {
                error!("download err is {:?}", err);
                match err.error_kind() {
                    ErrorKind::Timeout => {
                        self.change_task_status(State::Failed, Reason::ContinuousTaskTimeout);
                    }
                    // user triggered
                    ErrorKind::UserAborted => return true,
                    ErrorKind::BodyTransfer | ErrorKind::BodyDecode => {
                        self.handle_body_transfer_error().await;
                    }
                    _ => {
                        if format!("{}", err).contains("No space left on device") {
                            self.change_task_status(State::Failed, Reason::InsufficientSpace);
                        } else {
                            self.change_task_status(State::Failed, Reason::OthersError);
                        }
                    }
                }
                false
            }
        }
    }

    pub(crate) async fn handle_response_error(
        &self,
        response: &Result<Response, HttpClientError>,
    ) -> bool {
        let index = self.progress.lock().unwrap().common_data.index;
        match response {
            Ok(r) => {
                let http_response_code = r.status();
                info!(
                    "task {} get http response code {}",
                    self.conf.common_data.task_id, http_response_code
                );
                if http_response_code.is_server_error()
                    || (http_response_code.as_u16() != 408 && http_response_code.is_client_error())
                    || http_response_code.is_redirection()
                {
                    self.set_code(index, Reason::ProtocolError, false);
                    return false;
                }
                if http_response_code.as_u16() == 408 {
                    if !self.retry_for_request.load(Ordering::SeqCst) {
                        self.retry_for_request.store(true, Ordering::SeqCst);
                    } else {
                        self.set_code(index, Reason::ProtocolError, false);
                    }
                    return false;
                }

                if self.range_request.load(Ordering::SeqCst) {
                    match http_response_code.as_u16() {
                        206 => {
                            self.range_response.store(true, Ordering::SeqCst);
                        }
                        200 => {
                            self.range_response.store(false, Ordering::SeqCst);
                            if self.resume.load(Ordering::SeqCst) {
                                if !self.clear_downloaded_file().await {
                                    return false;
                                }
                            } else {
                                self.set_code(index, Reason::UnsupportedRangeRequest, false);
                                return false;
                            }
                        }
                        _ => {}
                    }
                }
                true
            }
            Err(e) => {
                error!("http client err is {:?}", e);
                match e.error_kind() {
                    ErrorKind::UserAborted => self.set_code(index, Reason::UserOperation, false),
                    ErrorKind::Timeout => {
                        self.set_code(index, Reason::ContinuousTaskTimeout, false)
                    }
                    ErrorKind::Request => self.set_code(index, Reason::RequestError, false),
                    ErrorKind::Redirect => self.set_code(index, Reason::RedirectError, false),
                    ErrorKind::Connect | ErrorKind::ConnectionUpgrade => {
                        if e.is_dns_error() {
                            self.set_code(index, Reason::Dns, false);
                        } else if e.is_tls_error() {
                            self.set_code(index, Reason::Ssl, false);
                        } else {
                            self.set_code(index, Reason::Tcp, false);
                        }
                    }
                    ErrorKind::BodyTransfer => self.handle_body_transfer_error().await,
                    _ => {
                        if format!("{}", e).contains("No space left on device") {
                            self.set_code(index, Reason::InsufficientSpace, false);
                        } else {
                            self.set_code(index, Reason::OthersError, false);
                        }
                    }
                }
                false
            }
        }
    }

    pub(crate) fn notify_response(&self, response: &Response) {
        let tid = self.conf.common_data.task_id;
        let version: String = response.version().as_str().into();
        let status_code: u32 = response.status().as_u16() as u32;
        let status_message: String;
        if let Some(reason) = response.status().reason() {
            status_message = reason.into();
        } else {
            error!("bad status_message {:?}", status_code);
            return;
        }
        let headers = response.headers().clone();
        debug!("notify_response");
        self.client_manager
            .send_response(tid, version, status_code, status_message, headers)
    }

    pub(crate) fn record_response_header(&self, response: &Result<Response, HttpClientError>) {
        if let Ok(r) = response {
            self.notify_response(r);
            let mut guard = self.progress.lock().unwrap();
            guard.extras.clear();
            for (k, v) in r.headers() {
                if let Ok(value) = v.to_string() {
                    guard.extras.insert(k.to_string().to_lowercase(), value);
                }
            }
        }
    }

    pub(crate) async fn record_upload_response(
        &self,
        index: usize,
        response: Result<Response, HttpClientError>,
    ) {
        self.record_response_header(&response);
        if let Ok(mut r) = response {
            let file = match self.body_files.get_mut(index) {
                Some(file) => file,
                None => return,
            };
            let _ = file.set_len(0).await;
            loop {
                let mut buf = [0u8; 1024];
                let size = r.data(&mut buf).await;
                let size = match size {
                    Ok(size) => size,
                    Err(_e) => break,
                };

                if size == 0 {
                    break;
                }
                let _ = file.write_all(&buf[..size]).await;
            }
            // Makes sure all the data has been written to the target file.
            let _ = file.sync_all().await;
        }
    }

    pub(crate) fn set_code(&self, index: usize, code: Reason, is_force: bool) {
        // why?
        if code == Reason::UploadFileError {
            return;
        }
        // `unwrap` for propagating panics among threads.
        let mut codes_guard = self.code.lock().unwrap();
        match codes_guard.get_mut(index) {
            Some(reason) => {
                if is_force || *reason == Reason::Default {
                    *reason = code;
                }
            }
            None => {
                info!(
                    "set code index error; tid: {}, index: {}, code: {:?}",
                    self.conf.common_data.task_id, index, code
                );
            }
        }
    }

    pub(crate) fn change_status(
        task_status: &mut MutexGuard<TaskStatus>,
        progress: &mut MutexGuard<Progress>,
        state: State,
        reason: Reason,
    ) {
        let time = get_current_timestamp();
        // TODO: other reason for waiting?
        if state == State::Waiting {
            task_status.waiting_network_time = Some(time);
        }
        task_status.mtime = time;
        task_status.state = state;
        task_status.reason = reason;
        progress.common_data.state = state.repr;
        debug!("current state is {:?}, reason is {:?}", state, reason);
    }

    pub(crate) fn change_task_status(&self, to_state: State, to_reason: Reason) -> ErrorCode {
        let mut task_status = self.status.lock().unwrap();
        let mut progress = self.progress.lock().unwrap();
        if !State::check_change(task_status.state, to_state) {
            return ErrorCode::TaskStateErr;
        }
        RequestTask::change_status(&mut task_status, &mut progress, to_state, to_reason);
        let index: usize = progress.common_data.index;
        if to_state == State::Failed {
            let file_counts = self.conf.file_specs.len();
            for i in index..file_counts {
                self.set_code(i, to_reason, false);
            }
        } else {
            self.set_code(index, to_reason, false);
        }
        let codes_guard = self.code.lock().unwrap();
        let update_info = UpdateInfo {
            mtime: task_status.mtime,
            reason: task_status.reason.repr,
            progress: progress.clone(),
            each_file_status: RequestTask::get_each_file_status_by_code(
                &codes_guard,
                &self.conf.file_specs,
            ),
            tries: self.tries.load(Ordering::SeqCst),
            mime_type: self.mime_type.lock().unwrap().clone(),
        };
        Database::get_instance().update_task(self.task_id(), update_info);
        ErrorCode::ErrOk
    }

    pub(crate) fn state_change_notify(&self) {
        let state = self.status.lock().unwrap().state;
        let total_processed = self.progress.lock().unwrap().common_data.total_processed;

        if state == State::Initialized
            || (total_processed == 0 && (state == State::Running || state == State::Retrying))
        {
            return;
        }

        debug!("state change notification");
        let notify_data = self.build_notify_data();

        Notifier::progress(&self.client_manager, notify_data.clone());
        match state {
            State::Completed => {
                let _ = publish_state_change_event(
                    self.conf.bundle.as_str(),
                    self.conf.common_data.task_id,
                    State::Completed.repr as i32,
                );

                Notifier::complete(&self.client_manager, notify_data)
            }
            State::Failed => {
                let _ = publish_state_change_event(
                    self.conf.bundle.as_str(),
                    self.conf.common_data.task_id,
                    State::Failed.repr as i32,
                );

                Notifier::fail(&self.client_manager, notify_data)
            }
            State::Paused | State::Waiting => Notifier::pause(&self.client_manager, notify_data),
            State::Removed => {
                Notifier::remove(&self.client_manager, notify_data);
            }
            _ => {}
        }
        self.background_notify();
    }

    pub(crate) fn state_change_notify_of_no_run(
        client_manager: &ClientManagerEntry,
        notify_data: NotifyData,
    ) {
        let state = State::from(notify_data.progress.common_data.state);
        let total_processed = notify_data.progress.common_data.total_processed;
        if state == State::Initialized
            || (total_processed == 0 && (state == State::Running || state == State::Retrying))
        {
            return;
        }
        debug!("no run task state change notification: {:?}", state);
        Notifier::progress(client_manager, notify_data.clone());
        match state {
            State::Completed => {
                let _ = publish_state_change_event(
                    notify_data.bundle.as_str(),
                    notify_data.task_id,
                    State::Completed.repr as i32,
                );
                Notifier::complete(client_manager, notify_data)
            }
            State::Failed => {
                let _ = publish_state_change_event(
                    notify_data.bundle.as_str(),
                    notify_data.task_id,
                    State::Failed.repr as i32,
                );
                Notifier::fail(client_manager, notify_data)
            }
            State::Paused | State::Waiting => Notifier::pause(client_manager, notify_data),
            State::Removed => {
                Notifier::remove(client_manager, notify_data);
            }
            _ => {}
        }
    }

    pub(crate) fn get_each_file_status(&self) -> Vec<EachFileStatus> {
        let mut vec = Vec::new();
        // `unwrap` for propagating panics among threads.
        let codes_guard = self.code.lock().unwrap();
        for (i, file_spec) in self.conf.file_specs.iter().enumerate() {
            let reason = *codes_guard.get(i).unwrap_or(&Reason::Default);
            vec.push(EachFileStatus {
                path: file_spec.path.clone(),
                reason,
                message: reason.to_str().into(),
            });
        }
        vec
    }

    pub(crate) fn get_each_file_status_by_code(
        codes_guard: &MutexGuard<Vec<Reason>>,
        file_specs: &[FileSpec],
    ) -> Vec<EachFileStatus> {
        let mut vec = Vec::new();
        for (i, file_spec) in file_specs.iter().enumerate() {
            let reason = *codes_guard.get(i).unwrap_or(&Reason::Default);
            vec.push(EachFileStatus {
                path: file_spec.path.clone(),
                reason,
                message: reason.to_str().into(),
            });
        }
        vec
    }

    pub(crate) fn info(&self) -> TaskInfo {
        let status = self.status.lock().unwrap();
        let progress = self.progress.lock().unwrap();
        TaskInfo {
            bundle: self.conf.bundle.clone(),
            url: self.conf.url.clone(),
            data: self.conf.data.clone(),
            token: self.conf.token.clone(),
            form_items: self.conf.form_items.clone(),
            file_specs: self.conf.file_specs.clone(),
            title: self.conf.title.clone(),
            description: self.conf.description.clone(),
            mime_type: {
                match self.conf.version {
                    Version::API10 => match self.conf.common_data.action {
                        Action::Download => match self.conf.headers.get("Content-Type") {
                            None => "".into(),
                            Some(v) => v.clone(),
                        },
                        Action::Upload => "multipart/form-data".into(),
                        _ => "".into(),
                    },
                    Version::API9 => self.mime_type.lock().unwrap().clone(),
                }
            },
            progress: progress.clone(),
            extras: progress.extras.clone(),
            each_file_status: self.get_each_file_status(),
            common_data: CommonTaskInfo {
                task_id: self.conf.common_data.task_id,
                uid: self.conf.common_data.uid,
                action: self.conf.common_data.action.repr,
                mode: self.conf.common_data.mode.repr,
                ctime: self.ctime,
                mtime: status.mtime,
                reason: status.reason.repr,
                gauge: self.conf.common_data.gauge,
                retry: match self.conf.common_data.mode {
                    Mode::FrontEnd => false,
                    _ => self.conf.common_data.retry,
                },
                tries: self.tries.load(Ordering::SeqCst),
                version: self.conf.version as u8,
                priority: self.conf.common_data.priority,
            },
        }
    }

    pub(crate) fn is_satisfied_configuration(&self) -> bool {
        self.network.satisfied_state(&self.conf)
    }

    pub(crate) fn background_notify(&self) {
        if self.conf.version == Version::API9 && !self.conf.common_data.background {
            return;
        }
        if self.conf.version == Version::API10 && self.conf.common_data.mode == Mode::FrontEnd {
            return;
        }
        let mut file_total_size = self.file_total_size.load(Ordering::SeqCst);
        let total_processed = self.progress.lock().unwrap().common_data.total_processed as u64;
        if file_total_size <= 0 || total_processed == 0 {
            return;
        }
        if self.conf.common_data.action == Action::Download {
            if self.conf.common_data.ends < 0 {
                file_total_size -= self.conf.common_data.begins as i64;
            } else {
                file_total_size =
                    self.conf.common_data.ends - self.conf.common_data.begins as i64 + 1;
            }
        }
        self.background_notify_time
            .store(get_current_timestamp(), Ordering::SeqCst);
        let index = self.progress.lock().unwrap().common_data.index;
        if index >= self.conf.file_specs.len() {
            return;
        }
        let percent = total_processed * 100 / (file_total_size as u64);
        debug!("background notify");
        let task_msg = RequestTaskMsg {
            task_id: self.conf.common_data.task_id,
            uid: self.conf.common_data.uid as i32,
            action: self.conf.common_data.action.repr,
        };

        let path = self.conf.file_specs[index].path.as_str();
        let file_name = self.conf.file_specs[index].file_name.as_str();
        let _ = request_background_notify(task_msg, path, file_name, percent as u32);
    }

    pub(crate) fn get_upload_info(&self, index: usize) -> (bool, u64) {
        let guard = self.progress.lock().unwrap();
        let file_size = guard.sizes[index];
        let mut is_partial_upload = false;
        let mut upload_file_length: u64 = file_size as u64 - guard.processed[index] as u64;
        if file_size == 0 {
            return (is_partial_upload, upload_file_length);
        }
        if index as u32 != self.conf.common_data.index {
            return (is_partial_upload, upload_file_length);
        }
        let begins = self.conf.common_data.begins;
        let mut ends = self.conf.common_data.ends;
        if ends < 0 || ends >= file_size {
            ends = file_size - 1;
        }
        if begins >= file_size as u64 || begins > ends as u64 {
            return (is_partial_upload, upload_file_length);
        }
        is_partial_upload = true;
        upload_file_length = ends as u64 - begins + 1 - guard.processed[index] as u64;
        (is_partial_upload, upload_file_length)
    }

    pub(crate) fn notify_header_receive(&self) {
        if self.conf.version == Version::API9 && self.conf.common_data.action == Action::Upload {
            let notify_data = self.build_notify_data();

            Notifier::header_receive(&self.client_manager, notify_data);
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TaskStatus {
    pub(crate) waiting_network_time: Option<u64>,
    pub(crate) mtime: u64,
    pub(crate) state: State,
    pub(crate) reason: Reason,
}

impl TaskStatus {
    pub(crate) fn new(mtime: u64) -> Self {
        TaskStatus {
            waiting_network_time: None,
            mtime,
            state: State::Initialized,
            reason: Reason::Default,
        }
    }
}

fn check_file_specs(file_specs: &[FileSpec]) -> bool {
    const EL1: &str = "/data/storage/el1/base/";
    const EL2: &str = "/data/storage/el2/base/";

    let mut result = true;
    for (idx, spec) in file_specs.iter().enumerate() {
        let path = &spec.path;
        if !spec.is_user_file && !path.starts_with(EL1) && !path.starts_with(EL2) {
            error!("File path invalid - path: {}, idx: {}", path, idx);
            result = false;
            break;
        }
    }

    result
}

pub(crate) fn check_config(
    config: &TaskConfig,
    system: SystemConfig,
) -> Result<(AttachedFiles, Client), ErrorCode> {
    if !check_file_specs(&config.file_specs) {
        return Err(ErrorCode::Other);
    }
    let files = AttachedFiles::open(config).map_err(|_| ErrorCode::FileOperationErr)?;
    let client = build_client(config, system).map_err(|_| ErrorCode::Other)?;
    Ok((files, client))
}
