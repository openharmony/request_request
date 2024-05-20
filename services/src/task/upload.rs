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
use std::pin::Pin;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::task::{Context, Poll};

use ylong_http_client::async_impl::{Body, MultiPart, Part, Request, UploadOperator, Uploader};
use ylong_http_client::HttpClientError;
use ylong_runtime::io::{AsyncRead, AsyncSeek, ReadBuf};

use super::operator::TaskOperator;
use super::reason::Reason;
use crate::task::info::State;
use crate::task::request_task::RequestTask;
use crate::trace::Trace;

struct TaskReader {
    pub(crate) task: Arc<RequestTask>,
}

impl TaskReader {
    pub(crate) fn new(task: Arc<RequestTask>) -> Self {
        Self { task }
    }
}

impl AsyncRead for TaskReader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let index = self.task.progress.lock().unwrap().common_data.index;
        let file = self.task.files.get_mut(index).unwrap();
        let (is_partial_upload, total_upload_bytes) = self.task.get_upload_info(index);
        let mut progress_guard = self.task.progress.lock().unwrap();
        if !is_partial_upload {
            let filled_len = buf.filled().len();
            match Pin::new(file).poll_read(cx, buf) {
                Poll::Ready(Ok(_)) => {
                    let current_filled_len = buf.filled().len();
                    let upload_size = current_filled_len - filled_len;
                    progress_guard.processed[index] += upload_size;
                    progress_guard.common_data.total_processed += upload_size;
                    Poll::Ready(Ok(()))
                }
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            }
        } else {
            let begins = self.task.conf.common_data.begins;
            if !self.task.seek_flag.load(Ordering::SeqCst) {
                match Pin::new(file).poll_seek(cx, SeekFrom::Start(begins)) {
                    Poll::Ready(Err(e)) => {
                        error!("seek err is {:?}", e);
                        return Poll::Ready(Err(e));
                    }
                    _ => self.task.seek_flag.store(true, Ordering::SeqCst),
                }
            }
            let buf_filled_len = buf.filled().len();
            let mut read_buf = buf.take(total_upload_bytes as usize);
            let filled_len = read_buf.filled().len();
            let file = self.task.files.get_mut(index).unwrap();
            match Pin::new(file).poll_read(cx, &mut read_buf) {
                Poll::Ready(Ok(_)) => {
                    let current_filled_len = read_buf.filled().len();
                    let upload_size = current_filled_len - filled_len;
                    // need update buf.filled and buf.initialized
                    buf.assume_init(upload_size);
                    buf.set_filled(buf_filled_len + upload_size);
                    progress_guard.processed[index] += upload_size;
                    progress_guard.common_data.total_processed += upload_size;
                    Poll::Ready(Ok(()))
                }
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            }
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

fn build_stream_request(task: Arc<RequestTask>, index: usize) -> Option<Request> {
    debug!("build stream request");
    let task_reader = TaskReader::new(task.clone());
    let task_operator = TaskOperator::new(task.clone());

    match task.build_request_builder() {
        Ok(mut request_builder) => {
            if task.conf.headers.get("Content-Type").is_none() {
                request_builder =
                    request_builder.header("Content-Type", "application/octet-stream");
            }
            let (_, upload_length) = task.get_upload_info(index);
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

fn build_multipart_request(task: Arc<RequestTask>, index: usize) -> Option<Request> {
    info!("build multipart request");
    let task_reader = TaskReader::new(task.clone());
    let task_operator = TaskOperator::new(task.clone());
    let mut multi_part = MultiPart::new();
    for item in task.conf.form_items.iter() {
        let part = Part::new()
            .name(item.name.as_str())
            .body(item.value.as_str());
        multi_part = multi_part.part(part);
    }
    let (_, upload_length) = task.get_upload_info(index);
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

fn build_request_common(
    task: &Arc<RequestTask>,
    index: usize,
    request: Result<Request, HttpClientError>,
) -> Option<Request> {
    match request {
        Ok(value) => {
            info!(
                "build upload request success, task_id is {}",
                task.conf.common_data.task_id
            );
            Some(value)
        }
        Err(e) => {
            error!("build upload request error is {:?}", e);
            {
                // `unwrap` for propagating panics among threads.
                let mut codes_guard = task.code.lock().unwrap();
                for (i, code) in codes_guard.iter_mut().enumerate() {
                    if i >= index {
                        *code = Reason::BuildRequestFailed;
                    }
                }
            }
            task.set_status(State::Failed, Reason::BuildRequestFailed);
            None
        }
    }
}

pub(crate) async fn upload(task: Arc<RequestTask>) {
    info!(
        "begin upload task, task_id is {}",
        task.conf.common_data.task_id
    );

    let url = task.conf.url.as_str();
    let num = task.conf.file_specs.len();
    // Ensures `_trace` can only be freed when this function exits.

    let _trace = Trace::new(&format!("exec upload task url: {url} file num: {num}"));

    let size = task.conf.file_specs.len();
    let index = task.progress.lock().unwrap().common_data.index;
    debug!("index is {}", index);
    for i in index..size {
        task.progress.lock().unwrap().common_data.index = i;
        let result: bool;
        match task.conf.headers.get("Content-Type") {
            None => {
                if task.conf.method.to_uppercase().eq("POST") {
                    result = upload_one_file(task.clone(), i, build_multipart_request).await;
                } else {
                    result = upload_one_file(task.clone(), i, build_stream_request).await;
                }
            }
            Some(v) => {
                if v == "multipart/form-data" {
                    result = upload_one_file(task.clone(), i, build_multipart_request).await;
                } else {
                    result = upload_one_file(task.clone(), i, build_stream_request).await;
                }
            }
        }
        if result {
            info!(
                "upload one file success, task_id is {}, index is {}",
                task.conf.common_data.task_id, i
            );
            task.upload_counts.fetch_add(1, Ordering::SeqCst);
        }
        let state = task.status.lock().unwrap().state;
        if state != State::Running && state != State::Retrying {
            return;
        }
    }

    let uploaded = task.upload_counts.load(Ordering::SeqCst);
    if uploaded == size {
        task.set_status(State::Completed, Reason::Default);
    } else {
        task.set_status(State::Failed, Reason::UploadFileError);

        use hisysevent::{build_number_param, build_str_param};

        use crate::sys_event::SysEvent;
        // Records sys event.

        SysEvent::task_fault()
            .param(build_str_param!(crate::sys_event::TASKS_TYPE, "UPLOAD"))
            .param(build_number_param!(crate::sys_event::TOTAL_FILE_NUM, size))
            .param(build_number_param!(
                crate::sys_event::FAIL_FILE_NUM,
                size - uploaded
            ))
            .param(build_number_param!(
                crate::sys_event::SUCCESS_FILE_NUM,
                uploaded
            ))
            .param(build_number_param!(
                crate::sys_event::ERROR_INFO,
                Reason::UploadFileError as i32
            ))
            .write();
    }

    info!("upload end, task_id is {}", task.conf.common_data.task_id);
}

async fn upload_one_file<F>(task: Arc<RequestTask>, index: usize, build_upload_request: F) -> bool
where
    F: Fn(Arc<RequestTask>, usize) -> Option<Request>,
{
    info!(
        "begin upload one file, task_id is {}, index is {}",
        task.conf.common_data.task_id, index
    );

    let (_, size) = task.get_upload_info(index);
    let name = task.conf.file_specs[index].file_name.as_str();

    // Ensures `_trace` can only be freed when this function exits.

    let _trace = Trace::new(&format!(
        "upload file name:{name} index:{index} size:{size}"
    ));

    loop {
        task.reset_code(index);
        let request = build_upload_request(task.clone(), index);
        if request.is_none() {
            return false;
        }
        let response = task.client.request(request.unwrap()).await;
        if task.handle_response_error(&response).await {
            // `unwrap` for propagating panics among threads.
            if let Some(code) = task.code.lock().unwrap().get_mut(index) {
                *code = Reason::Default;
            }
            task.record_upload_response(index, response).await;
            return true;
        }
        task.record_upload_response(index, response).await;
        // `unwrap` for propagating panics among threads.

        let state = task.status.lock().unwrap().state;
        if state != State::Running && state != State::Retrying {
            return false;
        }
        let code = *task
            .code
            .lock()
            .unwrap()
            .get(index)
            .unwrap_or(&Reason::OthersError);
        if code != Reason::Default {
            error!(
                "upload {} file fail, which reason is {}",
                index, code as u32
            );
            if code != Reason::UserOperation {
                task.set_status(State::Failed, code);
            }
            return false;
        }
    }
}
