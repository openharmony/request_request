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

use std::io::SeekFrom;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Instant;

use ylong_http_client::async_impl::{DownloadOperator, Downloader, Response};
use ylong_http_client::{ErrorKind, HttpClientError, SpeedLimit, Timeout};

use super::operator::TaskOperator;
use super::reason::Reason;
use super::request_task::{TaskError, TaskPhase};
use crate::manage::database::RequestDb;
use crate::task::info::State;
use crate::task::request_task::RequestTask;
use crate::task::task_control;
#[cfg(feature = "oh")]
use crate::trace::Trace;
use crate::utils::get_current_duration;

const SECONDS_IN_ONE_WEEK: u64 = 7 * 24 * 60 * 60;

const LOW_SPEED_TIME: u64 = 60;
const LOW_SPEED_LIMIT: u64 = 1;

impl DownloadOperator for TaskOperator {
    fn poll_download(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        data: &[u8],
    ) -> Poll<Result<usize, HttpClientError>> {
        self.poll_write_file(cx, data, 0)
    }

    fn poll_progress(
        mut self: Pin<&mut Self>,
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
    abort_flag: Arc<AtomicBool>,
) -> Downloader<TaskOperator> {
    let task_operator = TaskOperator::new(task, abort_flag);

    Downloader::builder()
        .body(response)
        .operator(task_operator)
        .timeout(Timeout::from_secs(SECONDS_IN_ONE_WEEK))
        .speed_limit(SpeedLimit::new().min_speed(LOW_SPEED_LIMIT, LOW_SPEED_TIME))
        .build()
}

pub(crate) async fn download(task: Arc<RequestTask>, abort_flag: Arc<AtomicBool>) {
    task.tries.store(0, Ordering::SeqCst);
    loop {
        let begin_time = Instant::now();
        if let Err(e) = download_inner(task.clone(), abort_flag.clone()).await {
            match e {
                TaskError::Waiting(phase) => match phase {
                    TaskPhase::NeedRetry => {
                        let download_time = begin_time.elapsed().as_secs();
                        task.rest_time.fetch_sub(download_time, Ordering::SeqCst);
                        let mut client = task.client.lock().await;
                        client.total_timeout(Timeout::from_secs(
                            task.rest_time.load(Ordering::SeqCst),
                        ));
                        continue;
                    }
                    TaskPhase::UserAbort => {}
                    TaskPhase::NetworkOffline => {
                        *task.running_result.lock().unwrap() = Some(Err(Reason::NetworkOffline));
                    }
                },
                TaskError::Failed(reason) => {
                    *task.running_result.lock().unwrap() = Some(Err(reason));
                }
            }
        } else {
            *task.running_result.lock().unwrap() = Some(Ok(()));
        }
        break;
    }
}

impl RequestTask {
    async fn prepare_download(&self) -> Result<(), TaskError> {
        if let Some(file) = self.files.get(0) {
            task_control::file_seek(file.clone(), SeekFrom::End(0)).await?;
            let downloaded = task_control::file_metadata(file).await?.len() as usize;

            let mut progress = self.progress.lock().unwrap();
            progress.common_data.index = 0;
            progress.common_data.total_processed = downloaded;
            progress.common_data.state = State::Running.repr;
            progress.processed = vec![downloaded];
        } else {
            error!("prepare_download err, no file in the task");
            return Err(TaskError::Failed(Reason::OthersError));
        }
        Ok(())
    }
}

pub(crate) async fn download_inner(
    task: Arc<RequestTask>,
    abort_flag: Arc<AtomicBool>,
) -> Result<(), TaskError> {
    // Ensures `_trace` can only be freed when this function exits.
    #[cfg(feature = "oh")]
    let _trace = Trace::new("download file");

    task.prepare_download().await?;

    info!("{} downloading", task.task_id());

    let request = RequestTask::build_download_request(task.clone()).await?;
    let start_time = get_current_duration().as_secs() as u64;

    task.start_time.store(start_time as u64, Ordering::SeqCst);
    let client = task.client.lock().await;
    let response = client.request(request).await;
    match response.as_ref() {
        Ok(response) => {
            let status_code = response.status();
            #[cfg(feature = "oh")]
            task.notify_response(response);
            info!(
                "{} response {}",
                task.conf.common_data.task_id, status_code
            );
            if status_code.is_server_error()
                || (status_code.as_u16() != 408 && status_code.is_client_error())
                || status_code.is_redirection()
            {
                return Err(TaskError::Failed(Reason::ProtocolError));
            }
            if status_code.as_u16() == 408 {
                if task.timeout_tries.load(Ordering::SeqCst) < 2 {
                    task.timeout_tries.fetch_add(1, Ordering::SeqCst);
                    return Err(TaskError::Waiting(TaskPhase::NeedRetry));
                } else {
                    return Err(TaskError::Failed(Reason::ProtocolError));
                }
            } else {
                task.timeout_tries.store(0, Ordering::SeqCst);
            }
            if status_code.as_u16() == 200 {
                if task.require_range() {
                    info!("task {} server not support range", task.task_id());
                    return Err(TaskError::Failed(Reason::UnsupportedRangeRequest));
                }
                if let Some(file) = task.files.get(0) {
                    let has_downloaded = task_control::file_metadata(file).await?.len() > 0;
                    if has_downloaded {
                        error!("task {} file not cleared", task.task_id());
                        sys_event!(
                            ExecFault,
                            DfxCode::TASK_FAULT_09,
                            &format!("task {} file not cleared", task.task_id())
                        );
                        task_control::clear_downloaded_file(task.clone()).await?;
                    }
                } else {
                    error!("download_inner err, no file in the `task`");
                    return Err(TaskError::Failed(Reason::OthersError));
                }
            }
        }
        Err(e) => {
            error!("Task {} {:?}", task.task_id(), e);

            match e.error_kind() {
                ErrorKind::Timeout => {
                    sys_event!(
                        ExecFault,
                        DfxCode::TASK_FAULT_01,
                        &format!("Task {} {:?}", task.task_id(), e)
                    );
                    return Err(TaskError::Failed(Reason::ContinuousTaskTimeout));
                }
                ErrorKind::Request => {
                    sys_event!(
                        ExecFault,
                        DfxCode::TASK_FAULT_02,
                        &format!("Task {} {:?}", task.task_id(), e)
                    );
                    return Err(TaskError::Failed(Reason::RequestError));
                }
                ErrorKind::Redirect => {
                    sys_event!(
                        ExecFault,
                        DfxCode::TASK_FAULT_08,
                        &format!("Task {} {:?}", task.task_id(), e)
                    );
                    return Err(TaskError::Failed(Reason::RedirectError));
                }
                ErrorKind::Connect | ErrorKind::ConnectionUpgrade => {
                    task.network_retry().await?;
                    if e.is_dns_error() {
                        sys_event!(
                            ExecFault,
                            DfxCode::TASK_FAULT_05,
                            &format!("Task {} {:?}", task.task_id(), e)
                        );
                        return Err(TaskError::Failed(Reason::Dns));
                    } else if e.is_tls_error() {
                        sys_event!(
                            ExecFault,
                            DfxCode::TASK_FAULT_07,
                            &format!("Task {} {:?}", task.task_id(), e)
                        );
                        return Err(TaskError::Failed(Reason::Ssl));
                    } else {
                        sys_event!(
                            ExecFault,
                            DfxCode::TASK_FAULT_06,
                            &format!("Task {} {:?}", task.task_id(), e)
                        );
                        return Err(TaskError::Failed(Reason::Tcp));
                    }
                }
                ErrorKind::BodyTransfer => {
                    task.network_retry().await?;
                    sys_event!(
                        ExecFault,
                        DfxCode::TASK_FAULT_09,
                        &format!("Task {} {:?}", task.task_id(), e)
                    );
                    return Err(TaskError::Failed(Reason::OthersError));
                }
                _ => {
                    if format!("{}", e).contains("No space left on device") {
                        sys_event!(
                            ExecFault,
                            DfxCode::TASK_FAULT_09,
                            &format!("Task {} {:?}", task.task_id(), e)
                        );
                        return Err(TaskError::Failed(Reason::InsufficientSpace));
                    } else {
                        sys_event!(
                            ExecFault,
                            DfxCode::TASK_FAULT_09,
                            &format!("Task {} {:?}", task.task_id(), e)
                        );
                        return Err(TaskError::Failed(Reason::OthersError));
                    }
                }
            };
        }
    };

    let response = response.unwrap();
    {
        let mut guard = task.progress.lock().unwrap();
        guard.extras.clear();
        for (k, v) in response.headers() {
            if let Ok(value) = v.to_string() {
                guard.extras.insert(k.to_string().to_lowercase(), value);
            }
        }
    }
    task.get_file_info(&response)?;
    task.update_progress_in_database();
    RequestDb::get_instance()
        .update_task_sizes(task.task_id(), &task.progress.lock().unwrap().sizes);

    #[cfg(feature = "oh")]
    let _trace = Trace::new(&format!(
        "download file tid:{} size:{}",
        task.task_id(),
        task.progress
            .lock()
            .unwrap()
            .sizes
            .first()
            .unwrap_or_else(|| {
                error!("Failed to get a progress lock size from an empty vector in Progress");
                &0
            })
    ));
    let mut downloader = build_downloader(task.clone(), response, abort_flag);

    if let Err(e) = downloader.download().await {
        return task.handle_download_error(e).await;
    }

    let file_mutex = task.files.get(0).unwrap();
    task_control::file_sync_all(file_mutex).await?;

    #[cfg(not(test))]
    check_file_exist(&task)?;
    {
        let mut guard = task.progress.lock().unwrap();
        guard.sizes = vec![guard.processed.first().map_or_else(
            || {
                error!("Failed to get a process size from an empty vector in RequestTask");
                Default::default()
            },
            |x| *x as i64,
        )];
    }

    info!("{} downloaded", task.task_id());
    Ok(())
}

#[cfg(not(test))]
fn check_file_exist(task: &Arc<RequestTask>) -> Result<(), TaskError> {
    use crate::task::files::{convert_path, BundleCache};

    let config = task.config();
    // download_server is unable to access the file path of user file.
    if let Some(first_file_spec) = config.file_specs.first() {
        if first_file_spec.is_user_file {
            return Ok(());
        }
    } else {
        info!("Failed to get the first FileSpec from an empty vector in TaskConfig");
    }
    let mut bundle_cache = BundleCache::new(config);
    let bundle_name = bundle_cache
        .get_value()
        .map_err(|_| TaskError::Failed(Reason::OthersError))?;
    let real_path = convert_path(
        config.common_data.uid,
        &bundle_name,
        match &config.file_specs.first() {
            Some(spec) => &spec.path,
            None => {
                error!("Failed to get the first file_spec from an empty vector in TaskConfig");
                Default::default()
            }
        },
    );
    // Cannot compare because file_total_size will be changed when resume task.
    match std::fs::metadata(real_path) {
        Ok(metadata) => {
            if !metadata.is_file() {
                error!("task {} check local not file", task.task_id());
                sys_event!(
                    ExecFault,
                    DfxCode::TASK_FAULT_04,
                    &format!("task {} check local not file", task.task_id())
                );
                return Err(TaskError::Failed(Reason::IoError));
            }
        }
        Err(e) => {
            // Skip this situation when we loss some permission.
            if e.kind() == std::io::ErrorKind::NotFound {
                error!("task {} check local not exist", task.task_id());
                sys_event!(
                    ExecFault,
                    DfxCode::TASK_FAULT_04,
                    &format!("task {} check local not exist", task.task_id())
                );
                return Err(TaskError::Failed(Reason::IoError));
            }
        }
    }
    Ok(())
}

#[cfg(not(feature = "oh"))]
#[cfg(test)]
mod ut_download {
    include!("../../tests/ut/task/ut_download.rs");
}
