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

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use ylong_http_client::HttpClientError;
use ylong_runtime::io::AsyncWrite;
use ylong_runtime::time::{sleep, Sleep};

use super::config::Mode;
use crate::manage::account::is_active_user;
use crate::manage::notifier::Notifier;
use crate::task::config::Version;
use crate::task::info::State;
use crate::task::reason::Reason;
use crate::task::request_task::RequestTask;
use crate::utils::get_current_timestamp;

const SPEED_LIMIT_INTERVAL: u64 = 1000;
const FRONT_NOTIFY_INTERVAL: u64 = 1000;
const BACKGROUND_NOTIFY_INTERVAL: u64 = 3000;

pub(crate) struct TaskOperator {
    pub(crate) sleep: Option<Sleep>,
    pub(crate) task: Arc<RequestTask>,
    pub(crate) last_time: u64,
    pub(crate) last_size: u64,
    pub(crate) more_sleep_time: u64,
}

impl TaskOperator {
    pub(crate) fn new(task: Arc<RequestTask>) -> Self {
        Self {
            sleep: None,
            task,
            last_time: 0,
            last_size: 0,
            more_sleep_time: 0,
        }
    }

    pub(crate) fn poll_progress_common(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), HttpClientError>> {
        let current = get_current_timestamp();

        let state = self.task.status.lock().unwrap().state;
        if (state != State::Running && state != State::Retrying)
            || (self.task.conf.version == Version::API10 && !self.task.check_net_work_status())
        {
            info!("pause the task, tid: {}", self.task.task_id());
            return Poll::Ready(Err(HttpClientError::user_aborted()));
        }
        if !self.task.check_app_state() {
            info!("pause for app state, tid: {}", self.task.task_id());
            return Poll::Ready(Err(HttpClientError::user_aborted()));
        }

        if self.task.conf.common_data.mode == Mode::BackGround && !is_active_user(self.task.uid()) {
            info!("pause for user stopped, tid: {}", self.task.task_id());
            self.task.set_status(State::Waiting, Reason::AccountStopped);
            return Poll::Ready(Err(HttpClientError::user_aborted()));
        }

        let version = self.task.conf.version;
        if current >= self.task.last_notify.load(Ordering::SeqCst) + FRONT_NOTIFY_INTERVAL {
            let notify_data = self.task.build_notify_data();
            self.task.last_notify.store(current, Ordering::SeqCst);
            Notifier::progress(&self.task.client_manager, notify_data);
        }

        let gauge = self.task.conf.common_data.gauge;
        if version == Version::API9 || gauge {
            let last_background_notify_time =
                self.task.background_notify_time.load(Ordering::SeqCst);
            if get_current_timestamp() - last_background_notify_time >= BACKGROUND_NOTIFY_INTERVAL {
                self.task.background_notify();
            }
        }

        let total_processed = self
            .task
            .progress
            .lock()
            .unwrap()
            .common_data
            .total_processed as u64;
        
        self.sleep = None;
        let speed_limit = self.task.rate_limiting.load(Ordering::SeqCst);
        if speed_limit != 0 {
            if self.more_sleep_time != 0 {
                // wake up for notify, sleep until speed limit conditions are met
                self.sleep = Some(sleep(Duration::from_millis(self.more_sleep_time)));
                self.more_sleep_time = 0;
            } else if self.last_time == 0 {
                // get the init time and size, for speed caculate
                self.last_time = current;
                self.last_size = total_processed;
            } else if current - self.last_time < SPEED_LIMIT_INTERVAL
                && ((total_processed - self.last_size) > speed_limit * SPEED_LIMIT_INTERVAL)
            {
                // sleep until notification is required or speed limit conditions are met
                let limit_time =
                    (total_processed - self.last_size) / speed_limit - (current - self.last_time);
                let notify_time = FRONT_NOTIFY_INTERVAL
                    - (current - self.task.last_notify.load(Ordering::SeqCst));
                let sleep_time = if limit_time > notify_time {
                    self.more_sleep_time = limit_time - notify_time;
                    notify_time
                } else {
                    limit_time
                };
                self.sleep = Some(sleep(Duration::from_millis(sleep_time)));
            } else if current - self.last_time >= SPEED_LIMIT_INTERVAL {
                // last caculate window has meet speed limit, update last_time and last_size,
                // for next poll's speed compare
                self.last_time = current;
                self.last_size = total_processed;
            }
        }

        if self.sleep.is_some() {
            match Pin::new(self.sleep.as_mut().unwrap()).poll(cx) {
                Poll::Ready(_) => return Poll::Ready(Ok(())),
                Poll::Pending => return Poll::Pending,
            }
        }
        Poll::Ready(Ok(()))
    }

    pub(crate) fn poll_write_partial_file(
        &self,
        cx: &mut Context<'_>,
        data: &[u8],
        begins: u64,
        ends: i64,
    ) -> Poll<Result<usize, HttpClientError>> {
        let data_size = data.len();
        let skip_size = self.task.skip_bytes.load(Ordering::SeqCst);
        if skip_size + data_size as u64 <= begins {
            self.task
                .skip_bytes
                .fetch_add(data_size as u64, Ordering::SeqCst);
            return Poll::Ready(Ok(data_size));
        }
        let remain_skip_bytes = (begins - skip_size) as usize;
        let mut data = &data[remain_skip_bytes..];
        self.task.skip_bytes.store(begins, Ordering::SeqCst);
        if ends >= 0 {
            let total_bytes = ends as u64 - begins + 1;
            let written_bytes = self.task.progress.lock().unwrap().processed[0] as u64;
            if written_bytes == total_bytes {
                return Poll::Ready(Err(HttpClientError::user_aborted()));
            }
            if data.len() as u64 + written_bytes >= total_bytes {
                let remain_bytes = (total_bytes - written_bytes) as usize;
                data = &data[..remain_bytes];
            }
        }
        self.poll_write_file(cx, data, remain_skip_bytes)
    }

    pub(crate) fn poll_write_file(
        &self,
        cx: &mut Context<'_>,
        data: &[u8],
        skip_size: usize,
    ) -> Poll<Result<usize, HttpClientError>> {
        let file = self.task.files.get_mut(0).unwrap();
        let mut progress_guard = self.task.progress.lock().unwrap();
        match Pin::new(file).poll_write(cx, data) {
            Poll::Ready(Ok(size)) => {
                progress_guard.processed[0] += size;
                progress_guard.common_data.total_processed += size;
                Poll::Ready(Ok(size + skip_size))
            }
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(HttpClientError::other(e))),
        }
    }
}
