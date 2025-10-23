// Copyright (C) 2023 Huawei Device Co., Ltd.
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

//! Permission management for request operations.
//! 
//! This module provides utilities for checking and managing permissions required
//! for performing download and upload operations within the request system.
//! It handles permission verification for both regular operations and management
//! capabilities.

use crate::config::Action;
use crate::utils::check_permission;

/// Permission string for internet access.
static INTERNET_PERMISSION: &str = "ohos.permission.INTERNET";
/// Permission string for download session management.
static MANAGER_DOWNLOAD: &str = "ohos.permission.DOWNLOAD_SESSION_MANAGER";
/// Permission string for upload session management.
static MANAGER_UPLOAD: &str = "ohos.permission.UPLOAD_SESSION_MANAGER";

/// Utility struct for checking permissions.
/// 
/// Provides static methods to verify various permissions required for
/// request operations.
pub(crate) struct PermissionChecker;

impl PermissionChecker {
    /// Checks if the caller has internet access permission.
    /// 
    /// # Returns
    /// 
    /// `true` if the caller has internet permission, `false` otherwise.
    pub(crate) fn check_internet() -> bool {
        check_permission(INTERNET_PERMISSION)
    }

    /// Checks if the caller has download session management permission.
    /// 
    /// # Returns
    /// 
    /// `true` if the caller has download management permission, `false` otherwise.
    pub(crate) fn check_down_permission() -> bool {
        check_permission(MANAGER_DOWNLOAD)
    }

    /// Checks the caller's management permissions for download and upload operations.
    /// 
    /// # Returns
    /// 
    /// A `ManagerPermission` enum value indicating the level of management permissions
    /// the caller possesses.
    pub(crate) fn check_manager() -> ManagerPermission {
        debug!("Checks MANAGER permission");

        // Check both download and upload management permissions
        let manager_download = check_permission(MANAGER_DOWNLOAD);
        let manager_upload = check_permission(MANAGER_UPLOAD);
        info!(
            "Checks manager_download permission is {}, manager_upload permission is {}",
            manager_download, manager_upload
        );

        // Determine the combined permission level
        match (manager_download, manager_upload) {
            (true, true) => ManagerPermission::ManagerAll,
            (true, false) => ManagerPermission::ManagerDownLoad,
            (false, true) => ManagerPermission::ManagerUpload,
            (false, false) => ManagerPermission::NoPermission,
        }
    }
}

/// Represents the level of management permissions for request operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ManagerPermission {
    /// No management permissions.
    NoPermission = 0,
    /// Permission to manage downloads only.
    ManagerDownLoad,
    /// Permission to manage uploads only.
    ManagerUpload,
    /// Permission to manage both downloads and uploads.
    ManagerAll,
}

impl ManagerPermission {
    /// Maps the permission level to a corresponding action type.
    /// 
    /// # Returns
    /// 
    /// An `Option<Action>` representing the action(s) allowed by this permission level.
    /// Returns `None` if no permissions are granted.
    pub(crate) fn get_action(&self) -> Option<Action> {
        match self {
            ManagerPermission::NoPermission => None,
            ManagerPermission::ManagerDownLoad => Some(Action::Download),
            ManagerPermission::ManagerUpload => Some(Action::Upload),
            ManagerPermission::ManagerAll => Some(Action::Any),
        }
    }

    /// Checks if a caller's action permission allows them to perform a specific task action.
    /// 
    /// # Arguments
    /// 
    /// * `caller_action` - The action permission level of the caller
    /// * `task_action` - The action type of the task being accessed
    /// 
    /// # Returns
    /// 
    /// `true` if the caller has permission to perform the specified task action,
    /// `false` otherwise.
    pub(crate) fn check_action(caller_action: Action, task_action: Action) -> bool {
        // Caller can perform the task if they have exact matching permission or full permission
        caller_action == task_action || caller_action == Action::Any
    }
}
