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

use crate::config::Action;
use crate::utils::check_permission;

static INTERNET_PERMISSION: &str = "ohos.permission.INTERNET";
static MANAGER_DOWNLOAD: &str = "ohos.permission.DOWNLOAD_SESSION_MANAGER";
static MANAGER_UPLOAD: &str = "ohos.permission.UPLOAD_SESSION_MANAGER";

pub(crate) struct PermissionChecker;

impl PermissionChecker {
    pub(crate) fn check_internet() -> bool {
        check_permission(INTERNET_PERMISSION)
    }

    pub(crate) fn check_down_permission() -> bool {
        check_permission(MANAGER_DOWNLOAD)
    }

    pub(crate) fn check_manager() -> ManagerPermission {
        debug!("Checks MANAGER permission");

        let manager_download = check_permission(MANAGER_DOWNLOAD);
        let manager_upload = check_permission(MANAGER_UPLOAD);
        info!(
            "Checks manager_download permission is {}, manager_upload permission is {}",
            manager_download, manager_upload
        );

        match (manager_download, manager_upload) {
            (true, true) => ManagerPermission::ManagerAll,
            (true, false) => ManagerPermission::ManagerDownLoad,
            (false, true) => ManagerPermission::ManagerUpload,
            (false, false) => ManagerPermission::NoPermission,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum ManagerPermission {
    NoPermission = 0,
    ManagerDownLoad,
    ManagerUpload,
    ManagerAll,
}

impl ManagerPermission {
    pub(crate) fn get_action(&self) -> Option<Action> {
        match self {
            ManagerPermission::NoPermission => None,
            ManagerPermission::ManagerDownLoad => Some(Action::Download),
            ManagerPermission::ManagerUpload => Some(Action::Upload),
            ManagerPermission::ManagerAll => Some(Action::Any),
        }
    }

    pub(crate) fn check_action(caller_action: Action, task_action: Action) -> bool {
        caller_action == task_action || caller_action == Action::Any
    }
}
