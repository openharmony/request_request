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

//! Download task implementation for HTTP requests.
//! 
//! This module provides functionality for downloading files via HTTP/HTTPS, including:
//! - Implementation of the `DownloadOperator` trait
//! - Download manager with retry logic
//! - Error handling and recovery
//! - Progress tracking and file verification
//! - Network state management


use std::io::SeekFrom;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Instant;

use ylong_http_client::async_impl::{DownloadOperator, Downloader, Response};
use ylong_http_client::{ErrorKind, HttpClientError, SpeedLimit, Timeout};

use super::operator::TaskOperator;
use super::reason::Reason;
use super::request_task::{TaskError, TaskPhase};
use crate::manage::database::RequestDb;
use crate::task::info::State;
use crate::task::request_task::RequestTask;
use crate::task::task_control;
#[cfg(feature = "oh")]
use crate::trace::Trace;
use crate::utils::get_current_duration;

/// Maximum download timeout duration (one week in seconds).
const SECONDS_IN_ONE_WEEK: u64 = 7 * 24 * 60 * 60;

/// Minimum time (in seconds) to consider a connection as low speed.
const LOW_SPEED_TIME: u64 = 60;

/// Minimum download speed (in bytes per second) before considering connection stalled.
const LOW_SPEED_LIMIT: u64 = 1;

/// Implementation of the `DownloadOperator` trait for `TaskOperator`.
///
/// This implementation enables `TaskOperator` to be used with the HTTP client's downloader
/// by providing file writing and progress reporting functionality.
impl DownloadOperator for TaskOperator {
    fn poll_download(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        data: &[u8],
    ) -> Poll<Result<usize, HttpClientError>> {
        self.poll_write_file(cx, data, 0)
    }

    fn poll_progress(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        _downloaded: u64,
        _total: Option<u64>,
    ) -> Poll<Result<(), HttpClientError>> {
        self.poll_progress_common(cx)
    }
}

/// Creates a downloader with the specified task, response, and abort flag.
///
/// Constructs a downloader configured with appropriate timeouts and speed limits
/// for the given download task.
///
/// # Arguments
///
/// * `task` - The download task containing configuration and state information.
/// * `response` - The HTTP response to download from.
/// * `abort_flag` - An atomic flag used to signal download cancellation.
///
/// # Returns
///
/// Returns a configured `Downloader` instance ready to start downloading.
///
/// # Examples
///
/// ```rust
/// use std::sync::{Arc, atomic::AtomicBool};
/// use ylong_http_client::async_impl::Response;
/// use crate::task::request_task::RequestTask;
/// use crate::task::download::build_downloader;
///
/// // Assuming we have a request task and response
/// let task = Arc::new(RequestTask::default());
/// let response = Response::default();
/// let abort_flag = Arc::new(AtomicBool::new(false));
///
/// // Build the downloader
/// let downloader = build_downloader(task, response, abort_flag);
///
/// // Start the download
/// // tokio::spawn(async move { downloader.download().await });
/// ```
pub(crate) fn build_downloader(
    task: Arc<RequestTask>,
    response: Response,
    abort_flag: Arc<AtomicBool>,
) -> Downloader<TaskOperator> {
    // Create a task operator to handle file writing and progress updates
    let task_operator = TaskOperator::new(task, abort_flag);

    // Configure the downloader with appropriate settings
    Downloader::builder()
        .body(response)  // Set the HTTP response to download from
        .operator(task_operator)  // Use our task operator for file operations
        .timeout(Timeout::from_secs(SECONDS_IN_ONE_WEEK))  // Set a long timeout for large downloads
        .speed_limit(SpeedLimit::new().min_speed(LOW_SPEED_LIMIT, LOW_SPEED_TIME))  // Set minimum speed threshold
        .build()
}

