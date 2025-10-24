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

//! Network interface and monitoring system.
//! 
//! This module provides core network functionality for the request service,
//! including network state monitoring, connectivity detection, and integration
//! with the underlying platform's network capabilities.

use std::sync::{Arc, RwLock};

use cxx::UniquePtr;
use ffi::NetworkRegistry;
pub(crate) use ffi::{NetworkInfo, NetworkType};
use NetworkState::{Offline, Online};

use crate::manage::network_manager::NetworkManager;

cfg_oh! {
    // OpenHarmony-specific imports for task management events
    use super::events::TaskManagerEvent;
    use super::task_manager::TaskManagerTx;
}

/// Main interface for network state management.
/// 
/// Provides access to network connectivity information and state monitoring.
/// 
/// # Fields
/// 
/// * `inner` - Internal implementation managing the network state
/// * `_registry` - Platform-specific network registry (only on OpenHarmony)
#[derive(Clone)]
pub struct Network {
    pub(crate) inner: NetworkInner,
    #[cfg(feature = "oh")]
    pub(crate) _registry: Option<Arc<UniquePtr<NetworkRegistry>>>,
}

/// Represents the current state of network connectivity.
/// 
/// Used to determine whether the device has an active network connection
/// and provide details about the connection when available.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NetworkState {
    /// No active network connection available
    Offline,
    /// Connected to a network with the provided network information
    Online(NetworkInfo),
}

impl Network {
    /// Retrieves the current network state.
    /// 
    /// # Returns
    /// 
    /// Returns a cloned copy of the current `NetworkState`.
    /// 
    /// # Panics
    /// 
    /// Panics if the read lock cannot be acquired, which typically indicates a deadlock.
    pub(crate) fn state(&self) -> NetworkState {
        self.inner.state.read().unwrap().clone()
    }
}

/// Registers for network connectivity change notifications.
/// 
/// Establishes a connection with the platform's network change notification system
/// to receive updates when the network state changes.
/// 
/// Retries the registration multiple times if initially unsuccessful,
/// with different retry counts based on whether code is running in test mode.
/// 
/// # Errors
/// 
/// Logs an error and emits a system event if the network registration fails
/// after all retry attempts.
pub(crate) fn register_network_change() {
    // Use shorter retry period in test mode, longer in production
    const RETRY_TIME: i32 = if cfg!(test) { 1 } else { 20 };
    let mut count: i32 = 0;
    let mut network_manager = NetworkManager::get_instance().lock().unwrap();
    let tx = network_manager.tx.clone();
    
    // Early return if already connected
    if network_manager.network.state() != Offline {
        return;
    }
    
    match tx {
        Some(tx) => {
            let mut registry: UniquePtr<NetworkRegistry> = UniquePtr::null();
            
            // Attempt to register for network changes with retry logic
            while count < RETRY_TIME {
                registry = ffi::RegisterNetworkChange(
                    Box::new(network_manager.network.inner.clone()),
                    Box::new(NetworkTaskManagerTx { inner: tx.clone() }),
                    |task_manager| {
                        task_manager.inner.send_event(TaskManagerEvent::network());
                    },
                    |task_manager| {
                        task_manager.inner.send_event(TaskManagerEvent::network());
                    },
                );
                
                if registry.is_null() {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    count += 1;
                    continue;
                }
                break;
            }
            
            if registry.is_null() {
                error!("RegisterNetworkChange failed!");
                sys_event!(
                    ExecFault,
                    DfxCode::NET_CONN_CLIENT_FAULT_02,
                    "RegisterNetworkChange failed!"
                );
                return;
            }
            
            // Store the registry to maintain the connection
            network_manager.network._registry = Some(Arc::new(registry));
        }
        None => {
            error!("register_network_change failed, tx is None!");
            sys_event!(
                ExecFault,
                DfxCode::NET_CONN_CLIENT_FAULT_02,
                "register_network_change failed, tx is None!"
            );
        }
    }
}

/// Internal implementation of network state management.
/// 
/// Handles the actual state storage and state change notifications for the network interface.
#[derive(Clone)]
pub struct NetworkInner {
    state: Arc<RwLock<NetworkState>>,
}

/// Adapter for the task manager to receive network change notifications.
/// 
/// Provides a bridge between the network system and the task manager,
/// allowing network events to trigger task manager actions.
pub struct NetworkTaskManagerTx {
    #[cfg(feature = "oh")]
    inner: TaskManagerTx,
}

impl NetworkInner {
    /// Creates a new network inner state with default offline status.
    pub(crate) fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(NetworkState::Offline)),
        }
    }

    /// Updates the network state to offline and logs the change.
    /// 
    /// Only updates if the current state is not already offline.
    /// 
    /// # Panics
    /// 
    /// Panics if the write lock cannot be acquired, which typically indicates a deadlock.
    pub(crate) fn notify_offline(&self) {
        let mut state = self.state.write().unwrap();
        if *state != Offline {
            info!("network is offline");
            *state = Offline;
        }
    }

    /// Updates the network state to online with the provided network information.
    /// 
    /// Only updates if the network information has changed from the current state.
    /// 
    /// # Arguments
    /// 
    /// * `info` - The new network information to set
    /// 
    /// # Returns
    /// 
    /// Returns `true` if the state was updated, `false` if the state was already correct.
    /// 
    /// # Panics
    /// 
    /// Panics if the write lock cannot be acquired, which typically indicates a deadlock.
    pub(crate) fn notify_online(&self, info: NetworkInfo) -> bool {
        let mut state = self.state.write().unwrap();
        if !matches!(&*state, Online(old_info) if old_info == &info  ) {
            info!("network online {:?}", info);
            *state = Online(info.clone());
            true
        } else {
            false
        }
    }
}

// Safety: NetworkRegistry is thread-safe as it's used via FFI with proper synchronization
unsafe impl Send for NetworkRegistry {}
unsafe impl Sync for NetworkRegistry {}

#[allow(unreachable_pub)]
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    // Network connection information structure
    #[derive(Clone, Eq, PartialEq, Debug)]
    struct NetworkInfo {
        network_type: NetworkType,
        is_metered: bool,
        is_roaming: bool,
    }

    // Types of network connections available
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum NetworkType {
        Other,
        Wifi,
        Cellular,
    }

    // Rust functions exposed to C++
    extern "Rust" {
        type NetworkInner;
        type NetworkTaskManagerTx;
        fn notify_online(self: &NetworkInner, info: NetworkInfo) -> bool;
        fn notify_offline(self: &NetworkInner);
    }

    // C++ functions exposed to Rust
    unsafe extern "C++" {
        include!("network.h");
        include!("c_request_database.h");
        type NetworkRegistry;
        fn RegisterNetworkChange(
            notifier: Box<NetworkInner>,
            task_manager: Box<NetworkTaskManagerTx>,
            notify_online: fn(&NetworkTaskManagerTx),
            notify_offline: fn(&NetworkTaskManagerTx),
        ) -> UniquePtr<NetworkRegistry>;
    }
}
