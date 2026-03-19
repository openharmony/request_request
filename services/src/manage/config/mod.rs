// Copyright (C) 2023 Huawei Device Co., Ltd.
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

//! System configuration management for the request service.
//!
//! This module provides centralized management of system configurations including
//! certificates and proxy settings for network requests. It combines functionality
//! from specialized managers into a unified interface for system-wide configuration.

#[cfg(feature = "oh")]
mod cert_manager;
mod system_proxy;

#[cfg(feature = "oh")]
pub(crate) use cert_manager::load_certificates_from_paths;
use system_proxy::SystemProxyManager;

/// Manages system-wide configurations for the request service.
///
/// Provides unified access to various system configurations including proxy settings.
/// Combines specialized configuration managers into a single interface for easy access
/// and management.
#[derive(Clone)]
pub(crate) struct SystemConfigManager {
    /// Proxy manager for handling system proxy settings.
    proxy: SystemProxyManager,
}

impl SystemConfigManager {
    /// Initializes a new system configuration manager.
    ///
    /// # Returns
    ///
    /// A new instance of `SystemConfigManager` with initialized proxy manager.
    pub(crate) fn init() -> Self {
        Self {
            proxy: SystemProxyManager::init(),
        }
    }

    /// Retrieves the current system configuration.
    ///
    /// # Returns
    ///
    /// A `SystemConfig` struct containing the current proxy settings.
    pub(crate) fn system_config(&self) -> SystemConfig {
        SystemConfig {
            proxy_host: self.proxy.host(),
            proxy_port: self.proxy.port(),
            proxy_exlist: self.proxy.exlist(),
        }
    }
}

/// Holds system configuration parameters for network requests.
///
/// Contains proxy settings required for making network requests.
/// Certificates are now loaded on-demand in build_client.
pub(crate) struct SystemConfig {
    /// Proxy server hostname or IP address.
    pub(crate) proxy_host: String,
    /// Proxy server port number.
    pub(crate) proxy_port: String,
    /// List of domains or URLs that should bypass the proxy.
    pub(crate) proxy_exlist: String,
}
