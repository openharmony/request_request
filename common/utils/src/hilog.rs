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

//! Logging utilities using HILog (HarmonyOS Logging).
//! 
//! This module provides macros for logging at different severity levels using
//! the HarmonyOS logging system. The macros included are deprecated versions
//! that should be used with caution.

/// Logs a debug message using HILog.
///
/// # Examples
///
/// ```rust
/// use request_utils::debug_deprecated;
///
/// let value = 42;
/// debug_deprecated!("Debug message with value: {}", value);
/// ```
///
/// # Notes
///
/// This macro is deprecated. Consider using a more up-to-date logging approach.
/// Uses the crate's configured domain and tag values for the log message.
#[macro_export]
macro_rules! debug_deprecated {
    ($fmt: literal $(, $args:expr)* $(,)?) => {{
        // Format the message string before passing to the logging function
        let fmt = format!($fmt $(, $args)*);
        $crate::hilog_print($crate::LogLevel::LOG_DEBUG, crate::DOMAIN, crate::TAG,  fmt);
    }}
}

/// Logs an info message using HILog.
///
/// # Examples
///
/// ```rust
/// use request_utils::info_deprecated;
///
/// let operation = "download";
/// info_deprecated!("Operation {} completed successfully", operation);
/// ```
///
/// # Notes
///
/// This macro is deprecated. Consider using a more up-to-date logging approach.
/// Uses the crate's configured domain and tag values for the log message.
#[macro_export]
macro_rules! info_deprecated {
    ($fmt: literal $(, $args:expr)* $(,)?) => {{
        // Format the message string before passing to the logging function
        let fmt = format!($fmt $(, $args)*);
        $crate::hilog_print($crate::LogLevel::LOG_INFO, crate::DOMAIN, crate::TAG,  fmt);
    }}
}

/// Logs an error message using HILog.
///
/// # Examples
///
/// ```rust
/// use request_utils::error_deprecated;
///
/// let error_code = 500;
/// error_deprecated!("Failed to process request with error code: {}", error_code);
/// ```
///
/// # Notes
///
/// This macro is deprecated. Consider using a more up-to-date logging approach.
/// Uses the crate's configured domain and tag values for the log message.
#[macro_export]
macro_rules! error_deprecated {
    ($fmt: literal $(, $args:expr)* $(,)?) => {{
        // Format the message string before passing to the logging function
        let fmt = format!($fmt $(, $args)*);
        $crate::hilog_print($crate::LogLevel::LOG_ERROR, crate::DOMAIN, crate::TAG,  fmt);
    }}
}
