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

//! Typology definitions and implementations for notification bar content.
//! 
//! This module provides types and functions for creating notification content
//! for download and upload tasks, including progress indicators, completion
//! notifications, and group notifications.

#![allow(clippy::bool_assert_comparison)]

// Resource keys for notification strings
const DOWNLOAD_FILE: &str = "request_agent_download_file\0";        // Template for download file notification title
const DOWNLOAD_SUCCESS: &str = "request_agent_download_success\0";    // Template for download success notification title
const DOWNLOAD_FAIL: &str = "request_agent_download_fail\0";        // Template for download failure notification title
const UPLOAD_FILE: &str = "request_agent_upload_file\0";            // Template for upload file notification title
const UPLOAD_SUCCESS: &str = "request_agent_upload_success\0";        // Template for upload success notification title
const UPLOAD_FAIL: &str = "request_agent_upload_fail\0";            // Template for upload failure notification title
const TASK_COUNT: &str = "request_agent_task_count\0";            // Template for task count text (successful/failed)
const DOWNLOAD_COMPLETE: &str = "request_agent_download_complete\0"; // Template for download complete notification title

use super::database::CustomizedNotification;
use super::ffi::{GetSystemResourceString, NotifyContent, ProgressCircle};
use super::notify_flow::{GroupProgress, ProgressNotify};
use super::progress_size;
use crate::config::Action;

/// Formats progress as a percentage string with two decimal places.
/// 
/// Returns a formatted percentage string representing the current progress relative to the total.
/// 
/// # Arguments
/// 
/// * `current` - Current progress value
/// * `total` - Total progress value
/// 
/// # Returns
/// 
/// Formatted percentage string (e.g., "45.67%")
fn progress_percentage(current: u64, total: u64) -> String {
    if total == 0 {
        return "100%".to_string();
    }
    format!(
        "{}.{:02}%",
        current * 100 / total,
        current * 100 % total * 100 / total
    )
}

/// Formats progress size as a human-readable string.
/// 
/// Uses the progress_size module to convert bytes to a human-readable format.
/// 
/// # Arguments
/// 
/// * `current` - Current size in bytes
/// 
/// # Returns
/// 
/// Human-readable size string (e.g., "1.5 MB")
fn progress_size(current: u64) -> String {
    progress_size::progress_size(current)
}

impl NotifyContent {
    /// Creates a notification for a task completion event.
    /// 
    /// Generates content for a notification shown when a task completes (successfully or failed),
    /// using custom values if provided or default system values.
    /// 
    /// # Arguments
    /// 
    /// * `customized` - Optional customized notification content
    /// * `action` - Action type (download or upload)
    /// * `task_id` - ID of the task
    /// * `uid` - User ID associated with the task
    /// * `file_name` - Name of the file
    /// * `is_successful` - Whether the task completed successfully
    /// 
    /// # Returns
    /// 
    /// Configured NotifyContent object
    pub(crate) fn task_eventual_notify(
        mut customized: Option<CustomizedNotification>,
        action: Action,
        task_id: u32,
        uid: u32,
        file_name: String,
        is_successful: bool,
    ) -> Self {
        // Use custom title if provided, otherwise get system resource based on action and status
        let title = customized
            .as_mut()
            .and_then(|c| c.title.take())
            .unwrap_or_else(|| match action {
                Action::Download => {
                    if is_successful {
                        GetSystemResourceString(DOWNLOAD_SUCCESS)
                    } else {
                        GetSystemResourceString(DOWNLOAD_FAIL)
                    }
                }
                Action::Upload => {
                    if is_successful {
                        GetSystemResourceString(UPLOAD_SUCCESS)
                    } else {
                        GetSystemResourceString(UPLOAD_FAIL)
                    }
                }
                _ => unreachable!(),
            });
        
        // Use custom text if provided, otherwise use file name
        let text = customized.as_mut().and_then(|c| c.text.take()).unwrap_or(file_name);
        let want_agent = customized.and_then(|c| c.want_agent).unwrap_or_default();

        Self {
            title,
            text,
            want_agent,
            request_id: task_id,
            uid,
            live_view: false,       // Not a live updating notification
            progress_circle: ProgressCircle::close(),
            x_mark: false,          // No close button needed for completed task
        }
    }

