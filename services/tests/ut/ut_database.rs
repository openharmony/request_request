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

use super::*;
use crate::tests::lock_database;

// @tc.name: ut_clear_database_test
// @tc.desc: Test the functionality of clearing expired tasks from database
// @tc.precon: NA
// @tc.step: 1. Create test table and insert sample tasks with different timestamps and states
//           2. Call clear_database_by_state function with threshold
//           3. Verify old tasks are removed based on state-specific retention policy
// @tc.expect: Tasks are removed according to state-based retention rules
// @tc.type: FUNC
// @tc.require: issues#ICN31I
#[test]
fn clear_database_test() {
    let _lock = lock_database();
    use request_utils::fastrand::fast_random;

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let a_week_ago = current_time - MILLIS_IN_A_WEEK;

    REQUEST_DB
        .execute(
            "CREATE TABLE IF NOT EXISTS request_task (task_id INTEGER PRIMARY KEY, mtime INTEGER, state INTEGER)",
            (),
        )
        .unwrap();
    let mut task_ids = [
        fast_random() as u32,
        fast_random() as u32,
        fast_random() as u32,
    ];

    task_ids.sort();
    let sql = "INSERT INTO request_task (task_id, mtime, state) VALUES (?, ?, ?)";
    // Insert old Removed tasks (should be cleaned after 1 day)
    for task_id in task_ids.iter().take(2) {
        REQUEST_DB
            .execute(sql, (*task_id, a_week_ago, State::Removed.repr as u64))
            .unwrap();
    }
    // Insert recent task (should remain)
    REQUEST_DB
        .execute(
            sql,
            (task_ids[2], a_week_ago + 20000, State::Running.repr as u64),
        )
        .unwrap();
    let query: Vec<_> = REQUEST_DB
        .query::<u32>("SELECT task_id from request_task", ())
        .unwrap()
        .collect();
    for task_id in task_ids.iter() {
        assert!(query.contains(task_id));
    }

    if let Ok(remain) = clear_database_by_state(query.len() + 1) {
        assert!(!remain);
    }

    let query: Vec<_> = REQUEST_DB
        .query::<u32>("SELECT task_id from request_task", ())
        .unwrap()
        .collect();
    for task_id in task_ids.iter().take(2) {
        assert!(!query.contains(task_id));
    }
    assert!(query.contains(&task_ids[2]));
}

// @tc.name: ut_clear_database_by_state_removed_tasks
// @tc.desc: Test clearing Removed tasks older than 1 day
// @tc.precon: NA
// @tc.step: 1. Create test table with Removed tasks of different ages
//           2. Call clear_database_by_state
//           3. Verify only old Removed tasks are removed
// @tc.expect: Removed tasks older than 1 day are removed, newer ones remain
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
#[test]
fn ut_clear_database_by_state_removed_tasks() {
    let _lock = lock_database();
    use request_utils::fastrand::fast_random;

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let two_days_ago = current_time - MILLIS_IN_TWO_DAYS;
    let half_day_ago = current_time - MILLIS_IN_ONE_DAY / 2;

    REQUEST_DB
        .execute(
            "CREATE TABLE IF NOT EXISTS request_task (task_id INTEGER PRIMARY KEY, mtime INTEGER, state INTEGER)",
            (),
        )
        .unwrap();

    let old_task_id = fast_random() as u32;
    let recent_task_id = fast_random() as u32;

    let sql = "INSERT INTO request_task (task_id, mtime, state) VALUES (?, ?, ?)";
    // Old Removed task (older than 1 day)
    REQUEST_DB
        .execute(sql, (old_task_id, two_days_ago, State::Removed.repr as u64))
        .unwrap();
    // Recent Removed task (less than 1 day)
    REQUEST_DB
        .execute(
            sql,
            (recent_task_id, half_day_ago, State::Removed.repr as u64),
        )
        .unwrap();

    let result = clear_database_by_state(1000).unwrap();
    assert!(!result);

    let query: Vec<_> = REQUEST_DB
        .query::<u32>("SELECT task_id from request_task", ())
        .unwrap()
        .collect();

    assert!(!query.contains(&old_task_id));
    assert!(query.contains(&recent_task_id));
}

