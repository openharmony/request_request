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

//! Callback module for API 10.
//!
//! This module provides callback registration and management functionality for request tasks
//! in API 10, allowing clients to receive notifications about task events such as progress,
//! completion, failure, and more.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use ani_rs::objects::{AniFnObject, GlobalRefCallback};
use ani_rs::AniEnv;
use request_client::RequestClient;
use request_core::info::{Progress, Response, Faults};

use crate::api10::bridge::{self, Task};

/// Registers a callback for a specific task event.
///
/// # Parameters
///
/// * `env` - The animation environment reference
/// * `this` - The task to register the callback for
/// * `event` - The event name to listen for ("completed", "pause", "failed", "remove", "progress", "resume")
/// * `callback` - The callback function to execute when the event occurs
///
/// # Returns
///
/// * `Ok(())` if the callback was successfully registered
/// * `Err(BusinessError)` if there was an error during callback registration
///
/// # Examples
///
/// ```rust
/// use ani_rs::AniEnv;
/// use ani_rs::objects::AniFnObject;
/// use request_api10::api10::bridge::Task;
/// use request_api10::api10::callback::on_event;
///
/// // Assuming env, task, and callback are properly initialized
/// match on_event(&env, task, "progress".to_string(), callback) {
///     Ok(_) => println!("Progress callback registered successfully"),
///     Err(e) => println!("Failed to register progress callback: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn on_event(
    env: &AniEnv,
    this: Task,
    event: String,
    callback: AniFnObject,
) -> Result<(), ani_rs::business_error::BusinessError> {
    // Convert task ID from string to integer for internal use
    let task_id = this.tid.parse().unwrap();
    info!("on_event called with event: {}", event);
    let callback_mgr = CallbackManager::get_instance();
    let callback = callback.into_global_callback(env).unwrap();

    // Determine which callback collection to use based on event type
    let coll = match event.as_str() {
        "completed" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                // Add to existing callback collection if it exists
                coll.on_complete.lock().unwrap().push(callback);
                return Ok(());
            } else {
                // Create new callback collection if none exists
                Arc::new(CallbackColl {
                    on_progress: Mutex::new(vec![]),
                    on_complete: Mutex::new(vec![callback]),
                    on_pause: Mutex::new(vec![]),
                    on_resume: Mutex::new(vec![]),
                    on_remove: Mutex::new(vec![]),
                    on_fail: Mutex::new(vec![]),
                    on_response: Mutex::new(vec![]),
                    on_fault: Mutex::new(vec![]),
                })
            }
        }
        "pause" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_pause.lock().unwrap().push(callback);
                return Ok(());
            } else {
                Arc::new(CallbackColl {
                    on_progress: Mutex::new(vec![]),
                    on_complete: Mutex::new(vec![]),
                    on_pause: Mutex::new(vec![callback]),
                    on_resume: Mutex::new(vec![]),
                    on_remove: Mutex::new(vec![]),
                    on_fail: Mutex::new(vec![]),
                    on_response: Mutex::new(vec![]),
                    on_fault: Mutex::new(vec![]),
                })
            }
        }
        "failed" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_fail.lock().unwrap().push(callback);
                return Ok(());
            } else {
                Arc::new(CallbackColl {
                    on_progress: Mutex::new(vec![]),
                    on_complete: Mutex::new(vec![]),
                    on_pause: Mutex::new(vec![]),
                    on_resume: Mutex::new(vec![]),
                    on_remove: Mutex::new(vec![]),
                    on_fail: Mutex::new(vec![callback]),
                    on_response: Mutex::new(vec![]),
                    on_fault: Mutex::new(vec![]),
                })
            }
        }
        "remove" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_remove.lock().unwrap().push(callback);
                return Ok(());
            } else {
                Arc::new(CallbackColl {
                    on_progress: Mutex::new(vec![]),
                    on_complete: Mutex::new(vec![]),
                    on_pause: Mutex::new(vec![]),
                    on_resume: Mutex::new(vec![]),
                    on_remove: Mutex::new(vec![callback]),
                    on_fail: Mutex::new(vec![]),
                    on_response: Mutex::new(vec![]),
                    on_fault: Mutex::new(vec![]),
                })
            }
        }
        "progress" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_progress.lock().unwrap().push(callback);
                return Ok(());
            } else {
                Arc::new(CallbackColl {
                    on_progress: Mutex::new(vec![callback]),
                    on_complete: Mutex::new(vec![]),
                    on_pause: Mutex::new(vec![]),
                    on_resume: Mutex::new(vec![]),
                    on_remove: Mutex::new(vec![]),
                    on_fail: Mutex::new(vec![]),
                    on_response: Mutex::new(vec![]),
                    on_fault: Mutex::new(vec![]),
                })
            }
        }
        "resume" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_resume.lock().unwrap().push(callback);
                return Ok(());
            } else {
                Arc::new(CallbackColl {
                    on_progress: Mutex::new(vec![]),
                    on_complete: Mutex::new(vec![]),
                    on_pause: Mutex::new(vec![]),
                    on_resume: Mutex::new(vec![callback]),
                    on_remove: Mutex::new(vec![]),
                    on_fail: Mutex::new(vec![]),
                    on_response: Mutex::new(vec![]),
                    on_fault: Mutex::new(vec![]),
                })
            }
        }
        // Handle unknown event types
        _ => unimplemented!()
    };

    // Register callback with request client and add to manager
    RequestClient::get_instance().register_callback(task_id, coll.clone());
    callback_mgr.tasks.lock().unwrap().insert(task_id, coll);
    Ok(())
}

