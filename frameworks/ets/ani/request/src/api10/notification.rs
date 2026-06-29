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

use ani_rs::business_error::BusinessError;
use ani_rs::AniEnv;
use request_client::RequestClient;
use request_utils::context::Context;

use crate::api10::bridge::GroupConfig;
use crate::constant::*;

/// Maximum allowed length for notification titles.
const MAX_TITLE_LENGTH: usize = 1024;
/// Maximum allowed length for notification text content.
const MAX_TEXT_LENGTH: usize = 3072;

/// Validates notification title and text length.
///
/// Checks that the title and text do not exceed their maximum allowed lengths.
///
/// # Parameters
///
/// * `title` - Optional notification title to validate
/// * `text` - Optional notification text to validate
///
/// # Returns
///
/// * `Ok(())` if both title and text are within limits
/// * `Err(BusinessError)` if either exceeds its maximum length
fn ParseTitleText(title: &Option<String>, text: &Option<String>) -> Result<(), BusinessError> {
    if let Some(v) = title {
        if v.len() > MAX_TITLE_LENGTH {
            return Err(BusinessError::new(
                ExceptionErrorCode::E_PARAMETER_CHECK as i32,
                "wrong parameters".to_string(),
            ));
        }
    }
    if let Some(v) = text {
        if v.len() > MAX_TEXT_LENGTH {
            return Err(BusinessError::new(
                ExceptionErrorCode::E_PARAMETER_CHECK as i32,
                "wrong parameters".to_string(),
            ));
        }
    }
    Ok(())
}

/// Validates group ID format.
///
/// Checks that the group ID is not empty.
///
/// # Parameters
///
/// * `gid` - Group ID to validate
///
/// # Returns
///
/// * `Ok(())` if the group ID is valid
/// * `Err(BusinessError)` if the group ID is empty
fn ParseGid(gid: &str) -> Result<(), BusinessError> {
    if gid.is_empty() {
        return Err(BusinessError::new(
            ExceptionErrorCode::E_PARAMETER_CHECK as i32,
            "wrong parameters".to_string(),
        ));
    }
    Ok(())
}

/// Validates the notification configuration for a task group.
///
/// # Returns
///
/// * `Ok(())` if the configuration is valid
/// * `Err(BusinessError)` if `visibility` is not 1, 2, or 3
#[ani_rs::native]
pub fn check_group_config(env: &AniEnv, config: GroupConfig) -> Result<(), BusinessError> {
    if let Some(visibility) = config.notification.visibility {
        if visibility == 0 || (visibility & 0b11) != visibility {
            error!("notification visibility must be 1 or 2 or 3");
            return Err(BusinessError::new(
                ExceptionErrorCode::E_PARAMETER_CHECK as i32,
                "notification visibility must be 1 or 2 or 3".to_string(),
            ));
        }
    }
    Ok(())
}

/// Creates a new task group and returns its group ID.
///
/// # Returns
///
/// * `Ok(String)` containing the new group ID
/// * `Err(BusinessError)` if title/text validation or group creation fails
#[ani_rs::native]
pub fn create_group(env: &AniEnv, mut config: GroupConfig) -> Result<String, BusinessError> {
    ParseTitleText(&config.notification.title, &config.notification.text)?;
    let want_agent = config
        .notification
        .want_agent
        .as_ref()
        .map(|agent| Context::stringfy_want_agent(env, agent));
    let notification = request_core::config::Notification {
        title: config.notification.title,
        text: config.notification.text,
        disable: config.notification.disable,
        visibility: config.notification.visibility,
        want_agent,
    };
    RequestClient::get_instance()
        .create_group(config.gauge, notification)
        .map(|info| {
            info!("create_group: {:?}", info);
            info
        })
        .map_err(|e| BusinessError::new_static(e, "Failed to create group"))
}

/// Attaches a list of tasks to an existing task group.
///
/// # Returns
///
/// * `Ok(())` if the tasks were successfully attached
/// * `Err(BusinessError)` if the group ID is empty or attachment fails
#[ani_rs::native]
pub fn attach_group(gid: String, tids: Vec<String>) -> Result<(), BusinessError> {
    ParseGid(&gid)?;
    RequestClient::get_instance()
        .attach_group(gid, tids)
        .map_err(|e| BusinessError::new_static(e, "Failed to attach group"))
}

/// Deletes an existing task group.
///
/// # Returns
///
/// * `Ok(())` if the group was successfully deleted
/// * `Err(BusinessError)` if the group ID is empty or deletion fails
#[ani_rs::native]
pub fn delete_group(gid: String) -> Result<(), BusinessError> {
    ParseGid(&gid)?;
    RequestClient::get_instance()
        .delete_group(gid)
        .map_err(|e| BusinessError::new_static(e, "Failed to delete group"))
}
