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
mod progress_percentage;
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

    /// Metadata of a serialized want_agent, extracted on the C++ side.
    ///
    /// `valid == false` means the string could not be parsed into a WantAgent;
    /// `bundle_name` is the target Ability bundle (empty for implicit start);
    /// `is_send_common_event` distinguishes SEND_COMMON_EVENT agents, which
    /// are allowed at creation but discarded by the publish path
    /// (PublishNotification), so the event can never be triggered from the
    /// notification click.
    #[derive(Eq, PartialEq, Debug)]
    struct WantAgentMeta {
        bundle_name: String,
        is_send_common_event: bool,
        valid: bool,
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

        /// Extracts metadata (target bundleName and operation type) from a
        /// serialized want_agent string for ownership validation.
        fn GetWantAgentMeta(want_agent: &str) -> WantAgentMeta;
        
        /// Subscribes to notification bar events with the provided task manager.
        /// 
        /// # Arguments
        /// 
        /// * `task_manager` - The task manager wrapper to handle notification interactions
        fn SubscribeNotification(task_manager: Box<TaskManagerWrapper>);
    }
}

/// Pure-Rust want_agent policy (no FFI), extracted for unit testing.
///
/// A SEND_COMMON_EVENT agent is not rejected at task creation: the caller
/// gets no error and the task completes normally. The defense lives on the
/// publish path — notification_bar.cpp drops the agent before attaching it
/// to the NotificationRequest, so a click on the notification can never
/// publish the common event (the agent identity is re-established inside
/// the service process at publish time, uid 3815, and common events must
/// not be reachable through that identity). Skipping the bundle-ownership
/// check for these agents guarantees creation never fails on them either.
///
/// `valid == false` keeps the existing lenient behavior for unparseable
/// strings (same as an empty bundle today); for other agent types the
/// bundle-ownership rule of `check_bundle_ownership` applies.
///
/// # Returns
///
/// `true` if the caller may set this want_agent, `false` to reject.
#[cfg(feature = "oh")]
fn check_want_agent_meta(meta: &ffi::WantAgentMeta, caller_bundle: &str) -> bool {
    if !meta.valid || meta.is_send_common_event {
        return true;
    }
    check_bundle_ownership(&meta.bundle_name, caller_bundle)
}

/// Validates that a want_agent's target Ability bundle belongs to the caller.
///
/// System API callers (`is_system_api`) are allowed to set any want_agent
/// (they are trusted). Non-system callers must own the target bundle: the
/// Want's bundleName (extracted via `GetWantAgentMeta`) must match the
/// caller's own bundle (`caller_bundle`, already resolved by the caller via
/// `query_calling_bundle` or `TaskConfig.bundle`). SEND_COMMON_EVENT agents
/// are allowed at creation (no error is reported) but the publish path
/// discards them (see `check_want_agent_meta` and the fallback in
/// `notification_bar.cpp`), so a common-event action can never be triggered
/// through the request service's SA identity at notification-trigger time.
/// The entry-side ownership check blocks proxying a want_agent that targets
/// another app's Ability.
///
/// # Returns
///
/// `true` if the caller may set this want_agent, `false` to reject.
#[cfg(feature = "oh")]
pub(crate) fn validate_want_agent_ownership(
    want_agent: &str,
    caller_bundle: &str,
    is_system_api: bool,
) -> bool {
    if want_agent.is_empty() {
        debug!("want_agent allowed: empty want_agent");
        return true;
    }
    let meta = ffi::GetWantAgentMeta(want_agent);
    debug!(
        "want_agent target: {}, send_common_event: {}, valid: {}, caller: {}",
        meta.bundle_name, meta.is_send_common_event, meta.valid, caller_bundle
    );
    if is_system_api {
        debug!("want_agent allowed: system api caller");
        return true;
    }
    check_want_agent_meta(&meta, caller_bundle)
}

/// Pure-Rust bundle ownership check (no FFI), extracted for unit testing.
///
/// `target_bundle` empty → allow (no explicit target, implicit start).
/// Otherwise `target_bundle == caller_bundle` → allow, else reject.
#[cfg(feature = "oh")]
fn check_bundle_ownership(target_bundle: &str, caller_bundle: &str) -> bool {
    if target_bundle.is_empty() {
        return true;
    }
    target_bundle == caller_bundle
}

#[cfg(test)]
mod tests {
    include!("../../../tests/ut/service/notification_bar/ut_validate_want_agent.rs");
}
