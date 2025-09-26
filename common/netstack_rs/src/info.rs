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

use std::sync::Mutex;

use request_utils::lru::LRUCache;
use request_utils::task_id::TaskId;
use request_utils::{debug, info};

/// A structure representing performance metrics for network operations.
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
    pub fn set_dns_timing(&mut self, time: f64) {
        self.dns_timing = time;
    }

    /// Sets the TCP connection timing.
    pub fn set_connect_timing(&mut self, time: f64) {
        self.connect_timing = time;
    }

    /// Sets the TLS handshake timing.
    pub fn set_tls_timing(&mut self, time: f64) {
        self.tls_timing = time;
    }

    /// Sets the timing for sending the first byte.
    pub fn set_first_send_timing(&mut self, time: f64) {
        self.first_send_timing = time;
    }

    /// Sets the timing for receiving the first byte.
    pub fn set_first_receive_timing(&mut self, time: f64) {
        self.first_receive_timing = time;
    }

    /// Sets the total request timing.
    pub fn set_total_timing(&mut self, time: f64) {
        self.total_timing = time;
    }

    /// Sets the redirection timing.
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

/// Information about a downloaded resource.
#[derive(Clone)]
struct ResourceInfo {
    /// Size of the resource in bytes. -1 indicates unknown size.
    size: i64,
}

impl ResourceInfo {
    /// Creates a new `ResourceInfo` with unknown size (-1).
    fn new() -> Self {
        ResourceInfo { size: -1 }
    }

    /// Sets the resource size.
    fn set_size(&mut self, size: i64) {
        self.size = size;
    }

    /// Returns the resource size.
    fn size(&self) -> i64 {
        self.size
    }
}

/// Network-related information for a download.
#[derive(Clone)]
struct NetworkInfo {
    /// Server address.
    addr: String,
    /// DNS servers used for resolution.
    dns: Vec<String>,
}

impl NetworkInfo {
    /// Creates a new `NetworkInfo` with empty fields.
    fn new() -> Self {
        NetworkInfo {
            addr: String::new(),
            dns: Vec::new(),
        }
    }

    /// Sets the DNS servers used.
    fn set_dns(&mut self, dns: Vec<String>) {
        self.dns = dns;
    }

    /// Returns a copy of the DNS servers list.
    fn dns(&self) -> Vec<String> {
        self.dns.clone()
    }

    /// Returns a copy of the server address.
    fn addr(&self) -> String {
        self.addr.clone()
    }
}

/// Comprehensive download information including resource, network and performance data.
#[derive(Clone)]
pub struct DownloadInfo {
    resource: ResourceInfo,
    network: NetworkInfo,
    performance: RustPerformanceInfo,
}

impl DownloadInfo {
    /// Creates a new `DownloadInfo` with default values.
    pub(crate) fn new() -> Self {
        Self {
            resource: ResourceInfo::new(),
            network: NetworkInfo::new(),
            performance: RustPerformanceInfo::default(),
        }
    }

    /// Sets the resource size.
    pub(crate) fn set_size(&mut self, size: i64) {
        self.resource.set_size(size);
    }

    /// Sets the performance metrics.
    pub(crate) fn set_performance(&mut self, performance: RustPerformanceInfo) {
        self.performance = performance;
    }

    /// Sets the DNS servers used.
    pub(crate) fn set_network_dns(&mut self, dns: Vec<String>) {
        self.network.set_dns(dns);
    }

    /// Returns the DNS resolution time in milliseconds.
    pub fn dns_time(&self) -> f64 {
        self.performance.dns_timing()
    }

    /// Returns the TCP connection time in milliseconds.
    pub fn connect_time(&self) -> f64 {
        self.performance.connect_timing()
    }

    /// Returns the TLS handshake time in milliseconds.
    pub fn tls_time(&self) -> f64 {
        self.performance.tls_timing()
    }

    /// Returns the time to first byte sent in milliseconds.
    pub fn first_send_time(&self) -> f64 {
        self.performance.first_send_timing()
    }

    /// Returns the time to first byte received in milliseconds.
    pub fn first_recv_time(&self) -> f64 {
        self.performance.first_recv_timing()
    }

    /// Returns the total redirection time in milliseconds.
    pub fn redirect_time(&self) -> f64 {
        self.performance.redirect_timing()
    }

    /// Returns the total request time in milliseconds.
    pub fn total_time(&self) -> f64 {
        self.performance.total_timing()
    }

