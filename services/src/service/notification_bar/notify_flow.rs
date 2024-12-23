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

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;

use ylong_runtime::sync::mpsc::{self, UnboundedReceiver};

use super::database::NotificationDb;
use super::ffi::{NotifyContent, PublishNotification};
use crate::config::Action;
use crate::info::State;
use crate::manage::database::RequestDb;
use crate::utils::get_current_timestamp;

const NOTIFY_PROGRESS_INTERVAL: u64 = if cfg!(test) { 1 } else { 500 };

pub(crate) struct NotifyFlow {
    database: Arc<NotificationDb>,
    // key for task_id.
    notify_type_map: HashMap<u32, NotifyType>,
    // key for request_id, group or task.
    last_notify_map: HashMap<u32, u64>,
    group_notify_progress: HashMap<u32, GroupProgress>,
    // value 1 for title, 2 for text.
    group_customized_notify: HashMap<u32, Option<(String, String)>>,
    group_gauge: HashMap<u32, bool>,
    task_customized_notify: HashMap<u32, Option<(String, String)>>,
    rx: mpsc::UnboundedReceiver<NotifyInfo>,
}

pub(crate) struct GroupProgress {
    task_progress: HashMap<u32, u64>,
    total_progress: u64,
    task_state: HashMap<u32, State>,
    successful: usize,
    failed: usize,
}

impl GroupProgress {
    pub(crate) fn new() -> Self {
        Self {
            task_progress: HashMap::new(),
            total_progress: 0,
            task_state: HashMap::new(),
            successful: 0,
            failed: 0,
        }
    }

    pub(crate) fn update_task_progress(&mut self, task_id: u32, processed: u64) {
        let prev = match self.task_progress.get_mut(&task_id) {
            Some(prev) => prev,
            None => {
                self.task_progress.insert(task_id, 0);
                self.task_progress.get_mut(&task_id).unwrap()
            }
        };
        self.total_progress += processed - *prev;
        *prev = processed;
    }

    pub(crate) fn update_task_state(&mut self, task_id: u32, state: State) {
        let prev = match self.task_state.get_mut(&task_id) {
            Some(prev) => prev,
            None => {
                self.task_state.insert(task_id, state);
                if state == State::Completed {
                    self.successful += 1;
                } else if state == State::Failed {
                    self.failed += 1;
                }
                return;
            }
        };
        if *prev == state {
            return;
        }
        if *prev != State::Completed && *prev != State::Failed {
            if state == State::Completed {
                self.successful += 1;
            } else if state == State::Failed {
                self.failed += 1;
            }
        } else if state == State::Completed {
            self.successful += 1;
            self.failed -= 1;
        } else if state == State::Failed {
            self.failed += 1;
            self.successful -= 1;
        }
        *prev = state;
    }

    pub(crate) fn successful(&self) -> usize {
        self.successful
    }

    pub(crate) fn failed(&self) -> usize {
        self.failed
    }

    pub(crate) fn total(&self) -> usize {
        self.task_state.len()
    }
    pub(crate) fn processed(&self) -> u64 {
        self.total_progress
    }

    pub(crate) fn is_finish(&self) -> bool {
        self.total() == self.successful + self.failed
    }
}

#[derive(Clone, Debug)]
pub struct ProgressNotify {
    pub(crate) action: Action,
    pub(crate) task_id: u32,
    pub(crate) uid: u64,
    pub(crate) processed: u64,
    pub(crate) total: Option<u64>,
    pub(crate) multi_upload: Option<(usize, usize)>,
    pub(crate) file_name: String,
}

#[derive(Clone, Debug)]
pub(crate) struct EventualNotify {
    pub(crate) action: Action,
    pub(crate) task_id: u32,
    pub(crate) uid: u64,
    pub(crate) file_name: String,
    pub(crate) is_successful: bool,
}

#[derive(Debug)]
pub(crate) enum NotifyInfo {
    Eventual(EventualNotify),
    Progress(ProgressNotify),
    GroupEventual(u32, u64),
}

#[derive(Clone, Copy)]
enum NotifyType {
    Group(u32),
    Task,
}

impl NotifyFlow {
    pub(crate) fn new(rx: UnboundedReceiver<NotifyInfo>, database: Arc<NotificationDb>) -> Self {
        Self {
            database,
            notify_type_map: HashMap::new(),
            last_notify_map: HashMap::new(),
            group_notify_progress: HashMap::new(),
            group_gauge: HashMap::new(),
            task_customized_notify: HashMap::new(),
            group_customized_notify: HashMap::new(),
            rx,
        }
    }

