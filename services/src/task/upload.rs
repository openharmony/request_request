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

//! Upload functionality for HTTP request tasks.
//! 
//! This module provides the implementation for file upload operations, including stream uploads,
//! multipart form data uploads, and batch uploads. It handles file reading, progress tracking,
//! request construction, and error handling for upload tasks.

use std::future::Future;
use std::io::{Read, SeekFrom};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Instant;

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
use crate::utils::get_current_duration;

/// A reader that reads data from a task's file for upload operations.
/// 
/// Implements `AsyncRead` and `ReusableReader` traits to provide streaming data
/// from files associated with a request task.
struct TaskReader {
    /// The request task containing the file to read.
    pub(crate) task: Arc<RequestTask>,
    /// The index of the file to read from the task's files collection.
    pub(crate) index: usize,
    /// Tracks bytes read during reuse operations.
    pub(crate) reused: Option<usize>,
}

impl TaskReader {
    /// Creates a new `TaskReader` for the specified task and file index.
    /// 
    /// # Arguments
    /// 
    /// * `task` - The request task containing the file to read.
    /// * `index` - The index of the file to read from the task's files collection.
    pub(crate) fn new(task: Arc<RequestTask>, index: usize) -> Self {
        Self {
            task,
            index,
            reused: None,
        }
    }
}

impl AsyncRead for TaskReader {
    /// Attempts to read data from the task's file into the provided buffer.
    /// 
    /// Handles progress tracking and resume operations for upload tasks.
    /// 
    /// # Arguments
    /// 
    /// * `cx` - The task context (unused in this implementation).
    /// * `buf` - The buffer to read data into.
    /// 
    /// # Returns
    /// 
    /// A `Poll` indicating whether the read is ready or pending.
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
        // This lock ordering is critical to avoid deadlocks when multiple operations access
        // the same task's resources concurrently.
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
    /// Prepares the reader for reuse in a new request.
    /// 
    /// Resets the file position to the appropriate starting point based on
    /// the task's configuration and index.
    /// 
    /// # Returns
    /// 
    /// A future that resolves when the reader is ready for reuse.
    fn reuse<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = std::io::Result<()>> + Send + Sync + 'a>>
    where
        Self: 'a,
    {
        self.reused = Some(0);
        let index = self.index;
        let optional_file = self.task.files.get(index);
        
        // Determine the appropriate file position based on task configuration
        if self.task.conf.common_data.index == index as u32 {
            let begins = self.task.conf.common_data.begins;
            Box::pin(async move {
                let file = optional_file.ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))?;
                task_control::file_seek(file, SeekFrom::Start(begins))
                    .await
                    .map(|_| ())
            })
        } else {
            Box::pin(async {
                let file = optional_file.ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))?;
                task_control::file_rewind(file).await.map(|_| ())
            })
        }
    }
}

impl UploadOperator for TaskOperator {
    /// Polls for progress updates during upload operations.
    /// 
    /// Delegates to the common progress polling implementation.
    /// 
    /// # Arguments
    /// 
    /// * `cx` - The task context.
    /// * `_uploaded` - The number of bytes uploaded (unused).
    /// * `_total` - The total number of bytes to upload (unused).
    /// 
    /// # Returns
    /// 
    /// A `Poll` indicating whether progress reporting is ready or pending.
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

/// Builds a streaming upload request for a single file.
/// 
/// Constructs an HTTP request with a streaming body for file uploads.
/// 
/// # Arguments
/// 
/// * `task` - The request task containing the file to upload.
/// * `index` - The index of the file to upload.
/// * `abort_flag` - Atomic flag to signal upload cancellation.
/// 
/// # Returns
/// 
/// A `Request` if successful, or `None` if construction fails.
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
            // Set default content type if not specified
            if !task.conf.headers.contains_key("Content-Type") {
                request_builder =
                    request_builder.header("Content-Type", "application/octet-stream");
            }
            
            // Calculate the remaining upload length
            let upload_length;
            {
                let progress = task.progress.lock().unwrap();
                upload_length = progress.sizes[index] as u64 - progress.processed[index] as u64;
            }
            debug!("upload length is {}", upload_length);
            
            // Set content length header
            request_builder =
                request_builder.header("Content-Length", upload_length.to_string().as_str());
            
