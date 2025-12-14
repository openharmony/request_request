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

pub struct NotificationVerifier {}

impl ConfigVerifier for NotificationVerifier {
    fn verify(&self, config: &TaskConfig) -> Result<(), i32> {
        const NOTIFICATION_TITLE_MAX_LEN: usize = 1024;
        const NOTIFICATION_TEXT_MAX_LEN: usize = 3072;
        if matches!(config.version, Version::API9) {
            return Ok(());
        }

        if let Some(title) = &config.notification.title {
            if title.len() > NOTIFICATION_TITLE_MAX_LEN {
                error!("notification title length must be less than 1024");
                return Err(401);
            }
        }

        if let Some(text) = &config.notification.text {
            if (text.len() > NOTIFICATION_TEXT_MAX_LEN) {
                error!("notification text length must be less than 3072");
                return Err(401);
            }
        }

        Ok(())
    }
}
