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

//! Common utilities for request operations.
//!
//! This crate provides a collection of utility functions and modules to support
//! request operations, including random number generation, file control,
//! logging, and other common utilities needed for request handling.

#![warn(missing_docs)]
#![allow(clippy::crate_in_macro_def)]
#![allow(missing_docs, clippy::new_without_default)]

/// Internal macros module.
#[macro_use]
mod macros;

/// Fast pseudorandom number generation utilities.
pub mod fastrand;

/// File path control and validation utilities.
pub mod file_control;

/// Hash utilities.
pub mod hash;

/// Internal logging module for HarmonyOS.
mod hilog;

/// Least Recently Used (LRU) cache implementation.
pub mod lru;

/// Task ID generation and management utilities.
pub mod task_id;

// Conditional compilation for non-OHOS platforms
// Provides standard logging macros from the log crate
cfg_not_ohos! {
    pub use log::{debug, error, info};
}

// Conditional compilation for OHOS platform
// Provides HarmonyOS-specific logging and utilities
cfg_ohos! {
    /// HarmonyOS-specific logging module.
    #[macro_use]
    pub mod request_hilog;

    /// Observation utilities for system events.
    pub mod observe;

    /// Application context management utilities.
    pub mod context;

    /// Internal wrapper module for system interactions.
    // todo:move some to request_next
    pub mod wrapper;

    /// Re-exports from the wrapper module for logging functionality.
    pub use wrapper::{hilog_print, LogLevel, LogType};

    /// Storage utilities for file operations.
    pub mod storage;
}

/// Testing utilities.
pub mod test;