            // Build the uploader with streaming body
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

/// Builds a multipart form-data upload request for a single file.
/// 
/// Constructs an HTTP request with multipart form data for file uploads,
/// including both form fields and file data.
/// 
/// # Arguments
/// 
/// * `task` - The request task containing the file to upload.
/// * `index` - The index of the file to upload.
/// * `abort_flag` - Atomic flag to signal upload cancellation.
/// 
/// # Returns
/// 
/// A `Request` if successful, or `None` if construction fails.
fn build_multipart_request(
    task: Arc<RequestTask>,
    index: usize,
    abort_flag: Arc<AtomicBool>,
) -> Option<Request> {
    debug!("build multipart request");
    let task_reader = TaskReader::new(task.clone(), index);
    let task_operator = TaskOperator::new(task.clone(), abort_flag);
    
    // Create multipart form data
    let mut multi_part = MultiPart::new();
    
    // Add form fields
    for item in task.conf.form_items.iter() {
        let part = Part::new()
            .name(item.name.as_str())
            .body(item.value.as_str());
        multi_part = multi_part.part(part);
    }
    
    // Calculate upload length for the file
    let upload_length;
    {
        let progress = task.progress.lock().unwrap();
        upload_length = progress.sizes[index] as u64 - progress.processed[index] as u64;
    }
    debug!("upload length is {}", upload_length);
    
    // Add file part
    let part = Part::new()
        .name(task.conf.file_specs[index].name.as_str())
        .file_name(task.conf.file_specs[index].file_name.as_str())
        .mime(task.conf.file_specs[index].mime_type.as_str())
        .length(Some(upload_length))
        .stream(task_reader);

    multi_part = multi_part.part(part);
    
    // Build the multipart uploader
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

/// Builds a multipart form-data upload request for multiple files in a batch.
/// 
/// Constructs an HTTP request with multipart form data containing multiple files
/// for batch upload operations.
/// 
/// # Arguments
/// 
/// * `task` - The request task containing the files to upload.
/// * `_index` - Unused index parameter (batch uploads start from the progress index).
/// * `abort_flag` - Atomic flag to signal upload cancellation.
/// 
/// # Returns
/// 
/// A `Request` if successful, or `None` if construction fails.
fn build_batch_multipart_request(
    task: Arc<RequestTask>,
    _index: usize,
    abort_flag: Arc<AtomicBool>,
) -> Option<Request> {
    // Create multipart form data
    let mut multi_part = MultiPart::new();
    let task_operator = TaskOperator::new(task.clone(), abort_flag);
    let start = task.progress.lock().unwrap().common_data.index;
    info!("multi part upload task {}", task.task_id());

    // Add form fields
    for item in task.conf.form_items.iter() {
        let part = Part::new()
            .name(item.name.as_str())
            .body(item.value.as_str());

        multi_part = multi_part.part(part);
    }
    
    // Add all files from the current progress index
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

    // Build the multipart uploader
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

/// Common request construction handler.
/// 
/// Handles the result of request construction, logging success or error.
/// 
/// # Arguments
/// 
/// * `task` - The request task associated with the request.
/// * `_index` - Unused index parameter.
/// * `request` - The result of request construction.
/// 
/// # Returns
/// 
/// A `Request` if successful, or `None` if construction fails.
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
    /// Prepares a single file for upload.
    /// 
    /// Resets progress tracking if not resuming, sets the current file index,
    /// and positions the file cursor for upload operations.
    /// 
    /// # Arguments
    /// 
    /// * `index` - The index of the file to prepare.
    /// 
    /// # Returns
    /// 
    /// `true` if preparation succeeded, `false` otherwise.
    async fn prepare_single_upload(&self, index: usize) -> bool {
        let Some(file) = self.files.get(index) else {
            error!("task {} file {} not found", self.task_id(), index);
            return false;
        };
        
        // Initialize or reset progress tracking
        {
            let mut progress = self.progress.lock().unwrap();
            if self.upload_resume.load(Ordering::SeqCst) {
                // Reset the resume flag without resetting progress
                self.upload_resume.store(false, Ordering::SeqCst);
            } else {
                // Start fresh upload for this file
                progress.processed[index] = 0;
            }
            progress.common_data.index = index;
            progress.common_data.total_processed = progress.processed.iter().take(index).sum();
        }

        let processed = self.progress.lock().unwrap().processed[index] as u64;
        
        // Position the file cursor appropriately
        if self.conf.common_data.index == index as u32 {
            // Special handling for the current indexed file
            let Ok(metadata) = task_control::file_metadata(file.clone()).await else {
                error!("get file metadata failed");
                return false;
            };
            if metadata.len() > self.progress.lock().unwrap().sizes[index] as u64 {
                // File is larger than expected, start from configured beginning
                task_control::file_seek(
                    file,
                    SeekFrom::Start(self.conf.common_data.begins + processed),
                )
                .await
            } else {
                // Start from processed position
                task_control::file_seek(file.clone(), SeekFrom::Start(processed)).await
            }
        } else {
            // Standard file seek to processed position
            task_control::file_seek(file, SeekFrom::Start(processed)).await
        }
        .is_ok()
    }

    /// Prepares multiple files for batch upload.
    /// 
    /// Determines the current file index based on total processed bytes,
    /// resets progress tracking if not resuming, and positions file cursors
    /// for all files in the batch.
    /// 
    /// # Arguments
    /// 
    /// * `start` - The starting index for preparation.
    /// * `size` - The number of files to prepare.
    /// 
    /// # Returns
    /// 
    /// `true` if preparation succeeded for all files, `false` otherwise.
    async fn prepare_batch_upload(&self, start: usize, size: usize) -> bool {
        let mut current_index = 0;
        
        // Determine current position and reset progress if needed
        {
            let mut progress = self.progress.lock().unwrap();

            let total = progress.common_data.total_processed;
            let file_sizes = &progress.sizes;
            let mut current_size = 0;
            
            // Find the file that contains the current progress position
            for (index, &file_size) in file_sizes.iter().enumerate() {
                current_size += file_size as usize;
                if total <= current_size {
                    current_index = index;
                    break;
                }
            }
            
            // Handle resume or reset progress
            if self.upload_resume.load(Ordering::SeqCst) {
                self.upload_resume.store(false, Ordering::SeqCst);
            } else {
                progress.processed[current_index] = 0;
            }
            progress.common_data.index = current_index;
            progress.common_data.total_processed = progress.processed.iter().take(current_index).sum();
        }

        // Prepare each file in the batch
        for index in start..size {
            let Some(file) = self.files.get(index) else {
                error!("task {} file {} not found", self.task_id(), index);
                return false;
            };
            let processed = self.progress.lock().unwrap().processed[index] as u64;
            
            // Calculate target seek position
            let target_start = if self.conf.common_data.index == index as u32 {
                let Ok(metadata) = task_control::file_metadata(file.clone()).await else {
                    error!("get file metadata failed");
                    return false;
                };
                if metadata.len() > self.progress.lock().unwrap().sizes[index] as u64 {
                    // File size mismatch, use configured beginning
                    self.conf.common_data.begins + processed
                } else {
                    processed
                }
            } else {
                processed
            };
            
            // Position file cursor
            if let Err(e) = task_control::file_seek(file, SeekFrom::Start(target_start)).await {
                error!("file seek err:{:}", e);
                return false;
            }
        }
        true
    }
}

/// Main upload entry point for request tasks.
/// 
/// Initializes the task state, executes the upload operation with retry logic,
/// and handles various error conditions.
/// 
/// # Arguments
/// 
/// * `task` - The request task to upload.
/// * `abort_flag` - Atomic flag to signal upload cancellation.
pub(crate) async fn upload(task: Arc<RequestTask>, abort_flag: Arc<AtomicBool>) {
    // Update task sizes in the database
    RequestDb::get_instance()
        .update_task_sizes(task.task_id(), &task.progress.lock().unwrap().sizes);
    
    // Set task state to running
    task.progress.lock().unwrap().common_data.state = State::Running.repr;
    task.tries.store(0, Ordering::SeqCst);
    
    // Main upload loop with retry logic
    loop {
        if let Err(e) = upload_inner(task.clone(), abort_flag.clone()).await {
            match e {
                TaskError::Failed(reason) => {
                    // Task failed with specific reason
                    *task.running_result.lock().unwrap() = Some(Err(reason));
                }
                TaskError::Waiting(phase) => match phase {
                    TaskPhase::NeedRetry => {
                        // Retry the upload
                        continue;
                    }
                    TaskPhase::UserAbort => {
                        // User requested abort, end without setting error
                    }
                    TaskPhase::NetworkOffline => {
                        // Network offline error
                        *task.running_result.lock().unwrap() = Some(Err(Reason::NetworkOffline));
                    }
                },
            }
        } else {
            // Upload succeeded
            *task.running_result.lock().unwrap() = Some(Ok(()));
        }
        break;
    }
}

/// Internal upload implementation that handles different upload modes.
/// 
/// Processes the upload based on task configuration, handling both single file
/// and batch upload operations with appropriate request types.
/// 
/// # Arguments
/// 
/// * `task` - The request task to upload.
/// * `abort_flag` - Atomic flag to signal upload cancellation.
/// 
/// # Returns
/// 
/// `Ok(())` if upload succeeds, or a `TaskError` if it fails.
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

    // Record start time
    let start_time = get_current_duration().as_secs() as u64;
    task.start_time.store(start_time as u64, Ordering::SeqCst);

    // Handle different upload modes
    if task.conf.common_data.multipart {
        // Batch multipart upload mode
        #[cfg(feature = "oh")]
        let _trace = Trace::new(&format!("upload file:{} index:{}", task.task_id(), start));

        // Prepare all files for batch upload
        if !task.prepare_batch_upload(start, size).await {
            return Err(TaskError::Failed(Reason::OthersError));
        }

        // Upload all files in a single multipart request
        upload_one_file(
            task.clone(),
            start,
            abort_flag.clone(),
            build_batch_multipart_request,
        )
        .await?
    } else {
        // Determine if multipart encoding is needed
        let is_multipart = match task.conf.headers.get("Content-Type") {
            Some(s) => s.eq("multipart/form-data"),
            None => task.conf.method.to_uppercase().eq("POST"),
        };
        
        // Upload files one by one
        for index in start..size {
            #[cfg(feature = "oh")]
            let _trace = Trace::new(&format!("upload file:{} index:{}", task.task_id(), index));

            // Prepare individual file for upload
            if !task.prepare_single_upload(index).await {
                return Err(TaskError::Failed(Reason::OthersError));
            }

            // Select appropriate request builder based on content type
            let func = match is_multipart {
                true => build_multipart_request,
                false => build_stream_request,
            };
            upload_one_file(task.clone(), index, abort_flag.clone(), func).await?;
            task.notify_header_receive();
        }
    }

    info!("{} uploaded", task.task_id());
    Ok(())
}

/// Uploads a single file with timeout management.
/// 
/// Tracks upload time and adjusts the client timeout for the remaining operation.
/// 
/// # Type Parameters
/// 
/// * `F` - A function that builds the upload request.
/// 
/// # Arguments
/// 
/// * `task` - The request task containing the file to upload.
/// * `index` - The index of the file to upload.
/// * `abort_flag` - Atomic flag to signal upload cancellation.
/// * `build_upload_request` - Function to build the appropriate upload request.
/// 
/// # Returns
/// 
/// `Ok(())` if upload succeeds, or a `TaskError` if it fails.
async fn upload_one_file<F>(
    task: Arc<RequestTask>,
    index: usize,
    abort_flag: Arc<AtomicBool>,
    build_upload_request: F,
) -> Result<(), TaskError>
where
    F: Fn(Arc<RequestTask>, usize, Arc<AtomicBool>) -> Option<Request>,
{
    // Track upload time
    let begin_time = Instant::now();
    let result = upload_one_file_inner(
        task.clone(),
        index,
        abort_flag.clone(),
        build_upload_request,
    )
    .await;
    
    // Adjust timeout for remaining operations
    let upload_time = begin_time.elapsed().as_secs();
    task.rest_time.fetch_sub(upload_time, Ordering::SeqCst);
    let mut client = task.client.lock().await;
    client.total_timeout(Timeout::from_secs(task.rest_time.load(Ordering::SeqCst)));
    
    result
}

/// Internal implementation for uploading a single file.
/// 
/// Handles request construction, execution, response processing, and error handling
/// for individual file uploads.
/// 
/// # Type Parameters
/// 
/// * `F` - A function that builds the upload request.
/// 
/// # Arguments
/// 
/// * `task` - The request task containing the file to upload.
/// * `index` - The index of the file to upload.
/// * `abort_flag` - Atomic flag to signal upload cancellation.
/// * `build_upload_request` - Function to build the appropriate upload request.
/// 
/// # Returns
/// 
/// `Ok(())` if upload succeeds, or a `TaskError` if it fails.
/// 
/// # Errors
/// 
/// Returns specific error reasons based on the failure type:
/// - `BuildRequestFailed`: If request construction fails
/// - `ProtocolError`: For server errors, most client errors, or redirections
/// - `ContinuousTaskTimeout`: For request timeouts
/// - `RequestError`, `RedirectError`: For specific HTTP errors
/// - `Dns`, `Ssl`, `Tcp`: For network connection errors
/// - `LowSpeed`: For slow transfer rates
/// - `InsufficientSpace`: For storage space issues
/// - `UserAbort`: When upload is cancelled by user
/// - `OthersError`: For other miscellaneous errors
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

