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

use std::io::{self, SeekFrom};
use std::sync::atomic::{AtomicI64, AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;

use ylong_http_client::async_impl::{Body, Client, Request, RequestBuilder, Response};
use ylong_http_client::{ErrorKind, HttpClientError};
use ylong_runtime::io::{AsyncSeekExt, AsyncWriteExt};

cfg_oh! {
    use crate::manage::SystemConfig;
    use crate::utils::{request_background_notify, RequestTaskMsg};
}

use super::config::{Mode, Version};
use super::info::{CommonTaskInfo, State, TaskInfo, UpdateInfo};
use super::notify::{EachFileStatus, NotifyData, Progress};
use super::reason::Reason;
use crate::error::ErrorCode;
use crate::manage::database::RequestDb;
use crate::manage::network::Network;
use crate::manage::notifier::Notifier;
use crate::service::client::ClientManagerEntry;
use crate::task::client::build_client;
use crate::task::config::{Action, TaskConfig};
use crate::task::files::{AttachedFiles, Files};
use crate::utils::form_item::FileSpec;
use crate::utils::get_current_timestamp;

const RETRY_TIMES: u32 = 4;
const RETRY_INTERVAL: u64 = 400;

pub(crate) struct RequestTask {
    pub(crate) conf: TaskConfig,
    pub(crate) client: Client,
    pub(crate) files: Files,
    pub(crate) body_files: Files,
    pub(crate) ctime: u64,
    pub(crate) mime_type: Mutex<String>,
    pub(crate) progress: Mutex<Progress>,
    pub(crate) status: Mutex<TaskStatus>,
    pub(crate) code: Mutex<Vec<Reason>>,
    pub(crate) tries: AtomicU32,
    pub(crate) background_notify_time: AtomicU64,
    pub(crate) file_total_size: AtomicI64,
    pub(crate) rate_limiting: AtomicU64,
    pub(crate) last_notify: AtomicU64,
    pub(crate) client_manager: ClientManagerEntry,
    pub(crate) running_result: Mutex<Option<Result<(), Reason>>>,
    pub(crate) network: Network,
    pub(crate) timeout_tries: AtomicU32,
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

    pub(crate) async fn network_retry(&self) -> Result<(), TaskError> {
        if self.tries.load(Ordering::SeqCst) < RETRY_TIMES {
            self.tries.fetch_add(1, Ordering::SeqCst);
            if !self.network.is_online() {
                return Err(TaskError::Waiting(TaskPhase::NetworkOffline));
            } else {
                ylong_runtime::time::sleep(Duration::from_millis(RETRY_INTERVAL)).await;
                return Err(TaskError::Waiting(TaskPhase::NeedRetry));
            }
        }
        Ok(())
    }
}

impl RequestTask {
    pub(crate) fn new(
        config: TaskConfig,
        files: AttachedFiles,
        client: Client,
        client_manager: ClientManagerEntry,
        network: Network,
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

        let mut sizes = files.sizes.clone();
        if action == Action::Upload
            && config.common_data.index < sizes.len() as u32
            && sizes[config.common_data.index as usize] > 0
            && config.common_data.begins < sizes[config.common_data.index as usize] as u64 - 1
            && config.common_data.ends >= 0
            && config.common_data.begins <= config.common_data.ends as u64
        {
            let ends = config
                .common_data
                .ends
                .min(sizes[config.common_data.index as usize] - 1);
            sizes[config.common_data.index as usize] = ends - config.common_data.begins as i64 + 1;
        }

        let time = get_current_timestamp();
        let status = TaskStatus::new(time);
        let progress = Progress::new(sizes);

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
            code: Mutex::new(vec![Reason::Default; file_len]),
            background_notify_time: AtomicU64::new(time),
            file_total_size: AtomicI64::new(file_total_size),
            rate_limiting: AtomicU64::new(0),
            last_notify: AtomicU64::new(time),
            client_manager,
            running_result: Mutex::new(None),
            network,
            timeout_tries: AtomicU32::new(0),
        }
    }

    pub(crate) fn new_by_info(
        config: TaskConfig,
        #[cfg(feature = "oh")] system: SystemConfig,
        info: TaskInfo,
        client_manager: ClientManagerEntry,
        network: Network,
    ) -> Result<RequestTask, ErrorCode> {
        #[cfg(feature = "oh")]
        let (files, client) = check_config(&config, system)?;
        #[cfg(not(feature = "oh"))]
        let (files, client) = check_config(&config)?;

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
        let status = TaskStatus {
            mtime: time,
            state: State::from(info.progress.common_data.state),
            reason: Reason::from(info.common_data.reason),
        };
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
            code: Mutex::new(vec![Reason::Default; file_len]),
            background_notify_time: AtomicU64::new(time),
            file_total_size: AtomicI64::new(file_total_size),
            rate_limiting: AtomicU64::new(0),
            last_notify: AtomicU64::new(time),
            client_manager,
            running_result: Mutex::new(None),
            network,
            timeout_tries: AtomicU32::new(0),
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
        }
    }

    pub(crate) fn update_progress_in_database(&self) {
        let mtime = self.status.lock().unwrap().mtime;
        let reason = self.status.lock().unwrap().reason;
        let progress = self.progress.lock().unwrap().clone();
        let update_info = UpdateInfo {
            mtime,
            reason: reason.repr,
            progress,
            each_file_status: RequestTask::get_each_file_status_by_code(
                &self.code.lock().unwrap(),
                &self.conf.file_specs,
            ),
            tries: self.tries.load(Ordering::SeqCst),
            mime_type: self.mime_type(),
        };
        RequestDb::get_instance().update_task(self.task_id(), update_info);
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

    pub(crate) async fn clear_downloaded_file(&self) -> Result<(), std::io::Error> {
        info!("task {} clear downloaded file", self.task_id());
        let file = self.files.get_mut(0).unwrap();
        file.set_len(0).await?;
        file.seek(SeekFrom::Start(0)).await?;

        let mut progress_guard = self.progress.lock().unwrap();
        progress_guard.common_data.total_processed = 0;
        progress_guard.processed[0] = 0;

        Ok(())
    }

    pub(crate) async fn build_download_request(&self) -> Result<Request, TaskError> {
        let mut request_builder = self.build_request_builder()?;

        let file = self.files.get_mut(0).unwrap();

        let has_downloaded = file.metadata().await?.len();
        let resume_download = has_downloaded > 0;
        let require_range = self.require_range();

        let begins = self.conf.common_data.begins;
        let ends = self.conf.common_data.ends;

        info!(
            "task {} build download request, resume_download: {}, require_range: {}",
            self.task_id(),
            resume_download,
            require_range
        );
        match (resume_download, require_range) {
            (true, false) => {
                let (builder, support_range) = self.support_range(request_builder);
                request_builder = builder;
                if support_range {
                    request_builder =
                        self.range_request(request_builder, begins + has_downloaded, ends);
                } else {
                    self.clear_downloaded_file().await?;
                }
            }
            (false, true) => {
                request_builder = self.range_request(request_builder, begins, ends);
            }
            (true, true) => {
                let (builder, support_range) = self.support_range(request_builder);
                request_builder = builder;
                if support_range {
                    request_builder =
                        self.range_request(request_builder, begins + has_downloaded, ends);
                } else {
                    return Err(TaskError::Failed(Reason::UnsupportedRangeRequest));
                }
            }
            (false, false) => {}
        };

        let request = request_builder.body(Body::slice(self.conf.data.clone()))?;
        Ok(request)
    }

    fn range_request(
        &self,
        request_builder: RequestBuilder,
        begins: u64,
        ends: i64,
    ) -> RequestBuilder {
        let range = if ends < 0 {
            format!("bytes={begins}-")
        } else {
            format!("bytes={begins}-{ends}")
        };
        request_builder.header("Range", range.as_str())
    }

    fn support_range(&self, mut request_builder: RequestBuilder) -> (RequestBuilder, bool) {
        let progress_guard = self.progress.lock().unwrap();
        let mut support_range = false;
        if let Some(etag) = progress_guard.extras.get("etag") {
            request_builder = request_builder.header("If-Range", etag.as_str());
            support_range = true;
        } else if let Some(last_modified) = progress_guard.extras.get("last-modified") {
            request_builder = request_builder.header("If-Range", last_modified.as_str());
            support_range = true;
        }
        if !support_range {
            info!("task {} does not support range request", self.task_id());
        }
        (request_builder, support_range)
    }

    pub(crate) fn get_file_info(&self, response: &Response) -> Result<(), TaskError> {
        let content_type = response.headers().get("content-type");
        if let Some(mime_type) = content_type {
            if let Ok(value) = mime_type.to_string() {
                *self.mime_type.lock().unwrap() = value;
            }
        }

        let content_length = response.headers().get("content-length");
        if let Some(Ok(len)) = content_length.map(|v| v.to_string()) {
            match len.parse::<i64>() {
                Ok(v) => {
                    let mut progress = self.progress.lock().unwrap();
                    progress.sizes = vec![v + progress.processed[0] as i64];
                    self.file_total_size.store(v, Ordering::SeqCst);
                    debug!("the download task content-length is {}", v);
                }
                Err(e) => {
                    error!("convert string to i64 error: {:?}", e);
                }
            }
        } else {
            error!("cannot get content-length of the task");
            if self.conf.common_data.precise {
                return Err(TaskError::Failed(Reason::GetFileSizeFailed));
            }
        }
        Ok(())
    }

    pub(crate) async fn handle_download_error(
        &self,
        err: HttpClientError,
    ) -> Result<(), TaskError> {
        error!("download err is {:?}", err);
        match err.error_kind() {
            ErrorKind::Timeout => Err(TaskError::Failed(Reason::ContinuousTaskTimeout)),
            // user triggered
            ErrorKind::UserAborted => Err(TaskError::Waiting(TaskPhase::UserAbort)),
            ErrorKind::BodyTransfer | ErrorKind::BodyDecode => {
                self.network_retry().await?;
                Err(TaskError::Failed(Reason::OthersError))
            }
            _ => {
                if format!("{}", err).contains("No space left on device") {
                    Err(TaskError::Failed(Reason::InsufficientSpace))
                } else {
                    Err(TaskError::Failed(Reason::OthersError))
                }
            }
        }
    }

    #[cfg(feature = "oh")]
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

    pub(crate) fn require_range(&self) -> bool {
        self.conf.common_data.begins > 0 || self.conf.common_data.ends >= 0
    }

    pub(crate) async fn record_upload_response(
        &self,
        index: usize,
        response: Result<Response, HttpClientError>,
    ) {
        if let Ok(mut r) = response {
            {
                let mut guard = self.progress.lock().unwrap();
                guard.extras.clear();
                for (k, v) in r.headers() {
                    if let Ok(value) = v.to_string() {
                        guard.extras.insert(k.to_string().to_lowercase(), value);
                    }
                }
            }

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
                retry: self.conf.common_data.retry,
                tries: self.tries.load(Ordering::SeqCst),
                version: self.conf.version as u8,
                priority: self.conf.common_data.priority,
            },
        }
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

        #[cfg(feature = "oh")]
        {
            let percent = total_processed * 100 / (file_total_size as u64);
            let task_msg = RequestTaskMsg {
                task_id: self.conf.common_data.task_id,
                uid: self.conf.common_data.uid as i32,
                action: self.conf.common_data.action.repr,
            };

            let path = self.conf.file_specs[index].path.as_str();
            let file_name = self.conf.file_specs[index].file_name.as_str();
            let _ = request_background_notify(task_msg, path, file_name, percent as u32);
        }
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
    pub(crate) mtime: u64,
    pub(crate) state: State,
    pub(crate) reason: Reason,
}

