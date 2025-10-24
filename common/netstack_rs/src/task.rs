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

//! Module for HTTP request task management.
//! 
//! This module provides types and functionality for managing asynchronous HTTP request tasks,
//! including starting, canceling, resetting, and querying their status.

use std::collections::HashMap;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use cxx::SharedPtr;
use request_utils::error;
use request_utils::task_id::TaskId;

use crate::info::DownloadInfoMgr;
use crate::request::RequestCallback;
use crate::response::Response;
use crate::wrapper::ffi::{HttpClientRequest, HttpClientTask, NewHttpClientTask, OnCallback};
use crate::wrapper::CallbackWrapper;

/// A handle to an asynchronous HTTP request task.
///
/// Provides control over an ongoing HTTP request, allowing operations like starting,
/// canceling, and resetting the request, as well as accessing response data.
///
/// # Examples
///
/// ```
/// use netstack_rs::{Request, RequestTask};
///
/// // Create a request
/// let mut request = Request::builder()
///     .uri("https://example.com")
///     .method("GET")
///     .build();
/// 
/// // Create a task from the request
/// let mut task = request.task().unwrap();
/// 
/// // Start the task
/// if task.start() {
///     println!("Task started successfully");
/// }
/// 
/// // Later, if needed, cancel the task
/// task.cancel();
/// ```
#[derive(Clone)]
pub struct RequestTask {
    /// Shared reference to the underlying FFI task object
    inner: Arc<Mutex<SharedPtr<HttpClientTask>>>,
    /// Flag indicating if the task should be reset
    reset: Arc<AtomicBool>,
}

// SAFETY: The inner HttpClientTask is thread-safe through Mutex and Arc
unsafe impl Send for RequestTask {}
unsafe impl Sync for RequestTask {}

/// The current status of a request task.
///
/// # Examples
///
/// ```
/// use netstack_rs::{Request, TaskStatus};
///
/// // Create a request and task
/// let mut request = Request::builder()
///     .uri("https://example.com")
///     .build();
/// let mut task = request.task().unwrap();
/// 
/// // Check initial status
/// let status = task.status();
/// println!("Initial status: {:?}", status);
/// 
/// // Start the task
/// task.start();
/// 
/// // Check if task is running
/// if let TaskStatus::Running = task.status() {
///     println!("Task is now running");
/// }
/// ```
#[derive(Debug, Default)]
pub enum TaskStatus {
    /// The task is idle and not currently running
    Idle,
    /// The task is actively running (default state)
    #[default]
    Running,
}

impl RequestTask {
    /// Creates a new RequestTask from an HTTP request.
    ///
    /// # Arguments
    ///
    /// * `request` - The prepared HTTP request to execute
    ///
    /// # Returns
    ///
    /// `Some(RequestTask)` if creation succeeded, `None` if creation failed
    pub(crate) fn from_http_request(request: &HttpClientRequest) -> Option<Self> {
        let http_task = NewHttpClientTask(request);
        // Check if task creation failed
        if http_task.is_null() {
            error!("from_http_request NewHttpClientTask return null.");
            return None;
        }
        // Wrap the FFI task in thread-safe containers
        Some(Self {
            inner: Arc::new(Mutex::new(http_task)),
            reset: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Creates a RequestTask from a raw FFI task pointer.
    ///
    /// # Arguments
    ///
    /// * `inner` - The raw FFI task pointer
    pub(crate) fn from_ffi(inner: SharedPtr<HttpClientTask>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
            reset: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Starts execution of the request task.
    ///
    /// # Returns
    ///
    /// `true` if the task started successfully, `false` otherwise
    ///
    /// # Safety
    ///
    /// Uses unsafe code to convert the shared pointer to a mutable pointer for the FFI API.
    pub fn start(&mut self) -> bool {
        unsafe {
            // Convert from const to mutable pointer for FFI compatibility
            let ptr = self.inner.lock().unwrap().as_ref().unwrap() as *const HttpClientTask
                as *mut HttpClientTask;
            // Create a pinned mutable reference to call the Start method
            Pin::new_unchecked(ptr.as_mut().unwrap()).Start()
        }
    }

    /// Cancels the ongoing request task.
    ///
    /// Terminates the request if it is in progress.
    pub fn cancel(&self) {
        // Clone the task to avoid holding the lock during cancellation
        let task = self.inner.lock().unwrap().clone();
        Self::pin_mut(&task).Cancel();
    }

    /// Resets the task for potential reuse.
    ///
    /// Cancels any ongoing operation and prepares the task for restarting with new parameters.
    /// Uses atomic compare-and-exchange to ensure reset is only performed once.
    pub fn reset(&self) {
        // Only perform reset if not already resetting
        if self
            .reset
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            self.cancel();
        }
    }

    /// Gets the current status of the task.
    ///
    /// # Returns
    ///
    /// The current `TaskStatus` of the request
    pub fn status(&mut self) -> TaskStatus {
        // Clone the task to avoid holding the lock during status check
        let task = self.inner.lock().unwrap().clone();
        // Try to convert the status code, defaulting to Running if conversion fails
        Self::pin_mut(&task)
            .GetStatus()
            .try_into()
            .unwrap_or_default()
    }

    /// Gets the response from the completed task.
    ///
    /// # Returns
    ///
    /// A `Response` object containing the HTTP response data
    pub fn response(&mut self) -> Response {
        // Clone the task to maintain ownership while creating the response
        let task = self.inner.lock().unwrap().clone();
        Response::from_shared(task)
    }

    /// Gets all response headers as a case-insensitive HashMap.
    ///
    /// Header names are converted to lowercase for consistent access.
    pub fn headers(&mut self) -> HashMap<String, String> {
        // Delegate to the response object's headers method
        self.response().headers()
    }

    /// Sets the callback handler for this task.
    ///
    /// # Arguments
    ///
    /// * `callback` - The callback implementation
    /// * `info_mgr` - Download info manager for performance tracking
    /// * `task_id` - Unique identifier for this task
    pub(crate) fn set_callback(
        &mut self,
        callback: Box<dyn RequestCallback + 'static>,
        info_mgr: Arc<DownloadInfoMgr>,
        task_id: TaskId,
    ) {
        let task = self.inner.lock().unwrap();
        OnCallback(
            &task,
            Box::new(CallbackWrapper::from_callback(
                callback,
                self.reset.clone(),
                Arc::downgrade(&self.inner),
                task_id,
                info_mgr,
                0,
            )),
        );
    }

    /// Helper method to get a pinned mutable reference to the underlying task.
    ///
    /// # Arguments
    ///
    /// * `ptr` - Shared pointer to the FFI task object
    ///
    /// # Returns
    ///
    /// Pinned mutable reference to the task
    ///
    /// # Safety
    ///
    /// Assumes the pointer is non-null and valid for the lifetime of the returned reference.
    pub(crate) fn pin_mut(ptr: &SharedPtr<HttpClientTask>) -> Pin<&mut HttpClientTask> {
        // Convert from const to mutable pointer for FFI compatibility
        let ptr = ptr.as_ref().unwrap() as *const HttpClientTask as *mut HttpClientTask;
        // Safety: Assumes the pointer is non-null and valid
        unsafe { Pin::new_unchecked(ptr.as_mut().unwrap()) }
    }
}

#[cfg(test)]
mod ut_task {
    include!("../tests/ut/ut_task.rs");
}