    /// Creates a notification for a task progress update.
    /// 
    /// Generates content for a progress notification that shows current status,
    /// with a progress indicator and relevant details.
    /// 
    /// # Arguments
    /// 
    /// * `customized` - Optional customized notification content
    /// * `info` - Progress information for the task
    /// 
    /// # Returns
    /// 
    /// Configured NotifyContent object with progress information
    pub(crate) fn task_progress_notify(
        mut customized: Option<CustomizedNotification>,
        info: &ProgressNotify,
    ) -> Self {
        // Generate title based on action type and progress information
        let title = customized
            .as_mut()
            .and_then(|c| c.title.take())
            .unwrap_or_else(|| match info.action {
                Action::Download => {
                    let title = GetSystemResourceString(DOWNLOAD_FILE);
                    match info.total {
                        Some(total) => {
                            title.replace("%s", &progress_percentage(info.processed, total))
                        }
                        None => title.replace("%s", &progress_size(info.processed)),
                    }
                }
                Action::Upload => {
                    let title = GetSystemResourceString(UPLOAD_FILE);
                    if let Some((current_count, total_count)) = info.multi_upload {
                        title.replace("%s", &format!("{}/{}", current_count, total_count))
                    } else {
                        match info.total {
                            Some(total) => {
                                title.replace("%s", &progress_percentage(info.processed, total))
                            }
                            None => title.replace("%s", &progress_size(info.processed)),
                        }
                    }
                }
                _ => unreachable!(),
            });

        // Use custom text if provided, otherwise use file name
        let text = customized.as_mut()
            .and_then(|c| c.text.clone())
            .unwrap_or_else(|| info.file_name.clone());
        
        let want_agent = customized.and_then(|c| c.want_agent).unwrap_or_default();
        
        // Create progress circle if total size is known
        let progress_circle = match info.total {
            Some(total) => ProgressCircle::open(info.processed, total),
            None => ProgressCircle::close(),
        };

        Self {
            title,
            text,
            want_agent,
            request_id: info.task_id,
            uid: info.uid as u32,
            live_view: true,
            progress_circle,
            x_mark: true,
        }
    }

    /// Creates a notification for a group of completed tasks.
    /// 
    /// Generates content for a notification summarizing the results of multiple tasks,
    /// showing successful and failed counts along with total size.
    /// 
    /// # Arguments
    /// 
    /// * `customized` - Optional customized notification content
    /// * `action` - Action type (download or upload)
    /// * `group_id` - ID of the notification group
    /// * `uid` - User ID associated with the tasks
    /// * `current_size` - Total size processed in the group
    /// * `successful_count` - Number of successfully completed tasks
    /// * `failed_count` - Number of failed tasks
    /// 
    /// # Returns
    /// 
    /// Configured NotifyContent object
    pub(crate) fn group_eventual_notify(
        mut customized: Option<CustomizedNotification>,
        action: Action,
        group_id: u32,
        uid: u32,
        current_size: u64,
        successful_count: i32,
        failed_count: i32,
    ) -> Self {
        // Generate download completion message with formatted size
        let text_download_complete = GetSystemResourceString(DOWNLOAD_COMPLETE);
        let text_download = text_download_complete.replace("%s", &progress_size(current_size).to_string());
        
        // Use custom title if provided, otherwise generate based on action
        let title = customized
            .as_mut()
            .and_then(|c| c.title.take())
            .unwrap_or_else(|| match action {
                Action::Download => text_download,
                Action::Upload => format!("上传完成 {}", progress_size(current_size)),
                _ => unreachable!(),
            });

        // Format task count text with successful and failed task numbers
        // Handle different format patterns (%d or %1$d/%2$d)
        let text_task_count = GetSystemResourceString(TASK_COUNT);
        let text_count = if text_task_count.contains("%d") {
            text_task_count
                .replacen("%d", &successful_count.to_string(), 1)
                .replacen("%d", &failed_count.to_string(), 1)
        } else {
            text_task_count
                .replace("%1$d", &successful_count.to_string())
                .replace("%2$d", &failed_count.to_string())
        };

        let text = customized.as_mut().and_then(|c| c.text.take()).unwrap_or(text_count);
        let want_agent = customized.and_then(|c| c.want_agent).unwrap_or_default();

        Self {
            title,
            text,
            want_agent,
            request_id: group_id,
            uid,
            live_view: false,
            progress_circle: ProgressCircle::close(),
            x_mark: false,
        }
    }

