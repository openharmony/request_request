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

#[cfg(test)]
mod ut_info {
    use request_utils::task_id::TaskId;
    use crate::info::{DownloadInfo, DownloadInfoMgr, InfoListSize, RustPerformanceInfo};

    #[test]
    fn ut_download_performance() {
        let mut performance = RustPerformanceInfo::default();
        performance.set_dns_timing(1.0f64);
        performance.set_connect_timing(2.0f64);
        performance.set_tls_timing(3.0f64);
        performance.set_first_send_timing(4.0f64);
        performance.set_first_receive_timing(5.0f64);
        performance.set_total_timing(6.0f64);
        performance.set_redirect_timing(10.0f64);
        let mut download_info = DownloadInfo::new();
        download_info.set_performance(performance);
        assert!(download_info.dns_time() - 1.0f64 < 0.01f64);
        assert!(download_info.connect_time() - 2.0f64 < 0.01f64);
        assert!(download_info.tls_time() - 3.0f64 < 0.01f64);
        assert!(download_info.first_send_time() - 4.0f64 < 0.01f64);
        assert!(download_info.first_recv_time() - 5.0f64 < 0.01f64);
        assert!(download_info.total_time() - 6.0f64 < 0.01f64);
        assert!(download_info.redirect_time() - 10.0f64 < 0.01f64);
    }

    #[test]
    fn ut_download_resource() {
        let mut download_info = DownloadInfo::new();
        assert_eq!(download_info.resource_size(), -1);
        download_info.set_size(0);
        assert_eq!(download_info.resource_size(), 0);
    }

    #[test]
    fn ut_download_net_dns() {
        let mut download_info = DownloadInfo::new();
        assert!(download_info.dns_servers().is_empty());
        download_info.set_network_dns(vec!["4.4.4.4".to_string()]);
        assert!(download_info.ip().is_empty());
        let dns = download_info.dns_servers().pop();
        assert_eq!(dns, Some("4.4.4.4".to_string()));
    }

    #[test]
    fn info_list_size_increment() {
        let mut info_size = InfoListSize::new();
        assert!(info_size.is_full_capacity());
        assert_eq!(info_size.total, 0);
        assert_eq!(info_size.used, 0);
        assert_eq!(info_size.total_size(), 0);
        assert!(!info_size.increment());
        assert!(info_size.update_total_size(1).is_none());
        assert!(info_size.increment());
    }

    #[test]
    fn info_list_size_release() {
        let mut info_size = InfoListSize::new();
        assert!(!info_size.release());
        info_size.update_total_size(1);
        assert_eq!(info_size.total, 1);
        info_size.increment();
        assert!(info_size.release());
    }

    #[test]
    fn info_list_size_update() {
        let mut info_size = InfoListSize::new();
        info_size.update_total_size(2);
        info_size.increment();
        assert_eq!(info_size.update_total_size(1), None);
        assert_eq!(info_size.update_total_size(0), Some(1));
    }

    #[test]
    fn info_collection_update() {
        let info_mgr = DownloadInfoMgr::new();
        let task_id = TaskId::from_url("https://www.example.coom/data/test1");
        let info = DownloadInfo::new();
        info_mgr.insert_download_info(task_id.clone(), info.clone());
        assert!(info_mgr.get_download_info(task_id.clone()).is_none());
        info_mgr.update_info_list_size(1);
        info_mgr.insert_download_info(task_id.clone(), info.clone());
        assert!(info_mgr.get_download_info(task_id.clone()).is_some());
        // Update the same task_id.
        info_mgr.insert_download_info(task_id.clone(), info);
        assert!(info_mgr.get_download_info(task_id.clone()).is_some());
        let task_id_2 = TaskId::from_url("https://www.example.coom/data/test2");
        let info_2 = DownloadInfo::new();
        info_mgr.insert_download_info(task_id_2.clone(), info_2);
        assert!(info_mgr.get_download_info(task_id).is_none());
        assert!(info_mgr.get_download_info(task_id_2).is_some());
    }

}
