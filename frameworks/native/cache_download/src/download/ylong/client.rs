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

//! HTTP client configuration for the Ylong downloader implementation.
//! 
//! This module provides a singleton HTTP client instance with optimized configuration
//! for cache download operations, including timeout settings, TLS configuration, and
//! redirect handling.

use std::sync::LazyLock;

use ylong_http_client::async_impl::Client;
use ylong_http_client::{Redirect, Timeout, TlsVersion};

/// Timeout for establishing a connection (in seconds).
const CONNECT_TIMEOUT: u64 = 60;

/// Maximum request timeout value (one week in seconds).
///
/// Used as the upper limit for long-running download operations to avoid premature
/// timeouts during large file downloads.
const SECONDS_IN_ONE_WEEK: u64 = 7 * 24 * 60 * 60;

/// Creates and returns a singleton HTTP client instance with optimized configuration.
///
/// Returns a static reference to a lazily initialized HTTP client with configuration
/// optimized for cache download operations. The client is configured with:
/// - 60-second connection timeout
/// - One-week request timeout for long-running downloads
/// - TLS 1.2 or higher
/// - Unlimited redirects
/// - Built-in root certificates for TLS validation
///
/// # Returns
/// A static reference to the configured HTTP client
///
/// # Panics
///
/// Panics if the client cannot be built, which could occur due to invalid configuration
/// or system limitations.
pub(crate) fn client() -> &'static Client {
    // Use LazyLock to create the client only once and share it across threads
    static CLIENT: LazyLock<Client> = LazyLock::new(|| {
        let client = Client::builder()
            // Set connection timeout to prevent hanging connections
            .connect_timeout(Timeout::from_secs(CONNECT_TIMEOUT))
            // Set very long request timeout to accommodate large file downloads
            .request_timeout(Timeout::from_secs(SECONDS_IN_ONE_WEEK))
            // Enforce minimum TLS version for security
            .min_tls_version(TlsVersion::TLS_1_2)
            // Allow unlimited redirects for maximum compatibility
            .redirect(Redirect::limited(usize::MAX))
            // Use system's built-in root certificates for TLS validation
            .tls_built_in_root_certs(true);
        client.build().unwrap()
    });
    &CLIENT
}
