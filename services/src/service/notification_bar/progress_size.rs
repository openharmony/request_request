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

use super::ffi::GetSystemLanguage;

const COMMA_SEPARATOR_LIST: &[&str] = &["hi", "bn"];
const SPACE_SEPARATOR_LIST: &[&str] = &["fi"];
const COMMA_DECIMAL_POINT_LIST: &[&str] = &["sl", "lt", "fi", "nl", "da"];
const SPACE_BEFORE_UNIT_LIST: &[&str] = &[
    "sl", "lt", "hi", "fi", "nl", "da", "my", "bn", "zh-Hant", "en",
];
const T_UNIT_LIST: &[&str] = &["fi"];

#[derive(Debug)]
struct FormattedSize {
    integer: u64,
    decimal: Option<String>,
    unit_str: &'static str,
}

#[derive(Debug, PartialEq)]
enum Unit {
    Bytes,
    KiloBytes,
    MegaBytes,
    GigaBytes,
}

impl Unit {
    fn as_str(&self, lang: &str) -> &'static str {
        if T_UNIT_LIST.contains(&lang) {
            match self {
                Unit::Bytes => "T",
                Unit::KiloBytes => "KT",
                Unit::MegaBytes => "MT",
                Unit::GigaBytes => "GT",
            }
        } else {
            match self {
                Unit::Bytes => "B",
                Unit::KiloBytes => "KB",
                Unit::MegaBytes => "MB",
                Unit::GigaBytes => "GB",
            }
        }
    }
}

impl FormattedSize {
    fn format_size_with_unit(size: f64, unit_str: &Unit, lang: &str) -> Self {
        let integer = size.trunc() as u64;

        let decimal = if unit_str == &Unit::Bytes {
            None
        } else {
            Some(format!("{:02}", (size.fract() * 100.0).floor()))
        };

        let unit_str = unit_str.as_str(lang);

        Self {
            integer,
            decimal,
            unit_str,
        }
    }

    fn separator(&self, lang: &str) -> &'static str {
        if COMMA_SEPARATOR_LIST.contains(&lang) {
            return ",";
        } else if SPACE_SEPARATOR_LIST.contains(&lang) {
            return " ";
        }
        ""
    }

    fn decimal_point(&self, lang: &str) -> &'static str {
        if self.decimal.is_none() {
            return "";
        }
        if COMMA_DECIMAL_POINT_LIST.contains(&lang) {
            ","
        } else {
            "."
        }
    }

    fn needs_space_before_unit(&self, lang: &str) -> bool {
        SPACE_BEFORE_UNIT_LIST.contains(&lang)
    }

    fn integer_format_with_separator(&self, separator: &str) -> String {
        let num_str = self.integer.to_string();
        let mut result = String::new();

        for (i, c) in num_str.chars().rev().enumerate() {
            if i != 0 && i % 3 == 0 {
                result.push_str(separator);
            }
            result.push(c);
        }

        result.chars().rev().collect()
    }

    fn with_locale(&self, lang: &str) -> String {
        let separator = self.separator(lang);
        let decimal_point = self.decimal_point(lang);
        let space = if self.needs_space_before_unit(lang) {
            " "
        } else {
            ""
        };

        let integer = self.integer_format_with_separator(separator);

        format!(
            "{}{}{}{}{}",
            integer,
            decimal_point,
            self.decimal.as_deref().unwrap_or(""),
            space,
            self.unit_str
        )
    }
}

pub fn progress_size(current: u64) -> String {
    let lang = GetSystemLanguage();
    progress_size_with_lang(current, &lang)
}

fn progress_size_with_lang(current: u64, lang: &str) -> String {
    let (size, unit_str) = calculate_size_and_unit(current);
    let formatted = FormattedSize::format_size_with_unit(size, &unit_str, lang);

    formatted.with_locale(lang)
}

fn calculate_size_and_unit(current: u64) -> (f64, Unit) {
    match current {
        0..=1023 => (current as f64, Unit::Bytes),
        1024..=1_048_575 => (current as f64 / 1024.0, Unit::KiloBytes),
        1_048_576..=1_073_741_823 => (current as f64 / 1_048_576.0, Unit::MegaBytes),
        _ => (current as f64 / 1_073_741_824.0, Unit::GigaBytes),
    }
}

#[cfg(test)]
mod ut_progress_size {
    include!("../../../tests/ut/service/notification_bar/ut_progress_size.rs");
}