/// Registers a callback for HTTP response events.
///
/// # Parameters
///
/// * `env` - The animation environment reference
/// * `this` - The task to register the callback for
/// * `event` - The event name to listen for (only "response" is supported)
/// * `callback` - The callback function to execute when the response is received
///
/// # Returns
///
/// * `Ok(())` if the callback was successfully registered
/// * `Err(BusinessError)` if there was an error during callback registration
///
/// # Examples
///
/// ```rust
/// use ani_rs::AniEnv;
/// use ani_rs::objects::AniFnObject;
/// use request_api10::api10::bridge::Task;
/// use request_api10::api10::callback::on_response_event;
///
/// // Assuming env, task, and callback are properly initialized
/// match on_response_event(&env, task, "response".to_string(), callback) {
///     Ok(_) => println!("Response callback registered successfully"),
///     Err(e) => println!("Failed to register response callback: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn on_response_event(
    env: &AniEnv,
    this: Task,
    event: String,
    callback: AniFnObject,
) -> Result<(), ani_rs::business_error::BusinessError> {
    // Convert task ID from string to integer for internal use
    let task_id = this.tid.parse().unwrap();
    info!("on_event called with event: {}", event);
    let callback_mgr = CallbackManager::get_instance();
    let callback = callback.into_global_callback(env).unwrap();

    // Handle response event type
    let coll = match event.as_str() {
        "response" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                // Add to existing callback collection if it exists
                coll.on_response.lock().unwrap().push(callback);
                return Ok(());
            } else {
                // Create new callback collection if none exists
                Arc::new(CallbackColl {
                    on_progress: Mutex::new(vec![]),
                    on_complete: Mutex::new(vec![]),
                    on_pause: Mutex::new(vec![]),
                    on_resume: Mutex::new(vec![]),
                    on_remove: Mutex::new(vec![]),
                    on_fail: Mutex::new(vec![]),
                    on_response: Mutex::new(vec![callback]),
                    on_fault: Mutex::new(vec![]),
                })
            }
        }
        // Handle unknown event types
        _ => unimplemented!()
    };
    RequestClient::get_instance().register_callback(task_id, coll.clone());
    callback_mgr.tasks.lock().unwrap().insert(task_id, coll);
    Ok(())
}

#[ani_rs::native]
pub fn on_fault_event(
    env: &AniEnv,
    this: Task,
    event: String,
    callback: AniFnObject,
) -> Result<(), ani_rs::business_error::BusinessError> {
    let task_id = this.tid.parse().unwrap();
    info!("on_fault_event called with event: {}", event);
    let callback_mgr = CallbackManager::get_instance();
    let callback = callback.into_global_callback(env).unwrap();
    let coll = match event.as_str() {
        "faultOccur" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_fault.lock().unwrap().push(callback);
                return Ok(());
            } else {
                Arc::new(CallbackColl {
                    on_progress: Mutex::new(vec![]),
                    on_complete: Mutex::new(vec![]),
                    on_pause: Mutex::new(vec![]),
                    on_resume: Mutex::new(vec![]),
                    on_remove: Mutex::new(vec![]),
                    on_fail: Mutex::new(vec![]),
                    on_response: Mutex::new(vec![]),
                    on_fault: Mutex::new(vec![callback]),
                })
            }
        }
        _ => unimplemented!()
    };

    // Register callback with request client and add to manager
    RequestClient::get_instance().register_callback(task_id, coll.clone());
    callback_mgr.tasks.lock().unwrap().insert(task_id, coll);
    Ok(())
}

