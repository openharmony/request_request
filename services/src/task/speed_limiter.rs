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

//! Speed limiting implementation for network operations.
//! 
//! This module provides a `SpeedLimiter` struct that can be used to control the rate
//! of data transfer operations, ensuring they don't exceed specified speed limits.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use ylong_http_client::HttpClientError;
use ylong_runtime::time::{sleep, Sleep};

/// Controls the rate of data transfer operations.
/// 
/// This struct implements a token bucket-like algorithm to limit the speed of data transfers.
#[derive(Default)]
pub(crate) struct SpeedLimiter {
    /// Timestamp of the last speed check in milliseconds.
    pub(crate) last_time: u64,
    
    /// Amount of data transferred at the last check in bytes.
    pub(crate) last_size: u64,
    
    /// Maximum allowed transfer rate in bytes per second.
    pub(crate) speed_limit: u64,
    
    /// Optional future for sleep operations when rate limiting is active.
    pub(crate) sleep: Option<Pin<Box<Sleep>>>,
}

impl SpeedLimiter {
    /// Updates the speed limit and resets internal state if changed.
    /// 
    /// # Arguments
    /// 
    /// * `speed_limit` - New speed limit in bytes per second. A value of 0 disables limiting.
    pub(crate) fn update_speed_limit(&mut self, speed_limit: u64) {
        if self.speed_limit != speed_limit {
            // Reset state when limit changes to ensure accurate speed measurement
            self.last_size = 0;
            self.last_time = 0;
            self.sleep = None;
            self.speed_limit = speed_limit;
        }
    }

    /// Checks if the transfer rate exceeds the limit and applies throttling if needed.
    /// 
    /// This method implements a polling interface to integrate with asynchronous operations.
    /// It calculates the current transfer speed and returns `Poll::Pending` if throttling is
    /// required, causing the executor to wait until the speed is back within limits.
    /// 
    /// # Arguments
    /// 
    /// * `cx` - The task context for registering wakeups.
    /// * `current_time` - Current timestamp in milliseconds.
    /// * `current_size` - Total number of bytes transferred so far.
    /// 
    /// # Returns
    /// 
    /// * `Poll::Ready(Ok(()))` - When the operation can proceed without throttling.
    /// * `Poll::Pending` - When the transfer rate exceeds the limit and the operation should wait.
    pub(crate) fn poll_check_limit(
        &mut self,
        cx: &mut Context<'_>,
        current_time: u64,
        current_size: u64,
    ) -> Poll<Result<(), HttpClientError>> {
        // Interval for speed measurement in milliseconds
        const SPEED_LIMIT_INTERVAL: u64 = 1000;
        
        self.sleep = None;
        if self.speed_limit != 0 {
            if self.last_time == 0 || current_time - self.last_time >= SPEED_LIMIT_INTERVAL {
                // Initialize or reset measurement period
                self.last_time = current_time;
                self.last_size = current_size;
            } else if current_time - self.last_time < SPEED_LIMIT_INTERVAL
                && ((current_size - self.last_size) >= self.speed_limit)
            {
                // Calculate required sleep time to maintain speed limit
                let limit_time = (current_size - self.last_size) * SPEED_LIMIT_INTERVAL
                    / self.speed_limit
                    - (current_time - self.last_time);
                self.sleep = Some(Box::pin(sleep(Duration::from_millis(limit_time))));
            }
        }

        // Check if we need to wait for the sleep future
        if let Some(sleep) = self.sleep.as_mut() {
            if Pin::new(sleep).poll(cx).is_pending() {
                return Poll::Pending;
            }
        }
        Poll::Ready(Ok(()))
    }
}
