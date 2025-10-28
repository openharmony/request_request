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

//! Core caching functionality for the request system.
//! 
//! This crate provides core caching mechanisms including RAM-based caching,
//! file system caching, and cache management utilities. It implements thread-safe
//! cache operations and efficient memory management for downloaded content.

#![deny(unused_must_use)]
#![allow(
    unknown_lints,
    static_mut_refs,
    stable_features,
    missing_docs,
    clippy::new_without_default
)]
#![feature(lazy_cell)]

#[macro_use]
extern crate request_utils;

mod data;
mod manage;
mod update;

pub mod observe;

/// In-memory cache implementation for task data.
pub use data::RamCache;

/// Central manager for cache operations and resources.
pub use manage::CacheManager;

/// Handles cache updates and synchronization operations.
pub use update::Updater;

// Conditional compilation for OHOS platform
cfg_ohos! {
    mod wrapper;
    // Use ffrt_spawn for thread spawning on OHOS
    use ffrt_rs::ffrt_spawn as spawn;
}

// Conditional compilation for non-OHOS platforms
cfg_not_ohos! {
    // Use spawn_blocking for thread spawning on other platforms
    use ylong_runtime::spawn_blocking as spawn;
}

use hilog_rust::{HiLogLabel, LogType};

/// Log label for the cache_core module.
/// 
/// Used for consistent logging across the caching system with the PreloadNative tag.
pub(crate) const LOG_LABEL: HiLogLabel = HiLogLabel {
    log_type: LogType::LogCore,
    domain: 0xD001C50,
    tag: "PreloadNative",
};
