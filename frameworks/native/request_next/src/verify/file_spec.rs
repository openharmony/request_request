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

pub struct FileSpecVerifier {}

impl ConfigVerifier for FileSpecVerifier {
    fn verify(&self, config: &TaskConfig) -> Result<(), i32> {
        if matches!(config.common_data.action, Action::Upload) {
            if config.file_specs.is_empty() {
                error!("file_specs must not be empty for upload action");
                return Err(401);
            }
        }
        Ok(())
    }
}