impl TaskStatus {
    pub(crate) fn new(mtime: u64) -> Self {
        TaskStatus {
            mtime,
            state: State::Initialized,
            reason: Reason::Default,
        }
    }
}

fn check_file_specs(file_specs: &[FileSpec]) -> bool {
    const EL1: &str = "/data/storage/el1/base/";
    const EL2: &str = "/data/storage/el2/base/";
    const EL5: &str = "/data/storage/el5/base/";

    let mut result = true;
    for (idx, spec) in file_specs.iter().enumerate() {
        let path = &spec.path;
        if !spec.is_user_file && !path.starts_with(EL1) && !path.starts_with(EL2) && !path.starts_with(EL5) {
            error!("File path invalid - path: {}, idx: {}", path, idx);
            result = false;
            break;
        }
    }

    result
}

pub(crate) fn check_config(
    config: &TaskConfig,
    #[cfg(feature = "oh")] system: SystemConfig,
) -> Result<(AttachedFiles, Client), ErrorCode> {
    if !check_file_specs(&config.file_specs) {
        return Err(ErrorCode::Other);
    }
    let files = AttachedFiles::open(config).map_err(|_| ErrorCode::FileOperationErr)?;
    #[cfg(feature = "oh")]
    let client = build_client(config, system).map_err(|_| ErrorCode::Other)?;

    #[cfg(not(feature = "oh"))]
    let client = build_client(config).map_err(|_| ErrorCode::Other)?;
    Ok((files, client))
}

impl From<HttpClientError> for TaskError {
    fn from(_value: HttpClientError) -> Self {
        TaskError::Failed(Reason::BuildRequestFailed)
    }
}

impl From<io::Error> for TaskError {
    fn from(_value: io::Error) -> Self {
        TaskError::Failed(Reason::IoError)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TaskPhase {
    NeedRetry,
    UserAbort,
    NetworkOffline,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TaskError {
    Failed(Reason),
    Waiting(TaskPhase),
}
