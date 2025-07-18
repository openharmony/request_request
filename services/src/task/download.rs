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
use std::time::{Instant, SystemTime, UNIX_EPOCH};

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
        let file = self.files.get(0).unwrap();
        task_control::file_seek(file.clone(), SeekFrom::End(0)).await?;
        let downloaded = task_control::file_metadata(file).await?.len() as usize;

        let mut progress = self.progress.lock().unwrap();
        progress.common_data.index = 0;
        progress.common_data.total_processed = downloaded;
        progress.common_data.state = State::Running.repr;
        progress.processed = vec![downloaded];
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

    info!("download task {} running", task.task_id());

    let request = RequestTask::build_download_request(task.clone()).await?;
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64;

    task.start_time.store(start_time as u64, Ordering::SeqCst);
    let client = task.client.lock().await;
    let response = client.request(request).await;
    match response.as_ref() {
        Ok(response) => {
            let status_code = response.status();
            task.status_code
                .store(status_code.as_u16() as i32, Ordering::SeqCst);
            #[cfg(feature = "oh")]
            task.notify_response(response);
            info!(
                "task {} get response {}",
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
                let file = task.files.get(0).unwrap();

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
        task.progress.lock().unwrap().sizes[0]
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
        guard.sizes = vec![guard.processed[0] as i64];
    }

    info!("task {} download ok", task.task_id());
    Ok(())
}

#[cfg(not(test))]
fn check_file_exist(task: &Arc<RequestTask>) -> Result<(), TaskError> {
    use crate::task::files::{convert_path, BundleCache};

    let config = task.config();
    // download_server is unable to access the file path of user file.
    if config.file_specs[0].is_user_file {
        return Ok(());
    }
    let mut bundle_cache = BundleCache::new(config);
    let bundle_name = bundle_cache
        .get_value()
        .map_err(|_| TaskError::Failed(Reason::OthersError))?;
    let real_path = convert_path(
        config.common_data.uid,
        &bundle_name,
        &config.file_specs[0].path,
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
        let (files, client) = check_config(&config, 0).unwrap();

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

    // @tc.name: ut_download_basic
    // @tc.desc: Test basic file download functionality
    // @tc.precon: NA
    // @tc.step: 1. Create test file and build download configuration
    //           2. Execute download_inner function
    //           3. Verify downloaded file length matches expected value
    // @tc.expect: File is downloaded successfully with correct length
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
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


    // @tc.name: ut_download_resume
    // @tc.desc: Test download resumption from partial file
    // @tc.precon: NA
    // @tc.step: 1. Create partial test file with initial content
    //           2. Build download configuration with resume capability
    //           3. Execute download_inner function
    //           4. Verify final file length matches expected value
    // @tc.expect: Download resumes successfully and file length is correct
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
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

    // @tc.name: ut_download_not_support_range
    // @tc.desc: Test download with range request on server that doesn't support
    // range
    // @tc.precon: NA
    // @tc.step: 1. Create test file and build download configuration with begins
    // parameter
    //           2. Execute download_inner function
    //           3. Verify error type and file length
    // @tc.expect: Download fails with UnsupportedRangeRequest error and file
    // remains empty
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
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

    // @tc.name: ut_download_resume_not_support_range
    // @tc.desc: Test download resumption when server doesn't support range requests
    // @tc.precon: NA
    // @tc.step: 1. Create test file and build download configuration
    //           2. Interrupt download and modify task state
    //           3. Resume download with partial file
    //           4. Verify final file length matches expected value
    // @tc.expect: Download resumes successfully despite server not supporting range
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
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

    // @tc.name: ut_download_not_support_range_resume
    // @tc.desc: Test resuming range download on server that doesn't support range
    // @tc.precon: NA
    // @tc.step: 1. Create partial test file with initial content
    //           2. Build download configuration with begins parameter
    //           3. Execute download_inner function
    //           4. Verify error type and file length
    // @tc.expect: Download fails with UnsupportedRangeRequest error and file length
    // remains 1000
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
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

    // @tc.name: ut_download_range_0
    // @tc.desc: Test download with specified range (begins and ends parameters)
    // @tc.precon: NA
    // @tc.step: 1. Create test file and build download configuration with
    // begins=5000 and ends=10000
    //           2. Execute download_inner function
    //           3. Verify downloaded file length is 5001 bytes
    // @tc.expect: Range download succeeds with correct file length
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
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

    // @tc.name: ut_download_range_1
    // @tc.desc: Test download with specified begins parameter only
    // @tc.precon: NA
    // @tc.step: 1. Create test file and build download configuration with
    // begins=273619168
    //           2. Execute download_inner function
    //           3. Verify downloaded file length matches expected remaining bytes
    // @tc.expect: Range download succeeds with correct remaining file length
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
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

    // @tc.name: ut_download_range_resume_0
    // @tc.desc: Test resuming range download with partial file
    // @tc.precon: NA
    // @tc.step: 1. Create partial test file with initial content
    //           2. Build download configuration
    //           3. Execute download_inner function
    //           4. Verify final file length matches expected value
    // @tc.expect: Range download resumes successfully with correct total length
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
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

    // @tc.name: ut_download_range_resume_1
    // @tc.desc: Test download resumption after range request interruption
    // @tc.precon: NA
    // @tc.step: 1. Create test file with partial length
    //           2. Configure download task with range request
    //           3. Simulate download interruption
    //           4. Resume download and verify completion
    // @tc.expect: File length matches expected size after resumption
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
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

    // @tc.name: ut_download_invalid_task
    // @tc.desc: Test handling of invalid task configuration
    // @tc.precon: NA
    // @tc.step: 1. Create test file
    //           2. Configure download task with invalid progress data
    //           3. Execute download and verify correction
    // @tc.expect: Task corrects invalid data and completes successfully
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
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
    // @tc.name: ut_download_sizes
    // @tc.desc: Test error handling for invalid download sizes
    // @tc.precon: NA
    // @tc.step: 1. Create test file
    //           2. Configure task with invalid size array
    //           3. Execute download with non-existent URL
    // @tc.expect: Returns ProtocolError and sets size to -1
    // @tc.type: FUNC
    // @tc.require: issues#ICN16H
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
