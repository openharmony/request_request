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

//! Network observer wrapper and C++ FFI interface.
//! 
//! This module provides a safe Rust wrapper around the C++ network observation
//! system. It defines the `NetObserverWrapper` that bridges between the C++
//! network events and Rust observers, as well as the FFI interface needed for
//! interop.

use std::sync::{Arc, Mutex};

use ffi::{NetInfo, NetUnregistration};

use super::Observer;

/// Wrapper that adapts Rust network observers to the C++ network event system.
///
/// `NetObserverWrapper` holds a collection of Rust observers and forwards
/// network events received from C++ to all registered observers.
pub struct NetObserverWrapper {
    /// Shared collection of observers to receive network event notifications.
    inner: Arc<Mutex<Vec<Box<dyn Observer>>>>,
}

impl NetObserverWrapper {
    /// Creates a new network observer wrapper with the given collection of observers.
    pub fn new(inner: Arc<Mutex<Vec<Box<dyn Observer>>>>) -> Self {
        Self { inner }
    }
}

impl NetObserverWrapper {
    /// Notifies all observers when a network becomes available.
    ///
    /// # Arguments
    ///
    /// * `net_id` - Identifier of the newly available network
    ///
    /// # Panics
    ///
    /// Panics if the mutex for the observer list is poisoned.
    pub(crate) fn net_available(&self, net_id: i32) {
        let inner = self.inner.lock().unwrap();
        for observer in inner.iter() {
            observer.net_available(net_id);
        }
    }

    /// Notifies all observers when a network is lost.
    ///
    /// # Arguments
    ///
    /// * `net_id` - Identifier of the network that was lost
    ///
    /// # Panics
    ///
    /// Panics if the mutex for the observer list is poisoned.
    pub(crate) fn net_lost(&self, net_id: i32) {
        let inner = self.inner.lock().unwrap();
        for observer in inner.iter() {
            observer.net_lost(net_id);
        }
    }

    /// Notifies all observers when network capabilities change.
    ///
    /// # Arguments
    ///
    /// * `net_id` - Identifier of the network whose capabilities changed
    /// * `net_info` - Updated network information containing new capabilities
    ///
    /// # Panics
    ///
    /// Panics if the mutex for the observer list is poisoned.
    pub(crate) fn net_capability_changed(&self, net_id: i32, net_info: NetInfo) {
        let inner = self.inner.lock().unwrap();
        for observer in inner.iter() {
            observer.net_capability_changed(net_id, &net_info);
        }
    }
}

/// Safety: `NetUnregistration` is safe to send between threads as it
/// doesn't contain any thread-local state.
unsafe impl Send for NetUnregistration {}

/// Safety: `NetUnregistration` can be shared between threads as its
/// unregister method is thread-safe.
unsafe impl Sync for NetUnregistration {}

// C++ FFI bridge for network observation
#[cxx::bridge(namespace = "OHOS::Request")]
pub mod ffi {
    // Network capability types from the NetManagerStandard namespace
    #[namespace = "OHOS::NetManagerStandard"]
    #[derive(Debug)]
    #[repr(i32)]
    enum NetCap {
        NET_CAPABILITY_MMS = 0,
        NET_CAPABILITY_SUPL = 1,
        NET_CAPABILITY_DUN = 2,
        NET_CAPABILITY_IA = 3,
        NET_CAPABILITY_XCAP = 4,
        NET_CAPABILITY_BIP = 5,
        NET_CAPABILITY_NOT_METERED = 11,
        NET_CAPABILITY_INTERNET = 12,
        NET_CAPABILITY_NOT_VPN = 15,
        NET_CAPABILITY_VALIDATED = 16,
        NET_CAPABILITY_PORTAL = 17,
        NET_CAPABILITY_INTERNAL_DEFAULT = 18,
        NET_CAPABILITY_CHECKING_CONNECTIVITY = 31,
        NET_CAPABILITY_END = 32,
    }

    // Network bearer types from the NetManagerStandard namespace
    #[namespace = "OHOS::NetManagerStandard"]
    #[derive(Debug)]
    #[repr(i32)]
    enum NetBearType {
        BEARER_CELLULAR = 0,
        BEARER_WIFI = 1,
        BEARER_BLUETOOTH = 2,
        BEARER_ETHERNET = 3,
        BEARER_VPN = 4,
        BEARER_WIFI_AWARE = 5,
        BEARER_DEFAULT,
    }

    /// Network information containing capabilities and bearer types.
    #[derive(Debug)]
    struct NetInfo {
        /// List of capabilities supported by the network.
        caps: Vec<NetCap>,
        /// Types of network bearers available.
        bear_types: Vec<NetBearType>,
    }

    // Rust functions exposed to C++
    extern "Rust" {
        type NetObserverWrapper;

        fn net_available(&self, net_id: i32);
        fn net_lost(&self, net_id: i32);
        fn net_capability_changed(&self, net_id: i32, net_info: NetInfo);
    }

    // C++ functions and types exposed to Rust
    unsafe extern "C++" {
        include!("net_all_capabilities.h");
        include!("request_utils_network.h");

        #[namespace = "OHOS::NetManagerStandard"]
        type NetCap;
        #[namespace = "OHOS::NetManagerStandard"]
        type NetBearType;

        type NetUnregistration;
        /// Unregisters the network observer from the system.
        fn unregister(self: &NetUnregistration) -> i32;

        /// Registers a network observer with the system.
        ///
        /// Returns a unique pointer to a NetUnregistration object that can be
        /// used to unregister the observer later.
        #[allow(unused)]
        fn RegisterNetObserver(
            wrapper: Box<NetObserverWrapper>,
            error: &mut i32,
        ) -> UniquePtr<NetUnregistration>;
    }
}
