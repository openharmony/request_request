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

use std::future::Future;
use std::io::{Read, SeekFrom};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use ylong_http_client::async_impl::{Body, MultiPart, Part, Request, UploadOperator, Uploader};
use ylong_http_client::{ErrorKind, HttpClientError, ReusableReader, Timeout};
use ylong_runtime::io::{AsyncRead, ReadBuf};

use super::info::State;
use super::operator::TaskOperator;
use super::reason::Reason;
use super::request_task::{TaskError, TaskPhase};
use super::task_control;
use crate::manage::database::RequestDb;
use crate::task::request_task::RequestTask;
#[cfg(feature = "oh")]
use crate::trace::Trace;

struct TaskReader {
    pub(crate) task: Arc<RequestTask>,
    pub(crate) index: usize,
    pub(crate) reused: Option<usize>,
}

impl TaskReader {
    pub(crate) fn new(task: Arc<RequestTask>, index: usize) -> Self {
        Self {
            task,
            index,
            reused: None,
        }
    }
}

impl AsyncRead for TaskReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let index = self.index;
        let file = self
            .task
            .files
            .get(index)
            .ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))?;

        // Obtain `file`` first and then `progress` to prevent deadlocks.
        let mut file = file.lock().unwrap();
        let mut progress_guard = self.task.progress.lock().unwrap();

        if self.task.conf.common_data.index == index as u32 || progress_guard.processed[index] != 0
        {
            let total_upload_bytes = if let Some(uploaded) = self.reused {
                progress_guard.sizes[index] as usize - uploaded
            } else {
                progress_guard.sizes[index] as usize - progress_guard.processed[index]
            };
            let buf_filled_len = buf.filled().len();
            let mut read_buf = buf.take(total_upload_bytes);
            match file.read(read_buf.initialize_unfilled()) {
                Ok(size) => {
                    let upload_size = read_buf.filled().len() + size;
                    read_buf.set_filled(upload_size);
                    // need update buf.filled and buf.initialized
                    buf.assume_init(upload_size);
                    buf.set_filled(buf_filled_len + upload_size);
                    match self.reused {
                        None => {
                            progress_guard.processed[index] += upload_size;
                            progress_guard.common_data.total_processed += upload_size;
                            progress_guard.common_data.index = index;
                        }
                        Some(uploaded) => {
                            drop(progress_guard);
                            self.reused = Some(uploaded + upload_size);
                        }
                    }
                    Poll::Ready(Ok(()))
                }
                Err(e) => Poll::Ready(Err(e)),
            }
        } else {
            match file.read(buf.initialize_unfilled()) {
                Ok(size) => {
                    let current_filled_len = buf.filled().len() + size;
                    buf.set_filled(current_filled_len);

                    progress_guard.processed[index] += size;
                    progress_guard.common_data.total_processed += size;
                    Poll::Ready(Ok(()))
                }
                Err(e) => Poll::Ready(Err(e)),
            }
        }
    }
}

impl ReusableReader for TaskReader {
    fn reuse<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = std::io::Result<()>> + Send + Sync + 'a>>
    where
        Self: 'a,
    {
        self.reused = Some(0);
        let index = self.index;
        let optional_file = self.task.files.get(index);
        if self.task.conf.common_data.index == index as u32 {
            let begins = self.task.conf.common_data.begins;
            Box::pin(async move {
                let file =
                    optional_file.ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))?;
                task_control::file_seek(file, SeekFrom::Start(begins))
                    .await
                    .map(|_| ())
            })
        } else {
            Box::pin(async {
                let file =
                    optional_file.ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))?;
                task_control::file_rewind(file).await.map(|_| ())
            })
        }
    }
}

impl UploadOperator for TaskOperator {
    fn poll_progress(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        _uploaded: u64,
        _total: Option<u64>,
    ) -> Poll<Result<(), HttpClientError>> {
        let mut this = self;
        this.poll_progress_common(cx)
    }
}

fn build_stream_request(
    task: Arc<RequestTask>,
    index: usize,
    abort_flag: Arc<AtomicBool>,
) -> Option<Request> {
    debug!("build stream request");
    let task_reader = TaskReader::new(task.clone(), index);
    let task_operator = TaskOperator::new(task.clone(), abort_flag);

    match task.build_request_builder() {
        Ok(mut request_builder) => {
            if !task.conf.headers.contains_key("Content-Type") {
                request_builder =
                    request_builder.header("Content-Type", "application/octet-stream");
            }
            let upload_length;
            {
                let progress = task.progress.lock().unwrap();
                upload_length = progress.sizes[index] as u64 - progress.processed[index] as u64;
            }
            debug!("upload length is {}", upload_length);
            request_builder =
                request_builder.header("Content-Length", upload_length.to_string().as_str());
            let uploader = Uploader::builder()
                .reader(task_reader)
                .operator(task_operator)
                .total_bytes(Some(upload_length))
                .build();
            let request = request_builder.body(Body::stream(uploader));
            build_request_common(&task, index, request)
        }
        Err(err) => build_request_common(&task, index, Err(err)),
    }
}

