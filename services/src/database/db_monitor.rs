// Copyright (C) 2026 Huawei Device Co., Ltd.
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

//! Database monitoring and metrics reporting for request service.
//!
//! This module provides functionality to monitor database size and record counts,
//! and report metrics via system events when thresholds are exceeded.

use super::REQUEST_DB;
use crate::sys_event::{isys_fault, DfxCode};
use crate::task::info::State;

/// Baseline threshold for database size in bytes (10MB).
const DB_SIZE_BASELINE: u64 = 10 * 1024 * 1024;

/// Number of top bundles to report.
const TOP_BUNDLE_COUNT: usize = 5;

/// Database file sizes for main, WAL, and SHM files.
#[derive(Debug, Default)]
pub(crate) struct DbFileSize {
    /// Main database file size in bytes.
    pub(crate) main_size: u64,
    /// WAL file size in bytes.
    pub(crate) wal_size: u64,
    /// SHM file size in bytes.
    pub(crate) shm_size: u64,
}

impl DbFileSize {
    /// Returns the total size of all database files.
    pub(crate) fn total(&self) -> u64 {
        self.main_size + self.wal_size + self.shm_size
    }
}

/// Database monitoring result containing metrics information.
#[derive(Debug, Default)]
pub(crate) struct DbMonitorResult {
    /// Database file sizes (main, WAL, SHM).
    pub(crate) db_file_size: DbFileSize,
    /// Total number of records in the task table.
    pub(crate) total_records: u64,
    /// Distribution of tasks by state.
    pub(crate) state_distribution: Vec<(State, u64)>,
    /// Top bundles by task count.
    pub(crate) top_bundles: Vec<(String, u64)>,
    /// Maximum token length in the task table.
    pub(crate) max_token_len: u64,
    /// Whether the database size exceeds baseline.
    pub(crate) size_exceeded: bool,
}

/// Monitors database metrics and reports via system events if thresholds are exceeded.
///
/// This function checks:
/// 1. Database file size against `DB_SIZE_BASELINE`
/// 2. State distribution when size exceeds baseline
/// 3. Top bundles by task count when size exceeds baseline
///
/// Reports are sent via HiSysEvent for analytics.
pub(crate) fn monitor_database() {
    let result = match collect_db_metrics() {
        Some(r) => r,
        None => return,
    };

    // Report only if database size exceeds baseline
    if !result.size_exceeded {
        debug!(
            "Database metrics normal: main={} wal={} shm={} total={} bytes",
            result.db_file_size.main_size,
            result.db_file_size.wal_size,
            result.db_file_size.shm_size,
            result.db_file_size.total()
        );
        return;
    }

    info!(
        "Database metrics exceeded baseline: main={} wal={} shm={} total={} bytes (baseline={})",
        result.db_file_size.main_size,
        result.db_file_size.wal_size,
        result.db_file_size.shm_size,
        result.db_file_size.total(),
        DB_SIZE_BASELINE
    );

    // Write the system event
    let extra_info = format!(
        "main_size={},wal_size={},shm_size={},total_size={},records={},size_exceeded={},state_dist={},top_bundles={},max_token_len={}",
        result.db_file_size.main_size,
        result.db_file_size.wal_size,
        result.db_file_size.shm_size,
        result.db_file_size.total(),
        result.total_records,
        result.size_exceeded,
        format_state_distribution(&result.state_distribution),
        format_top_bundles(&result.top_bundles),
        result.max_token_len
    );
    isys_fault(DfxCode::SA_FAULT_02, extra_info.as_str());
}

/// Collects database metrics including size, record count, state distribution, and top bundles.
///
/// # Returns
///
/// `Some(DbMonitorResult)` if metrics were successfully collected, `None` otherwise.
fn collect_db_metrics() -> Option<DbMonitorResult> {
    let db_file_size = match get_db_size() {
        Some(size) => size,
        None => {
            error!("Failed to get database size");
            return None;
        }
    };

    let size_exceeded = db_file_size.total() > DB_SIZE_BASELINE;

    // Only collect additional metrics if baseline is exceeded
    if !size_exceeded {
        return Some(DbMonitorResult {
            db_file_size,
            total_records: 0,
            state_distribution: Vec::new(),
            top_bundles: Vec::new(),
            max_token_len: 0,
            size_exceeded,
        });
    }

    let total_records = match get_total_record_count() {
        Some(count) => count,
        None => {
            error!("Failed to get total record count");
            return None;
        }
    };

    let state_distribution = get_state_distribution();
    let top_bundles = get_top_bundles();
    let max_token_len = match get_max_token_len() {
        Some(len) => len,
        None => {
            error!("Failed to get max token length");
            return None;
        }
    };

    Some(DbMonitorResult {
        db_file_size,
        total_records,
        state_distribution,
        top_bundles,
        max_token_len,
        size_exceeded,
    })
}

