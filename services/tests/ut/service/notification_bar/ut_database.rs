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
use std::time::{SystemTime, UNIX_EPOCH};
const TEST_TITLE: &str = "田文镜";
const TEST_TEXT: &str = "我XXX";
const TEST_WANT_AGENT: &str = "wantAgent";

// @tc.name: ut_notify_database_query_tasks
// @tc.desc: Test querying tasks in a notification group
// @tc.precon: NA
// @tc.step: 1. Create a NotificationDb instance
//           2. Generate a random group ID and multiple task IDs
//           3. Associate tasks with the group using update_task_group
//           4. Call query_group_tasks to retrieve tasks
//           5. Compare retrieved tasks with expected list
// @tc.expect: Retrieved task IDs match the expected sorted list
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_notify_database_query_tasks() {
    let db = NotificationDb::new();
    let group_id = fast_random() as u32;
    let mut v = vec![];
    for _ in 0..100 {
        let task_id = fast_random() as u32;
        v.push(task_id);
        db.update_task_group(task_id, group_id);
    }
    v.sort();
    let mut ans = db.query_group_tasks(group_id);
    ans.sort();
    assert_eq!(v, ans);
}

// @tc.name: ut_notify_database_query_task_gid
// @tc.desc: Test querying task's group ID
// @tc.precon: NA
// @tc.step: 1. Create a NotificationDb instance
//           2. Generate a random group ID
//           3. For multiple tasks, update their group using update_task_group
//           4. Call query_task_gid for each task and verify the group ID
// @tc.expect: Each task's queried group ID matches the generated group ID
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_notify_database_query_task_gid() {
    let db = NotificationDb::new();
    let group_id = fast_random() as u32;

    for _ in 0..100 {
        let task_id = fast_random() as u32;
        db.update_task_group(task_id, group_id);
        assert_eq!(db.query_task_gid(task_id).unwrap(), group_id);
    }
}

// @tc.name: ut_notify_database_query_task_customized
// @tc.desc: Test querying task's customized notification content
// @tc.precon: NA
// @tc.step: 1. Create a NotificationDb instance
//           2. Generate a random task ID
//           3. Update task's customized notification with test title and text
//           4. Query the customized notification using
//              query_task_customized_notification
//           5. Verify the retrieved title and text match test values
// @tc.expect: Customized notification title and text match the test values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_notify_database_query_task_customized() {
    let db = NotificationDb::new();
    let task_id = fast_random() as u32;
    let config = NotificationConfig::new(
        task_id,
        Some(TEST_TITLE.to_string()),
        Some(TEST_TEXT.to_string()),
        Some(TEST_WANT_AGENT.to_string()),
        false,
        0b01,
    );

    db.update_task_customized_notification(&config);
    let customized = db.query_task_customized_notification(task_id).unwrap();
    assert_eq!(customized.title.unwrap(), TEST_TITLE);
    assert_eq!(customized.text.unwrap(), TEST_TEXT);
    assert_eq!(customized.want_agent.unwrap(), TEST_WANT_AGENT);
}

// @tc.name: ut_notify_database_query_group_customized
// @tc.desc: Test querying group's customized notification content
// @tc.precon: NA
// @tc.step: 1. Create a NotificationDb instance
//           2. Generate a random group ID
//           3. Update group's customized notification with test title and text
//           4. Query the customized notification using
//              query_group_customized_notification
//           5. Verify the retrieved title and text match test values
// @tc.expect: Customized notification title and text match the test values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_notify_database_query_group_customized() {
    let db = NotificationDb::new();
    let group_id = fast_random() as u32;

    db.update_group_customized_notification(
        group_id,
        Some(TEST_TITLE.to_string()),
        Some(TEST_TEXT.to_string()),
        Some(TEST_WANT_AGENT.to_string()),
    );
    let customized = db.query_group_customized_notification(group_id).unwrap();
    assert_eq!(customized.title.unwrap(), TEST_TITLE);
    assert_eq!(customized.text.unwrap(), TEST_TEXT);
    assert_eq!(customized.want_agent.unwrap(), TEST_WANT_AGENT);
}

// @tc.name: ut_notify_database_group_config
// @tc.desc: Test group notification configuration operations
// @tc.precon: NA
// @tc.step: 1. Create a NotificationDb instance
//           2. Generate a random group ID
//           3. Verify group does not exist initially
//           4. Update group config with gauge=true, display=false
//           5. Verify group exists, is_gauge returns true, attach_able returns
//              true
//           6. Update group config with gauge=false and disable attach_able
//           7. Verify is_gauge returns false and attach_able returns false
// @tc.expect: Group configuration updates and queries return correct values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_notify_database_group_config() {
    let db = NotificationDb::new();
    let group_id = fast_random() as u32;

    assert!(!db.contains_group(group_id));
    db.update_group_config(group_id, true, 0, false, 0b01);
    assert!(db.contains_group(group_id));
    assert!(db.is_gauge(group_id));
    assert!(db.attach_able(group_id));
    db.update_group_config(group_id, false, 0, false, 0b01);
    db.disable_attach_group(group_id);
    assert!(!db.attach_able(group_id));
    assert!(!db.is_gauge(group_id));
}

