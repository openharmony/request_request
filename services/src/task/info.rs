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

pub use ffi::State;

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

    pub(crate) fn token(&self) -> String {
        self.token.clone()
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

#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    #[derive(Clone, Copy, PartialEq, Debug)]
    #[repr(u8)]
    /// Task state
    pub enum State {
        /// Initialized
        Initialized = 0x00,
        /// Waiting
        Waiting = 0x10,
        /// Running
        Running = 0x20,
        /// Retrying
        Retrying = 0x21,
        /// Paused
        Paused = 0x30,
        /// Stopped
        Stopped = 0x31,
        /// Completed
        Completed = 0x40,
        /// Failed
        Failed = 0x41,
        /// Removed
        Removed = 0x50,
        /// Any
        Any = 0x61,
    }
}

#[derive(Debug)]
pub(crate) struct UpdateInfo {
    pub(crate) mtime: u64,
    pub(crate) reason: u8,
    pub(crate) tries: u32,
    pub(crate) mime_type: String,
    pub(crate) progress: Progress,
    pub(crate) each_file_status: Vec<EachFileStatus>,
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
            bundle: self.bundle.clone(),
            progress: self.progress.clone(),
            action: Action::from(self.common_data.action),
            version: Version::from(self.common_data.version),
            each_file_status: self.each_file_status.clone(),
            task_id: self.common_data.task_id,
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_enum_state() {
        assert_eq!(State::Initialized.repr, 0);
        assert_eq!(State::Waiting.repr, 16);
        assert_eq!(State::Running.repr, 32);
        assert_eq!(State::Retrying.repr, 33);
        assert_eq!(State::Paused.repr, 48);
        assert_eq!(State::Stopped.repr, 49);
        assert_eq!(State::Completed.repr, 64);
        assert_eq!(State::Failed.repr, 65);
        assert_eq!(State::Removed.repr, 80);
        assert_eq!(State::Any.repr, 97);
    }
}
