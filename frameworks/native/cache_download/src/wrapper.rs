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

use std::sync::{mpsc, Arc, Mutex};

use cache_core::RamCache;
use cxx::{SharedPtr, UniquePtr};
use ffi::{FfiPredownloadOptions, PreloadCallbackWrapper, PreloadProgressCallbackWrapper};

use crate::download::task::{Downloader, TaskHandle};
use crate::download::CacheDownloadError;
use crate::services::{CacheDownloadService, DownloadRequest, PreloadCallback};

pub(super) struct FfiCallback {
    callback: UniquePtr<PreloadCallbackWrapper>,
    progress_callback: SharedPtr<PreloadProgressCallbackWrapper>,
    tx: Option<mpsc::Sender<(u64, u64)>>,
    finish_lock: Arc<Mutex<bool>>,
}

unsafe impl Send for FfiCallback {}
unsafe impl Sync for PreloadProgressCallbackWrapper {}
unsafe impl Send for PreloadProgressCallbackWrapper {}

impl FfiCallback {
    pub(crate) fn from_ffi(
        callback: UniquePtr<PreloadCallbackWrapper>,
        progress_callback: SharedPtr<PreloadProgressCallbackWrapper>,
    ) -> Self {
        Self {
            callback,
            progress_callback,
            tx: None,
            finish_lock: Arc::new(Mutex::new(false)),
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

impl PreloadCallback for FfiCallback {
    fn on_success(&mut self, data: Arc<RamCache>, task_id: &str) {
        if self.callback.is_null() {
            return;
        }
        let rust_data = RustData::new(data);
        let shared_data = ffi::BuildSharedData(Box::new(rust_data));
        self.callback.OnSuccess(shared_data, task_id);
    }

    fn on_fail(&mut self, error: CacheDownloadError, task_id: &str) {
        if self.callback.is_null() {
            return;
        }
        self.callback.OnFail(Box::new(error), task_id);
    }

    fn on_cancel(&mut self) {
        if self.callback.is_null() {
            return;
        }
        self.callback.OnCancel();
    }

    fn on_progress(&mut self, progress: u64, total: u64) {
        if self.progress_callback.is_null() {
            return;
        }
        if progress == total {
            let progress_callback = self.progress_callback.clone();
            let mutex = self.finish_lock.clone();
            crate::spawn(move || {
                *mutex.lock().unwrap() = true;
                progress_callback.OnProgress(progress, total);
            });
            return;
        }

        if let Some(tx) = &self.tx {
            if tx.send((progress, total)).is_ok() {
                return;
            }
        }
        let (tx, rx) = mpsc::channel();
        tx.send((progress, total)).unwrap();
        self.tx = Some(tx);
        let progress_callback = self.progress_callback.clone();
        let mutex = self.finish_lock.clone();
        crate::spawn(move || {
            let lock = mutex.lock().unwrap();
            if *lock {
                return;
            }
            while let Ok((progress, total)) = rx.try_recv() {
                progress_callback.OnProgress(progress, total);
            }
        });
    }
}

impl CacheDownloadService {
    fn ffi_preload(
        &'static self,
        url: &str,
        callback: cxx::UniquePtr<PreloadCallbackWrapper>,
        progress_callback: cxx::SharedPtr<PreloadProgressCallbackWrapper>,
        update: bool,
        options: &FfiPredownloadOptions,
    ) -> Box<TaskHandle> {
        let callback = FfiCallback::from_ffi(callback, progress_callback);
        let mut request = DownloadRequest::new(url);
        if !options.headers.is_empty() {
            let headers = options
                .headers
                .chunks(2)
                .map(|a| (a[0], a[1]))
                .collect::<Vec<(&str, &str)>>();
            request.headers(headers);
        }

        Box::new(self.preload(request, Box::new(callback), update, Downloader::Netstack))
    }
}

fn cache_download_service() -> *const CacheDownloadService {
    CacheDownloadService::get_instance() as *const CacheDownloadService
}

#[cxx::bridge(namespace = "OHOS::Request")]
pub(crate) mod ffi {
    struct FfiPredownloadOptions<'a> {
        headers: Vec<&'a str>,
    }

    extern "Rust" {
        type CacheDownloadService;
        type RustData;
        type TaskHandle;
        type CacheDownloadError;

        fn bytes(self: &RustData) -> &[u8];
        fn ffi_preload(
            self: &'static CacheDownloadService,
            url: &str,
            callback: UniquePtr<PreloadCallbackWrapper>,
            progress_callback: SharedPtr<PreloadProgressCallbackWrapper>,
            update: bool,
            options: &FfiPredownloadOptions,
        ) -> Box<TaskHandle>;
        fn set_file_cache_size(self: &CacheDownloadService, size: u64);
        fn set_ram_cache_size(self: &CacheDownloadService, size: u64);

        fn cache_download_service() -> *const CacheDownloadService;
        fn cancel(self: &CacheDownloadService, url: &str);
        fn remove(self: &CacheDownloadService, url: &str);

        fn cancel(self: &mut TaskHandle);
        fn task_id(self: &TaskHandle) -> String;
        fn is_finish(self: &TaskHandle) -> bool;
        fn state(self: &TaskHandle) -> usize;

        fn code(self: &CacheDownloadError) -> i32;
        fn message(self: &CacheDownloadError) -> &str;
        fn ffi_kind(self: &CacheDownloadError) -> i32;
    }

    unsafe extern "C++" {
        include!("preload_callback.h");
        include!("request_preload.h");
        include!("context.h");

        type PreloadCallbackWrapper;
        type PreloadProgressCallbackWrapper;
        type Data;

        fn BuildSharedData(data: Box<RustData>) -> SharedPtr<Data>;
        fn OnSuccess(self: &PreloadCallbackWrapper, data: SharedPtr<Data>, task_id: &str);
        fn OnFail(self: &PreloadCallbackWrapper, error: Box<CacheDownloadError>, task_id: &str);
        fn OnCancel(self: &PreloadCallbackWrapper);
        fn OnProgress(self: &PreloadProgressCallbackWrapper, progress: u64, total: u64);
    }
}
