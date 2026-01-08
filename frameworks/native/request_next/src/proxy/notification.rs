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

//! Notification group management for download tasks.
//!
//! This module extends the `RequestProxy` with functionality for managing
//! notification groups for download tasks. Notification groups allow multiple
//! related download tasks to be displayed together in the notification system.

// Local dependencies
use ipc::parcel::MsgParcel;
use ipc::remote;
use request_core::config::Notification;
use request_core::interface;

use crate::proxy::{RequestProxy, SERVICE_TOKEN};

impl RequestProxy {
    /// Creates a new notification group for download tasks.
    ///
    /// # Returns
    /// - `Ok(())` on success
    /// - `Err(i32)` with an error code on failure
    ///
    /// # Notes
    /// This method is currently not implemented. It will remain as a
    /// placeholder until the notification grouping functionality is fully
    /// developed.
    pub(crate) fn create_group(
        &self,
        gauge: Option<bool>,
        notification: Notification,
    ) -> Result<String, i32> {
        let remote = self.remote()?;
        let mut data = MsgParcel::new();

        data.write_interface_token(SERVICE_TOKEN).unwrap();
        let parsed_gauge = gauge.unwrap_or(false);
        data.write(&parsed_gauge).unwrap();

        // Serialize notification fields
        if let Some(title) = &notification.title {
            data.write(&true).unwrap();
            data.write(title).unwrap();
        } else {
            data.write(&false).unwrap();
        }

        if let Some(text) = &notification.text {
            data.write(&true).unwrap();
            data.write(text).unwrap();
        } else {
            data.write(&false).unwrap();
        }

        if let Some(ref agent) = notification.want_agent {
            data.write(&true).unwrap();
            data.write(agent).unwrap();
        } else {
            data.write(&false).unwrap();
        }

        if let Some(disable) = notification.disable {
            data.write(&disable).unwrap();
        } else {
            data.write(&false).unwrap();
        }

        if let Some(visibility) = notification.visibility {
            data.write(&(visibility as u32)).unwrap();
        } else {
            // Write gauge configuration based on task settings
            if parsed_gauge {
                data.write(&3u32).unwrap();
            } else {
                data.write(&1u32).unwrap();
            }
        }

        let mut reply = remote
            .send_request(interface::CREATE_GROUP, &mut data)
            .unwrap();

        let group_id = reply.read::<String>().unwrap();
        Ok(group_id)
    }

    /// Deletes an existing notification group.
    ///
    /// # Parameters
    /// - `group_id`: Unique identifier of the notification group to delete
    ///
    /// # Returns
    /// - `Ok(())` on success
    /// - `Err(i32)` with an error code on failure
    ///
    /// # Notes
    /// This method is currently not implemented. It will remain as a
    /// placeholder until the notification grouping functionality is fully
    /// developed.
    pub(crate) fn delete_group(&self, group_id: String) -> Result<(), i32> {
        let remote = self.remote()?;
        let mut data = MsgParcel::new();

        data.write_interface_token(SERVICE_TOKEN).unwrap();
        data.write(&group_id).unwrap();

        let mut reply = remote
            .send_request(interface::DELETE_GROUP, &mut data)
            .unwrap();

        let code = reply.read::<i32>().unwrap();
        if code != 0 {
            return Err(code);
        }
        Ok(())
    }

    /// Attaches download tasks to a notification group.
    ///
    /// # Parameters
    /// - `group_id`: Unique identifier of the notification group to attach
    ///   tasks to
    /// - `task_ids`: List of task IDs to attach to the notification group
    ///
    /// # Returns
    /// - `Ok(())` on success
    /// - `Err(i32)` with an error code on failure
    ///
    /// # Notes
    /// This method is currently not implemented. It will remain as a
    /// placeholder until the notification grouping functionality is fully
    /// developed.
    pub(crate) fn attach_group(&self, group_id: String, task_ids: Vec<String>) -> Result<(), i32> {
        let remote = self.remote()?;
        let mut data = MsgParcel::new();

        data.write_interface_token(SERVICE_TOKEN).unwrap();

        data.write(&group_id).unwrap();
        data.write(&task_ids).unwrap();

        let mut reply = remote
            .send_request(interface::ATTACH_GROUP, &mut data)
            .unwrap();

        let code = reply.read::<i32>().unwrap();
        if code != 0 {
            return Err(code);
        }
        Ok(())
    }
}
