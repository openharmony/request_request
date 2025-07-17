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

use ylong_runtime::sync::mpsc::unbounded_channel;

use super::database::RequestDb;
use super::network::{NetworkInfo, NetworkInner, NetworkType};
use super::TaskManager;
use crate::config::{Action, ConfigBuilder, Mode};
use crate::error::ErrorCode;
use crate::info::{State, TaskInfo};
use crate::manage::task_manager::{TaskManagerRx, TaskManagerTx};
use crate::service::active_counter::ActiveCounter;
use crate::service::client::ClientManagerEntry;
use crate::service::run_count::RunCountManagerEntry;
use crate::tests::{lock_database, test_init};

fn task_manager() -> TaskManager {
    let (tx, rx) = unbounded_channel();
    let task_manager_tx = TaskManagerTx::new(tx);
    let rx = TaskManagerRx::new(rx);
    let inner = NetworkInner::new();
    inner.notify_online(NetworkInfo {
        network_type: NetworkType::Wifi,
        is_metered: false,
        is_roaming: false,
    });
    let (tx, _rx) = unbounded_channel();
    let run_count = RunCountManagerEntry::new(tx);
    let (tx, _rx) = unbounded_channel();
    let client = ClientManagerEntry::new(tx);
    TaskManager::new(task_manager_tx, rx, run_count, client, ActiveCounter::new())
}

fn task_into(task_id: u32) -> TaskInfo {
    let db = RequestDb::get_instance();
    db.get_task_info(task_id).unwrap()
}

// @tc.name: ut_manager_state_change_error
// @tc.desc: Test error handling of task state transitions
// @tc.precon: NA
// @tc.step: 1. Initialize task manager and create download task
//           2. Attempt invalid state transitions (pause/resume/stop on initialized task)
//           3. Verify error codes for invalid transitions
//           4. Test valid state transitions and verify state changes
// @tc.expect: Invalid transitions return TaskStateErr, valid transitions succeed
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_manager_state_change_error() {
    test_init();
    let _lock = lock_database();
    let mut manager = task_manager();
    let file_path = "test_files/ut_manager_state_change_error.txt";

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

    // initialized
    let task_id = manager.create(config.clone()).unwrap();
    assert_eq!(
        task_into(task_id).progress.common_data.state,
        State::Initialized.repr
    );
    assert_eq!(manager.pause(uid, task_id), ErrorCode::TaskStateErr);
    assert_eq!(manager.resume(uid, task_id), ErrorCode::TaskStateErr);
    assert_eq!(manager.stop(uid, task_id), ErrorCode::TaskStateErr);
    assert_eq!(manager.remove(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(
        task_into(task_id).progress.common_data.state,
        State::Removed.repr
    );

    // started
    let task_id = manager.create(config.clone()).unwrap();
    assert_eq!(manager.start(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(manager.resume(uid, task_id), ErrorCode::TaskStateErr);
    assert_eq!(manager.start(uid, task_id), ErrorCode::TaskStateErr);
    assert_eq!(manager.remove(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(
        task_into(task_id).progress.common_data.state,
        State::Removed.repr
    );

    // paused
    let task_id = manager.create(config.clone()).unwrap();
    assert_eq!(manager.start(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(manager.pause(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(manager.pause(uid, task_id), ErrorCode::TaskStateErr);
    assert_eq!(manager.stop(uid, task_id), ErrorCode::TaskStateErr);
    assert_eq!(manager.start(uid, task_id), ErrorCode::TaskStateErr);
    assert_eq!(manager.remove(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(
        task_into(task_id).progress.common_data.state,
        State::Removed.repr
    );

    // stopped
    let task_id = manager.create(config.clone()).unwrap();
    assert_eq!(manager.start(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(manager.stop(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(manager.pause(uid, task_id), ErrorCode::TaskStateErr);
    assert_eq!(manager.stop(uid, task_id), ErrorCode::TaskStateErr);
    assert_eq!(manager.resume(uid, task_id), ErrorCode::TaskStateErr);
    assert_eq!(manager.start(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(manager.stop(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(manager.remove(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(
        task_into(task_id).progress.common_data.state,
        State::Removed.repr
    );

    // resumed
    let task_id = manager.create(config.clone()).unwrap();
    assert_eq!(manager.start(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(manager.pause(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(manager.resume(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(manager.resume(uid, task_id), ErrorCode::TaskStateErr);
    assert_eq!(manager.start(uid, task_id), ErrorCode::TaskStateErr);
    assert_eq!(manager.pause(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(manager.resume(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(manager.remove(uid, task_id), ErrorCode::ErrOk);
    assert_eq!(
        task_into(task_id).progress.common_data.state,
        State::Removed.repr
    );
}