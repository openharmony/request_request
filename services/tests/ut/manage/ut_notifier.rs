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
use crate::service::active_counter::ActiveCounter;
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
        TaskManager::new(task_manager_tx, rx, run_count, client, ActiveCounter::new()),
        client_rx,
    )
}

// @tc.name: ut_network
// @tc.desc: Test network online/offline status detection
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Notify network online with different configurations
//           3. Verify network status updates correctly
// @tc.expect: Network status is accurately reported
// @tc.type: FUNC
// @tc.require: issues#ICN16H
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

// @tc.name: ut_network_notify
// @tc.desc: Test network notification mechanism
// @tc.precon: NA
// @tc.step: 1. Initialize network notifier
//           2. Send multiple online/offline notifications
//           3. Verify notification status changes
// @tc.expect: Network notifications are processed correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
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

// @tc.name: ut_notify_progress
// @tc.desc: Test download progress notification functionality
// @tc.precon: NA
// @tc.step: 1. Initialize task manager and create download task
//           2. Monitor progress notifications during download
//           3. Verify progress updates and completion status
// @tc.expect: Progress notifications are accurate and complete with correct file size
// @tc.type: FUNC
// @tc.require: issues#ICN16H
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

// @tc.name: ut_notify_pause_resume
// @tc.desc: Test pause and resume notification functionality
// @tc.precon: NA
// @tc.step: 1. Initialize task manager and create download task
//           2. Pause the download task and verify notification
//           3. Resume the download task and verify notification
// @tc.expect: Pause and resume events trigger correct notifications
// @tc.type: FUNC
// @tc.require: issues#ICN16H
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

// @tc.name: ut_notify_remove
// @tc.desc: Test notification when task is removed
// @tc.precon: NA
// @tc.step: 1. Initialize test environment and database
//           2. Create download task with background mode
//           3. Call remove method to delete the task
//           4. Verify notification data and type
// @tc.expect: Receive Remove type notification with correct task state and progress
// @tc.type: FUNC
// @tc.require: issues#ICN16H
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

// @tc.name: ut_notify_completed
// @tc.desc: Test notification when task is completed
// @tc.precon: NA
// @tc.step: 1. Initialize test environment and database
//           2. Create and start download task
//           3. Trigger task completion
//           4. Verify notification data and type
// @tc.expect: Receive Complete type notification with correct task state and progress
// @tc.type: FUNC
// @tc.require: issues#ICN16H
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

// @tc.name: ut_notify_failed
// @tc.desc: Test notification when task fails
// @tc.precon: NA
// @tc.step: 1. Initialize test environment and database
//           2. Create and start download task
//           3. Trigger task failure with IoError reason
//           4. Verify notification data and type
// @tc.expect: Receive Fail type notification with correct task state and progress
// @tc.type: FUNC
// @tc.require: issues#ICN16H
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

// @tc.name: ut_notify_pause_resume_completed
// @tc.desc: Test pause and resume notifications when task is completed
// @tc.precon: NA
// @tc.step: 1. Initialize test environment and database
//           2. Create, start and pause download task
//           3. Trigger task completion and resume task
//           4. Verify pause and resume notifications
// @tc.expect: Receive both Pause and Resume type notifications
// @tc.type: FUNC
// @tc.require: issues#ICN31I
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

// @tc.name: ut_notify_pause_resume_failed
// @tc.desc: Test pause and resume notifications when task fails
// @tc.precon: NA
// @tc.step: 1. Initialize test environment and database
//           2. Create, start and pause download task
//           3. Trigger task failure and resume task
//           4. Verify pause and resume notifications
// @tc.expect: Receive both Pause and Resume type notifications
// @tc.type: FUNC
// @tc.require: issues#ICN16H
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