    // Build the upload request
    let Some(request) = build_upload_request(task.clone(), index, abort_flag) else {
        return Err(TaskError::Failed(Reason::BuildRequestFailed));
    };

    // Execute the request
    let client = task.client.lock().await;
    let response = client.request(request).await;
    
    // Process the response
    match response.as_ref() {
        Ok(response) => {
            let status_code = response.status();
            #[cfg(feature = "oh")]
            task.notify_response(response);
            info!(
                "{} response {}",
                task.conf.common_data.task_id, status_code,
            );
            
            // Handle various HTTP status codes
            if status_code.is_server_error()
                || (status_code.as_u16() != 408 && status_code.is_client_error())
                || status_code.is_redirection()
            {
                return Err(TaskError::Failed(Reason::ProtocolError));
            }
            
            // Special handling for timeout status (408)
            if status_code.as_u16() == 408 {
                if task.timeout_tries.load(Ordering::SeqCst) < 2 {
                    // Retry on timeout, but limit retry attempts
                    task.timeout_tries.fetch_add(1, Ordering::SeqCst);
                    return Err(TaskError::Waiting(TaskPhase::NeedRetry));
                } else {
                    // Too many timeout retries, fail permanently
                    return Err(TaskError::Failed(Reason::ProtocolError));
                }
            } else {
                // Reset timeout counter on successful response
                task.timeout_tries.store(0, Ordering::SeqCst);
            }
        }
        Err(e) => {
            // Only log non-abort errors
            if e.error_kind() != ErrorKind::UserAborted {
                error!("Task {} {:?}", task.task_id(), e);
            }

            // Map HTTP client errors to task errors
            match e.error_kind() {
                ErrorKind::Timeout => return Err(TaskError::Failed(Reason::ContinuousTaskTimeout)),
                ErrorKind::Request => return Err(TaskError::Failed(Reason::RequestError)),
                ErrorKind::Redirect => return Err(TaskError::Failed(Reason::RedirectError)),
                ErrorKind::Connect | ErrorKind::ConnectionUpgrade => {
                    // Handle connection errors with retry logic
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
                    // Handle transfer errors
                    if format!("{}", e).contains("Below low speed limit") {
                        return Err(TaskError::Failed(Reason::LowSpeed));
                    } else {
                        task.network_retry().await?;
                        return Err(TaskError::Failed(Reason::OthersError));
                    }
                }
                ErrorKind::UserAborted => return Err(TaskError::Waiting(TaskPhase::UserAbort)),
                _ => {
                    // Handle miscellaneous errors
                    if format!("{}", e).contains("No space left on device") {
                        return Err(TaskError::Failed(Reason::InsufficientSpace));
                    } else {
                        return Err(TaskError::Failed(Reason::OthersError));
                    }
                }
            };
        }
    };
    
    // Record the response
    task.record_upload_response(index, response).await;
    Ok(())
}

/// Unit tests for upload functionality.
/// 
/// Contains test cases for verifying upload operations under various conditions.
#[cfg(test)]
mod ut_upload {
    include!("../../tests/ut/task/ut_upload.rs");
}
