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

use std::cell::UnsafeCell;
use std::ffi::{c_char, CString};
use std::fs::File;
use std::io::{Read, SeekFrom};
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU32, AtomicU64, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use ylong_http_client::async_impl::Client;
use ylong_http_client::{
    Body, Certificate, ErrorKind, HttpClientError, Method, Redirect, Request, RequestBuilder,
    Response, Timeout, TlsVersion,
};
use ylong_runtime::fs::File as YlongFile;
use ylong_runtime::io::{AsyncSeekExt, AsyncWriteExt};

use super::config::{Network, Version};
use super::download::download;
use super::ffi::{HasRequestTaskRecord, PublishStateChangeEvents};
use super::info::{CommonTaskInfo, Mode, State, TaskInfo, UpdateInfo};
use super::notify::{EachFileStatus, NotifyData, Progress};
use super::reason::Reason;
use super::upload::upload;
use crate::manager::monitor::IsOnline;
use crate::task::config::{Action, TaskConfig};
use crate::task::ffi::{
    GetNetworkInfo, RecordRequestTaskInfo, RequestBackgroundNotify, RequestTaskMsg,
    UpdateRequestTaskInfo,
};
use crate::utils::{get_current_timestamp, hashmap_to_string};

cfg_oh! {
    use crate::service::{open_file_readonly, open_file_readwrite, convert_path};
    use crate::manager::Notifier;
}

const SECONDS_IN_ONE_WEEK: u64 = 7 * 24 * 60 * 60;

const CONNECT_TIMEOUT: u64 = 60;
const RETRY_INTERVAL: u64 = 20;

#[derive(Clone, Debug)]
pub(crate) struct TaskStatus {
    pub(crate) waitting_network_time: Option<u64>,
    pub(crate) mtime: u64,
    pub(crate) state: State,
    pub(crate) reason: Reason,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus {
            waitting_network_time: None,
            mtime: get_current_timestamp(),
            state: State::Created,
            reason: Reason::Default,
        }
    }
}

pub(crate) struct Files(pub(crate) UnsafeCell<Vec<YlongFile>>);

impl Files {
    pub(crate) fn get(&self, index: usize) -> Option<&YlongFile> {
        unsafe { &*self.0.get() }.get(index)
    }
}

unsafe impl Sync for Files {}
unsafe impl Send for Files {}

// Need to release file timely.
pub(crate) struct BodyFiles(UnsafeCell<Vec<Option<YlongFile>>>);
unsafe impl Sync for BodyFiles {}
unsafe impl Send for BodyFiles {}

pub(crate) struct RequestTask {
    pub(crate) conf: TaskConfig,
    pub(crate) ctime: u64,
    pub(crate) mime_type: Mutex<String>,
    pub(crate) progress: Mutex<Progress>,
    pub(crate) tries: AtomicU32,
    pub(crate) status: Mutex<TaskStatus>,
    pub(crate) retry: AtomicBool,
    pub(crate) get_file_info: AtomicBool,
    pub(crate) retry_for_request: AtomicBool,
    #[allow(unused)]
    pub(crate) retry_for_speed: AtomicBool,
    pub(crate) code: Mutex<Vec<Reason>>,
    pub(crate) background_notify_time: AtomicU64,
    pub(crate) file_total_size: AtomicI64,
    pub(crate) resume: AtomicBool,
    pub(crate) files: Files,
    pub(crate) body_files: BodyFiles,
    pub(crate) seek_flag: AtomicBool,
    pub(crate) range_request: AtomicBool,
    pub(crate) range_response: AtomicBool,
    pub(crate) restored: AtomicBool,
    pub(crate) skip_bytes: AtomicU64,
    pub(crate) upload_counts: AtomicU32,
    pub(crate) client: Option<Client>,
    pub(crate) recording_rdb_num: Arc<AtomicU32>,
    pub(crate) rate_limiting: AtomicBool,
    pub(crate) app_state: Arc<AtomicU8>,
    pub(crate) last_notify: AtomicU64,
}

