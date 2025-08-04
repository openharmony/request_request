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

pub(crate) fn clear_database_part(pre_count: usize) -> Result<bool, ()> {
    let mut remain = true;
    // rdb not support RETURNING expr.
    let current_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration,
        Err(e) => {
            error!("Failed to get current time: {}", e);
            return Err(());
        }
    }
    .as_millis() as u64;

    let task_ids: Vec<_> = match REQUEST_DB.query::<u32>(
        "SELECT task_id from request_task WHERE mtime < ? LIMIT ?",
        (current_time - MILLIS_IN_A_WEEK, pre_count as u64),
    ) {
        Ok(rows) => rows.collect(),
        Err(e) => {
            error!("Failed to clear database: {}", e);
            return Err(());
        }
    };

    if task_ids.len() < pre_count {
        remain = false;
    }

    for task_id in task_ids {
        debug!(
            "clear {} info for have been overdue for more than a week.",
            task_id
        );
        if let Err(e) = REQUEST_DB.execute("DELETE from request_task WHERE task_id = ?", task_id) {
            error!("Failed to clear task {} info: {}", task_id, e);
        }
        NotificationDispatcher::get_instance().clear_task_info(task_id);
    }
    Ok(remain)
}

#[cfg(test)]
mod ut_database {
    include!("../tests/ut/ut_database.rs");
}
