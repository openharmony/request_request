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

mod cert_manager;
mod system_proxy;

use cert_manager::CertManager;
use system_proxy::SystemProxyManager;
use ylong_http_client::Certificate;

/// Manages system-wide configurations for the request service.
///
/// Provides unified access to various system configurations including certificates
/// and proxy settings. Combines specialized configuration managers into a single
/// interface for easy access and management.
#[derive(Clone)]
pub(crate) struct SystemConfigManager {
    /// Certificate manager for handling SSL/TLS certificates.
    cert: CertManager,
    /// Proxy manager for handling system proxy settings.
    proxy: SystemProxyManager,
}

impl SystemConfigManager {
    /// Initializes a new system configuration manager.
    ///
    /// # Returns
    ///
    /// A new instance of `SystemConfigManager` with initialized certificate and proxy managers.
    pub(crate) fn init() -> Self {
        Self {
            cert: CertManager::init(),
            proxy: SystemProxyManager::init(),
        }
    }

    /// Retrieves the current system configuration.
    ///
    /// # Returns
    ///
    /// A `SystemConfig` struct containing the current proxy settings and certificates.
    ///
    /// # Notes
    ///
    /// If certificates are not available, this method forces an immediate certificate update
    /// attempt before returning.
    pub(crate) fn system_config(&self) -> SystemConfig {
        let mut certs = self.cert.certificate();

        // Force certificate update if no certificates are available
        if certs.is_none() {
            self.cert.force_update();
            certs = self.cert.certificate();
        }

        SystemConfig {
            proxy_host: self.proxy.host(),
            proxy_port: self.proxy.port(),
            proxy_exlist: self.proxy.exlist(),
            certs,
        }
    }
}

/// Holds system configuration parameters for network requests.
///
/// Contains proxy settings and certificates required for making secure network requests.
pub(crate) struct SystemConfig {
    /// Proxy server hostname or IP address.
    pub(crate) proxy_host: String,
    /// Proxy server port number.
    pub(crate) proxy_port: String,
    /// List of domains or URLs that should bypass the proxy.
    pub(crate) proxy_exlist: String,
    /// SSL/TLS certificates for secure connections, if available.
    pub(crate) certs: Option<Vec<Certificate>>,
}
