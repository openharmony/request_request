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

//! Notification bar service module for download task notifications.
//! 
//! This module provides components for managing, configuring, and displaying
//! notifications for download tasks, including database management, configuration
//! handling, notification publishing, and interaction with the system notification
//! infrastructure.

mod database;
mod notification_config;
mod notify_flow;
mod progress_size;
mod publish;
mod task_handle;
mod typology;

// Re-export for internal use within the service
pub(crate) use notification_config::NotificationConfig;

/// Notification dispatcher for managing and publishing download task notifications.
/// 
/// Provides functionality for displaying, updating, and removing notifications for
/// download tasks and groups of tasks.
pub use publish::NotificationDispatcher;

/// Interval in milliseconds for updating notification progress.
/// 
/// Controls how frequently progress updates are published to the notification bar.
pub(crate) use publish::NOTIFY_PROGRESS_INTERVAL;

// Subscribe function for notification bar events (internal use)
pub(crate) use task_handle::subscribe_notification_bar;
use task_handle::TaskManagerWrapper;

// CXX bridge for FFI between Rust and C++ components
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    /// Content structure for publishing notifications to the system.
    /// 
    /// Contains all necessary information to display a download task notification,
    /// including title, text, progress, and interaction options.
    #[derive(Eq, PartialEq, Debug)]
    pub(crate) struct NotifyContent {
        title: String,
        text: String,
        want_agent: String,
        request_id: u32,
        uid: u32,
        live_view: bool,
        progress_circle: ProgressCircle,
        x_mark: bool,
    }

    /// Progress circle information for notifications.
    /// 
    /// Represents the progress visualization in notification items, showing
    /// current progress and total size information.
    #[derive(Eq, PartialEq, Debug)]
    struct ProgressCircle {
        open: bool,
        current: u64,
        total: u64,
    }

    // Rust functions exposed to C++
    extern "Rust" {
        /// Wrapper around task management functionality for notification callbacks.
        type TaskManagerWrapper;
        
        /// Attempts to pause the specified download task.
        /// 
        /// # Arguments
        /// 
        /// * `task_id` - The ID of the task to pause
        /// 
        /// # Returns
        /// 
        /// * `true` - If the task was successfully paused
        /// * `false` - If the task pause failed
        fn pause_task(self: &TaskManagerWrapper, task_id: u32) -> bool;
        
        /// Attempts to resume the specified download task.
        /// 
        /// # Arguments
        /// 
        /// * `task_id` - The ID of the task to resume
        /// 
        /// # Returns
        /// 
        /// * `true` - If the task was successfully resumed
        /// * `false` - If the task resume failed
        fn resume_task(self: &TaskManagerWrapper, task_id: u32) -> bool;
        
        /// Attempts to stop the specified download task.
        /// 
        /// # Arguments
        /// 
        /// * `task_id` - The ID of the task to stop
        /// 
        /// # Returns
        /// 
        /// * `true` - If the task was successfully stopped
        /// * `false` - If the task stop failed
        fn stop_task(self: &TaskManagerWrapper, task_id: u32) -> bool;
    }

    // C++ functions exposed to Rust
    unsafe extern "C++" {
        include!("notification_bar.h");

        /// Cancels a system notification with the specified ID.
        /// 
        /// # Arguments
        /// 
        /// * `notificationId` - The ID of the notification to cancel
        /// 
        /// # Returns
        /// 
        /// * `0` - If the notification was successfully cancelled
        /// * Error code - If the cancellation failed
        fn CancelNotification(notificationId: u32) -> i32;
        
        /// Retrieves a string from system resources by name.
        /// 
        /// # Arguments
        /// 
        /// * `name` - The name of the resource string to retrieve
        /// 
        /// # Returns
        /// 
        /// The requested system resource string
        fn GetSystemResourceString(name: &str) -> String;
        
        /// Gets the current system language setting.
        /// 
        /// # Returns
        /// 
        /// The system language code
        fn GetSystemLanguage() -> String;
        
        /// Publishes a notification to the system notification bar.
        /// 
        /// # Arguments
        /// 
        /// * `content` - The notification content to publish
        /// 
        /// # Returns
        /// 
        /// * `0` - If the notification was successfully published
        /// * Error code - If the publication failed
        fn PublishNotification(content: &NotifyContent) -> i32;
        
        /// Subscribes to notification bar events with the provided task manager.
        /// 
        /// # Arguments
        /// 
        /// * `task_manager` - The task manager wrapper to handle notification interactions
        fn SubscribeNotification(task_manager: Box<TaskManagerWrapper>);
    }
}
