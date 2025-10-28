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

//! HarmonyOS logging macros for request operations.
//! 
//! This module provides convenient logging macros that wrap the `hilog_rust` crate,
//! offering a simplified interface for logging at different severity levels
//! (debug, info, and error) with consistent formatting and label usage.

/// Logs a debug-level message using HarmonyOS logging.
///
/// Uses the crate's configured `LOG_LABEL` for consistent log identification.
/// Supports formatted strings with the same syntax as standard formatting macros.
/// All arguments are logged as public information.
///
/// # Examples
///
/// ```rust
/// use request_utils::debug;
///
/// let value = 42;
/// debug!("Debug message with value: {}", value);
///
/// // With trailing comma
/// debug!("Multiple values: {}, {}", 1, 2,);
/// ```
#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $args:expr)* $(,)?) => {{
        use std::ffi::{CString, c_char};
        use hilog_rust::{debug, hilog};
        use crate::LOG_LABEL;

        // Log debug message with public visibility
        hilog_rust::debug!(LOG_LABEL, $fmt $(, @public($args))*);
    }}
}

/// Logs an info-level message using HarmonyOS logging.
///
/// Uses the crate's configured `LOG_LABEL` for consistent log identification.
/// Supports formatted strings with the same syntax as standard formatting macros.
/// All arguments are logged as public information.
///
/// # Examples
///
/// ```rust
/// use request_utils::info;
///
/// let operation = "initialize";
/// info!("Operation completed: {}", operation);
///
/// // With trailing comma
/// info!("Status: {}, ID: {}", "active", "123",);
/// ```
#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $args:expr)* $(,)?) => {{
        use std::ffi::{CString, c_char};
        use hilog_rust::{info, hilog};
        use crate::LOG_LABEL;

        // Log info message with public visibility
        hilog_rust::info!(LOG_LABEL, $fmt $(, @public($args))*);
    }}
}

/// Logs an error-level message using HarmonyOS logging.
///
/// Uses the crate's configured `LOG_LABEL` for consistent log identification.
/// Supports formatted strings with the same syntax as standard formatting macros.
/// All arguments are logged as public information.
///
/// # Examples
///
/// ```rust
/// use request_utils::error;
///
/// let error_code = 404;
/// error!("Request failed with error code: {}", error_code);
///
/// // With trailing comma
/// error!("Critical failure: {}, Details: {}", "connection lost", "timeout",);
/// ```
#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $args:expr)* $(,)?) => {{
        use std::ffi::{CString, c_char};
        use hilog_rust::{error, hilog};
        use crate::LOG_LABEL;

        // Log error message with public visibility
        hilog_rust::error!(LOG_LABEL, $fmt $(, @public($args))*);
    }}
}
