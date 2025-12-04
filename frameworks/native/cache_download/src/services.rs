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

//! Cache download service interfaces and core functionality.
//!
//! This module defines the primary service interfaces for the cache download system,
//! including request structures, callback traits, and the main cache download service
//! implementation with singleton pattern.

// Standard library imports for thread safety and collections
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, Once, OnceLock};

// External dependencies
use cache_core::{CacheManager, RamCache};
use netstack_rs::info::{DownloadInfo, DownloadInfoMgr};
use request_utils::observe::network::NetRegistrar;
use request_utils::task_id::TaskId;

// Internal dependencies
use crate::download::task::{DownloadTask, Downloader, TaskHandle};
use crate::download::CacheDownloadError;
use crate::info::RustDownloadInfo;
use crate::observe::NetObserver;

/// Trait defining callback methods for preload operations.
///
/// Implementations of this trait receive notifications about various download events
/// including success, failure, cancellation, and progress updates.
#[allow(unused_variables)]
pub trait PreloadCallback: Send {
    /// Called when a download operation completes successfully.
    ///
    /// # Parameters
    /// - `data`: The downloaded content in RAM cache
    /// - `task_id`: Brief identifier for the completed task
    fn on_success(&mut self, data: Arc<RamCache>, task_id: &str) {}

    /// Called when a download operation fails.
    ///
    /// # Parameters
    /// - `error`: The error that caused the failure
    /// - `info`: Download information for the failed task
    /// - `task_id`: Brief identifier for the failed task
    fn on_fail(&mut self, error: CacheDownloadError, info: RustDownloadInfo, task_id: &str) {}

    /// Called when a download operation is cancelled.
    fn on_cancel(&mut self) {}

    /// Called periodically to report download progress.
    ///
    /// # Parameters
    /// - `progress`: Number of bytes downloaded so far
    /// - `total`: Total number of bytes to download
    fn on_progress(&mut self, progress: u64, total: u64) {}
}

/// Main service for managing cache downloads.
///
/// Implements the singleton pattern to provide a global instance for handling
/// all download operations, task management, and cache maintenance.
pub struct CacheDownloadService {
    /// Mapping of task IDs to their corresponding download tasks.
    running_tasks: Mutex<HashMap<TaskId, Arc<Mutex<DownloadTask>>>>,
    /// Manager for handling cached content in memory and on disk.
    cache_manager: CacheManager,
    /// Manager for storing and retrieving download information metrics.
    info_mgr: Arc<DownloadInfoMgr>,
    /// Registrar for network state observation and notifications.
    net_registrar: NetRegistrar,
}

/// Builder-style request for configuring downloads.
///
/// Provides a fluent interface for specifying download parameters like URL,
/// headers, SSL configuration, and certificate paths.
pub struct DownloadRequest<'a> {
    /// URL to download from.
    pub url: &'a str,
    /// Optional HTTP headers to include in the request.
    pub headers: Option<Vec<(&'a str, &'a str)>>,
    /// Optional SSL type specification.
    pub ssl_type: Option<&'a str>,
    /// Optional path to CA certificates.
    pub ca_path: Option<&'a str>,
}

impl<'a> DownloadRequest<'a> {
    /// Creates a new download request with the specified URL.
    ///
    /// # Parameters
    /// - `url`: The URL to download from
    ///
    /// # Examples
    ///
    /// ```rust
    /// let request = DownloadRequest::new("https://example.com/file.txt");
    /// ```
    pub fn new(url: &'a str) -> Self {
        Self {
            url,
            headers: None,
            ssl_type: None,
            ca_path: None,
        }
    }

    /// Adds HTTP headers to the download request.
    ///
    /// # Parameters
    /// - `headers`: Vector of (header_name, header_value) pairs
    ///
    /// # Returns
    /// A mutable reference to self for method chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut request = DownloadRequest::new("https://example.com/file.txt")
    ///     .headers(vec![
    ///         ("User-Agent", "CacheDownload/1.0"),
    ///         ("Accept", "application/json")
    ///     ]);
    /// ```
    pub fn headers(&mut self, headers: Vec<(&'a str, &'a str)>) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    /// Sets the SSL type for the download request.
    ///
    /// # Parameters
    /// - `ssl_type`: The SSL type to use
    ///
    /// # Returns
    /// A mutable reference to self for method chaining
    pub fn ssl_type(&mut self, ssl_type: &'a str) -> &mut Self {
        self.ssl_type = Some(ssl_type);
        self
    }

    /// Sets the path to CA certificates for SSL verification.
    ///
    /// # Parameters
    /// - `ca_path`: Path to CA certificate file or directory
    ///
    /// # Returns
    /// A mutable reference to self for method chaining
    pub fn ca_path(&mut self, ca_path: &'a str) -> &mut Self {
        self.ca_path = Some(ca_path);
        self
    }
}

