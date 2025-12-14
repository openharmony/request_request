// Copyright (C) 2025 Huawei Device Co., Ltd.
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

//! Client API for managing download tasks.
//!
//! This module provides a high-level interface for creating, controlling, and monitoring
//! download tasks. It implements a singleton pattern for access to the request service
//! and handles communication through the request proxy.
//!
//! # Examples
//!
//! ```rust
//! use std::sync::Arc;
//! use request_utils::context::Context;
//! use request_core::config::{TaskConfig, Version};
//! use request_core::filter::SearchFilter;
//! use request_next::client::RequestClient;
//! use request_next::Callback;
//!
//! // Define a callback to receive download notifications
//! struct MyCallback;
//!
//! impl Callback for MyCallback {
//!     fn on_progress(&self, task_id: i64, current: i64, total: i64) {
//!         println!("Download progress: {}/{} bytes", current, total);
//!     }
//!
//!     fn on_complete(&self, task_id: i64) {
//!         println!("Download completed!");
//!     }
//!
//!     fn on_failed(&self, task_id: i64, error_code: i32) {
//!         println!("Download failed with code: {}", error_code);
//!     }
//! }
//!
//! fn manage_downloads() {
//!     // Get the singleton client instance
//!     let client = RequestClient::get_instance();
//!
//!     // Create and configure a download task
//!     let context = Context::new();
//!     let mut config = TaskConfig::new();
//!     config.url = "https://example.com/large-file.iso".to_string();
//!     config.add_header("User-Agent", "MyApp/1.0");
//!
//!     // Create the task
//!     match client.crate_task(
//!         context,
//!         Version::Api10,
//!         config,
//!         "/data/data/com.example.app/downloads/file.iso",
//!         false
//!     ) {
//!         Ok(task_id) => {
//!             println!("Created task with ID: {}", task_id);
//!
//!             // Register for progress updates
//!             client.register_callback(task_id, Arc::new(MyCallback));
//!
//!             // Start the download
//!             if let Err(err) = client.start(task_id) {
//!                 println!("Failed to start task: {}", err);
//!                 return;
//!             }
//!
//!             // ... later, if needed
//!             // Pause the download
//!             // client.pause(task_id).unwrap();
//!
//!             // Resume the download
//!             // client.resume(task_id).unwrap();
//!
//!             // Limit download speed to 1MB/s
//!             // client.set_max_speed(task_id, 1_048_576).unwrap();
//!
//!             // Get task information
//!             if let Ok(info) = client.show_task(task_id) {
//!                 println!("Task info: {:?}", info);
//!             }
//!         },
//!         Err(e) => {
//!             println!("Failed to create task: {:?}", e);
//!         }
//!     }
//!
//!     // Search for tasks
//!     let filter = SearchFilter::new();
//!     if let Ok(task_ids) = client.search(filter) {
//!         println!("Found {} tasks", task_ids.len());
//!     }
//! }
//! ```

// Public module exports
pub mod error;
mod native_task;
use std::path::PathBuf;

// Standard library imports
use std::sync::{Arc, OnceLock};

// External dependencies
use request_core::config::{Action, TaskConfig, Version};
use request_core::error_code::{CHANNEL_NOT_OPEN, OTHER};
use request_core::file::FileSpec;
use request_core::filter::SearchFilter;
use request_core::info::TaskInfo;
use request_utils::context::Context;

// Internal dependencies
use crate::client::error::CreateTaskError;
use crate::client::native_task::{NativeTask, NativeTaskManager};
use crate::file::FileManager;
use crate::listen::Observer;
use crate::proxy::RequestProxy;
use crate::verify::TaskConfigVerifier;
use crate::{check, Callback};

/// Client for interacting with the download service.
///
/// Provides methods to create, control, and monitor download tasks, maintaining
/// a singleton instance for consistent service access.
pub struct RequestClient<'a> {
    /// Listener for task status updates and events
    listener: Observer,
    pub task_manager: NativeTaskManager,
    /// Proxy for communicating with the download service
    proxy: &'a RequestProxy,
}

