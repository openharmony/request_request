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

use std::collections::HashMap;

use super::ffi::CEachFileStatus;
use super::notify::{EachFileStatus, NotifyData, Progress};
use crate::task::config::{Action, Version};
use crate::task::reason::Reason;
use crate::utils::c_wrapper::{CFileSpec, CFormItem};
use crate::utils::form_item::{FileSpec, FormItem};
use crate::utils::hashmap_to_string;

#[derive(Debug)]
pub(crate) struct TaskInfo {
    pub(crate) bundle: String,
    pub(crate) url: String,
    pub(crate) data: String,
    pub(crate) token: String,
    pub(crate) form_items: Vec<FormItem>,
    pub(crate) file_specs: Vec<FileSpec>,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) mime_type: String,
    pub(crate) progress: Progress,
    pub(crate) extras: HashMap<String, String>,
    pub(crate) each_file_status: Vec<EachFileStatus>,
    pub(crate) common_data: CommonTaskInfo,
}

impl TaskInfo {
    pub(crate) fn uid(&self) -> u64 {
        self.common_data.uid
    }

    pub(crate) fn mime_type(&self) -> String {
        self.mime_type.clone()
    }

    pub(crate) fn action(&self) -> Action {
        Action::from(self.common_data.action)
    }

    #[allow(dead_code)]
    pub(crate) fn state(&self) -> State {
        State::from(self.progress.common_data.state)
    }

    pub(crate) fn token(&self) -> String {
        self.token.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn set_state(&mut self, state: State) {
        self.progress.common_data.state = state as u8;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct CommonTaskInfo {
    pub(crate) task_id: u32,
    pub(crate) uid: u64,
    pub(crate) action: u8,
    pub(crate) mode: u8,
    pub(crate) ctime: u64,
    pub(crate) mtime: u64,
    pub(crate) reason: u8,
    pub(crate) gauge: bool,
    pub(crate) retry: bool,
    pub(crate) tries: u32,
    pub(crate) version: u8,
    pub(crate) priority: u32,
}

pub(crate) struct InfoSet {
    pub(crate) form_items: Vec<CFormItem>,
    pub(crate) file_specs: Vec<CFileSpec>,
    pub(crate) sizes: String,
    pub(crate) processed: String,
    pub(crate) extras: String,
    pub(crate) each_file_status: Vec<CEachFileStatus>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[repr(u8)]
pub(crate) enum Mode {
    BackGround = 0,
    FrontEnd,
    Any,
}

impl PartialOrd for Mode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Mode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let me = match self {
            Mode::FrontEnd => 0,
            Mode::Any => 1,
            Mode::BackGround => 2,
        };
        let other = match other {
            Mode::FrontEnd => 0,
            Mode::Any => 1,
            Mode::BackGround => 2,
        };
        me.cmp(&other)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub(crate) enum State {
    Initialized = 0x00,
    Waiting = 0x10,
    Running = 0x20,
    Retrying = 0x21,
    Paused = 0x30,
    Stopped = 0x31,
    Completed = 0x40,
    Failed = 0x41,
    Removed = 0x50,
    Created = 0x60,
    Any = 0x61,
}

pub(crate) struct UpdateInfo {
    pub(crate) mtime: u64,
    pub(crate) reason: u8,
    pub(crate) tries: u32,
    pub(crate) mime_type: String,
    pub(crate) progress: Progress,
    pub(crate) each_file_status: Vec<EachFileStatus>,
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        match value {
            0 => Mode::BackGround,
            1 => Mode::FrontEnd,
            _ => Mode::Any,
        }
    }
}

impl From<u8> for State {
    fn from(value: u8) -> Self {
        match value {
            0 => State::Initialized,
            16 => State::Waiting,
            32 => State::Running,
            33 => State::Retrying,
            48 => State::Paused,
            49 => State::Stopped,
            64 => State::Completed,
            65 => State::Failed,
            80 => State::Removed,
            96 => State::Created,
            _ => State::Any,
        }
    }
}

impl TaskInfo {
    pub(crate) fn build_info_set(&self) -> InfoSet {
        InfoSet {
            form_items: self.form_items.iter().map(|x| x.to_c_struct()).collect(),
            file_specs: self.file_specs.iter().map(|x| x.to_c_struct()).collect(),
            sizes: format!("{:?}", self.progress.sizes),
            processed: format!("{:?}", self.progress.processed),
            extras: hashmap_to_string(&self.extras),
            each_file_status: self
                .each_file_status
                .iter()
                .map(|x| x.to_c_struct())
                .collect(),
        }
    }

    pub(crate) fn build_notify_data(&self) -> NotifyData {
        NotifyData {
            progress: self.progress.clone(),
            action: Action::from(self.common_data.action),
            version: Version::from(self.common_data.version),
            each_file_status: self.each_file_status.clone(),
            task_id: self.common_data.task_id,
            _uid: self.common_data.uid,
        }
    }
}

#[derive(Debug)]
pub(crate) struct DumpAllInfo {
    pub(crate) vec: Vec<DumpAllEachInfo>,
}

#[derive(Debug)]
pub(crate) struct DumpAllEachInfo {
    pub(crate) task_id: u32,
    pub(crate) action: Action,
    pub(crate) state: State,
    pub(crate) reason: Reason,
}

#[derive(Debug)]
pub(crate) struct DumpOneInfo {
    pub(crate) task_id: u32,
    pub(crate) action: Action,
    pub(crate) state: State,
    pub(crate) reason: Reason,
    pub(crate) total_size: i64,
    pub(crate) tran_size: usize,
    pub(crate) url: String,
}

#[derive(Clone, Copy, PartialEq, Debug, Eq, PartialOrd, Ord)]
pub(crate) enum ApplicationState {
    Foreground = 2,
    Background = 4,
    Terminated = 5,
}

impl ApplicationState {
    pub(crate) fn from_bundles(top: &str, target: &str) -> Self {
        if top == target {
            ApplicationState::Foreground
        } else {
            ApplicationState::Background
        }
    }
}

impl From<u8> for ApplicationState {
    fn from(value: u8) -> Self {
        match value {
            2 => ApplicationState::Foreground,
            4 => ApplicationState::Background,
            5 => ApplicationState::Terminated,
            _ => panic!("wrong application value"),
        }
    }
}
