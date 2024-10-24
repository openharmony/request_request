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

use cxx::UniquePtr;
use ffi::{PreDownloadOptions, PreDownloadTaskCallback};
use netstack_rs::request::Request;

use super::task::{DownloadTask, TaskHandle};

pub fn pre_download(url: String, options: PreDownloadOptions) -> Box<TaskHandle> {
    let mut request = Request::new();
    request.url(&url);
    Box::new(DownloadTask::run(
        request,
        UserCallback::from_ffi(options.callback),
    ))
}
pub(super) struct UserCallback {
    inner: UniquePtr<PreDownloadTaskCallback>,
}

impl UserCallback {
    fn from_ffi(ffi: UniquePtr<PreDownloadTaskCallback>) -> Option<Self> {
        if ffi.is_null() {
            None
        } else {
            Some(Self { inner: ffi })
        }
    }

    #[inline]
    pub(super) fn on_success(&self) {
        self.inner.OnSuccess();
    }

    #[inline]
    pub(super) fn on_fail(&self) {
        self.inner.OnFail();
    }

    #[inline]
    pub(super) fn on_cancel(&self) {
        self.inner.OnCancel();
    }
}

#[cxx::bridge(namespace = "OHOS::Request")]
pub(crate) mod ffi {

    struct PreDownloadOptions {
        callback: UniquePtr<PreDownloadTaskCallback>,
    }

    extern "Rust" {
        type DownloadTask;
        type TaskHandle;

        fn pre_download(url: String, options: PreDownloadOptions) -> Box<TaskHandle>;
        fn cancel(self: &mut TaskHandle);
    }

    unsafe extern "C++" {
        include!("pre_download.h");
        type PreDownloadTaskCallback;

        fn OnSuccess(self: &PreDownloadTaskCallback);
        fn OnFail(self: &PreDownloadTaskCallback);
        fn OnCancel(self: &PreDownloadTaskCallback);
    }
}
