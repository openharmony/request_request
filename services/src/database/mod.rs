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

//! Database module for request service.
//!
//! This module provides database operations and monitoring functionality.

mod db_monitor;

pub(crate) use db_monitor::monitor_database;

use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};

use rdb::{OpenConfig, RdbStore, SecurityLevel};

use crate::service::notification_bar::NotificationDispatcher;
use crate::task::info::State;

const DB_PATH: &str = if cfg!(test) {
    "/data/test/notification.db"
} else {
    "/data/service/el1/public/database/request/request.db"
};

const MILLIS_IN_A_WEEK: u64 = 7 * 24 * 60 * 60 * 1000;
const MILLIS_IN_ONE_DAY: u64 = 24 * 60 * 60 * 1000;

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

pub(crate) fn clear_database_by_state(pre_count: usize) -> Result<bool, ()> {    
        // rdb not support RETURNING expr.
        let current_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration,
        Err(e) => {
            error!("Failed to get current time: {}", e);
            return Err(());
        }
    }
    .as_millis() as u64;

    let mut any_remain = false;

    // Clear Removed tasks immediately (no retention threshold)
    let removed_ids = match REQUEST_DB.query::<u32>(
        "SELECT task_id FROM request_task WHERE state = ? LIMIT ?",
        (State::Removed.repr as u64, pre_count as u64),
    ) {
        Ok(rows) => rows.collect::<Vec<_>>(),
        Err(e) => {
            error!("Failed to query removed tasks: {}", e);
            Vec::new()
        }
    };

    if removed_ids.len() >= pre_count {
        any_remain = true;
    }

    for task_id in removed_ids {
        debug!("clear removed task {} info for have been in removed state.", task_id);
        if let Err(e) = REQUEST_DB.execute("DELETE from request_task WHERE task_id = ?", task_id) {
            error!("Failed to clear removed task {} info: {}", task_id, e);
        }
        NotificationDispatcher::get_instance().clear_task_info(task_id);
    }

    // Clear Completed tasks older than 1 day
    let completed_ids = match REQUEST_DB.query::<u32>(
        "SELECT task_id FROM request_task WHERE state = ? AND mtime < ? LIMIT ?",
        (State::Completed.repr as u64, current_time - MILLIS_IN_ONE_DAY, pre_count as u64),
    ) {
        Ok(rows) => rows.collect::<Vec<_>>(),
        Err(e) => {
            error!("Failed to query completed tasks: {}", e);
            Vec::new()
        }
    };

    if completed_ids.len() >= pre_count {
        any_remain = true;
    }

    for task_id in completed_ids {
        debug!("clear completed task {} info for have been overdue for more than a day.", task_id);
        if let Err(e) = REQUEST_DB.execute("DELETE from request_task WHERE task_id = ?", task_id) {
            error!("Failed to clear completed task {} info: {}", task_id, e);
        }
        NotificationDispatcher::get_instance().clear_task_info(task_id);
    }

    // Clear other states (except Completed and Removed) older than 7 days
    let other_ids = match REQUEST_DB.query::<u32>(
        "SELECT task_id FROM request_task WHERE state != ? AND state != ? AND mtime < ? LIMIT ?",
        (State::Completed.repr as u64, State::Removed.repr as u64, current_time - MILLIS_IN_A_WEEK, pre_count as u64),
    ) {
        Ok(rows) => rows.collect::<Vec<_>>(),
        Err(e) => {
            error!("Failed to query other tasks: {}", e);
            Vec::new()
        }
    };

    if other_ids.len() >= pre_count {
        any_remain = true;
    }

    for task_id in other_ids {
        debug!("clear other task {} info for have been overdue for more than a week.", task_id);
        if let Err(e) = REQUEST_DB.execute("DELETE from request_task WHERE task_id = ?", task_id) {
            error!("Failed to clear other task {} info: {}", task_id, e);
        }
        NotificationDispatcher::get_instance().clear_task_info(task_id);
    }

    Ok(any_remain)
}

#[cfg(test)]
mod ut_database {
    include!("../../tests/ut/ut_database.rs");
}
