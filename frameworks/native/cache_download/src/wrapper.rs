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

//! C++ FFI bridge for cache download service.
//!
//! This module provides the C++ Foreign Function Interface (FFI) bridge for the cache
//! download service, allowing C++ code to interact with the Rust implementation through
//! type-safe bindings.

// Standard library imports for thread synchronization and communication
use std::sync::{mpsc, Arc, Mutex};

// External dependencies for cache core and FFI bridge
use cache_core::observe::observe_image_file_delete;
use cache_core::RamCache;
use cxx::{SharedPtr, UniquePtr};
use ffi::{FfiPredownloadOptions, PreloadCallbackWrapper, PreloadProgressCallbackWrapper};

// Internal dependencies from cache_download
use crate::download::task::{Downloader, TaskHandle};
use crate::download::CacheDownloadError;
use crate::info::RustDownloadInfo;
use crate::services::{CacheDownloadService, DownloadRequest, PreloadCallback};

/// FFI implementation of the PreloadCallback trait for C++ interoperability.
///
/// Translates Rust download events into C++ callback invocations, managing
/// synchronization between threads and handling progress reporting with buffering.
pub(super) struct FfiCallback {
    /// C++ callback wrapper for download completion events
    callback: UniquePtr<PreloadCallbackWrapper>,
    /// C++ callback wrapper for progress events
    progress_callback: SharedPtr<PreloadProgressCallbackWrapper>,
    /// Channel for buffering progress updates
    tx: Option<mpsc::Sender<(u64, u64)>>,
    /// Mutex to track if download is finished
    finish_lock: Arc<Mutex<bool>>,
}

// Safety: FfiCallback is Send because it contains Send-compatible components
unsafe impl Send for FfiCallback {}
// Safety: PreloadProgressCallbackWrapper is designed to be thread-safe from C++ side
unsafe impl Sync for PreloadProgressCallbackWrapper {}
unsafe impl Send for PreloadProgressCallbackWrapper {}

impl FfiCallback {
    /// Creates a new FfiCallback from C++ callback pointers.
    ///
    /// # Parameters
    /// - `callback`: C++ callback for completion events
    /// - `progress_callback`: C++ callback for progress events
    ///
    /// # Returns
    /// A new FfiCallback instance configured with the provided callbacks
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

/// Rust data wrapper for exposing cached content to C++.
///
/// Provides a safe interface for accessing RamCache data from C++ code.
pub struct RustData {
    /// Underlying cached data
    data: Arc<RamCache>,
}

impl RustData {
    /// Creates a new RustData wrapper.
    ///
    /// # Parameters
    /// - `data`: Shared reference to the cached data
    fn new(data: Arc<RamCache>) -> Self {
        Self { data }
    }

    /// Gets a reference to the cached data bytes.
    ///
    /// # Returns
    /// A slice of the underlying data bytes
    fn bytes(&self) -> &[u8] {
        self.data.cursor().get_ref()
    }
}

impl PreloadCallback for FfiCallback {
    /// Handles successful download completion and notifies C++.
    ///
    /// Converts the Rust cached data into a C++ compatible format and invokes
    /// the C++ OnSuccess callback.
    ///
    /// # Parameters
    /// - `data`: The downloaded content in RAM cache
    /// - `task_id`: Identifier for the completed task
    fn on_success(&mut self, data: Arc<RamCache>, task_id: &str) {
        if self.callback.is_null() {
            return;
        }
        let rust_data = RustData::new(data);
        let shared_data = ffi::SharedData(Box::new(rust_data));
        self.callback.OnSuccess(shared_data, task_id);
    }

    /// Handles download failure and notifies C++.
    ///
    /// Converts the Rust error into a C++ compatible format and invokes
    /// the C++ OnFail callback.
    ///
    /// # Parameters
    /// - `error`: The error that caused the failure
    /// - `task_id`: Identifier for the failed task
    fn on_fail(&mut self, error: CacheDownloadError, info: RustDownloadInfo, task_id: &str) {
        if self.callback.is_null() {
            return;
        }
        self.callback
            .OnFail(Box::new(error), Box::new(info), task_id);
    }

    /// Handles download cancellation and notifies C++.
    ///
    /// Invokes the C++ OnCancel callback if the callback is not null.
    fn on_cancel(&mut self) {
        if self.callback.is_null() {
            return;
        }
        self.callback.OnCancel();
    }

