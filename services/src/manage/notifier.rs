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

//! Notification system for task state changes and events.
//! 
//! This module provides a central notification system for broadcasting various task-related
//! events to clients and, on OpenHarmony platforms, to the system event infrastructure.

use crate::info::State;
use crate::service::client::ClientManagerEntry;
use crate::task::notify::{NotifyData, SubscribeType, WaitingCause};
use crate::task::reason::Reason;
/// Central notification dispatcher for task events.
/// 
/// Provides methods for sending various types of task-related notifications to clients
/// and publishing system events when running on OpenHarmony platforms.
pub(crate) struct Notifier;

impl Notifier {
    /// Sends a completion notification for a task.
    /// 
    /// Notifies clients that a task has completed successfully.
    /// On OpenHarmony platforms, also publishes a system event.
    /// 
    /// # Arguments
    /// 
    /// * `client_manager` - The client manager used to dispatch the notification
    /// * `notify_data` - The notification data containing task information
    pub(crate) fn complete(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        #[cfg(feature = "oh")]
        let _ = publish_state_change_event(
            notify_data.bundle.as_str(),
            notify_data.task_id,
            State::Completed.repr as i32,
            notify_data.uid,
        );
        client_manager.send_notify_data(SubscribeType::Complete, notify_data)
    }

    /// Sends a failure notification for a task.
    /// 
    /// Notifies clients that a task has failed.
    /// On OpenHarmony platforms, also publishes a system event.
    /// 
    /// # Arguments
    /// 
    /// * `client_manager` - The client manager used to dispatch the notification
    /// * `notify_data` - The notification data containing task information
    pub(crate) fn fail(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        #[cfg(feature = "oh")]
        let _ = publish_state_change_event(
            notify_data.bundle.as_str(),
            notify_data.task_id,
            State::Failed.repr as i32,
            notify_data.uid,
        );
        client_manager.send_notify_data(SubscribeType::Fail, notify_data)
    }

    /// Sends a fault notification for a task.
    /// 
    /// Notifies clients that a fault has occurred with a task.
    /// 
    /// # Arguments
    /// 
    /// * `tid` - The thread ID associated with the fault
    /// * `client_manager` - The client manager used to dispatch the notification
    /// * `reason` - The reason for the fault
    pub(crate) fn faults(tid: u32, client_manager: &ClientManagerEntry, reason: Reason) {
        client_manager.send_faults(tid, SubscribeType::FaultOccur, reason)
    }

    /// Sends a pause notification for a task.
    /// 
    /// Notifies clients that a task has been paused.
    /// 
    /// # Arguments
    /// 
    /// * `client_manager` - The client manager used to dispatch the notification
    /// * `notify_data` - The notification data containing task information
    pub(crate) fn pause(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        client_manager.send_notify_data(SubscribeType::Pause, notify_data)
    }

    /// Sends a resume notification for a task.
    /// 
    /// Notifies clients that a task has been resumed.
    /// 
    /// # Arguments
    /// 
    /// * `client_manager` - The client manager used to dispatch the notification
    /// * `notify_data` - The notification data containing task information
    pub(crate) fn resume(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        client_manager.send_notify_data(SubscribeType::Resume, notify_data)
    }

    /// Sends a header receive notification for a task.
    /// 
    /// Notifies clients that HTTP headers have been received for a task.
    /// 
    /// # Arguments
    /// 
    /// * `client_manager` - The client manager used to dispatch the notification
    /// * `notify_data` - The notification data containing task information
    pub(crate) fn header_receive(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        client_manager.send_notify_data(SubscribeType::HeaderReceive, notify_data)
    }

    /// Sends a progress notification for a task.
    /// 
    /// Notifies clients about the current progress of a task.
    /// Skips notification if total processed bytes is zero and file size is negative,
    /// which indicates an invalid state.
    /// 
    /// # Arguments
    /// 
    /// * `client_manager` - The client manager used to dispatch the notification
    /// * `notify_data` - The notification data containing progress information
    pub(crate) fn progress(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        let total_processed = notify_data.progress.common_data.total_processed;
        let file_total_size: i64 = notify_data.progress.sizes.iter().sum();
        // Skip notification for invalid progress states
        if total_processed == 0 && file_total_size < 0 {
            return;
        }
        client_manager.send_notify_data(SubscribeType::Progress, notify_data)
    }

    /// Sends a removal notification for a task.
    /// 
    /// Notifies clients that a task has been removed and marks the task as finished.
    /// 
    /// # Arguments
    /// 
    /// * `client_manager` - The client manager used to dispatch the notification
    /// * `notify_data` - The notification data containing task information
    pub(crate) fn remove(client_manager: &ClientManagerEntry, notify_data: NotifyData) {
        let task_id = notify_data.task_id;
        client_manager.send_notify_data(SubscribeType::Remove, notify_data);
        client_manager.notify_task_finished(task_id);
    }

    /// Sends a waiting notification for a task.
    /// 
    /// Notifies clients that a task is waiting due to a specific cause.
    /// 
    /// # Arguments
    /// 
    /// * `client_manager` - The client manager used to dispatch the notification
    /// * `task_id` - The ID of the task that is waiting
    /// * `cause` - The reason why the task is waiting
    pub(crate) fn waiting(client_manager: &ClientManagerEntry, task_id: u32, cause: WaitingCause) {
        client_manager.send_wait_reason(task_id, cause);
    }
}

#[cfg(feature = "oh")]
/// Publishes a task state change event to the system.
/// 
/// On OpenHarmony platforms, this function sends a system-wide event about a task's state change.
/// 
/// # Arguments
/// 
/// * `bundle_name` - The name of the application bundle associated with the task
/// * `task_id` - The ID of the task whose state has changed
/// * `state` - The new state of the task as an integer representation
/// * `uid` - The user ID associated with the task
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the event was successfully published, or `Err(())` if it failed.
pub(crate) fn publish_state_change_event(
    bundle_name: &str,
    task_id: u32,
    state: i32,
    uid: u64,
) -> Result<(), ()> {
    match crate::utils::PublishStateChangeEvent(bundle_name, task_id, state, uid as i32) {
        true => Ok(()),
        false => Err(()),
    }
}
#[allow(unused)]
#[cfg(test)]
// Unit tests for the notifier module
mod ut_notifier {
    include!("../../tests/ut/manage/ut_notifier.rs");
}
