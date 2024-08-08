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

cfg_oh! {
    use super::events::TaskManagerEvent;
    use super::task_manager::TaskManagerTx;
}

#[derive(Clone)]
pub struct Network {
    inner: NetworkInner,
    #[cfg(feature = "oh")]
    _registry: Arc<UniquePtr<NetworkRegistry>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NetworkState {
    Offline,
    Online(NetworkInfo),
}

impl Network {
    pub(crate) fn is_online(&self) -> bool {
        matches!(*self.inner.state.read().unwrap(), Online(_))
    }

    pub(crate) fn state(&self) -> NetworkState {
        self.inner.state.read().unwrap().clone()
    }
}

pub(crate) fn register_network_change(
    #[cfg(feature = "oh")] task_manager: TaskManagerTx,
) -> Network {
    let inner = NetworkInner::new();
    #[cfg(feature = "oh")]
    let registry = ffi::RegisterNetworkChange(
        Box::new(inner.clone()),
        Box::new(NetworkTaskManagerTx {
            inner: task_manager.clone(),
        }),
        |task_manager| {
            task_manager.inner.send_event(TaskManagerEvent::network());
        },
        |task_manager| {
            task_manager.inner.send_event(TaskManagerEvent::network());
        },
    );
    #[cfg(feature = "oh")]
    if registry.is_null() {
        error!("RegisterNetworkChange failed sleep 1s and retry");
        #[cfg(not(test))]
        {
            std::thread::sleep(std::time::Duration::from_secs(1));
            return register_network_change(task_manager);
        }
    }
    Network {
        inner,
        #[cfg(feature = "oh")]
        _registry: Arc::new(registry),
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
    fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(NetworkState::Offline)),
        }
    }

    fn notify_offline(&self) {
        let mut state = self.state.write().unwrap();
        if *state != Offline {
            info!("network is offline");
            *state = Offline;
        }
    }

    fn notify_online(&self, info: NetworkInfo) -> bool {
        let mut state = self.state.write().unwrap();
        if !matches!(&*state, Online(old_info) if old_info == &info  ) {
            info!("Network is online: {:?}", info);
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

    use core::time;

    use ylong_runtime::sync::mpsc;

    use super::*;
    use crate::manage::events::StateEvent;
    use crate::tests::{test_init, DB_LOCK};

    #[test]
    fn ut_network() {
        test_init();
        let _lock = DB_LOCK.lock().unwrap();

        let notifier = NetworkInner::new();
        let network = Network {
            inner: notifier.clone(),
            _registry: Arc::new(UniquePtr::null()),
        };

        assert!(!network.is_online());

        notifier.notify_online(NetworkInfo {
            network_type: NetworkType::Wifi,
            is_metered: false,
            is_roaming: false,
        });

        assert!(network.is_online());
        assert_eq!(
            network.state(),
            Online(NetworkInfo {
                network_type: NetworkType::Wifi,
                is_metered: false,
                is_roaming: false,
            })
        );
        notifier.notify_offline();

        assert!(!network.is_online());

        notifier.notify_online(NetworkInfo {
            network_type: NetworkType::Cellular,
            is_metered: true,
            is_roaming: true,
        });

        assert!(network.is_online());
        assert_eq!(
            network.state(),
            Online(NetworkInfo {
                network_type: NetworkType::Cellular,
                is_metered: true,
                is_roaming: true,
            })
        );
    }

    #[test]
    fn ut_network_oh() {
        test_init();
        let (tx, mut rx) = mpsc::unbounded_channel();
        let task_manager_tx = TaskManagerTx::new(tx);
        let network = register_network_change(task_manager_tx);
        assert!(network.is_online());
        assert_eq!(
            network.state(),
            Online(NetworkInfo {
                network_type: NetworkType::Wifi,
                is_metered: false,
                is_roaming: false
            })
        );

        loop {
            if let Ok(msg) = rx.try_recv() {
                assert!(matches!(msg, TaskManagerEvent::State(StateEvent::Network)));
                break;
            }
            std::thread::sleep(time::Duration::from_millis(100));
        }
    }

    #[test]
    fn ut_network_notify() {
        test_init();
        let _lock = DB_LOCK.lock().unwrap();

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

        assert!(!notifier.notify_online(NetworkInfo {
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
