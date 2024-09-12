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

use ffi::{RequestTaskMsg, SubscribeNotification};

use crate::config::{Action, Mode, Version};
use crate::error::ErrorCode;
use crate::info::TaskInfo;
use crate::manage::database::RequestDb;
use crate::manage::events::TaskManagerEvent;
use crate::manage::task_manager::TaskManagerTx;
use crate::task::notify::Progress;
use crate::task::request_task::RequestTask;
use crate::utils::{get_current_timestamp, Recv};
const NOTIFY_PROGRESS_INTERVAL: u64 = 3000;

pub(crate) fn publish_progress_notification(task: &RequestTask) {
    if !task.notification_check() {
        return;
    }
    let index = task.progress.lock().unwrap().common_data.index;
    let msg = RequestTaskMsg::new(
        task.task_id(),
        task.uid(),
        task.action(),
        task.conf.file_specs[index].file_name.clone(),
        task.progress.lock().unwrap().clone(),
    );
    ffi::RequestNotifyProgress(msg);
}

pub(crate) fn publish_success_notification(info: &TaskInfo) {
    if !info.notification_check() {
        return;
    }
    ffi::RequestNotifyCompleted(
        info.action().repr,
        info.common_data.task_id,
        info.common_data.uid as i32,
        info.file_specs[info.progress.common_data.index]
            .file_name
            .clone(),
    );
}

pub(crate) fn publish_failed_notification(info: &TaskInfo) {
    if !info.notification_check() {
        return;
    }
    ffi::RequestNotifyFailed(
        info.action().repr,
        info.common_data.task_id,
        info.common_data.uid as i32,
        info.file_specs[info.progress.common_data.index]
            .file_name
            .clone(),
    );
}

trait NotificationCheck {
    fn notification_check(&self) -> bool;
}

impl NotificationCheck for RequestTask {
    fn notification_check(&self) -> bool {
        if !(self.conf.version == Version::API9 && self.conf.common_data.background
            || self.conf.version == Version::API10
                && self.conf.common_data.gauge
                && self.conf.common_data.mode == Mode::BackGround)
        {
            return false;
        }
        let last_background_notify_time = self.background_notify_time.load(Ordering::SeqCst);
        let current = get_current_timestamp();
        if current - last_background_notify_time < NOTIFY_PROGRESS_INTERVAL {
            return false;
        }
        self.background_notify_time.store(current, Ordering::SeqCst);
        true
    }
}

impl NotificationCheck for TaskInfo {
    fn notification_check(&self) -> bool {
        Version::from(self.common_data.version) == Version::API9 && !self.common_data.gauge
            || Version::from(self.common_data.version) == Version::API10
                && self.common_data.gauge
                && self.common_data.mode == Mode::BackGround.repr
    }
}

impl RequestTaskMsg {
    pub(crate) fn new(
        task_id: u32,
        uid: u64,
        action: Action,
        file_name: String,
        progress: Progress,
    ) -> Self {
        Self {
            task_id,
            uid: uid as i32,
            action: action.repr,
            file_name,
            index: progress.common_data.index,
            processed: progress.processed,
            sizes: progress.sizes,
        }
    }
}

pub struct TaskManagerWrapper {
    task_manager: TaskManagerTx,
}

impl TaskManagerWrapper {
    fn new(task_manager: TaskManagerTx) -> Self {
        Self { task_manager }
    }

    fn pause_task(&self, task_id: u32) -> bool {
        self.event_inner(task_id, TaskManagerEvent::pause)
    }

    fn resume_task(&self, task_id: u32) -> bool {
        self.event_inner(task_id, TaskManagerEvent::resume)
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
    fn query_task_uid(&self, task_id: u32) -> Option<u64> {
        let sql = format!("SELECT uid FROM request_task WHERE task_id = {}", task_id);
        self.query_integer(&sql)
            .first()
            .map(|uid: &u32| *uid as u64)
    }
}

#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {

    extern "Rust" {
        type TaskManagerWrapper;
        fn pause_task(&self, task_id: u32) -> bool;
        fn resume_task(&self, task_id: u32) -> bool;
    }

    #[derive(Debug)]
    struct RequestTaskMsg {
        pub(crate) task_id: u32,
        pub(crate) uid: i32,
        pub(crate) action: u8,
        pub(crate) file_name: String,
        pub(crate) index: usize,
        pub(crate) processed: Vec<usize>,
        pub(crate) sizes: Vec<i64>,
    }

    unsafe extern "C++" {
        include!("notification_bar.h");
        fn RequestNotifyProgress(msg: RequestTaskMsg);
        fn RequestNotifyFailed(action: u8, task_id: u32, uid: i32, file_name: String);
        fn RequestNotifyCompleted(action: u8, task_id: u32, uid: i32, file_name: String);
        fn SubscribeNotification(task_manager: Box<TaskManagerWrapper>);
    }
}
