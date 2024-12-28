// Copyright (C) 2024 Huawei Device Co., Ltd.
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

use crate::config::Mode;
use crate::error::ErrorCode;
use crate::manage::TaskManager;

impl TaskManager {
    pub(crate) fn set_mode(&mut self, uid: u64, task_id: u32, mode: Mode) -> ErrorCode {
        debug!("TaskManager change_mode, tid{} mode{:?}", task_id, mode);
        match self.scheduler.task_set_mode(uid, task_id, mode) {
            Ok(_) => ErrorCode::ErrOk,
            Err(e) => e,
        }
    }
}
