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

use request_core::config::{Action, TaskConfig};

use crate::verify::ConfigVerifier;

pub struct IndexVerifier {}

impl ConfigVerifier for IndexVerifier {
    fn verify(&self, config: &TaskConfig) -> Result<(), i32> {
        if matches!(config.common_data.action, Action::Download) {
            if config.common_data.index != 0 {
                error!("index must be 0 for download action");
                return Err(401);
            }
        } else {
            if config.common_data.index > config.file_specs.len() as u32 {
                error!("index must be less than file_specs len");
                return Err(401);
            }
        }
        Ok(())
    }
}
