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

use crate::service::notification_bar::progress_size::*;

// @tc.name: ut_progress_size_new
// @tc.desc: Test ProgressSizeFormatter can be created successful.
// @tc.precon: NA
// @tc.step: 1. Create a ProgressSizeFormatter
// @tc.expect: ProgressSizeFormatter be created successful and inited.
// @tc.type: FUNC
// @tc.require: issues#ICLN0G
#[test]
fn ut_progress_size_new() {
    let (size, unit_str) = calculate_size_and_unit(1);
    let formatter = FormattedSize::format_size_with_unit(size, &unit_str, "zh-Hans");
    assert_eq!(formatter.integer, 1);
    assert!(formatter.decimal.is_none());
    assert_eq!(formatter.unit_str, "B");
}

// @tc.name: ut_progress_size_progress_size_units
// @tc.desc: Test unit conversion for different byte sizes.
// @tc.precon: ProgressSizeFormatter created with default values.
// @tc.step: 1. Call progress_size_units with various byte sizes.
// @tc.expect: Correct unit string (B/KB/MB/GB) and size value set for each
// input. @tc.type: FUNC
// @tc.require: issues#ICLN0G
#[test]
fn ut_progress_size_progress_size_units() {
    let (size, unit_str) = calculate_size_and_unit(1);
    assert_eq!(unit_str, Unit::Bytes);
    assert_eq!(size, 1.0);
    let (size, unit_str) = calculate_size_and_unit(1024);
    assert_eq!(unit_str, Unit::KiloBytes);
    assert_eq!(size, 1.0);
    let (size, unit_str) = calculate_size_and_unit(1024 * 1024);
    assert_eq!(unit_str, Unit::MegaBytes);
    assert_eq!(size, 1.0);
    let (size, unit_str) = calculate_size_and_unit(1024 * 1024 * 1024);
    assert_eq!(unit_str, Unit::GigaBytes);
    assert_eq!(size, 1.0);
}

// @tc.name: ut_progress_size_space_format
// @tc.desc: Test space formatting based on language.
// @tc.precon: ProgressSizeFormatter with lang field set.
// @tc.step: 1. Call space_format method with different languages
// @tc.expect: Correct space character (empty or " ") based on language.
// @tc.type: FUNC
// @tc.require: issues#ICLN0G
#[test]
fn ut_progress_size_space_format() {
    let (size, unit_str) = calculate_size_and_unit(1);
    let formatter = FormattedSize::format_size_with_unit(size, &unit_str, "zh-Hans");
    let needs = formatter.needs_space_before_unit("zh-Hans");
    assert!(!needs);
    let needs = formatter.needs_space_before_unit("sl");
    assert!(needs);
}

// @tc.name: ut_progress_size_point_format
// @tc.desc: Test decimal point formatting based on language and value.
// @tc.precon: ProgressSizeFormatter with lang field set and size units
// initialized. @tc.step: 1. Call point_format with different languages and
// sizes @tc.expect: Correct decimal point character ("." or ",") based on
// language. @tc.type: FUNC
// @tc.require: issues#ICLN0G
#[test]
fn ut_progress_size_point_format() {
    let (size, unit_str) = calculate_size_and_unit(1);
    let formatter = FormattedSize::format_size_with_unit(size, &unit_str, "zh-Hans");
    assert_eq!(formatter.decimal_point("zh-Hans"), "");

    let (size, unit_str) = calculate_size_and_unit(1025);
    let formatter = FormattedSize::format_size_with_unit(size, &unit_str, "zh-Hans");
    assert_eq!(formatter.decimal_point("zh-Hans"), ".");

    let (size, unit_str) = calculate_size_and_unit(1025);
    let formatter = FormattedSize::format_size_with_unit(size, &unit_str, "sl");
    assert_eq!(formatter.decimal_point("sl"), ",");
}