impl RequestTask {
    pub(crate) fn constructor(
        conf: TaskConfig,
        files: Vec<File>,
        body_files: Vec<File>,
        recording_rdb_num: Arc<AtomicU32>,
        rate_limiting: AtomicBool,
        app_state: Arc<AtomicU8>,
    ) -> Self {
        let mut sizes = Vec::new();
        match conf.common_data.action {
            Action::DownLoad => sizes.push(-1),
            Action::UpLoad => {
                for f in files.iter() {
                    let file_size = f.metadata().unwrap().len() as i64;
                    debug!("file size size is {}", file_size);
                    sizes.push(file_size);
                }
            }
            _ => {}
        }
        let file_count = files.len();
        let action = conf.common_data.action;

        let mut task = RequestTask {
            conf,
            ctime: get_current_timestamp(),
            files: Files(UnsafeCell::new(
                files.into_iter().map(YlongFile::new).collect(),
            )),
            body_files: BodyFiles(UnsafeCell::new(
                body_files
                    .into_iter()
                    .map(|f| Some(YlongFile::new(f)))
                    .collect(),
            )),
            mime_type: Mutex::new(String::new()),
            progress: Mutex::new(Progress::new(sizes)),
            tries: AtomicU32::new(0),
            status: Mutex::new(TaskStatus::default()),
            retry: AtomicBool::new(false),
            get_file_info: AtomicBool::new(false),
            retry_for_request: AtomicBool::new(false),
            retry_for_speed: AtomicBool::new(false),
            code: Mutex::new(vec![Reason::Default; file_count]),
            background_notify_time: AtomicU64::new(get_current_timestamp()),
            file_total_size: AtomicI64::new(-1),
            resume: AtomicBool::new(false),
            seek_flag: AtomicBool::new(false),
            range_request: AtomicBool::new(false),
            range_response: AtomicBool::new(false),
            restored: AtomicBool::new(false),
            skip_bytes: AtomicU64::new(0),
            upload_counts: AtomicU32::new(0),
            client: None,
            recording_rdb_num,
            rate_limiting,
            app_state,
            last_notify: AtomicU64::new(get_current_timestamp()),
        };
        task.client = task.build_client();

        if action == Action::UpLoad {
            task.file_total_size
                .store(task.get_upload_file_total_size() as i64, Ordering::SeqCst);
        }
        task
    }

    pub(crate) fn restore_task(
        conf: TaskConfig,
        info: TaskInfo,
        recording_rdb_num: Arc<AtomicU32>,
        rate_limiting: AtomicBool,
        app_state: Arc<AtomicU8>,
    ) -> Self {
        let progress_index = info.progress.common_data.index;
        let uid = info.common_data.uid;
        let action = conf.common_data.action;
        let files = get_restore_files(&conf, uid);
        let body_files = get_restore_body_files(&conf, uid);
        let file_count = files.len();

        let mut task = RequestTask {
            conf,
            ctime: info.common_data.ctime,
            files: Files(UnsafeCell::new(
                files.into_iter().map(YlongFile::new).collect(),
            )),
            body_files: BodyFiles(UnsafeCell::new(
                body_files
                    .into_iter()
                    .map(|f| Some(YlongFile::new(f)))
                    .collect(),
            )),
            mime_type: Mutex::new(info.mime_type),
            progress: Mutex::new(info.progress.clone()),
            tries: AtomicU32::new(info.common_data.tries),
            status: Mutex::new(TaskStatus {
                waitting_network_time: None,
                mtime: get_current_timestamp(),
                state: State::from(info.progress.common_data.state),
                reason: Reason::from(info.common_data.reason),
            }),
            retry: AtomicBool::new(info.common_data.retry),
            get_file_info: AtomicBool::new(false),
            retry_for_request: AtomicBool::new(false),
            retry_for_speed: AtomicBool::new(false),
            code: Mutex::new(vec![Reason::Default; file_count]),
            background_notify_time: AtomicU64::new(get_current_timestamp()),
            file_total_size: AtomicI64::new(-1),
            resume: AtomicBool::new(false),
            seek_flag: AtomicBool::new(false),
            range_request: AtomicBool::new(false),
            range_response: AtomicBool::new(false),
            restored: AtomicBool::new(true),
            skip_bytes: AtomicU64::new(0),
            upload_counts: AtomicU32::new(progress_index as u32),
            client: None,
            recording_rdb_num,
            rate_limiting,
            app_state,
            last_notify: AtomicU64::new(get_current_timestamp()),
        };
        task.client = task.build_client();
        match action {
            Action::UpLoad => task
                .file_total_size
                .store(task.get_upload_file_total_size() as i64, Ordering::SeqCst),
            Action::DownLoad => task
                .file_total_size
                .store(info.progress.sizes[progress_index], Ordering::SeqCst),
            _ => {}
        }
        task
    }

