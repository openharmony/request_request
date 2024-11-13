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
use ffi::DownloadCallbackWrapper;

use crate::agent::DownloadAgent;
use crate::cache::RamCache;
use crate::download::TaskHandle;
use crate::{CustomCallback, DownloadError};

pub(super) struct FfiCallback {
    inner: UniquePtr<DownloadCallbackWrapper>,
}

unsafe impl Send for FfiCallback {}

impl FfiCallback {
    pub(crate) fn from_ffi(ffi: UniquePtr<DownloadCallbackWrapper>) -> Option<Self> {
        if ffi.is_null() {
            None
        } else {
            Some(Self { inner: ffi })
        }
    }
}

pub struct RustData {
    data: Arc<RamCache>,
}

impl RustData {
    fn new(data: Arc<RamCache>) -> Self {
        Self { data }
    }

    fn bytes(&self) -> &[u8] {
        self.data.cursor().get_ref()
    }
}

impl CustomCallback for FfiCallback {
    fn on_success(&mut self, data: Arc<RamCache>) {
        let rust_data = RustData::new(data);
        let shared_data = ffi::BuildSharedData(Box::new(rust_data));
        self.inner.OnSuccess(shared_data);
    }

    fn on_fail(&mut self, error: DownloadError) {
        self.inner.OnFail(Box::new(error));
    }

    fn on_cancel(&mut self) {
        self.inner.OnCancel();
    }

    fn on_progress(&mut self, progress: u64, total: u64) {
        self.inner.OnProgress(progress, total);
    }
}

fn download_agent() -> *const DownloadAgent {
    DownloadAgent::get_instance() as *const DownloadAgent
}

#[cxx::bridge(namespace = "OHOS::Request")]
pub(crate) mod ffi {

    struct FfiPredownloadOptions<'a> {
        headers: Vec<&'a str>,
    }

    extern "Rust" {
        type DownloadAgent;
        type RustData;
        type TaskHandle;
        type DownloadError;

        fn bytes(self: &RustData) -> &[u8];
        fn ffi_pre_download(
            self: &DownloadAgent,
            url: &str,
            mut callback: UniquePtr<DownloadCallbackWrapper>,
            update: bool,
            options: &FfiPredownloadOptions,
        ) -> Box<TaskHandle>;
        fn set_file_cache_size(self: &DownloadAgent, size: u64);
        fn set_ram_cache_size(self: &DownloadAgent, size: u64);

        fn download_agent() -> *const DownloadAgent;
        fn cancel(self: &DownloadAgent, url: &str);
        fn remove(self: &DownloadAgent, url: &str);

        fn cancel(self: &mut TaskHandle);
        fn task_id(self: &TaskHandle) -> String;
        fn is_finish(self: &TaskHandle) -> bool;
        fn state(self: &TaskHandle) -> usize;

        fn code(self: &DownloadError) -> i32;
        fn message(self: &DownloadError) -> &str;
        fn ffi_kind(self: &DownloadError) -> i32;
    }

    unsafe extern "C++" {
        include!("pre_download_callback.h");
        include!("request_pre_download.h");
        include!("context.h");

        type DownloadCallbackWrapper;
        type Data;

        fn BuildSharedData(data: Box<RustData>) -> SharedPtr<Data>;
        fn OnSuccess(self: &DownloadCallbackWrapper, data: SharedPtr<Data>);
        fn OnFail(self: &DownloadCallbackWrapper, error: Box<DownloadError>);
        fn OnCancel(self: &DownloadCallbackWrapper);
        fn OnProgress(self: &DownloadCallbackWrapper, progress: u64, total: u64);
    }
}
