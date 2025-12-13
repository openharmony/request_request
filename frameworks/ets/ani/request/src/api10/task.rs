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

//! Task management module for API 10.
//! 
//! This module provides functions for controlling request tasks in API 10,
//! including operations like starting, pausing, resuming, stopping tasks,
//! and setting speed limits.

use ani_rs::business_error::BusinessError;
use request_client::RequestClient;

use crate::api10::bridge::Task;
use crate::constant::*;

const MIN_SPEED_LIMIT: i64 = 16 * 1024;

/// Starts a request task.
///
/// # Parameters
///
/// * `this` - The task to start
///
/// # Returns
///
/// * `Ok(())` if the task started successfully
/// * `Err(BusinessError)` if the task failed to start
///
/// # Examples
///
/// ```rust
/// use request_api10::api10::task::start;
/// use request_api10::api10::bridge::Task;
/// 
/// // Assuming task is properly initialized
/// match start(task) {
///     Ok(_) => println!("Task started successfully"),
///     Err(e) => println!("Failed to start task: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn start(this: Task) -> Result<(), BusinessError> {
    // Convert task ID from string to integer for internal use
    let task_id = this.tid.parse().unwrap();
    RequestClient::get_instance()
        .start(task_id)
        .map_err(|e| BusinessError::new_static(e, "Failed to start task"))
}

/// Pauses a running request task.
///
/// # Parameters
///
/// * `this` - The task to pause
///
/// # Returns
///
/// * `Ok(())` if the task paused successfully
/// * `Err(BusinessError)` if the task failed to pause
///
/// # Examples
///
/// ```rust
/// use request_api10::api10::task::pause;
/// use request_api10::api10::bridge::Task;
/// 
/// // Assuming task is properly initialized
/// match pause(task) {
///     Ok(_) => println!("Task paused successfully"),
///     Err(e) => println!("Failed to pause task: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn pause(this: Task) -> Result<(), BusinessError> {
    // Convert task ID from string to integer for internal use
    let task_id = this.tid.parse().unwrap();
    RequestClient::get_instance()
        .pause(task_id)
        .map_err(|e| BusinessError::new_static(e, "Failed to pause task"))
}

/// Resumes a paused request task.
///
/// # Parameters
///
/// * `this` - The task to resume
///
/// # Returns
///
/// * `Ok(())` if the task resumed successfully
/// * `Err(BusinessError)` if the task failed to resume
///
/// # Examples
///
/// ```rust
/// use request_api10::api10::task::resume;
/// use request_api10::api10::bridge::Task;
/// 
/// // Assuming task is properly initialized
/// match resume(task) {
///     Ok(_) => println!("Task resumed successfully"),
///     Err(e) => println!("Failed to resume task: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn resume(this: Task) -> Result<(), BusinessError> {
    // Convert task ID from string to integer for internal use
    let task_id = this.tid.parse().unwrap();
    RequestClient::get_instance()
        .resume(task_id)
        .map_err(|e| BusinessError::new_static(e, "Failed to resume task"))
}

/// Stops a running request task.
///
/// # Parameters
///
/// * `this` - The task to stop
///
/// # Returns
///
/// * `Ok(())` if the task stopped successfully
/// * `Err(BusinessError)` if the task failed to stop
///
/// # Examples
///
/// ```rust
/// use request_api10::api10::task::stop;
/// use request_api10::api10::bridge::Task;
/// 
/// // Assuming task is properly initialized
/// match stop(task) {
///     Ok(_) => println!("Task stopped successfully"),
///     Err(e) => println!("Failed to stop task: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn stop(this: Task) -> Result<(), BusinessError> {
    // Convert task ID from string to integer for internal use
    let task_id = this.tid.parse().unwrap();
    RequestClient::get_instance()
        .stop(task_id)
        .map_err(|e| BusinessError::new_static(e, "Failed to stop task"))
}

/// Sets the maximum speed limit for a request task.
///
/// # Parameters
///
/// * `this` - The task to set speed limit for
/// * `speed` - The maximum speed in bytes per second
///
/// # Returns
///
/// * `Ok(())` if the speed limit was set successfully
/// * `Err(BusinessError)` if the speed limit failed to set
///
/// # Examples
///
/// ```rust
/// use request_api10::api10::task::set_max_speed;
/// use request_api10::api10::bridge::Task;
/// 
/// // Assuming task is properly initialized
/// // Set max speed to 1MB per second
/// match set_max_speed(task, 1_048_576) {
///     Ok(_) => println!("Speed limit set successfully"),
///     Err(e) => println!("Failed to set speed limit: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn set_max_speed(this: Task, speed: i64) -> Result<(), BusinessError> {
    if (speed < MIN_SPEED_LIMIT) {
        return Err(BusinessError::new(
            ExceptionErrorCode::E_PARAMETER_CHECK as i32,
            "Incorrect parameter value, minimum speed value is 16 KB/s".to_string()
        ));
    }
    // Convert task ID from string to integer for internal use
    let task_id = this.tid.parse().unwrap();
    RequestClient::get_instance()
        .set_max_speed(task_id, speed)
        .map_err(|e| BusinessError::new_static(e, "Failed to set task max speed"))
}