#[ani_rs::native]
pub fn off_event(
    env: &AniEnv,
    this: Task,
    event: String,
    callback: AniFnObject,
) -> Result<(), ani_rs::business_error::BusinessError> {
    let task_id = this.tid.parse().unwrap();
    info!("off_event called with event: {}", event);
    let callback_mgr = CallbackManager::get_instance();
    let callback: GlobalRefCallback<(bridge::Progress,)> = callback.into_global_callback(env).unwrap();
    match event.as_str() {
        "completed" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_complete.lock().unwrap().retain(|x| *x != callback);
            }
        }
        "pause" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_pause.lock().unwrap().retain(|x| *x != callback);
            }
        }
        "failed" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_fail.lock().unwrap().retain(|x| *x != callback);
            }
        }
        "remove" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_remove.lock().unwrap().retain(|x| *x != callback);
            }
        }
        "progress" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_progress.lock().unwrap().retain(|x| *x != callback);
            }
        }
        "resume" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_resume.lock().unwrap().retain(|x| *x != callback);
            }
        }
        _ => unimplemented!()
    };
    Ok(())
}

#[ani_rs::native]
pub fn off_response_event(
    env: &AniEnv,
    this: Task,
    event: String,
    callback: AniFnObject,
) -> Result<(), ani_rs::business_error::BusinessError> {
    let task_id = this.tid.parse().unwrap();
    info!("off_response_event called with event: {}", event);
    let callback_mgr = CallbackManager::get_instance();
    let callback = callback.into_global_callback(env).unwrap();
    match event.as_str() {
        "response" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_response.lock().unwrap().retain(|x| *x != callback);
            }
        }
        _ => unimplemented!()
    };
    Ok(())
}

#[ani_rs::native]
pub fn off_fault_event(
    env: &AniEnv,
    this: Task,
    event: String,
    callback: AniFnObject,
) -> Result<(), ani_rs::business_error::BusinessError> {
    let task_id = this.tid.parse().unwrap();
    info!("off_fault_event called with event: {}", event);
    let callback_mgr = CallbackManager::get_instance();
    let callback = callback.into_global_callback(env).unwrap();
    match event.as_str() {
        "faultOccur" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_fault.lock().unwrap().retain(|x| *x != callback);
            }
        }
        _ => unimplemented!()
    };
    Ok(())
}

#[ani_rs::native]
pub fn off_events(
    env: &AniEnv,
    this: Task,
    event: String,
) -> Result<(), ani_rs::business_error::BusinessError> {
    let task_id = this.tid.parse().unwrap();
    info!("off_fault_event called with event: {}", event);
    let callback_mgr = CallbackManager::get_instance();
    match event.as_str() {
        "completed" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_complete.lock().unwrap().clear();
            }
        }
        "pause" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_pause.lock().unwrap().clear();
            }
        }
        "failed" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_fail.lock().unwrap().clear();
            }
        }
        "remove" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_remove.lock().unwrap().clear();
            }
        }
        "progress" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_progress.lock().unwrap().clear();
            }
        }
        "resume" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_resume.lock().unwrap().clear();
            }
        }
        "faultOccur" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_fault.lock().unwrap().clear();
            }
        }
        "response" => {
            if let Some(coll) = callback_mgr.tasks.lock().unwrap().get(&task_id) {
                coll.on_response.lock().unwrap().clear();
            }
        }
        _ => unimplemented!()
    };
    Ok(())
}

