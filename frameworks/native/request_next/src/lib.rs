// Copyright (C) 2025 Huawei Device Co., Ltd.
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

//! Native Rust interface for the Request framework.
//!
//! The `request_next` crate provides a native Rust interface for interacting with the
//! download/upload service, enabling efficient task management, state observation,
//! and proxy communication.

#![feature(lazy_cell)]

/// Utility functions for request validation and error checking.
pub mod check;

/// Client interface for managing download/upload requests.
pub mod client;
pub mod file;
pub mod verify;
// pub mod wrapper;

/// Internal proxy implementation for service communication.
mod proxy;

/// Re-export of the main client interface.
pub use client::RequestClient;

/// Callback and observation functionality for tracking request state changes.
mod listen;

/// Re-export of the callback trait for request state monitoring.
pub use listen::Callback;

// Import utility macros
#[macro_use]
extern crate request_utils;

// Import logging utilities
use hilog_rust::{HiLogLabel, LogType};

/// Log label for the RequestNative component.
///
/// Used for consistent logging across the request_next crate, with the domain
/// 0xD001C50 (hexadecimal) and the tag "RequestNative".
pub(crate) const LOG_LABEL: HiLogLabel = HiLogLabel {
    log_type: LogType::LogCore,
    domain: 0xD001C50,
    tag: "RequestNative",
};
