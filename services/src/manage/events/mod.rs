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
mod set_max_speed;
mod set_mode;
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

    pub(crate) fn set_mode(uid: u64, task_id: u32, mode: Mode) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::SetMode(uid, task_id, mode, tx)),
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

    pub(crate) fn set_max_speed(uid: u64, task_id: u32, max_speed: i64) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceEvent::SetMaxSpeed(uid, task_id, max_speed, tx)),
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
    SetMaxSpeed(u64, u32, i64, Sender<ErrorCode>),
    SetMode(u64, u32, Mode, Sender<ErrorCode>),
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
    AppUninstall(u64),
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
    RestoreAllTasks,
    Unload,
}

#[cfg(not(feature = "oh"))]
#[cfg(test)]
mod ut_mod {
    include!("../../../tests/ut/manage/events/ut_mod.rs");
}
