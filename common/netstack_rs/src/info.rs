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

#[derive(Clone, Copy, Default)]
pub struct RustPerformanceInfo {
    // Time taken from startup to DNS resolution completion, in milliseconds.
    dns_timing: f64,
    // Time taken from startup to TCP connection completion, in milliseconds.
    connect_timing: f64,
    // Time taken from startup to TLS connection completion, in milliseconds.
    tls_timing: f64,
    // Time taken from startup to start sending the first byte, in milliseconds.
    first_send_timing: f64,
    // Time taken from startup to receiving the first byte, in milliseconds.
    first_receive_timing: f64,
    // Time taken from startup to the completion of the request, in milliseconds.
    total_timing: f64,
    // Time taken from startup to completion of all redirection steps, in milliseconds.
    redirect_timing: f64,
}

impl RustPerformanceInfo {
    pub fn set_dns_timing(&mut self, time: f64) {
        self.dns_timing = time;
    }

    pub fn set_connect_timing(&mut self, time: f64) {
        self.connect_timing = time;
    }

    pub fn set_tls_timing(&mut self, time: f64) {
        self.tls_timing = time;
    }

    pub fn set_first_send_timing(&mut self, time: f64) {
        self.first_send_timing = time;
    }

    pub fn set_first_receive_timing(&mut self, time: f64) {
        self.first_receive_timing = time;
    }

    pub fn set_total_timing(&mut self, time: f64) {
        self.total_timing = time;
    }

    pub fn set_redirect_timing(&mut self, time: f64) {
        self.redirect_timing = time;
    }
}

#[derive(Clone)]
pub struct ResourceInfo {
    pub size: i64,
}

impl ResourceInfo {
    pub fn new() -> Self {
        ResourceInfo { size: -1 }
    }

    pub fn set_size(&mut self, size: i64) {
        self.size = size;
    }
}

#[derive(Clone)]
pub struct NetworkInfo {
    pub ip: String,
    pub dns: Vec<String>,
}

impl NetworkInfo {
    pub fn new() -> Self {
        NetworkInfo {
            ip: String::new(),
            dns: Vec::new(),
        }
    }

    pub fn set_dns(&mut self, dns: Vec<String>) {
        self.dns = dns;
    }
}

#[derive(Clone)]
pub struct DownloadInfo {
    pub resource: ResourceInfo,
    pub network: NetworkInfo,
    pub performance: RustPerformanceInfo,
}

impl DownloadInfo {
    pub fn new() -> Self {
        Self {
            resource: ResourceInfo::new(),
            network: NetworkInfo::new(),
            performance: RustPerformanceInfo::default(),
        }
    }

    pub fn set_size(&mut self, size: i64) {
        self.resource.set_size(size);
    }

    pub fn set_performance(&mut self, performance: RustPerformanceInfo) {
        self.performance = performance;
    }

    pub fn set_network_dns(&mut self, dns: Vec<String>) {
        self.network.set_dns(dns);
    }

    pub fn dns_time(&self) -> f64 {
        self.performance.dns_timing
    }

    pub fn connect_time(&self) -> f64 {
        self.performance.connect_timing
    }

    pub fn tls_time(&self) -> f64 {
        self.performance.tls_timing
    }

    pub fn first_send_time(&self) -> f64 {
        self.performance.first_send_timing
    }

    pub fn first_recv_time(&self) -> f64 {
        self.performance.first_receive_timing
    }

    pub fn redirect_time(&self) -> f64 {
        self.performance.redirect_timing
    }

    pub fn total_time(&self) -> f64 {
        self.performance.total_timing
    }

    pub fn resource_size(&self) -> i64 {
        self.resource.size
    }

    pub fn ip(&self) -> String {
        self.network.ip.clone()
    }

    pub fn dns_servers(&self) -> Vec<String> {
        self.network.dns.clone()
    }
}

pub struct InfoListSize {
    total: u16,
    used: u16,
}

impl InfoListSize {
    pub fn new() -> Self {
        InfoListSize { total: 0, used: 0 }
    }

    pub fn increment(&mut self) -> bool {
        if self.used >= self.total {
            false
        } else {
            self.used += 1;
            true
        }
    }

    pub fn release(&mut self) -> bool {
        if self.used == 0 || self.total == 0 {
            false
        } else {
            self.used -= 1;
            true
        }
    }

    pub fn total_size(&self) -> u16 {
        self.total
    }

    pub fn is_full_capacity(&self) -> bool {
        self.used >= self.total
    }

    pub fn update_total_size(&mut self, total: u16) -> Option<u16> {
        self.total = total;
        if self.used > total {
            let overflow = self.used - total;
            self.used = total;
            return Some(overflow);
        }
        None
    }
}

pub struct InfoCollection {
    list_size: InfoListSize,
    info_list: LRUCache<TaskId, DownloadInfo>,
}

impl InfoCollection {
    pub fn new() -> Self {
        InfoCollection {
            list_size: InfoListSize::new(),
            info_list: LRUCache::new(),
        }
    }

    pub fn insert_info(&mut self, task_id: TaskId, info: DownloadInfo) {
        if self.list_size.total_size() == 0 {
            debug!("DownloadInfoMgr insert info failed, total sizi is 0");
            return;
        }
        if self.list_size.is_full_capacity() {
            self.list_size.release();
            if self.info_list.remove(&task_id).is_none() {
                self.info_list.pop();
            }
        }
        if self.info_list.insert(task_id, info).is_none() {
            self.list_size.increment();
        };
    }

    pub fn update_total_size(&mut self, total: u16) {
        if let Some(overflow) = self.list_size.update_total_size(total) {
            for _i in 0..overflow {
                self.info_list.pop();
            }
        }
    }
}

pub struct DownloadInfoMgr {
    info: Mutex<InfoCollection>,
}

impl DownloadInfoMgr {
    pub fn new() -> Self {
        DownloadInfoMgr {
            info: Mutex::new(InfoCollection::new()),
        }
    }

    pub fn insert_download_info(&self, task_id: TaskId, info: DownloadInfo) {
        let mut info_guard = self.info.lock().unwrap();
        info_guard.insert_info(task_id, info);
    }

    pub fn update_info_list_size(&self, size: u16) {
        let mut info_guard = self.info.lock().unwrap();
        info_guard.update_total_size(size);
        info!("DownloadInfoMgr update total size, total size is {}", size);
    }

    pub fn get_download_info(&self, task_id: TaskId) -> Option<DownloadInfo> {
        let mut info_guard = self.info.lock().unwrap();
        info_guard.info_list.get(&task_id).cloned()
    }
}
