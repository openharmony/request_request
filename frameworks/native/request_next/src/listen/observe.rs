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

//! Event observation system for download tasks.
//!
//! This module provides the infrastructure for monitoring and responding to download
//! task events through a callback mechanism. It includes the `Observer` struct that
//! manages callbacks and dispatches events, and the `Callback` trait that defines the
//! interface for handling these events.

// Standard library imports
use std::collections::HashMap;
use std::fs::File;
use std::sync::{Arc, Mutex};

// External dependencies
use request_core::config::{Action, Version};
use request_core::info::{Faults, Progress, Response, SubscribeType, TaskState, NotifyData};
use ylong_runtime::task::JoinHandle;
use crate::client::RequestClient;
use crate::file::FileManager;

// Internal dependencies
use crate::listen::uds::{Message, UdsListener};

/// Manages callbacks and dispatches task events to registered observers.
///
/// Maintains a registry of callbacks associated with task IDs and listens for events
/// from the download service through a Unix domain socket. When events are received,
/// they are dispatched to the appropriate callback based on the task ID and event type.
pub struct Observer {
    /// Registry mapping task IDs to their corresponding callback implementations
    callbacks: Arc<Mutex<HashMap<i64, Arc<dyn Callback + Send + Sync + 'static>>>>,
    /// Handle to the background task listening for events
    listener: Mutex<Option<JoinHandle<()>>>,
}

/// Trait defining the interface for handling download task events.
///
/// Implementations of this trait can receive notifications about various download
/// task events such as progress updates, completion, failure, and state changes.
/// All methods have empty default implementations to allow implementing only the
/// methods of interest.
///
/// # Examples
///
/// ```rust
/// use request_core::info::{Progress, Response};
/// use request_next::listen::observe::Callback;
/// use std::sync::Arc;
///
/// // Custom callback implementation that logs download progress
/// struct ProgressLogger;
///
/// impl Callback for ProgressLogger {
///     fn on_progress(&self, progress: &Progress) {
///         println!("Download progress: {} bytes downloaded of {} total",
///                  progress.download_size, progress.total_size);
///     }
///
///     fn on_completed(&self, progress: &Progress) {
///         println!("Download completed: {} bytes downloaded", progress.download_size);
///     }
///
///     fn on_failed(&self, progress: &Progress, error_code: i32) {
///         println!("Download failed with error code {} after {} bytes",
///                  error_code, progress.download_size);
///     }
/// }
///
/// // Create and use the callback
/// let callback = Arc::new(ProgressLogger);
/// // observer.register_callback(task_id, callback); // Register with Observer
/// ```
pub trait Callback {
    /// Called when download progress is updated.
    ///
    /// # Parameters
    /// - `progress`: Current progress information including bytes downloaded and total size
    fn on_progress(&self, progress: &Progress) {}

    /// Called when a download completes successfully.
    ///
    /// # Parameters
    /// - `progress`: Final progress information with complete download details
    fn on_completed(&self, progress: &Progress) {}

    /// Called when a download fails.
    ///
    /// # Parameters
    /// - `progress`: Progress information at the time of failure
    /// - `error_code`: Error code indicating the reason for failure
    fn on_failed(&self, progress: &Progress, error_code: i32) {}

    /// Called when a download is paused.
    ///
    /// # Parameters
    /// - `progress`: Progress information at the time of pausing
    fn on_pause(&self, progress: &Progress) {}

    /// Called when a paused download is resumed.
    ///
    /// # Parameters
    /// - `progress`: Progress information at the time of resuming
    fn on_resume(&self, progress: &Progress) {}

    /// Called when a download task is removed.
    ///
    /// # Parameters
    /// - `progress`: Progress information at the time of removal
    fn on_remove(&self, progress: &Progress) {}

    /// Called when an HTTP response is received.
    ///
    /// # Parameters
    /// - `response`: HTTP response details including status code, headers, etc.
    fn on_response(&self, response: &Response) {}

    /// Called when HTTP headers are received but before the response body starts downloading.
    fn on_header_receive(&self, progress: &Progress) {}
    fn on_fault(&self, faults: Faults) {}
    fn on_complete_upload(&self, task_states: Vec<TaskState>) {}
    fn on_fail_upload(&self, task_states: Vec<TaskState>) {}
}

impl Observer {
    /// Creates a new `Observer` instance.
    ///
    /// Initializes empty collections for callbacks and the listener handle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_next::listen::observe::Observer;
    ///
    /// let observer = Observer::new();
    /// ```
    pub fn new() -> Self {
        Observer {
            callbacks: Arc::new(Mutex::new(HashMap::new())),
            listener: Mutex::new(None),
        }
    }

