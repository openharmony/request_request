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

//! Task operation utilities for the request service.
//! 
//! This module provides core functionality for managing task operations including
//! progress tracking, notifications, and file writing operations.

use std::cmp::min;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

use ylong_http_client::HttpClientError;

use crate::manage::notifier::Notifier;
use crate::service::notification_bar::{NotificationDispatcher, NOTIFY_PROGRESS_INTERVAL};
use crate::task::request_task::RequestTask;
use crate::task::speed_limiter::SpeedLimiter;
use crate::utils::get_current_timestamp;

/// Interval in milliseconds for frontend progress notifications.
const FRONT_NOTIFY_INTERVAL: u64 = 1000;

/// Task operator that handles task execution operations.
/// 
/// This struct manages the execution of download and upload tasks,
/// handling progress tracking, speed limiting, and file operations.
pub(crate) struct TaskOperator {
    /// Shared reference to the task being operated on.
    pub(crate) task: Arc<RequestTask>,
    /// Speed limiter to control data transfer rates.
    pub(crate) speed_limiter: SpeedLimiter,
    /// Flag to signal task abortion requests.
    pub(crate) abort_flag: Arc<AtomicBool>,
}

impl TaskOperator {
    /// Creates a new task operator for the given task.
    /// 
    /// # Arguments
    /// 
    /// * `task` - The task to operate on.
    /// * `abort_flag` - Flag to signal task abortion requests.
    pub(crate) fn new(task: Arc<RequestTask>, abort_flag: Arc<AtomicBool>) -> Self {
        Self {
            task,
            speed_limiter: SpeedLimiter::default(),
            abort_flag,
        }
    }

    /// Polls for common progress updates and handles notifications.
    /// 
    /// This method checks for task abortion, sends progress notifications at appropriate
    /// intervals, and applies speed limiting.
    /// 
    /// # Arguments
    /// 
    /// * `cx` - The task context for asynchronous operations.
    /// 
    /// # Returns
    /// 
    /// - `Poll::Ready(Ok(()))` if ready to continue processing.
    /// - `Poll::Pending` if the operation is blocked on speed limiting.
    /// - `Poll::Ready(Err(HttpClientError))` if the task was aborted.
    pub(crate) fn poll_progress_common(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), HttpClientError>> {
        // Check for task abortion first
        if self.abort_flag.load(Ordering::Acquire) {
            return Poll::Ready(Err(HttpClientError::user_aborted()));
        }
        
        let current = get_current_timestamp();

        // Check if it's time to send frontend notification
        let next_notify_time = self.task.last_notify.load(Ordering::SeqCst) + FRONT_NOTIFY_INTERVAL;

        if current >= next_notify_time {
            // Build and send notification data
            let notify_data = self.task.build_notify_data();
            self.task.last_notify.store(current, Ordering::SeqCst);
            Notifier::progress(&self.task.client_manager, notify_data);
        }

        // Check if background notification should be sent
        if self.task.background_notify.load(Ordering::Acquire)
            && current
                > self.task.background_notify_time.load(Ordering::SeqCst) + NOTIFY_PROGRESS_INTERVAL
        {
            self.task
                .background_notify_time
                .store(current, Ordering::SeqCst);
            NotificationDispatcher::get_instance().publish_progress_notification(&self.task);
        }

        // Apply speed limiting
        let total_processed = self
            .task
            .progress
            .lock()
            .unwrap()
            .common_data
            .total_processed as u64;

        let rate_limiting = self.task.rate_limiting.load(Ordering::SeqCst);
        let max_speed = self.task.max_speed.load(Ordering::SeqCst) as u64;

        // Determine effective speed limit based on configured values
        let speed_limit = match (rate_limiting, max_speed) {
            (0, max_speed) => max_speed,       // Only max_speed is set
            (rate_limiting, 0) => rate_limiting, // Only rate_limiting is set
            (rate_limiting, max_speed) => min(rate_limiting, max_speed), // Use the lower value
        };

        self.speed_limiter.update_speed_limit(speed_limit);
        self.speed_limiter
            .poll_check_limit(cx, current, total_processed)
    }

    /// Polls for file writing operations.
    /// 
    /// This method writes data to the first file associated with the task
    /// and updates progress tracking information.
    /// 
    /// # Arguments
    /// 
    /// * `_cx` - The task context (currently unused).
    /// * `data` - The data to write to the file.
    /// * `skip_size` - Size to add to the reported written size (for resume operations).
    /// 
    /// # Returns
    /// 
    /// - `Poll::Ready(Ok(usize))` with the total bytes written (including skip_size).
    /// - `Poll::Ready(Err(HttpClientError))` if an error occurs.
    /// 
    /// # Errors
    /// 
    /// - Returns an error if no files are associated with the task.
    /// - Returns an error if the task was aborted.
    /// - Returns an error if writing to the file fails.
    pub(crate) fn poll_write_file(
        &self,
        _cx: &mut Context<'_>,
        data: &[u8],
        skip_size: usize,
    ) -> Poll<Result<usize, HttpClientError>> {
        // Get the first file from the task
        let file_mutex = if let Some(mutex) = self.task.files.get(0) {
            mutex
        } else {
            error!("poll_write_file err, no file in the `task`");
            return Poll::Ready(Err(HttpClientError::other("error msg")));
        };
        
        let mut file = file_mutex.lock().unwrap();

        // Check for task abortion before writing
        if self.abort_flag.load(Ordering::Acquire) {
            return Poll::Ready(Err(HttpClientError::user_aborted()));
        }
        
        // Perform the write operation
        match file.write(data) {
            Ok(size) => {
                // Update progress tracking
                let mut progress_guard = self.task.progress.lock().unwrap();
                progress_guard.processed[0] += size;
                progress_guard.common_data.total_processed += size;
                Poll::Ready(Ok(size + skip_size))
            }
            Err(e) => Poll::Ready(Err(HttpClientError::other(e))),
        }
    }
}
