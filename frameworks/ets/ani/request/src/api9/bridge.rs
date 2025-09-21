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

use std::collections::HashMap;

use request_core::config::{NetworkConfig, TaskConfig, TaskConfigBuilder, Version, FormItem};
use request_core::info::{self, TaskInfo};
use request_core::file::FileSpec;

#[ani_rs::ani]
pub struct DownloadConfig {
    pub url: String,
    pub header: Option<HashMap<String, String>>,
    pub enable_metered: Option<bool>,
    pub enable_roaming: Option<bool>,
    pub description: Option<String>,
    pub network_type: Option<i32>,
    pub file_path: Option<String>,
    pub title: Option<String>,
    pub background: Option<bool>,
}

#[ani_rs::ani]
pub struct UploadConfig {
    pub url: String,
    pub header: Option<HashMap<String, String>>,
    pub method: String,
    pub index: Option<i64>,
    pub begins: Option<i64>,
    pub ends: Option<i64>,
    pub files: Vec<File>,
    pub data: Vec<RequestData>,
}

#[ani_rs::ani(path = "@ohos.request.request.DownloadTaskInner")]
pub struct DownloadTask {
    pub task_id: String,
}

#[ani_rs::ani(path = "@ohos.request.request.UploadTaskInner")]
pub struct UploadTask {
    pub task_id: String,
}

#[allow(non_snake_case)]
#[ani_rs::ani(path = "@ohos.request.request.DownloadInfoInner")]
pub struct DownloadInfo {
    pub description: String,
    pub downloaded_bytes: i64,
    pub download_id: i64,
    pub failed_reason: i32,
    pub file_name: String,
    pub file_path: String,
    pub paused_reason: i32,
    pub status: i32,
    pub target_URI: String,
    pub download_title: String,
    pub download_total_bytes: i64,
}

#[ani_rs::ani(path = "@ohos.request.request.FileInner")]
pub struct File {
    filename: String,
    name: String,
    uri: String,
    type_: String,
}

impl From<File> for FileSpec {
    fn from(value: File) -> Self {
        FileSpec {
            file_name: value.filename,
            name: value.name,
            path: value.uri,
            mime_type: value.type_,
            is_user_file: false,
            fd: None,
        }
    }
}

#[ani_rs::ani]
pub struct RequestData {
    name: String,
    value: String,
}

impl From<RequestData> for FormItem {
    fn from(value: RequestData) -> Self {
        FormItem {
            name: value.name,
            value: value.value, 
        }
    }
}

#[ani_rs::ani(path = "L@ohos/request/request/TaskStateInner")]
#[derive(Clone)]
pub struct TaskState {
    path: String,
    response_code: i32,
    message: String,
}

impl From<request_core::info::TaskState> for TaskState {
    fn from(value: request_core::info::TaskState) -> Self {
        TaskState {
            path: value.path,
            response_code: value.response_code as i32,
            message: value.message,
        }
    }
}

impl From<DownloadConfig> for TaskConfig {
    fn from(config: DownloadConfig) -> Self {
        let mut config_builder = TaskConfigBuilder::new(Version::API9);
        config_builder.url(config.url);
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

        config_builder.build()
    }
}

impl From<TaskInfo> for DownloadInfo {
    fn from(info: TaskInfo) -> Self {
        DownloadInfo {
            description: info.description,
            downloaded_bytes: info.progress.processed[0] as i64,
            download_id: info.common_data.task_id as i64,
            failed_reason: info.common_data.reason as i32,
            file_name: info.file_specs[0].file_name.clone(),
            file_path: info.file_specs[0].path.clone(),
            paused_reason: info.common_data.reason as i32,
            status: info.progress.common_data.state as i32,
            target_URI: info.url,
            download_title: info.title,
            download_total_bytes: info.progress.sizes[0] as i64,
        }
    }
}

impl From<UploadConfig> for TaskConfig {
    fn from(config: UploadConfig) -> Self {
        let mut config_builder = TaskConfigBuilder::new(Version::API9);
        config_builder.url(config.url);
        if let Some(headers) = config.header {
            config_builder.headers(headers);
        }
        config_builder.method(config.method);
        if let Some(index) = config.index {
            config_builder.index(index);
        }
        if let Some(begins) = config.begins {
            config_builder.begins(begins);
        }
        if let Some(ends) = config.ends {
            config_builder.ends(ends);
        }
        config_builder.files(config.files.into_iter().map(Into::into).collect());
        config_builder.data(config.data.into_iter().map(Into::into).collect());
        config_builder.action(request_core::config::Action::Upload);
        config_builder.build()
    }
}