// @tc.name: ut_progress_size_unitstr_format
// @tc.desc: Test unit string localization.
// @tc.precon: ProgressSizeFormatter with lang field set and size units
// initialized. @tc.step: 1. Call unitstr_format with different languages
// @tc.expect: Unit string localized according to language.
// @tc.type: FUNC
// @tc.require: issues#ICLN0G
#[test]
fn ut_progress_size_unitstr_format() {
    let (size, unit_str) = calculate_size_and_unit(1025);
    let formatter = FormattedSize::format_size_with_unit(size, &unit_str, "zh-Hans");
    assert_eq!(formatter.unit_str, "KB");

    let (size, unit_str) = calculate_size_and_unit(1025);
    let formatter = FormattedSize::format_size_with_unit(size, &unit_str, "fi");
    assert_eq!(formatter.unit_str, "KT");
}

// @tc.name: ut_progress_size_decimal_format
// @tc.desc: Test decimal part formatting based on fractional value.
// @tc.precon: ProgressSizeFormatter with size units initialized.
// @tc.step: 1. Call decimal_format with different fractional values
// @tc.expect: Correct decimal digits generated.
// @tc.type: FUNC
// @tc.require: issues#ICLN0G
#[test]
fn ut_progress_size_decimal_format() {
    let (size, unit_str) = calculate_size_and_unit(1);
    let formatter = FormattedSize::format_size_with_unit(size, &unit_str, "zh-Hans");
    assert!(formatter.decimal.is_none());

    let (size, unit_str) = calculate_size_and_unit(1024);
    let formatter = FormattedSize::format_size_with_unit(size, &unit_str, "zh-Hans");
    assert!(formatter.decimal.is_some());
    assert_eq!(formatter.decimal.unwrap(), "00");

    let (size, unit_str) = calculate_size_and_unit(1536);
    let formatter = FormattedSize::format_size_with_unit(size, &unit_str, "zh-Hans");
    assert!(formatter.decimal.is_some());
    assert_eq!(formatter.decimal.unwrap(), "50");
}

// @tc.name: ut_progress_size_separator_format
// @tc.desc: Test digit separator localization.
// @tc.precon: ProgressSizeFormatter with lang set and integer components
// initialized. @tc.step: 1. Call separator_format with different languages
// @tc.expect: Correct thousand/million separators ("," or space) based on
// language. @tc.type: FUNC
// @tc.require: issues#ICLN0G
#[test]
fn ut_progress_size_separator_format() {
    let size = 1.234;
    let unit_str = Unit::Bytes;
    let formatter = FormattedSize::format_size_with_unit(size, &unit_str, "zh-Hans");

    assert_eq!(formatter.separator("zh-Hans"), "");
    assert_eq!(formatter.separator("hi"), ",");
    assert_eq!(formatter.separator("fi"), " ");
}

// @tc.name: ut_progress_size_inner
// @tc.desc: Test full formatting pipeline with localization.
// @tc.precon: ProgressSizeFormatter with lang set.
// @tc.step: 1. Call progress_size_inner with large byte value
// @tc.expect: Correctly formatted and localized size string for each language.
// @tc.type: FUNC
// @tc.require: issues#ICLN0G
#[test]
fn ut_progress_size_inner() {
    let current: u64 = 123456789123456789;
    let lang = "zh-Hans".to_string();
    let res = progress_size_with_lang(current, &lang);
    assert_eq!(res, "114978094.70GB");

    let lang = "zh-Hant".to_string();
    let res = progress_size_with_lang(current, &lang);
    assert_eq!(res, "114978094.70 GB");

    let lang = "hi".to_string();
    let res = progress_size_with_lang(current, &lang);
    assert_eq!(res, "114,978,094.70 GB");

    let lang = "sl".to_string();
    let res = progress_size_with_lang(current, &lang);
    assert_eq!(res, "114978094,70 GB");

    let lang = "fi".to_string();
    let res = progress_size_with_lang(current, &lang);
    assert_eq!(res, "114 978 094,70 GT");
}
