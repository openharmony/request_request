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

use crate::info::State;
use crate::service::client::ClientManagerEntry;
use crate::task::notify::{NotifyData, SubscribeType};

pub(crate) struct Notifier;

impl Notifier {
    pub(crate) fn complete(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        #[cfg(feature = "oh")]
        let _ = publish_state_change_event(
            notify_data.bundle.as_str(),
            notify_data.task_id,
            State::Completed.repr as i32,
            notify_data.uid,
        );
        client_manager.send_notify_data(SubscribeType::Complete, notify_data)
    }

    pub(crate) fn fail(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        #[cfg(feature = "oh")]
        let _ = publish_state_change_event(
            notify_data.bundle.as_str(),
            notify_data.task_id,
            State::Failed.repr as i32,
            notify_data.uid,
        );
        client_manager.send_notify_data(SubscribeType::Fail, notify_data)
    }

    pub(crate) fn pause(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        client_manager.send_notify_data(SubscribeType::Pause, notify_data)
    }

    pub(crate) fn resume(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        client_manager.send_notify_data(SubscribeType::Resume, notify_data)
    }

    pub(crate) fn header_receive(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        client_manager.send_notify_data(SubscribeType::HeaderReceive, notify_data)
    }

    pub(crate) fn progress(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        let total_processed = notify_data.progress.common_data.total_processed;
        let file_total_size: i64 = notify_data.progress.sizes.iter().sum();
        if total_processed == 0 && file_total_size < 0 {
            return;
        }
        client_manager.send_notify_data(SubscribeType::Progress, notify_data)
    }

    pub(crate) fn remove(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        let task_id = notify_data.task_id;
        client_manager.send_notify_data(SubscribeType::Remove, notify_data);
        client_manager.notify_task_finished(task_id);
    }
}

#[cfg(feature = "oh")]
pub(crate) fn publish_state_change_event(
    bundle_name: &str,
    task_id: u32,
    state: i32,
    uid: u64,
) -> Result<(), ()> {
    match crate::utils::PublishStateChangeEvent(bundle_name, task_id, state, uid as i32) {
        true => Ok(()),
        false => Err(()),
    }
}
#[allow(unused)]
#[cfg(test)]
mod test {
    use std::fs::File;
    use std::sync::Arc;
    use std::time::Duration;

    use cxx::UniquePtr;
    use ylong_runtime::sync::mpsc::{unbounded_channel, UnboundedReceiver};

    use crate::config::{Action, ConfigBuilder, Mode};
    use crate::error::ErrorCode;
    use crate::info::{State, TaskInfo};
    use crate::manage::database::RequestDb;
    use crate::manage::events::{TaskEvent, TaskManagerEvent};
    use crate::manage::network::{Network, NetworkInfo, NetworkInner, NetworkState, NetworkType};
    use crate::manage::network_manager::NetworkManager;
    use crate::manage::task_manager::{TaskManagerRx, TaskManagerTx};
    use crate::manage::TaskManager;
    use crate::service::client::{ClientEvent, ClientManager, ClientManagerEntry};
    use crate::service::run_count::RunCountManagerEntry;
    use crate::task::notify::SubscribeType;
    use crate::task::reason::Reason;
    use crate::tests::{lock_database, test_init};

    const GITEE_FILE_LEN: usize = 1042003;

    fn init_manager() -> (TaskManager, UnboundedReceiver<ClientEvent>) {
        let (tx, rx) = unbounded_channel();
        let task_manager_tx = TaskManagerTx::new(tx);
        let rx = TaskManagerRx::new(rx);
        {
            let network_manager = NetworkManager::get_instance().lock().unwrap();
            let notifier = network_manager.network.inner.clone();
            notifier.notify_online(NetworkInfo {
                network_type: NetworkType::Wifi,
                is_metered: false,
                is_roaming: false,
            });
        }
        let (tx, _rx) = unbounded_channel();
        let run_count = RunCountManagerEntry::new(tx);
        let (tx, client_rx) = unbounded_channel();
        let client = ClientManagerEntry::new(tx);
        (
            TaskManager::new(task_manager_tx, rx, run_count, client),
            client_rx,
        )
    }

