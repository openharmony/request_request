// Copyright (C) 2025 Huawei Device Co., Ltd.
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

//! Module for tracking and managing network operation information.
//!
//! This module provides structures for collecting, storing, and accessing
//! various metrics and information related to network downloads, including
//! performance timings, resource details, and network configuration.

use std::sync::Mutex;

use request_utils::lru::LRUCache;
use request_utils::task_id::TaskId;
use request_utils::{debug, info};

/// Represents performance metrics for network operations.
///
/// This struct tracks various timing metrics during network operations,
/// including DNS resolution, TCP connection, TLS handshake, and data transfer times.
/// All timings are stored in milliseconds.
#[derive(Clone, Copy, Default)]
pub struct RustPerformanceInfo {
    /// Time taken from startup to DNS resolution completion, in milliseconds.
    dns_timing: f64,
    /// Time taken from startup to TCP connection completion, in milliseconds.
    connect_timing: f64,
    /// Time taken from startup to TLS connection completion, in milliseconds.
    tls_timing: f64,
    /// Time taken from startup to start sending the first byte, in milliseconds.
    first_send_timing: f64,
    /// Time taken from startup to receiving the first byte, in milliseconds.
    first_receive_timing: f64,
    /// Time taken from startup to the completion of the request, in milliseconds.
    total_timing: f64,
    /// Time taken from startup to completion of all redirection steps, in milliseconds.
    redirect_timing: f64,
}

impl RustPerformanceInfo {
    /// Sets the DNS resolution timing.
    ///
    /// # Arguments
    ///
    /// * `time` - The time taken in milliseconds from startup to DNS resolution completion.
    pub fn set_dns_timing(&mut self, time: f64) {
        self.dns_timing = time;
    }

    /// Sets the TCP connection timing.
    ///
    /// # Arguments
    ///
    /// * `time` - The time taken in milliseconds from startup to TCP connection completion.
    pub fn set_connect_timing(&mut self, time: f64) {
        self.connect_timing = time;
    }

    /// Sets the TLS handshake timing.
    ///
    /// # Arguments
    ///
    /// * `time` - The time taken in milliseconds from startup to TLS connection completion.
    pub fn set_tls_timing(&mut self, time: f64) {
        self.tls_timing = time;
    }

    /// Sets the timing for sending the first byte.
    ///
    /// # Arguments
    ///
    /// * `time` - The time taken in milliseconds from startup to sending the first byte.
    pub fn set_first_send_timing(&mut self, time: f64) {
        self.first_send_timing = time;
    }

    /// Sets the timing for receiving the first byte.
    ///
    /// # Arguments
    ///
    /// * `time` - The time taken in milliseconds from startup to receiving the first byte.
    pub fn set_first_receive_timing(&mut self, time: f64) {
        self.first_receive_timing = time;
    }

    /// Sets the total request timing.
    ///
    /// # Arguments
    ///
    /// * `time` - The time taken in milliseconds from startup to request completion.
    pub fn set_total_timing(&mut self, time: f64) {
        self.total_timing = time;
    }

    /// Sets the redirection timing.
    ///
    /// # Arguments
    ///
    /// * `time` - The time taken in milliseconds from startup to completion of all redirects.
    pub fn set_redirect_timing(&mut self, time: f64) {
        self.redirect_timing = time;
    }

    /// Returns the DNS resolution timing.
    fn dns_timing(&self) -> f64 {
        self.dns_timing
    }

    /// Returns the TCP connection timing.
    fn connect_timing(&self) -> f64 {
        self.connect_timing
    }

    /// Returns the TLS handshake timing.
    fn tls_timing(&self) -> f64 {
        self.tls_timing
    }

    /// Returns the timing for sending the first byte.
    fn first_send_timing(&self) -> f64 {
        self.first_send_timing
    }

    /// Returns the timing for receiving the first byte.
    fn first_recv_timing(&self) -> f64 {
        self.first_receive_timing
    }

    /// Returns the total request timing.
    fn total_timing(&self) -> f64 {
        self.total_timing
    }

    /// Returns the redirection timing.
    fn redirect_timing(&self) -> f64 {
        self.redirect_timing
    }
}

/// Contains metadata about a downloaded resource.
///
/// Stores information such as the size of the resource being downloaded.
#[derive(Clone)]
struct ResourceInfo {
    /// Size of the resource in bytes. -1 indicates unknown size.
    size: i64,
}

