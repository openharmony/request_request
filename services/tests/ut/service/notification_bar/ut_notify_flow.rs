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

use ylong_runtime::fastrand::fast_random;

use super::*;
use crate::service::notification_bar::NotificationConfig;
use crate::task::config::Version;
use std::time::{SystemTime, UNIX_EPOCH};

const TEST_TITLE: &str = "test_title";
const TEST_TEXT: &str = "test_text";
const TEST_WANT_AGENT: &str = "test_want_agent";

// @tc.name: ut_notify_flow_group
// @tc.desc: Test group progress calculation and state updates
// @tc.precon: NA
// @tc.step: 1. Create a GroupProgress instance
//           2. Update 100 tasks with Running state
//           3. Update each task's progress to 100 and set alternating states to
//              Completed/Failed
//           4. Verify processed progress, successful and failed counts
//           5. Update all tasks' progress to 150 and set all states to
//              Completed
//           6. Verify final progress and state counts
// @tc.expect: Group progress and state counts update correctly through all
// operations @tc.type: FUNC
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
    let mut flow = NotifyFlow::new(rx, db.clone());
    let task_id = fast_random() as u32;
    let uid = fast_random();

    let config = NotificationConfig::new(task_id, None, None, None, false, 0b10);
    db.update_task_customized_notification(&config);

    let progress = ProgressNotify {
        action: Action::Download,
        task_id,
        uid,
        processed: 0,
        total: Some(100),
        multi_upload: None,
        file_name: "test".to_string(),
        version: Version::API10,
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
//           3. Create EventualNotify with test parameters and
//              is_successful=true
//           4. Call publish_completed_notify
//           5. Verify returned notification content matches expected default
// @tc.expect: Task completion notification content is correctly generated
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_notify_flow_task_eventual() {
    let (_, rx) = mpsc::unbounded_channel();
    let db = Arc::new(NotificationDb::new());
    let mut flow = NotifyFlow::new(rx, db.clone());
    let task_id = fast_random() as u32;
    let uid = fast_random();

    let config = NotificationConfig::new(task_id, None, None, None, false, 0b01);
    db.update_task_customized_notification(&config);

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
//           4. Create EventualNotify with is_successful=false and call
//              publish_completed_notify
//           5. Verify notification contains customized content and task info
//              still exists
//           6. Update EventualNotify to is_successful=true and call
//              publish_completed_notify
//           7. Verify notification contains customized content and task info is
//              cleared
// @tc.expect: Customized content is included in notifications and task info is
// cleared only when successful @tc.type: FUNC
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
    let config = NotificationConfig::new(
        task_id,
        Some(TEST_TITLE.to_string()),
        Some(TEST_TEXT.to_string()),
        Some(TEST_WANT_AGENT.to_string()),
        false,
        0b01,
    );
    db.update_task_customized_notification(&config);

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

// @tc.name: ut_group_visibility_progress
// @tc.desc: Test group progress notification with different visibility values
// @tc.precon: NA
// @tc.step: 1. Create a NotifyFlow instance with test channel
//           2. Create a group and set progress visibility
//           3. Attach a task to the group
//           4. Call publish_progress_notification and verify results
// @tc.expect: Group progress notification is generated when visibility is true
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_group_visibility_progress() {
    let (_, rx) = mpsc::unbounded_channel();
    let db = Arc::new(NotificationDb::new());
    let mut flow = NotifyFlow::new(rx, db.clone());
    let group_id = fast_random() as u32;
    let task_id = fast_random() as u32;
    let uid = fast_random();
    let _current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    // Setup group with progress visibility true
    db.update_group_config(group_id, true, _current_time, true, 0b10); // PROGRESS bit set
    db.update_task_group(task_id, group_id);

    // Test with progress visibility true
    let progress = ProgressNotify {
        action: Action::Download,
        task_id,
        uid,
        processed: 50,
        total: Some(100),
        multi_upload: None,
        file_name: "test".to_string(),
        version: Version::API10,
    };
    let content = flow.publish_progress_notification(progress.clone());
    assert!(content.is_some());

    // Update group progress visibility to false
    db.update_group_config(group_id, true, _current_time, true, 0b00); // No bits set
    // Verify visibility is updated in database
    assert!(!db.is_progress_visible_from_group(group_id));

    // Clear visibility cache to force recheck
    flow.group_progress_visibility.clear();

    // Test with progress visibility false
    let content = flow.publish_progress_notification(progress);
    assert!(content.is_none());
}

// @tc.name: ut_group_visibility_completion
// @tc.desc: Test group completion notification with different visibility values
// @tc.precon: NA
// @tc.step: 1. Create a NotifyFlow instance with test channel
//           2. Create a group and set completion visibility
//           3. Attach a task to the group
//           4. Call publish_completed_notify and verify results
// @tc.expect: Group completion notification is generated when visibility is true
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_group_visibility_completion() {
    let (_, rx) = mpsc::unbounded_channel();
    let db = Arc::new(NotificationDb::new());
    let mut flow = NotifyFlow::new(rx, db.clone());
    let group_id = fast_random() as u32;
    let task_id = fast_random() as u32;
    let uid = fast_random();
    let _current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    // Setup group with completion visibility true and not attachable
    db.update_group_config(group_id, true, _current_time, true, 0b01); // COMPLETION bit set
    db.disable_attach_group(group_id);
    db.update_task_group(task_id, group_id);

    // Test with completion visibility true
    // Simulate task completion (assuming this is handled elsewhere in the actual code)
    // Since there's no update_task_state method, we'll proceed with the existing setup

    // Trigger group progress update in NotifyFlow
    let info = EventualNotify {
        action: Action::Download,
        task_id,
        processed: 100,
        uid,
        file_name: "test".to_string(),
        is_successful: true,
    };
    // First call to update group_progress cache
    flow.publish_completed_notify(&info);
    // Second call to get the actual content
    let content = flow.publish_completed_notify(&info);
    assert!(content.is_some());

    // Update group completion visibility to false
    db.update_group_config(group_id, true, _current_time, true, 0b00); // No bits set

    let content = flow.publish_completed_notify(&info);
    assert!(content.is_none());
}
