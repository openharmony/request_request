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

use cxx::let_cxx_string;
use request_core::config::TaskConfig;

use crate::verify::ConfigVerifier;

pub struct UrlVerifier {}

impl ConfigVerifier for UrlVerifier {
    fn verify(&self, config: &TaskConfig) -> Result<(), i32> {
        const URL_MAX_SIZE: usize = 8192;
        if config.url.len() > URL_MAX_SIZE {
            error!("url length must be less than 8192");
            return Err(401);
        }

        let host_name = get_hostname_from_url(&config.url);

        let_cxx_string!(target_file = host_name);
        let cleartext_permitted = request_utils::wrapper::IsCleartextPermitted(&target_file);

        if !cleartext_permitted {
            if !config.url.starts_with("https://") {
                error!("ParseUrl error: url must start with https://");
                return Err(401);
            }
        } else {
            if !config.url.starts_with("http://") && !config.url.starts_with("https://") {
                error!("ParseUrl error: url must start with http:// or https://");
                return Err(401);
            }
        }
        Ok(())
    }
}

pub(crate) fn get_hostname_from_url(url: &str) -> String {
    if url.is_empty() {
        return String::new();
    }

    let delimiter = "://";
    let temp_url = url.replace('\\', "/");

    let mut pos_start = match temp_url.find(delimiter) {
        Some(pos) => pos + delimiter.len(),
        None => 0,
    };

    if let Some(not_slash) = temp_url[pos_start..].find(|c: char| c != '/') {
        pos_start += not_slash;
    }

    if let Some(end) = temp_url[pos_start..].find(|c| c == ':' || c == '/' || c == '?') {
        temp_url[pos_start..pos_start + end].to_string()
    } else {
        temp_url[pos_start..].to_string()
    }
}