// @tc.name: ut_clear_database_by_state_completed_tasks
// @tc.desc: Test clearing Completed tasks older than 2 days
// @tc.precon: NA
// @tc.step: 1. Create test table with Completed tasks of different ages
//           2. Call clear_database_by_state
//           3. Verify only old Completed tasks are removed
// @tc.expect: Completed tasks older than 2 days are removed, newer ones remain
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
#[test]
fn ut_clear_database_by_state_completed_tasks() {
    let _lock = lock_database();
    use request_utils::fastrand::fast_random;

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let three_days_ago = current_time - MILLIS_IN_TWO_DAYS - MILLIS_IN_ONE_DAY;
    let one_day_ago = current_time - MILLIS_IN_ONE_DAY;

    REQUEST_DB
        .execute(
            "CREATE TABLE IF NOT EXISTS request_task (task_id INTEGER PRIMARY KEY, mtime INTEGER, state INTEGER)",
            (),
        )
        .unwrap();

    let old_task_id = fast_random() as u32;
    let recent_task_id = fast_random() as u32;

    let sql = "INSERT INTO request_task (task_id, mtime, state) VALUES (?, ?, ?)";
    // Old Completed task (older than 2 days)
    REQUEST_DB
        .execute(
            sql,
            (old_task_id, three_days_ago, State::Completed.repr as u64),
        )
        .unwrap();
    // Recent Completed task (less than 2 days)
    REQUEST_DB
        .execute(
            sql,
            (recent_task_id, one_day_ago, State::Completed.repr as u64),
        )
        .unwrap();

    let result = clear_database_by_state(1000).unwrap();
    assert!(!result);

    let query: Vec<_> = REQUEST_DB
        .query::<u32>("SELECT task_id from request_task", ())
        .unwrap()
        .collect();

    assert!(!query.contains(&old_task_id));
    assert!(query.contains(&recent_task_id));
}

// @tc.name: ut_clear_database_by_state_failed_stopped_tasks
// @tc.desc: Test clearing Failed/Stopped tasks older than 7 days
// @tc.precon: NA
// @tc.step: 1. Create test table with Failed and Stopped tasks of different ages
//           2. Call clear_database_by_state
//           3. Verify only old Failed/Stopped tasks are removed
// @tc.expect: Failed/Stopped tasks older than 7 days are removed, newer ones remain
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
#[test]
fn ut_clear_database_by_state_failed_stopped_tasks() {
    let _lock = lock_database();
    use request_utils::fastrand::fast_random;

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let two_weeks_ago = current_time - MILLIS_IN_A_WEEK * 2;
    let three_days_ago = current_time - MILLIS_IN_TWO_DAYS - MILLIS_IN_ONE_DAY;

    REQUEST_DB
        .execute(
            "CREATE TABLE IF NOT EXISTS request_task (task_id INTEGER PRIMARY KEY, mtime INTEGER, state INTEGER)",
            (),
        )
        .unwrap();

    let old_failed_id = fast_random() as u32;
    let old_stopped_id = fast_random() as u32;
    let recent_failed_id = fast_random() as u32;
    let recent_stopped_id = fast_random() as u32;

    let sql = "INSERT INTO request_task (task_id, mtime, state) VALUES (?, ?, ?)";
    // Old Failed task (older than 7 days)
    REQUEST_DB
        .execute(
            sql,
            (old_failed_id, two_weeks_ago, State::Failed.repr as u64),
        )
        .unwrap();
    // Old Stopped task (older than 7 days)
    REQUEST_DB
        .execute(
            sql,
            (old_stopped_id, two_weeks_ago, State::Stopped.repr as u64),
        )
        .unwrap();
    // Recent Failed task (less than 7 days)
    REQUEST_DB
        .execute(
            sql,
            (recent_failed_id, three_days_ago, State::Failed.repr as u64),
        )
        .unwrap();
    // Recent Stopped task (less than 7 days)
    REQUEST_DB
        .execute(
            sql,
            (
                recent_stopped_id,
                three_days_ago,
                State::Stopped.repr as u64,
            ),
        )
        .unwrap();

    let result = clear_database_by_state(1000).unwrap();
    assert!(!result);

    let query: Vec<_> = REQUEST_DB
        .query::<u32>("SELECT task_id from request_task", ())
        .unwrap()
        .collect();

    assert!(!query.contains(&old_failed_id));
    assert!(!query.contains(&old_stopped_id));
    assert!(query.contains(&recent_failed_id));
    assert!(query.contains(&recent_stopped_id));
}

