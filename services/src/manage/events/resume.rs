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

use std::sync::atomic::Ordering;

use crate::error::ErrorCode;
use crate::manage::TaskManager;
use crate::task::info::State;

cfg_oh! {
    use crate::manage::notifier::Notifier;
}

impl TaskManager {
    pub(crate) fn resume(&mut self, uid: u64, task_id: u32) -> ErrorCode {
        debug!("TaskManager resume, uid:{}, task_id:{}", uid, task_id);

        if let Some(task) = self.get_task(uid, task_id) {
            let state = task.status.lock().unwrap().state;
            if state != State::Paused {
                error!("can not resume a task which state is not paused");
                return ErrorCode::TaskStateErr;
            }
            error!("resume the task success");
            task.resume.store(true, Ordering::SeqCst);
            let notify_data = task.build_notify_data();

            #[cfg(feature = "oh")]
            Notifier::service_front_notify(
                "resume".into(),
                notify_data,
                &self.app_state(task.conf.common_data.uid, &task.conf.bundle),
            );
            self.start_inner(task);
            ErrorCode::ErrOk
        } else {
            if self.tasks.contains_key(&task_id) {
                error!("TaskManager resume a task, task_id:{} exist, but not found in app_task_map, uid:{}", task_id, uid);
            } else {
                error!(
                    "TaskManager resume a task, uid:{}, task_id:{} not exist",
                    uid, task_id
                );
            }

            ErrorCode::TaskStateErr
        }
    }
}
