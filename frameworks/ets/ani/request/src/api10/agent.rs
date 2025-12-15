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

//! Agent module for API 10.
//!
//! This module provides functions to manage download tasks in API 10,
//! including task creation, retrieval, removal, and search operations.

// use std::path::PathBuf;

use ani_rs::business_error::BusinessError;
use ani_rs::objects::{AniObject, AniRef};
use ani_rs::AniEnv;
use request_client::client::error::CreateTaskError;
use request_client::RequestClient;
use request_client::check::file::DownloadPathError;
use request_core::config::Version;
use request_core::filter::SearchFilter;
use request_utils::context::Context;
use request_core::config::TaskConfig;

use crate::api10::bridge::{Config, Filter, Task, TaskInfo};
use crate::seq::TaskSeq;
use crate::constant::*;

const TOKEN_MIN_BYTES: usize = 8;
const TOKEN_MAX_BYTES: usize = 2048;

#[ani_rs::native]
pub fn check_tid(id: String) -> Result<(), BusinessError> {
    if id.is_empty() {
        return Err(BusinessError::new(
            ExceptionErrorCode::E_PARAMETER_CHECK as i32,
            "Parameter verification failed, tid is empty".to_string()
        ));
    }
    if id.len() > 32 {
        return Err(BusinessError::new(
            ExceptionErrorCode::E_TASK_NOT_FOUND as i32,
            "task not found error".to_string()
        ));
    }
    Ok(())
}

#[ani_rs::native]
pub fn check_token(token: String) -> Result<(), BusinessError> {
    if token.len() < TOKEN_MIN_BYTES || token.len() > TOKEN_MAX_BYTES {
        return Err(BusinessError::new(
            ExceptionErrorCode::E_PARAMETER_CHECK as i32,
            "Parameter verification failed, the length of token should between 8 and 2048 bytes".to_string()
        ));
    }
    Ok(())
}

#[ani_rs::native]
pub fn check_config(env: &AniEnv, context: AniRef, config: Config) -> Result<i64, BusinessError> {
    let context = AniObject::from(context);
    // Generate a new sequential task ID for tracking
    let seq = TaskSeq::next().0.get();
    info!("Check Config, seq: {}", seq);
    let context = Context::new(env, &context);
    let mut config: TaskConfig = config.into();
    // TODO: CHECK NULLPTR
    config.bundle_type = context.get_bundle_type() as u32;
    config.bundle = context.get_bundle_name();

    match RequestClient::get_instance().check_config(
        context,
        seq,
        config,
    ) {
        Ok(_) => Ok(seq as i64),
        Err(e) => {
            error!("Create task failed: {:?}", e);
            // Handle specific error types and return appropriate business errors
            match e {
                CreateTaskError::DownloadPath(err) => {
                    let (code, message) = match err {
                        DownloadPathError::InvalidPath => (401, "Invalid Path"),
                        _ => (13400001, "Invalid file or file system error.")
                    };
                    Err(BusinessError::new_static(code, message))
                },
                CreateTaskError::Code(code) => {
                    Err(BusinessError::new_static(code, "Create Task Failed"))
                }
            }
        }
    }
}

/// Creates a new download task with the given configuration.
///
/// # Parameters
///
/// * `env` - The animation environment reference
/// * `context` - The application context
/// * `config` - The task configuration containing URL, save path, etc.
///
/// # Returns
///
/// * `Ok(Task)` if the task was successfully created
/// * `Err(BusinessError)` if there was an error during task creation
///
/// # Errors
///
/// Returns an error if:
/// * The download path is invalid (error code 401)
/// * There's a file system error (error code 13400001)
/// * Task creation fails with a specific error code
///
/// # Examples
///
/// ```rust
/// use ani_rs::AniEnv;
/// use ani_rs::objects::AniRef;
/// use request_api10::api10::agent::create;
/// use request_api10::api10::bridge::Config;
///
/// // Assuming env and context are properly initialized
/// let config = Config {
///     url: "https://example.com/file.zip".to_string(),
///     saveas: Some("./downloads/file.zip".to_string()),
///     overwrite: Some(true),
///     // Other configuration fields...
/// };
///
/// match create(&env, context, config) {
///     Ok(task) => println!("Task created with ID: {}", task.tid),
///     Err(e) => println!("Error creating task: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn create(env: &AniEnv, context: AniRef, seq: i64) -> Result<String, BusinessError> {
    let context = AniObject::from(context);
    let context = Context::new(env, &context);

    match RequestClient::get_instance().create_task(
        context,
        seq as u64,
    ) {
        Ok(task_id) => Ok(task_id.to_string()),
        Err(e) => {
            error!("Create task failed: {:?}", e);
            // Handle specific error types and return appropriate business errors
            match e {
                CreateTaskError::DownloadPath(err) => {
                    let (code, message) = match err {
                        DownloadPathError::InvalidPath => (401, "Invalid Path"),
                        _ => (13400001, "Invalid file or file system error.")
                    };
                    Err(BusinessError::new_static(code, message))
                },
                CreateTaskError::Code(code) => {
                    Err(BusinessError::new_static(code, "Create Task Failed"))
                }
            }
        }
    }
}

/// Retrieves a task by its ID and authentication token.
///
/// # Parameters
///
/// * `context` - The application context
/// * `task_id` - The ID of the task to retrieve
/// * `token` - Optional authentication token
///
/// # Returns
///
/// * `Result<Task, BusinessError>` - Unimplemented
///
/// # Panics
///
/// Panics as this function is unimplemented (`todo!()`).
#[ani_rs::native]
pub fn get_task(
    context: AniRef,
    id: String,
    token: Option<String>,
) -> Result<Task, BusinessError> {
    let task_id = id
        .parse::<i64>()
        .map_err(|_| BusinessError::new(ExceptionErrorCode::E_PARAMETER_CHECK as i32,
            "Invalid task ID format".to_string()))?;
    RequestClient::get_instance()
        .get_task(task_id, token)
        .map(|c| Task { tid: id, config: Config::from(c) })
        .map_err(|e| BusinessError::new(e, "Failed to get download task".to_string()))
}