    /// Sets up the event listener with the provided file descriptor.
    ///
    /// Creates a new UDS listener with the given file descriptor and starts a background
    /// task to listen for and process incoming messages. Cancels any existing listener
    /// if one is already running.
    ///
    /// # Parameters
    /// - `file`: File descriptor connected to the download service's event stream
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_next::listen::observe::Observer;
    /// use std::fs::File;
    ///
    /// // In a real application, this file would be obtained from a service connection
    /// // let file = ...; // File descriptor connected to UDS socket
    ///
    /// // let observer = Observer::new();
    /// // observer.set_listenr(file); // Start listening for events
    /// ```
    ///
    /// # Notes
    /// The function name contains a typo (`set_listenr` instead of `set_listener`).
    pub fn set_listenr(&self, file: File) {
        let mut listener = UdsListener::new(file);
        let callbacks = self.callbacks.clone();

        // Spawn background task to process incoming messages
        let handle = ylong_runtime::spawn(async move {
            loop {
                match listener.recv().await {
                    Ok(mut message) => match &mut message {
                        Message::HttpResponse(response) => {
                            // Convert task_id from string to i64 for lookup
                            let task_id = response.task_id.parse().unwrap();
                            if let Some(callback) = callbacks.lock().unwrap().get(&task_id) {
                                callback.on_response(&response);
                            }
                        }
                        Message::NotifyData(data) => {
                            let task_id = data.task_id as i64;
                            Observer::process_header_receive(data);
                            let mut progress = &data.progress;

                            // Find the appropriate callback for the task
                            if let Some(callback) = callbacks.lock().unwrap().get(&task_id) {
                                // Dispatch to the appropriate callback method based on event type
                                match data.version {
                                    Version::API10 => match data.subscribe_type {
                                        SubscribeType::Progress => {
                                            callback.on_progress(&progress);
                                        }
                                        SubscribeType::Completed => {
                                            callback.on_completed(&progress);
                                        }
                                        SubscribeType::Failed => {
                                            callback.on_failed(
                                                &progress,
                                                data.task_states[0].response_code as i32,
                                            );
                                        }
                                        SubscribeType::Pause => {
                                            callback.on_pause(&progress);
                                        }
                                        SubscribeType::Resume => {
                                            callback.on_resume(&progress);
                                        }
                                        SubscribeType::Remove => {
                                            callback.on_remove(&progress);
                                        }
                                        _ => {}
                                    },
                                    Version::API9 => match data.action {
                                        Action::Download => match data.subscribe_type {
                                            SubscribeType::Completed => {
                                                callback.on_completed(&progress);
                                            }
                                            SubscribeType::Pause => {
                                                callback.on_pause(&progress);
                                            }
                                            SubscribeType::Remove => {
                                                callback.on_remove(&progress);
                                            }
                                            SubscribeType::Failed => {
                                                callback.on_failed(
                                                    &progress,
                                                    data.task_states[0].response_code as i32,
                                                );
                                            }
                                            SubscribeType::Progress => {
                                                callback.on_progress(&progress);
                                            }
                                            _ => {
                                                error!("bad subscribeType ");
                                            }
                                        },
                                        Action::Upload => match data.subscribe_type {
                                            SubscribeType::Progress => {
                                                callback.on_progress(&progress);
                                            }
                                            SubscribeType::Completed => {
                                                callback.on_complete_upload(data.task_states.clone());
                                            }
                                            SubscribeType::Failed => {
                                                callback.on_fail_upload(data.task_states.clone());
                                            }
                                            SubscribeType::HeaderReceive => {
                                                callback.on_header_receive(&progress);
                                            }
                                            _ => {
                                                error!("bad subscribeType ");
                                            }
                                        },
                                    },
                                }

                            }
                        }
                        Message::Faults(faultOccur) => {
                            let task_id = faultOccur.task_id as i64;
                            if let Some(callback) = callbacks.lock().unwrap().get(&task_id) {
                                callback.on_fault(faultOccur.faults);
                            }
                        }
                    },
                    Err(e) => error!("Error receiving message: {}", e),
                }
            }
        });

        // Replace and cancel any existing listener
        if let Some(old_listener) = self.listener.lock().unwrap().replace(handle) {
            old_listener.cancel();
        }
    }

    /// Registers a callback for a specific task.
    ///
    /// Associates a callback implementation with a task ID, allowing the callback to receive
    /// events for that specific task.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to monitor
    /// - `callback`: Callback implementation to receive events
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_core::info::{Progress, Response};
    /// use request_next::listen::observe::{Callback, Observer};
    /// use std::sync::Arc;
    ///
    /// struct SimpleCallback;
    ///
    /// impl Callback for SimpleCallback {
    ///     fn on_progress(&self, progress: &Progress) {
    ///         println!("Progress: {}/{} bytes", progress.download_size, progress.total_size);
    ///     }
    /// }
    ///
    /// let observer = Observer::new();
    /// let task_id = 12345;
    /// let callback = Arc::new(SimpleCallback);
    ///
    /// // Register the callback for the specific task
    /// observer.register_callback(task_id, callback);
    /// ```
    pub fn register_callback(
        &self,
        task_id: i64,
        callback: Arc<dyn Callback + Send + Sync + 'static>,
    ) {
        self.callbacks.lock().unwrap().insert(task_id, callback);
    }

    /// Unregisters a callback for a specific task.
    ///
    /// Removes the callback association for the given task ID, stopping event notifications
    /// for that task.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to stop monitoring
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_next::listen::observe::Observer;
    ///
    /// let observer = Observer::new();
    /// let task_id = 12345;
    ///
    /// // Unregister any callback associated with the task
    /// observer.unregister_callback(task_id);
    /// ```
    pub fn unregister_callback(&self, task_id: i64) {
        self.callbacks.lock().unwrap().remove(&task_id);
    }

    pub fn process_header_receive(notify_data: &mut NotifyData) {
        let mut index = notify_data.progress.index as usize;
        let mut file_path = String::new();
        let mut len = 0;
        let item = RequestClient::get_instance().task_manager.get_by_id(&(notify_data.task_id as i64));

        if item.is_none() {
            error!("Task ID not found");
            return;
        }

        let config = &item.unwrap().config;
        if config.common_data.multipart {
            index = 0;
        }
        len = config.body_file_paths.len();
        if index >= len {
            return;
        }
        file_path = config.body_file_paths[index].clone();

        notify_data.progress.body_bytes = FileManager::read_bytes_from_file(&file_path).unwrap_or_default();
        // Waiting for "complete" to read and delete.
    }
}
