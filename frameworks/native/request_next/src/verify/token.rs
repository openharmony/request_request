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

pub struct TokenVerifier {}

impl ConfigVerifier for TokenVerifier {
    fn verify(&self, config: &TaskConfig) -> Result<(), i32> {
        const TOKEN_MAX_LEN: usize = 2048;
        const TOKEN_MIN_LEN: usize = 8;

        if matches!(config.version, Version::API9) {
            return Ok(());
        }

        if config.token.len() == 0 {
            return Ok(());
        }

        if config.token.len() < TOKEN_MIN_LEN || config.token.len() > TOKEN_MAX_LEN {
            error!("token length must be between 8 and 2048");
            return Err(401);
        }

        Ok(())
    }
}
