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

use crate::utils::c_wrapper::CStringWrapper;

static INTERNET_PERMISSION: &str = "ohos.permission.INTERNET";
static QUERY_DOWNLOAD: &str = "ohos.permission.DOWNLOAD_SESSION_MANAGER";
static QUERY_UPLOAD: &str = "ohos.permission.UPLOAD_SESSION_MANAGER";

pub(crate) struct PermissionChecker;

impl PermissionChecker {
    pub(crate) fn check_internet() -> bool {
        debug!("Checks INTERNET permission");
        unsafe {
            DownloadServerCheckPermission(
                ipc::Skeleton::calling_full_token_id(),
                CStringWrapper::from(INTERNET_PERMISSION),
            )
        }
    }

    pub(crate) fn check_query() -> QueryPermission {
        debug!("Checks QUERY permission");
        let id = ipc::Skeleton::calling_full_token_id();
        let query_download =
            unsafe { DownloadServerCheckPermission(id, CStringWrapper::from(QUERY_DOWNLOAD)) };
        let query_upload =
            unsafe { DownloadServerCheckPermission(id, CStringWrapper::from(QUERY_UPLOAD)) };
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

#[link(name = "download_server_cxx", kind = "static")]
extern "C" {
    pub(crate) fn DownloadServerCheckPermission(token_id: u64, permission: CStringWrapper) -> bool;
}