    pub(crate) fn build_notify_data(&self) -> NotifyData {
        let mut vec = Vec::new();
        let size = self.conf.file_specs.len();
        let guard = self.code.lock().unwrap();
        for i in 0..size {
            vec.push(EachFileStatus {
                path: self.conf.file_specs[i].path.clone(),
                reason: guard[i],
                message: guard[i].to_str().into(),
            });
        }
        NotifyData {
            progress: self.progress.lock().unwrap().clone(),
            action: self.conf.common_data.action,
            version: self.conf.version,
            each_file_status: vec,
            task_id: self.conf.common_data.task_id,
            _uid: self.conf.common_data.uid,
            _bundle: self.conf.bundle.clone(),
        }
    }

    pub(crate) fn record_waitting_network_time(&self) {
        let mut staus = self.status.lock().unwrap();
        staus.waitting_network_time = Some(get_current_timestamp());
    }

    pub(crate) fn check_net_work_status(&self) -> bool {
        if !self.is_satisfied_configuration() {
            if self.conf.version == Version::API10
                && self.conf.common_data.mode == Mode::BackGround
                && self.conf.common_data.retry
            {
                self.set_status(State::Waiting, Reason::UnSupportedNetWorkType);
            } else {
                self.set_status(State::Failed, Reason::UnSupportedNetWorkType);
            }
            return false;
        }
        true
    }

    pub(crate) fn net_work_online(&self) -> bool {
        if unsafe { !IsOnline() } {
            if self.conf.version == Version::API10
                && self.conf.common_data.mode == Mode::BackGround
                && self.conf.common_data.retry
            {
                self.set_status(State::Waiting, Reason::NetWorkOffline);
            } else {
                let retry_times = 20;
                for _ in 0..retry_times {
                    if unsafe { IsOnline() } {
                        return true;
                    }
                    sleep(Duration::from_millis(RETRY_INTERVAL));
                }
                self.set_status(State::Failed, Reason::NetWorkOffline);
            }
            return false;
        }
        true
    }

    fn build_client(&self) -> Option<Client> {
        let mut client = Client::builder()
            .connect_timeout(Timeout::from_secs(CONNECT_TIMEOUT))
            .request_timeout(Timeout::from_secs(SECONDS_IN_ONE_WEEK))
            .min_tls_version(TlsVersion::TLS_1_2);

        if self.conf.common_data.redirect {
            client = client.redirect(Redirect::limited(usize::MAX));
        } else {
            client = client.redirect(Redirect::none());
        }

        if self.conf.url.contains("https") {
            let path_list = self.conf.certs_path.clone();
            if path_list.is_empty() {
                let mut buf = Vec::new();
                let file = File::open("/etc/ssl/certs/cacert.pem");
                match file {
                    Ok(mut f) => {
                        f.read_to_end(&mut buf).unwrap();
                        let cert = Certificate::from_pem(&buf).unwrap();
                        client = client.add_root_certificate(cert);
                    }
                    Err(e) => {
                        error!("open cacert.pem failed, error is {:?}", e);
                        self.set_status(State::Failed, Reason::IoError);
                        return None;
                    }
                }
            } else {
                for path in path_list.into_iter() {
                    let real_path = convert_path(self.conf.common_data.uid, &self.conf.bundle, &path);
                    let file = Certificate::from_path(&real_path);
                    match file {
                        Ok(c) => {
                            client = client.add_root_certificate(c);
                        }
                        Err(e) => {
                            debug!("open {:?} failed, reason is {:?}", path, e);
                            self.set_status(State::Failed, Reason::IoError);
                            return None;
                        }
                    }
                }
            }
        }
        let result = client.build();
        match result {
            Ok(value) => Some(value),
            Err(e) => {
                error!("build client error is {:?}", e);
                self.set_status(State::Failed, Reason::BuildClientFailed);
                None
            }
        }
    }

