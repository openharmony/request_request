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

//! Task management for cache download operations.
//! 
//! This module defines the core task structures and management functionality for download
//! operations, supporting different download backends and callback handling.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use cache_core::CacheManager;
use netstack_rs::info::DownloadInfoMgr;
use request_utils::info;
use request_utils::task_id::TaskId;

use super::callback::PrimeCallback;
use super::common::CommonHandle;
use super::{INIT, SUCCESS};

cfg_ylong! {
    use crate::download::ylong;
}

cfg_netstack! {
    use crate::download::netstack;
}

use crate::services::{DownloadRequest, PreloadCallback};

/// Enum representing available download backends.
///
/// Used to select between different HTTP client implementations for download operations.
pub enum Downloader {
    /// Netstack-based HTTP client implementation.
    Netstack,
    /// Ylong-based HTTP client implementation.
    Ylong,
}

/// Main download task structure for managing download operations.
///
/// Represents a single download operation with its state and handle.
pub(crate) struct DownloadTask {
    /// Flag indicating whether the task should be removed from tracking.
    pub(crate) remove_flag: bool,
    /// Sequence number for task ordering.
    pub(crate) seq: usize,
    /// Handle for controlling the download operation.
    pub(crate) handle: TaskHandle,
}

impl DownloadTask {
    /// Creates a new download task with the specified parameters.
    ///
    /// # Parameters
    /// - `task_id`: Unique identifier for the download task.
    /// - `cache_manager`: Reference to the cache manager for storing downloaded content.
    /// - `info_mgr`: Manager for download information.
    /// - `request`: Download request configuration.
    /// - `callback`: Callback for download events.
    /// - `downloader`: Type of download backend to use.
    /// - `seq`: Sequence number for task ordering.
    ///
    /// # Returns
    /// A new `DownloadTask` if creation was successful, otherwise `None`.
    pub(crate) fn new(
        task_id: TaskId,
        cache_manager: &'static CacheManager,
        info_mgr: Arc<DownloadInfoMgr>,
        request: DownloadRequest,
        callback: Box<dyn PreloadCallback>,
        downloader: Downloader,
        seq: usize,
    ) -> Option<DownloadTask> {
        info!("new task {} seq {}", task_id.brief(), seq);
        let mut handle = None;
        match downloader {
            Downloader::Netstack => {
                #[cfg(feature = "netstack")]
                {
                    handle = download_inner(
                        task_id,
                        cache_manager,
                        info_mgr,
                        request,
                        Some(callback),
                        netstack::DownloadTask::run,
                        seq,
                    );
                }
            }
            Downloader::Ylong => {
                #[cfg(feature = "ylong")]
                {
                    handle = Some(download_inner(
                        task_id,
                        cache_manager,
                        request,
                        Some(callback),
                        ylong::DownloadTask::run,
                        seq,
                    ));
                }
            }
        };
        handle.map(|handle| DownloadTask {
            remove_flag: false,
            seq,
            handle,
        })
    }

    /// Cancels the download task.
    pub(crate) fn cancel(&mut self) {
        self.handle.cancel();
    }

    /// Gets a clone of the task handle.
    ///
    /// # Returns
    /// A clone of the `TaskHandle` associated with this task.
    pub(crate) fn task_handle(&self) -> TaskHandle {
        self.handle.clone()
    }

    /// Attempts to add a callback to the task.
    ///
    /// # Parameters
    /// - `callback`: Callback to add to the task.
    ///
    /// # Returns
    /// `Ok(())` if the callback was successfully added, otherwise returns the callback in `Err`.
    pub(crate) fn try_add_callback(
        &mut self,
        callback: Box<dyn PreloadCallback>,
    ) -> Result<(), Box<dyn PreloadCallback>> {
        self.handle.try_add_callback(callback)
    }
}

/// Handle for controlling a download task.
///
/// Provides methods for managing and monitoring download operations.
#[derive(Clone)]
pub struct TaskHandle {
    /// Unique identifier for the task.
    task_id: TaskId,
    /// Optional handle to the underlying download implementation.
    handle: Option<Arc<dyn CommonHandle>>,
    /// Atomic state of the download task.
    state: Arc<AtomicUsize>,
    /// Atomic flag indicating if the task has finished.
    finish: Arc<AtomicBool>,
    /// Queue of callbacks to notify about download events.
    callbacks: Arc<Mutex<VecDeque<Box<dyn PreloadCallback>>>>,
}

impl TaskHandle {
    /// Creates a new task handle with the specified task ID.
    ///
    /// # Parameters
    /// - `task_id`: Unique identifier for the task.
    pub(crate) fn new(task_id: TaskId) -> Self {
        Self {
            state: Arc::new(AtomicUsize::new(INIT)),
            task_id,
            handle: None,
            finish: Arc::new(AtomicBool::new(false)),
            callbacks: Arc::new(Mutex::new(VecDeque::with_capacity(1))),
        }
    }
    
