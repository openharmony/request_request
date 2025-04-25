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
use crate::FileSpec;

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
    //Response,
    Faults = 8,
}

#[derive(Debug, Clone)]
pub(crate) struct NotifyData {
    pub(crate) bundle: String,
    pub(crate) progress: Progress,
    pub(crate) action: Action,
    pub(crate) version: Version,
    pub(crate) each_file_status: Vec<EachFileStatus>,
    pub(crate) task_id: u32,
    pub(crate) uid: u64,
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
    /// Total size of the files.
    pub(crate) sizes: Vec<i64>,
    /// Each progress size of the files.
    pub(crate) processed: Vec<usize>,
    pub(crate) extras: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub(crate) struct EachFileStatus {
    pub(crate) path: String,
    pub(crate) reason: Reason,
    pub(crate) message: String,
}

impl EachFileStatus {
    pub(crate) fn create_each_file_status(
        file_specs: &[FileSpec],
        index: usize,
        reason: Reason,
    ) -> Vec<EachFileStatus> {
        let mut vec = Vec::new();
        for (i, file_spec) in file_specs.iter().enumerate() {
            let code = if i >= index { reason } else { Reason::Default };
            let each_file_status = EachFileStatus {
                path: file_spec.path.clone(),
                reason: code,
                message: code.to_str().into(),
            };
            vec.push(each_file_status);
        }
        vec
    }
}

impl Progress {
    pub(crate) fn new(sizes: Vec<i64>) -> Self {
        let len = sizes.len();
        Progress {
            common_data: CommonProgress {
                state: State::Initialized.repr,
                index: 0,
                total_processed: 0,
            },
            sizes,
            processed: vec![0; len],
            extras: HashMap::<String, String>::new(),
        }
    }

    pub(crate) fn is_finish(&self) -> bool {
        self.sizes.iter().all(|a| *a != -1)
            && self.processed.iter().sum::<usize>() == self.sizes.iter().sum::<i64>() as usize
    }
}
