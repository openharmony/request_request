/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::enumration::*;
use std::collections::HashMap;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CommonProgress {
    pub state: State,
    pub index: usize,
    pub total_processed: usize,
}
#[derive(Debug)]
pub struct Progress {
    pub common_data: CommonProgress,
    pub sizes: Vec<i64>,
    pub processed: Vec<usize>,
    pub extras: HashMap<String, String>,
}

impl Progress {
    pub fn new(sizes: Vec<i64>) -> Self {
        let len = sizes.len();
        Progress {
            common_data: CommonProgress {
                state: State::INITIALIZED,
                index: 0,
                total_processed: 0,
            },
            sizes,
            processed: vec![0; len],
            extras: HashMap::<String, String>::new(),
        }
    }
}

impl Clone for Progress {
    fn clone(&self) -> Self {
        Progress {
            common_data: self.common_data,
            sizes: self.sizes.clone(),
            processed: self.processed.clone(),
            extras: self.extras.clone(),
        }
    }
}
#[derive(Debug)]
pub struct NotifyData {
    pub progress: Progress,
    pub action: Action,
    pub version: Version,
    pub each_file_status: Vec<(String, Reason, String)>, // (path, each_file_state, reason)
    pub task_id: u32,
    pub uid: u64,
    pub bundle: String,
}

#[repr(C)]
pub struct RequestTaskMsg {
    pub taskId: u32,
    pub uid: i32,
    pub action: u8,
}