/// Removes a task with the specified ID.
///
/// # Parameters
///
/// * `id` - The ID of the task to remove as a string
///
/// # Returns
///
/// * `Ok(())` if the task was successfully removed
/// * `Err(BusinessError)` if there was an error during removal
///
/// # Errors
///
/// Returns an error if:
/// * The task ID format is invalid
/// * The task removal fails
///
/// # Examples
///
/// ```rust
/// use request_api10::api10::agent::remove;
///
/// match remove("12345".to_string()) {
///     Ok(_) => println!("Task removed successfully"),
///     Err(e) => println!("Error removing task: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn remove(id: String) -> Result<(), BusinessError> {
    let task_id = id
        .parse::<i64>()
        .map_err(|_| BusinessError::new(ExceptionErrorCode::E_TASK_NOT_FOUND as i32,
            "Invalid task ID format".to_string()))?;
    RequestClient::get_instance()
        .remove(task_id)
        .map_err(|e| BusinessError::new_static(e, "Failed to remove task"))
}

/// Shows detailed information about a task with the specified ID.
///
/// # Parameters
///
/// * `id` - The ID of the task to show information for
///
/// # Returns
///
/// * `Ok(TaskInfo)` containing the task details
/// * `Err(BusinessError)` if there was an error retrieving the information
///
/// # Errors
///
/// Returns an error if the task information cannot be retrieved.
///
/// # Examples
///
/// ```rust
/// use request_api10::api10::agent::show;
///
/// match show("12345".to_string()) {
///     Ok(info) => println!("Task info: {:?}", info),
///     Err(e) => println!("Error getting task info: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn show(id: String) -> Result<TaskInfo, BusinessError> {
    // Parse string task ID to integer for internal use
    let task_id = id.parse::<i64>()
        .map_err(|_| BusinessError::new(ExceptionErrorCode::E_PARAMETER_CHECK as i32,
            "Invalid task ID format".to_string()))?;
    RequestClient::get_instance()
        .show_task(task_id)
        .map(|info| {
            info!("Api10 get task info: {:?}", info);
            TaskInfo::from(info)
        })
        .map_err(|e| BusinessError::new(e, "Failed to get download task info".to_string()))
}

/// Touches a task with the specified ID and authentication token.
///
/// Performs an operation to update the task's last access time or status.
///
/// # Parameters
///
/// * `id` - The ID of the task to touch
/// * `token` - Authentication token
///
/// # Returns
///
/// * `Ok(())` unconditionally (placeholder implementation)
#[ani_rs::native]
pub fn touch(id: String, token: String) -> Result<TaskInfo, BusinessError> {
    let task_id = id
        .parse::<i64>()
        .map_err(|_| BusinessError::new(ExceptionErrorCode::E_TASK_NOT_FOUND as i32,
            "task not found error".to_string()))?;
    RequestClient::get_instance()
        .touch(task_id, token)
        .map(|info| {
            info!("Api10 touch task info: {:?}", info);
            TaskInfo::from(info)
        })
        .map_err(|e| BusinessError::new_static(e, "Failed to touch task"))
}

/// Searches for tasks matching the given filter criteria.
///
/// # Parameters
///
/// * `filter` - Optional filter criteria for the search
///
/// # Returns
///
/// * `Ok(Vec<String>)` containing the IDs of matching tasks
/// * `Err(BusinessError)` if there was an error during the search
///
/// # Errors
///
/// Returns an error if the search operation fails.
///
/// # Examples
///
/// ```rust
/// use request_api10::api10::agent::search;
/// use request_api10::api10::bridge::Filter;
///
/// // Search with no filter (find all tasks)
/// match search(None) {
///     Ok(task_ids) => println!("Found {} tasks", task_ids.len()),
///     Err(e) => println!("Error searching tasks: {}", e),
/// }
///
/// // Search with specific filter
/// let filter = Filter {
///     // Set filter criteria
/// };
/// match search(Some(filter)) {
///     Ok(task_ids) => println!("Found {} matching tasks", task_ids.len()),
///     Err(e) => println!("Error searching tasks: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn search(filter: Option<Filter>) -> Result<Vec<String>, BusinessError> {
    // Convert API filter to core filter, or create empty filter if none provided
    let filter = match filter {
        Some(f) => f.into(),
        None => SearchFilter::new(),
    };
    RequestClient::get_instance()
        .search(filter)
        .map(|tasks| {
            info!("Api10 search tasks: {:?}", tasks);
            tasks
        })
        .map_err(|e| BusinessError::new(e, "Failed to search tasks".to_string()))
}

/// Queries a task with the specified ID.
///
/// # Parameters
///
/// * `id` - The ID of the task to query
///
/// # Returns
///
/// * `Result<TaskInfo, BusinessError>` - Unimplemented
///
/// # Panics
///
/// Panics as this function is unimplemented (`todo!()`).
#[ani_rs::native]
pub fn query(id: String) -> Result<TaskInfo, BusinessError> {
    let task_id = id
        .parse::<i64>()
        .map_err(|_| BusinessError::new(ExceptionErrorCode::E_PARAMETER_CHECK as i32,
            "Invalid task ID format".to_string()))?;
    RequestClient::get_instance()
        .query(task_id)
        .map(|info| {
            info!("Api10 query task info: {:?}", info);
            TaskInfo::from(info)
        })
        .map_err(|e| BusinessError::new_static(e, "Failed to query task"))
}
