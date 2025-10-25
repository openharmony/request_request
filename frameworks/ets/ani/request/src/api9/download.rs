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

#![allow(unused)]

//! Download module for API 9.
//! 
//! This module provides functions to manage download tasks in API 9, including
//! creating, starting, pausing, resuming, and deleting download tasks, as well as
//! retrieving task information.

use std::path::PathBuf;

use ani_rs::business_error::BusinessError;
use ani_rs::objects::{AniObject, AniRef};
use ani_rs::AniEnv;
use request_client::RequestClient;
use request_core::config::Version;
use request_core::info::TaskInfo;
use request_utils::context::{is_stage_context, Context};

use super::bridge::{DownloadConfig, DownloadTask};
use crate::api9::bridge::DownloadInfo;
use crate::seq::TaskSeq;

/// Creates and starts a download task with the given configuration.
///
/// # Parameters
///
/// * `env` - The animation environment reference
/// * `context` - The application context
/// * `config` - The download configuration containing URL, file path, etc.
///
/// # Returns
///
/// * `Ok(DownloadTask)` if the task was successfully created and started
/// * `Err(BusinessError)` if there was an error during task creation or start
///
/// # Errors
///
/// Returns an error if:
/// * Task creation fails
/// * Task start fails
///
/// # Examples
///
/// ```rust
/// use ani_rs::AniEnv;
/// use ani_rs::objects::AniRef;
/// use request_api9::api9::download::download_file;
/// use request_api9::api9::bridge::DownloadConfig;
///
/// // Assuming env and context are properly initialized
/// let config = DownloadConfig {
///     url: "https://example.com/file.zip".to_string(),
///     file_path: Some("./downloads/file.zip".to_string()),
///     // Other configuration fields...
/// };
/// 
/// match download_file(&env, context, config) {
///     Ok(task) => println!("Download started with task ID: {}", task.task_id),
///     Err(e) => println!("Error starting download: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn download_file(
    env: &AniEnv,
    context: AniRef,
    config: DownloadConfig,
) -> Result<DownloadTask, BusinessError> {
    let context = AniObject::from(context);
    info!("is {}", is_stage_context(env, &context));

    // Generate a new sequential task ID for tracking
    let seq = TaskSeq::next();
    info!("Api9 task, seq: {}", seq.0);
    let context = Context::new(env, &context);

    // Determine the save path based on config or URL
    let save_as = match &config.file_path {
        // Use specified path if it exists and is not just a directory marker
        Some(path) if path != "./" => path.to_string(),
        _ => {
            // Extract filename from URL if no path specified
            let name = PathBuf::from(&config.url);
            name.file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or(config.url.clone())
        }
    };

    // Create the download task
    let task = match RequestClient::get_instance().crate_task(
        context,
        Version::API9,
        config.into(),
        &save_as,
        false,
    ) {
        Ok(task_id) => DownloadTask { task_id },
        Err(e) => {
            return Err(BusinessError::new(-1, format!("Download failed")));
        }
    };

    // Start the download task
    match RequestClient::get_instance().start(task.task_id) {
        Ok(_) => {
            info!("Api9 download started successfully, seq: {}", seq.0);
            Ok(task)
        }
        Err(e) => {
            error!("Api9 download start failed, error: {}", e);
            Err(BusinessError::new(
                e,
                format!("Download start failed with error code: {}", e),
            ))
        }
    }
}

/// Deletes a download task.
///
/// Removes the specified download task from the system.
///
/// # Parameters
///
/// * `this` - The download task to delete
///
/// # Returns
///
/// * `Ok(())` if the task was successfully deleted
/// * `Err(BusinessError)` if there was an error during deletion
///
/// # Errors
///
/// Returns an error if the task cannot be deleted.
///
/// # Examples
///
/// ```rust
/// use request_api9::api9::download::delete;
/// use request_api9::api9::bridge::DownloadTask;
///
/// let task = DownloadTask { task_id: 123 };
/// match delete(task) {
///     Ok(_) => println!("Download task deleted successfully"),
///     Err(e) => println!("Error deleting task: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn delete(this: DownloadTask) -> Result<(), BusinessError> {
    RequestClient::get_instance()
        .remove(this.task_id)
        .map_err(|e| BusinessError::new(e, "Failed to delete download task".to_string()))
}

