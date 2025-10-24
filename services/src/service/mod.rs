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

//! Core service implementation for the request system.
//! 
//! This module provides the main service components for handling request operations,
//! including task management, permission handling, and communication interfaces.
//! It serves as the foundation for processing download and upload requests within
//! the request server.

/// Atomic counter implementation for tracking active operations.
pub(crate) mod active_counter;
/// Client interface for interacting with the request service.
pub(crate) mod client;

/// IPC interface definitions for the request server service.
pub mod interface;

// Platform-specific service components for OpenHarmony.
// 
// This section includes modules that are only available when targeting the
// OpenHarmony platform, providing platform-specific implementations for
// permission management, command handling, and notification services.
cfg_oh! {
    /// Permission management for request operations.
    pub(crate) mod permission;
    /// Command processing for request operations.
    pub(crate) mod command;
    /// Notification bar integration for request status updates.
    pub(crate) mod notification_bar;
    /// Internal service stub implementation.
    mod stub;
    /// Main service interface implementation.
    pub(crate) use stub::RequestServiceStub;
    /// Utility for serializing task information.
    pub(crate) use stub::serialize_task_info;
    /// Utility for serializing task configuration.
    pub(crate) use stub::serialize_task_config;
}

/// Running count management for tracking active requests.
pub(crate) mod run_count;