    pub(crate) fn build_request_builder(&self) -> RequestBuilder {
        let url = self.conf.url.clone();
        let method = match self.conf.method.to_uppercase().as_str() {
            "PUT" => "PUT",
            "POST" => "POST",
            "GET" => "GET",
            _ => match self.conf.common_data.action {
                Action::UpLoad => {
                    if self.conf.version == Version::API10 {
                        "PUT"
                    } else {
                        "POST"
                    }
                }
                Action::DownLoad => "GET",
                _ => "",
            },
        };
        let method = Method::try_from(method).unwrap();
        let mut request = RequestBuilder::new().method(method).url(url.as_str());
        for (key, value) in self.conf.headers.iter() {
            request = request.header(key.as_str(), value.as_str());
        }
        request
    }

    async fn clear_downloaded_file(&self) -> bool {
        let file = unsafe { &mut *self.files.0.get() }.get_mut(0).unwrap();
        let res = file.set_len(0).await;
        match res {
            Err(e) => {
                error!("clear download file error: {:?}", e);
                self.set_status(State::Failed, Reason::IoError);
                false
            }
            _ => {
                debug!("set len success");
                match file.seek(SeekFrom::Start(0)).await {
                    Err(e) => {
                        error!("seek err is {:?}", e);
                        self.set_status(State::Failed, Reason::IoError);
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

    pub(crate) async fn build_download_request(&self) -> Option<Request<String>> {
        let mut request_builder = self.build_request_builder();
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
            begins += self.progress.lock().unwrap().processed[0] as u64;
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
        let result = request_builder.body(self.conf.data.clone());
        match result {
            Ok(value) => Some(value),
            Err(e) => {
                error!("build download request error is {:?}", e);
                self.set_status(State::Failed, Reason::BuildRequestFailed);
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
            if let Ok(value) = mime_type.to_str() {
                *self.mime_type.lock().unwrap() = value;
            }
        }

        let content_length = response.headers().get("content-length");
        if let Some(len) = content_length {
            let length = len.to_str();
            match length {
                Ok(value) => {
                    let len = value.parse::<i64>();
                    match len {
                        Ok(v) => {
                            let mut guard = self.progress.lock().unwrap();
                            if !self.restored.load(Ordering::SeqCst) {
                                guard.sizes[0] = v;
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
                self.set_status(State::Failed, Reason::GetFileSizeFailed);
                return false;
            }
        }
        true
    }

    fn handle_body_transfer_error(&self) {
        if unsafe { !IsOnline() } {
            match self.conf.version {
                Version::API9 => {
                    if self.conf.common_data.action == Action::DownLoad {
                        self.set_status(State::Waiting, Reason::NetWorkOffline);
                    } else {
                        self.set_status(State::Failed, Reason::NetWorkOffline);
                    }
                }
                Version::API10 => {
                    if self.conf.common_data.mode == Mode::FrontEnd || !self.conf.common_data.retry
                    {
                        self.set_status(State::Failed, Reason::NetWorkOffline);
                    } else {
                        self.set_status(State::Waiting, Reason::NetWorkOffline);
                    }
                }
            }
        } else {
            let index = self.progress.lock().unwrap().common_data.index;
            self.set_code(index, Reason::OthersError);
        }
    }

    pub(crate) fn handle_download_error(&self, result: &Result<(), HttpClientError>) -> bool {
        match result {
            Ok(_) => true,
            Err(err) => {
                error!("download err is {:?}", err);
                match err.error_kind() {
                    ErrorKind::Timeout => {
                        self.set_status(State::Failed, Reason::ContinuousTaskTimeOut);
                    }
                    // user triggered
                    ErrorKind::UserAborted => return true,
                    ErrorKind::BodyTransfer => self.handle_body_transfer_error(),
                    _ => {
                        self.set_status(State::Failed, Reason::OthersError);
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
                info!("the http response code is {}", http_response_code);
                if http_response_code.is_server_error()
                    || (http_response_code.as_u16() != 408 && http_response_code.is_client_error())
                    || http_response_code.is_redirection()
                {
                    self.set_code(index, Reason::ProtocolError);
                    return false;
                }
                if http_response_code.as_u16() == 408 {
                    if !self.retry_for_request.load(Ordering::SeqCst) {
                        self.retry_for_request.store(true, Ordering::SeqCst);
                    } else {
                        self.set_code(index, Reason::ProtocolError);
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
                                self.set_code(index, Reason::UnSupportRangeRequest);
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
                    ErrorKind::UserAborted => self.set_code(index, Reason::UserOperation),
                    ErrorKind::Timeout => self.set_code(index, Reason::ContinuousTaskTimeOut),
                    ErrorKind::Request => self.set_code(index, Reason::RequestError),
                    ErrorKind::Redirect => self.set_code(index, Reason::RedirectError),
                    ErrorKind::Connect | ErrorKind::ConnectionUpgrade => {
                        self.set_code(index, Reason::ConnectError)
                    }
                    ErrorKind::BodyTransfer => self.handle_body_transfer_error(),
                    _ => self.set_code(index, Reason::OthersError),
                }
                false
            }
        }
    }

    pub(crate) fn record_response_header(&self, response: &Result<Response, HttpClientError>) {
        if let Ok(r) = response {
            let mut guard = self.progress.lock().unwrap();
            guard.extras.clear();
            for (k, v) in r.headers() {
                if let Ok(value) = v.to_str() {
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
            let mut yfile = match unsafe { &mut *self.body_files.0.get() }.get_mut(index) {
                Some(yfile) => match yfile.take() {
                    Some(yf) => yf,
                    None => return,
                },
                None => return,
            };

            loop {
                let mut buf = [0u8; 1024];
                let size = r.body_mut().data(&mut buf).await;
                let size = match size {
                    Ok(size) => size,
                    Err(_e) => break,
                };

                if size == 0 {
                    break;
                }
                let _ = yfile.write_all(&buf[..size]).await;
            }
            // Makes sure all the data has been written to the target file.
            let _ = yfile.sync_all().await;
        }
        if self.conf.version == Version::API9 && self.conf.common_data.action == Action::UpLoad {
            let notify_data = self.build_notify_data();
            #[cfg(feature = "oh")]
            Notifier::service_front_notify("headerReceive".into(), notify_data, &self.app_state);
        }
    }

    fn set_code(&self, index: usize, code: Reason) {
        if code == Reason::UploadFileError {
            return;
        }
        let mut code_guard = self.code.lock().unwrap();
        if index < code_guard.len() && code_guard[index] == Reason::Default {
            debug!("set code");
            code_guard[index] = code;
        }
    }

    pub(crate) fn reset_code(&self, index: usize) {
        let file_counts = self.conf.file_specs.len();
        let mut code_guard = self.code.lock().unwrap();
        if index < file_counts {
            debug!("reset code");
            code_guard[index] = Reason::Default;
        }
    }

    pub(crate) fn set_status(&self, state: State, reason: Reason) -> bool {
        debug!("set status");
        {
            let mut current_status = self.status.lock().unwrap();
            if state == current_status.state && reason == current_status.reason {
                return true;
            }
            let mut progress_guard = self.progress.lock().unwrap();
            let index = progress_guard.common_data.index;
            let current_state = current_status.state;
            debug!(
                "set state {:?}, reason {:?} current_state {:?}",
                state, reason, current_state
            );
            match state {
                State::Paused | State::Stopped => {
                    if current_state != State::Running
                        && current_state != State::Retrying
                        && current_state != State::Waiting
                    {
                        return false;
                    }
                    self.set_code(index, reason);
                }
                State::Completed => {
                    if current_state != State::Running && current_state != State::Retrying {
                        return false;
                    }
                }
                State::Failed | State::Waiting => {
                    if current_state == State::Completed
                        || current_state == State::Removed
                        || current_state == State::Stopped
                        || current_state == State::Failed
                    {
                        return false;
                    }
                    self.set_code(index, reason);
                    if state == State::Failed {
                        let file_counts = self.conf.file_specs.len();
                        for i in index..file_counts {
                            self.set_code(i, reason);
                        }
                    }
                }
                State::Removed => self.set_code(index, reason),
                _ => {}
            }
            current_status.mtime = get_current_timestamp();
            progress_guard.common_data.state = state as u8;
            current_status.state = state;
            current_status.reason = reason;
            info!("current state is {:?}, reason is {:?}", state, reason);
        }
        if state == State::Waiting {
            self.record_waitting_network_time();
        }
        self.record_task_info();
        self.state_change_notify(state);
        true
    }

    fn state_change_notify(&self, state: State) {
        if state == State::Initialized
            || (self.progress.lock().unwrap().common_data.total_processed == 0
                && (state == State::Running || state == State::Retrying))
        {
            return;
        }

        debug!("state change notification");
        let notify_data = self.build_notify_data();
        #[cfg(feature = "oh")]
        Notifier::service_front_notify("progress".into(), notify_data.clone(), &self.app_state);
        let bundle = CString::new(self.conf.bundle.as_str()).unwrap();
        match state {
            State::Completed => {
                unsafe {
                    PublishStateChangeEvents(
                        bundle.as_ptr(),
                        self.conf.bundle.len() as u32,
                        self.conf.common_data.task_id,
                        State::Completed as i32,
                    );
                }
                #[cfg(feature = "oh")]
                Notifier::service_front_notify("complete".into(), notify_data, &self.app_state)
            }
            State::Failed => {
                unsafe {
                    PublishStateChangeEvents(
                        bundle.as_ptr(),
                        self.conf.bundle.len() as u32,
                        self.conf.common_data.task_id,
                        State::Failed as i32,
                    );
                }
                #[cfg(feature = "oh")]
                Notifier::service_front_notify("fail".into(), notify_data, &self.app_state)
            }
            State::Paused | State::Waiting =>
            {
                #[cfg(feature = "oh")]
                Notifier::service_front_notify("pause".into(), notify_data, &self.app_state)
            }
            _ => {}
        }
        self.background_notify();
    }

    fn record_task_info(&self) {
        debug!(
            "RequestTask record task info, task_id:{}",
            self.conf.common_data.task_id
        );

        self.recording_rdb_num.fetch_add(1, Ordering::SeqCst);

        let has_record = unsafe { HasRequestTaskRecord(self.conf.common_data.task_id) };
        if !has_record {
            let task_info = self.show();
            let info_set = task_info.build_info_set();
            let c_task_info = task_info.to_c_struct(&info_set);
            let ret = unsafe { RecordRequestTaskInfo(&c_task_info) };
            info!("insert database ret is {}", ret);
        } else {
            let update_info = self.get_update_info();
            let sizes: String = format!("{:?}", update_info.progress.sizes);
            let processed: String = format!("{:?}", update_info.progress.processed);
            let extras = hashmap_to_string(&update_info.progress.extras);
            let each_file_status = update_info
                .each_file_status
                .iter()
                .map(|x| x.to_c_struct())
                .collect();
            let c_update_info =
                update_info.to_c_struct(&sizes, &processed, &extras, &each_file_status);
            let ret =
                unsafe { UpdateRequestTaskInfo(self.conf.common_data.task_id, &c_update_info) };
            debug!("update database ret is {}", ret);
        }

        self.recording_rdb_num.fetch_sub(1, Ordering::SeqCst);
    }

    fn get_each_file_status(&self) -> Vec<EachFileStatus> {
        let mut vec = Vec::new();
        let size = self.conf.file_specs.len();
        let guard = self.code.lock().unwrap();
        for i in 0..size {
            vec.push(EachFileStatus {
                path: self.conf.file_specs[i].path.clone(),
                reason: guard[i],
                message: guard[i].to_str().into(),
            });
        }
        vec
    }

    fn get_update_info(&self) -> UpdateInfo {
        let status = self.status.lock().unwrap();
        let progress = self.progress.lock().unwrap();
        UpdateInfo {
            mtime: status.mtime,
            reason: status.reason as u8,
            tries: self.tries.load(Ordering::SeqCst),
            mime_type: self.mime_type.lock().unwrap().clone(),
            progress: progress.clone(),
            each_file_status: self.get_each_file_status(),
        }
    }

    pub(crate) fn show(&self) -> TaskInfo {
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
                        Action::DownLoad => match self.conf.headers.get("Content-Type") {
                            None => "".into(),
                            Some(v) => v.clone(),
                        },
                        Action::UpLoad => "multipart/form-data".into(),
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
                action: self.conf.common_data.action as u8,
                mode: self.conf.common_data.mode as u8,
                ctime: self.ctime,
                mtime: status.mtime,
                reason: status.reason as u8,
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

    // only use for download task
    pub(crate) fn query_mime_type(&self) -> String {
        self.mime_type.lock().unwrap().clone()
    }

    pub(crate) fn is_satisfied_configuration(&self) -> bool {
        if self.conf.common_data.network == Network::Any {
            return true;
        }
        unsafe {
            let network_info = GetNetworkInfo();
            if !self.conf.common_data.roaming && (*network_info).is_roaming {
                error!("not allow roaming");
                return false;
            }
            if !self.conf.common_data.metered && (*network_info).is_metered {
                error!("not allow metered");
                return false;
            }
            if (*network_info).network_type != self.conf.common_data.network {
                error!("dismatch network type");
                return false;
            }
        };
        true
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
        if self.conf.common_data.action == Action::DownLoad {
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
        let file_path = self.conf.file_specs[index].path.as_ptr() as *const c_char;
        let file_path_len = self.conf.file_specs[index].path.as_bytes().len() as i32;
        let percent = total_processed * 100 / (file_total_size as u64);
        debug!("background notify");
        let task_msg = RequestTaskMsg {
            task_id: self.conf.common_data.task_id,
            uid: self.conf.common_data.uid as i32,
            action: self.conf.common_data.action as u8,
        };
        unsafe {
            RequestBackgroundNotify(task_msg, file_path, file_path_len, percent as u32);
        };
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

    fn get_upload_file_total_size(&self) -> u64 {
        let mut file_total_size = 0;
        for i in 0..self.conf.file_specs.len() {
            let (_, upload_size) = self.get_upload_info(i);
            file_total_size += upload_size;
        }
        file_total_size
    }
}

pub(crate) async fn run(task: Arc<RequestTask>) {
    info!("run the task which id is {}", task.conf.common_data.task_id);
    if !task.net_work_online() || !task.check_net_work_status() {
        return;
    }
    let action = task.conf.common_data.action;
    match action {
        Action::DownLoad => loop {
            task.reset_code(0);

            download(task.clone()).await;

            let state = task.status.lock().unwrap().state;
            if state != State::Running && state != State::Retrying {
                break;
            }
            let code = task.code.lock().unwrap()[0];
            if code != Reason::Default {
                task.set_status(State::Failed, code);
                break;
            }
        },
        Action::UpLoad => {
            let state = task.status.lock().unwrap().state;
            if state == State::Retrying {
                let index = {
                    let mut progress_guard = task.progress.lock().unwrap();
                    let index = progress_guard.common_data.index;
                    progress_guard.common_data.total_processed -= progress_guard.processed[index];
                    progress_guard.processed[index] = 0;
                    index
                };
                let file = unsafe { &mut *task.files.0.get() }.get_mut(index).unwrap();
                let mut begins = task.conf.common_data.begins;
                let (is_partial_upload, _) = task.get_upload_info(index);
                if !is_partial_upload {
                    begins = 0;
                }
                if let Err(e) = file.seek(SeekFrom::Start(begins)).await {
                    task.set_code(index, Reason::IoError);
                    error!("seek err is {:?}", e);
                }
            }
            upload(task.clone()).await;
        }
        _ => {}
    }
    info!("run end");
}

fn get_restore_files(conf: &TaskConfig, uid: u64) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();

    #[cfg(feature = "oh")]
    for fs in &conf.file_specs {
        if conf.common_data.action == Action::UpLoad {
            match open_file_readonly(uid, &conf.bundle, &fs.path) {
                Ok(file) => files.push(file),
                Err(e) => {
                    error!("open file RO failed, err is {:?}", e);
                }
            }
        } else {
            match open_file_readwrite(uid, &conf.bundle, &fs.path) {
                Ok(file) => files.push(file),
                Err(e) => {
                    error!("open file RW failed, err is {:?}", e);
                }
            }
        }
    }
    files
}

fn get_restore_body_files(conf: &TaskConfig, uid: u64) -> Vec<File> {
    let mut body_files: Vec<File> = Vec::new();

    #[cfg(feature = "oh")]
    for name in &conf.body_file_names {
        match open_file_readwrite(uid, &conf.bundle, name) {
            Ok(body_file) => body_files.push(body_file),
            Err(e) => {
                error!("open body_file failed, err is {:?}", e);
            }
        }
    }
    body_files
}
