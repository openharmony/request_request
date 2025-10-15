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

pub struct TitleVerifier {}

impl ConfigVerifier for TitleVerifier {
    fn verify(&self, config: &TaskConfig) -> Result<(), i32> {
        const TITLE_MAX_LEN: usize = 256;
        if matches!(config.version, Version::API10) && config.title.len() > TITLE_MAX_LEN {
            error!("title length must be less than 256");
            return Err(401);
        }
        Ok(())
    }
}
