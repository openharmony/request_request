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

// Language codes that use comma as decimal separator for percentage
const COMMA_DECIMAL_LIST: &[&str] = &["sl", "lt", "fi", "nl", "da", "id"];

/// Formats a percentage value with locale-aware decimal separator.
///
/// # Arguments
///
/// * `percentage` - The percentage value as a floating point number (0-100)
/// * `lang` - Language code to determine decimal separator format
///
/// # Returns
///
/// Formatted percentage string with locale-aware decimal separator
fn format_percentage(percentage: f64, lang: &str) -> String {
    let truncated = (percentage * 100.0).floor() / 100.0;
    let formatted = format!("{:.2}%", truncated);

    let decimal_separator = if COMMA_DECIMAL_LIST.contains(&lang) {
        formatted.replace('.', ",")
    } else {
        formatted
    };

    decimal_separator
}

/// Formats a percentage progress value as a human-readable string.
/// 
/// # Arguments
/// 
/// * `current` - Current progress value
/// * `total` - Total progress value
/// 
/// # Returns
/// 
/// Human-readable percentage string with locale-aware formatting
/// 
/// # Examples
/// 
/// ```rust
/// # use service::notification_bar::progress_percentage::progress_percentage;
/// // For English locale, returns "50.00%" or similar based on locale
/// let result = progress_percentage(50, 100);
/// assert!(result.contains('%'));
/// ```
pub fn progress_percentage(current: u64, total: u64) -> String {
    if total == 0 {
        let lang = GetSystemLanguage();
        return format_percentage(100.0, &lang);
    }
    
    let percentage = (current as f64 * 100.0) / (total as f64);
    let lang = GetSystemLanguage();
    
    format_percentage(percentage, &lang)
}

#[cfg(test)]
mod ut_progress_percentage {
    include!("../../../tests/ut/service/notification_bar/ut_progress_percentage.rs");
}