    /// Creates a notification for a group of tasks in progress.
    /// 
    /// Generates content for a notification showing the combined progress of multiple tasks,
    /// with counts of successful and failed tasks.
    /// 
    /// # Arguments
    /// 
    /// * `customized` - Optional customized notification content
    /// * `action` - Action type (download or upload)
    /// * `group_id` - ID of the notification group
    /// * `uid` - User ID associated with the tasks
    /// * `group_progress` - Progress information for the group of tasks
    /// 
    /// # Returns
    /// 
    /// Configured NotifyContent object with group progress information
    pub(crate) fn group_progress_notify(
        mut customized: Option<CustomizedNotification>,
        action: Action,
        group_id: u32,
        uid: u32,
        group_progress: &GroupProgress,
    ) -> Self {
        let title = customized
            .as_mut()
            .and_then(|c| c.title.take())
            .unwrap_or_else(|| match action {
                Action::Download => {
                    let title = GetSystemResourceString(DOWNLOAD_FILE);
                    title.replace("%s", &progress_size(group_progress.processed()))
                }
                Action::Upload => {
                    let title = GetSystemResourceString(UPLOAD_FILE);
                    title.replace("%s", &progress_size(group_progress.processed()))
                }
                _ => unreachable!(),
            });

        // Get current successful and failed task counts from group progress
        let (successful, failed) = (group_progress.successful(), group_progress.failed());
        
        // Format task count text with successful and failed task numbers
        // Handle different format patterns (%d or %1$d/%2$d)
        let text_task_count = GetSystemResourceString(TASK_COUNT);
        let text_count = if text_task_count.contains("%d") {
            text_task_count
                .replacen("%d", &successful.to_string(), 1)
                .replacen("%d", &failed.to_string(), 1)
        } else {
            text_task_count
                .replace("%1$d", &successful.to_string())
                .replace("%2$d", &failed.to_string())
        };

        let text = customized.as_mut().and_then(|c| c.text.take()).unwrap_or(text_count);
        let want_agent = customized.and_then(|c| c.want_agent).unwrap_or_default();

        let progress_circle =
            ProgressCircle::open((successful + failed) as u64, group_progress.total() as u64);
        Self {
            title,
            text,
            want_agent,
            request_id: group_id,
            uid,
            live_view: true,
            progress_circle,
            x_mark: false,
        }
    }
}

/// Represents a progress circle indicator for notifications.
/// 
/// Controls whether a progress indicator is shown and, if so, its current and total values.
impl ProgressCircle {
    /// Creates a closed (hidden) progress circle.
    /// 
    /// # Returns
    /// 
    /// ProgressCircle configured to be hidden
    pub(crate) fn close() -> Self {
        Self {
            open: false,
            current: 0,
            total: 0,
        }
    }
    
    /// Creates an open (visible) progress circle with specified values.
    /// 
    /// Automatically scales down large values to fit within i32 range if needed.
    /// 
    /// # Arguments
    /// 
    /// * `current` - Current progress value
    /// * `total` - Total progress value
    /// 
    /// # Returns
    /// 
    /// ProgressCircle configured with the provided values
    pub(crate) fn open(mut current: u64, mut total: u64) -> Self {
        // Scale down values if they exceed i32::MAX to prevent overflow
        while total > i32::MAX as u64 {
            total >>= 1;
            current >>= 1;
        }
        Self {
            open: true,
            current,
            total,
        }
    }
}

#[cfg(test)]
mod ut_typology {
    include!("../../../tests/ut/service/notification_bar/ut_typology.rs");
}
