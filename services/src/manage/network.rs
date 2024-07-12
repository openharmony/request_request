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
use std::sync::{Arc, RwLock, RwLockReadGuard};

use cxx::UniquePtr;
use ffi::NetworkRegistry;
pub(crate) use ffi::{NetworkInfo, NetworkType};
use NetworkState::{Offline, Online};

use super::database::RequestDb;
use super::events::TaskManagerEvent;
use super::task_manager::TaskManagerTx;
use crate::task::config::{NetworkConfig, TaskConfig};
use crate::task::info::State;
use crate::task::reason::Reason;
use crate::utils::get_current_timestamp;

#[derive(Clone)]
pub struct Network {
    inner: NetworkInner,
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

    pub(crate) fn state(&self) -> RwLockReadGuard<NetworkState> {
        self.inner.state.read().unwrap()
    }

    pub(crate) fn satisfied_state(&self, config: &TaskConfig) -> bool {
        match &*self.state() {
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

pub(crate) fn register_network_change(task_manager: TaskManagerTx) -> Network {
    let inner = NetworkInner::new();
    let registry = ffi::RegisterNetworkChange(
        Box::new(inner.clone()),
        Box::new(NetworkTaskManagerTx {
            inner: task_manager.clone(),
        }),
        |task_manager| {
            task_manager
                .inner
                .send_event(TaskManagerEvent::network_online());
        },
        |task_manager| {
            task_manager
                .inner
                .send_event(TaskManagerEvent::network_offline());
        },
    );
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
        _registry: Arc::new(registry),
    }
}

#[derive(Clone)]
pub struct NetworkInner {
    state: Arc<RwLock<NetworkState>>,
    last_notify_time: Arc<RwLock<u64>>,
}

pub struct NetworkTaskManagerTx {
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
            self.update_database(NetworkState::Offline);
        }
    }

    fn notify_online(&self, info: NetworkInfo) -> bool {
        let mut state = self.state.write().unwrap();
        let ret =
            !matches!(&*state, Online(old_info) if old_info.network_type == info.network_type);
        if !matches!(&*state, Online(old_info) if old_info == &info  ) {
            info!("Network is online: {:?}", info);
            *self.last_notify_time.write().unwrap() = get_current_timestamp();

            *state = Online(info.clone());

            #[cfg(not(test))]
            self.update_database(Online(info));
        } else {
            info!("Network change with the same: {:?}", info);
        }
        ret
    }

    fn update_database(&self, state: NetworkState) {
        let mut database = RequestDb::get_instance();
        match state {
            Offline => database.update_for_network_offline(),
            Online(info) => {
                database.update_for_network_available(&info);
                database.update_for_network_unavailable(&info)
            }
        }
    }
}

impl RequestDb {
    fn update_for_network_available(&mut self, info: &NetworkInfo) {
        let mut sql = format!(
            "UPDATE request_task SET reason = {} WHERE state = {} AND (reason = {} OR reason = {})",
            Reason::RunningTaskMeetLimits.repr,
            State::Waiting.repr,
            Reason::UnsupportedNetworkType.repr,
            Reason::NetworkOffline.repr,
        );

        if info.network_type != NetworkType::Other {
            sql.push_str(&format!(
                " AND (network = {} OR network = 0)",
                info.network_type.repr
            ));
        }

        if info.is_metered {
            sql.push_str(" AND metered = 1");
        }

        if info.is_roaming {
            sql.push_str(" AND roaming = 1");
        }
        if let Err(e) = self.execute_sql(&sql) {
            error!("update_for_network_available sql failed: {}", e);
        };
    }

    fn update_for_network_unavailable(&mut self, info: &NetworkInfo) {
        let mut sql = format!(
            "UPDATE request_task SET state = {}, reason = {} WHERE ((state = {} AND reason = {} ) OR state = {} OR state = {})",
            State::Waiting.repr,
            Reason::UnsupportedNetworkType.repr,
            State::Waiting.repr,
            Reason::RunningTaskMeetLimits.repr,
            State::Running.repr,
            State::Retrying.repr,
        );

        let mut sql_1 = String::new();
        if info.network_type != NetworkType::Other {
            sql_1.push_str(&format!(
                "(network != {} AND network != 0)",
                info.network_type.repr
            ));
        }

        if info.is_metered {
            if !sql_1.is_empty() {
                sql_1.push_str(" OR ");
            }
            sql_1.push_str("metered = 0");
        }

        if info.is_roaming {
            if !sql_1.is_empty() {
                sql_1.push_str(" OR ");
            }
            sql_1.push_str("roaming = 0");
        }
        if !sql_1.is_empty() {
            sql = format!("{} AND ({})", sql, sql_1);
        }
        if let Err(e) = self.execute_sql(&sql) {
            error!("update_for_network_unavailable sql failed: {}", e);
        }
    }