pub struct CallbackColl {
    /// Callbacks to be executed on progress updates.
    on_progress: Mutex<Vec<GlobalRefCallback<(bridge::Progress,)>>>,
    /// Callbacks to be executed on task completion.
    on_complete: Mutex<Vec<GlobalRefCallback<(bridge::Progress,)>>>,
    /// Callbacks to be executed when task is paused.
    on_pause: Mutex<Vec<GlobalRefCallback<(bridge::Progress,)>>>,
    /// Callbacks to be executed when task is resumed.
    on_resume: Mutex<Vec<GlobalRefCallback<(bridge::Progress,)>>>,
    /// Callbacks to be executed when task is removed.
    on_remove: Mutex<Vec<GlobalRefCallback<(bridge::Progress,)>>>,
    /// Callbacks to be executed when task fails.
    on_fail: Mutex<Vec<GlobalRefCallback<(bridge::Progress,)>>>,
    /// Callbacks to be executed when HTTP response is received.
    on_response: Mutex<Vec<GlobalRefCallback<(bridge::HttpResponse,)>>>,
    on_fault: Mutex<Vec<GlobalRefCallback<(bridge::Faults,)>>>,
}

impl request_client::Callback for CallbackColl {
    /// Executes all registered progress callbacks with the current progress information.
    ///
    /// # Parameters
    ///
    /// * `progress` - The current progress information of the task
    fn on_progress(&self, progress: &Progress) {
        // Lock the callbacks vector to prevent concurrent modifications during execution
        let callbacks = self.on_progress.lock().unwrap();
        for callback in callbacks.iter() {
            // Execute each callback with converted progress data
            callback.execute((progress.into(),));
        }
    }

    /// Executes all registered completion callbacks when a task completes.
    ///
    /// # Parameters
    ///
    /// * `progress` - The final progress information of the completed task
    fn on_completed(&self, progress: &Progress) {
        let callbacks = self.on_complete.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute((progress.into(),));
        }
    }

    /// Executes all registered pause callbacks when a task is paused.
    ///
    /// # Parameters
    ///
    /// * `progress` - The progress information at the time of pausing
    fn on_pause(&self, progress: &Progress) {
        let callbacks = self.on_pause.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute((progress.into(),));
        }
    }

    /// Executes all registered resume callbacks when a task is resumed.
    ///
    /// # Parameters
    ///
    /// * `progress` - The progress information at the time of resuming
    fn on_resume(&self, progress: &Progress) {
        let callbacks = self.on_resume.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute((progress.into(),));
        }
    }

    /// Executes all registered remove callbacks when a task is removed.
    ///
    /// # Parameters
    ///
    /// * `progress` - The progress information at the time of removal
    fn on_remove(&self, progress: &Progress) {
        let callbacks = self.on_remove.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute((progress.into(),));
        }
    }

    /// Executes all registered response callbacks when an HTTP response is received.
    ///
    /// # Parameters
    ///
    /// * `response` - The HTTP response information
    fn on_response(&self, response: &Response) {
        let callbacks = self.on_response.lock().unwrap();
        for callback in callbacks.iter() {
            // Execute each callback with converted response data
            callback.execute((response.into(),));
        }
    }

    /// Executes all registered failure callbacks when a task fails.
    ///
    /// # Parameters
    ///
    /// * `progress` - The progress information at the time of failure
    /// * `_error_code` - The error code associated with the failure (currently unused)
    fn on_failed(&self, progress: &Progress, _error_code: i32) {
        let callbacks = self.on_fail.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute((progress.into(),));
        }
    }

    fn on_fault(&self, faults: Faults) {
        let callbacks = self.on_fault.lock().unwrap();
        for callback in callbacks.iter() {
            callback.execute((faults.into(),));
        }
    }
}

/// Manages callbacks for request tasks using a singleton pattern.
///
/// Maintains a collection of callback sets indexed by task ID to facilitate
/// event notification to registered callbacks.
pub struct CallbackManager {
    /// Map of task IDs to their corresponding callback collections.
    tasks: Mutex<HashMap<i64, Arc<CallbackColl>>>,
}

impl CallbackManager {
    /// Retrieves the singleton instance of the CallbackManager.
    ///
    /// # Returns
    ///
    /// A static reference to the CallbackManager instance
    ///
    /// # Notes
    ///
    /// Uses OnceLock to ensure thread-safe initialization of the singleton
    pub fn get_instance() -> &'static Self {
        // Static instance initialized only once
        static INSTANCE: OnceLock<CallbackManager> = OnceLock::new();

        // Create new instance if it doesn't exist, otherwise return the existing one
        INSTANCE.get_or_init(|| CallbackManager {
            tasks: Mutex::new(HashMap::new()),
        })
    }
}
