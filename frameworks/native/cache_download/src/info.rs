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

use netstack_rs::info::DownloadInfo;

pub struct RustDownloadInfo {
    info: DownloadInfo,
}

impl RustDownloadInfo {
    pub fn dns_time(&self) -> f64 {
        self.info.dns_time()
    }

    pub fn connect_time(&self) -> f64 {
        self.info.connect_time()
    }

    pub fn tls_time(&self) -> f64 {
        self.info.tls_time()
    }

    pub fn first_send_time(&self) -> f64 {
        self.info.first_send_time()
    }

    pub fn first_recv_time(&self) -> f64 {
        self.info.first_recv_time()
    }

    pub fn redirect_time(&self) -> f64 {
        self.info.redirect_time()
    }

    pub fn total_time(&self) -> f64 {
        self.info.total_time()
    }

    pub fn resource_size(&self) -> i64 {
        self.info.resource_size()
    }

    pub fn ip(&self) -> String {
        self.info.ip()
    }

    pub fn dns_servers(&self) -> Vec<String> {
        self.info.dns_servers()
    }

    pub fn from_download_info(info: DownloadInfo) -> Self {
        Self { info }
    }
}
