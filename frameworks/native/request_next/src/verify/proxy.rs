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

use request_core::config::{Action, TaskConfig, Version};

use crate::verify::ConfigVerifier;

pub struct ProxyVerifier {}

impl ConfigVerifier for ProxyVerifier {
    fn verify(&self, config: &TaskConfig) -> Result<(), i32> {
        const PROXY_MAX_LEN: usize = 512;
        if matches!(config.version, Version::API9) {
            return Ok(());
        }
        if config.proxy.is_empty() {
            return Ok(());
        }
        if config.proxy.len() > PROXY_MAX_LEN {
            error!("proxy length must be less than 512");
            return Err(401);
        }
        if !config.proxy.starts_with("http://") {
            error!("ParseProxy error");
            return Err(401);
        }
        let pos = config.proxy.rfind(':').ok_or({
            error!("ParseProxy error");
            401
        })?;
        let port_str = &config.proxy[pos + 1..];
        if port_str.len() > 5 || port_str.is_empty() {
            error!("ParseProxy error");
            return Err(401);
        }
        if !port_str.chars().all(|c| c.is_ascii_digit()) {
            error!("ParseProxy error");
            return Err(401);
        }
        Ok(())
    }
}