/// Gets the database file sizes (main, WAL, SHM).
///
/// # Returns
///
/// `Some(DbFileSize)` with the file sizes in bytes, or `None` if the main file doesn't exist.
fn get_db_size() -> Option<DbFileSize> {
    let db_path = super::DB_PATH;

    // Main database file
    let main_size = match std::fs::metadata(db_path) {
        Ok(metadata) => metadata.len(),
        Err(e) => {
            error!("Failed to get database metadata: {}", e);
            return None;
        }
    };

    // WAL file
    let wal_path = format!("{}-wal", db_path);
    let wal_size = if let Ok(metadata) = std::fs::metadata(wal_path) {
        metadata.len()
    } else {
        0
    };

    // SHM file
    let shm_path = format!("{}-shm", db_path);
    let shm_size = if let Ok(metadata) = std::fs::metadata(shm_path) {
        metadata.len()
    } else {
        0
    };

    Some(DbFileSize {
        main_size,
        wal_size,
        shm_size,
    })
}

/// Gets the total record count from the request_task table.
///
/// # Returns
///
/// `Some(u64)` with the total record count, or `None` if the query failed.
fn get_total_record_count() -> Option<u64> {
    let mut count = match REQUEST_DB.query::<u64>("SELECT COUNT(*) FROM request_task", ()) {
        Ok(rows) => rows,
        Err(e) => {
            error!("Failed to query total record count: {}", e);
            return None;
        }
    };

    count.next()
}

/// Gets the maximum token length from the request_task table.
///
/// # Returns
///
/// `Some(u64)` with the maximum token length, or `None` if the query failed.
/// Returns 0 when the table is empty or all tokens are NULL/empty.
fn get_max_token_len() -> Option<u64> {
    let mut len = match REQUEST_DB.query::<u64>(
        "SELECT COALESCE(MAX(LENGTH(token)), 0) FROM request_task",
        (),
    ) {
        Ok(rows) => rows,
        Err(e) => {
            error!("Failed to query max token length: {}", e);
            return None;
        }
    };

    len.next()
}

/// Gets the distribution of tasks by state.
///
/// # Returns
///
/// A vector of (State, count) tuples.
fn get_state_distribution() -> Vec<(State, u64)> {
    let sql = "SELECT state, COUNT(*) FROM request_task GROUP BY state";
    let rows = match REQUEST_DB.query::<(u32, u64)>(sql, ()) {
        Ok(rows) => rows,
        Err(e) => {
            error!("Failed to query state distribution: {}", e);
            return Vec::new();
        }
    };

    rows.filter_map(|(repr, count)| {
        if count == 0 {
            return None;
        }
        // Map repr value back to State enum
        let state = match repr {
            0x00 => State::Initialized,
            0x10 => State::Waiting,
            0x20 => State::Running,
            0x21 => State::Retrying,
            0x30 => State::Paused,
            0x31 => State::Stopped,
            0x40 => State::Completed,
            0x41 => State::Failed,
            0x50 => State::Removed,
            0x61 => State::Any,
            _ => {
                error!("Unknown state repr: {}", repr);
                return None;
            }
        };
        Some((state, count))
    }).collect()
}

/// Gets the top bundles by task count.
///
/// # Returns
///
/// A vector of (bundle_name, count) tuples, sorted by count descending.
fn get_top_bundles() -> Vec<(String, u64)> {
    let sql = format!(
        "SELECT bundle, COUNT(*) as cnt FROM request_task GROUP BY bundle ORDER BY cnt DESC LIMIT {}",
        TOP_BUNDLE_COUNT
    );

    let rows = match REQUEST_DB.query::<(String, u64)>(&sql, ()) {
        Ok(rows) => rows,
        Err(e) => {
            error!("Failed to query top bundles: {}", e);
            return Vec::new();
        }
    };

    rows.collect()
}

/// Formats state distribution as a string for reporting.
fn format_state_distribution(distribution: &[(State, u64)]) -> String {
    distribution
        .iter()
        .map(|(state, count)| format!("{:?}:{}", state, count))
        .collect::<Vec<_>>()
        .join(",")
}

/// Formats top bundles as a string for reporting.
fn format_top_bundles(bundles: &[(String, u64)]) -> String {
    bundles
        .iter()
        .map(|(bundle, count)| format!("{}:{}", bundle, count))
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod ut_db_monitor {
    include!("../../tests/ut/db_monitor.rs");
}