    #[cfg(feature = "oh")]
    #[test]
    fn ut_network() {
        test_init();
        let notifier;
        {
            let network_manager = NetworkManager::get_instance().lock().unwrap();
            notifier = network_manager.network.inner.clone();
        }

        notifier.notify_online(NetworkInfo {
            network_type: NetworkType::Wifi,
            is_metered: false,
            is_roaming: false,
        });
        assert!(NetworkManager::is_online());
        assert_eq!(
            NetworkManager::query_network(),
            NetworkState::Online(NetworkInfo {
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
            NetworkState::Online(NetworkInfo {
                network_type: NetworkType::Cellular,
                is_metered: true,
                is_roaming: true,
            })
        );
    }

    #[cfg(feature = "oh")]
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

    #[test]
    fn ut_notify_progress() {
        test_init();
        let _lock = lock_database();
        let (mut manager, mut client_rx) = init_manager();

        let file_path = "test_files/ut_notify_completed.txt";

        let file = File::create(file_path).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .retry(true)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .build();
        let uid = config.common_data.uid;
        let task_id = manager.create(config).unwrap();
        manager.start(uid, task_id);
        manager.scheduler.reschedule();
        ylong_runtime::block_on(async {
            let info = client_rx.recv().await.unwrap();
            let ClientEvent::SendResponse(tid, version, status_code, reason, headers) = info else {
                panic!("unexpected event: {:?}", info);
            };
            assert_eq!(tid, task_id);
            assert_eq!(version, "HTTP/1.1");
            assert_eq!(status_code, 200);
            assert_eq!(reason, "OK");
            assert!(!headers.is_empty());
            loop {
                let info = client_rx.recv().await.unwrap();
                let ClientEvent::SendNotifyData(subscribe_type, data) = info else {
                    panic!("unexpected event: {:?}", info);
                };
                let mut previous = 0;
                assert_eq!(subscribe_type, SubscribeType::Progress);
                assert_eq!(data.task_id, task_id);
                assert!(!data.progress.extras.is_empty());
                assert_eq!(data.progress.common_data.state, State::Running.repr);
                assert_eq!(data.progress.common_data.index, 0);
                assert_eq!(
                    data.progress.processed[0],
                    data.progress.common_data.total_processed
                );

                assert!(data.progress.common_data.total_processed >= previous);
                previous = data.progress.common_data.total_processed;
                if data.progress.common_data.total_processed == GITEE_FILE_LEN {
                    break;
                }
            }
        })
    }

    #[test]
    fn ut_notify_pause_resume() {
        test_init();
        let _lock = lock_database();
        let (mut manager, mut client_rx) = init_manager();

        let file_path = "test_files/ut_notify";

        let file = File::create(file_path).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .retry(true)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .build();
        let uid = config.common_data.uid;
        let task_id = manager.create(config).unwrap();
        manager.start(uid, task_id);
        manager.pause(uid, task_id);
        manager.resume(uid, task_id);
        ylong_runtime::block_on(async {
            let info = client_rx.recv().await.unwrap();
            let ClientEvent::SendNotifyData(subscribe_type, data) = info else {
                panic!("unexpected event: {:?}", info);
            };
            assert_eq!(subscribe_type, SubscribeType::Pause);
            assert!(data.progress.extras.is_empty());
            assert_eq!(data.progress.common_data.state, State::Paused.repr);
            assert_eq!(data.progress.common_data.index, 0);
            assert_eq!(
                data.progress.processed[0],
                data.progress.common_data.total_processed
            );
            assert_eq!(data.progress.common_data.total_processed, 0);
            let info = client_rx.recv().await.unwrap();
            let ClientEvent::SendNotifyData(subscribe_type, data) = info else {
                panic!("unexpected event: {:?}", info);
            };
            assert_eq!(subscribe_type, SubscribeType::Resume);
            assert!(data.progress.extras.is_empty());
            assert_eq!(data.progress.common_data.state, State::Waiting.repr);
            assert_eq!(data.progress.common_data.index, 0);
            assert_eq!(
                data.progress.processed[0],
                data.progress.common_data.total_processed
            );
            assert_eq!(data.progress.common_data.total_processed, 0);
        })
    }

    #[test]
    fn ut_notify_remove() {
        test_init();
        let _lock = lock_database();
        let (mut manager, mut client_rx) = init_manager();

        let file_path = "test_files/ut_notify";

        let file = File::create(file_path).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .retry(true)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .build();
        let uid = config.common_data.uid;
        let task_id = manager.create(config).unwrap();
        manager.remove(uid, task_id);
        ylong_runtime::block_on(async {
            let info = client_rx.recv().await.unwrap();
            let ClientEvent::SendNotifyData(subscribe_type, data) = info else {
                panic!("unexpected event: {:?}", info);
            };
            assert_eq!(subscribe_type, SubscribeType::Remove);
            assert!(data.progress.extras.is_empty());
            assert_eq!(data.progress.common_data.state, State::Removed.repr);
            assert_eq!(data.progress.common_data.index, 0);
            assert_eq!(
                data.progress.processed[0],
                data.progress.common_data.total_processed
            );
            assert_eq!(data.progress.common_data.total_processed, 0);
        })
    }

    #[test]
    fn ut_notify_completed() {
        test_init();
        let _lock = lock_database();
        let (mut manager, mut client_rx) = init_manager();

        let file_path = "test_files/ut_notify";

        let file = File::create(file_path).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .retry(true)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .build();
        let uid = config.common_data.uid;
        let task_id = manager.create(config).unwrap();
        manager.start(uid, task_id);
        manager.scheduler.task_completed(uid, task_id);
        ylong_runtime::block_on(async {
            let info = client_rx.recv().await.unwrap();
            let ClientEvent::SendNotifyData(subscribe_type, data) = info else {
                panic!("unexpected event: {:?}", info);
            };
            assert_eq!(subscribe_type, SubscribeType::Complete);
            assert!(data.progress.extras.is_empty());
            assert_eq!(data.progress.common_data.state, State::Completed.repr);
            assert_eq!(data.progress.common_data.index, 0);
            assert_eq!(
                data.progress.processed[0],
                data.progress.common_data.total_processed
            );
            assert_eq!(data.progress.common_data.total_processed, 0);
        })
    }

    #[test]
    fn ut_notify_failed() {
        test_init();
        let _lock = lock_database();
        let (mut manager, mut client_rx) = init_manager();

        let file_path = "test_files/ut_notify";

        let file = File::create(file_path).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .retry(true)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .build();
        let uid = config.common_data.uid;
        let task_id = manager.create(config).unwrap();
        manager.start(uid, task_id);
        manager.scheduler.task_failed(uid, task_id, Reason::IoError);
        ylong_runtime::block_on(async {
            let info = client_rx.recv().await.unwrap();
            let ClientEvent::SendNotifyData(subscribe_type, data) = info else {
                panic!("unexpected event: {:?}", info);
            };
            assert_eq!(subscribe_type, SubscribeType::Fail);
            assert!(data.progress.extras.is_empty());
            assert_eq!(data.progress.common_data.state, State::Failed.repr);
            assert_eq!(data.progress.common_data.index, 0);
            assert_eq!(
                data.progress.processed[0],
                data.progress.common_data.total_processed
            );
            assert_eq!(data.progress.common_data.total_processed, 0);
        })
    }

    #[test]
    fn ut_notify_pause_resume_completed() {
        test_init();
        let _lock = lock_database();
        let (mut manager, mut client_rx) = init_manager();

        let file_path = "test_files/ut_notify";

        let file = File::create(file_path).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .retry(true)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .build();
        let uid = config.common_data.uid;
        let task_id = manager.create(config).unwrap();
        manager.start(uid, task_id);
        manager.pause(uid, task_id);
        manager.scheduler.task_completed(uid, task_id);
        manager.resume(uid, task_id);
        ylong_runtime::block_on(async {
            let info = client_rx.recv().await.unwrap();
            let ClientEvent::SendNotifyData(subscribe_type, data) = info else {
                panic!("unexpected event: {:?}", info);
            };
            assert_eq!(subscribe_type, SubscribeType::Pause);
            let info = client_rx.recv().await.unwrap();
            let ClientEvent::SendNotifyData(subscribe_type, data) = info else {
                panic!("unexpected event: {:?}", info);
            };
            assert_eq!(subscribe_type, SubscribeType::Resume);
            assert!(client_rx.is_empty());
        })
    }

    #[test]
    fn ut_notify_pause_resume_failed() {
        test_init();
        let _lock = lock_database();
        let (mut manager, mut client_rx) = init_manager();

        let file_path = "test_files/ut_notify";

        let file = File::create(file_path).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .retry(true)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .build();
        let uid = config.common_data.uid;
        let task_id = manager.create(config).unwrap();
        manager.start(uid, task_id);
        manager.pause(uid, task_id);
        manager.scheduler.task_failed(uid, task_id, Reason::IoError);
        manager.resume(uid, task_id);
        ylong_runtime::block_on(async {
            let info = client_rx.recv().await.unwrap();
            let ClientEvent::SendNotifyData(subscribe_type, data) = info else {
                panic!("unexpected event: {:?}", info);
            };
            assert_eq!(subscribe_type, SubscribeType::Pause);
            let info = client_rx.recv().await.unwrap();
            let ClientEvent::SendNotifyData(subscribe_type, data) = info else {
                panic!("unexpected event: {:?}", info);
            };
            assert_eq!(subscribe_type, SubscribeType::Resume);
            assert!(client_rx.is_empty());
        })
    }
}
