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

use std::fmt::Debug;

use ylong_runtime::sync::oneshot::{channel, Sender};

use super::account::AccountEvent;
use crate::config::{Action, Mode};
use crate::error::ErrorCode;
use crate::info::TaskInfo;
use crate::task::config::TaskConfig;
use crate::task::info::{DumpAllInfo, DumpOneInfo};
use crate::task::reason::Reason;
use crate::utils::Recv;

mod construct;
mod dump;
mod pause;
mod remove;
mod resume;
mod start;
mod stop;

#[derive(Debug)]
pub(crate) enum TaskManagerEvent {
    Service(ServiceEvent),
    State(StateEvent),
    Schedule(ScheduleEvent),
    Task(TaskEvent),
    Device(i32),
    Account(AccountEvent),
    Query(QueryEvent),
    Reschedule,
}

impl TaskManagerEvent {
    pub(crate) fn construct(config: TaskConfig) -> (Self, Recv<Result<u32, ErrorCode>>) {
        let (tx, rx) = channel::<Result<u32, ErrorCode>>();
        (
            Self::Service(ServiceEvent::Construct(
                Box::new(ConstructMessage { config }),
                tx,
            )),
            Recv::new(rx),
        )
    }

    pub(crate) fn pause(uid: u64, task_id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::Pause(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn start(uid: u64, task_id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::Start(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn stop(uid: u64, task_id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::Stop(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn remove(uid: u64, task_id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::Remove(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn resume(uid: u64, task_id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::Resume(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn dump_all() -> (Self, Recv<DumpAllInfo>) {
        let (tx, rx) = channel::<DumpAllInfo>();
        (Self::Service(ServiceEvent::DumpAll(tx)), Recv::new(rx))
    }

    pub(crate) fn dump_one(task_id: u32) -> (Self, Recv<Option<DumpOneInfo>>) {
        let (tx, rx) = channel::<Option<DumpOneInfo>>();
        (
            Self::Service(ServiceEvent::DumpOne(task_id, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn network() -> Self {
        Self::State(StateEvent::Network)
    }

    pub(crate) fn subscribe(task_id: u32, token_id: u64) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Task(TaskEvent::Subscribe(task_id, token_id, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn attach_group(
        uid: u64,
        task_ids: Vec<u32>,
        group_id: u32,
    ) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::AttachGroup(uid, task_ids, group_id, tx)),
            Recv::new(rx),
        )
    }
}

#[derive(Debug)]
pub(crate) enum QueryEvent {
    Query(u32, Action, Sender<Option<TaskInfo>>),
    Show(u32, u64, Sender<Option<TaskInfo>>),
    Touch(u32, u64, String, Sender<Option<TaskInfo>>),
}

#[derive(Debug)]
pub(crate) enum ServiceEvent {
    Construct(Box<ConstructMessage>, Sender<Result<u32, ErrorCode>>),
    Pause(u64, u32, Sender<ErrorCode>),
    Start(u64, u32, Sender<ErrorCode>),
    Stop(u64, u32, Sender<ErrorCode>),
    Remove(u64, u32, Sender<ErrorCode>),
    Resume(u64, u32, Sender<ErrorCode>),
    DumpOne(u32, Sender<Option<DumpOneInfo>>),
    DumpAll(Sender<DumpAllInfo>),
    AttachGroup(u64, Vec<u32>, u32, Sender<ErrorCode>),
}

#[derive(Debug)]
pub(crate) enum TaskEvent {
    Completed(u32, u64, Mode),
    Failed(u32, u64, Reason, Mode),
    Offline(u32, u64, Mode),
    Running(u32, u64, Mode),
    Subscribe(u32, u64, Sender<ErrorCode>),
}

#[derive(Debug)]
pub(crate) enum StateEvent {
    Network,
    ForegroundApp(u64),
    Background(u64),
    BackgroundTimeout(u64),
    SpecialTerminate(u64),
}

pub(crate) struct ConstructMessage {
    pub(crate) config: TaskConfig,
}

impl Debug for ConstructMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Construct")
            .field("uid", &self.config.common_data.uid)
            .field("task_id", &self.config.common_data.task_id)
            .field("title", &self.config.title)
            .field("mode", &self.config.method)
            .field("version", &self.config.version)
            .finish()
    }
}

#[derive(Debug)]
pub(crate) enum ScheduleEvent {
    ClearTimeoutTasks,
    LogTasks,
    RestoreAllTasks,
    Unload,
}

#[cfg(not(feature = "oh"))]
#[cfg(test)]
mod test {
    use core::time;
    use std::fs::File;

    use once_cell::sync::Lazy;

    use super::TaskManagerEvent;
    use crate::config::{Action, ConfigBuilder, Mode};
    use crate::error::ErrorCode;
    use crate::manage::network::Network;
    use crate::manage::task_manager::TaskManagerTx;
    use crate::manage::TaskManager;
    use crate::service::client::{ClientManager, ClientManagerEntry};
    use crate::service::run_count::{RunCountManager, RunCountManagerEntry};

    static CLIENT: Lazy<ClientManagerEntry> = Lazy::new(|| ClientManager::init());
    static RUN_COUNT_MANAGER: Lazy<RunCountManagerEntry> = Lazy::new(|| RunCountManager::init());
    static NETWORK: Lazy<Network> = Lazy::new(|| Network::new());

    static TASK_MANGER: Lazy<TaskManagerTx> =
        Lazy::new(|| TaskManager::init(RUN_COUNT_MANAGER.clone(), CLIENT.clone(), NETWORK.clone()));
    fn build_task() {}

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
        let _ = std::fs::create_dir("test_files/");
    }

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
}