    fn update_for_network_offline(&mut self) {
        let sql = format!(
            "UPDATE request_task SET state = {}, reason = {} WHERE (state = {} AND reason = {}  OR state = {} OR state = {})",
            State::Waiting.repr,
            Reason::UnsupportedNetworkType.repr,
            State::Waiting.repr,
            Reason::RunningTaskMeetLimits.repr,
            State::Running.repr,
            State::Retrying.repr,
        );
        if let Err(e) = self.execute_sql(&sql) {
            error!("update_for_network_offline sql failed: {}", e);
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

#[cfg(test)]
mod test {

    use core::time;
    use std::future::join;

    use ylong_runtime::sync::mpsc;

    use super::*;
    use crate::manage::events::StateEvent;
    use crate::tests::test_init;
    use crate::utils::task_id_generator::TaskIdGenerator;

    #[test]
    fn ut_network() {
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
            *network.state(),
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
            *network.state(),
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
            *network.state(),
            Online(NetworkInfo {
                network_type: NetworkType::Wifi,
                is_metered: false,
                is_roaming: false
            })
        );

        loop {
            if let Ok(msg) = rx.try_recv() {
                assert!(matches!(
                    msg,
                    TaskManagerEvent::State(StateEvent::NetworkOnline)
                ));
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
                    notifier.notify_online(NetworkInfo {
                        network_type: NetworkType::Wifi,
                        is_metered: false,
                        is_roaming: false,
                    });
                }
            )
            .await;
            assert!(!network.check_interval_online().await);
            ylong_runtime::time::sleep(std::time::Duration::from_millis(3000)).await;
            assert!(network.check_interval_online().await);
        });
    }

    #[test]
    fn ut_network_database_available() {
        test_init();
        let task_id = TaskIdGenerator::generate();
        let mut db = RequestDb::get_instance();
        db.execute_sql(&format!(
            "INSERT INTO request_task (task_id, state, reason, network,  metered, roaming) VALUES ({}, {}, {}, {}, 0, 0)",
            task_id,
            State::Waiting.repr,
            Reason::UnsupportedNetworkType.repr,
            NetworkType::Wifi.repr,
        ))
        .unwrap();

        db.update_for_network_available(&NetworkInfo {
            network_type: NetworkType::Wifi,
            is_metered: false,
            is_roaming: false,
        });

        let v = db
            .query_integer(&format!(
                "SELECT task_id from request_task WHERE state = {} AND reason = {}",
                State::Waiting.repr,
                Reason::RunningTaskMeetLimits.repr
            ))
            .unwrap();
        assert!(v.contains(&task_id));
    }

    #[test]
    fn ut_network_database_unavailable() {
        test_init();
        let task_id = TaskIdGenerator::generate();
        let mut db = RequestDb::get_instance();
        db.execute_sql(&format!(
            "INSERT INTO request_task (task_id, state, reason, network, metered, roaming) VALUES ({}, {}, {}, {}, 1, 1)",
            task_id,
            State::Waiting.repr,
            Reason::RunningTaskMeetLimits.repr,
            NetworkType::Wifi.repr,
        ))
        .unwrap();

        db.update_for_network_unavailable(&NetworkInfo {
            network_type: NetworkType::Wifi,
            is_metered: true,
            is_roaming: true,
        });

        let v = db
            .query_integer(&format!(
                "SELECT task_id from request_task WHERE state = {} AND reason = {}",
                State::Waiting.repr,
                Reason::UnsupportedNetworkType.repr
            ))
            .unwrap();
        assert!(!v.contains(&task_id));

        db.update_for_network_unavailable(&NetworkInfo {
            network_type: NetworkType::Cellular,
            is_metered: true,
            is_roaming: true,
        });

        let v = db
            .query_integer(&format!(
                "SELECT task_id from request_task WHERE state = {} AND reason = {}",
                State::Waiting.repr,
                Reason::UnsupportedNetworkType.repr
            ))
            .unwrap();
        assert!(v.contains(&task_id));
    }

    #[test]
    fn ut_network_database_offline() {
        test_init();
        let task_id = TaskIdGenerator::generate();
        let mut db = RequestDb::get_instance();
        db.execute_sql(&format!(
            "INSERT INTO request_task (task_id, state, reason, network, metered, roaming) VALUES ({}, {}, {}, {}, 1, 1)",
            task_id,
            State::Waiting.repr,
            Reason::RunningTaskMeetLimits.repr,
            NetworkType::Wifi.repr,
        ))
        .unwrap();

        db.update_for_network_offline();

        let v = db
            .query_integer(&format!(
                "SELECT task_id from request_task WHERE state = {} AND reason = {}",
                State::Waiting.repr,
                Reason::UnsupportedNetworkType.repr
            ))
            .unwrap();
        assert!(v.contains(&task_id));
    }
}
