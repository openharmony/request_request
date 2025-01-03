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

mod database;
mod notify_flow;
mod publish;
mod task_handle;
mod typology;

pub use publish::NotificationDispatcher;
pub(crate) use publish::NOTIFY_PROGRESS_INTERVAL;
pub(crate) use task_handle::subscribe_notification_bar;
use task_handle::TaskManagerWrapper;

#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    #[derive(Eq, PartialEq, Debug)]
    pub(crate) struct NotifyContent {
        title: String,
        text: String,
        request_id: u32,
        uid: u32,
        live_view: bool,
        progress_circle: ProgressCircle,
        x_mark: bool,
    }

    #[derive(Eq, PartialEq, Debug)]
    struct ProgressCircle {
        open: bool,
        current: u64,
        total: u64,
    }

    extern "Rust" {
        type TaskManagerWrapper;
        fn pause_task(self: &TaskManagerWrapper, task_id: u32) -> bool;
        fn resume_task(self: &TaskManagerWrapper, task_id: u32) -> bool;
        fn stop_task(self: &TaskManagerWrapper, task_id: u32) -> bool;
    }

    unsafe extern "C++" {
        include!("notification_bar.h");

        fn CancelNotification(notificationId: u32) -> i32;
        fn GetSystemResourceString(name: &str) -> String;
        fn PublishNotification(content: &NotifyContent) -> i32;
        fn SubscribeNotification(task_manager: Box<TaskManagerWrapper>);
    }
}
