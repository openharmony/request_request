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

const FRONT_NOTIFY_INTERVAL: u64 = 1000;

pub(crate) struct TaskOperator {
    pub(crate) task: Arc<RequestTask>,
    pub(crate) speed_limiter: SpeedLimiter,
    pub(crate) abort_flag: Arc<AtomicBool>,
}

impl TaskOperator {
    pub(crate) fn new(task: Arc<RequestTask>, abort_flag: Arc<AtomicBool>) -> Self {
        Self {
            task,
            speed_limiter: SpeedLimiter::default(),
            abort_flag,
        }
    }

    pub(crate) fn poll_progress_common(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), HttpClientError>> {
        if self.abort_flag.load(Ordering::Acquire) {
            return Poll::Ready(Err(HttpClientError::user_aborted()));
        }
        let current = get_current_timestamp();

        let next_notify_time = self.task.last_notify.load(Ordering::SeqCst) + FRONT_NOTIFY_INTERVAL;

        if current >= next_notify_time {
            let notify_data = self.task.build_notify_data();
            self.task.last_notify.store(current, Ordering::SeqCst);
            Notifier::progress(&self.task.client_manager, notify_data);
        }

        if self.task.background_notify.load(Ordering::Acquire)
            && current
                > self.task.background_notify_time.load(Ordering::SeqCst) + NOTIFY_PROGRESS_INTERVAL
        {
            self.task
                .background_notify_time
                .store(current, Ordering::SeqCst);
            NotificationDispatcher::get_instance().publish_progress_notification(&self.task);
        }

        let total_processed = self
            .task
            .progress
            .lock()
            .unwrap()
            .common_data
            .total_processed as u64;

        let rate_limiting = self.task.rate_limiting.load(Ordering::SeqCst);
        let max_speed = self.task.max_speed.load(Ordering::SeqCst) as u64;

        let speed_limit = match (rate_limiting, max_speed) {
            (0, max_speed) => max_speed,
            (rate_limiting, 0) => rate_limiting,
            (rate_limiting, max_speed) => min(rate_limiting, max_speed),
        };

        self.speed_limiter.update_speed_limit(speed_limit);
        self.speed_limiter
            .poll_check_limit(cx, current, total_processed)
    }

    pub(crate) fn poll_write_file(
        &self,
        _cx: &mut Context<'_>,
        data: &[u8],
        skip_size: usize,
    ) -> Poll<Result<usize, HttpClientError>> {
        let file_mutex = self.task.files.get(0).unwrap();
        let mut file = file_mutex.lock().unwrap();

        if self.abort_flag.load(Ordering::Acquire) {
            return Poll::Ready(Err(HttpClientError::user_aborted()));
        }
        match file.write(data) {
            Ok(size) => {
                let mut progress_guard = self.task.progress.lock().unwrap();
                progress_guard.processed[0] += size;
                progress_guard.common_data.total_processed += size;
                Poll::Ready(Ok(size + skip_size))
            }
            Err(e) => Poll::Ready(Err(HttpClientError::other(e))),
        }
    }
}