impl ResourceInfo {
    /// Creates a new `ResourceInfo` instance with unknown size.
    ///
    /// Initializes with a default size value of -1, indicating the size is unknown.
    fn new() -> Self {
        ResourceInfo { size: -1 }
    }

    /// Sets the resource size in bytes.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the resource in bytes. Use -1 to indicate unknown size.
    fn set_size(&mut self, size: i64) {
        self.size = size;
    }

    /// Returns the resource size in bytes.
    ///
    /// # Returns
    ///
    /// The size of the resource in bytes, or -1 if the size is unknown.
    fn size(&self) -> i64 {
        self.size
    }
}

/// Contains network configuration and connection details for a download.
///
/// Tracks information related to the network aspects of a download, including
/// server address and DNS servers used for hostname resolution.
#[derive(Clone)]
struct NetworkInfo {
    /// Server address.
    addr: String,
    /// DNS servers used for resolution.
    dns: Vec<String>,
}

impl NetworkInfo {
    /// Creates a new `NetworkInfo` instance with empty fields.
    ///
    /// Initializes with an empty server address and DNS servers list.
    fn new() -> Self {
        NetworkInfo {
            addr: String::new(),
            dns: Vec::new(),
        }
    }

    /// Sets the DNS servers used for hostname resolution.
    ///
    /// # Arguments
    ///
    /// * `dns` - A vector of DNS server addresses.
    fn set_dns(&mut self, dns: Vec<String>) {
        self.dns = dns;
    }

    /// Sets the server address.
    ///
    /// # Arguments
    ///
    /// * `addr` - The server address to connect to.
    fn set_ip_address(&mut self, addr: String) {
        self.addr = addr;
    }

    /// Returns a copy of the DNS servers list.
    ///
    /// # Returns
    ///
    /// A vector containing the DNS server addresses used during resolution.
    fn dns(&self) -> Vec<String> {
        self.dns.clone()
    }

    /// Returns a copy of the server address.
    ///
    /// # Returns
    ///
    /// The address of the server providing the resource.
    fn addr(&self) -> String {
        self.addr.clone()
    }
}

/// Combines resource, network, and performance information for a download operation.
///
/// Provides a unified view of all relevant information about a download, including
/// metadata about the resource, network configuration, and performance metrics.
#[derive(Clone)]
pub struct DownloadInfo {
    /// Resource metadata information.
    resource: ResourceInfo,
    /// Network configuration and connection details.
    network: NetworkInfo,
    /// Performance metrics tracking operation timings.
    performance: RustPerformanceInfo,
}

impl DownloadInfo {
    /// Creates a new `DownloadInfo` instance with default values.
    ///
    /// Initializes all nested structures with their default values.
    pub(crate) fn new() -> Self {
        Self {
            resource: ResourceInfo::new(),
            network: NetworkInfo::new(),
            performance: RustPerformanceInfo::default(),
        }
    }

    /// Sets the resource size in bytes.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the downloaded resource in bytes. Use -1 for unknown size.
    pub(crate) fn set_size(&mut self, size: i64) {
        self.resource.set_size(size);
    }

    /// Sets the performance metrics for this download.
    ///
    /// # Arguments
    ///
    /// * `performance` - The performance timing data to associate with this download.
    pub(crate) fn set_performance(&mut self, performance: RustPerformanceInfo) {
        self.performance = performance;
    }

    /// Sets the DNS servers used for hostname resolution during this download.
    ///
    /// # Arguments
    ///
    /// * `dns` - A vector of DNS server addresses used during resolution.
    pub(crate) fn set_network_dns(&mut self, dns: Vec<String>) {
        self.network.set_dns(dns);
    }

    pub(crate) fn set_ip_address(&mut self, addr: String) {
        self.network.set_ip_address(addr);
    }

    /// Returns the DNS resolution time in milliseconds.
    ///
    /// # Returns
    ///
    /// The time taken in milliseconds from operation start to DNS resolution completion.
    pub fn dns_time(&self) -> f64 {
        self.performance.dns_timing()
    }

    /// Returns the TCP connection time in milliseconds.
    ///
    /// # Returns
    ///
    /// The time taken in milliseconds from operation start to TCP connection completion.
    pub fn connect_time(&self) -> f64 {
        self.performance.connect_timing()
    }

    /// Returns the TLS handshake time in milliseconds.
    ///
    /// # Returns
    ///
    /// The time taken in milliseconds from operation start to TLS connection completion.
    pub fn tls_time(&self) -> f64 {
        self.performance.tls_timing()
    }