// @tc.name: ut_clear_database_by_state_mixed_states
// @tc.desc: Test clearing tasks with mixed states and ages
// @tc.precon: NA
// @tc.step: 1. Create test table with tasks of various states and ages
//           2. Call clear_database_by_state
//           3. Verify each task is handled according to its state retention policy
// @tc.expect: Tasks are removed based on state-specific retention rules
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
#[test]
fn ut_clear_database_by_state_mixed_states() {
    let _lock = lock_database();
    use request_utils::fastrand::fast_random;

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let two_weeks_ago = current_time - MILLIS_IN_A_WEEK * 2;
    let three_days_ago = current_time - MILLIS_IN_TWO_DAYS - MILLIS_IN_ONE_DAY;
    let half_day_ago = current_time - MILLIS_IN_ONE_DAY / 2;

    REQUEST_DB
        .execute(
            "CREATE TABLE IF NOT EXISTS request_task (task_id INTEGER PRIMARY KEY, mtime INTEGER, state INTEGER)",
            (),
        )
        .unwrap();

    let old_removed_id = fast_random() as u32;
    let old_completed_id = fast_random() as u32;
    let old_failed_id = fast_random() as u32;
    let old_running_id = fast_random() as u32;
    let recent_removed_id = fast_random() as u32;
    let recent_running_id = fast_random() as u32;

    let sql = "INSERT INTO request_task (task_id, mtime, state) VALUES (?, ?, ?)";
    // Old Removed (should be cleaned - > 1 day)
    REQUEST_DB
        .execute(
            sql,
            (old_removed_id, two_weeks_ago, State::Removed.repr as u64),
        )
        .unwrap();
    // Old Completed (should be cleaned - > 2 days)
    REQUEST_DB
        .execute(
            sql,
            (
                old_completed_id,
                three_days_ago,
                State::Completed.repr as u64,
            ),
        )
        .unwrap();
    // Old Failed (should be cleaned - > 7 days)
    REQUEST_DB
        .execute(
            sql,
            (old_failed_id, two_weeks_ago, State::Failed.repr as u64),
        )
        .unwrap();
    // Old Running (should be cleaned - > 7 days, other states)
    REQUEST_DB
        .execute(
            sql,
            (old_running_id, two_weeks_ago, State::Running.repr as u64),
        )
        .unwrap();
    // Recent Removed (should remain - < 1 day)
    REQUEST_DB
        .execute(
            sql,
            (recent_removed_id, half_day_ago, State::Removed.repr as u64),
        )
        .unwrap();
    // Recent Running (should remain - < 7 days)
    REQUEST_DB
        .execute(
            sql,
            (
                recent_running_id,
                three_days_ago,
                State::Running.repr as u64,
            ),
        )
        .unwrap();

    let result = clear_database_by_state(1000).unwrap();
    assert!(!result);

    let query: Vec<_> = REQUEST_DB
        .query::<u32>("SELECT task_id from request_task", ())
        .unwrap()
        .collect();

    assert!(!query.contains(&old_removed_id));
    assert!(!query.contains(&old_completed_id));
    assert!(!query.contains(&old_failed_id));
    assert!(!query.contains(&old_running_id));
    assert!(query.contains(&recent_removed_id));
    assert!(query.contains(&recent_running_id));
}

// @tc.name: ut_clear_database_by_state_remain_flag
// @tc.desc: Test that remain flag is correctly set when there are more tasks to clean
// @tc.precon: NA
// @tc.step: 1. Create multiple old tasks exceeding pre_count limit
//           2. Call clear_database_by_state with small pre_count
//           3. Verify remain flag is true
// @tc.expect: Returns true when there are more tasks to clean
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
#[test]
fn ut_clear_database_by_state_remain_flag() {
    let _lock = lock_database();
    use request_utils::fastrand::fast_random;

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let two_days_ago = current_time - MILLIS_IN_TWO_DAYS;
    REQUEST_DB.execute("DELETE FROM request_task", ()).unwrap();
    REQUEST_DB
        .execute(
            "CREATE TABLE IF NOT EXISTS request_task (task_id INTEGER PRIMARY KEY, mtime INTEGER, state INTEGER)",
            (),
        )
        .unwrap();

    let mut task_ids = Vec::new();
    let sql = "INSERT INTO request_task (task_id, mtime, state) VALUES (?, ?, ?)";
    for _ in 0..5 {
        let task_id = fast_random() as u32;
        task_ids.push(task_id);
        REQUEST_DB
            .execute(sql, (task_id, two_days_ago, State::Completed.repr as u64))
            .unwrap();
    }

    // Call with pre_count of 2, should return true since there are more tasks
    let result = clear_database_by_state(2).unwrap();
    assert!(result);

    // Verify only 2 tasks were removed
    let query: Vec<_> = REQUEST_DB
        .query::<u32>("SELECT task_id from request_task", ())
        .unwrap()
        .collect();
    assert_eq!(query.len(), 3);
}
