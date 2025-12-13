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

//! Cache download library for efficient resource preloading and management.
//! 
//! This library provides functionality for downloading and caching resources with support
//! for various HTTP client backends, progress tracking, and callback notification systems.
//! It is designed to efficiently handle resource preloading scenarios with caching capabilities.

// Allow specific lints as needed for compatibility and development
#![allow(
    unknown_lints,
    stable_features,
    missing_docs,
    clippy::new_without_default
)]

// Enable lazy initialization feature
#![feature(lazy_cell)]

// Import utility macros from request_utils
#[macro_use]
extern crate request_utils;

// Import logging macros defined in this crate
#[macro_use]
mod macros;

// Core download functionality module
mod download;

// Public modules exposing API interfaces
pub mod info;    // Download information and metrics
pub mod observe; // Observation and monitoring functionality
pub mod services; // Service interfaces and types

pub use services::{CacheDownloadService, DownloadRequest, PreloadCallback};

// Re-export downloader enum for public API use
pub use download::task::Downloader;

// Conditional compilation for OpenHarmony platform
cfg_ohos! {
    mod wrapper; // Platform-specific wrapper for OpenHarmony
    use ffrt_rs::ffrt_spawn as spawn; // Use ffrt task spawning on OpenHarmony
}

// Conditional compilation for non-OpenHarmony platforms
cfg_not_ohos! {
    use ylong_runtime::spawn_blocking as spawn; // Use ylong runtime for other platforms
}

use hilog_rust::{HiLogLabel, LogType};

/// Log label for the preload native component.
///
/// Used for consistent logging throughout the cache download library.
pub(crate) const LOG_LABEL: HiLogLabel = HiLogLabel {
    log_type: LogType::LogCore,
    domain: 0xD001C50,
    tag: "PreloadNative",
};
