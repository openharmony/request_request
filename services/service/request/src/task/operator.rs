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

use std::pin::Pin;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::task::{Context, Poll};

use ylong_http_client::HttpClientError;
use ylong_runtime::io::AsyncWrite;

#[cfg(feature = "oh")]
use crate::manager::Notifier;
use crate::task::config::Version;
use crate::task::info::State;
use crate::task::RequestTask;
use crate::utils::get_current_timestamp;

const FRONT_NOTIFY_INTERVAL: u64 = 1000;
const BACKGROUND_NOTIFY_INTERVAL: u64 = 3000;

pub(crate) struct TaskOperator {
    pub(crate) task: Arc<RequestTask>,
    pub(crate) check_point: Option<()>,
    pub(crate) tick_waiting: usize,
}

impl TaskOperator {
    pub(crate) fn new(task: Arc<RequestTask>) -> Self {
        Self {
            task,
            check_point: None,
            tick_waiting: 0,
        }
    }

    pub(crate) fn poll_progress_common(
        &self,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), HttpClientError>> {
        let current = get_current_timestamp();

        let state = self.task.status.lock().unwrap().state;
        if (state != State::Running && state != State::Retrying)
            || (self.task.conf.version == Version::API10 && !self.task.check_net_work_status())
        {
            debug!("pause the task");
            return Poll::Ready(Err(HttpClientError::user_aborted()));
        }

        let version = self.task.conf.version;
        if current > self.task.last_notify.load(Ordering::SeqCst) + FRONT_NOTIFY_INTERVAL {
            let notify_data = self.task.build_notify_data();
            self.task.last_notify.store(current, Ordering::SeqCst);

            #[cfg(feature = "oh")]
            Notifier::service_front_notify("progress".into(), notify_data, &self.task.app_state);
        }

        let gauge = self.task.conf.common_data.gauge;
        if version == Version::API9 || gauge {
            let last_background_notify_time =
                self.task.background_notify_time.load(Ordering::SeqCst);
            if get_current_timestamp() - last_background_notify_time >= BACKGROUND_NOTIFY_INTERVAL {
                self.task.background_notify();
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
        let file = unsafe { &mut *self.task.files.0.get() }.get_mut(0).unwrap();
        let mut progress_guard = self.task.progress.lock().unwrap();
        match Pin::new(file).poll_write(cx, data) {
            Poll::Ready(Ok(size)) => {
                progress_guard.processed[0] += size;
                progress_guard.common_data.total_processed += size;
                Poll::Ready(Ok(size + skip_size))
            }
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(HttpClientError::other(Some(e)))),
        }
    }
}