impl<'a> RequestClient<'a> {
    /// Gets the singleton instance of the `RequestClient`.
    ///
    /// Creates the instance if it doesn't exist, initializing the listener and
    /// opening a communication channel with the download service.
    ///
    /// # Returns
    /// A static reference to the `RequestClient` singleton
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_next::client::RequestClient;
    ///
    /// // Get the singleton instance
    /// let client1 = RequestClient::get_instance();
    /// let client2 = RequestClient::get_instance();
    ///
    /// // Both references point to the same instance
    /// assert!(std::ptr::eq(client1, client2));
    ///
    /// // Use the client for operations
    /// // let result = client1.start(123);
    /// ```
    pub fn get_instance() -> &'static Self {
        static INSTANCE: OnceLock<RequestClient> = OnceLock::new();

        INSTANCE.get_or_init(|| {
            let listener = Observer::new();
            let res = RequestClient {
                listener,
                task_manager: NativeTaskManager::default(),
                proxy: RequestProxy::get_instance(),
            };
            // Initialize communication channel on first creation
            res.open_channel();
            res
        })
    }

    pub fn check_config(
        &self,
        context: Context,
        seq: u64,
        mut config: TaskConfig,
    ) -> Result<(), CreateTaskError> {
        debug!("Creating task with config: {:?}", config);
        // todo: errcode and errmsg
        TaskConfigVerifier::get_instance().verify(&config)?;
        let token = FileManager::get_instance().apply(context, &mut config)?;
        let task = NativeTask {
            config,
            token,
        };
        self.task_manager.insert(seq, task);

        Ok(())
    }

    /// Creates a new download task with the specified configuration.
    ///
    /// Validates the download path, creates a file specification, and sends the task
    /// creation request to the service. Automatically reopens the channel if needed.
    ///
    /// # Parameters
    /// - `context`: Application context for path validation
    /// - `version`: API version to determine path handling
    /// - `config`: Task configuration including URL, headers, etc.
    /// - `save_as`: Path where the downloaded file should be saved
    /// - `overwrite`: Whether to overwrite existing files
    ///
    /// # Returns
    /// A task ID on success, or a `CreateTaskError` on failure
    ///
    /// # Errors
    /// - `CreateTaskError::DownloadPath`: If path validation fails
    /// - `CreateTaskError::Code`: If task creation fails for other reasons
    ///
    /// # Notes
    /// The function name contains a typo (`crate_task` instead of `create_task`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::sync::Arc;
    /// use request_utils::context::Context;
    /// use request_core::config::{TaskConfig, Version};
    /// use request_next::client::RequestClient;
    /// use request_next::Callback;
    ///
    /// // Create a simple callback implementation
    /// struct DownloadCallback;
    /// impl Callback for DownloadCallback {
    ///     fn on_progress(&self, task_id: i64, bytes_downloaded: i64, total_bytes: i64) {
    ///         println!("Task {} progress: {}/{} bytes", task_id, bytes_downloaded, total_bytes);
    ///     }
    ///
    ///     fn on_complete(&self, task_id: i64) {
    ///         println!("Task {} completed successfully", task_id);
    ///     }
    ///
    ///     fn on_failed(&self, task_id: i64, error_code: i32) {
    ///         println!("Task {} failed with error: {}", task_id, error_code);
    ///     }
    /// }
    ///
    /// // Example usage
    /// fn create_and_start_download() {
    ///     // Get the client instance
    ///     let client = RequestClient::get_instance();
    ///
    ///     // Create context and task configuration
    ///     let context = Context::new();
    ///     let mut config = TaskConfig::new();
    ///     config.url = "https://example.com/file.zip".to_string();
    ///
    ///     // Create the download task
    ///     match client.crate_task(
    ///         context,
    ///         Version::Api10,
    ///         config,
    ///         "/data/data/com.example.app/files/download.zip",
    ///         true
    ///     ) {
    ///         Ok(task_id) => {
    ///             println!("Task created with ID: {}", task_id);
    ///
    ///             // Register a callback for status updates
    ///             client.register_callback(task_id, Arc::new(DownloadCallback));
    ///
    ///             // Start the download
    ///             if let Err(err) = client.start(task_id) {
    ///                 println!("Failed to start task: {}", err);
    ///             }
    ///         },
    ///         Err(e) => {
    ///             println!("Failed to create task: {:?}", e);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn create_task(
        &self,
        context: Context,
        seq: u64
    ) -> Result<i64, CreateTaskError> {
        let task = self.task_manager.get_by_seq(&seq).ok_or(CreateTaskError::Code(OTHER))?;

        // Retry loop for channel reconnection
        loop {
            let res = match self.proxy.create(&task.config) {
                Err(e) => {
                    error!("Failed to create task: {:?}", e);
                    // Attempt to reopen channel if it's closed
                    if matches!(e, CreateTaskError::Code(CHANNEL_NOT_OPEN)) {
                        self.open_channel();
                        continue;
                    }
                    self.task_manager.remove(&seq);
                    Err(e)
                }
                Ok(task_id) => {
                    info!("Task created successfully with ID: {}", task_id);
                    self.task_manager.bind(task_id, seq);
                    Ok(task_id)
                }
            };
            break res;
        }
    }

    pub fn get_task(&self, task_id: i64, token: Option<String>) -> Result<TaskConfig, i32> {
        
        self.proxy.get_task(task_id, token)
    }

    /// Starts a download task with the specified ID.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to start
    ///
    /// # Returns
    /// `Ok(())` on success, or an error code on failure
    pub fn start(&self, task_id: i64) -> Result<(), i32> {
        self.proxy.start(task_id)
    }

    /// Pauses a running download task.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to pause
    ///
    /// # Returns
    /// `Ok(())` on success, or an error code on failure
    pub fn pause(&self, task_id: i64) -> Result<(), i32> {
        self.proxy.pause(task_id)
    }

    /// Resumes a paused download task.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to resume
    ///
    /// # Returns
    /// `Ok(())` on success, or an error code on failure
    pub fn resume(&self, task_id: i64) -> Result<(), i32> {
        self.proxy.resume(task_id)
    }

    /// Removes a download task and its associated files.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to remove
    ///
    /// # Returns
    /// `Ok(())` on success, or an error code on failure
    pub fn remove(&self, task_id: i64) -> Result<(), i32> {
        self.task_manager.remove_task(&task_id);
        self.proxy.remove(task_id)
    }

    /// Stops a running download task without removing files.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to stop
    ///
    /// # Returns
    /// `Ok(())` on success, or an error code on failure
    pub fn stop(&self, task_id: i64) -> Result<(), i32> {
        self.proxy.stop(task_id)
    }

    /// Sets the maximum download speed for a task.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to configure
    /// - `speed`: Maximum speed limit in bytes per second
    ///
    /// # Returns
    /// `Ok(())` on success, or an error code on failure
    pub fn set_max_speed(&self, task_id: i64, speed: i64) -> Result<(), i32> {
        self.proxy.set_max_speed(task_id, speed)
    }

    pub fn query_mime_type(&self, task_id: i64) -> Result<String, i32> {
        self.proxy.query_mime_type(task_id)
    }

    /// Registers a callback for task status updates.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to monitor
    /// - `callback`: Callback to receive status updates
    pub fn register_callback(
        &self,
        task_id: i64,
        callback: Arc<dyn Callback + Send + Sync + 'static>,
    ) {
        self.listener.register_callback(task_id, callback);
    }

    /// Opens the communication channel with the download service.
    ///
    /// Initializes the listener with a file descriptor from the proxy.
    pub fn open_channel(&self) {
        // Unwrap is safe as the proxy handles error conditions
        let file = self.proxy.open_channel().unwrap();
        self.listener.set_listenr(file);
    }

    /// Retrieves information about a specific task.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to query
    ///
    /// # Returns
    /// Task information on success, or an error code on failure
    pub fn show_task(&self, task_id: i64) -> Result<TaskInfo, i32> {
        self.proxy.show(task_id)
    }

    /// Searches for tasks matching the specified filter.
    ///
    /// # Parameters
    /// - `keyword`: Search filter defining the search criteria
    ///
    /// # Returns
    /// A list of matching task IDs on success, or an error code on failure
    pub fn search(&self, keyword: SearchFilter) -> Result<Vec<String>, i32> {
        self.proxy.search(keyword)
    }

    pub fn touch(&self, task_id: i64, token: String) -> Result<TaskInfo, i32> {
        self.proxy.touch(task_id, token)
    }

    pub fn query(&self, task_id: i64) -> Result<TaskInfo, i32> {
        self.proxy.query(task_id)
    }

    pub fn create_group(&self, gauge: Option<bool>, title: Option<String>, text: Option<String>, disable: Option<bool>) -> Result<String, i32> {
        self.proxy.create_group(gauge, title, text, disable)
    }

    pub fn attach_group(&self, group_id: String, task_ids: Vec<String>) -> Result<(), i32> {
        self.proxy.attach_group(group_id, task_ids)
    }

    pub fn delete_group(&self, group_id: String) -> Result<(), i32> {
        self.proxy.delete_group(group_id)
    }
}