/// Handles the main download process with retry logic.
///
/// Manages the download lifecycle including retries, error handling, and result reporting.
/// This function coordinates the download_inner function and processes its results.
///
/// # Arguments
///
/// * `task` - The download task to execute.
/// * `abort_flag` - An atomic flag used to signal download cancellation.
///
/// # Examples
///
/// ```rust
/// use std::sync::{Arc, atomic::AtomicBool};
/// use crate::task::request_task::RequestTask;
/// use crate::task::download::download;
///
/// // Assuming we have a request task
/// let task = Arc::new(RequestTask::default());
/// let abort_flag = Arc::new(AtomicBool::new(false));
///
/// // Start the download
/// tokio::spawn(async move { download(task, abort_flag).await });
/// ```
pub(crate) async fn download(task: Arc<RequestTask>, abort_flag: Arc<AtomicBool>) {
    // Initialize retry counter
    task.tries.store(0, Ordering::SeqCst);
    
    // Main download loop with retry logic
    loop {
        let begin_time = Instant::now();
        
        // Execute the actual download logic
        if let Err(e) = download_inner(task.clone(), abort_flag.clone()).await {
            match e {
                TaskError::Waiting(phase) => match phase {
                    // Handle retry case: update timeout and continue the loop
                    TaskPhase::NeedRetry => {
                        // Update the remaining time based on elapsed download time
                        let download_time = begin_time.elapsed().as_secs();
                        task.rest_time.fetch_sub(download_time, Ordering::SeqCst);
                        
                        // Adjust client timeout to match remaining task time
                        let mut client = task.client.lock().await;
                        client.total_timeout(Timeout::from_secs(
                            task.rest_time.load(Ordering::SeqCst),
                        ));
                        
                        // Continue to next iteration for retry
                        continue;
                    }
                    // Handle user abort: break the loop without setting an error
                    TaskPhase::UserAbort => {},
                    // Handle network offline: record the error
                    TaskPhase::NetworkOffline => {
                        *task.running_result.lock().unwrap() = Some(Err(Reason::NetworkOffline));
                    }
                },
                // Handle failure errors: record the specific failure reason
                TaskError::Failed(reason) => {
                    *task.running_result.lock().unwrap() = Some(Err(reason));
                }
            }
        } else {
            // Download completed successfully
            *task.running_result.lock().unwrap() = Some(Ok(()));
        }
        
        // Exit the loop after handling success or non-retryable errors
        break;
    }
}

impl RequestTask {
    async fn prepare_download(&self) -> Result<(), TaskError> {
        if let Some(file) = self.files.get(0) {
            // Seek to the end of the file to get the current size (for resuming downloads)
            task_control::file_seek(file.clone(), SeekFrom::End(0)).await?;
            
            // Get the current file size to determine how much has already been downloaded
            let downloaded = task_control::file_metadata(file).await?.len() as usize;

            // Update progress tracking information
            let mut progress = self.progress.lock().unwrap();
            progress.common_data.index = 0;  // Set file index
            progress.common_data.total_processed = downloaded;  // Set bytes already downloaded
            progress.common_data.state = State::Running.repr;  // Set task state to running
            progress.processed = vec![downloaded];  // Track processed bytes for the file
        } else {
            // Log and return error if no file is available
            error!("prepare_download err, no file in the task");
            return Err(TaskError::Failed(Reason::OthersError));
        }
        Ok(())
    }
}

