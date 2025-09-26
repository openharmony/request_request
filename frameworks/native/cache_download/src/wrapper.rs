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

use cache_core::observe::observe_image_file_delete;
use cache_core::RamCache;
use cxx::{SharedPtr, UniquePtr};
use ffi::{FfiPredownloadOptions, PreloadCallbackWrapper, PreloadProgressCallbackWrapper};

use crate::download::task::{Downloader, TaskHandle};
use crate::download::CacheDownloadError;
use crate::info::RustDownloadInfo;
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
        let shared_data = ffi::SharedData(Box::new(rust_data));
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
        match tx.send((progress, total)) {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to send progress message: {}", e);
                return;
            }
        }

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
    ) -> SharedPtr<ffi::PreloadHandle> {
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
        if !options.ssl_type.is_empty() {
            request.ssl_type(options.ssl_type);
        }
        if !options.ca_path.is_empty() {
            request.ca_path(options.ca_path);
        }
        match self.preload(request, Box::new(callback), update, Downloader::Netstack) {
            Some(handle) => ffi::ShareTaskHandle(Box::new(handle)),
            None => SharedPtr::null(),
        }
    }

    fn ffi_fetch(&'static self, url: &str) -> UniquePtr<ffi::Data> {
        match self.fetch(url).map(RustData::new) {
            Some(data) => ffi::UniqueData(Box::new(data)),
            _ => UniquePtr::null(),
        }
    }

    fn ffi_get_download_info(&'static self, url: &str) -> UniquePtr<ffi::CppDownloadInfo> {
        match self.get_download_info(url) {
            Some(info) => ffi::UniqueInfo(Box::new(RustDownloadInfo::from_download_info(info))),
            None => UniquePtr::null(),
        }
    }
}

fn cache_download_service() -> *const CacheDownloadService {
    CacheDownloadService::get_instance() as *const CacheDownloadService
}

fn set_file_cache_path(path: String) {
    observe_image_file_delete(path);
}

#[cxx::bridge(namespace = "OHOS::Request")]
pub(crate) mod ffi {
    struct FfiPredownloadOptions<'a> {
        headers: Vec<&'a str>,
        ssl_type: &'a str,
        ca_path: &'a str,
    }

    extern "Rust" {
        type CacheDownloadService;
        type RustData;
        type TaskHandle;
        type CacheDownloadError;
        type RustDownloadInfo;

        fn bytes(self: &RustData) -> &[u8];
        fn ffi_preload(
            self: &'static CacheDownloadService,
            url: &str,
            callback: UniquePtr<PreloadCallbackWrapper>,
            progress_callback: SharedPtr<PreloadProgressCallbackWrapper>,
            update: bool,
            options: &FfiPredownloadOptions,
        ) -> SharedPtr<PreloadHandle>;
        fn ffi_fetch(self: &'static CacheDownloadService, url: &str) -> UniquePtr<Data>;

        fn set_file_cache_size(self: &CacheDownloadService, size: u64);
        fn set_ram_cache_size(self: &CacheDownloadService, size: u64);
        fn set_info_list_size(self: &CacheDownloadService, size: u16);

        fn dns_time(self: &RustDownloadInfo) -> f64;
        fn connect_time(self: &RustDownloadInfo) -> f64;
        fn tls_time(self: &RustDownloadInfo) -> f64;
        fn first_send_time(self: &RustDownloadInfo) -> f64;
        fn first_recv_time(self: &RustDownloadInfo) -> f64;
        fn redirect_time(self: &RustDownloadInfo) -> f64;
        fn total_time(self: &RustDownloadInfo) -> f64;
        fn resource_size(self: &RustDownloadInfo) -> i64;
        fn server_addr(self: &RustDownloadInfo) -> String;
        fn dns_servers(self: &RustDownloadInfo) -> Vec<String>;

        fn ffi_get_download_info(
            self: &'static CacheDownloadService,
            url: &str,
        ) -> UniquePtr<CppDownloadInfo>;

        fn cache_download_service() -> *const CacheDownloadService;
        fn set_file_cache_path(path: String);
        fn cancel(self: &CacheDownloadService, url: &str);
        fn remove(self: &CacheDownloadService, url: &str);
        fn contains(self: &CacheDownloadService, url: &str) -> bool;

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
        type CppDownloadInfo;
        type PreloadHandle;

        fn SharedData(data: Box<RustData>) -> SharedPtr<Data>;
        fn ShareTaskHandle(handle: Box<TaskHandle>) -> SharedPtr<PreloadHandle>;
        fn UniqueData(data: Box<RustData>) -> UniquePtr<Data>;
        fn UniqueInfo(data: Box<RustDownloadInfo>) -> UniquePtr<CppDownloadInfo>;
        fn OnSuccess(self: &PreloadCallbackWrapper, data: SharedPtr<Data>, task_id: &str);
        fn OnFail(self: &PreloadCallbackWrapper, error: Box<CacheDownloadError>, task_id: &str);
        fn OnCancel(self: &PreloadCallbackWrapper);
        fn OnProgress(self: &PreloadProgressCallbackWrapper, progress: u64, total: u64);
    }
}