/// Suspends a download task.
///
/// Pauses an active download task.
///
/// # Parameters
///
/// * `this` - The download task to suspend
///
/// # Returns
///
/// * `Ok(())` if the task was successfully suspended
/// * `Err(BusinessError)` if there was an error during suspension
///
/// # Errors
///
/// Returns an error if the task cannot be paused.
///
/// # Examples
///
/// ```rust
/// use request_api9::api9::download::suspend;
/// use request_api9::api9::bridge::DownloadTask;
///
/// let task = DownloadTask { task_id: 123 };
/// match suspend(task) {
///     Ok(_) => println!("Download task suspended successfully"),
///     Err(e) => println!("Error suspending task: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn suspend(this: DownloadTask) -> Result<(), BusinessError> {
    RequestClient::get_instance()
        .pause(this.task_id)
        .map_err(|e| BusinessError::new(e, "Failed to suspend download task".to_string()))
}

/// Restores a suspended download task.
///
/// Resumes a previously paused download task.
///
/// # Parameters
///
/// * `this` - The download task to restore
///
/// # Returns
///
/// * `Ok(())` if the task was successfully resumed
/// * `Err(BusinessError)` if there was an error during restoration
///
/// # Errors
///
/// Returns an error if the task cannot be resumed.
///
/// # Examples
///
/// ```rust
/// use request_api9::api9::download::restore;
/// use request_api9::api9::bridge::DownloadTask;
///
/// let task = DownloadTask { task_id: 123 };
/// match restore(task) {
///     Ok(_) => println!("Download task restored successfully"),
///     Err(e) => println!("Error restoring task: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn restore(this: DownloadTask) -> Result<(), BusinessError> {
    RequestClient::get_instance()
        .resume(this.task_id)
        .map_err(|e| BusinessError::new(e, "Failed to restore download task".to_string()))
}

/// Retrieves information about a download task.
///
/// Gets detailed information about the specified download task.
///
/// # Parameters
///
/// * `this` - The download task to get information for
///
/// # Returns
///
/// * `Ok(DownloadInfo)` containing the task information
/// * `Err(BusinessError)` if there was an error retrieving the information
///
/// # Errors
///
/// Returns an error if the task information cannot be retrieved.
///
/// # Examples
///
/// ```rust
/// use request_api9::api9::download::get_task_info;
/// use request_api9::api9::bridge::DownloadTask;
///
/// let task = DownloadTask { task_id: 123 };
/// match get_task_info(task) {
///     Ok(info) => println!("Task status: {}", info.status),
///     Err(e) => println!("Error getting task info: {}", e),
/// }
/// ```
#[ani_rs::native]
pub fn get_task_info(this: DownloadTask) -> Result<DownloadInfo, BusinessError> {
    RequestClient::get_instance()
        .show_task(this.task_id)
        .map(|info| DownloadInfo::from(info))
        .map_err(|e| BusinessError::new(e, "Failed to get download task info".to_string()))
}

/// Gets the MIME type of a download task.
///
/// Returns the MIME type for the specified download task.
/// 
/// # Notes
/// 
/// Currently returns a static value of "application/octet-stream" for all tasks.
///
/// # Parameters
///
/// * `this` - The download task to get MIME type for
///
/// # Returns
///
/// * `Ok(String)` containing the MIME type
///
/// # Examples
///
/// ```rust
/// use request_api9::api9::download::get_task_mime_type;
/// use request_api9::api9::bridge::DownloadTask;
///
/// let task = DownloadTask { task_id: 123 };
/// let mime_type = get_task_mime_type(task).unwrap();
/// println!("Task MIME type: {}", mime_type);
/// // Output: Task MIME type: application/octet-stream
/// ```
#[ani_rs::native]
pub fn get_task_mime_type(this: DownloadTask) -> Result<String, BusinessError> {
    Ok("application/octet-stream".to_string())
}
