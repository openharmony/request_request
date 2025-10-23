// Copyright (C) 2023 Huawei Device Co., Ltd.
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

//! Task management module for the request service.
//! 
//! This module contains all components related to download and upload task handling,
//! including configuration, state management, progress tracking, and event notifications.

/// Configuration types and utilities for tasks.
pub mod config;

/// Task information structures and state management.
pub mod info;

// Internal modules for task implementation
pub(crate) mod download;     // Download task handling
pub(crate) mod files;         // File management utilities
pub(crate) mod notify;        // Notification and event handling
mod operator;                 // Task operation implementations
pub(crate) mod reason;        // Error and state reason codes
pub(crate) mod request_task;  // Core task abstraction

/// Constant representing atomic service identifier.
pub(crate) const ATOMIC_SERVICE: u32 = 1;

// Additional internal modules
pub(crate) mod bundle;          // Bundle-related utilities
pub(crate) mod client;          // Client connection management
pub(crate) mod ffi;             // Foreign function interface bindings
pub(crate) mod speed_limiter;   // Speed limiting implementation
pub(crate) mod task_control;    // Task control mechanisms
pub(crate) mod upload;          // Upload task handling
