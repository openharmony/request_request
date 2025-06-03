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

use request_core::config::{NetworkConfig, TaskConfig, TaskConfigBuilder, Version};

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
    pub header: HashMap<String, String>,
    pub method: String,
    pub index: Option<i64>,
    pub begins: Option<i64>,
    pub ends: Option<i64>,
    pub files: Vec<File>,
    pub data: Vec<RequestData>,
}

#[ani_rs::ani(path = "L@ohos/request/request/DownloadTaskInner")]
pub struct DownloadTask {
    pub task_id: i64,
}

#[ani_rs::ani(path = "L@ohos/request/request/UploadTaskInner")]
pub struct UploadTask {
    pub task_id: i64,
}

#[allow(non_snake_case)]
#[ani_rs::ani(path = "L@ohos/request/request/DownloadInfoInner")]
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

#[ani_rs::ani(path = "L@ohos/request/request/FileInner")]
pub struct File {
    filename: String,
    name: String,
    uri: String,
    type_: String,
}

#[ani_rs::ani]
pub struct RequestData {
    name: String,
    value: String,
}

#[ani_rs::ani]
pub struct TaskState {
    path: String,
    response_code: f64,
    message: String,
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
