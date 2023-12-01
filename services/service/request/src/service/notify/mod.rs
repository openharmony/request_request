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

mod manager;

use ipc_rust::RemoteObj;
pub(crate) use manager::{NotifyEntry, NotifyManager};
use ylong_runtime::sync::oneshot::{channel, Sender};

use crate::error::ErrorCode;
use crate::task::notify::NotifyData;
use crate::utils::Recv;

pub(crate) enum NotifyEvent {
    Notify(Event, Box<NotifyData>),
    On(Event, u32, RemoteObj, Sender<ErrorCode>),
    Off(Event, u32, Sender<ErrorCode>),
    Clear(u32),
    Shutdown,
}

impl NotifyEvent {
    pub(crate) fn notify(event: Event, data: NotifyData) -> Self {
        Self::Notify(event, Box::new(data))
    }

    pub(crate) fn on(event: Event, id: u32, obj: RemoteObj) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (Self::On(event, id, obj, tx), Recv::new(rx))
    }

    pub(crate) fn off(event: Event, id: u32) -> (Self, Recv<ErrorCode>) {
        let (tx, rx) = channel::<ErrorCode>();
        (Self::Off(event, id, tx), Recv::new(rx))
    }

    pub(crate) fn clear(id: u32) -> Self {
        Self::Clear(id)
    }

    pub(crate) fn shutdown() -> Self {
        Self::Shutdown
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) enum Event {
    Complete,
    Fail,
    HeaderReceive,
    Pause,
    Progress,
    Remove,
    Resume,
}

impl Event {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Fail => "fail",
            Self::HeaderReceive => "headerReceive",
            Self::Pause => "pause",
            Self::Progress => "progress",
            Self::Remove => "remove",
            Self::Resume => "resume",
        }
    }
}

impl TryFrom<&str> for Event {
    type Error = EventConvertError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "complete" => Ok(Self::Complete),
            "fail" => Ok(Self::Fail),
            "headerReceive" => Ok(Self::HeaderReceive),
            "pause" => Ok(Self::Pause),
            "progress" => Ok(Self::Progress),
            "remove" => Ok(Self::Remove),
            "resume" => Ok(Self::Resume),
            _ => Err(EventConvertError),
        }
    }
}

impl TryFrom<String> for Event {
    type Error = EventConvertError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Event::try_from(value.as_str())
    }
}

#[derive(Debug)]
pub(crate) struct EventConvertError;