fn build_multipart_request(
    task: Arc<RequestTask>,
    index: usize,
    abort_flag: Arc<AtomicBool>,
) -> Option<Request> {
    debug!("build multipart request");
    let task_reader = TaskReader::new(task.clone(), index);
    let task_operator = TaskOperator::new(task.clone(), abort_flag);
    let mut multi_part = MultiPart::new();
    for item in task.conf.form_items.iter() {
        let part = Part::new()
            .name(item.name.as_str())
            .body(item.value.as_str());
        multi_part = multi_part.part(part);
    }
    let upload_length;
    {
        let progress = task.progress.lock().unwrap();
        upload_length = progress.sizes[index] as u64 - progress.processed[index] as u64;
    }
    debug!("upload length is {}", upload_length);
    let part = Part::new()
        .name(task.conf.file_specs[index].name.as_str())
        .file_name(task.conf.file_specs[index].file_name.as_str())
        .mime(task.conf.file_specs[index].mime_type.as_str())
        .length(Some(upload_length))
        .stream(task_reader);

    multi_part = multi_part.part(part);
    let uploader = Uploader::builder()
        .multipart(multi_part)
        .operator(task_operator)
        .build();

    match task.build_request_builder() {
        Ok(request_builder) => {
            let request: Result<Request, HttpClientError> =
                request_builder.body(Body::multipart(uploader));
            build_request_common(&task, index, request)
        }
        Err(err) => build_request_common(&task, index, Err(err)),
    }
}

fn build_batch_multipart_request(
    task: Arc<RequestTask>,
    _index: usize,
    abort_flag: Arc<AtomicBool>,
) -> Option<Request> {
    let mut multi_part = MultiPart::new();
    let task_operator = TaskOperator::new(task.clone(), abort_flag);
    let start = task.progress.lock().unwrap().common_data.index;
    info!("multi part upload task {}", task.task_id());

    for item in task.conf.form_items.iter() {
        let part = Part::new()
            .name(item.name.as_str())
            .body(item.value.as_str());

        multi_part = multi_part.part(part);
    }
    for index in start..task.conf.file_specs.len() {
        let task_reader = TaskReader::new(task.clone(), index);
        let upload_length = {
            let progress = task.progress.lock().unwrap();
            progress.sizes[index] as u64 - progress.processed[index] as u64
        };
        let part = Part::new()
            .name(task.conf.file_specs[index].name.as_str())
            .file_name(task.conf.file_specs[index].file_name.as_str())
            .mime(task.conf.file_specs[index].mime_type.as_str())
            .length(Some(upload_length))
            .stream(task_reader);

        multi_part = multi_part.part(part);
    }

    let uploader = Uploader::builder()
        .multipart(multi_part)
        .operator(task_operator)
        .build();

    match task.build_request_builder() {
        Ok(request_builder) => {
            let request: Result<Request, HttpClientError> =
                request_builder.body(Body::multipart(uploader));
            build_request_common(&task, 0, request)
        }
        Err(err) => build_request_common(&task, 0, Err(err)),
    }
}

fn build_request_common(
    task: &Arc<RequestTask>,
    _index: usize,
    request: Result<Request, HttpClientError>,
) -> Option<Request> {
    match request {
        Ok(value) => {
            debug!(
                "build upload request success, tid: {}",
                task.conf.common_data.task_id
            );
            Some(value)
        }
        Err(e) => {
            error!("build upload request error is {:?}", e);
            None
        }
    }
}

impl RequestTask {
    async fn prepare_single_upload(&self, index: usize) -> bool {
        let Some(file) = self.files.get(index) else {
            error!("task {} file {} not found", self.task_id(), index);
            return false;
        };
        {
            let mut progress = self.progress.lock().unwrap();
            if self.upload_resume.load(Ordering::SeqCst) {
                self.upload_resume.store(false, Ordering::SeqCst);
            } else {
                progress.processed[index] = 0;
            }
            progress.common_data.index = index;
            progress.common_data.total_processed = progress.processed.iter().take(index).sum();
        }

        let processed = self.progress.lock().unwrap().processed[index] as u64;
        if self.conf.common_data.index == index as u32 {
            let Ok(metadata) = task_control::file_metadata(file.clone()).await else {
                error!("get file metadata failed");
                return false;
            };
            if metadata.len() > self.progress.lock().unwrap().sizes[index] as u64 {
                task_control::file_seek(
                    file,
                    SeekFrom::Start(self.conf.common_data.begins + processed),
                )
                .await
            } else {
                task_control::file_seek(file.clone(), SeekFrom::Start(processed)).await
            }
        } else {
            task_control::file_seek(file, SeekFrom::Start(processed)).await
        }
        .is_ok()
    }

