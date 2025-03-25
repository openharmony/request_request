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

use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};

use rdb::{OpenConfig, RdbStore, SecurityLevel};

use crate::service::notification_bar::NotificationDispatcher;

const DB_PATH: &str = if cfg!(test) {
    "/data/test/notification.db"
} else {
    "/data/service/el1/public/database/request/request.db"
};

const MILLIS_IN_A_WEEK: u64 = 7 * 24 * 60 * 60 * 1000;

pub(crate) static REQUEST_DB: LazyLock<RdbStore<'static>> = LazyLock::new(|| {
    let mut config = OpenConfig::new(DB_PATH);
    config.security_level(SecurityLevel::S1);
    if cfg!(test) {
        config.encrypt_status(false);
        config.bundle_name("Test");
    } else {
        config.encrypt_status(true);
    }
    RdbStore::open(config).unwrap()
});

pub(crate) fn clear_database() {
    // rdb not support RETURNING expr.
    let current_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration,
        Err(e) => {
            error!("Failed to get current time: {}", e);
            return;
        }
    }
    .as_millis() as u64;

    let task_ids = match REQUEST_DB.query::<u32>(
        "SELECT task_id from request_task WHERE mtime < ?",
        current_time - MILLIS_IN_A_WEEK,
    ) {
        Ok(rows) => rows,
        Err(e) => {
            error!("Failed to clear database: {}", e);
            return;
        }
    };

    for task_id in task_ids {
        info!(
            "clear {} info for have been overdue for more than a week.",
            task_id
        );
        if let Err(e) = REQUEST_DB.execute("DELETE from request_task WHERE task_id = ?", task_id) {
            error!("Failed to clear task {} info: {}", task_id, e);
        }
        NotificationDispatcher::get_instance().clear_task_info(task_id);
    }
    NotificationDispatcher::get_instance().clear_group_info();
}

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

    clear_database();
    let query: Vec<_> = REQUEST_DB
        .query::<u32>("SELECT task_id from request_task", ())
        .unwrap()
        .collect();
    for task_id in task_ids.iter().take(2) {
        assert!(!query.contains(task_id));
    }
    assert!(query.contains(&task_ids[2]));
}
