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

use crate::utils::check_permission;

static INTERNET_PERMISSION: &str = "ohos.permission.INTERNET";
static QUERY_DOWNLOAD: &str = "ohos.permission.DOWNLOAD_SESSION_MANAGER";
static QUERY_UPLOAD: &str = "ohos.permission.UPLOAD_SESSION_MANAGER";

pub(crate) struct PermissionChecker;

impl PermissionChecker {
    pub(crate) fn check_internet() -> bool {
        check_permission(INTERNET_PERMISSION)
    }

    pub(crate) fn check_down_permission() -> bool {
        check_permission(QUERY_DOWNLOAD)
    }

    pub(crate) fn check_query() -> QueryPermission {
        debug!("Checks QUERY permission");

        let query_download = check_permission(QUERY_DOWNLOAD);
        let query_upload = check_permission(QUERY_UPLOAD);
        info!(
            "Checks query_download permission is {}, query_upload permission is {}",
            query_download, query_upload
        );

        match (query_download, query_upload) {
            (true, true) => QueryPermission::QueryAll,
            (true, false) => QueryPermission::QueryDownLoad,
            (false, true) => QueryPermission::QueryUpload,
            (false, false) => QueryPermission::NoPermission,
        }
    }
}

pub(crate) enum QueryPermission {
    NoPermission = 0,
    QueryDownLoad,
    QueryUpload,
    QueryAll,
}
