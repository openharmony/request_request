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
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use ylong_http_client::async_impl::{DownloadOperator, Downloader, Response};
use ylong_http_client::{ErrorKind, HttpClientError, SpeedLimit, Timeout};
use ylong_runtime::io::AsyncSeekExt;

use super::operator::TaskOperator;
use super::reason::Reason;
use super::request_task::{TaskError, TaskPhase};
use crate::task::info::State;
use crate::task::request_task::RequestTask;
#[cfg(feature = "oh")]
use crate::trace::Trace;

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
    loop {
        if let Err(e) = download_inner(task.clone()).await {
            match e {
                TaskError::Waiting(phase) => match phase {
                    TaskPhase::NeedRetry => {
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
        let file = self.files.get_mut(0).unwrap();
        file.seek(SeekFrom::End(0));
        let downloaded = file.metadata().await?.len() as usize;
        let mut progress = self.progress.lock().unwrap();
        progress.common_data.index = 0;
        progress.common_data.total_processed = downloaded;
        progress.common_data.state = State::Running.repr;
        progress.processed = vec![downloaded];
        progress.sizes = vec![-1];
        Ok(())
    }
}

pub(crate) async fn download_inner(task: Arc<RequestTask>) -> Result<(), TaskError> {
    // Ensures `_trace` can only be freed when this function exits.
    #[cfg(feature = "oh")]
    let _trace = Trace::new("download file");
    task.prepare_running();
    task.prepare_download().await?;

    info!("download task {} start running", task.task_id());

    task.range_response.store(false, Ordering::SeqCst);
    task.range_request.store(false, Ordering::SeqCst);
    let request = task.build_download_request().await?;

    let response = task.client.request(request).await;
    match response.as_ref() {
        Ok(response) => {
            let status_code = response.status();
            #[cfg(feature = "oh")]
            task.notify_response(response);
            info!(
                "task {} get http response code {}",
                task.conf.common_data.task_id, status_code,
            );
            if status_code.is_server_error()
                || (status_code.as_u16() != 408 && status_code.is_client_error())
                || status_code.is_redirection()
            {
                return Err(TaskError::Failed(Reason::ProtocolError));
            }
            if status_code.as_u16() == 408 {
                if task.tries.load(Ordering::SeqCst) < 2 {
                    task.tries.fetch_add(1, Ordering::SeqCst);
                    info!("task {} server timeout", task.task_id());
                    return Err(TaskError::Waiting(TaskPhase::NeedRetry));
                } else {
                    info!("task {} retry 3 times", task.task_id());
                    return Err(TaskError::Failed(Reason::ProtocolError));
                }
            }
            if status_code.as_u16() == 200 {
                if task.require_range() {
                    info!("task {} server not support range request", task.task_id());
                    return Err(TaskError::Failed(Reason::UnsupportedRangeRequest));
                }
                let file = task.files.get(0).unwrap();
                let has_downloaded = file.metadata().await?.len() > 0;
                if has_downloaded {
                    error!("task {} file not cleared", task.task_id());
                    task.clear_downloaded_file().await?;
                }
            }
        }
        Err(e) => {
            error!("Task {} {:?}", task.task_id(), e);

            match e.error_kind() {
                ErrorKind::Timeout => return Err(TaskError::Failed(Reason::ContinuousTaskTimeout)),
                ErrorKind::Request => return Err(TaskError::Failed(Reason::RequestError)),
                ErrorKind::Redirect => return Err(TaskError::Failed(Reason::RedirectError)),
                ErrorKind::Connect | ErrorKind::ConnectionUpgrade => {
                    if task.tries.load(Ordering::SeqCst) < 2 {
                        task.tries.fetch_add(1, Ordering::SeqCst);
                        if !task.network.is_online() {
                            return Err(TaskError::Waiting(TaskPhase::NetworkOffline));
                        } else {
                            ylong_runtime::time::sleep(Duration::from_millis(500)).await;
                            return Err(TaskError::Waiting(TaskPhase::NeedRetry));
                        }
                    }
                    info!("task {} retry 3 times", task.task_id());
                    if e.is_dns_error() {
                        return Err(TaskError::Failed(Reason::Dns));
                    } else if e.is_tls_error() {
                        return Err(TaskError::Failed(Reason::Ssl));
                    } else {
                        return Err(TaskError::Failed(Reason::Tcp));
                    }
                }
                ErrorKind::BodyTransfer => {
                    return Err(TaskError::Waiting(TaskPhase::NetworkOffline))
                }
                _ => {
                    if format!("{}", e).contains("No space left on device") {
                        return Err(TaskError::Failed(Reason::InsufficientSpace));
                    } else {
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
            if k.to_string() != "etag" && k.to_string() != "last-modified" {
                continue;
            }
            if let Ok(value) = v.to_string() {
                guard.extras.insert(k.to_string().to_lowercase(), value);
            }
        }
    }
    task.get_file_info(&response)?;
    task.update_progress_in_database();

    let mut downloader = build_downloader(task.clone(), response);

    if let Err(e) = downloader.download().await {
        return task.handle_download_error(e);
    }
    let file = task.files.get_mut(0).unwrap();
    file.sync_all().await?;

    info!("task {} download success", task.task_id());
    Ok(())
}

#[cfg(not(feature = "oh"))]
#[cfg(test)]
mod test {
    use core::time;
    use std::fs::File;
    use std::io::{SeekFrom, Write};
    use std::sync::Arc;

    use once_cell::sync::Lazy;
    use ylong_runtime::io::AsyncSeekExt;

    use crate::config::{Action, ConfigBuilder, Mode, TaskConfig};
    use crate::info::State;
    use crate::manage::network::Network;
    use crate::manage::task_manager::TaskManagerTx;
    use crate::manage::TaskManager;
    use crate::service::client::{ClientManager, ClientManagerEntry};
    use crate::service::run_count::{RunCountManager, RunCountManagerEntry};
    use crate::task::download::{download_inner, TaskPhase};
    use crate::task::reason::Reason;
    use crate::task::request_task::{check_config, RequestTask, TaskError};

    const GITEE_FILE_LEN: u64 = 1042003;
    const FS_FILE_LEN: u64 = 274619168;

    fn build_task(config: TaskConfig) -> Arc<RequestTask> {
        static CLIENT: Lazy<ClientManagerEntry> = Lazy::new(|| ClientManager::init());
        static RUN_COUNT_MANAGER: Lazy<RunCountManagerEntry> =
            Lazy::new(|| RunCountManager::init());
        static NETWORK: Lazy<Network> = Lazy::new(|| Network::new());

        static TASK_MANGER: Lazy<TaskManagerTx> = Lazy::new(|| {
            TaskManager::init(RUN_COUNT_MANAGER.clone(), CLIENT.clone(), NETWORK.clone())
        });
        let (files, client) = check_config(&config).unwrap();

        let task = Arc::new(RequestTask::new(
            config,
            files,
            client,
            CLIENT.clone(),
            NETWORK.clone(),
        ));
        task.status.lock().unwrap().state = State::Initialized;
        task
    }

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
        let _ = std::fs::create_dir("test_files/");
    }

    #[test]
    fn ut_download_basic() {
        init();
        let file_path = "test_files/ut_download_basic.txt";

        let file = File::create(file_path).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .build();

        let task = build_task(config);
        ylong_runtime::block_on(async {
            download_inner(task).await.unwrap();
            let file = File::open(file_path).unwrap();
            assert_eq!(GITEE_FILE_LEN, file.metadata().unwrap().len());
        });
    }

    #[test]
    fn ut_download_resume() {
        init();
        let file_path = "test_files/ut_download_resume.txt";

        let mut file = File::create(file_path).unwrap();
        file.write(&[0; GITEE_FILE_LEN as usize - 10000]).unwrap();

        let config = ConfigBuilder::new()
        .action(Action::Download)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .build();
        let task = build_task(config);
        ylong_runtime::block_on(async {
            download_inner(task).await.unwrap();
            let file = File::open(file_path).unwrap();
            assert_eq!(GITEE_FILE_LEN, file.metadata().unwrap().len());
        });
    }

    #[test]
    fn ut_download_not_support_range() {
        init();
        let file_path = "test_files/ut_download_not_support_range.txt";

        let file = File::create(file_path).unwrap();

        let config = ConfigBuilder::new()
        .action(Action::Download)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .begins(5000)
        .build();
        let task = build_task(config);
        ylong_runtime::block_on(async {
            let res = download_inner(task).await.unwrap_err();
            assert_eq!(res, TaskError::Failed(Reason::UnsupportedRangeRequest));
            let file = File::open(file_path).unwrap();
            assert_eq!(0, file.metadata().unwrap().len());
        });
    }

    #[test]
    fn ut_download_resume_not_support_range() {
        init();
        let file_path = "test_files/ut_download_resume_not_support_range.txt";

        let file = File::create(file_path).unwrap();

        let config = ConfigBuilder::new()
        .action(Action::Download)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .build();
        let task = build_task(config);
        ylong_runtime::block_on(async {
            let clone_task = task.clone();
            ylong_runtime::spawn(async move {
                ylong_runtime::time::sleep(time::Duration::from_secs(2)).await;
                clone_task.status.lock().unwrap().state = State::Waiting;
            });
            let err = download_inner(task.clone()).await.unwrap_err();
            assert_eq!(err, TaskError::Waiting(TaskPhase::UserAbort));

            let file = task.files.get_mut(0).unwrap();
            file.set_len(10000).await.unwrap();
            file.seek(SeekFrom::End(0));

            download_inner(task.clone()).await.unwrap();
            let file = File::open(file_path).unwrap();
            assert_eq!(GITEE_FILE_LEN, file.metadata().unwrap().len());
        });
    }

    #[test]
    fn ut_download_not_support_range_resume() {
        init();
        let file_path = "test_files/ut_download_not_support_range_resume.txt";

        let mut file = File::create(file_path).unwrap();
        file.write(&[0; 1000]).unwrap();

        let config = ConfigBuilder::new()
        .action(Action::Download)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .begins(5000)
        .build();
        let task = build_task(config);
        ylong_runtime::block_on(async {
            let res = download_inner(task).await.unwrap_err();
            assert_eq!(res, TaskError::Failed(Reason::UnsupportedRangeRequest));
            let file = File::open(file_path).unwrap();
            assert_eq!(1000, file.metadata().unwrap().len());
        });
    }

    #[test]
    fn ut_download_range_0() {
        init();
        let file_path = "test_files/ut_download_range_0.txt";
        let file = File::create(file_path).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://sf3-cn.feishucdn.com/obj/ee-appcenter/47273f95/Feishu-win32_ia32-7.9.7-signed.exe")
        .redirect(true)
        .begins(5000)
        .ends(10000)
        .build();
        let task = build_task(config);
        ylong_runtime::block_on(async {
            download_inner(task).await.unwrap();
            let file = File::open(file_path).unwrap();
            assert_eq!(5001, file.metadata().unwrap().len());
        });
    }

    #[test]
    fn ut_download_range_1() {
        init();
        let file_path = "test_files/ut_download_range_1.txt";

        let file = File::create(file_path).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://sf3-cn.feishucdn.com/obj/ee-appcenter/47273f95/Feishu-win32_ia32-7.9.7-signed.exe")
        .redirect(true)
        .begins(273619168)
        .build();
        let task = build_task(config);
        ylong_runtime::block_on(async {
            download_inner(task).await.unwrap();
            let file = File::open(file_path).unwrap();
            assert_eq!(FS_FILE_LEN - 273619168, file.metadata().unwrap().len());
        });
    }

    #[test]
    fn ut_download_range_resume_0() {
        init();
        let file_path = "test_files/ut_download_range_resume_0.txt";

        let mut file = File::create(file_path).unwrap();
        file.write(&[0; FS_FILE_LEN as usize - 10000]).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://sf3-cn.feishucdn.com/obj/ee-appcenter/47273f95/Feishu-win32_ia32-7.9.7-signed.exe")
        .redirect(true)
        .build();
        let task = build_task(config);
        ylong_runtime::block_on(async {
            download_inner(task).await.unwrap();
            let file = File::open(file_path).unwrap();
            assert_eq!(FS_FILE_LEN, file.metadata().unwrap().len());
        });
    }

    #[test]
    fn ut_download_range_resume_1() {
        init();
        let file_path = "test_files/ut_download_range_resume_1.txt";

        let file = File::create(file_path).unwrap();
        file.set_len(FS_FILE_LEN - 10000).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://sf3-cn.feishucdn.com/obj/ee-appcenter/47273f95/Feishu-win32_ia32-7.9.7-signed.exe")
        .redirect(true)
        .build();
        let task = build_task(config);
        ylong_runtime::block_on(async {
            let clone_task = task.clone();
            ylong_runtime::spawn(async move {
                ylong_runtime::time::sleep(time::Duration::from_secs(2)).await;
                clone_task.status.lock().unwrap().state = State::Waiting;
            });
            let ret = download_inner(task.clone()).await.unwrap_err();
            assert_eq!(ret, TaskError::Waiting(TaskPhase::UserAbort));
            let file = File::open(file_path).unwrap();
            assert!(file.metadata().unwrap().len() < FS_FILE_LEN - 20000);
            download_inner(task.clone()).await.unwrap();
            assert_eq!(file.metadata().unwrap().len(), FS_FILE_LEN);
        });
    }

    #[test]
    fn ut_download_invalid_task() {
        init();
        let file_path = "test_files/ut_download_basic.txt";

        let file = File::create(file_path).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://www.gitee.com/tiga-ultraman/downloadTests/releases/download/v1.01/test.txt")
        .redirect(true)
        .build();

        let task = build_task(config);
        {
            let mut progress = task.progress.lock().unwrap();
            progress.sizes = vec![0];
            progress.processed = vec![];
            progress.common_data.index = 23;
            progress.common_data.state = State::Failed.repr;
            progress.common_data.total_processed = 321223;
        }
        ylong_runtime::block_on(async {
            download_inner(task.clone()).await.unwrap();
            let file = File::open(file_path).unwrap();
            assert_eq!(GITEE_FILE_LEN, file.metadata().unwrap().len());

            assert_eq!(State::Completed, task.status.lock().unwrap().state);
            assert_eq!(0, task.progress.lock().unwrap().common_data.index);
            assert_eq!(
                GITEE_FILE_LEN,
                task.progress.lock().unwrap().common_data.total_processed as u64
            );
            assert_eq!(
                GITEE_FILE_LEN,
                task.progress.lock().unwrap().processed[0] as u64
            );
            assert_eq!(
                GITEE_FILE_LEN,
                task.progress.lock().unwrap().sizes[0] as u64
            );
        });
    }

    /// For xts SUB_REQUEST_CROSSPLATFORM_DOWNDLOAD_API_TASKINFO_0002,
    /// downloadTotalBytes to be -1
    #[test]
    fn ut_download_sizes() {
        init();
        let file_path = "test_files/ut_download_basic.txt";

        let file = File::create(file_path).unwrap();
        let config = ConfigBuilder::new()
        .action(Action::Download)
        .mode(Mode::BackGround)
        .file_spec(file)
        .url("https://gitee.com/chenzhixue/downloadTest/releases/download/v1.0/test_not_exists.apk")
        .redirect(true)
        .build();

        let task = build_task(config);
        {
            let mut progress = task.progress.lock().unwrap();
            progress.sizes = vec![0, 1, 2, 3];
            progress.processed = vec![];
            progress.common_data.index = 23;
            progress.common_data.state = State::Failed.repr;
            progress.common_data.total_processed = 321223;
        }
        ylong_runtime::block_on(async {
            let err = download_inner(task.clone()).await.unwrap_err();
            assert_eq!(err, TaskError::Failed(Reason::ProtocolError));
            let sizes = task.progress.lock().unwrap().sizes.clone();
            assert_eq!(sizes, vec![-1]);
        });
    }
}