    pub(crate) fn run(mut self) {
        ylong_runtime::spawn(async move {
            loop {
                let info = match self.rx.recv().await {
                    Ok(message) => message,
                    Err(e) => {
                        error!("Notification flow channel error: {:?}", e);
                        continue;
                    }
                };

                if let Some(content) = match info {
                    NotifyInfo::Eventual(info) => self.publish_completed_notify(&info),
                    NotifyInfo::Progress(info) => self.publish_progress_notification(info),
                    NotifyInfo::GroupEventual(group_id, uid) => self.group_eventual(group_id, uid),
                } {
                    PublishNotification(&content);
                }
            }
        });
    }

    pub(crate) fn get_customized_notification(
        &mut self,
        request_id: u32,
        is_group: bool,
    ) -> Option<(String, String)> {
        if is_group {
            let info = self
                .database
                .query_group_customized_notification(request_id)?;
            Some((info.title, info.text))
        } else {
            let info = self
                .database
                .query_task_customized_notification(request_id)?;
            Some((info.title, info.text))
        }
    }

    fn get_group_progress(&self, group_id: u32) -> GroupProgress {
        let mut group_progress = GroupProgress::new();
        for task_id in self.database.query_group_tasks(group_id) {
            let Some(processed) = RequestDb::get_instance().query_task_total_processed(task_id)
            else {
                error!("Failed to get {} info in group {}", task_id, group_id);
                continue;
            };
            let Some(state) = RequestDb::get_instance().query_task_state(task_id) else {
                error!("Failed to get {} state in group {}", task_id, group_id);
                continue;
            };
            if state == State::Removed.repr {
                continue;
            }
            group_progress.update_task_state(task_id, State::from(state));
            group_progress.update_task_progress(task_id, processed as u64);
        }
        group_progress
    }

    pub(crate) fn publish_progress_notification(
        &mut self,
        info: ProgressNotify,
    ) -> Option<NotifyContent> {
        let content = match self.get_request_id(info.task_id) {
            NotifyType::Group(group_id) => {
                let gauge = match self.group_gauge.get(&group_id) {
                    Some(gauge) => *gauge,
                    None => {
                        let gauge = self.database.is_gauge(group_id);
                        self.group_gauge.insert(group_id, gauge);
                        gauge
                    }
                };
                if !gauge {
                    return None;
                }

                let progress_interval_check = self.progress_interval_check(group_id);
                if let Some(customized) = match self.group_customized_notify.get(&group_id) {
                    Some(customized) => customized,
                    None => {
                        let customized = self.get_customized_notification(group_id, true);
                        self.group_customized_notify.insert(group_id, customized);
                        self.group_customized_notify.get(&group_id).unwrap()
                    }
                } {
                    if !progress_interval_check {
                        return None;
                    };
                    NotifyContent::customized_notify(
                        group_id,
                        info.uid as u32,
                        customized.0.clone(),
                        customized.1.clone(),
                        true,
                    )
                } else {
                    match self.group_notify_progress.get_mut(&group_id) {
                        Some(progress) => {
                            progress.update_task_progress(info.task_id, info.processed)
                        }
                        None => {
                            let mut progress = self.get_group_progress(group_id);
                            progress.update_task_progress(info.task_id, info.processed);
                            self.group_notify_progress.insert(group_id, progress);
                        }
                    };
                    if !progress_interval_check {
                        return None;
                    }
                    NotifyContent::default_group_progress_notify(
                        info.action,
                        group_id,
                        info.uid as u32,
                        self.group_notify_progress.get(&group_id).unwrap(),
                    )
                }
            }
            NotifyType::Task => {
                if let Some(customized) = match self.task_customized_notify.get(&info.task_id) {
                    Some(customized) => customized,
                    None => {
                        let customized = self.get_customized_notification(info.task_id, false);
                        self.group_customized_notify
                            .insert(info.task_id, customized);
                        self.group_customized_notify.get(&info.task_id).unwrap()
                    }
                } {
                    NotifyContent::customized_notify(
                        info.task_id,
                        info.uid as u32,
                        customized.0.clone(),
                        customized.1.clone(),
                        true,
                    )
                } else {
                    NotifyContent::default_task_progress_notify(&info)
                }
            }
        };
        Some(content)
    }

    fn progress_interval_check(&mut self, request_id: u32) -> bool {
        match self.last_notify_map.entry(request_id) {
            Entry::Occupied(mut entry) => {
                let last_notify = entry.get_mut();
                let current = get_current_timestamp();
                if current < NOTIFY_PROGRESS_INTERVAL + *last_notify {
                    return false;
                }
                *last_notify = current;
                true
            }
            Entry::Vacant(entry) => {
                let last_notify = get_current_timestamp();
                entry.insert(last_notify);
                true
            }
        }
    }

