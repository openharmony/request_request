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

use crate::error::ErrorCode;
use crate::info::State;
use crate::manage::database::RequestDb;
use crate::manage::TaskManager;

impl TaskManager {
    pub(crate) fn set_max_speed(&mut self, uid: u64, task_id: u32, max_speed: i64) -> ErrorCode {
        debug!(
            "TaskManager set_max_speed, uid{}, tid{}, max_speed{}",
            uid, task_id, max_speed
        );

        let db = RequestDb::get_instance();
        if let Some(info) = db.get_task_qos_info(task_id) {
            if info.state == State::Removed.repr {
                return ErrorCode::TaskStateErr;
            }
            db.update_task_max_speed(task_id, max_speed);
        } else {
            return ErrorCode::TaskStateErr;
        }

        match self.scheduler.set_max_speed(uid, task_id, max_speed) {
            Ok(_) => ErrorCode::ErrOk,
            Err(e) => e,
        }
    }
}