// @tc.name: ut_clear_task_info
// @tc.desc: Test clearing task notification information
// @tc.precon: NA
// @tc.step: 1. Create a NotificationDb instance
//           2. Generate random group and task IDs
//           3. Disable task notification and update customized content
//           4. Verify task notification is disabled and content exists
//           5. Call clear_task_info for the task
//           6. Verify task notification is enabled and content is removed
//           7. Associate task with group, clear task info, and verify group
//              association is removed
// @tc.expect: All task-related information is successfully cleared
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_clear_task_info() {
    let db = NotificationDb::new();

    let group_id = fast_random() as u32;
    let task_id = fast_random() as u32;
    let config = NotificationConfig::new(task_id, None, None, None, true, 0b01);

    db.disable_task_notification(task_id);
    db.update_task_customized_notification(&config);
    assert!(!db.check_task_notification_available(&task_id));
    assert!(db.query_task_customized_notification(task_id).is_some());
    db.clear_task_info(task_id);
    assert!(db.check_task_notification_available(&task_id));
    assert!(db.query_task_customized_notification(task_id).is_none());

    db.update_task_group(task_id, group_id);
    assert_eq!(db.query_task_gid(task_id).unwrap(), group_id);
    db.clear_task_info(task_id);
    assert!(db.query_task_gid(task_id).is_none());
}

// @tc.name: ut_clear_group_info
// @tc.desc: Test clearing group notification information
// @tc.precon: NA
// @tc.step: 1. Create a NotificationDb instance
//           2. Generate random group and task IDs
//           3. Update group customized notification, config, and associate task
//              with group
//           4. Verify group content, existence, and task association exist
//           5. Call clear_group_info for the group
//           6. Verify group content, existence, and task association are
//              removed
// @tc.expect: All group-related information is successfully cleared
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_clear_group_info() {
    let db = NotificationDb::new();

    let group_id = fast_random() as u32;
    let task_id = fast_random() as u32;
    db.update_group_customized_notification(group_id, None, None, None);
    db.update_group_config(group_id, true, 0, false, 0b01);
    db.update_task_group(task_id, group_id);

    assert!(db.query_group_customized_notification(group_id).is_some());
    assert!(db.contains_group(group_id));
    assert_eq!(db.query_task_gid(task_id).unwrap(), group_id);

    db.clear_group_info(group_id);
    assert!(db.query_group_customized_notification(group_id).is_none());
    assert!(!db.contains_group(group_id));
    assert!(db.query_task_gid(task_id).is_none());
}

// @tc.name: ut_clear_group_info_a_week_ago
// @tc.desc: Test clearing group info older than a week
// @tc.precon: NA
// @tc.step: 1. Create a NotificationDb instance
//           2. Generate current time and one week ago timestamp
//           3. Create group with current time and verify it's not cleared
//           4. Update group with one week ago timestamp and associate task
//           5. Verify group is not cleared when task exists
//           6. Clear task info and verify group is cleared
// @tc.expect: Only groups older than a week with no tasks are cleared
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_clear_group_info_a_week_ago() {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let a_week_ago = current_time - MILLIS_IN_A_WEEK;

    let db = NotificationDb::new();
    let group_id = fast_random() as u32;
    let task_id = fast_random() as u32;

    db.update_group_customized_notification(group_id, None, None, None);
    db.update_group_config(group_id, true, current_time, false, 0b01);

    db.clear_group_info_a_week_ago();
    assert!(db.query_group_customized_notification(group_id).is_some());
    assert!(db.contains_group(group_id));

    db.update_group_config(group_id, true, a_week_ago, false, 0b01);
    db.update_task_group(task_id, group_id);
    db.clear_group_info_a_week_ago();
    assert!(db.query_group_customized_notification(group_id).is_some());
    assert!(db.contains_group(group_id));

    db.clear_task_info(task_id);
    db.clear_group_info_a_week_ago();
    assert!(db.query_group_customized_notification(group_id).is_none());
    assert!(!db.contains_group(group_id));
}

