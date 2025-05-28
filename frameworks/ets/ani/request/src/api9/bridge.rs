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

use request_core::{Action, CommonTaskConfig, Mode, NetworkConfig, TaskConfig, Version};
use serde::{Deserialize, Serialize};

#[ani_rs::ani(path = "L@ohos/request/request/DownloadConfigInner")]
#[derive(Debug)]
pub struct DownloadConfig {
    pub url: Option<String>,
    pub header: Option<HashMap<String, String>>,
    pub enable_metered: Option<bool>,
    pub enable_roaming: Option<bool>,
    pub description: Option<String>,
    pub network_type: Option<f64>,
    pub file_path: Option<String>,
    pub title: Option<String>,
    pub background: Option<bool>,
}

#[ani_rs::ani(path = "L@ohos/request/request/DownloadInfoInner")]
struct DownloadInfo {
    description: String,
    downloaded_bytes: f64,
    download_id: f64,
    failed_reason: f64,
    file_name: String,
    file_path: String,
    paused_reason: f64,
    status: f64,
    target_URI: String,
    download_title: String,
    download_total_bytes: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "L@ohos/request/request/FileInner;\0")]
pub struct File {
    #[serde(rename = "filename\0")]
    filename: String,
    #[serde(rename = "name\0")]
    name: String,
    #[serde(rename = "url\0")]
    uri: String,
    #[serde(rename = "type\0")]
    type_: String,
}

#[ani_rs::ani(path = "L@ohos/request/request/RequestDataInner")]
struct RequestData {
    name: String,
    value: String,
}

#[ani_rs::ani(path = "L@ohos/request/request/UploadConfigInner")]
struct UploadConfig {
    url: String,
    header: HashMap<String, String>,
    method: String,
    index: Option<f64>,
    begins: Option<f64>,
    ends: Option<f64>,
    files: Vec<File>,
    data: Vec<RequestData>,
}

#[ani_rs::ani(path = "L@ohos/request/request/TaskStateInner")]
pub struct TaskState {
    path: String,
    response_code: f64,
    message: String,
}

impl From<DownloadConfig> for TaskConfig {
    fn from(config: DownloadConfig) -> Self {
        TaskConfig {
            bundle: "".to_string(),
            bundle_type: 0,
            atomic_account: "".to_string(),
            url: config.url.unwrap(),
            title: config.title.unwrap_or("".to_string()),
            description: config.description.unwrap_or_default(),
            method: "".to_string(),
            headers: config.header.unwrap_or_default(),
            data: "".to_string(),
            token: "".to_string(),
            proxy: "".to_string(),
            certificate_pins: "".to_string(),
            extras: HashMap::new(),
            version: Version::API9,
            form_items: vec![],
            file_specs: vec![],
            body_file_paths: vec![],
            certs_path: vec![],
            common_data: CommonTaskConfig {
                task_id: 0,
                uid: 0,
                token_id: 0,
                action: Action::Download,
                mode: Mode::Any,
                cover: false,
                network_config: NetworkConfig::Any,
                metered: false,
                roaming: false,
                retry: false,
                redirect: false,
                index: 0,
                begins: 0u64,
                ends: 0i64,
                gauge: false,
                precise: false,
                priority: 0u32,
                background: false,
                multipart: false,
            },
        }
    }
}