/// Performs the core download operation including request handling and file writing.
///
/// Handles the complete download process including preparing the request, sending it,
/// processing the response, and downloading the file content.
///
/// # Arguments
///
/// * `task` - The download task to execute.
/// * `abort_flag` - An atomic flag used to signal download cancellation.
///
/// # Returns
///
/// Returns `Ok(())` if the download completes successfully, or a `TaskError` if any
/// part of the download process fails.
///
/// # Errors
///
/// Returns various `TaskError` variants depending on the nature of the failure:
/// - `TaskError::Waiting` for temporary issues that may be retried
/// - `TaskError::Failed` for permanent failures with specific reasons
pub(crate) async fn download_inner(
    task: Arc<RequestTask>,
    abort_flag: Arc<AtomicBool>,
) -> Result<(), TaskError> {
    // Ensures `_trace` can only be freed when this function exits.
    #[cfg(feature = "oh")]
    let _trace = Trace::new("download file");

    // Prepare the download task by initializing file pointers and progress tracking
    task.prepare_download().await?;

    // Log that the download has started
    info!("{} downloading", task.task_id());

    // Build the HTTP request for downloading
    let request = RequestTask::build_download_request(task.clone()).await?;

    // Record the start time for tracking
    let start_time = get_current_duration().as_secs() as u64;
    task.start_time.store(start_time as u64, Ordering::SeqCst);

    // Acquire the client lock and send the request
    // Send HTTP request and handle response with detailed error categorization
    let client = task.client.lock().await;
    let response = client.request(request).await;

    // Handle response and categorize errors based on status codes and error types
    match response.as_ref() {
        Ok(response) => {
            // Extract and log the status code
            let status_code = response.status();
            #[cfg(feature = "oh")]
            task.notify_response(response);
            info!(
                "{} response {}",
                task.conf.common_data.task_id, status_code
            );

            // Handle protocol errors (server errors, most client errors, and redirects)
            if status_code.is_server_error()
                || (status_code.as_u16() != 408 && status_code.is_client_error())
                || status_code.is_redirection()
            {
                return Err(TaskError::Failed(Reason::ProtocolError));
            }

            // Handle timeout errors with retry logic
            if status_code.as_u16() == 408 {
                if task.timeout_tries.load(Ordering::SeqCst) < 2 {
                    // Retry up to 2 times for timeout errors
                    task.timeout_tries.fetch_add(1, Ordering::SeqCst);
                    return Err(TaskError::Waiting(TaskPhase::NeedRetry));
                } else {
                    // Too many timeout retries, consider it a failure
                    return Err(TaskError::Failed(Reason::ProtocolError));
                }
            } else {
                // Reset timeout retry counter on successful responses
                task.timeout_tries.store(0, Ordering::SeqCst);
            }

            // Handle OK status code (200)
            if status_code.as_u16() == 200 {
                // Check if range requests are required but not supported
                if task.require_range() {
                    info!("task {} server not support range", task.task_id());
                    return Err(TaskError::Failed(Reason::UnsupportedRangeRequest));
                }

                // Verify and prepare the download file
                if let Some(file) = task.files.get(0) {
                    // Check if file already has content (which shouldn't happen for new downloads)
                    let has_downloaded = task_control::file_metadata(file).await?.len() > 0;
                    if has_downloaded {
                        error!("task {} file not cleared", task.task_id());
                        sys_event!(
                            ExecFault,
                            DfxCode::TASK_FAULT_09,
                            &format!("task {} file not cleared", task.task_id())
                        );
                        // Clear partial downloads before starting fresh
                        task_control::clear_downloaded_file(task.clone()).await?;
                    }
                } else {
                    error!("download_inner err, no file in the `task`");
                    return Err(TaskError::Failed(Reason::OthersError));
                }
            }
        }
        Err(e) => {
            // Log the error for debugging purposes
            error!("Task {} {:?}", task.task_id(), e);

            // Categorize errors based on their type for appropriate handling
            match e.error_kind() {
                ErrorKind::Timeout => {
                    // Handle timeout errors
                    sys_event!(
                        ExecFault,
                        DfxCode::TASK_FAULT_01,
                        &format!("Task {} {:?}", task.task_id(), e)
                    );
                    return Err(TaskError::Failed(Reason::ContinuousTaskTimeout));
                }
                ErrorKind::Request => {
                    // Handle request errors (malformed requests, invalid parameters)
                    sys_event!(
                        ExecFault,
                        DfxCode::TASK_FAULT_02,
                        &format!("Task {} {:?}", task.task_id(), e)
                    );
                    return Err(TaskError::Failed(Reason::RequestError));
                }
                ErrorKind::Redirect => {
                    // Handle redirect errors (too many redirects, invalid redirect URLs)
                    sys_event!(
                        ExecFault,
                        DfxCode::TASK_FAULT_08,
                        &format!("Task {} {:?}", task.task_id(), e)
                    );
                    return Err(TaskError::Failed(Reason::RedirectError));
                }
                ErrorKind::Connect | ErrorKind::ConnectionUpgrade => {
                    // Handle connection errors with network retry and further categorization
                    task.network_retry().await?;
                    if e.is_dns_error() {
                        // DNS resolution errors
                        sys_event!(
                            ExecFault,
                            DfxCode::TASK_FAULT_05,
                            &format!("Task {} {:?}", task.task_id(), e)
                        );
                        return Err(TaskError::Failed(Reason::Dns));
                    } else if e.is_tls_error() {
                        // TLS/SSL handshake errors
                        sys_event!(
                            ExecFault,
                            DfxCode::TASK_FAULT_07,
                            &format!("Task {} {:?}", task.task_id(), e)
                        );
                        return Err(TaskError::Failed(Reason::Ssl));
                    } else {
                        // General TCP connection errors
                        sys_event!(
                            ExecFault,
                            DfxCode::TASK_FAULT_06,
                            &format!("Task {} {:?}", task.task_id(), e)
                        );
                        return Err(TaskError::Failed(Reason::Tcp));
                    }
                }
                ErrorKind::BodyTransfer => {
                    // Handle data transfer errors during body download
                    task.network_retry().await?;
                    sys_event!(
                        ExecFault,
                        DfxCode::TASK_FAULT_09,
                        &format!("Task {} {:?}", task.task_id(), e)
                    );
                    return Err(TaskError::Failed(Reason::OthersError));
                }
                _ => {
                    // Handle miscellaneous errors
                    if format!("{}", e).contains("No space left on device") {
                        // Specifically detect storage space errors
                        sys_event!(
                            ExecFault,
                            DfxCode::TASK_FAULT_09,
                            &format!("Task {} {:?}", task.task_id(), e)
                        );
                        return Err(TaskError::Failed(Reason::InsufficientSpace));
                    } else {
                        // Catch-all for other types of errors
                        sys_event!(
                            ExecFault,
                            DfxCode::TASK_FAULT_09,
                            &format!("Task {} {:?}", task.task_id(), e)
                        );
                        return Err(TaskError::Failed(Reason::OthersError));
                    }
                }
            };
        }
    };

    let response = response.unwrap();
    {
        let mut guard = task.progress.lock().unwrap();
        guard.extras.clear();
        for (k, v) in response.headers() {
            if let Ok(value) = v.to_string() {
                guard.extras.insert(k.to_string().to_lowercase(), value);
            }
        }
    }
    task.get_file_info(&response)?;
    task.update_progress_in_database();
    RequestDb::get_instance()
        .update_task_sizes(task.task_id(), &task.progress.lock().unwrap().sizes);

    #[cfg(feature = "oh")]
    let _trace = Trace::new(&format!(
        "download file tid:{} size:{}",
        task.task_id(),
        task.progress
            .lock()
            .unwrap()
            .sizes
            .first()
            .unwrap_or_else(|| {
                error!("Failed to get a progress lock size from an empty vector in Progress");
                &0
            })
    ));
    let mut downloader = build_downloader(task.clone(), response, abort_flag);

    if let Err(e) = downloader.download().await {
        return task.handle_download_error(e).await;
    }

    let file_mutex = task.files.get(0).unwrap();
    task_control::file_sync_all(file_mutex).await?;

    #[cfg(not(test))]
    check_file_exist(&task)?;
    {
        let mut guard = task.progress.lock().unwrap();
        guard.sizes = vec![guard.processed.first().map_or_else(
            || {
                error!("Failed to get a process size from an empty vector in RequestTask");
                Default::default()
            },
            |x| *x as i64,
        )];
    }

    info!("{} downloaded", task.task_id());
    Ok(())
}

