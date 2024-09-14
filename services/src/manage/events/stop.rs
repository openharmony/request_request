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

use crate::error::ErrorCode;
use crate::info::State;
use crate::manage::database::RequestDb;
use crate::manage::TaskManager;

impl TaskManager {
    pub(crate) fn stop(&mut self, uid: u64, task_id: u32) -> ErrorCode {
        debug!("stop task, tid: {}", task_id);
        let db = RequestDb::get_instance();
        if let Some(info) = db.get_task_qos_info(task_id) {
            if info.state == State::Running.repr
                || info.state == State::Retrying.repr
                || info.state == State::Waiting.repr
            {
                if let Some(count) = self.task_count.get_mut(&uid) {
                    let count = match info.mode {
                        1 => &mut count.0,
                        _ => &mut count.1,
                    };
                    if *count > 0 {
                        *count -= 1;
                    }
                }
            }
        }

        match self.scheduler.stop_task(uid, task_id) {
            Ok(_) => ErrorCode::ErrOk,
            Err(e) => e,
        }
    }
}
