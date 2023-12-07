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

use ylong_http_client::async_impl::{DownloadOperator, Downloader};
use ylong_http_client::{HttpClientError, Response, SpeedLimit, Timeout};

use super::operator::TaskOperator;
use super::reason::Reason;
use super::tick::Clock;
use crate::task::info::State;
use crate::task::RequestTask;

cfg_oh! {
    use crate::trace::Trace;
}

const SECONDS_IN_ONE_WEEK: u64 = 7 * 24 * 60 * 60;

const LOW_SPEED_TIME: u64 = 60;
const LOW_SPEED_LIMIT: u64 = 1;

impl DownloadOperator for TaskOperator {
    fn poll_download(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        data: &[u8],
    ) -> Poll<Result<usize, HttpClientError>> {
        let me = self.get_mut();

        // Repeated queue entry can affect performance, pay attention.
        // need more test and research.

        if me.task.rate_limiting.load(Ordering::Relaxed) {
            if me.check_point.take().is_none() {
                Clock::get_instance().register(cx);
                me.check_point = Some(());
                return Poll::Pending;
            }
        } else {
            me.tick_waiting += 1;
            if me.tick_waiting == 10 {
                me.tick_waiting = 0;
                Clock::get_instance().tick();
            }
        }

        if me.task.range_request.load(Ordering::SeqCst) {
            if me.task.range_response.load(Ordering::SeqCst) {
                return me.poll_write_file(cx, data, 0);
            }
            // write partial response data
            let begins = me.task.conf.common_data.begins;
            let ends = me.task.conf.common_data.ends;
            return me.poll_write_partial_file(cx, data, begins, ends);
        }
        me.poll_write_file(cx, data, 0)
    }

    fn poll_progress(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        _downloaded: u64,
        _total: Option<u64>,
    ) -> Poll<Result<(), HttpClientError>> {
        self.poll_progress_common(cx)
    }
}

pub(crate) fn build_downloader(
    task: Arc<RequestTask>,
    response: Response,
) -> Downloader<TaskOperator> {
    let task_operator = TaskOperator::new(task);

    Downloader::builder()
        .body(response)
        .operator(task_operator)
        .timeout(Timeout::from_secs(SECONDS_IN_ONE_WEEK))
        .speed_limit(SpeedLimit::new().min_speed(LOW_SPEED_LIMIT, LOW_SPEED_TIME))
        .build()
}

pub(crate) async fn download(task: Arc<RequestTask>) {
    download_inner(task.clone()).await;

    #[cfg(feature = "oh")]
    use hisysevent::{build_number_param, build_str_param};

    #[cfg(feature = "oh")]
    use crate::sys_event::SysEvent;
    #[cfg(feature = "oh")]
    let reason = task.code.lock().unwrap()[0];
    // If `Reason` is not `Default`a records this sys event.
    #[cfg(feature = "oh")]
    if reason != Reason::Default {
        SysEvent::task_fault()
            .param(build_str_param!(crate::sys_event::TASKS_TYPE, "DOWNLOAD"))
            .param(build_number_param!(crate::sys_event::TOTAL_FILE_NUM, 1))
            .param(build_number_param!(crate::sys_event::FAIL_FILE_NUM, 1))
            .param(build_number_param!(crate::sys_event::SUCCESS_FILE_NUM, 0))
            .param(build_number_param!(
                crate::sys_event::ERROR_INFO,
                reason as i32
            ))
            .write();
    }
}

async fn download_inner(task: Arc<RequestTask>) {
    info!("begin download");

    // Ensures `_trace` can only be freed when this function exits.
    #[cfg(feature = "oh")]
    Trace::start("download file");

    let response = match task.client.as_ref() {
        Some(client) => {
            let request = match task.build_download_request().await {
                Some(request) => request,
                None => return,
            };

            let name = task.conf.file_specs[0].path.as_str();
            let download = task.progress.lock().unwrap().processed[0];
            // Ensures `_trace` can only be freed when this function exits.
            #[cfg(feature = "oh")]
            Trace::start(&format!(
                "download file name: {name} downloaded size: {download}"
            ));
            #[cfg(feature = "oh")]
            Trace::finish();
            client.request(request).await
        }
        None => {
            return;
        }
    };

    task.record_response_header(&response);
    if !task.handle_response_error(&response).await {
        error!("response error");
        return;
    }
    let response = response.unwrap();

    if !task.get_file_info(&response) {
        return;
    }

    let mut downloader = build_downloader(task.clone(), response);

    let result = downloader.download().await;

    if !task.handle_download_error(&result) {
        error!("handle_download_error");
        return;
    }

    // Makes sure all the data has been written to the target file.
    if let Some(file) = task.files.get(0) {
        let _ = file.sync_all().await;
    }
    task.set_status(State::Completed, Reason::Default);
    #[cfg(feature = "oh")]
    Trace::finish();
}
