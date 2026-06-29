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

use preload_native_rlib::{CacheDownloadError, ErrorKind};

/// SSL/TLS variant to use for a cache download request.
#[ani_rs::ani(path = "@ohos.request.cacheDownload.cacheDownload.SslType")]
pub enum SslType {
    /// Standard TLS protocol.
    TLS,
    /// Transport Layer Cryptographic Protocol (Chinese national standard).
    TLCP,
}

/// Cache lookup strategy for a download request.
#[ani_rs::ani(path = "@ohos.request.cacheDownload.cacheDownload.CacheStrategy")]
#[derive(PartialEq)]
pub enum CacheStrategy {
    /// Force a cache update by re-downloading the resource.
    FORCE,
    /// Download lazily, only fetching when the resource is requested.
    LAZY,
}

/// Per-request retry options for a cache download.
#[ani_rs::ani(path = "@ohos.request.cacheDownload.cacheDownload.RetryOptions")]
pub struct RetryOptions {
    /// Maximum number of retry attempts on failure.
    pub max_retry_count: Option<i32>,
}

/// Per-request timeout options for a cache download.
#[ani_rs::ani(path = "@ohos.request.cacheDownload.cacheDownload.TimeoutOptions")]
pub struct TimeoutOptions {
    /// Timeout in seconds for network reachability checks.
    pub network_check_timeout: Option<i32>,
    /// Overall HTTP request timeout in seconds.
    pub http_total_timeout: Option<i32>,
}

/// Configuration options for a cache download request.
#[ani_rs::ani(path = "@ohos.request.cacheDownload.cacheDownload.CacheDownloadOptions")]
pub struct CacheDownloadOptions {
    /// Optional HTTP headers to send with the request.
    pub headers: Option<HashMap<String, String>>,
    /// Optional SSL/TLS protocol variant.
    pub ssl_type: Option<SslType>,
    /// Optional path to a custom CA certificate file.
    pub ca_path: Option<String>,
    /// Optional cache lookup strategy.
    pub cache_strategy: Option<CacheStrategy>,
    /// Optional per-request retry settings.
    pub retry: Option<RetryOptions>,
    /// Optional per-request timeout settings.
    pub timeout: Option<TimeoutOptions>,
}

/// Information about the downloaded resource itself.
#[ani_rs::ani(path = "@ohos.request.cacheDownload.cacheDownload.ResourceInfoInner")]
pub struct ResourceInfo {
    /// Size of the downloaded resource in bytes.
    pub size: i64,
}

/// Network-related information collected during the download.
#[ani_rs::ani(path = "@ohos.request.cacheDownload.cacheDownload.NetworkInfoInner")]
pub struct NetworkInfo {
    /// List of DNS server addresses used for resolution.
    pub dns_servers: Vec<String>,
    /// Resolved server IP address, if available.
    pub ip: Option<String>,
}

/// Timing measurements for the phases of a download.
#[ani_rs::ani(path = "@ohos.request.cacheDownload.cacheDownload.PerformanceInfoInner")]
pub struct PerformanceInfo {
    /// Time spent on DNS resolution, in milliseconds.
    pub dns_time: f64,
    /// Time spent establishing the TCP connection, in milliseconds.
    pub connect_time: f64,
    /// Time spent on the TLS handshake, in milliseconds.
    pub tls_time: f64,
    /// Time from connection ready to first byte sent, in milliseconds.
    pub first_send_time: f64,
    /// Time from connection ready to first byte received, in milliseconds.
    pub first_receive_time: f64,
    /// Total time of the download, in milliseconds.
    pub total_time: f64,
    /// Time spent following redirects, in milliseconds.
    pub redirect_time: f64,
}

/// Aggregated download information returned to the caller.
#[ani_rs::ani(path = "@ohos.request.cacheDownload.cacheDownload.DownloadInfoInner")]
pub struct DownloadInfo {
    /// Information about the downloaded resource.
    pub resource: ResourceInfo,
    /// Network information gathered during the download.
    pub network: NetworkInfo,
    /// Performance timing measurements.
    pub performance: PerformanceInfo,
}

impl DownloadInfo {
    /// Builds a `DownloadInfo` from the native download info structure.
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

/// Categories of errors that can occur during a cache download.
#[derive(Clone)]
#[ani_rs::ani(path = "@ohos.request.cacheDownload.cacheDownload.ErrorCode")]
pub enum ErrorCode {
    /// Error that does not fit any other category.
    Others = 0xFF,
    /// DNS resolution failure.
    Dns = 0x00,
    /// TCP connection failure.
    Tcp = 0x10,
    /// SSL/TLS handshake or protocol failure.
    Ssl = 0x20,
    /// HTTP-level error (e.g. non-success status code).
    Http = 0x30,
}

/// Error details reported to the caller when a cache download fails.
#[derive(Clone)]
#[ani_rs::ani(path = "@ohos.request.cacheDownload.cacheDownload.DownloadErrorInner")]
pub struct DownloadError {
    /// Category of the download error.
    pub error_code: ErrorCode,
    /// Human-readable error message.
    pub message: String,
}

impl DownloadError {
    /// Converts a native cache download error into the ETS-facing form.
    pub fn from_native(error: CacheDownloadError) -> Self {
        Self {
            error_code: match error.kind() {
                ErrorKind::Dns => ErrorCode::Dns,
                ErrorKind::Tcp => ErrorCode::Tcp,
                ErrorKind::Ssl => ErrorCode::Ssl,
                ErrorKind::Http => ErrorCode::Http,
                _ => ErrorCode::Others,
            },
            message: error.message().to_string(),
        }
    }
}
