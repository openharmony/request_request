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

//! Callback handling for download operations.
//!
//! This module provides a prime callback implementation that handles download events
//! and communicates with cache storage, manages download state, and notifies registered
//! callbacks about download progress, success, failure, and cancellation.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use super::common::{CommonError, CommonResponse};
use super::{CacheDownloadError, RUNNING};
use crate::download::{CANCEL, FAIL, SUCCESS};
use crate::info::RustDownloadInfo;
use crate::services::{CacheDownloadService, PreloadCallback};
use cache_core::{CacheManager, Updater};
use netstack_rs::info::DownloadInfo;
use request_utils::task_id::TaskId;

/// Interval for reporting progress updates.
///
/// Progress updates are only reported once every PROGRESS_INTERVAL calls to avoid
/// excessive callback invocations during rapid data reception.
const PROGRESS_INTERVAL: usize = 8;

/// Primary callback handler for managing download operations and notifications.
///
/// Handles download lifecycle events, cache updates, progress reporting,
/// and callback management for download operations.
pub(crate) struct PrimeCallback {
    /// Unique identifier for the download task
    task_id: TaskId,
    /// Flag indicating whether the download has finished
    finish: Arc<AtomicBool>,
    /// Current state of the download (running, success, fail, cancel)
    state: Arc<AtomicUsize>,
    /// Handle for updating cache storage with downloaded data
    cache_handle: Updater,
    /// Queue of user-registered callbacks to notify about download events
    callbacks: Arc<Mutex<VecDeque<Box<dyn PreloadCallback>>>>,
    /// Restricts how frequently progress updates are reported
    progress_restriction: ProgressRestriction,
    /// Sequence number for task ordering
    seq: usize,
}

/// Restricts the frequency of progress updates.
///
/// Prevents excessive progress notifications by tracking the last reported
/// progress value and implementing a counter-based throttling mechanism.
struct ProgressRestriction {
    /// Last processed download position
    processed: u64,
    /// Counter for throttling progress updates
    count: usize,
    /// Whether any data has been received yet
    data_receive: bool,
}

impl ProgressRestriction {
    /// Creates a new progress restriction with default initial values.
    fn new() -> Self {
        Self {
            processed: 0,
            count: 0,
            data_receive: false,
        }
    }
}

impl PrimeCallback {
    /// Creates a new prime callback for a download task.
    ///
    /// # Parameters
    /// - `task_id`: Unique identifier for the download task
    /// - `cache_manager`: Cache manager for storing downloaded data
    /// - `finish`: Flag to indicate when the download has finished
    /// - `state`: Current state of the download
    /// - `callbacks`: Queue of callbacks to notify about download events
    /// - `seq`: Sequence number for task ordering
    ///
    /// # Returns
    /// A new `PrimeCallback` instance
    pub(crate) fn new(
        task_id: TaskId,
        cache_manager: &'static CacheManager,
        finish: Arc<AtomicBool>,
        state: Arc<AtomicUsize>,
        callbacks: Arc<Mutex<VecDeque<Box<dyn PreloadCallback>>>>,
        seq: usize,
    ) -> Self {
        Self {
            task_id: task_id.clone(),
            finish,
            state,
            cache_handle: Updater::new(task_id, cache_manager),
            callbacks,
            progress_restriction: ProgressRestriction::new(),
            seq,
        }
    }

    /// Sets the download state to running.
    pub(crate) fn set_running(&self) {
        self.state.store(RUNNING, Ordering::Release);
    }

    /// Gets the task ID associated with this callback.
    ///
    /// # Returns
    /// A copy of the task ID
    pub(crate) fn task_id(&self) -> TaskId {
        self.task_id.clone()
    }
}

impl PrimeCallback {
    /// Handles successful download completion.
    ///
    /// Updates the cache, changes the download state to success, and notifies all
    /// registered callbacks of the successful completion. Reports 100% progress
    /// before calling each callback's success method.
    ///
    /// # Type Parameters
    /// - `R`: Type implementing `CommonResponse` containing the HTTP status code
    ///
    /// # Parameters
    /// - `response`: Response object containing the status code
    pub(crate) fn common_success<R>(&mut self, response: R)
    where
        R: CommonResponse,
    {
        let code = response.code();
        info!("{} status {}", self.task_id.brief(), code);

        // Finalize cache storage
        let cache = self.cache_handle.cache_finish();
        // Update task state to success
        self.state.store(SUCCESS, Ordering::Release);
        self.finish.store(true, Ordering::Release);

        // Notify all registered callbacks
        let mut callbacks = self.callbacks.lock().unwrap();

        while let Some(mut callback) = callbacks.pop_front() {
            let clone_cache = cache.clone();
            let task_id = self.task_id.brief().to_string();
            // Spawn in separate tasks to avoid blocking
            crate::spawn(move || {
                // Report 100% progress before success
                callback.on_progress(clone_cache.size() as u64, clone_cache.size() as u64);
                callback.on_success(clone_cache, &task_id)
            });
        }

        // Explicit drop to release the mutex
        drop(callbacks);
        // Notify the service that the task has finished
        self.notify_agent_finish();
    }

