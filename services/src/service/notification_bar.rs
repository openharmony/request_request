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
const NOTIFY_PROGRESS_INTERVAL: u64 = 200;

pub(crate) fn publish_progress_notification(task: &RequestTask) {
    if !task.notification_check(false) {
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
    ffi::RequestProgressNotification(msg);
}

pub(crate) fn publish_success_notification(info: &TaskInfo) {
    if !info.notification_check(true) {
        return;
    }
    ffi::RequestCompletedNotification(
        info.action().repr,
        info.common_data.task_id,
        info.common_data.uid as i32,
        info.file_specs[info.progress.common_data.index]
            .file_name
            .clone(),
        true,
    );
}

pub(crate) fn publish_failed_notification(info: &TaskInfo) {
    if !info.notification_check(true) {
        return;
    }
    ffi::RequestCompletedNotification(
        info.action().repr,
        info.common_data.task_id,
        info.common_data.uid as i32,
        info.file_specs[info.progress.common_data.index]
            .file_name
            .clone(),
        false,
    );
}

pub(crate) fn cancel_progress_notification(info: &TaskInfo) {
    if !info.notification_check(false) {
        return;
    }
    force_cancel_progress_notification(info.common_data.task_id);
}

pub(crate) fn force_cancel_progress_notification(task_id: u32) {
    let ret = ffi::CancelNotification(task_id);
    if ret != 0 {
        error!("cancel notification failed {}", ret);
    }
}

trait NotificationCheck {
    fn notification_check(&self, completed_notify: bool) -> bool;
}

impl NotificationCheck for RequestTask {
    fn notification_check(&self, completed_notify: bool) -> bool {
        if !notification_check_common(
            self.conf.version,
            self.conf.common_data.gauge,
            self.conf.common_data.mode,
            self.conf.common_data.background,
            completed_notify,
        ) {
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
    fn notification_check(&self, completed_notify: bool) -> bool {
        notification_check_common(
            Version::from(self.common_data.version),
            self.common_data.gauge,
            Mode::from(self.common_data.mode),
            RequestDb::get_instance().query_task_background(self.common_data.task_id),
            completed_notify,
        )
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

impl RequestTaskMsg {
    pub(crate) fn new(
        task_id: u32,
        uid: u64,
        action: Action,
        file_name: String,
        progress: Progress,
    ) -> Self {
        let extras = progress.extras;
        let support_range = extras.contains_key("etag") || extras.contains_key("last-modified");
        Self {
            task_id,
            uid: uid as i32,
            action: action.repr,
            file_name,
            index: progress.common_data.index,
            processed: progress.processed,
            sizes: progress.sizes,
            support_range,
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

    fn stop_task(&self, task_id: u32) -> bool {
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
}

#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {

    extern "Rust" {
        type TaskManagerWrapper;
        fn pause_task(&self, task_id: u32) -> bool;
        fn resume_task(&self, task_id: u32) -> bool;
        fn stop_task(&self, task_id: u32) -> bool;
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
        pub(crate) support_range: bool,
    }

    unsafe extern "C++" {
        include!("notification_bar.h");
        fn RequestProgressNotification(msg: RequestTaskMsg);
        fn CancelNotification(notificationId: u32) -> i32;
        fn RequestCompletedNotification(
            action: u8,
            task_id: u32,
            uid: i32,
            file_name: String,
            is_succeed: bool,
        );
        fn SubscribeNotification(task_manager: Box<TaskManagerWrapper>);
    }
}
