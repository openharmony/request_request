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

use crate::error::ErrorCode;
use crate::task::config::{Action, TaskConfig};
use crate::task::info::{ApplicationState, DumpAllInfo, DumpOneInfo, TaskInfo};
use crate::utils::filter::Filter;
use crate::utils::Recv;

mod construct;
mod dump;
mod get_task;
mod pause;
mod query;
mod query_mime_type;
mod remove;
mod resume;
mod search;
mod show;
mod start;
mod stop;
mod touch;

#[derive(Debug)]
pub(crate) enum EventMessage {
    Service(ServiceMessage),
    State(StateMessage),
    Scheduled(ScheduledMessage),
    Task(TaskMessage),
}

impl EventMessage {
    pub(crate) fn construct(config: TaskConfig) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceMessage::Construct(
                Box::new(ConstructMessage { config }),
                tx,
            )),
            Recv::new(rx),
        )
    }

    pub(crate) fn pause(uid: u64, task_id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceMessage::Pause(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn query(task_id: u32, action: Action) -> (Self, Recv<Option<TaskInfo>>) {
        let (tx, rx) = channel::<Option<TaskInfo>>();
        (
            Self::Service(ServiceMessage::Query(task_id, action, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn query_mime_type(uid: u64, task_id: u32) -> (Self, Recv<String>) {
        let (tx, rx) = channel::<String>();
        (
            Self::Service(ServiceMessage::QueryMimeType(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn start(uid: u64, task_id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceMessage::Start(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn stop(uid: u64, task_id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceMessage::Stop(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn show(uid: u64, task_id: u32) -> (Self, Recv<Option<TaskInfo>>) {
        let (tx, rx) = channel::<Option<TaskInfo>>();
        (
            Self::Service(ServiceMessage::Show(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn search(filter: Filter) -> (Self, Recv<Vec<u32>>) {
        let (tx, rx) = channel::<Vec<u32>>();
        (
            Self::Service(ServiceMessage::Search(filter, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn touch(uid: u64, task_id: u32, token: String) -> (Self, Recv<Option<TaskInfo>>) {
        let (tx, rx) = channel::<Option<TaskInfo>>();
        (
            Self::Service(ServiceMessage::Touch(uid, task_id, token, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn get_task(
        uid: u64,
        task_id: u32,
        token: String,
    ) -> (Self, Recv<Option<TaskConfig>>) {
        let (tx, rx) = channel::<Option<TaskConfig>>();
        (
            Self::Service(ServiceMessage::GetTask(uid, task_id, token, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn remove(uid: u64, task_id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceMessage::Remove(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn resume(uid: u64, task_id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Service(ServiceMessage::Resume(uid, task_id, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn dump_all() -> (Self, Recv<DumpAllInfo>) {
        let (tx, rx) = channel::<DumpAllInfo>();
        (Self::Service(ServiceMessage::DumpAll(tx)), Recv::new(rx))
    }

    pub(crate) fn dump_one(task_id: u32) -> (Self, Recv<Option<DumpOneInfo>>) {
        let (tx, rx) = channel::<Option<DumpOneInfo>>();
        (
            Self::Service(ServiceMessage::DumpOne(task_id, tx)),
            Recv::new(rx),
        )
    }

    pub(crate) fn app_state_change(uid: u64, state: ApplicationState) -> Self {
        Self::State(StateMessage::AppStateChange(uid, state))
    }

    pub(crate) fn network_change() -> Self {
        Self::State(StateMessage::NetworkChange)
    }

    pub(crate) fn subscribe(task_id: u32, token_id: u64) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (
            Self::Task(TaskMessage::Subscribe(task_id, token_id, tx)),
            Recv::new(rx),
        )
    }
}

pub(crate) enum ServiceMessage {
    Construct(Box<ConstructMessage>, Sender<ErrorCode>),
    Pause(u64, u32, Sender<ErrorCode>),
    QueryMimeType(u64, u32, Sender<String>),
    Start(u64, u32, Sender<ErrorCode>),
    Stop(u64, u32, Sender<ErrorCode>),
    Show(u64, u32, Sender<Option<TaskInfo>>),
    Remove(u64, u32, Sender<ErrorCode>),
    Resume(u64, u32, Sender<ErrorCode>),
    Touch(u64, u32, String, Sender<Option<TaskInfo>>),
    Query(u32, Action, Sender<Option<TaskInfo>>),
    GetTask(u64, u32, String, Sender<Option<TaskConfig>>),
    DumpOne(u32, Sender<Option<DumpOneInfo>>),
    Search(Filter, Sender<Vec<u32>>),
    DumpAll(Sender<DumpAllInfo>),
}

pub(crate) enum TaskMessage {
    Finished(u32),
    Subscribe(u32, u64, Sender<ErrorCode>),
}

pub(crate) enum StateMessage {
    NetworkChange,
    AppStateChange(u64, ApplicationState),
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

impl Debug for ServiceMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Construct(message, _) => message.fmt(f),
            Self::Pause(uid, task_id, _) => f
                .debug_struct("Pause")
                .field("uid", uid)
                .field("task_id", task_id)
                .finish(),
            Self::QueryMimeType(uid, task_id, _) => f
                .debug_struct("QueryMimeType")
                .field("uid", uid)
                .field("task_id", task_id)
                .finish(),
            Self::Start(uid, task_id, _) => f
                .debug_struct("Start")
                .field("uid", uid)
                .field("task_id", task_id)
                .finish(),
            Self::Stop(uid, task_id, _) => f
                .debug_struct("Stop")
                .field("uid", uid)
                .field("task_id", task_id)
                .finish(),
            Self::Show(uid, task_id, _) => f
                .debug_struct("Show")
                .field("uid", uid)
                .field("task_id", task_id)
                .finish(),
            Self::Remove(uid, task_id, _) => f
                .debug_struct("Remove")
                .field("uid", uid)
                .field("task_id", task_id)
                .finish(),
            Self::Resume(uid, task_id, _) => f
                .debug_struct("Resume")
                .field("uid", uid)
                .field("task_id", task_id)
                .finish(),
            Self::Touch(uid, task_id, token, _) => f
                .debug_struct("Touch")
                .field("uid", uid)
                .field("task_id", task_id)
                .field("token", token)
                .finish(),
            Self::Query(task_id, action, _) => f
                .debug_struct("Query")
                .field("task_id", task_id)
                .field("action", action)
                .finish(),
            Self::GetTask(uid, task_id, token, _) => f
                .debug_struct("GetTask")
                .field("uid", uid)
                .field("task_id", task_id)
                .field("token", token)
                .finish(),
            Self::DumpOne(task_id, _) => {
                f.debug_struct("DumpOne").field("task_id", task_id).finish()
            }
            Self::Search(filter, _) => f.debug_struct("Search").field("filter", filter).finish(),
            Self::DumpAll(_) => f.debug_struct("DumpAll").finish(),
        }
    }
}

impl Debug for TaskMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Finished(task_id) => f
                .debug_struct("Finished")
                .field("task_id", task_id)
                .finish(),
            Self::Subscribe(task_id, token_id, _) => f
                .debug_struct("Subscribe")
                .field("task_id", task_id)
                .field("token_id", token_id)
                .finish(),
        }
    }
}

impl Debug for StateMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NetworkChange => f.pad("NetworkChange"),
            Self::AppStateChange(uid, state) => f
                .debug_struct("AppStateChange")
                .field("uid", uid)
                .field("state", state)
                .finish(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum ScheduledMessage {
    ClearTimeoutTasks,
    LogTasks,
    Unload,
    UpdateBackgroundApp(u64),
    RestoreAllTasks,
}
