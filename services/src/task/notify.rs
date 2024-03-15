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

use super::config::{Action, Version};
use super::info::State;
use super::reason::Reason;

// NotifyData's callback arg
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) enum SubscribeType {
    Complete = 0,
    Fail,
    HeaderReceive,
    Pause,
    Progress,
    Remove,
    Resume,
}

#[derive(Debug, Clone)]
pub(crate) struct NotifyData {
    pub(crate) progress: Progress,
    pub(crate) action: Action,
    pub(crate) version: Version,
    pub(crate) each_file_status: Vec<EachFileStatus>,
    pub(crate) task_id: u32,
    pub(crate) _uid: u64,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub(crate) struct CommonProgress {
    pub(crate) state: u8,
    pub(crate) index: usize,
    pub(crate) total_processed: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct Progress {
    pub(crate) common_data: CommonProgress,
    pub(crate) sizes: Vec<i64>,
    pub(crate) processed: Vec<usize>,
    pub(crate) extras: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub(crate) struct EachFileStatus {
    pub(crate) path: String,
    pub(crate) reason: Reason,
    pub(crate) message: String,
}

impl Progress {
    pub(crate) fn new(sizes: Vec<i64>) -> Self {
        let len = sizes.len();
        Progress {
            common_data: CommonProgress {
                state: State::Created as u8,
                index: 0,
                total_processed: 0,
            },
            sizes,
            processed: vec![0; len],
            extras: HashMap::<String, String>::new(),
        }
    }
}