    fn publish_completed_notify(&mut self, info: &EventualNotify) -> Option<NotifyContent> {
        let content = match self.get_request_id(info.task_id) {
            NotifyType::Group(group_id) => {
                let group_progress = match self.group_notify_progress.get_mut(&group_id) {
                    Some(progress) => {
                        if info.is_successful {
                            progress.update_task_state(info.task_id, State::Completed);
                        } else {
                            progress.update_task_state(info.task_id, State::Failed);
                        }
                        progress
                    }
                    None => {
                        let progress = self.get_group_progress(group_id);
                        self.group_notify_progress.insert(group_id, progress);
                        self.group_notify_progress.get_mut(&group_id).unwrap()
                    }
                };
                let total_progress = group_progress.processed();
                let successful = group_progress.successful() as i32;
                let failed = group_progress.failed() as i32;

                if let Some(customized) = match self.group_customized_notify.get(&group_id) {
                    Some(customized) => customized,
                    None => {
                        let customized = self.get_customized_notification(group_id, true);
                        self.group_customized_notify.insert(group_id, customized);
                        self.group_customized_notify.get(&group_id).unwrap()
                    }
                } {
                    let title = customized.0.clone();
                    let text = customized.1.clone();
                    if !self.group_eventual_check(group_id) {
                        return None;
                    }
                    NotifyContent::customized_notify(group_id, info.uid as u32, title, text, false)
                } else {
                    if !self.group_eventual_check(group_id) {
                        return None;
                    }
                    NotifyContent::default_group_eventual_notify(
                        info.action,
                        group_id,
                        info.uid as u32,
                        total_progress,
                        successful,
                        failed,
                    )
                }
            }
            NotifyType::Task => {
                if let Some(customized) = self
                    .database
                    .query_task_customized_notification(info.task_id)
                {
                    NotifyContent::customized_notify(
                        info.task_id,
                        info.uid as u32,
                        customized.title,
                        customized.text,
                        false,
                    )
                } else {
                    NotifyContent::default_task_eventual_notify(
                        info.action,
                        info.task_id,
                        info.uid as u32,
                        info.file_name.clone(),
                        info.is_successful,
                    )
                }
            }
        };
        Some(content)
    }

    fn group_eventual(&mut self, group_id: u32, uid: u64) -> Option<NotifyContent> {
        if let Some(customized) = match self.group_customized_notify.get(&group_id) {
            Some(customized) => customized,
            None => {
                let customized = self.get_customized_notification(group_id, true);
                self.group_customized_notify.insert(group_id, customized);
                self.group_customized_notify.get(&group_id).unwrap()
            }
        } {
            let title = customized.0.clone();
            let text = customized.1.clone();
            if !self.group_eventual_check(group_id) {
                return None;
            }
            Some(NotifyContent::customized_notify(
                group_id, uid as u32, title, text, false,
            ))
        } else {
            let group_progress = match self.group_notify_progress.get_mut(&group_id) {
                Some(progress) => progress,
                None => {
                    let progress = self.get_group_progress(group_id);
                    self.group_notify_progress.insert(group_id, progress);
                    self.group_notify_progress.get_mut(&group_id).unwrap()
                }
            };
            let total_progress = group_progress.processed();
            let successful = group_progress.successful() as i32;
            let failed = group_progress.failed() as i32;
            if !self.group_eventual_check(group_id) {
                return None;
            }
            Some(NotifyContent::default_group_eventual_notify(
                Action::Download,
                group_id,
                uid as u32,
                total_progress,
                successful,
                failed,
            ))
        }
    }

    fn get_request_id(&mut self, task_id: u32) -> NotifyType {
        if let Some(n_type) = self.notify_type_map.get(&task_id) {
            return *n_type;
        }
        let n_type = match self.database.query_task_gid(task_id) {
            Some(group_id) => NotifyType::Group(group_id),
            None => NotifyType::Task,
        };

        self.notify_type_map.insert(task_id, n_type);
        n_type
    }

    fn group_eventual_check(&mut self, group_id: u32) -> bool {
        let group_progress = match self.group_notify_progress.get_mut(&group_id) {
            Some(progress) => progress,
            None => {
                let progress = self.get_group_progress(group_id);
                self.group_notify_progress.insert(group_id, progress);
                self.group_notify_progress.get_mut(&group_id).unwrap()
            }
        };
        !self.database.attach_able(group_id) && group_progress.is_finish()
    }
}

