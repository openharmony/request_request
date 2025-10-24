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

//! Callback module for API 9 download task events.
//! 
//! This module provides functions to register and manage callbacks for download task events,
//! including progress updates, completion, failure, pause, and resume events. It implements
//! a thread-safe callback management system using a singleton pattern.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use ani_rs::business_error::BusinessError;
use ani_rs::objects::{AniFnObject, GlobalRefCallback};
use ani_rs::AniEnv;
use request_client::RequestClient;
use request_core::info::Progress;

use crate::api9::bridge::DownloadTask;

/// Registers a progress callback for a download task.
///
/// Adds a callback function that will be invoked when progress updates are available for
/// the specified download task.
///
/// # Parameters
///
/// * `env` - The animation environment reference
/// * `this` - The download task to monitor
/// * `callback` - The callback function to execute on progress updates
///
/// # Returns
///
/// * `Ok(())` if the callback was successfully registered
/// * `Err(BusinessError)` if there was an error during registration
///
/// # Errors
///
/// Returns an error if the callback conversion fails.
///
/// # Examples
///
/// ```rust
/// use ani_rs::objects::AniFnObject;
/// use ani_rs::AniEnv;
/// use request_api9::callback::on_progress;
/// use request_api9::bridge::DownloadTask;
///
/// // Assuming env and callback_fn are properly initialized
/// let task = DownloadTask { task_id: 123 };
/// let result = on_progress(&env, task, callback_fn);
/// ```
#[ani_rs::native]
pub fn on_progress(
    env: &AniEnv,
    this: DownloadTask,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    info!("on_progress called for task_id: {}", this.task_id);
    let callback_mgr = CallbackManager::get_instance();
    // Convert to global reference to ensure callback persists across function calls
    let callback = callback.into_global_callback(env).unwrap();
    
    // Add callback to existing task or create new task entry
    if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&this.task_id) {
        coll.on_progress.lock().unwrap().push(callback);
    } else {
        // Create new callback collection with progress callback
        let coll = Arc::new(CallbackColl {
            on_progress: Mutex::new(vec![callback]),
            on_complete: Mutex::new(vec![]),
            on_pause: Mutex::new(vec![]),
            on_resume: Mutex::new(vec![]),
            on_fail: Mutex::new(vec![]),
        });
        // Register with RequestClient to receive events
        RequestClient::get_instance().register_callback(this.task_id, coll.clone());
        // Store in manager for future callback additions
        callback_mgr
            .tasks
            .lock()
            .unwrap()
            .insert(this.task_id, coll);
    }
    Ok(())
}

