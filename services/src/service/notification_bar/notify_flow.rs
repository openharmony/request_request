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

use super::database::{CustomizedNotification, NotificationDb};
use super::ffi::{NotifyContent, PublishNotification};
use super::task_handle::cancel_notification;
use crate::config::Action;
use crate::info::State;
use crate::manage::database::RequestDb;
use crate::utils::{get_current_timestamp, runtime_spawn};

const NOTIFY_PROGRESS_INTERVAL: u64 = if cfg!(test) { 1 } else { 500 };

pub(crate) struct NotifyFlow {
    database: Arc<NotificationDb>,
    // key for task_id.
    notify_type_map: HashMap<u32, NotifyType>,

    // key for request_id, group or task.
    last_notify_map: HashMap<u32, u64>,

    group_notify_progress: HashMap<u32, GroupProgress>,
    // value 1 for title, 2 for text.
    group_customized_notify: HashMap<u32, Option<CustomizedNotification>>,
    group_gauge: HashMap<u32, bool>,
    task_customized_notify: HashMap<u32, Option<CustomizedNotification>>,
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
        let prev = match self.task_progress.entry(task_id) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(0),
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
    pub(crate) processed: u64,
    pub(crate) file_name: String,
    pub(crate) is_successful: bool,
}

#[derive(Debug)]
pub(crate) enum NotifyInfo {
    Eventual(EventualNotify),
    Progress(ProgressNotify),
    AttachGroup(u32, u64, Vec<u32>),
    Unregister(u64, u32, u32),
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
        runtime_spawn(async move {
            loop {
                let info = match self.rx.recv().await {
                    Ok(message) => message,
                    Err(e) => {
                        error!("Notification flow channel error: {:?}", e);
                        sys_event!(
                            ExecFault,
                            DfxCode::UDS_FAULT_03,
                            &format!("Notification flow channel error: {:?}", e)
                        );
                        continue;
                    }
                };

                if let Some(content) = match info {
                    NotifyInfo::Eventual(info) => self.publish_completed_notify(&info),
                    NotifyInfo::Progress(info) => self.publish_progress_notification(info),
                    NotifyInfo::GroupEventual(group_id, uid) => self.group_eventual(group_id, uid),
                    NotifyInfo::AttachGroup(group_id, uid, task_ids) => {
                        self.attach_group(group_id, task_ids, uid)
                    }
                    NotifyInfo::Unregister(uid, task_id, group_id) => {
                        self.unregister_task(uid, task_id, group_id)
                    }
                } {
                    PublishNotification(&content);
                }
            }
        });
    }

    fn unregister_task(&mut self, uid: u64, task_id: u32, group_id: u32) -> Option<NotifyContent> {
        info!(
            "Unregister task: uid: {}, task_id: {}, group_id: {}",
            uid, task_id, group_id
        );
        let customized = self.group_customized_notify(group_id);
        let progress = match self.group_notify_progress.entry(group_id) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let progress = Self::get_group_progress(&self.database, group_id);
                entry.insert(progress)
            }
        };
        if progress
            .task_state
            .get(&task_id)
            .is_some_and(|state| *state != State::Completed && *state != State::Failed)
        {
            progress.task_state.remove(&task_id);
        }
        if progress.task_state.is_empty() {
            cancel_notification(group_id);
            return None;
        }
        if !Self::group_eventual_check(&self.database, progress, group_id) {
            return None;
        }
        Some(NotifyContent::group_eventual_notify(
            customized,
            Action::Download,
            group_id,
            uid as u32,
            progress.processed(),
            progress.successful() as i32,
            progress.failed() as i32,
        ))
    }

    fn update_db_task_state_and_progress(group_progress: &mut GroupProgress, task_id: u32) {
        let Some(processed) = RequestDb::get_instance().query_task_total_processed(task_id) else {
            return;
        };
        let Some(state) = RequestDb::get_instance().query_task_state(task_id) else {
            return;
        };
        if state == State::Removed.repr {
            return;
        }
        group_progress.update_task_state(task_id, State::from(state));
        group_progress.update_task_progress(task_id, processed as u64);
    }

    fn get_group_progress(database: &NotificationDb, group_id: u32) -> GroupProgress {
        let mut group_progress = GroupProgress::new();
        for task_id in database.query_group_tasks(group_id) {
            Self::update_db_task_state_and_progress(&mut group_progress, task_id);
        }
        group_progress
    }

    fn attach_group(
        &mut self,
        group_id: u32,
        task_ids: Vec<u32>,
        uid: u64,
    ) -> Option<NotifyContent> {
        let is_gauge = self.check_gauge(group_id);
        let customized = self.group_customized_notify(group_id);
        let progress = match self.group_notify_progress.entry(group_id) {
            Entry::Occupied(entry) => {
                let progress = entry.into_mut();
                for task_id in task_ids {
                    Self::update_db_task_state_and_progress(progress, task_id);
                }
                progress
            }
            Entry::Vacant(entry) => {
                let progress = Self::get_group_progress(&self.database, group_id);
                entry.insert(progress)
            }
        };
        if !is_gauge {
            return None;
        }
        Some(NotifyContent::group_progress_notify(
            customized,
            Action::Download,
            group_id,
            uid as u32,
            progress,
        ))
    }

    fn check_gauge(&mut self, group_id: u32) -> bool {
        match self.group_gauge.get(&group_id) {
            Some(gauge) => *gauge,
            None => {
                let gauge = self.database.is_gauge(group_id);
                self.group_gauge.insert(group_id, gauge);
                gauge
            }
        }
    }

    fn group_customized_notify(&mut self, group_id: u32) -> Option<CustomizedNotification> {
        match self.group_customized_notify.entry(group_id) {
            Entry::Occupied(entry) => entry.get().clone(),
            Entry::Vacant(entry) => {
                let customized = self.database.query_group_customized_notification(group_id);
                entry.insert(customized).clone()
            }
        }
    }

    fn task_customized_notify(&mut self, task_id: u32) -> Option<CustomizedNotification> {
        match self.task_customized_notify.entry(task_id) {
            Entry::Occupied(entry) => entry.get().clone(),
            Entry::Vacant(entry) => {
                let customized = self.database.query_task_customized_notification(task_id);
                entry.insert(customized).clone()
            }
        }
    }

    fn publish_progress_notification(&mut self, info: ProgressNotify) -> Option<NotifyContent> {
        let content = match self.get_request_id(info.task_id) {
            NotifyType::Group(group_id) => {
                if !self.check_gauge(group_id) {
                    return None;
                }
                let progress_interval_check = self.progress_interval_check(group_id);

                let customized = self.group_customized_notify(group_id);
                let progress = match self.group_notify_progress.entry(group_id) {
                    Entry::Occupied(entry) => entry.into_mut(),
                    Entry::Vacant(entry) => {
                        let progress = Self::get_group_progress(&self.database, group_id);
                        entry.insert(progress)
                    }
                };
                progress.update_task_progress(info.task_id, info.processed);

                if !progress_interval_check {
                    return None;
                }
                NotifyContent::group_progress_notify(
                    customized,
                    info.action,
                    group_id,
                    info.uid as u32,
                    progress,
                )
            }
            NotifyType::Task => NotifyContent::task_progress_notify(
                self.task_customized_notify(info.task_id),
                &info,
            ),
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
                let is_gauge = self.check_gauge(group_id);

                let customized = self.group_customized_notify(group_id);
                let group_progress = match self.group_notify_progress.entry(group_id) {
                    Entry::Occupied(entry) => {
                        let progress = entry.into_mut();
                        progress.update_task_progress(info.task_id, info.processed);
                        if info.is_successful {
                            progress.update_task_state(info.task_id, State::Completed);
                        } else {
                            progress.update_task_state(info.task_id, State::Failed);
                        }
                        progress
                    }
                    Entry::Vacant(entry) => {
                        let progress = Self::get_group_progress(&self.database, group_id);
                        entry.insert(progress)
                    }
                };

                let group_eventual =
                    Self::group_eventual_check(&self.database, group_progress, group_id);

                if !group_eventual {
                    if is_gauge {
                        NotifyContent::group_progress_notify(
                            customized,
                            info.action,
                            group_id,
                            info.uid as u32,
                            group_progress,
                        )
                    } else {
                        return None;
                    }
                } else {
                    self.database.clear_group_info(group_id);
                    NotifyContent::group_eventual_notify(
                        customized,
                        info.action,
                        group_id,
                        info.uid as u32,
                        group_progress.processed(),
                        group_progress.successful() as i32,
                        group_progress.failed() as i32,
                    )
                }
            }
            NotifyType::Task => {
                let content = NotifyContent::task_eventual_notify(
                    self.task_customized_notify(info.task_id),
                    info.action,
                    info.task_id,
                    info.uid as u32,
                    info.file_name.clone(),
                    info.is_successful,
                );
                if info.is_successful {
                    self.database.clear_task_info(info.task_id);
                }
                content
            }
        };
        Some(content)
    }

    fn group_eventual(&mut self, group_id: u32, uid: u64) -> Option<NotifyContent> {
        let customized = self.group_customized_notify(group_id);
        let group_progress = match self.group_notify_progress.entry(group_id) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let progress = Self::get_group_progress(&self.database, group_id);
                entry.insert(progress)
            }
        };

        let group_eventual = Self::group_eventual_check(&self.database, group_progress, group_id);

        if !group_eventual {
            return None;
        }
        Some(NotifyContent::group_eventual_notify(
            customized,
            Action::Download,
            group_id,
            uid as u32,
            group_progress.processed(),
            group_progress.successful() as i32,
            group_progress.failed() as i32,
        ))
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

    fn group_eventual_check(
        database: &NotificationDb,
        group_progress: &mut GroupProgress,
        group_id: u32,
    ) -> bool {
        !database.attach_able(group_id) && group_progress.is_finish()
    }
}