    /// Handles download failure.
    ///
    /// Updates the download state to failed, and notifies all registered callbacks
    /// of the failure with the appropriate error information.
    ///
    /// # Type Parameters
    /// - `E`: Type implementing `CommonError` containing error information
    ///
    /// # Parameters
    /// - `error`: Error object containing the failure details
    pub(crate) fn common_fail<E>(&mut self, error: E, info: DownloadInfo)
    where
        E: CommonError,
    {
        info!("{} download failed {}", self.task_id.brief(), error.code());
        // Update task state to failed
        self.state.store(FAIL, Ordering::Release);
        self.finish.store(true, Ordering::Release);

        // Notify all registered callbacks
        let mut callbacks = self.callbacks.lock().unwrap();

        while let Some(mut callback) = callbacks.pop_front() {
            let task_id = self.task_id.brief().to_string();
            // Convert to the standard cache download error type
            let error = CacheDownloadError::from(&error);
            let info = RustDownloadInfo::from_download_info(info.clone());
            // Spawn in separate tasks to avoid blocking
            crate::spawn(move || callback.on_fail(error, info, &task_id));
        }

        // Explicit drop to release the mutex
        drop(callbacks);
        // Notify the service that the task has finished
        self.notify_agent_finish();
    }

    /// Handles download cancellation.
    ///
    /// Updates the download state to canceled, and notifies all registered callbacks
    /// of the cancellation.
    pub(crate) fn common_cancel(&mut self) {
        info!("{} is cancel", self.task_id.brief());
        // Update task state to canceled
        self.state.store(CANCEL, Ordering::Release);
        self.finish.store(true, Ordering::Release);

        // Notify all registered callbacks
        let mut callbacks = self.callbacks.lock().unwrap();

        while let Some(mut callback) = callbacks.pop_front() {
            // Spawn in separate tasks to avoid blocking
            crate::spawn(move || callback.on_cancel());
        }

        // Explicit drop to release the mutex
        drop(callbacks);
        // Notify the service that the task has finished
        self.notify_agent_finish();
    }

    /// Reports download progress to registered callbacks.
    ///
    /// Implements throttling to prevent excessive progress notifications, only
    /// reporting progress when the download has advanced and at a limited frequency.
    ///
    /// # Parameters
    /// - `dl_total`: Total number of bytes to download
    /// - `dl_now`: Current number of bytes downloaded
    /// - `_ul_total`: Total number of bytes to upload (unused in downloads)
    /// - `_ul_now`: Current number of bytes uploaded (unused in downloads)
    pub(crate) fn common_progress(
        &mut self,
        dl_total: u64,
        dl_now: u64,
        _ul_total: u64,
        _ul_now: u64,
    ) {
        // Skip if no data has been received yet, or if no progress has been made,
        // or if the download is complete (which will be handled by common_success)
        if !self.progress_restriction.data_receive
            || dl_now == self.progress_restriction.processed
            || dl_now == dl_total
        {
            return;
        }

        // Update the last processed position
        self.progress_restriction.processed = dl_now;

        // Implement throttling using a counter
        let count = self.progress_restriction.count;
        self.progress_restriction.count += 1;
        if count % PROGRESS_INTERVAL != 0 {
            return;
        }

        // Reset counter for next interval
        self.progress_restriction.count = 1;

        // Notify all registered callbacks of progress
        let mut callbacks = self.callbacks.lock().unwrap();
        for callback in callbacks.iter_mut() {
            callback.on_progress(dl_now, dl_total);
        }
    }

    /// Processes received data and updates the cache.
    ///
    /// Marks that data reception has started and forwards the data to the cache handler.
    ///
    /// # Type Parameters
    /// - `F`: Function type that returns the content length when called
    ///
    /// # Parameters
    /// - `data`: Buffer containing the received data
    /// - `content_length`: Function that returns the total content length if available
    pub(crate) fn common_data_receive<F>(&mut self, data: &[u8], content_length: F)
    where
        F: FnOnce() -> Option<usize>,
    {
        // Mark that data reception has started
        self.progress_restriction.data_receive = true;
        // Forward data to cache storage
        self.cache_handle.cache_receive(data, content_length);
    }

    /// Restarts the download by resetting the cache.
    ///
    /// # Notes
    /// Only available when the `netstack` feature is enabled.
    #[cfg(feature = "netstack")]
    pub(crate) fn common_restart(&mut self) {
        self.cache_handle.reset_cache();
    }

    /// Notifies the cache download service that the task has finished.
    ///
    /// Used to trigger any necessary cleanup or notification operations in the service.
    fn notify_agent_finish(&self) {
        CacheDownloadService::get_instance().task_finish(&self.task_id, self.seq);
    }
}