/// Registers an event callback for a download task.
///
/// Adds a callback function that will be invoked when a specific event occurs for
/// the specified download task.
///
/// # Parameters
///
/// * `env` - The animation environment reference
/// * `this` - The download task to monitor
/// * `event` - The event type ("complete", "pause", "resume")
/// * `callback` - The callback function to execute when the event occurs
///
/// # Returns
///
/// * `Ok(())` if the callback was successfully registered
/// * `Err(BusinessError)` if there was an error during registration or the event type is unsupported
///
/// # Errors
///
/// Returns an error if:
/// * The callback conversion fails
/// * The event type is not one of the supported values ("complete", "pause", "resume")
///
/// # Examples
///
/// ```rust
/// use ani_rs::objects::AniFnObject;
/// use ani_rs::AniEnv;
/// use request_api9::callback::on_event;
/// use request_api9::bridge::DownloadTask;
///
/// // Assuming env and callback_fn are properly initialized
/// let task = DownloadTask { task_id: 123 };
/// let result = on_event(&env, task, "complete".to_string(), callback_fn);
/// ```
#[ani_rs::native]
pub fn on_event(
    env: &AniEnv,
    this: DownloadTask,
    event: String,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_mgr = CallbackManager::get_instance();
    // Convert to global reference to ensure callback persists across function calls
    let callback = callback.into_global_callback(env).unwrap();
    info!(
        "on_event called for task_id: {}, event: {}",
        this.task_id, event
    );
    
    // Handle different event types
    let coll = if event == "complete" {
        // Add to existing task or create new with complete callback
        if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&this.task_id) {
            coll.on_complete.lock().unwrap().push(callback);
            return Ok(());
        } else {
            Arc::new(CallbackColl {
                on_progress: Mutex::new(vec![]),
                on_complete: Mutex::new(vec![callback]),
                on_pause: Mutex::new(vec![]),
                on_resume: Mutex::new(vec![]),
                on_fail: Mutex::new(vec![]),
            })
        }
    } else if event == "pause" {
        // Add to existing task or create new with pause callback
        if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&this.task_id) {
            coll.on_pause.lock().unwrap().push(callback);
            return Ok(());
        } else {
            Arc::new(CallbackColl {
                on_progress: Mutex::new(vec![]),
                on_complete: Mutex::new(vec![]),
                on_pause: Mutex::new(vec![callback]),
                on_resume: Mutex::new(vec![]),
                on_fail: Mutex::new(vec![]),
            })
        }
    } else if event == "resume" {
        // Add to existing task or create new with resume callback
        if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&this.task_id) {
            coll.on_resume.lock().unwrap().push(callback);
            return Ok(());
        } else {
            Arc::new(CallbackColl {
                on_progress: Mutex::new(vec![]),
                on_complete: Mutex::new(vec![]),
                on_pause: Mutex::new(vec![]),
                on_resume: Mutex::new(vec![callback]),
                on_fail: Mutex::new(vec![]),
            })
        }
    } else {
        // Return error for unsupported event types
        return Err(BusinessError::new(
            -1,
            format!("Unsupported event type: {}", event),
        ));
    };
    
    // Register with RequestClient to receive events and store in manager
    RequestClient::get_instance().register_callback(this.task_id, coll.clone());
    callback_mgr
        .tasks
        .lock()
        .unwrap()
        .insert(this.task_id, coll);
    Ok(())
}

/// Registers a failure callback for a download task.
///
/// Adds a callback function that will be invoked when the download task fails.
///
/// # Parameters
///
/// * `env` - The animation environment reference
/// * `this` - The download task to monitor
/// * `callback` - The callback function to execute on task failure
///
/// # Returns
///
/// * `Ok(())` if the callback was successfully registered
/// * `Err(BusinessError)` if there was an error during registration
///
/// # Errors
///
/// Returns an error if the callback conversion fails.
///
/// # Examples
///
/// ```rust
/// use ani_rs::objects::AniFnObject;
/// use ani_rs::AniEnv;
/// use request_api9::callback::on_fail;
/// use request_api9::bridge::DownloadTask;
///
/// // Assuming env and callback_fn are properly initialized
/// let task = DownloadTask { task_id: 123 };
/// let result = on_fail(&env, task, callback_fn);
/// ```
#[ani_rs::native]
pub fn on_fail(
    env: &AniEnv,
    this: DownloadTask,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_mgr = CallbackManager::get_instance();
    // Convert to global reference to ensure callback persists across function calls
    let callback = callback.into_global_callback(env).unwrap();
    
    // Add callback to existing task or create new task entry
    if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&this.task_id) {
        coll.on_fail.lock().unwrap().push(callback);
    } else {
        // Create new callback collection with fail callback
        let coll = Arc::new(CallbackColl {
            on_progress: Mutex::new(vec![]),
            on_complete: Mutex::new(vec![]),
            on_pause: Mutex::new(vec![]),
            on_resume: Mutex::new(vec![]),
            on_fail: Mutex::new(vec![callback]),
        });
        // Register with RequestClient to receive events
        RequestClient::get_instance().register_callback(this.task_id, coll.clone());

        // Store in manager for future callback additions
        callback_mgr
            .tasks
            .lock()
            .unwrap()
            .insert(this.task_id, coll);
    }
    Ok(())
}

