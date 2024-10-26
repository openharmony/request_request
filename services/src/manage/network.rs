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
use std::sync::{Arc, RwLock};

use cxx::UniquePtr;
use ffi::NetworkRegistry;
pub(crate) use ffi::{NetworkInfo, NetworkType};
use NetworkState::{Offline, Online};

use crate::manage::network_manager::NetworkManager;

cfg_oh! {
    use super::events::TaskManagerEvent;
    use super::task_manager::TaskManagerTx;
}

#[derive(Clone)]
pub struct Network {
    pub(crate) inner: NetworkInner,
    #[cfg(feature = "oh")]
    pub(crate) _registry: Option<Arc<UniquePtr<NetworkRegistry>>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NetworkState {
    Offline,
    Online(NetworkInfo),
}

impl Network {
    pub(crate) fn state(&self) -> NetworkState {
        self.inner.state.read().unwrap().clone()
    }
}

pub(crate) fn register_network_change() {
    const RETRY_TIME: i32 = if cfg!(test) { 1 } else { 20 };
    let mut count: i32 = 0;
    let mut network_manager = NetworkManager::get_instance().lock().unwrap();
    let tx = network_manager.tx.clone();
    if network_manager.network.state() != Offline {
        return;
    }
    match tx {
        Some(tx) => {
            let mut registry: UniquePtr<NetworkRegistry> = UniquePtr::null();
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
                return;
            }
            network_manager.network._registry = Some(Arc::new(registry));
        }
        None => {
            error!("register_network_change failed, tx is None!");
        }
    }
}

#[derive(Clone)]
pub struct NetworkInner {
    state: Arc<RwLock<NetworkState>>,
}

pub struct NetworkTaskManagerTx {
    #[cfg(feature = "oh")]
    inner: TaskManagerTx,
}

impl NetworkInner {
    pub(crate) fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(NetworkState::Offline)),
        }
    }

    pub(crate) fn notify_offline(&self) {
        let mut state = self.state.write().unwrap();
        if *state != Offline {
            info!("network is offline");
            *state = Offline;
        }
    }

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

unsafe impl Send for NetworkRegistry {}
unsafe impl Sync for NetworkRegistry {}

#[allow(unreachable_pub)]
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {

    #[derive(Clone, Eq, PartialEq, Debug)]
    struct NetworkInfo {
        network_type: NetworkType,
        is_metered: bool,
        is_roaming: bool,
    }

    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum NetworkType {
        Other,
        Wifi,
        Cellular,
    }

    extern "Rust" {
        type NetworkInner;
        type NetworkTaskManagerTx;
        fn notify_online(self: &NetworkInner, info: NetworkInfo) -> bool;
        fn notify_offline(self: &NetworkInner);
    }

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

#[cfg(feature = "oh")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::test_init;

    #[test]
    fn ut_network() {
        test_init();
        let notifier;
        {
            let network_manager = NetworkManager::get_instance().lock().unwrap();
            notifier = network_manager.network.inner.clone();
        }
        assert!(!NetworkManager::is_online());

        notifier.notify_online(NetworkInfo {
            network_type: NetworkType::Wifi,
            is_metered: false,
            is_roaming: false,
        });
        assert!(NetworkManager::is_online());
        assert_eq!(
            NetworkManager::query_network(),
            Online(NetworkInfo {
                network_type: NetworkType::Wifi,
                is_metered: false,
                is_roaming: false,
            })
        );
        notifier.notify_offline();
        assert!(!NetworkManager::is_online());
        notifier.notify_online(NetworkInfo {
            network_type: NetworkType::Cellular,
            is_metered: true,
            is_roaming: true,
        });
        assert!(NetworkManager::is_online());
        assert_eq!(
            NetworkManager::query_network(),
            Online(NetworkInfo {
                network_type: NetworkType::Cellular,
                is_metered: true,
                is_roaming: true,
            })
        );
    }

    #[test]
    fn ut_network_notify() {
        test_init();
        let notifier = NetworkInner::new();
        notifier.notify_offline();
        assert!(notifier.notify_online(NetworkInfo {
            network_type: NetworkType::Wifi,
            is_metered: true,
            is_roaming: true,
        }));
        assert!(!notifier.notify_online(NetworkInfo {
            network_type: NetworkType::Wifi,
            is_metered: true,
            is_roaming: true,
        }));
        assert!(notifier.notify_online(NetworkInfo {
            network_type: NetworkType::Wifi,
            is_metered: false,
            is_roaming: true,
        }));
        assert!(notifier.notify_online(NetworkInfo {
            network_type: NetworkType::Cellular,
            is_metered: false,
            is_roaming: true,
        }));
    }
}