    /// Handles and buffers progress updates for C++.
    ///
    /// Manages progress reporting with a dedicated thread for C++ callbacks,
    /// buffering updates to avoid overwhelming the C++ side with too many
    /// rapid progress events.
    ///
    /// # Parameters
    /// - `progress`: Number of bytes downloaded so far
    /// - `total`: Total number of bytes to download
    fn on_progress(&mut self, progress: u64, total: u64) {
        if self.progress_callback.is_null() {
            return;
        }

        // Special handling for 100% completion to ensure it's always reported
        if progress == total {
            let progress_callback = self.progress_callback.clone();
            let mutex = self.finish_lock.clone();
            crate::spawn(move || {
                *mutex.lock().unwrap() = true;
                progress_callback.OnProgress(progress, total);
            });
            return;
        }

        // Try to send on existing channel if available
        if let Some(tx) = &self.tx {
            if tx.send((progress, total)).is_ok() {
                return;
            }
        }

        // Create new channel if none exists or previous one failed
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

        // Spawn a thread to process progress updates and notify C++
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
    /// FFI-compatible preload method for C++.
    ///
    /// Translates C++ download options and callbacks into the Rust equivalents,
    /// then invokes the underlying preload method.
    ///
    /// # Parameters
    /// - `url`: URL to download
    /// - `callback`: C++ callback for completion events
    /// - `progress_callback`: C++ callback for progress events
    /// - `update`: Whether to update existing cached content
    /// - `options`: Additional download options from C++
    ///
    /// # Returns
    /// A C++ shared pointer to a PreloadHandle if successful, null otherwise
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

        // Convert C++ headers format to Rust format
        if !options.headers.is_empty() {
            let headers = options
                .headers
                .chunks(2)
                .map(|a| (a[0], a[1]))
                .collect::<Vec<(&str, &str)>>();
            request.headers(headers);
        }

        // Add SSL configuration if provided
        if !options.ssl_type.is_empty() {
            request.ssl_type(options.ssl_type);
        }
        if !options.ca_path.is_empty() {
            request.ca_path(options.ca_path);
        }

        // Perform preload and convert the result to C++ format
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

/// Gets a raw pointer to the cache download service singleton for C++.
///
/// # Returns
/// A raw pointer to the singleton CacheDownloadService instance
///
/// # Safety
/// The pointer remains valid for the lifetime of the program as it points to
/// a static singleton.
fn cache_download_service() -> *const CacheDownloadService {
    CacheDownloadService::get_instance() as *const CacheDownloadService
}

/// Sets the file cache path and registers it for observation.
///
/// # Parameters
/// - `path`: The file path to set for the cache
fn set_file_cache_path(path: String) {
    observe_image_file_delete(path);
}

// C++ FFI bridge definition using the cxx macro
// This defines the type-safe interface between Rust and C++
#[cxx::bridge(namespace = "OHOS::Request")]
pub(crate) mod ffi {
    /// C++ download options passed to the preload method
    struct FfiPredownloadOptions<'a> {
        headers: Vec<&'a str>,
        ssl_type: &'a str,
        ca_path: &'a str,
    }

    // Rust functions and types exposed to C++
    extern "Rust" {
        type CacheDownloadService;
        type RustData;
        type TaskHandle;
        type CacheDownloadError;
        type RustDownloadInfo;

        // RustData methods
        fn bytes(self: &RustData) -> &[u8];

        // CacheDownloadService methods
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
        fn clear_memory_cache(self: &CacheDownloadService);
        fn clear_file_cache(self: &CacheDownloadService);

        fn cancel(self: &mut TaskHandle);
        fn task_id(self: &TaskHandle) -> String;
        fn is_finish(self: &TaskHandle) -> bool;
        fn state(self: &TaskHandle) -> usize;

        // CacheDownloadError methods
        fn code(self: &CacheDownloadError) -> i32;
        fn message(self: &CacheDownloadError) -> &str;
        fn ffi_kind(self: &CacheDownloadError) -> i32;
    }

    // C++ types and functions imported into Rust
    unsafe extern "C++" {
        // C++ header includes
        include!("preload_callback.h");
        include!("request_preload.h");
        include!("context.h");

        // C++ types used in the bridge
        type PreloadCallbackWrapper;
        type PreloadProgressCallbackWrapper;
        type Data;
        type CppDownloadInfo;
        type PreloadHandle;

        // C++ functions for converting between Rust and C++ types
        fn SharedData(data: Box<RustData>) -> SharedPtr<Data>;
        fn ShareTaskHandle(handle: Box<TaskHandle>) -> SharedPtr<PreloadHandle>;
        fn UniqueData(data: Box<RustData>) -> UniquePtr<Data>;
        fn UniqueInfo(data: Box<RustDownloadInfo>) -> UniquePtr<CppDownloadInfo>;

        // C++ callback methods
        fn OnSuccess(self: &PreloadCallbackWrapper, data: SharedPtr<Data>, task_id: &str);
        fn OnFail(
            self: &PreloadCallbackWrapper,
            error: Box<CacheDownloadError>,
            info: Box<RustDownloadInfo>,
            task_id: &str,
        );
        fn OnCancel(self: &PreloadCallbackWrapper);
        fn OnProgress(self: &PreloadProgressCallbackWrapper, progress: u64, total: u64);
    }
}
