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
#[cfg(feature = "multi-instance")]
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use rdb::{OpenConfig, RdbStore, SecurityLevel};

use crate::service::notification_bar::NotificationDispatcher;
use crate::task::info::State;

// Multi-instance: per-user DB path. Each SA serves one OS user, so the DB is
// named `request_<userId>.db`; single-instance keeps the legacy `request.db`.

/// Base directory for request DB files (shared across users).
const DB_DIR: &str = "/data/service/el1/public/database/request";

/// Current user id (multi-instance), default `0`.
#[cfg(feature = "multi-instance")]
static CURRENT_USER_ID: AtomicI32 = AtomicI32::new(0);

/// Sets the process-wide user id (called from `on_start_with_reason`).
///
/// NOTE: effective only before `RequestDb::get_instance()` locks the DB path.
#[cfg(feature = "multi-instance")]
pub(crate) fn set_current_user_id(user_id: i32) {
    CURRENT_USER_ID.store(user_id, Ordering::SeqCst);
}

/// Returns the current user id (default `0`).
#[cfg(feature = "multi-instance")]
pub(crate) fn current_user_id() -> i32 {
    CURRENT_USER_ID.load(Ordering::SeqCst)
}

/// Builds the per-process DB file path:
/// - multi-instance: `{DB_DIR}/request_<userId>.db`
/// - single-instance: `{DB_DIR}/request.db` (legacy)
/// - test: `/data/test/notification.db` (test-env preset file)
pub(crate) fn db_path() -> String {
    if cfg!(test) {
        return "/data/test/notification.db".to_string();
    }
    #[cfg(feature = "multi-instance")]
    {
        format!("{}/request_{}.db", DB_DIR, current_user_id())
    }
    #[cfg(not(feature = "multi-instance"))]
    {
        format!("{}/request.db", DB_DIR)
    }
}

/// Removes the legacy single-instance `request.db` (+ `-wal`/`-shm`/`-compare`)
/// left after OTA, so the per-user DBs start clean. Best-effort: failures are
/// logged, never block startup. Runs only when the legacy file still exists.
#[cfg(feature = "multi-instance")]
pub(crate) fn cleanup_legacy_db() {
    let legacy = format!("{}/request.db", DB_DIR);
    if !std::path::Path::new(&legacy).exists() {
        return;
    }
    info!("multi-instance: legacy request.db found, cleaning up");
    for suffix in ["", "-wal", "-shm", "-compare"] {
        let p = format!("{}{}", legacy, suffix);
        match std::fs::remove_file(&p) {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => info!("multi-instance: remove legacy db file {} failed: {}", p, e),
        }
    }
}


const MILLIS_IN_A_WEEK: u64 = 7 * 24 * 60 * 60 * 1000;
const MILLIS_IN_ONE_DAY: u64 = 24 * 60 * 60 * 1000;

pub(crate) static REQUEST_DB: LazyLock<RdbStore<'static>> = LazyLock::new(|| {
    let path = db_path();
    info!("REQUEST_DB open path: {}", path);
    let mut config = OpenConfig::new(&path);
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
        debug!(
            "clear removed task {} info for have been in removed state.",
            task_id
        );
        if let Err(e) = REQUEST_DB.execute("DELETE from request_task WHERE task_id = ?", task_id) {
            error!("Failed to clear removed task {} info: {}", task_id, e);
        }
        NotificationDispatcher::get_instance().clear_task_info(task_id);
    }

    // Clear Completed tasks older than 1 day
    let completed_ids = match REQUEST_DB.query::<u32>(
        "SELECT task_id FROM request_task WHERE state = ? AND mtime < ? LIMIT ?",
        (
            State::Completed.repr as u64,
            current_time - MILLIS_IN_ONE_DAY,
            pre_count as u64,
        ),
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
        debug!(
            "clear completed task {} info for have been overdue for more than a day.",
            task_id
        );
        if let Err(e) = REQUEST_DB.execute("DELETE from request_task WHERE task_id = ?", task_id) {
            error!("Failed to clear completed task {} info: {}", task_id, e);
        }
        NotificationDispatcher::get_instance().clear_task_info(task_id);
    }

    // Clear other states (except Completed and Removed) older than 7 days
    let other_ids = match REQUEST_DB.query::<u32>(
        "SELECT task_id FROM request_task WHERE state != ? AND state != ? AND mtime < ? LIMIT ?",
        (
            State::Completed.repr as u64,
            State::Removed.repr as u64,
            current_time - MILLIS_IN_A_WEEK,
            pre_count as u64,
        ),
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
        debug!(
            "clear other task {} info for have been overdue for more than a week.",
            task_id
        );
        if let Err(e) = REQUEST_DB.execute("DELETE from request_task WHERE task_id = ?", task_id) {
            error!("Failed to clear other task {} info: {}", task_id, e);
        }
        NotificationDispatcher::get_instance().clear_task_info(task_id);
    }

    Ok(any_remain)
}

/// Checkpoints the WAL back into the main database file.
///
/// Runs `PRAGMA wal_checkpoint(RESTART)`: waits for in-flight readers to finish, merges as
/// many WAL frames as possible into the main DB, and resets the WAL file. This shrinks the
/// `-wal` file after bulk deletes and is safe for the encrypted store. Failures are logged
/// but do not propagate, since a missed checkpoint only leaves the WAL larger than desired.
pub(crate) fn checkpoint_wal() {
    // ExecuteSql is the only path that accepts this PRAGMA: Execute rejects multi-column
    // PRAGMAs and Query rejects PRAGMA as non-query sql. TRUNCATE merges the WAL into the
    // main DB and physically truncates -wal to 0 bytes; degrades to a plain checkpoint (no
    // truncation) if a reader is in flight.
    if let Err(e) = REQUEST_DB.execute_sql("PRAGMA wal_checkpoint(TRUNCATE)") {
        error!("Failed to checkpoint WAL: {}", e);
    } else {
        info!("WAL checkpoint completed");
    }
}

#[cfg(test)]
mod ut_database {
    include!("../../tests/ut/ut_database.rs");
}
