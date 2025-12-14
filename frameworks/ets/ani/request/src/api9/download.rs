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
use request_client::client::error::CreateTaskError;
use request_core::config::Version;
use request_core::info::TaskInfo;
use request_utils::context::{is_stage_context, Context};
use request_core::config::TaskConfig;

use super::bridge::{DownloadConfig, DownloadTask};
use crate::api9::bridge::DownloadInfo;
use crate::seq::TaskSeq;
use crate::constant::*;

#[ani_rs::native]
pub fn check_config(
    env: &AniEnv,
    context: AniRef,
    config: DownloadConfig,
) -> Result<i64, BusinessError> {
    let context = AniObject::from(context);
    debug!("is {}", is_stage_context(env, &context));

    // Generate a new sequential task ID for tracking
    let seq = TaskSeq::next().0.get();
    info!("check task, seq: {}", seq);
    let context = Context::new(env, &context);

    let mut config: TaskConfig = config.into();
    config.bundle_type = context.get_bundle_type() as u32;
    config.bundle = context.get_bundle_name();

    // Create the download task
    match RequestClient::get_instance().check_config(
        context,
        seq,
        config
    ) {
        Ok(()) => Ok(seq as i64),
        Err(CreateTaskError::DownloadPath(_)) => {
            return Err(BusinessError::new(
                13400001,
                "Invalid file or file system error.".to_string(),
            ))
        }
        Err(CreateTaskError::Code(code)) => {
            return Err(BusinessError::new(
                code,
                "Download failed.".to_string(),
            ))
        }
    }
}

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
    seq: i64
) -> Result<DownloadTask, BusinessError> {
    let context = AniObject::from(context);
    debug!("is {}", is_stage_context(env, &context));
    let context = Context::new(env, &context);
    // Create the download task
    let task = match RequestClient::get_instance().create_task(
        context,
        seq as u64
    ) {
        Ok(task_id) => DownloadTask { task_id: task_id.to_string() },
        Err(CreateTaskError::DownloadPath(_)) => {
            return Err(BusinessError::new(
                13400001,
                "Invalid file or file system error.".to_string(),
            ))
        }
        Err(CreateTaskError::Code(code)) => {
            return Err(BusinessError::new(
                code,
                "Download failed.".to_string(),
            ))
        }
    };

    let tid = task.task_id.parse().unwrap();
    // Start the download task
    match RequestClient::get_instance().start(tid) {
        Ok(_) => {
            info!("Api9 download started successfully, seq: {}", seq);
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
pub fn delete(this: DownloadTask) -> Result<bool, BusinessError> {
    RequestClient::get_instance()
        .remove(this.task_id.parse().unwrap())
        .map_err(|e| {
            if e != ExceptionErrorCode::E_PERMISSION as i32 {
                Ok(true)
            } else {
                Err(BusinessError::new(e, "Failed to delete download task".to_string()))
            }
        });
    Ok(true)
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
pub fn suspend(this: DownloadTask) -> Result<bool, BusinessError> {
    RequestClient::get_instance()
        .pause(this.task_id.parse().unwrap())
        .map_err(|e| {
            if e != ExceptionErrorCode::E_PERMISSION as i32 {
                Ok(true)
            } else {
                Err(BusinessError::new(e, "Failed to delete download task".to_string()))
            }
        });
    Ok(true)
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
pub fn restore(this: DownloadTask) -> Result<bool, BusinessError> {
    RequestClient::get_instance()
        .resume(this.task_id.parse().unwrap())
        .map_err(|e| {
            if e != ExceptionErrorCode::E_PERMISSION as i32 {
                Ok(true)
            } else {
                Err(BusinessError::new(e, "Failed to delete download task".to_string()))
            }
        });
    Ok(true)
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
        .show_task(this.task_id.parse().unwrap())
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
    let task_id = this.task_id.parse().unwrap();
    let result = RequestClient::get_instance().query_mime_type(task_id);
    match result {
        Ok(info) => Ok(info),
        Err(e) => {
            if e != ExceptionErrorCode::E_PERMISSION as i32 {
                Ok("".to_string())
            } else {
                Err(BusinessError::new(e, "Failed to get task mime type".to_string()))
            }
        }
    }
}