    async fn prepare_batch_upload(&self, start: usize, size: usize) -> bool {
        let mut current_index = 0;
        {
            let mut progress = self.progress.lock().unwrap();

            let total = progress.common_data.total_processed;
            let file_sizes = &progress.sizes;
            let mut current_size = 0;
            for (index, &file_size) in file_sizes.iter().enumerate() {
                current_size += file_size as usize;
                if total <= current_size {
                    current_index = index;
                    break;
                }
            }
            if self.upload_resume.load(Ordering::SeqCst) {
                self.upload_resume.store(false, Ordering::SeqCst);
            } else {
                progress.processed[current_index] = 0;
            }
            progress.common_data.index = current_index;
            progress.common_data.total_processed =
                progress.processed.iter().take(current_index).sum();
        }

        for index in start..size {
            let Some(file) = self.files.get(index) else {
                error!("task {} file {} not found", self.task_id(), index);
                return false;
            };
            let processed = self.progress.lock().unwrap().processed[index] as u64;
            let target_start = if self.conf.common_data.index == index as u32 {
                let Ok(metadata) = task_control::file_metadata(file.clone()).await else {
                    error!("get file metadata failed");
                    return false;
                };
                if metadata.len() > self.progress.lock().unwrap().sizes[index] as u64 {
                    self.conf.common_data.begins + processed
                } else {
                    processed
                }
            } else {
                processed
            };
            if let Err(e) = task_control::file_seek(file, SeekFrom::Start(target_start)).await {
                error!("file seek err:{:}", e);
                return false;
            }
        }
        true
    }
}

pub(crate) async fn upload(task: Arc<RequestTask>, abort_flag: Arc<AtomicBool>) {
    RequestDb::get_instance()
        .update_task_sizes(task.task_id(), &task.progress.lock().unwrap().sizes);
    task.progress.lock().unwrap().common_data.state = State::Running.repr;
    task.tries.store(0, Ordering::SeqCst);
    loop {
        if let Err(e) = upload_inner(task.clone(), abort_flag.clone()).await {
            match e {
                TaskError::Failed(reason) => {
                    *task.running_result.lock().unwrap() = Some(Err(reason));
                }
                TaskError::Waiting(phase) => match phase {
                    TaskPhase::NeedRetry => {
                        continue;
                    }
                    TaskPhase::UserAbort => {}
                    TaskPhase::NetworkOffline => {
                        *task.running_result.lock().unwrap() = Some(Err(Reason::NetworkOffline));
                    }
                },
            }
        } else {
            *task.running_result.lock().unwrap() = Some(Ok(()));
        }
        break;
    }
}

async fn upload_inner(
    task: Arc<RequestTask>,
    abort_flag: Arc<AtomicBool>,
) -> Result<(), TaskError> {
    info!("upload task {} running", task.task_id());

    #[cfg(feature = "oh")]
    let _trace = Trace::new(&format!(
        "exec upload task:{} file num:{}",
        task.task_id(),
        task.conf.file_specs.len()
    ));

    let size = task.conf.file_specs.len();
    let start = task.progress.lock().unwrap().common_data.index;

    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64;
    task.start_time.store(start_time as u64, Ordering::SeqCst);

    if task.conf.common_data.multipart {
        #[cfg(feature = "oh")]
        let _trace = Trace::new(&format!("upload file:{} index:{}", task.task_id(), start));

        if !task.prepare_batch_upload(start, size).await {
            return Err(TaskError::Failed(Reason::OthersError));
        }

        upload_one_file(
            task.clone(),
            start,
            abort_flag.clone(),
            build_batch_multipart_request,
        )
        .await?
    } else {
        let is_multipart = match task.conf.headers.get("Content-Type") {
            Some(s) => s.eq("multipart/form-data"),
            None => task.conf.method.to_uppercase().eq("POST"),
        };
        for index in start..size {
            #[cfg(feature = "oh")]
            let _trace = Trace::new(&format!("upload file:{} index:{}", task.task_id(), index));

            if !task.prepare_single_upload(index).await {
                return Err(TaskError::Failed(Reason::OthersError));
            }

            let func = match is_multipart {
                true => build_multipart_request,
                false => build_stream_request,
            };
            upload_one_file(task.clone(), index, abort_flag.clone(), func).await?;
            task.notify_header_receive();
        }
    }

    info!("task {} upload ok", task.task_id());
    Ok(())
}