// @tc.name: ut_notify_database_visibility
// @tc.desc: Test notification visibility functions
// @tc.precon: NA
// @tc.step: 1. Create a NotificationDb instance
//           2. Update task notification with different visibility values
//           3. Test is_completion_visible and is_progress_visible functions
//           4. Update group notification with different visibility values
//           5. Test is_completion_visible_from_group and is_progress_visible_from_group functions
// @tc.expect: Visibility functions return correct values based on bitmask
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_notify_database_visibility() {
    let db = NotificationDb::new();
    let task_id = 1;
    let group_id = 1;
    let _current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let config = NotificationConfig::new(
        task_id,
        Some("title".to_string()),
        Some("text".to_string()),
        None,
        false,
        0b00,
    );
    db.update_task_customized_notification(&config);
    assert!(!db.is_completion_visible(task_id));
    assert!(!db.is_progress_visible(task_id));

    let config = NotificationConfig::new(
        task_id,
        Some("title".to_string()),
        Some("text".to_string()),
        None,
        false,
        0b01,
    );
    db.update_task_customized_notification(&config);
    assert!(db.is_completion_visible(task_id));
    assert!(!db.is_progress_visible(task_id));

    let config = NotificationConfig::new(
        task_id,
        Some("title".to_string()),
        Some("text".to_string()),
        None,
        false,
        0b10,
    );
    db.update_task_customized_notification(&config);
    assert!(!db.is_completion_visible(task_id));
    assert!(db.is_progress_visible(task_id));

    let config = NotificationConfig::new(
        task_id,
        Some("title".to_string()),
        Some("text".to_string()),
        None,
        false,
        0b11,
    );
    db.update_task_customized_notification(&config);
    assert!(db.is_completion_visible(task_id));
    assert!(db.is_progress_visible(task_id));

    db.update_group_config(group_id, true, _current_time, true, 0b00);
    assert!(!db.is_completion_visible_from_group(group_id));
    assert!(!db.is_progress_visible_from_group(group_id));

    db.update_group_config(group_id, true, _current_time, true, 0b01);
    assert!(db.is_completion_visible_from_group(group_id));
    assert!(!db.is_progress_visible_from_group(group_id));

    db.update_group_config(group_id, true, _current_time, true, 0b10);
    assert!(!db.is_completion_visible_from_group(group_id));
    assert!(db.is_progress_visible_from_group(group_id));

    db.update_group_config(group_id, true, _current_time, true, 0b11);
    assert!(db.is_completion_visible_from_group(group_id));
    assert!(db.is_progress_visible_from_group(group_id));
}

// @tc.name: ut_notify_database_want_agent
// @tc.desc: Test want_agent field operations for task and group notifications
// @tc.precon: NA
// @tc.step: 1. Create a NotificationDb instance
//           2. Test setting and retrieving valid want_agent for task
//           3. Test updating existing want_agent for task
//           4. Test clearing want_agent for task
//           5. Test setting and retrieving valid want_agent for group
//           6. Test updating existing want_agent for group
//           7. Test clearing want_agent for group
// @tc.expect: want_agent field can be correctly set, updated, retrieved and cleared
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_notify_database_want_agent() {
    let db = NotificationDb::new();
    let task_id = fast_random() as u32;
    let group_id = fast_random() as u32;
    let updated_want_agent = "updatedWantAgent";

    // Test task want_agent operations
    // 1. Set and retrieve valid want_agent
    let config = NotificationConfig::new(
        task_id,
        None,
        None,
        Some(TEST_WANT_AGENT.to_string()),
        false,
        0b00,
    );
    db.update_task_customized_notification(&config);
    let customized = db.query_task_customized_notification(task_id).unwrap();
    assert_eq!(customized.want_agent.unwrap(), TEST_WANT_AGENT);

    // 2. Update existing want_agent
    let config = NotificationConfig::new(
        task_id,
        None,
        None,
        Some(updated_want_agent.to_string()),
        false,
        0b00,
    );
    db.update_task_customized_notification(&config);
    let customized = db.query_task_customized_notification(task_id).unwrap();
    assert_eq!(customized.want_agent.unwrap(), updated_want_agent);

    // 3. Clear want_agent
    let config = NotificationConfig::new(
        task_id,
        None,
        None,
        None,
        false,
        0b00,
    );
    db.update_task_customized_notification(&config);
    let customized = db.query_task_customized_notification(task_id).unwrap();
    assert!(customized.want_agent.is_none());

    // Test group want_agent operations
    // 4. Set and retrieve valid want_agent
    db.update_group_customized_notification(
        group_id,
        None,
        None,
        Some(TEST_WANT_AGENT.to_string()),
    );
    let customized = db.query_group_customized_notification(group_id).unwrap();
    assert_eq!(customized.want_agent.unwrap(), TEST_WANT_AGENT);

    // 5. Update existing want_agent
    db.update_group_customized_notification(
        group_id,
        None,
        None,
        Some(updated_want_agent.to_string()),
    );
    let customized = db.query_group_customized_notification(group_id).unwrap();
    assert_eq!(customized.want_agent.unwrap(), updated_want_agent);

    // 6. Clear want_agent
    db.update_group_customized_notification(
        group_id,
        None,
        None,
        None,
    );
    let customized = db.query_group_customized_notification(group_id).unwrap();
    assert!(customized.want_agent.is_none());

    // 7. Test want_agent persistence after clearing other fields
    db.update_group_customized_notification(
        group_id,
        Some("title".to_string()),
        Some("text".to_string()),
        Some(TEST_WANT_AGENT.to_string()),
    );
    // Update only title and text, want_agent should remain
    db.update_group_customized_notification(
        group_id,
        Some("new_title".to_string()),
        Some("new_text".to_string()),
        None,
    );
    let customized = db.query_group_customized_notification(group_id).unwrap();
    assert_eq!(customized.title.unwrap(), "new_title");
    assert_eq!(customized.text.unwrap(), "new_text");
    assert!(customized.want_agent.is_none());
}