/// Checks if the download file exists and is valid.
///
/// Verifies file existence and metadata, with special handling for user files
/// which cannot be directly accessed by the download server.
///
/// # Arguments
///
/// * `task` - The download task containing the file to check.
///
/// # Returns
///
/// Returns `Ok(())` if the file exists and is valid, or a `TaskError` if there
/// are issues with the file.
///
/// # Errors
///
/// Returns `TaskError::Failed(Reason::OthersError)` if bundle cache operations fail.
/// Returns `TaskError::Failed(Reason::IoError)` if the file doesn't exist or isn't a file.
#[cfg(not(test))]
fn check_file_exist(task: &Arc<RequestTask>) -> Result<(), TaskError> {
    use crate::task::files::{convert_path, BundleCache};

    let config = task.config();
    // Skip check for user files which download_server cannot access directly
    if let Some(first_file_spec) = config.file_specs.first() {
        if first_file_spec.is_user_file {
            return Ok(());
        }
    } else {
        info!("Failed to get the first FileSpec from an empty vector in TaskConfig");
    }

    // Resolve the bundle name from the cache
    let mut bundle_cache = BundleCache::new(config);
    let bundle_name = bundle_cache
        .get_value()
        .map_err(|_| TaskError::Failed(Reason::OthersError))?;

    // Convert the logical path to a real filesystem path
    let real_path = convert_path(
        config.common_data.uid,
        &bundle_name,
        match &config.file_specs.first() {
            Some(spec) => &spec.path,
            None => {
                error!("Failed to get the first file_spec from an empty vector in TaskConfig");
                Default::default()
            }
        },
    );

    // Check if the file exists and is a regular file
    // Note: Cannot compare sizes because file_total_size may change when resuming a task
    match std::fs::metadata(real_path) {
        Ok(metadata) => {
            if !metadata.is_file() {
                error!("task {} check local not file", task.task_id());
                sys_event!(
                    ExecFault,
                    DfxCode::TASK_FAULT_04,
                    &format!("task {} check local not file", task.task_id())
                );
                return Err(TaskError::Failed(Reason::IoError));
            }
        }
        Err(e) => {
            // Handle file not found errors
            if e.kind() == std::io::ErrorKind::NotFound {
                error!("task {} check local not exist", task.task_id());
                sys_event!(
                    ExecFault,
                    DfxCode::TASK_FAULT_04,
                    &format!("task {} check local not exist", task.task_id())
                );
                return Err(TaskError::Failed(Reason::IoError));
            }
        }
    }
    Ok(())
}

/// Unit tests for the download module.
///
/// Contains tests for download functionality, error handling, and edge cases.
/// Tests use mock implementations to simulate various network and file system conditions.
#[cfg(not(feature = "oh"))]
#[cfg(test)]
mod ut_download {
    include!("../../tests/ut/task/ut_download.rs");
}
