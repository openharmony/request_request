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

//! Rust interface to the netstack library.
//! 
//! This crate provides a safe, idiomatic Rust API for interacting with the 
//! netstack library, handling HTTP requests, responses, and task management.
//! 
//! # Modules
//! 
//! * [`request`] - Types and functionality for creating HTTP requests
//! * [`task`] - Types and functionality for managing HTTP request tasks
//! * [`response`] - Types and functionality for handling HTTP responses
//! * [`error`] - Error types and handling
//! * [`info`] - Types for download and performance information

#![warn(
    missing_docs,
    clippy::redundant_static_lifetimes,
    clippy::enum_variant_names,
    clippy::clone_on_copy,
    clippy::unused_async
)]
#![deny(unused_must_use)]
#![allow(missing_docs, clippy::new_without_default)]

/// Types and functionality for creating HTTP requests.
///
/// This module provides structures and methods for constructing HTTP requests,
/// including setting headers, method, URL, and body content.
pub mod request;

/// Types and functionality for managing HTTP request tasks.
///
/// This module provides structures for tracking and controlling the lifecycle
/// of HTTP requests, including starting, canceling, and checking status.
pub mod task;

/// Types and functionality for handling HTTP responses.
///
/// This module provides structures for accessing response data, including
/// status codes, headers, and response body.
pub mod response;

/// Error types and handling for HTTP operations.
///
/// This module defines error types that can occur during HTTP operations.
pub mod error;

/// Internal FFI wrapper module.
///
/// Provides bridging between Rust code and the underlying netstack C++ implementation.
mod wrapper;

/// Types for download and performance information.
///
/// This module provides structures for tracking download progress and collecting
/// performance metrics during HTTP operations.
pub mod info;

use hilog_rust::{HiLogLabel, LogType};

/// Log label used for logging within the netstack_rs crate.
///
/// # Notes
///
/// This label is used with the HiLog logging system for consistent log categorization.
pub(crate) const LOG_LABEL: HiLogLabel = HiLogLabel {
    log_type: LogType::LogCore,
    domain: 0xD001C50,
    tag: "PreloadNative",
};
