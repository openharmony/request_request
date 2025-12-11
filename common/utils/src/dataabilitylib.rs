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

// Conditional compilation for OHOS platform
// Provides HarmonyOS-specific logging and utilities
cfg_ohos! {
    pub mod dataability;
}

/// Testing utilities.
pub mod test;
