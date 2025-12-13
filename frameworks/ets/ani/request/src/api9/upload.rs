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

//! Upload module for API 9.
//!
//! This module provides functions to manage upload tasks in API 9, including
//! creating and deleting upload tasks.

#![allow(unused)]

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

use crate::api9::bridge::{UploadConfig, UploadTask};
use crate::constant::*;

#[ani_rs::native]
pub fn check_config(env: &AniEnv, context: AniRef, config: UploadConfig) -> Result<i64, BusinessError> {
    // Placeholder implementation that returns a task with ID 0
    let context = AniObject::from(context);
    let seq = TaskSeq::next().0.get();
    info!("Check task, seq: {}", seq);
    let context = Context::new(env, &context);

    let mut config: TaskConfig = config.into();
    config.bundle_type = context.get_bundle_type() as u32;
    config.bundle = context.get_bundle_name();

    match RequestClient::get_instance().check_config(
        context,
        seq,
        config,
    ) {
        Ok(()) => Ok(seq as i64),
        Err(CreateTaskError::DownloadPath(_)) => {
            return Err(BusinessError::new(
                13400001,
                "Invalid file or file system error.".to_string(),
            ))
        },
        Err(CreateTaskError::Code(code)) => {
            return Err(BusinessError::new(
                code,
                "Upload failed.".to_string(),
            ))
        }
    }
}

/// Creates an upload task with the given configuration.
///
/// # Parameters
///
/// * `context` - The application context
/// * `config` - The upload configuration containing URL, file path, etc.
///
/// # Returns
///
/// * `Ok(UploadTask)` with a task ID of 0 (placeholder implementation)
///
/// # Examples
///
/// ```rust
/// use ani_rs::objects::AniRef;
/// use request_api9::api9::upload::upload_file;
/// use request_api9::api9::bridge::UploadConfig;
///
/// // Assuming context is properly initialized
/// let config = UploadConfig {
///     url: "https://example.com/upload".to_string(),
///     file_path: "./local/file.txt".to_string(),
///     // Other configuration fields...
/// };
///
/// match upload_file(context, config) {
///     Ok(task) => println!("Upload task created with ID: {}", task.task_id),
///     Err(e) => println!("Error creating upload task: {}", e),
/// }
/// ```
///
/// # Notes
///
/// This is a placeholder implementation that returns a task with ID 0.
#[ani_rs::native]
pub fn upload_file(env: &AniEnv, context: AniRef, seq: i64) -> Result<UploadTask, BusinessError> {
    // Placeholder implementation that returns a task with ID 0
    let context = AniObject::from(context);
    let context = Context::new(env, &context);

    let task = match RequestClient::get_instance().create_task(
        context,
        seq as u64,
    ) {
        Ok(task_id) => UploadTask { task_id: task_id.to_string() },
        Err(CreateTaskError::DownloadPath(_)) => {
            return Err(BusinessError::new(
                13400001,
                "Invalid file or file system error.".to_string(),
            ))
        },
        Err(CreateTaskError::Code(code)) => {
            return Err(BusinessError::new(
                code,
                "Upload failed.".to_string(),
            ))
        }
    };

    let tid = task.task_id.parse().unwrap();
    match RequestClient::get_instance().start(tid) {
        Ok(_) => {
            info!("Api9 upload started successfully, seq: {}", seq);
            Ok(task)
        }
        Err(e) => {
            error!("Api9 upload start failed, error: {}", e);
            Err(BusinessError::new(
                e,
                format!("Upload start failed with error code: {}", e),
            ))
        }
    }
}

/// Deletes an upload task.
///
/// # Parameters
///
/// * `this` - The upload task to delete
///
/// # Returns
///
/// * `Ok(())` unconditionally (placeholder implementation)
///
/// # Examples
///
/// ```rust
/// use request_api9::api9::upload::delete;
/// use request_api9::api9::bridge::UploadTask;
///
/// let task = UploadTask { task_id: 0 };
/// match delete(task) {
///     Ok(_) => println!("Upload task deleted successfully"),
///     Err(e) => println!("Error deleting upload task: {}", e),
/// }
/// ```
///
/// # Notes
///
/// This is a placeholder implementation that always succeeds.
#[ani_rs::native]
pub fn delete(this: UploadTask) -> Result<bool, BusinessError> {
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
