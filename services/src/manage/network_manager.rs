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

//! Network management system for request services.
//! 
//! This module provides central management of network connectivity for the request service,
//! including network state monitoring and communication with the task manager.

use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};

use super::network::{NetworkInner, NetworkState};
use super::task_manager::TaskManagerTx;
use crate::manage::network::Network;
use crate::utils::call_once;

/// Central manager for network connectivity and state monitoring.
/// 
/// Manages the network state and provides an interface for the task manager to communicate
/// with the network system.
/// 
/// # Fields
/// 
/// * `network` - The underlying network interface used to monitor connectivity
/// * `tx` - Optional channel to send messages to the task manager
pub(crate) struct NetworkManager {
    pub(crate) network: Network,
    pub(crate) tx: Option<TaskManagerTx>,
}

impl NetworkManager {
    /// Returns the singleton instance of the network manager.
    /// 
    /// Uses lazy initialization with thread-safe synchronization to ensure only
    /// one instance is created during program execution.
    /// 
    /// # Safety
    /// 
    /// Uses unsafe code to access the static mutable variable, but ensures thread safety
    /// through proper synchronization with `Once` and `Mutex`.
    /// 
    /// # Returns
    /// 
    /// Returns a static reference to a `Mutex<NetworkManager>` that can be locked
    /// to access the network manager instance.
    pub(crate) fn get_instance() -> &'static Mutex<NetworkManager> {
        static mut NETWORK_MANAGER: MaybeUninit<Mutex<NetworkManager>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        unsafe {
            call_once(&ONCE, || {
                // Create the network interface and initialize the network manager
                let inner = NetworkInner::new();
                let network = Network {
                    inner,
                    _registry: None,
                };
                let network_manager = NetworkManager { network, tx: None };
                NETWORK_MANAGER.write(Mutex::new(network_manager));
            });
            &*NETWORK_MANAGER.as_ptr()
        }
    }

    /// Checks if the device is currently online.
    /// 
    /// # Returns
    /// 
    /// Returns `true` if the network is in an `Online` state, otherwise `false`.
    /// 
    /// # Panics
    /// 
    /// Panics if the mutex cannot be locked, which typically indicates a deadlock.
    pub(crate) fn is_online() -> bool {
        let network_manager = NetworkManager::get_instance().lock().unwrap();
        matches!(network_manager.network.state(), NetworkState::Online(_))
    }

    /// Queries the current network state.
    /// 
    /// # Returns
    /// 
    /// Returns the current `NetworkState` of the network interface.
    /// 
    /// # Panics
    /// 
    /// Panics if the mutex cannot be locked, which typically indicates a deadlock.
    pub(super) fn query_network() -> NetworkState {
        let network_manager = NetworkManager::get_instance().lock().unwrap();
        network_manager.network.state()
    }
}
