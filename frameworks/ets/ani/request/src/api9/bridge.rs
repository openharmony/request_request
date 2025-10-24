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

//! Bridge module for API 9 request functionality.
//! 
//! This module defines data structures and conversion traits to bridge between the ETS interface
//! and the underlying request core functionality. It provides type definitions for download and
//! upload configurations, tasks, and information.

use std::collections::HashMap;

use request_core::config::{NetworkConfig, TaskConfig, TaskConfigBuilder, Version};
use request_core::info::TaskInfo;

/// Configuration for a download task.
///
/// Represents the parameters needed to configure a download operation through the ETS API.
#[ani_rs::ani]
pub struct DownloadConfig {
    /// The URL to download from.
    pub url: String,
    /// Optional HTTP headers to include in the request.
    pub header: Option<HashMap<String, String>>,
    /// Whether to allow downloads on metered networks.
    pub enable_metered: Option<bool>,
    /// Whether to allow downloads while roaming.
    pub enable_roaming: Option<bool>,
    /// Optional description of the download.
    pub description: Option<String>,
    /// Optional network type restriction.
    pub network_type: Option<i32>,
    /// Optional file path for saving the download.
    pub file_path: Option<String>,
    /// Optional title for the download.
    pub title: Option<String>,
    /// Whether to download in the background.
    pub background: Option<bool>,
}

/// Configuration for an upload task.
///
/// Represents the parameters needed to configure an upload operation through the ETS API.
#[ani_rs::ani]
pub struct UploadConfig {
    /// The URL to upload to.
    pub url: String,
    /// HTTP headers to include in the request.
    pub header: HashMap<String, String>,
    /// The HTTP method to use for the upload.
    pub method: String,
    /// Optional index parameter.
    pub index: Option<i64>,
    /// Optional beginning byte offset for partial uploads.
    pub begins: Option<i64>,
    /// Optional ending byte offset for partial uploads.
    pub ends: Option<i64>,
    /// List of files to upload.
    pub files: Vec<File>,
    /// List of form data to include in the upload.
    pub data: Vec<RequestData>,
}

/// Represents a download task.
///
/// Provides a handle to interact with a download operation.
#[ani_rs::ani(path = "L@ohos/request/request/DownloadTaskInner")]
pub struct DownloadTask {
    /// The unique identifier of the download task.
    pub task_id: i64,
}

/// Represents an upload task.
///
/// Provides a handle to interact with an upload operation.
#[ani_rs::ani(path = "L@ohos/request/request/UploadTaskInner")]
pub struct UploadTask {
    /// The unique identifier of the upload task.
    pub task_id: i64,
}

/// Information about a download task.
///
/// Contains detailed information about the state and progress of a download operation.
#[allow(non_snake_case)]
#[ani_rs::ani(path = "L@ohos/request/request/DownloadInfoInner")]
pub struct DownloadInfo {
    /// Description of the download.
    pub description: String,
    /// Number of bytes already downloaded.
    pub downloaded_bytes: i64,
    /// Unique identifier of the download.
    pub download_id: i64,
    /// Reason code if the download failed.
    pub failed_reason: i32,
    /// Name of the downloaded file.
    pub file_name: String,
    /// Path where the file is saved.
    pub file_path: String,
    /// Reason code if the download is paused.
    pub paused_reason: i32,
    /// Current status of the download.
    pub status: i32,
    /// Target URI being downloaded.
    pub target_URI: String,
    /// Title of the download.
    pub download_title: String,
    /// Total size of the download in bytes.
    pub download_total_bytes: i64,
}

/// Represents a file to be uploaded.
///
/// Contains information about a file that will be included in an upload request.
#[ani_rs::ani(path = "L@ohos/request/request/FileInner")]
pub struct File {
    /// Original filename of the file.
    filename: String,
    /// Form field name for the file.
    name: String,
    /// URI pointing to the file location.
    uri: String,
    /// MIME type of the file.
    type_: String,
}

/// Represents form data for a request.
///
/// Contains a key-value pair to be included in a request.
#[ani_rs::ani]
pub struct RequestData {
    /// Name of the form field.
    name: String,
    /// Value of the form field.
    value: String,
}

/// Represents the state of a task.
///
/// Contains information about the current state of a task operation.
#[ani_rs::ani]
pub struct TaskState {
    /// Path associated with the task.
    path: String,
    /// HTTP response code from the server.
    response_code: f64,
    /// Status message associated with the task.
    message: String,
}

/// Converts from `DownloadConfig` to `TaskConfig`.
///
/// Transforms the ETS API configuration into the format expected by the underlying request core.
impl From<DownloadConfig> for TaskConfig {
    fn from(config: DownloadConfig) -> Self {
        // Create builder configured for API9
        let mut config_builder = TaskConfigBuilder::new(Version::API9);
        
        // Set required URL
        config_builder.url(config.url);
        
        // Add optional parameters if provided
        if let Some(headers) = config.header {
            config_builder.headers(headers);
        }
        if let Some(enable_metered) = config.enable_metered {
            config_builder.metered(enable_metered);
        }
        if let Some(enable_roaming) = config.enable_roaming {
            config_builder.roaming(enable_roaming);
        }
        if let Some(network_type) = config.network_type {
            config_builder.network_type(NetworkConfig::from(network_type));
        }
        if let Some(description) = config.description {
            config_builder.description(description);
        }
        if let Some(title) = config.title {
            config_builder.title(title);
        }
        if let Some(background) = config.background {
            config_builder.background(background);
        }

        // Build the final task configuration
        config_builder.build()
    }
}

/// Converts from `TaskInfo` to `DownloadInfo`.
///
/// Transforms the core task information into the format expected by the ETS API.
impl From<TaskInfo> for DownloadInfo {
    fn from(info: TaskInfo) -> Self {
        DownloadInfo {
            // Direct field mappings
            description: info.description,
            // Extract first element from arrays (assuming single-file downloads)
            downloaded_bytes: info.progress.processed[0] as i64,
            // Convert ID types as needed
            download_id: info.common_data.task_id as i64,
            failed_reason: info.common_data.reason as i32,
            // Extract file information from first file spec
            file_name: info.file_specs[0].file_name.clone(),
            file_path: info.file_specs[0].path.clone(),
            // Reason code used for both failure and pause states
            paused_reason: info.common_data.reason as i32,
            status: info.progress.common_data.state as i32,
            target_URI: info.url,
            download_title: info.title,
            // Get total size from progress information
            download_total_bytes: info.progress.sizes[0] as i64,
        }
    }
}