#[cfg(test)]
mod test {

    use ylong_runtime::fastrand::fast_random;

    use super::*;

    const TEST_TITLE: &str = "test_title";
    const TEST_TEXT: &str = "test_text";

    // @tc.name: ut_notify_flow_group
    // @tc.desc: Test group progress calculation and state updates
    // @tc.precon: NA
    // @tc.step: 1. Create a GroupProgress instance
    //           2. Update 100 tasks with Running state
    //           3. Update each task's progress to 100 and set alternating states to Completed/Failed
    //           4. Verify processed progress, successful and failed counts
    //           5. Update all tasks' progress to 150 and set all states to Completed
    //           6. Verify final progress and state counts
    // @tc.expect: Group progress and state counts update correctly through all operations
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
    #[test]
    fn ut_notify_flow_group() {
        let mut group_progress = GroupProgress::new();

        for i in 0..100 {
            group_progress.update_task_state(i, State::Running);
        }
        assert_eq!(group_progress.successful(), 0);
        assert_eq!(group_progress.failed(), 0);
        assert_eq!(group_progress.total(), 100);

        for i in 0..100 {
            group_progress.update_task_progress(i, 100);
            if i % 2 == 0 {
                group_progress.update_task_state(i, State::Completed);
            } else {
                group_progress.update_task_state(i, State::Failed);
            }
        }
        assert_eq!(group_progress.processed(), 100 * 100);
        assert_eq!(group_progress.successful(), 50);
        assert_eq!(group_progress.failed(), 50);
        assert_eq!(group_progress.total(), 100);
        for i in 0..100 {
            group_progress.update_task_progress(i, 150);
            group_progress.update_task_state(i, State::Completed);
        }
        assert_eq!(group_progress.processed(), 100 * 150);
        assert_eq!(group_progress.successful(), 100);
        assert_eq!(group_progress.failed(), 0);
        assert_eq!(group_progress.total(), 100);
    }

