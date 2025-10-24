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

//! System proxy configuration management.
//! 
//! This module provides functionality to manage and access system proxy settings
//! for network requests. It interacts with the underlying system to retrieve proxy
//! configuration parameters including host, port, and exclusion lists.

use crate::utils::c_wrapper::CStringWrapper;

/// Manages system proxy settings for network requests.
///
/// Provides access to the system's proxy configuration and subscribes to proxy
/// setting changes. Acts as a wrapper around system-level proxy functionality.
#[derive(Clone)]
pub(crate) struct SystemProxyManager;

impl SystemProxyManager {
    /// Initializes the system proxy manager and registers for proxy setting updates.
    ///
    /// # Returns
    ///
    /// A new instance of `SystemProxyManager` with registered proxy subscription.
    ///
    /// # Safety
    ///
    /// Calls unsafe FFI function `RegisterProxySubscriber()` to subscribe to system
    /// proxy changes.
    pub(crate) fn init() -> Self {
        unsafe {
            RegisterProxySubscriber();
        }
        Self
    }

    /// Retrieves the current proxy host address.
    ///
    /// # Returns
    ///
    /// A `String` containing the proxy host address, or empty string if no proxy is set.
    ///
    /// # Safety
    ///
    /// Calls unsafe FFI function `GetHost()` to retrieve the proxy host from the system.
    pub(crate) fn host(&self) -> String {
        unsafe { GetHost() }.to_string()
    }

    /// Retrieves the current proxy port number.
    ///
    /// # Returns
    ///
    /// A `String` containing the proxy port number, or empty string if no proxy is set.
    ///
    /// # Safety
    ///
    /// Calls unsafe FFI function `GetPort()` to retrieve the proxy port from the system.
    pub(crate) fn port(&self) -> String {
        unsafe { GetPort() }.to_string()
    }

    /// Retrieves the current proxy exclusion list.
    ///
    /// # Returns
    ///
    /// A `String` containing domains/hosts that should bypass the proxy, or empty string
    /// if no exclusions are set.
    ///
    /// # Safety
    ///
    /// Calls unsafe FFI function `GetExclusionList()` to retrieve the proxy exclusion
    /// list from the system.
    pub(crate) fn exlist(&self) -> String {
        unsafe { GetExclusionList() }.to_string()
    }
}

// C API functions for accessing system proxy settings
#[cfg(feature = "oh")]
extern "C" {
    /// Registers a subscriber for system proxy configuration changes.
    ///
    /// # Safety
    ///
    /// This is an FFI function that interacts with system components.
    pub(crate) fn RegisterProxySubscriber();
    
    /// Retrieves the current proxy host address.
    ///
    /// # Returns
    ///
    /// A `CStringWrapper` containing the proxy host address.
    ///
    /// # Safety
    ///
    /// This is an FFI function that interacts with system components.
    pub(crate) fn GetHost() -> CStringWrapper;
    
    /// Retrieves the current proxy port number.
    ///
    /// # Returns
    ///
    /// A `CStringWrapper` containing the proxy port number.
    ///
    /// # Safety
    ///
    /// This is an FFI function that interacts with system components.
    pub(crate) fn GetPort() -> CStringWrapper;
    
    /// Retrieves the current proxy exclusion list.
    ///
    /// # Returns
    ///
    /// A `CStringWrapper` containing the proxy exclusion list.
    ///
    /// # Safety
    ///
    /// This is an FFI function that interacts with system components.
    pub(crate) fn GetExclusionList() -> CStringWrapper;
}