    /// Cancels the download task if it exists and hasn't already finished.
    ///
    /// Uses atomic operations to ensure thread safety when checking and updating task state.
    pub(crate) fn cancel(&mut self) {
        if let Some(handle) = self.handle.take() {
            info!("cancel task {}", self.task_id.brief());
            if self.finish.load(Ordering::Acquire) {
                return;
            }
            let _callback = self.callbacks.lock().unwrap();
            // Double-check finish flag after acquiring lock
            if self.finish.load(Ordering::Acquire) {
                return;
            }
            if handle.cancel() {
                self.finish.store(true, Ordering::Release);
            }
        } else {
            error!("cancel task {} not exist", self.task_id.brief());
        }
    }

    /// Resets the download task if it hasn't finished.
    pub(crate) fn reset(&mut self) {
        if self.finish.load(Ordering::Acquire) {
            return;
        }
        if let Some(handle) = self.handle.as_ref() {
            handle.reset();
        }
    }

    /// Returns the task ID as a string.
    ///
    /// # Returns
    /// A string representation of the task ID.
    pub fn task_id(&self) -> String {
        self.task_id.to_string()
    }

    /// Checks if the task has finished.
    ///
    /// # Returns
    /// `true` if the task has finished, otherwise `false`.
    pub fn is_finish(&self) -> bool {
        self.finish.load(Ordering::Acquire)
    }

    /// Gets the current state of the task.
    ///
    /// # Returns
    /// The current state code as a `usize`.
    pub fn state(&self) -> usize {
        self.state.load(Ordering::Acquire)
    }

    /// Marks the task as completed successfully.
    pub(crate) fn set_completed(&self) {
        self.state.store(SUCCESS, Ordering::Relaxed);
        self.finish.store(true, Ordering::Relaxed);
    }

    /// Attempts to add a callback to the task if it hasn't finished.
    ///
    /// # Parameters
    /// - `callback`: Callback to add to the task.
    ///
    /// # Returns
    /// `Ok(())` if the callback was successfully added, otherwise returns the callback in `Err`.
    pub(crate) fn try_add_callback(
        &mut self,
        callback: Box<dyn PreloadCallback>,
    ) -> Result<(), Box<dyn PreloadCallback>> {
        let mut callbacks = self.callbacks.lock().unwrap();
        if !self.finish.load(Ordering::Acquire) {
            info!("add callback to task {}", self.task_id.brief());
            callbacks.push_back(callback);
            if let Some(handle) = self.handle.as_ref() {
                handle.add_count();
            }
            Ok(())
        } else {
            Err(callback)
        }
    }

    /// Gets a clone of the state atomic flag.
    ///
    /// # Returns
    /// A clone of the atomic state flag.
    #[inline]
    fn state_flag(&self) -> Arc<AtomicUsize> {
        self.state.clone()
    }

    /// Gets a clone of the finish atomic flag.
    ///
    /// # Returns
    /// A clone of the atomic finish flag.
    #[inline]
    fn finish_flag(&self) -> Arc<AtomicBool> {
        self.finish.clone()
    }

    /// Gets a clone of the callbacks queue.
    ///
    /// # Returns
    /// A clone of the mutex-protected callbacks queue.
    #[inline]
    fn callbacks(&self) -> Arc<Mutex<VecDeque<Box<dyn PreloadCallback>>>> {
        self.callbacks.clone()
    }

    /// Sets the underlying download handle.
    ///
    /// # Parameters
    /// - `handle`: The download handle to set.
    #[inline]
    fn set_handle(&mut self, handle: Arc<dyn CommonHandle>) {
        self.handle = Some(handle);
    }
}

/// Internal function to create and start a download task using the specified downloader.
///
/// # Type Parameters
/// - `F`: Type of the downloader function that performs the actual download operation.
///
/// # Parameters
/// - `task_id`: Unique identifier for the download task.
/// - `cache_manager`: Reference to the cache manager for storing downloaded content.
/// - `info_mgr`: Manager for download information.
/// - `request`: Download request configuration.
/// - `callback`: Optional callback for download events.
/// - `downloader`: Function that performs the actual download operation.
/// - `seq`: Sequence number for task ordering.
///
/// # Returns
/// A new `TaskHandle` if the download operation was successfully started, otherwise `None`.
fn download_inner<F>(
    task_id: TaskId,
    cache_manager: &'static CacheManager,
    info_mgr: Arc<DownloadInfoMgr>,
    request: DownloadRequest,
    callback: Option<Box<dyn PreloadCallback>>,
    downloader: F,
    seq: usize,
) -> Option<TaskHandle>
where
    F: Fn(DownloadRequest, PrimeCallback, Arc<DownloadInfoMgr>) -> Option<Arc<dyn CommonHandle>>,
{
    let mut handle = TaskHandle::new(task_id.clone());
    if let Some(callback) = callback {
        handle.callbacks.lock().unwrap().push_back(callback);
    }

    let callback = PrimeCallback::new(
        task_id,
        cache_manager,
        handle.finish_flag(),
        handle.state_flag(),
        handle.callbacks(),
        seq,
    );
    downloader(request, callback, info_mgr).map(move |command| {
        handle.set_handle(command);
        handle
    })
}

#[cfg(test)]
mod ut_task {
    include!("../../tests/ut/download/ut_task.rs");
}
