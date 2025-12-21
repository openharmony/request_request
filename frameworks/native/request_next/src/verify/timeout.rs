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

use crate::verify::ConfigVerifier;

use request_core::config::TaskConfig;

const MIN_TIMEOUT: u64 = 1;
const MAX_TIMEOUT: u64 = 604800;

pub struct TimeoutVerifier {}

impl ConfigVerifier for TimeoutVerifier {
    fn verify(&self, config: &TaskConfig) -> Result<(), i32> {
        if config.timeout.connection_timeout < MIN_TIMEOUT {
                error!("Parameter verification failed, the connectionTimeout is less than minimum");
                return Err(401);
        }
        if config.timeout.total_timeout < MIN_TIMEOUT || config.timeout.total_timeout > MAX_TIMEOUT {
            error!("Parameter verification failed, the totalTimeout exceeds the limit");
            return Err(401);
    }
        Ok(())
    }
}