    // @tc.name: ut_notify_flow_task_progress
    // @tc.desc: Test task progress notification generation
    // @tc.precon: NA
    // @tc.step: 1. Create a NotifyFlow instance with test channel
    //           2. Generate random task ID and UID
    //           3. Create ProgressNotify with test parameters
    //           4. Call publish_progress_notification
    //           5. Verify returned notification content matches expected default
    // @tc.expect: Task progress notification content is correctly generated
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
    #[test]
    fn ut_notify_flow_task_progress() {
        let (_, rx) = mpsc::unbounded_channel();
        let db = Arc::new(NotificationDb::new());
        let mut flow = NotifyFlow::new(rx, db);
        let task_id = fast_random() as u32;
        let uid = fast_random();
        let progress = ProgressNotify {
            action: Action::Download,
            task_id,
            uid,
            processed: 0,
            total: Some(100),
            multi_upload: None,
            file_name: "test".to_string(),
        };
        let content_default = NotifyContent::task_progress_notify(None, &progress);
        let content = flow
            .publish_progress_notification(progress.clone())
            .unwrap();
        assert_eq!(content, content_default);
    }

    // @tc.name: ut_notify_flow_task_eventual
    // @tc.desc: Test task completion notification generation
    // @tc.precon: NA
    // @tc.step: 1. Create a NotifyFlow instance with test channel
    //           2. Generate random task ID and UID
    //           3. Create EventualNotify with test parameters and is_successful=true
    //           4. Call publish_completed_notify
    //           5. Verify returned notification content matches expected default
    // @tc.expect: Task completion notification content is correctly generated
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
    #[test]
    fn ut_notify_flow_task_eventual() {
        let (_, rx) = mpsc::unbounded_channel();
        let db = Arc::new(NotificationDb::new());
        let mut flow = NotifyFlow::new(rx, db);
        let task_id = fast_random() as u32;
        let uid = fast_random();
        let info = EventualNotify {
            action: Action::Download,
            task_id,
            processed: 0,
            uid,
            file_name: "test".to_string(),
            is_successful: true,
        };
        let content_default = NotifyContent::task_eventual_notify(
            None,
            info.action,
            info.task_id,
            info.uid as u32,
            info.file_name.clone(),
            info.is_successful,
        );
        let content = flow.publish_completed_notify(&info).unwrap();
        assert_eq!(content, content_default);
    }

