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

use std::collections::HashMap;

#[ani_rs::ani(path = "L@ohos/request/cacheDownload/cacheDownload/SslType")]
pub enum SslType {
    TLS,
    TLCP,
}

#[ani_rs::ani(path = "L@ohos/request/cacheDownload/cacheDownload/CacheStrategy")]
pub enum CacheStrategy {
    FORCE,
    LAZY,
}

#[ani_rs::ani(path = "L@ohos/request/cacheDownload/cacheDownload/CacheDownloadOptions")]
pub struct CacheDownloadOptions {
    pub headers: Option<HashMap<String, String>>,
    pub ssl_type: Option<SslType>,
    pub ca_path: Option<String>,
    pub cache_strategy: Option<CacheStrategy>,
}

#[ani_rs::ani(path = "L@ohos/request/cacheDownload/cacheDownload/ResourceInfoInner")]
pub struct ResourceInfo {
    pub size: i64,
}

#[ani_rs::ani(path = "L@ohos/request/cacheDownload/cacheDownload/NetworkInfoInner")]
pub struct NetworkInfo {
    pub dns_servers: Vec<String>,
    pub ip: Option<String>,
}

#[ani_rs::ani(path = "L@ohos/request/cacheDownload/cacheDownload/PerformanceInfoInner")]
pub struct PerformanceInfo {
    pub dns_time: f64,
    pub connect_time: f64,
    pub tls_time: f64,
    pub first_send_time: f64,
    pub first_receive_time: f64,
    pub total_time: f64,
    pub redirect_time: f64,
}

#[ani_rs::ani(path = "L@ohos/request/cacheDownload/cacheDownload/DownloadInfoInner")]
pub struct DownloadInfo {
    pub resource: ResourceInfo,
    pub network: NetworkInfo,
    pub performance: PerformanceInfo,
}

impl DownloadInfo {
    pub fn from_native(native_info: preload_native_rlib::info::RustDownloadInfo) -> Self {
        let ip = native_info.server_addr();
        Self {
            resource: ResourceInfo {
                size: native_info.resource_size(),
            },
            network: NetworkInfo {
                dns_servers: native_info.dns_servers(),
                ip: if ip.is_empty() { None } else { Some(ip) },
            },
            performance: PerformanceInfo {
                dns_time: native_info.dns_time(),
                connect_time: native_info.connect_time(),
                tls_time: native_info.tls_time(),
                first_send_time: native_info.first_send_time(),
                first_receive_time: native_info.first_recv_time(),
                total_time: native_info.total_time(),
                redirect_time: native_info.redirect_time(),
            },
        }
    }
}