async fn upload_one_file<F>(
    task: Arc<RequestTask>,
    index: usize,
    abort_flag: Arc<AtomicBool>,
    build_upload_request: F,
) -> Result<(), TaskError>
where
    F: Fn(Arc<RequestTask>, usize, Arc<AtomicBool>) -> Option<Request>,
{
    let begin_time = Instant::now();
    let result = upload_one_file_inner(
        task.clone(),
        index,
        abort_flag.clone(),
        build_upload_request,
    )
    .await;
    let upload_time = begin_time.elapsed().as_secs();
    task.rest_time.fetch_sub(upload_time, Ordering::SeqCst);
    let mut client = task.client.lock().await;
    client.total_timeout(Timeout::from_secs(task.rest_time.load(Ordering::SeqCst)));
    result
}

async fn upload_one_file_inner<F>(
    task: Arc<RequestTask>,
    index: usize,
    abort_flag: Arc<AtomicBool>,
    build_upload_request: F,
) -> Result<(), TaskError>
where
    F: Fn(Arc<RequestTask>, usize, Arc<AtomicBool>) -> Option<Request>,
{
    info!(
        "begin 1 upload tid {} index {} sizes {}",
        task.conf.common_data.task_id,
        index,
        task.progress.lock().unwrap().sizes[index]
    );

    let Some(request) = build_upload_request(task.clone(), index, abort_flag) else {
        return Err(TaskError::Failed(Reason::BuildRequestFailed));
    };

    let client = task.client.lock().await;
    let response = client.request(request).await;
    match response.as_ref() {
        Ok(response) => {
            let status_code = response.status();
            task.status_code
                .store(status_code.as_u16() as i32, Ordering::SeqCst);
            #[cfg(feature = "oh")]
            task.notify_response(response);
            info!(
                "task {} get response {}",
                task.conf.common_data.task_id, status_code,
            );
            if status_code.is_server_error()
                || (status_code.as_u16() != 408 && status_code.is_client_error())
                || status_code.is_redirection()
            {
                return Err(TaskError::Failed(Reason::ProtocolError));
            }
            if status_code.as_u16() == 408 {
                if task.timeout_tries.load(Ordering::SeqCst) < 2 {
                    task.timeout_tries.fetch_add(1, Ordering::SeqCst);
                    return Err(TaskError::Waiting(TaskPhase::NeedRetry));
                } else {
                    return Err(TaskError::Failed(Reason::ProtocolError));
                }
            } else {
                task.timeout_tries.store(0, Ordering::SeqCst);
            }
        }
        Err(e) => {
            if e.error_kind() != ErrorKind::UserAborted {
                error!("Task {} {:?}", task.task_id(), e);
            }

            match e.error_kind() {
                ErrorKind::Timeout => return Err(TaskError::Failed(Reason::ContinuousTaskTimeout)),
                ErrorKind::Request => return Err(TaskError::Failed(Reason::RequestError)),
                ErrorKind::Redirect => return Err(TaskError::Failed(Reason::RedirectError)),
                ErrorKind::Connect | ErrorKind::ConnectionUpgrade => {
                    task.network_retry().await?;
                    if e.is_dns_error() {
                        return Err(TaskError::Failed(Reason::Dns));
                    } else if e.is_tls_error() {
                        return Err(TaskError::Failed(Reason::Ssl));
                    } else {
                        return Err(TaskError::Failed(Reason::Tcp));
                    }
                }
                ErrorKind::BodyTransfer => {
                    if format!("{}", e).contains("Below low speed limit") {
                        return Err(TaskError::Failed(Reason::LowSpeed));
                    } else {
                        task.network_retry().await?;
                        return Err(TaskError::Failed(Reason::OthersError));
                    }
                }
                ErrorKind::UserAborted => return Err(TaskError::Waiting(TaskPhase::UserAbort)),
                _ => {
                    if format!("{}", e).contains("No space left on device") {
                        return Err(TaskError::Failed(Reason::InsufficientSpace));
                    } else {
                        return Err(TaskError::Failed(Reason::OthersError));
                    }
                }
            };
        }
    };
    task.record_upload_response(index, response).await;
    Ok(())
}

#[cfg(test)]
mod ut_upload {
    include!("../../tests/ut/task/ut_upload.rs");
}
