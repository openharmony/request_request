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

//! Configuration for download task notifications.
//! 
//! This module defines the notification configuration structure used to customize
//! how download tasks appear and behave in the notification bar, including title,
//! text content, interaction options, visibility settings, and display preferences.

#[cfg(feature = "oh")]
use ipc::parcel::Deserialize;

/// Configuration structure for customizing download task notifications.
/// 
/// Allows specifying various aspects of how a download task's notification is
/// displayed, including title, description, interaction options, and visibility
/// settings for different notification elements.
pub(crate) struct NotificationConfig {
    /// The ID of the task this notification configuration applies to.
    pub(crate) task_id: u32,
    /// Optional custom title for the notification.
    pub(crate) title: Option<String>,
    /// Optional custom text description for the notification.
    pub(crate) text: Option<String>,
    /// Optional WantAgent for notification click handling.
    pub(crate) want_agent: Option<String>,
    /// Whether to disable the notification completely.
    pub(crate) disable: bool,
    /// Bitmask controlling which notification elements are visible.
    /// 
    /// - 0b01: Controls visibility of completion status
    /// - 0b10: Controls visibility of progress information
    pub(crate) visibility: u32,
}

#[cfg(test)]
impl NotificationConfig {
    /// Creates a new notification configuration with specified parameters.
    /// 
    /// This method is only available in test mode.
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task
    /// * `title` - Optional custom title
    /// * `text` - Optional custom text description
    /// * `want_agent` - Optional WantAgent for click handling
    /// * `disable` - Whether to disable the notification
    /// * `visibility` - Bitmask for visibility settings
    /// 
    /// # Returns
    /// 
    /// A new `NotificationConfig` instance with the provided configuration
    pub(crate) fn new(
        task_id: u32,
        title: Option<String>,
        text: Option<String>,
        want_agent: Option<String>,
        disable: bool,
        visibility: u32,
    ) -> Self {
        Self {
            task_id,
            title,
            text,
            want_agent,
            disable,
            visibility,
        }
    }
}

#[cfg(feature = "oh")]
impl Deserialize for NotificationConfig {
    /// Deserializes a `NotificationConfig` from an IPC parcel.
    /// 
    /// # Arguments
    /// 
    /// * `parcel` - The message parcel to read from
    /// 
    /// # Returns
    /// 
    /// * `Ok(Self)` - If deserialization succeeds
    /// * `Err(IpcError)` - If deserialization fails
    fn deserialize(parcel: &mut ipc::parcel::MsgParcel) -> ipc::IpcResult<Self> {
        // Read optional fields using a flag followed by the data if flag is true
        let title = if parcel.read::<bool>()? {
            Some(parcel.read::<String>()?)
        } else {
            None
        };

        let text = if parcel.read::<bool>()? {
            Some(parcel.read::<String>()?)
        } else {
            None
        };

        let want_agent = if parcel.read::<bool>()? {
            Some(parcel.read::<String>()?)
        } else {
            None
        };
        
        let disable = parcel.read::<bool>()?;
        let visibility = parcel.read::<u32>()?;

        // Note: task_id is initialized to 0 here and will be set externally
        // after deserialization
        let config = NotificationConfig {
            task_id: 0,
            title,
            text,
            want_agent,
            disable,
            visibility,
        };
        Ok(config)
    }
}
