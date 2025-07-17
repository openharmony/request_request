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

// @tc.name: ut_clear_database_test
// @tc.desc: Test the functionality of clearing expired tasks from database
// @tc.precon: NA
// @tc.step: 1. Create test table and insert sample tasks with different timestamps
//           2. Call clear_database_part function with threshold
//           3. Verify old tasks are removed and recent task remains
// @tc.expect: Only tasks newer than one week remain in database
// @tc.type: FUNC
// @tc.require: issues#ICN31I
#[test]
fn clear_database_test() {
    use request_utils::fastrand::fast_random;

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let a_week_ago = current_time - MILLIS_IN_A_WEEK;

    REQUEST_DB
        .execute(
            "CREATE TABLE IF NOT EXISTS request_task (task_id INTEGER PRIMARY KEY, mtime INTEGER)",
            (),
        )
        .unwrap();
    let mut task_ids = [
        fast_random() as u32,
        fast_random() as u32,
        fast_random() as u32,
    ];

    task_ids.sort();
    let sql = "INSERT INTO request_task (task_id, mtime) VALUES (?, ?)";
    for task_id in task_ids.iter().take(2) {
        REQUEST_DB.execute(sql, (*task_id, a_week_ago)).unwrap();
    }
    REQUEST_DB
        .execute(sql, (task_ids[2], a_week_ago + 20000))
        .unwrap();
    let query: Vec<_> = REQUEST_DB
        .query::<u32>("SELECT task_id from request_task", ())
        .unwrap()
        .collect();
    for task_id in task_ids.iter() {
        assert!(query.contains(task_id));
    }

    if let Ok(remain) = clear_database_part(query.len() + 1) {
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