    /// Returns the resource size in bytes.
    pub fn resource_size(&self) -> i64 {
        self.resource.size()
    }

    /// Returns the server address.
    pub fn server_addr(&self) -> String {
        self.network.addr()
    }

    /// Returns the list of DNS servers used.
    pub fn dns_servers(&self) -> Vec<String> {
        self.network.dns()
    }
}

/// Tracks the size and usage of an information list.
struct InfoListSize {
    /// Total capacity of the list.
    total: u16,
    /// Number of currently used slots.
    used: u16,
}

impl InfoListSize {
    /// Creates a new `InfoListSize` with zero capacity.
    fn new() -> Self {
        InfoListSize { total: 0, used: 0 }
    }

    /// Attempts to increment the used count.
    ///
    /// Returns `true` if successful, `false` if already at capacity.
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
    /// Returns `true` if successful, `false` if already empty.
    fn release(&mut self) -> bool {
        if self.used == 0 || self.total == 0 {
            false
        } else {
            self.used -= 1;
            true
        }
    }

    /// Returns the total capacity.
    fn total_size(&self) -> u16 {
        self.total
    }

    /// Checks if the list is at full capacity.
    fn is_full_capacity(&self) -> bool {
        self.used >= self.total
    }

    /// Updates the total capacity and adjusts used count if necessary.
    ///
    /// Returns `Some(overflow)` if the used count exceeds the new total,
    /// where `overflow` is the number of excess items.
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

/// Manages a collection of download information with LRU caching.
struct InfoCollection {
    /// Size tracking for the collection.
    list_size: InfoListSize,
    /// LRU cache holding the download information.
    info_list: LRUCache<TaskId, DownloadInfo>,
}

impl InfoCollection {
    /// Creates a new empty `InfoCollection`.
    fn new() -> Self {
        InfoCollection {
            list_size: InfoListSize::new(),
            info_list: LRUCache::new(),
        }
    }

    /// Inserts download information for a task.
    ///
    /// If the collection is full, removes the least recently used item.
    fn insert_info(&mut self, task_id: TaskId, info: DownloadInfo) {
        if self.list_size.total_size() == 0 {
            debug!("DownloadInfoMgr insert info failed, total size is 0");
            return;
        }
        if self.list_size.is_full_capacity() {
            self.list_size.release();
            if self.info_list.remove(&task_id).is_none() {
                self.info_list.pop();
            }
        }
        info!("DownloadInfoMgr insert task {} info", task_id.brief());
        if self.info_list.insert(task_id, info).is_none() {
            self.list_size.increment();
        };
    }

    /// Updates the total capacity of the collection.
    ///
    /// If the new capacity is smaller than the current usage,
    /// removes excess items from the LRU cache.
    fn update_total_size(&mut self, total: u16) {
        if let Some(overflow) = self.list_size.update_total_size(total) {
            for _i in 0..overflow {
                self.info_list.pop();
            }
        }
    }
}

/// Manager for download information with thread-safe access.
pub struct DownloadInfoMgr {
    /// Thread-safe wrapper around the information collection.
    info: Mutex<InfoCollection>,
}

impl DownloadInfoMgr {
    /// Creates a new `DownloadInfoMgr` with empty collection.
    pub fn new() -> Self {
        DownloadInfoMgr {
            info: Mutex::new(InfoCollection::new()),
        }
    }

    /// Inserts download information for a task.
    ///
    /// This operation is thread-safe.
    pub fn insert_download_info(&self, task_id: TaskId, info: DownloadInfo) {
        let mut info_guard = self.info.lock().unwrap();
        info_guard.insert_info(task_id, info);
    }

    /// Updates the total capacity of the information collection.
    ///
    /// This operation is thread-safe.
    pub fn update_info_list_size(&self, size: u16) {
        let mut info_guard = self.info.lock().unwrap();
        info_guard.update_total_size(size);
        info!("DownloadInfoMgr update total size, total size is {}", size);
    }

    /// Retrieves download information for a task.
    ///
    /// Returns `None` if the task ID is not found.
    /// This operation is thread-safe.
    pub fn get_download_info(&self, task_id: TaskId) -> Option<DownloadInfo> {
        let mut info_guard = self.info.lock().unwrap();
        info_guard.info_list.get(&task_id).cloned()
    }
}

#[cfg(test)]
mod ut_info {
    include!("../tests/ut/ut_info.rs");
}