    // @tc.name: ut_customized_task_eventual
    // @tc.desc: Test task completion notification with customized content
    // @tc.precon: NA
    // @tc.step: 1. Create a NotifyFlow instance with test channel
    //           2. Generate random task ID and UID
    //           3. Update task's customized notification with test title and text
    //           4. Create EventualNotify with is_successful=false and call publish_completed_notify
    //           5. Verify notification contains customized content and task info still exists
    //           6. Update EventualNotify to is_successful=true and call publish_completed_notify
    //           7. Verify notification contains customized content and task info is cleared
    // @tc.expect: Customized content is included in notifications and task info is cleared only when successful
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
    #[test]
    fn ut_customized_task_eventual() {
        let (_, rx) = mpsc::unbounded_channel();
        let db = Arc::new(NotificationDb::new());
        let mut flow = NotifyFlow::new(rx, db.clone());
        let task_id = fast_random() as u32;
        let uid = fast_random();
        let mut info = EventualNotify {
            action: Action::Download,
            task_id,
            processed: 0,
            uid,
            file_name: "test".to_string(),
            is_successful: false,
        };
        db.update_task_customized_notification(
            task_id,
            Some(TEST_TITLE.to_string()),
            Some(TEST_TEXT.to_string()),
        );

        let customized = db.query_task_customized_notification(task_id);
        let content_default = NotifyContent::task_eventual_notify(
            customized,
            info.action,
            info.task_id,
            info.uid as u32,
            info.file_name.clone(),
            info.is_successful,
        );
        let content = flow.publish_completed_notify(&info).unwrap();
        let customized = db.query_task_customized_notification(task_id);
        assert!(customized.is_some());
        assert_eq!(content, content_default);

        info.is_successful = true;
        let content = flow.publish_completed_notify(&info).unwrap();
        let content_default = NotifyContent::task_eventual_notify(
            customized,
            info.action,
            info.task_id,
            info.uid as u32,
            info.file_name.clone(),
            info.is_successful,
        );
        assert!(db.query_task_customized_notification(task_id).is_none());
        assert_eq!(content, content_default);
    }
}
