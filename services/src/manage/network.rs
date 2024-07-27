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
cfg_not_oh! {
    use mockall::automock;
}

use crate::task::config::{NetworkConfig, TaskConfig};
use crate::utils::get_current_timestamp;

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
    #![allow(unused)]
    pub(crate) fn new() -> Self {
        Self {
            inner: NetworkInner::new(),
            #[cfg(feature = "oh")]
            _registry: Arc::new(UniquePtr::null()),
        }
    }

    pub(crate) fn is_online(&self) -> bool {
        matches!(*self.inner.state.read().unwrap(), Online(_))
    }

    pub(crate) fn state(&self) -> NetworkState {
        self.inner.state.read().unwrap().clone()
    }

    pub(crate) fn satisfied_state(&self, config: &TaskConfig) -> bool {
        match self.state() {
            // Handles in `RequestTask::network_online`.
            NetworkState::Offline => true,
            NetworkState::Online(info) => match config.common_data.network_config {
                NetworkConfig::Any => true,
                NetworkConfig::Wifi if info.network_type == NetworkType::Cellular => false,
                NetworkConfig::Cellular if info.network_type == NetworkType::Wifi => false,
                _ => {
                    (config.common_data.roaming || !info.is_roaming)
                        && (config.common_data.metered || !info.is_metered)
                }
            },
        }
    }

    pub(crate) async fn check_interval_online(&self) -> bool {
        const NOTIFY_INTERVAL: u64 = 3000;
        let current_time = get_current_timestamp();
        if current_time - self.last_notify_time() < NOTIFY_INTERVAL {
            return false;
        }

        for _ in 0..3 {
            if !self.is_online() || self.last_notify_time() > current_time {
                return false;
            }
            ylong_runtime::time::sleep(std::time::Duration::from_secs(1)).await;
        }
        true
    }

    fn last_notify_time(&self) -> u64 {
        *self.inner.last_notify_time.read().unwrap()
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
    last_notify_time: Arc<RwLock<u64>>,
}

pub struct NetworkTaskManagerTx {
    #[cfg(feature = "oh")]
    inner: TaskManagerTx,
}

impl NetworkInner {
    fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(NetworkState::Offline)),
            last_notify_time: Arc::new(RwLock::new(get_current_timestamp())),
        }
    }

    fn notify_offline(&self) {
        let mut state = self.state.write().unwrap();
        if *state != Offline {
            info!("network is offline");
            *self.last_notify_time.write().unwrap() = get_current_timestamp();
            *state = Offline;
        }
    }

    fn notify_online(&self, info: NetworkInfo) -> bool {
        let mut state = self.state.write().unwrap();
        if !matches!(&*state, Online(old_info) if old_info == &info  ) {
            info!("Network is online: {:?}", info);
            *self.last_notify_time.write().unwrap() = get_current_timestamp();
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
    use std::future::join;

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
    fn ut_network_satisfied_state() {
        let mut config = TaskConfig::default();
        config.common_data.network_config = NetworkConfig::Cellular;
        config.common_data.roaming = true;
        config.common_data.metered = true;

        let notifier = NetworkInner::new();
        let network = Network {
            inner: notifier.clone(),
            _registry: Arc::new(UniquePtr::null()),
        };

        notifier.notify_online(NetworkInfo {
            network_type: NetworkType::Cellular,
            is_metered: true,
            is_roaming: true,
        });

        assert!(network.satisfied_state(&config));

        config.common_data.roaming = false;
        assert!(!network.satisfied_state(&config));

        config.common_data.roaming = true;
        config.common_data.metered = false;
        assert!(!network.satisfied_state(&config));

        config.common_data.metered = true;
        notifier.notify_online(NetworkInfo {
            network_type: NetworkType::Wifi,
            is_metered: false,
            is_roaming: false,
        });
        assert!(!network.satisfied_state(&config));

        config.common_data.network_config = NetworkConfig::Wifi;
        assert!(network.satisfied_state(&config));
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

    #[test]
    fn ut_network_online_interval() {
        test_init();

        ylong_runtime::block_on(async {
            let notifier = NetworkInner::new();

            let network = Network {
                inner: notifier.clone(),
                _registry: Arc::new(UniquePtr::null()),
            };
            assert!(!network.check_interval_online().await);
            join!(
                async {
                    assert!(!network.check_interval_online().await);
                },
                async {
                    ylong_runtime::time::sleep(std::time::Duration::from_millis(500)).await;
                    let _lock = DB_LOCK.lock().unwrap();
                    notifier.notify_online(NetworkInfo {
                        network_type: NetworkType::Wifi,
                        is_metered: false,
                        is_roaming: false,
                    });
                }
            )
            .await;
            assert!(!network.check_interval_online().await);
            ylong_runtime::time::sleep(std::time::Duration::from_millis(4000)).await;
            assert!(network.check_interval_online().await);
        });
    }
}
