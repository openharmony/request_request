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

//! Network observation functionality for cache download operations.
//! 
//! This module provides observers for monitoring network state changes and
//! triggering appropriate actions in the cache download system.

// Import network observation trait from request_utils
use request_utils::observe::network;

// Import the cache download service to manage tasks when network state changes
use crate::services::CacheDownloadService;

/// Network state observer for cache download operations.
///
/// Implements the `network::Observer` trait to respond to network availability changes
/// and coordinate task management in the cache download service.
pub(crate) struct NetObserver;

impl network::Observer for NetObserver {
    /// Handles notification when network connectivity becomes available.
    ///
    /// When a network connection is established, this method logs the event
    /// and triggers a reset of all download tasks in the cache download service.
    ///
    /// # Parameters
    /// - `net_id`: Identifier of the newly available network
    fn net_available(&self, net_id: i32) {
        info!("net available, net_id: {}", net_id);
        // Reset all tasks when network becomes available to resume paused downloads
        CacheDownloadService::get_instance().reset_all_tasks();
    }
}