    /// Returns the time to first byte sent in milliseconds.
    ///
    /// # Returns
    ///
    /// The time taken in milliseconds from operation start to sending the first byte.
    pub fn first_send_time(&self) -> f64 {
        self.performance.first_send_timing()
    }

    /// Returns the time to first byte received in milliseconds.
    ///
    /// # Returns
    ///
    /// The time taken in milliseconds from operation start to receiving the first byte.
    pub fn first_recv_time(&self) -> f64 {
        self.performance.first_recv_timing()
    }

    /// Returns the total redirection time in milliseconds.
    ///
    /// # Returns
    ///
    /// The time taken in milliseconds from operation start to completion of all redirects.
    pub fn redirect_time(&self) -> f64 {
        self.performance.redirect_timing()
    }

    /// Returns the total request time in milliseconds.
    ///
    /// # Returns
    ///
    /// The time taken in milliseconds from operation start to request completion.
    pub fn total_time(&self) -> f64 {
        self.performance.total_timing()
    }

    /// Returns the resource size in bytes.
    ///
    /// # Returns
    ///
    /// The size of the downloaded resource in bytes, or -1 if unknown.
    pub fn resource_size(&self) -> i64 {
        self.resource.size()
    }

    /// Returns the server address.
    ///
    /// # Returns
    ///
    /// The address of the server that provided the resource.
    pub fn server_addr(&self) -> String {
        self.network.addr()
    }

    /// Returns the list of DNS servers used.
    ///
    /// # Returns
    ///
    /// A vector containing the DNS server addresses used during hostname resolution.
    pub fn dns_servers(&self) -> Vec<String> {
        self.network.dns()
    }
}

/// Tracks the capacity and usage statistics of an information collection.
///
/// Provides methods to manage and query the size and capacity of collections
/// holding download information.
struct InfoListSize {
    /// Total capacity of the list.
    total: u16,
    /// Number of currently used slots.
    used: u16,
}

impl InfoListSize {
    /// Creates a new `InfoListSize` instance with zero capacity.
    ///
    /// Initializes with total and used counts set to zero.
    fn new() -> Self {
        InfoListSize { total: 0, used: 0 }
    }

    /// Attempts to increment the used count.
    ///
    /// # Returns
    ///
    /// * `true` if the used count was successfully incremented.
    /// * `false` if the collection is already at full capacity.
    fn increment(&mut self) -> bool {
        if self.used >= self.total {
            false
        } else {
            self.used += 1;
            true
        }
    }

    /// Attempts to decrement the used count.
    ///
    /// # Returns
    ///
    /// * `true` if the used count was successfully decremented.
    /// * `false` if the collection is already empty or has zero capacity.
    fn release(&mut self) -> bool {
        if self.used == 0 || self.total == 0 {
            false
        } else {
            self.used -= 1;
            true
        }
    }

    /// Returns the total capacity.
    ///
    /// # Returns
    ///
    /// The maximum number of items that can be stored in the collection.
    fn total_size(&self) -> u16 {
        self.total
    }

    /// Checks if the list is at full capacity.
    ///
    /// # Returns
    ///
    /// `true` if the number of used slots equals or exceeds the total capacity.
    fn is_full_capacity(&self) -> bool {
        self.used >= self.total
    }

    /// Updates the total capacity and adjusts used count if necessary.
    ///
    /// # Arguments
    ///
    /// * `total` - The new total capacity for the collection.
    ///
    /// # Returns
    ///
    /// * `Some(overflow)` if the current used count exceeds the new total, where
    ///   `overflow` is the number of excess items that would need to be removed.
    /// * `None` if the used count does not exceed the new total.
    fn update_total_size(&mut self, total: u16) -> Option<u16> {
        self.total = total;
        if self.used > total {
            let overflow = self.used - total;
            self.used = total;
            return Some(overflow);
        }
        None
    }
}

/// Manages a collection of download information with LRU caching behavior.
///
/// Provides methods to store and retrieve download information using task IDs as keys,
/// with least recently used items being evicted when capacity is reached.
struct InfoCollection {
    /// Size tracking for the collection.
    list_size: InfoListSize,
    /// LRU cache holding the download information.
    info_list: LRUCache<TaskId, DownloadInfo>,
}

