// Copyright (C) 2024 Huawei Device Co., Ltd.
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

use std::sync::Arc;

use cxx::UniquePtr;
use ffi::PreDownloadCallback;

use crate::agent::DownloadAgent;
use crate::cache::Cache;
use crate::download::CancelHandle;
use crate::CustomCallback;

pub(super) struct FfiCallback {
    inner: UniquePtr<PreDownloadCallback>,
}

unsafe impl Send for FfiCallback {}

impl FfiCallback {
    pub(crate) fn from_ffi(ffi: UniquePtr<PreDownloadCallback>) -> Option<Self> {
        if ffi.is_null() {
            None
        } else {
            Some(Self { inner: ffi })
        }
    }
}

impl CustomCallback for FfiCallback {
    fn on_success(&mut self, data: Arc<Cache>) {
        self.inner.OnSuccess();
    }

    fn on_fail(&mut self, error: &str) {
        self.inner.OnFail();
    }

    fn on_cancel(&mut self) {
        self.inner.OnCancel();
    }
}

fn download_agent() -> &'static DownloadAgent {
    DownloadAgent::get_instance()
}

#[cxx::bridge(namespace = "OHOS::Request")]
pub(crate) mod ffi {
    extern "Rust" {
        type DownloadAgent;
        type CancelHandle;

        fn cancel(self: &mut CancelHandle);
        fn ffi_pre_download(
            self: &DownloadAgent,
            url: String,
            mut callback: UniquePtr<PreDownloadCallback>,
            update: bool,
        );
        fn download_agent() -> &'static DownloadAgent;
    }

    unsafe extern "C++" {
        include!("request_pre_download.h");
        type PreDownloadCallback;

        fn OnSuccess(self: &PreDownloadCallback);
        fn OnFail(self: &PreDownloadCallback);
        fn OnCancel(self: &PreDownloadCallback);
    }
}
