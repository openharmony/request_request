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

use super::*;

// @tc.name: ut_db_monitor_format_state_distribution_empty
// @tc.desc: Test formatting empty state distribution
// @tc.precon: NA
// @tc.step: 1. Call format_state_distribution with empty vector
// @tc.expect: Returns empty string
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
#[test]
fn ut_db_monitor_format_state_distribution_empty() {
    let result = format_state_distribution(&[]);
    assert_eq!(result, "");
}

// @tc.name: ut_db_monitor_format_state_distribution_single
// @tc.desc: Test formatting single state distribution entry
// @tc.precon: NA
// @tc.step: 1. Create vector with single (State, count) tuple
//           2. Call format_state_distribution
// @tc.expect: Returns formatted string without trailing comma
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
#[test]
fn ut_db_monitor_format_state_distribution_single() {
    let distribution = vec![(State::Running, 10)];
    let result = format_state_distribution(&distribution);
    assert_eq!(result, "Running:10");
}

// @tc.name: ut_db_monitor_format_state_distribution_multiple
// @tc.desc: Test formatting multiple state distribution entries
// @tc.precon: NA
// @tc.step: 1. Create vector with multiple (State, count) tuples
//           2. Call format_state_distribution
// @tc.expect: Returns comma-separated formatted string
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
#[test]
fn ut_db_monitor_format_state_distribution_multiple() {
    let distribution = vec![
        (State::Initialized, 5),
        (State::Running, 10),
        (State::Completed, 3),
    ];
    let result = format_state_distribution(&distribution);
    assert_eq!(result, "Initialized:5,Running:10,Completed:3");
}

// @tc.name: ut_db_monitor_format_top_bundles_empty
// @tc.desc: Test formatting empty top bundles
// @tc.precon: NA
// @tc.step: 1. Call format_top_bundles with empty vector
// @tc.expect: Returns empty string
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
#[test]
fn ut_db_monitor_format_top_bundles_empty() {
    let result = format_top_bundles(&[]);
    assert_eq!(result, "");
}

// @tc.name: ut_db_monitor_format_top_bundles_single
// @tc.desc: Test formatting single top bundle entry
// @tc.precon: NA
// @tc.step: 1. Create vector with single (bundle, count) tuple
//           2. Call format_top_bundles
// @tc.expect: Returns formatted string without trailing comma
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
#[test]
fn ut_db_monitor_format_top_bundles_single() {
    let bundles = vec![("com.example.app".to_string(), 15)];
    let result = format_top_bundles(&bundles);
    assert_eq!(result, "com.example.app:15");
}

// @tc.name: ut_db_monitor_format_top_bundles_multiple
// @tc.desc: Test formatting multiple top bundle entries
// @tc.precon: NA
// @tc.step: 1. Create vector with multiple (bundle, count) tuples
//           2. Call format_top_bundles
// @tc.expect: Returns comma-separated formatted string
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
#[test]
fn ut_db_monitor_format_top_bundles_multiple() {
    let bundles = vec![
        ("com.example.app1".to_string(), 20),
        ("com.example.app2".to_string(), 15),
        ("com.example.app3".to_string(), 10),
    ];
    let result = format_top_bundles(&bundles);
    assert_eq!(
        result,
        "com.example.app1:20,com.example.app2:15,com.example.app3:10"
    );
}

// @tc.name: ut_db_monitor_result_default
// @tc.desc: Test DbMonitorResult default values
// @tc.precon: NA
// @tc.step: 1. Create DbMonitorResult with default values
// @tc.expect: All fields have default values (0, empty vectors, false)
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
#[test]
fn ut_db_monitor_result_default() {
    let result = DbMonitorResult::default();
    assert_eq!(result.db_file_size.main_size, 0);
    assert_eq!(result.db_file_size.wal_size, 0);
    assert_eq!(result.db_file_size.shm_size, 0);
    assert_eq!(result.total_records, 0);
    assert!(result.state_distribution.is_empty());
    assert!(result.top_bundles.is_empty());
    assert_eq!(result.max_token_len, 0);
    assert!(!result.size_exceeded);
}
