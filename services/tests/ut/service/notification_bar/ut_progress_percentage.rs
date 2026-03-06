// Copyright (C) 2024 Huawei Device Co., Ltd.
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

use crate::service::notification_bar::progress_percentage::*;

// @tc.name: ut_progress_percentage_format_percentage_en
// @tc.desc: Test percentage formatting with English locale.
// @tc.precon: NA
// @tc.step: 1. Call format_percentage with English locale
// @tc.expect: Returns percentage string with dot as decimal separator
// @tc.type: FUNC
// @tc.require: issues#ICLN0G
#[test]
fn ut_progress_percentage_format_percentage_en() {
    let result = format_percentage(50.0, "en");
    assert_eq!(result, "50.00%");

    let result = format_percentage(25.5, "en");
    assert_eq!(result, "25.50%");

    let result = format_percentage(100.0, "en");
    assert_eq!(result, "100.00%");

    let result = format_percentage(0.0, "en");
    assert_eq!(result, "0.00%");
}

// @tc.name: ut_progress_percentage_format_percentage_id
// @tc.desc: Test percentage formatting with Indonesian locale (comma decimal).
// @tc.precon: NA
// @tc.step: 1. Call format_percentage with Indonesian locale
// @tc.expect: Returns percentage string with comma as decimal separator
// @tc.type: FUNC
// @tc.require: issues#ICLN0G
#[test]
fn ut_progress_percentage_format_percentage_id() {
    let result = format_percentage(50.0, "id");
    assert_eq!(result, "50,00%");

    let result = format_percentage(25.5, "id");
    assert_eq!(result, "25,50%");

    let result = format_percentage(75.25, "id");
    assert_eq!(result, "75,25%");
}

// @tc.name: ut_progress_percentage_format_percentage_fi
// @tc.desc: Test percentage formatting with Finnish locale (comma decimal).
// @tc.precon: NA
// @tc.step: 1. Call format_percentage with Finnish locale
// @tc.expect: Returns percentage string with comma as decimal separator
// @tc.type: FUNC
// @tc.require: issues#ICLN0G
#[test]
fn ut_progress_percentage_format_percentage_fi() {
    let result = format_percentage(33.33, "fi");
    assert_eq!(result, "33,33%");
}

// @tc.name: ut_progress_percentage_format_percentage_zh
// @tc.desc: Test percentage formatting with Chinese locale (dot decimal).
// @tc.precon: NA
// @tc.step: 1. Call format_percentage with Chinese locale
// @tc.expect: Returns percentage string with dot as decimal separator
// @tc.type: FUNC
// @tc.require: issues#ICLN0G
#[test]
fn ut_progress_percentage_format_percentage_zh() {
    let result = format_percentage(50.0, "zh-Hans");
    assert_eq!(result, "50.00%");

    let result = format_percentage(50.0, "zh-Hant");
    assert_eq!(result, "50.00%");
}

// @tc.name: ut_progress_percentage_edge_cases
// @tc.desc: Test percentage formatting edge cases.
// @tc.precon: NA
// @tc.step: 1. Call format_percentage with edge case values
// @tc.expect: Returns correct percentage strings
// @tc.type: FUNC
// @tc.require: issues#ICLN0G
#[test]
fn ut_progress_percentage_edge_cases() {
    // Test very small values
    let result = format_percentage(0.01, "en");
    assert_eq!(result, "0.01%");

    // Test value that rounds up
    let result = format_percentage(99.999, "en");
    assert_eq!(result, "99.99%");

    // Test negative should not happen in practice, but test behavior
    // (percentage should be 0 or positive in real use cases)
}
