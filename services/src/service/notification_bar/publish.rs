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

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, Mutex};

use ylong_runtime::fastrand::fast_random;
use ylong_runtime::sync::mpsc::{self, unbounded_channel};

use super::database::NotificationDb;
use super::notify_flow::{EventualNotify, NotifyFlow, NotifyInfo, ProgressNotify};
use super::task_handle::{cancel_notification, NotificationCheck};
use crate::info::TaskInfo;
use crate::service::notification_bar::NotificationConfig;
use crate::task::request_task::RequestTask;
use crate::utils::get_current_duration;

pub(crate) const NOTIFY_PROGRESS_INTERVAL: u64 = 500;

pub struct NotificationDispatcher {
    database: Arc<NotificationDb>,
    task_gauge: Mutex<HashMap<u32, Arc<AtomicBool>>>,
    flow: mpsc::UnboundedSender<NotifyInfo>,
}

impl NotificationDispatcher {
    fn new() -> Self {
        let database = Arc::new(NotificationDb::new());
        let (tx, rx) = unbounded_channel();
        NotifyFlow::new(rx, database.clone()).run();
        Self {
            database: database.clone(),
            task_gauge: Mutex::new(HashMap::new()),
            flow: tx,
        }
    }

    pub(crate) fn get_instance() -> &'static Self {
        static INSTANCE: LazyLock<NotificationDispatcher> =
            LazyLock::new(NotificationDispatcher::new);
        &INSTANCE
    }

    pub(crate) fn clear_task_info(&self, task_id: u32) {
        self.database.clear_task_info(task_id);
    }

    pub(crate) fn clear_group_info(&self) {
        self.database.clear_group_info_a_week_ago();
    }

    pub(crate) fn disable_task_notification(&self, uid: u64, task_id: u32) {
        self.database.disable_task_notification(task_id);
        self.unregister_task(uid, task_id, true);
    }

    pub(crate) fn enable_task_progress_notification(&self, task_id: u32) {
        if let Some(gauge) = self.task_gauge.lock().unwrap().get(&task_id) {
            gauge.store(true, Ordering::Release);
        }
    }

    pub(crate) fn update_task_customized_notification(&self, config: &NotificationConfig) {
        self.database.update_task_customized_notification(config);
    }

    pub(crate) fn check_task_notification_available(&self, task_id: u32) -> bool {
        self.database.check_task_notification_available(&task_id)
    }

    /// Get the gauge value for a specific task
    /// Returns None if the task doesn't exist
    pub(crate) fn get_task_gauge(&self, task_id: u32) -> Option<bool> {
        self.task_gauge.lock().ok()?.get(&task_id).map(|gauge| gauge.load(Ordering::Acquire))
    }

    pub(crate) fn register_task(&self, task: &RequestTask) -> Arc<AtomicBool> {
        let gauge = if let Some(gid) = self.database.query_task_gid(task.task_id()) {
            if self.database.check_group_notification_available(&gid) {
                Arc::new(AtomicBool::new(true))
            } else {
                Arc::new(AtomicBool::new(false))
            }
        } else {
            let gauge = task.notification_check(&self.database);
            Arc::new(AtomicBool::new(gauge))
        };
        self.task_gauge
            .lock()
            .unwrap()
            .insert(task.task_id(), gauge.clone());
        gauge
    }

    pub(crate) fn unregister_task(&self, uid: u64, task_id: u32, affect_group: bool) {
        match (
            self.task_gauge.lock().unwrap().get(&task_id).cloned(),
            self.database.query_task_gid(task_id),
        ) {
            (Some(gauge), Some(gid)) => {
                if affect_group {
                    gauge.store(false, Ordering::Release);
                    let _ = self.flow.send(NotifyInfo::Unregister(uid, task_id, gid));
                }
            }
            (None, Some(gid)) => {
                if affect_group {
                    let _ = self.flow.send(NotifyInfo::Unregister(uid, task_id, gid));
                }
            }
            (Some(gauge), None) => {
                gauge.store(false, Ordering::Release);
                cancel_notification(task_id);
            }
            (None, None) => {}
        }
    }

    pub(crate) fn publish_progress_notification(&self, task: &RequestTask) {
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
            file_name: match task.conf.file_specs.first() {
                Some(spec) => spec.file_name.clone(),
                None => {
                    error!("Failed to get the first file_spec from an empty vector in TaskConfig");
                    String::new()
                }
            },
            processed: progress.common_data.total_processed as u64,
            total,
            multi_upload,
            version: task.conf.version,
        };
        let _ = self.flow.send(NotifyInfo::Progress(notify));
    }

    pub(crate) fn publish_success_notification(&self, info: &TaskInfo) {
        self.task_gauge
            .lock()
            .unwrap()
            .remove(&info.common_data.task_id);
        if !info.notification_check(&self.database) {
            return;
        }
        let notify = EventualNotify {
            action: info.action(),
            task_id: info.common_data.task_id,
            processed: info.progress.common_data.total_processed as u64,
            uid: info.uid(),
            file_name: match info.file_specs.first() {
                Some(spec) => spec.file_name.clone(),
                None => {
                    error!("Failed to get the first file_spec from an empty vector in TaskInfo");
                    String::new()
                }
            },
            is_successful: true,
        };
        let _ = self.flow.send(NotifyInfo::Eventual(notify));
    }

    pub(crate) fn publish_failed_notification(&self, info: &TaskInfo) {
        self.task_gauge
            .lock()
            .unwrap()
            .remove(&info.common_data.task_id);
        if !info.notification_check(&self.database) {
            return;
        }
        let notify = EventualNotify {
            action: info.action(),
            task_id: info.common_data.task_id,
            processed: info.progress.common_data.total_processed as u64,
            uid: info.uid(),
            file_name: match info.file_specs.first() {
                Some(spec) => spec.file_name.clone(),
                None => {
                    error!("Failed to get the first file_spec from an empty vector in TaskInfo");
                    String::new()
                }
            },
            is_successful: false,
        };
        let _ = self.flow.send(NotifyInfo::Eventual(notify));
    }

    pub(crate) fn attach_group(&self, task_ids: Vec<u32>, group_id: u32, uid: u64) -> bool {
        if !self.database.attach_able(group_id) {
            return false;
        }
        info!("Attach task {:?} to group {}", task_ids, group_id);
        let is_gauge = self.database.is_gauge(group_id);
        for task_id in task_ids.iter().copied() {
            self.database.update_task_group(task_id, group_id);
            if let Some(gauge) = self.task_gauge.lock().unwrap().get(&task_id) {
                gauge.store(is_gauge, std::sync::atomic::Ordering::Release);
            }
        }
        if !self.database.check_group_notification_available(&group_id) {
            return true;
        }

        let _ = self
            .flow
            .send(NotifyInfo::AttachGroup(group_id, uid, task_ids));
        true
    }

    pub(crate) fn delete_group(&self, group_id: u32, uid: u64) -> bool {
        info!("Delete group {}", group_id);
        if !self.database.attach_able(group_id) {
            return false;
        }
        self.database.disable_attach_group(group_id);
        if !self.database.check_group_notification_available(&group_id) {
            return true;
        }
        let notify = NotifyInfo::GroupEventual(group_id, uid);
        let _ = self.flow.send(notify);
        true
    }

    pub(crate) fn create_group(
        &self,
        gauge: bool,
        title: Option<String>,
        text: Option<String>,
        want_agent: Option<String>,
        disable: bool,
        visibility: u32,
    ) -> u32 {
        let new_group_id = loop {
            let candidate = fast_random() as u32;
            if !self.database.contains_group(candidate) {
                break candidate;
            }
        };
        info!(
            "Create group {} gauge {} customized_title {:?} customized_text {:?} want_agent {:?} disable {} visibility {}",
            new_group_id, gauge, title, text, want_agent, disable, visibility
        );

        let current_time = get_current_duration().as_millis() as u64;
        self.database
            .update_group_config(new_group_id, gauge, current_time, !disable, visibility);
        if title.is_some() || text.is_some() || want_agent.is_some() {
            self.database
                .update_group_customized_notification(new_group_id, title, text, want_agent);
        }
        new_group_id
    }
}