impl InfoCollection {
    /// Creates a new empty `InfoCollection` instance.
    ///
    /// Initializes with an empty LRU cache and size tracker with zero capacity.
    fn new() -> Self {
        InfoCollection {
            list_size: InfoListSize::new(),
            info_list: LRUCache::new(),
        }
    }

    /// Inserts download information for a specific task.
    ///
    /// Manages capacity constraints by removing the least recently used item if the
    /// collection is full. If the task ID already exists, updates its information.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The identifier of the task to associate with the download info.
    /// * `info` - The download information to store.
    fn insert_info(&mut self, task_id: TaskId, info: DownloadInfo) {
        // Early return if no capacity is configured
        if self.list_size.total_size() == 0 {
            debug!("DownloadInfoMgr insert info failed, total size is 0");
            return;
        }

        // If collection is at capacity, make room by removing an item
        if self.list_size.is_full_capacity() {
            self.list_size.release();
            // Try to remove the specific task ID first, otherwise remove LRU item
            if self.info_list.remove(&task_id).is_none() {
                self.info_list.pop();
            }
        }

        info!("Insert {} info", task_id.brief());
        // Increment usage counter only if this is a new insertion
        if self.info_list.insert(task_id, info).is_none() {
            self.list_size.increment();
        };
    }

    /// Updates the total capacity of the collection.
    ///
    /// If the new capacity is smaller than the current usage, removes excess items
    /// from the LRU cache (evicting least recently used items first).
    ///
    /// # Arguments
    ///
    /// * `total` - The new total capacity for the collection.
    fn update_total_size(&mut self, total: u16) {
        if let Some(overflow) = self.list_size.update_total_size(total) {
            // Remove excess items starting with the least recently used
            for _i in 0..overflow {
                self.info_list.pop();
            }
        }
    }
}

/// Manages a collection of download information with thread-safe access.
///
/// Provides synchronized methods to store, retrieve, and manage download
/// information for multiple tasks in a thread-safe manner.
pub struct DownloadInfoMgr {
    /// Thread-safe wrapper around the information collection.
    info: Mutex<InfoCollection>,
}

impl DownloadInfoMgr {
    /// Creates a new `DownloadInfoMgr` instance with an empty collection.
    ///
    /// Initializes with a new mutex-protected `InfoCollection`.
    ///
    /// # Examples
    ///
    /// ```
    /// let manager = DownloadInfoMgr::new();
    /// // The manager is ready to store and retrieve download information
    /// ```
    pub fn new() -> Self {
        DownloadInfoMgr {
            info: Mutex::new(InfoCollection::new()),
        }
    }

    /// Inserts download information for a specific task.
    ///
    /// This operation is thread-safe and will evict least recently used items if
    /// the collection reaches capacity.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The identifier of the task to associate with the download info.
    /// * `info` - The download information to store.
    ///
    /// # Safety
    ///
    /// This function will panic if the underlying mutex is poisoned.
    pub fn insert_download_info(&self, task_id: TaskId, info: DownloadInfo) {
        let mut info_guard = self.info.lock().unwrap();
        info_guard.insert_info(task_id, info);
    }

    /// Updates the total capacity of the information collection.
    ///
    /// This operation is thread-safe and will evict excess items if the new
    /// capacity is smaller than the current usage.
    ///
    /// # Arguments
    ///
    /// * `size` - The new maximum capacity for the collection.
    ///
    /// # Safety
    ///
    /// This function will panic if the underlying mutex is poisoned.
    pub fn update_info_list_size(&self, size: u16) {
        let mut info_guard = self.info.lock().unwrap();
        info_guard.update_total_size(size);
        info!("DownloadInfoMgr update total size, total size is {}", size);
    }

    /// Retrieves download information for a specific task.
    ///
    /// Returns `None` if no information is found for the given task ID.
    /// This operation is thread-safe and updates the LRU status of the accessed item.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The identifier of the task to retrieve information for.
    ///
    /// # Returns
    ///
    /// * `Some(DownloadInfo)` if information for the task was found.
    /// * `None` if no information is available for the given task ID.
    ///
    /// # Safety
    ///
    /// This function will panic if the underlying mutex is poisoned.
    pub fn get_download_info(&self, task_id: TaskId) -> Option<DownloadInfo> {
        let mut info_guard = self.info.lock().unwrap();
        info_guard.info_list.get(&task_id).cloned()
    }
}

#[cfg(test)]
mod ut_info {
    // Include unit tests from the dedicated test file
    include!("../tests/ut/ut_info.rs");
}
