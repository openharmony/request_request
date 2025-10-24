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

// Language codes that use comma as thousand separator
const COMMA_SEPARATOR_LIST: &[&str] = &["hi", "bn"];

// Language codes that use space as thousand separator
const SPACE_SEPARATOR_LIST: &[&str] = &["fi"];

// Language codes that use comma as decimal point
const COMMA_DECIMAL_POINT_LIST: &[&str] = &["sl", "lt", "fi", "nl", "da"];

// Language codes that require space between number and unit
const SPACE_BEFORE_UNIT_LIST: &[&str] = &[
    "sl", "lt", "hi", "fi", "nl", "da", "my", "bn", "zh-Hant", "en",
];

// Language codes that use 'T' instead of 'B' for byte units (e.g., 'KT' instead of 'KB')
const T_UNIT_LIST: &[&str] = &["fi"];

/// Represents a formatted file size with locale-aware components.
#[derive(Debug)]
struct FormattedSize {
    /// Integer part of the size value.
    integer: u64,
    /// Optional decimal part of the size value (used for units larger than bytes).
    decimal: Option<String>,
    /// String representation of the size unit (B, KB, MB, GB or locale variants).
    unit_str: &'static str,
}

/// Size units for file size representation.
#[derive(Debug, PartialEq)]
enum Unit {
    /// Byte unit (B).
    Bytes,
    /// Kilobyte unit (KB) = 1024 bytes.
    KiloBytes,
    /// Megabyte unit (MB) = 1024 KB.
    MegaBytes,
    /// Gigabyte unit (GB) = 1024 MB.
    GigaBytes,
}

impl Unit {
    /// Returns the string representation of the unit based on the specified language.
    /// 
    /// # Arguments
    /// 
    /// * `lang` - System language code to determine unit format
    /// 
    /// # Returns
    /// 
    /// String representation of the unit (B/KB/MB/GB or locale-specific variants)
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
    /// Creates a new FormattedSize instance with the given size and unit.
    /// 
    /// # Arguments
    /// 
    /// * `size` - Size value to format
    /// * `unit_str` - Unit to use for the size
    /// * `lang` - Language code to determine formatting
    /// 
    /// # Returns
    /// 
    /// New FormattedSize instance with integer and optional decimal parts
    fn format_size_with_unit(size: f64, unit_str: &Unit, lang: &str) -> Self {
        // Extract integer part of the size
        let integer = size.trunc() as u64;

        // Calculate decimal part (only for non-byte units)
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

    /// Determines the thousand separator to use based on the specified language.
    /// 
    /// # Arguments
    /// 
    /// * `lang` - System language code to determine separator
    /// 
    /// # Returns
    /// 
    /// String to use as thousand separator (comma, space, or empty string)
    fn separator(&self, lang: &str) -> &'static str {
        if COMMA_SEPARATOR_LIST.contains(&lang) {
            return ",";
        } else if SPACE_SEPARATOR_LIST.contains(&lang) {
            return " ";
        }
        ""
    }

    /// Determines the decimal point character to use based on the specified language.
    /// 
    /// # Arguments
    /// 
    /// * `lang` - System language code to determine decimal point
    /// 
    /// # Returns
    /// 
    /// String to use as decimal point (comma or period), or empty string if no decimal part
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

    /// Determines whether a space is needed between the number and unit based on the language.
    /// 
    /// # Arguments
    /// 
    /// * `lang` - System language code to determine spacing
    /// 
    /// # Returns
    /// 
    /// Boolean indicating whether a space is needed before the unit
    fn needs_space_before_unit(&self, lang: &str) -> bool {
        SPACE_BEFORE_UNIT_LIST.contains(&lang)
    }

    /// Formats the integer part with thousand separators.
    /// 
    /// # Arguments
    /// 
    /// * `separator` - Separator to use between thousands places
    /// 
    /// # Returns
    /// 
    /// String representation of the integer with proper thousand separators
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

    /// Formats the size with locale-specific formatting rules.
    /// 
    /// # Arguments
    /// 
    /// * `lang` - System language code to determine formatting rules
    /// 
    /// # Returns
    /// 
    /// Fully formatted string representation of the size with proper locale formatting
    fn with_locale(&self, lang: &str) -> String {
        // Get locale-specific formatting components
        let separator = self.separator(lang);
        let decimal_point = self.decimal_point(lang);
        let space = if self.needs_space_before_unit(lang) {
            " "
        } else {
            ""
        };

        // Format integer with appropriate thousand separators
        let integer = self.integer_format_with_separator(separator);

        // Combine all components into the final formatted string
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

/// Formats a file size in bytes to a human-readable string using the system language.
/// 
/// # Arguments
/// 
/// * `current` - File size in bytes
/// 
/// # Returns
/// 
/// Human-readable string representation of the size with appropriate unit and formatting
/// 
/// # Examples
/// 
/// ```rust
/// # use service::notification_bar::progress_size::progress_size;
/// assert!(progress_size(512).contains("512 B"));
/// assert!(progress_size(1536).contains("1.50")); // 1.50 KB or similar based on locale
/// assert!(progress_size(1048576).contains("1.00")); // 1.00 MB or similar based on locale
/// ```
pub fn progress_size(current: u64) -> String {
    let lang = GetSystemLanguage();
    progress_size_with_lang(current, &lang)
}

/// Formats a file size in bytes to a human-readable string using the specified language.
/// 
/// # Arguments
/// 
/// * `current` - File size in bytes
/// * `lang` - Language code to use for formatting
/// 
/// # Returns
/// 
/// Human-readable string representation of the size with locale-specific formatting
fn progress_size_with_lang(current: u64, lang: &str) -> String {
    let (size, unit_str) = calculate_size_and_unit(current);
    let formatted = FormattedSize::format_size_with_unit(size, &unit_str, lang);

    formatted.with_locale(lang)
}

/// Calculates the appropriate size value and unit for a given byte count.
/// 
/// # Arguments
/// 
/// * `current` - File size in bytes
/// 
/// # Returns
/// 
/// Tuple containing the adjusted size value and corresponding unit
/// 
/// # Notes
/// 
/// This function uses base-1024 units (1 KB = 1024 bytes) as is standard for file sizes.
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
