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

use std::sync::atomic::Ordering;

use super::database::NotificationDb;
use super::ffi::{self, SubscribeNotification};
use super::NotificationDispatcher;
use crate::config::{Mode, Version};
use crate::error::ErrorCode;
use crate::info::{State, TaskInfo};
use crate::manage::database::RequestDb;
use crate::manage::events::TaskManagerEvent;
use crate::manage::task_manager::TaskManagerTx;
use crate::manage::TaskManager;
use crate::task::request_task::RequestTask;
use crate::utils::Recv;

pub(super) fn cancel_notification(request_id: u32) {
    info!("cancel notification {}", request_id);
    let ret = ffi::CancelNotification(request_id);
    if ret != 0 {
        error!("cancel notification failed {}", ret);
    }
}

impl TaskManager {
    pub(crate) fn attach_group(&self, uid: u64, task_ids: Vec<u32>, group_id: u32) -> ErrorCode {
        for task_id in task_ids.iter().copied() {
            let Some(mode) = RequestDb::get_instance().query_task_mode(task_id) else {
                return ErrorCode::TaskNotFound;
            };
            if mode != Mode::BackGround {
                return ErrorCode::TaskModeErr;
            }
            let Some(state) = RequestDb::get_instance().query_task_state(task_id) else {
                return ErrorCode::TaskNotFound;
            };
            if state != State::Initialized.repr {
                return ErrorCode::TaskStateErr;
            }
        }
        if !NotificationDispatcher::get_instance().attach_group(task_ids, group_id, uid) {
            return ErrorCode::GroupNotFound;
        }
        ErrorCode::ErrOk
    }
}

pub(crate) trait NotificationCheck {
    fn notification_check(&self, db: &NotificationDb) -> bool;
}

impl NotificationCheck for RequestTask {
    fn notification_check(&self, db: &NotificationDb) -> bool {
        let mode = self.mode.load(Ordering::Acquire);
        notification_check_common(
            self.conf.version,
            true,
            Mode::from(mode),
            self.conf.common_data.background,
            false,
        ) && db.check_task_notification_available(&self.conf.common_data.task_id)
    }
}

impl NotificationCheck for TaskInfo {
    fn notification_check(&self, db: &NotificationDb) -> bool {
        notification_check_common(
            Version::from(self.common_data.version),
            self.common_data.gauge,
            Mode::from(self.common_data.mode),
            RequestDb::get_instance().query_task_background(self.common_data.task_id),
            true,
        ) && db.check_task_notification_available(&self.common_data.task_id)
    }
}

fn notification_check_common(
    version: Version,
    gauge: bool,
    mode: Mode,
    background: bool,
    completed_notify: bool,
) -> bool {
    version == Version::API10 && mode == Mode::BackGround && (gauge || completed_notify)
        || version == Version::API9 && background
}

pub struct TaskManagerWrapper {
    task_manager: TaskManagerTx,
}

impl TaskManagerWrapper {
    fn new(task_manager: TaskManagerTx) -> Self {
        Self { task_manager }
    }

    pub(crate) fn pause_task(&self, task_id: u32) -> bool {
        self.event_inner(task_id, TaskManagerEvent::pause)
    }

    pub(crate) fn resume_task(&self, task_id: u32) -> bool {
        self.event_inner(task_id, TaskManagerEvent::resume)
    }

    pub(crate) fn stop_task(&self, task_id: u32) -> bool {
        self.event_inner(task_id, TaskManagerEvent::stop)
    }

    fn event_inner<F>(&self, task_id: u32, f: F) -> bool
    where
        F: Fn(u64, u32) -> (TaskManagerEvent, Recv<ErrorCode>),
    {
        let Some(uid) = RequestDb::get_instance().query_task_uid(task_id) else {
            return false;
        };
        let (event, rx) = f(uid, task_id);
        self.task_manager.send_event(event);
        let Some(ret) = rx.get() else {
            return false;
        };
        if ret != ErrorCode::ErrOk {
            error!("notification_bar {} failed: {}", task_id, ret as u32);
            return false;
        }
        true
    }
}

pub(crate) fn subscribe_notification_bar(task_manager: TaskManagerTx) {
    SubscribeNotification(Box::new(TaskManagerWrapper::new(task_manager)));
}

impl RequestDb {
    fn query_task_background(&self, task_id: u32) -> bool {
        let sql = format!(
            "SELECT background FROM request_task WHERE task_id = {}",
            task_id
        );
        self.query_integer(&sql)
            .first()
            .map(|background: &i32| *background == 1)
            .unwrap_or(false)
    }

    pub(crate) fn query_task_mode(&self, task_id: u32) -> Option<Mode> {
        let sql = format!("SELECT mode FROM request_task WHERE task_id = {}", task_id);
        self.query_integer(&sql)
            .first()
            .map(|mode: &i32| Mode::from(*mode as u8))
    }
}
