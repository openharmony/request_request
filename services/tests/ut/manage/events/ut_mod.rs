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

use core::time;
use std::fs::File;

use once_cell::sync::Lazy;

use super::TaskManagerEvent;
use crate::config::{Action, ConfigBuilder, Mode};
use crate::error::ErrorCode;
use crate::manage::network::Network;
use crate::manage::task_manager::TaskManagerTx;
use crate::service::active_counter::ActiveCounter;
use crate::manage::TaskManager;
use crate::service::client::{ClientManager, ClientManagerEntry};
use crate::service::run_count::{RunCountManager, RunCountManagerEntry};

static CLIENT: Lazy<ClientManagerEntry> = Lazy::new(|| ClientManager::init());
static RUN_COUNT_MANAGER: Lazy<RunCountManagerEntry> = Lazy::new(|| RunCountManager::init());
static NETWORK: Lazy<Network> = Lazy::new(|| Network::new());

static TASK_MANGER: Lazy<TaskManagerTx> =
    Lazy::new(|| TaskManager::init(RUN_COUNT_MANAGER.clone(), CLIENT.clone(), ActiveCounter::new(), NETWORK.clone()));
fn build_task() {}

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
    let _ = std::fs::create_dir("test_files/");
}

// @tc.name: ut_task_manager_construct
// @tc.desc: Test task manager event construction
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Create test file
//           3. Build configuration
//           4. Construct TaskManagerEvent
//           5. Send event and verify response
// @tc.expect: Event is constructed successfully and response is received
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_construct() {
    init();
    let file_path = "test_files/ut_task_manager_construct.txt";

    let file = File::create(file_path).unwrap();
    let config = ConfigBuilder::new()
    .action(Action::Download)
    .mode(Mode::BackGround)
    .file_spec(file)
    .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
    .redirect(true)
    .build();
    let (event, rx) = TaskManagerEvent::construct(config);
    TASK_MANGER.send_event(event);
    rx.get().unwrap().unwrap();
}

// @tc.name: ut_task_manager_start
// @tc.desc: Test starting a task via task manager
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Create test file and configuration
//           3. Construct and send event to create task
//           4. Send start event and check result
//           5. Wait for task to start
// @tc.expect: Task starts successfully with no error
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_start() {
    init();
    let file_path = "test_files/ut_task_manager_construct.txt";

    let file = File::create(file_path).unwrap();
    let uid = 111;
    let config = ConfigBuilder::new()
    .action(Action::Download)
    .mode(Mode::BackGround)
    .file_spec(file)
    .url("https://sf3-cn.feishucdn.com/obj/ee-appcenter/47273f95/Feishu-win32_ia32-7.9.7-signed.exe")
    .redirect(true)
    .uid(uid)
    .build();
    let (event, rx) = TaskManagerEvent::construct(config.clone());
    TASK_MANGER.send_event(event);
    let task_id = rx.get().unwrap().unwrap();
    let (event, rx) = TaskManagerEvent::start(uid, task_id);
    TASK_MANGER.send_event(event);
    let res = rx.get().unwrap();
    assert_eq!(res, ErrorCode::ErrOk);
    std::thread::sleep(time::Duration::from_secs(10));
}

// @tc.name: ut_task_manager_pause_resume
// @tc.desc: Test pausing and resuming a task
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Create test file and configuration
//           3. Create task and send start event
//           4. Send pause event
//           5. Send resume event
//           6. Wait for task to resume
// @tc.expect: Task pauses and resumes successfully
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_pause_resume() {
    init();
    let file_path = "test_files/ut_task_manager_pause_resume.txt";

    let file = File::create(file_path).unwrap();
    let uid = 111;
    let config = ConfigBuilder::new()
    .action(Action::Download)
    .mode(Mode::BackGround)
    .file_spec(file)
    .url("https://sf3-cn.feishucdn.com/obj/ee-appcenter/47273f95/Feishu-win32_ia32-7.9.7-signed.exe")
    .redirect(true)
    .uid(uid)
    .build();
    let (event, rx) = TaskManagerEvent::construct(config.clone());
    TASK_MANGER.send_event(event);
    let task_id = rx.get().unwrap().unwrap();
    let (event, _rx) = TaskManagerEvent::start(uid, task_id);
    TASK_MANGER.send_event(event);
    let (event, _rx) = TaskManagerEvent::pause(uid, task_id);
    TASK_MANGER.send_event(event);
    let (event, _rx) = TaskManagerEvent::resume(uid, task_id);
    TASK_MANGER.send_event(event);
    std::thread::sleep(time::Duration::from_secs(20));
}

// @tc.name: ut_task_manager_stop_resume
// @tc.desc: Test stopping and attempting to resume a task
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Create test file and configuration
//           3. Create task and send start event
//           4. Send stop event
//           5. Send resume event
//           6. Wait for task to handle events
// @tc.expect: Task stops and cannot resume after stop
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_stop_resume() {
    init();
    let file_path = "test_files/ut_task_manager_pause_resume.txt";

    let file = File::create(file_path).unwrap();
    let uid = 111;
    let config = ConfigBuilder::new()
    .action(Action::Download)
    .mode(Mode::BackGround)
    .file_spec(file)
    .url("https://sf3-cn.feishucdn.com/obj/ee-appcenter/47273f95/Feishu-win32_ia32-7.9.7-signed.exe")
    .redirect(true)
    .uid(uid)
    .build();
    let (event, rx) = TaskManagerEvent::construct(config.clone());
    TASK_MANGER.send_event(event);
    let task_id = rx.get().unwrap().unwrap();
    let (event, _rx) = TaskManagerEvent::start(uid, task_id);
    TASK_MANGER.send_event(event);
    let (event, _rx) = TaskManagerEvent::stop(uid, task_id);
    TASK_MANGER.send_event(event);
    let (event, _rx) = TaskManagerEvent::resume(uid, task_id);
    TASK_MANGER.send_event(event);
    std::thread::sleep(time::Duration::from_secs(20));
}