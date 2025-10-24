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

//! Download information structures and utilities.
//! 
//! This module defines structures for accessing detailed download metrics and connection
//! information from completed download operations.

use netstack_rs::info::DownloadInfo;

/// Rust wrapper for download information and metrics.
///
/// Provides access to detailed timing and resource information about completed download
/// operations.
pub struct RustDownloadInfo {
    /// Underlying download information from netstack.
    info: DownloadInfo,
}

impl RustDownloadInfo {
    /// Returns the DNS resolution time in seconds.
    ///
    /// # Returns
    /// The time taken to resolve the domain name, in seconds.
    pub fn dns_time(&self) -> f64 {
        self.info.dns_time()
    }

    /// Returns the connection establishment time in seconds.
    ///
    /// # Returns
    /// The time taken to establish the TCP connection, in seconds.
    pub fn connect_time(&self) -> f64 {
        self.info.connect_time()
    }

    /// Returns the TLS handshake time in seconds.
    ///
    /// # Returns
    /// The time taken to complete the TLS handshake, in seconds.
    pub fn tls_time(&self) -> f64 {
        self.info.tls_time()
    }

    /// Returns the time to first byte sent in seconds.
    ///
    /// # Returns
    /// The time taken to send the first byte of the request, in seconds.
    pub fn first_send_time(&self) -> f64 {
        self.info.first_send_time()
    }

    /// Returns the time to first byte received in seconds.
    ///
    /// # Returns
    /// The time taken to receive the first byte of the response, in seconds.
    pub fn first_recv_time(&self) -> f64 {
        self.info.first_recv_time()
    }

    /// Returns the time spent on redirects in seconds.
    ///
    /// # Returns
    /// The total time spent processing redirects, in seconds.
    pub fn redirect_time(&self) -> f64 {
        self.info.redirect_time()
    }

    /// Returns the total download time in seconds.
    ///
    /// # Returns
    /// The total time from request initiation to completion, in seconds.
    pub fn total_time(&self) -> f64 {
        self.info.total_time()
    }

    /// Returns the total resource size in bytes.
    ///
    /// # Returns
    /// The size of the downloaded resource in bytes.
    pub fn resource_size(&self) -> i64 {
        self.info.resource_size()
    }

    /// Returns the server address.
    ///
    /// # Returns
    /// A string representing the server address.
    pub fn server_addr(&self) -> String {
        self.info.server_addr()
    }

    /// Returns the list of DNS servers used.
    ///
    /// # Returns
    /// A vector of strings representing the DNS servers used for resolution.
    pub fn dns_servers(&self) -> Vec<String> {
        self.info.dns_servers()
    }

    /// Creates a new `RustDownloadInfo` from a `DownloadInfo`.
    ///
    /// # Parameters
    /// - `info`: The download information to wrap.
    ///
    /// # Returns
    /// A new `RustDownloadInfo` instance.
    pub fn from_download_info(info: DownloadInfo) -> Self {
        Self { info }
    }
}