/// Collection of callbacks for different download task events.
///
/// Stores and manages different types of callbacks for a download task, ensuring thread
/// safety for concurrent access.
pub struct CallbackColl {
    /// Callbacks triggered when download progress updates.
    on_progress: Mutex<Vec<GlobalRefCallback<(i64, i64)>>>,
    /// Callbacks triggered when download completes successfully.
    on_complete: Mutex<Vec<GlobalRefCallback<()>>>,
    /// Callbacks triggered when download is paused.
    on_pause: Mutex<Vec<GlobalRefCallback<()>>>,
    /// Callbacks triggered when download is resumed.
    on_resume: Mutex<Vec<GlobalRefCallback<()>>>,
    /// Callbacks triggered when download fails.
    on_fail: Mutex<Vec<GlobalRefCallback<(i32,)>>>,
}

/// Implements the `request_client::Callback` trait for `CallbackColl`.
///
/// Provides methods to handle different download events and execute the corresponding
/// registered callbacks with appropriate parameters.
impl request_client::Callback for CallbackColl {
    /// Handles progress update events.
    ///
    /// Executes all registered progress callbacks with the current progress data.
    ///
    /// # Parameters
    ///
    /// * `progress` - The progress information containing processed bytes and total size
    fn on_progress(&self, progress: &Progress) {
        // Lock the callback vector to prevent concurrent modifications
        let callbacks = self.on_progress.lock().unwrap();
        // Execute each callback with processed bytes and total size
        for callback in callbacks.iter() {
            callback.execute((progress.processed as i64, progress.sizes[0]));
        }
    }

    /// Handles download completion events.
    ///
    /// Executes all registered completion callbacks.
    ///
    /// # Parameters
    ///
    /// * `_progress` - The final progress information (unused in this implementation)
    fn on_completed(&self, _progress: &Progress) {
        // Lock the callback vector to prevent concurrent modifications
        let callbacks = self.on_complete.lock().unwrap();
        // Execute each callback with no parameters
        for callback in callbacks.iter() {
            callback.execute(());
        }
    }

    /// Handles download failure events.
    ///
    /// Executes all registered failure callbacks with the error code.
    ///
    /// # Parameters
    ///
    /// * `_progress` - The progress information at time of failure (unused in this implementation)
    /// * `error_code` - The error code indicating the reason for failure
    fn on_failed(&self, _progress: &Progress, error_code: i32) {
        // Lock the callback vector to prevent concurrent modifications
        let callbacks = self.on_fail.lock().unwrap();
        // Execute each callback with the error code
        for callback in callbacks.iter() {
            callback.execute((error_code,));
        }
    }

    /// Handles download pause events.
    ///
    /// Executes all registered pause callbacks.
    ///
    /// # Parameters
    ///
    /// * `_progress` - The progress information at time of pause (unused in this implementation)
    fn on_pause(&self, _progress: &Progress) {
        // Lock the callback vector to prevent concurrent modifications
        let callbacks = self.on_pause.lock().unwrap();
        // Execute each callback with no parameters
        for callback in callbacks.iter() {
            callback.execute(());
        }
    }

    /// Handles download resume events.
    ///
    /// Executes all registered resume callbacks.
    ///
    /// # Parameters
    ///
    /// * `_progress` - The progress information at time of resume (unused in this implementation)
    fn on_resume(&self, _progress: &Progress) {
        // Lock the callback vector to prevent concurrent modifications
        let callbacks = self.on_resume.lock().unwrap();
        // Execute each callback with no parameters
        for callback in callbacks.iter() {
            callback.execute(());
        }
    }
}

/// Manages callbacks for all active download tasks.
///
/// Implements a singleton pattern to maintain a central registry of callback collections
/// indexed by task IDs.
pub struct CallbackManager {
    /// Map of task IDs to their callback collections.
    tasks: Mutex<HashMap<i64, Arc<CallbackColl>>>,
}

impl CallbackManager {
    /// Returns a reference to the singleton instance of `CallbackManager`.
    ///
    /// Creates the instance if it doesn't already exist.
    ///
    /// # Returns
    ///
    /// A static reference to the `CallbackManager` instance.
    pub fn get_instance() -> &'static Self {
        // Create static instance with OnceLock to ensure thread-safe initialization
        static INSTANCE: OnceLock<CallbackManager> = OnceLock::new();

        INSTANCE.get_or_init(|| CallbackManager {
            tasks: Mutex::new(HashMap::new()),
        })
    }
}