impl CacheDownloadService {
    /// Creates a new cache download service instance.
    ///
    /// Initializes all required components including task tracking,
    /// cache management, and network observation.
    fn new() -> Self {
        Self {
            running_tasks: Mutex::new(HashMap::new()),
            cache_manager: CacheManager::new(),
            info_mgr: Arc::new(DownloadInfoMgr::new()),
            net_registrar: NetRegistrar::new(),
        }
    }

    /// Gets the singleton instance of the cache download service.
    ///
    /// Uses lazy initialization with thread safety guarantees to create
    /// the service instance only when first accessed. Also performs one-time
    /// initialization including panic handler setup and network observer registration.
    ///
    /// # Returns
    /// A static reference to the singleton service instance
    pub fn get_instance() -> &'static Self {
        static DOWNLOAD_AGENT: OnceLock<CacheDownloadService> = OnceLock::new();
        static ONCE: Once = Once::new();
        let cache_download = DOWNLOAD_AGENT.get_or_init(CacheDownloadService::new);

        ONCE.call_once(|| {
            // Set up custom panic handling for the service
            let old_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |info| {
                error!("Panic occurred {:?}", info);
                old_hook(info);
            }));
            // Restore cached files from previous sessions
            cache_download.cache_manager.restore_files();
            // Register network observer to monitor connectivity changes
            cache_download.net_registrar.add_observer(NetObserver);
            if let Err(e) = cache_download.net_registrar.register() {
                error!("Failed to register network observer: {:?}", e);
            }
        });

        cache_download
    }

    /// Cancels a download task identified by URL.
    ///
    /// # Parameters
    /// - `url`: URL of the download task to cancel
    pub fn cancel(&self, url: &str) {
        let task_id = TaskId::from_url(url);
        if let Some(updater) = self.running_tasks.lock().unwrap().get(&task_id).cloned() {
            updater.lock().unwrap().cancel();
        }
    }

    /// Resets all currently running download tasks.
    ///
    /// Called when network connectivity is restored to resume paused downloads.
    pub(crate) fn reset_all_tasks(&self) {
        let running_tasks = self.running_tasks.lock().unwrap();
        for task in running_tasks.values() {
            task.lock().unwrap().handle.reset();
        }
    }

    /// Removes a cached item identified by URL.
    ///
    /// # Parameters
    /// - `url`: URL of the cached item to remove
    pub fn remove(&self, url: &str) {
        let task_id = TaskId::from_url(url);
        self.cache_manager.remove(task_id);
    }

    /// Checks if a URL is already cached.
    ///
    /// # Parameters
    /// - `url`: URL to check in the cache
    ///
    /// # Returns
    /// `true` if the URL is in the cache, `false` otherwise
    pub fn contains(&self, url: &str) -> bool {
        let task_id = TaskId::from_url(url);
        self.cache_manager.contains(&task_id)
    }

    /// Preloads content from a URL into the cache.
    ///
    /// Initiates a download operation for the specified URL, optionally updating
    /// existing cached content, and using the provided callback for progress notifications.
    ///
    /// # Parameters
    /// - `request`: Download request with URL and optional configuration
    /// - `callback`: Callback to receive download events
    /// - `update`: Whether to update existing cached content
    /// - `downloader`: Type of downloader to use for the operation
    ///
    /// # Returns
    /// An optional task handle for controlling the download if it was successfully started
    pub fn preload(
        &'static self,
        request: DownloadRequest,
        mut callback: Box<dyn PreloadCallback>,
        update: bool,
        downloader: Downloader,
    ) -> Option<TaskHandle> {
        let url = request.url;
        let task_id = TaskId::from_url(url);
        info!("preload {}", task_id.brief());

        // Try to fetch from cache first if not updating
        if !update {
            if let Err(ret) = self.fetch_with_callback(&task_id, callback) {
                callback = ret;
            } else {
                info!("{} fetch success", task_id.brief());
                let handle = TaskHandle::new(task_id);
                handle.set_completed();
                return Some(handle);
            }
        }

        // Main loop to manage task creation and callback handling
        loop {
            let updater = match self.running_tasks.lock().unwrap().entry(task_id.clone()) {
                Entry::Occupied(entry) => entry.get().clone(),
                Entry::Vacant(entry) => {
                    // Create new download task if none exists
                    let download_task = DownloadTask::new(
                        task_id.clone(),
                        &self.cache_manager,
                        self.info_mgr.clone(),
                        request,
                        callback,
                        downloader,
                        0,
                    );
                    match download_task {
                        Some(task) => {
                            let updater = Arc::new(Mutex::new(task));
                            let handle = updater.lock().unwrap().task_handle();
                            entry.insert(updater);
                            return Some(handle);
                        }
                        None => return None,
                    }
                }
            };

            let mut updater = updater.lock().unwrap();
            match updater.try_add_callback(callback) {
                Ok(()) => return Some(updater.task_handle()),
                Err(mut cb) => {
                    if update {
                        info!("add callback failed, update task {}", task_id.brief());
                    } else if let Err(callback) = self.fetch_with_callback(&task_id, cb) {
                        error!("{} fetch fail after update", task_id.brief());
                        cb = callback;
                    } else {
                        info!("{} fetch success", task_id.brief());
                        let handle = TaskHandle::new(task_id);
                        handle.set_completed();
                        return Some(handle);
                    }

                    if !updater.remove_flag {
                        // Create updated download task with incremented sequence number
                        let seq = updater.seq + 1;
                        let download_task = DownloadTask::new(
                            task_id.clone(),
                            &self.cache_manager,
                            self.info_mgr.clone(),
                            request,
                            cb,
                            downloader,
                            seq,
                        );
                        match download_task {
                            Some(task) => {
                                *updater = task;
                                return Some(updater.task_handle());
                            }
                            None => return None,
                        }
                    } else {
                        callback = cb;
                    }
                }
            };
        }
    }

    /// Fetches cached content for a URL.
    ///
    /// # Parameters
    /// - `url`: URL of the content to fetch
    ///
    /// # Returns
    /// An optional Arc to the cached content if found
    pub fn fetch(&'static self, url: &str) -> Option<Arc<RamCache>> {
        let task_id = TaskId::from_url(url);
        self.cache_manager.fetch(&task_id)
    }

    /// Handles task completion notification.
    ///
    /// Removes the task from tracking if the sequence number matches the current task.
    ///
    /// # Parameters
    /// - `task_id`: ID of the completed task
    /// - `seq`: Sequence number of the task completion
    pub(crate) fn task_finish(&self, task_id: &TaskId, seq: usize) {
        let Some(updater) = self.running_tasks.lock().unwrap().get(task_id).cloned() else {
            return;
        };
        let mut updater = updater.lock().unwrap();
        if updater.seq == seq {
            updater.remove_flag = true;
            self.running_tasks.lock().unwrap().remove(task_id);
        }
    }

    /// Sets the maximum file cache size.
    ///
    /// # Parameters
    /// - `size`: Maximum size in bytes for file cache
    pub fn set_file_cache_size(&self, size: u64) {
        info!("set file cache size to {}", size);
        self.cache_manager.set_file_cache_size(size);
    }

    /// Sets the maximum RAM cache size.
    ///
    /// # Parameters
    /// - `size`: Maximum size in bytes for RAM cache
    pub fn set_ram_cache_size(&self, size: u64) {
        info!("set ram cache size to {}", size);
        self.cache_manager.set_ram_cache_size(size);
    }

    /// Sets the maximum number of download info entries to keep.
    ///
    /// # Parameters
    /// - `size`: Maximum number of download info entries
    pub fn set_info_list_size(&self, size: u16) {
        self.info_mgr.update_info_list_size(size);
    }

    /// Gets download information for a URL.
    ///
    /// # Parameters
    /// - `url`: URL to get download information for
    ///
    /// # Returns
    /// Optional download information if available
    pub fn get_download_info(&self, url: &str) -> Option<DownloadInfo> {
        let task_id = TaskId::from_url(url);
        self.info_mgr.get_download_info(task_id)
    }

    /// Clears all memory cache.
    pub fn clear_memory_cache(&self) {
        let running_tasks = self
            .running_tasks
            .lock()
            .unwrap()
            .keys()
            .cloned()
            .collect::<HashSet<_>>();
        self.cache_manager.clear_memory_cache(&running_tasks);
        info!("clear memory cache");
    }

    /// Clears all file cache.
    pub fn clear_file_cache(&self) {
        let running_tasks = self
            .running_tasks
            .lock()
            .unwrap()
            .keys()
            .cloned()
            .collect::<HashSet<_>>();
        self.cache_manager.clear_file_cache(&running_tasks);
        info!("clear file cache");
    }

    /// Fetches content from cache with callback notification.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to fetch
    /// - `callback`: Callback to notify on success or return on failure
    ///
    /// # Returns
    /// Ok(()) if content was found and callback notified, Err(callback) otherwise
    fn fetch_with_callback(
        &'static self,
        task_id: &TaskId,
        mut callback: Box<dyn PreloadCallback>,
    ) -> Result<(), Box<dyn PreloadCallback>> {
        let task_id = task_id.clone();
        if let Some(cache) = self.cache_manager.fetch(&task_id) {
            // Spawn callback in a separate thread to avoid blocking
            crate::spawn(move || callback.on_success(cache, task_id.brief()));
            Ok(())
        } else {
            Err(callback)
        }
    }
}

#[cfg(test)]
mod ut_services {
    include!("../tests/ut/ut_services.rs");
}
