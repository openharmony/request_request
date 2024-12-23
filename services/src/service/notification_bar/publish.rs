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

use std::sync::{Arc, LazyLock};

use ylong_runtime::fastrand::fast_random;
use ylong_runtime::sync::mpsc::{self, unbounded_channel};

use super::database::NotificationDb;
use super::notify_flow::{EventualNotify, NotifyFlow, NotifyInfo, ProgressNotify};
use super::task_handle::NotificationCheck;
use crate::info::TaskInfo;
use crate::task::request_task::RequestTask;

pub(crate) const NOTIFY_PROGRESS_INTERVAL: u64 = 500;

pub struct NotificationDispatcher {
    database: Arc<NotificationDb>,
    flow: mpsc::UnboundedSender<NotifyInfo>,
}

impl NotificationDispatcher {
    fn new() -> Self {
        let database = Arc::new(NotificationDb::new());
        let (tx, rx) = unbounded_channel();
        NotifyFlow::new(rx, database.clone()).run();
        Self {
            database: database.clone(),
            flow: tx,
        }
    }

    pub(crate) fn get_instance() -> &'static Self {
        static INSTANCE: LazyLock<NotificationDispatcher> =
            LazyLock::new(NotificationDispatcher::new);
        &INSTANCE
    }

    pub(crate) fn publish_progress_notification(&self, task: &RequestTask) {
        if !task.notification_check(false) {
            return;
        }
        let progress = task.progress.lock().unwrap();
        let mut total = Some(0);
        for size in progress.sizes.iter() {
            if *size < 0 {
                total = None;
                break;
            }
            *total.as_mut().unwrap() += *size as u64;
        }
        let multi_upload = match progress.sizes.len() {
            0 | 1 => None,
            len => Some((progress.common_data.index, len)),
        };
        let notify = ProgressNotify {
            action: task.action(),
            task_id: task.task_id(),
            uid: task.uid(),
            file_name: task.conf.file_specs[0].file_name.clone(),
            processed: progress.common_data.total_processed as u64,
            total,
            multi_upload,
        };
        let _ = self.flow.send(NotifyInfo::Progress(notify));
    }

    pub(crate) fn publish_success_notification(&self, info: &TaskInfo) {
        if !info.notification_check(true) {
            return;
        }
        let notify = EventualNotify {
            action: info.action(),
            task_id: info.common_data.task_id,
            uid: info.uid(),
            file_name: info.file_specs[0].file_name.clone(),
            is_successful: true,
        };
        let _ = self.flow.send(NotifyInfo::Eventual(notify));
    }

    pub(crate) fn publish_failed_notification(&self, info: &TaskInfo) {
        let notify = EventualNotify {
            action: info.action(),
            task_id: info.common_data.task_id,
            uid: info.uid(),
            file_name: info.file_specs[0].file_name.clone(),
            is_successful: false,
        };
        let _ = self.flow.send(NotifyInfo::Eventual(notify));
    }

    pub(crate) fn attach_group(&self, task_id: u32, group_id: u32) -> bool {
        info!("Attach task {} to group {}", task_id, group_id);
        if !self.database.attach_able(group_id) {
            return false;
        }

        self.database.update_task_group(task_id, group_id);
        true
    }

    pub(crate) fn delete_group(&self, group_id: u32, uid: u64) -> bool {
        info!("Delete group {}", group_id);
        if !self.database.attach_able(group_id) {
            return false;
        }
        self.database.disable_attach_group(group_id);
        let notify = NotifyInfo::GroupEventual(group_id, uid);
        let _ = self.flow.send(notify);
        true
    }

    pub(crate) fn create_group(
        &self,
        gauge: bool,
        customized: bool,
        title: String,
        text: String,
    ) -> u32 {
        let new_group_id = loop {
            let candidate = fast_random() as u32;
            if !self.database.contains_group(candidate) {
                break candidate;
            }
        };
        info!(
            "Create group {} gauge {} customized {}",
            new_group_id, gauge, customized
        );
        self.database.update_group_config(new_group_id, gauge);
        if customized {
            self.database
                .update_group_customized_notification(new_group_id, &title, &text);
        }
        new_group_id
    }
}
