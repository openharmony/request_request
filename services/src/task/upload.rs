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
use std::io::SeekFrom;
use std::pin::Pin;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::task::{Context, Poll};

use ylong_http_client::async_impl::{Body, MultiPart, Part, Request, UploadOperator, Uploader};
use ylong_http_client::{ErrorKind, HttpClientError, ReusableReader};
use ylong_runtime::io::{AsyncRead, AsyncSeekExt, ReadBuf};

use super::operator::TaskOperator;
use super::reason::Reason;
use super::request_task::{TaskError, TaskPhase};
use crate::manage::database::RequestDb;
use crate::task::request_task::RequestTask;
#[cfg(feature = "oh")]
use crate::trace::Trace;

struct TaskReader {
    pub(crate) task: Arc<RequestTask>,
    pub(crate) index: usize,
    pub(crate) reused: Option<usize>,
}

impl TaskReader {
    pub(crate) fn new(task: Arc<RequestTask>, index: usize) -> Self {
        Self {
            task,
            index,
            reused: None,
        }
    }
}

impl AsyncRead for TaskReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let index = self.index;
        let file = self
            .task
            .files
            .get_mut(index)
            .ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))?;
        let mut progress_guard = self.task.progress.lock().unwrap();

        if self.task.conf.common_data.index == index as u32 || progress_guard.processed[index] != 0
        {
            let total_upload_bytes = if let Some(uploaded) = self.reused {
                progress_guard.sizes[index] as usize - uploaded
            } else {
                progress_guard.sizes[index] as usize - progress_guard.processed[index]
            };
            let buf_filled_len = buf.filled().len();
            let mut read_buf = buf.take(total_upload_bytes);
            match Pin::new(file).poll_read(cx, &mut read_buf) {
                Poll::Ready(Ok(_)) => {
                    let upload_size = read_buf.filled().len();
                    // need update buf.filled and buf.initialized
                    buf.assume_init(upload_size);
                    buf.set_filled(buf_filled_len + upload_size);
                    match self.reused {
                        None => {
                            progress_guard.processed[index] += upload_size;
                            progress_guard.common_data.total_processed += upload_size;
                        }
                        Some(uploaded) => {
                            drop(progress_guard);
                            self.reused = Some(uploaded + upload_size);
                        }
                    }
                    Poll::Ready(Ok(()))
                }
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            }
        } else {
            let filled_len = buf.filled().len();
            match Pin::new(file).poll_read(cx, buf) {
                Poll::Ready(Ok(_)) => {
                    let current_filled_len = buf.filled().len();
                    let upload_size = current_filled_len - filled_len;
                    progress_guard.processed[index] += upload_size;
                    progress_guard.common_data.total_processed += upload_size;
                    Poll::Ready(Ok(()))
                }
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            }
        }
    }
}

impl ReusableReader for TaskReader {
    fn reuse<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = std::io::Result<()>> + Send + Sync + 'a>>
    where
        Self: 'a,
    {
        self.reused = Some(0);
        let index = self.index;
        let optional_file = self.task.files.get_mut(index);
        Box::pin(async {
            let file = optional_file.ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))?;
            file.rewind().await.map(|_| ())
        })
    }
}

impl UploadOperator for TaskOperator {
    fn poll_progress(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        _uploaded: u64,
        _total: Option<u64>,
    ) -> Poll<Result<(), HttpClientError>> {
        let mut this = self;
        this.poll_progress_common(cx)
    }
}

fn build_stream_request(task: Arc<RequestTask>, index: usize) -> Option<Request> {
    debug!("build stream request");
    let task_reader = TaskReader::new(task.clone(), index);
    let task_operator = TaskOperator::new(task.clone());

    match task.build_request_builder() {
        Ok(mut request_builder) => {
            if !task.conf.headers.contains_key("Content-Type") {
                request_builder =
                    request_builder.header("Content-Type", "application/octet-stream");
            }
            let upload_length;
            {
                let progress = task.progress.lock().unwrap();
                upload_length = progress.sizes[index] as u64 - progress.processed[index] as u64;
            }
            debug!("upload length is {}", upload_length);
            request_builder =
                request_builder.header("Content-Length", upload_length.to_string().as_str());
            let uploader = Uploader::builder()
                .reader(task_reader)
                .operator(task_operator)
                .total_bytes(Some(upload_length))
                .build();
            let request = request_builder.body(Body::stream(uploader));
            build_request_common(&task, index, request)
        }
        Err(err) => build_request_common(&task, index, Err(err)),
    }
}

fn build_multipart_request(task: Arc<RequestTask>, index: usize) -> Option<Request> {
    debug!("build multipart request");
    let task_reader = TaskReader::new(task.clone(), index);
    let task_operator = TaskOperator::new(task.clone());
    let mut multi_part = MultiPart::new();
    for item in task.conf.form_items.iter() {
        let part = Part::new()
            .name(item.name.as_str())
            .body(item.value.as_str());
        multi_part = multi_part.part(part);
    }
    let upload_length;
    {
        let progress = task.progress.lock().unwrap();
        upload_length = progress.sizes[index] as u64 - progress.processed[index] as u64;
    }
    debug!("upload length is {}", upload_length);
    let part = Part::new()
        .name(task.conf.file_specs[index].name.as_str())
        .file_name(task.conf.file_specs[index].file_name.as_str())
        .mime(task.conf.file_specs[index].mime_type.as_str())
        .length(Some(upload_length))
        .stream(task_reader);

    multi_part = multi_part.part(part);
    let uploader = Uploader::builder()
        .multipart(multi_part)
        .operator(task_operator)
        .build();

    match task.build_request_builder() {
        Ok(request_builder) => {
            let request: Result<Request, HttpClientError> =
                request_builder.body(Body::multipart(uploader));
            build_request_common(&task, index, request)
        }
        Err(err) => build_request_common(&task, index, Err(err)),
    }
}

fn build_request_common(
    task: &Arc<RequestTask>,
    _index: usize,
    request: Result<Request, HttpClientError>,
) -> Option<Request> {
    match request {
        Ok(value) => {
            debug!(
                "build upload request success, tid: {}",
                task.conf.common_data.task_id
            );
            Some(value)
        }
        Err(e) => {
            error!("build upload request error is {:?}", e);
            None
        }
    }
}

impl RequestTask {
    async fn prepare_single_upload(&self, index: usize) -> bool {
        if let Some(file) = self.files.get_mut(index) {
            let size;
            {
                let mut progress = self.progress.lock().unwrap();
                progress.common_data.index = index;
                progress.processed[index] = 0;
                progress.common_data.total_processed = progress.processed.iter().take(index).sum();
                size = progress.sizes[index] as u64;
            }
            if self.conf.common_data.index == index as u32 {
                let Ok(metadata) = file.metadata().await else {
                    error!("get file metadata failed");
                    return false;
                };
                if metadata.len() > size {
                    file.seek(SeekFrom::Start(self.conf.common_data.begins));
                }
            } else {
                file.seek(SeekFrom::Start(0));
            }
            true
        } else {
            error!("task {} file {} not found", self.task_id(), index);
            false
        }
    }
}

pub(crate) async fn upload(task: Arc<RequestTask>) {
    RequestDb::get_instance()
        .update_task_sizes(task.task_id(), &task.progress.lock().unwrap().sizes);
    
    task.tries.store(0, Ordering::SeqCst);
    loop {
        if let Err(e) = upload_inner(task.clone()).await {
            match e {
                TaskError::Failed(reason) => {
                    *task.running_result.lock().unwrap() = Some(Err(reason));
                }
                TaskError::Waiting(phase) => match phase {
                    TaskPhase::NeedRetry => {
                        continue;
                    }
                    TaskPhase::UserAbort => {}
                    TaskPhase::NetworkOffline => {
                        *task.running_result.lock().unwrap() = Some(Err(Reason::NetworkOffline));
                    }
                },
            }
        } else {
            *task.running_result.lock().unwrap() = Some(Ok(()));
        }
        break;
    }
}

async fn upload_inner(task: Arc<RequestTask>) -> Result<(), TaskError> {
    info!("upload task {} start running", task.task_id());

    #[cfg(feature = "oh")]
    let _trace = Trace::new(&format!(
        "exec upload task:{} file num:{}",
        task.task_id(),
        task.conf.file_specs.len()
    ));

    let size = task.conf.file_specs.len();
    let start = task.progress.lock().unwrap().common_data.index;

    for index in start..size {
        #[cfg(feature = "oh")]
        let _trace = Trace::new(&format!("upload file:{} index:{}", task.task_id(), index));

        if !task.prepare_single_upload(index).await {
            return Err(TaskError::Failed(Reason::OthersError));
        }
        let is_multipart = match task.conf.headers.get("Content-Type") {
            Some(s) => s.eq("multipart/form-data"),
            None => task.conf.method.to_uppercase().eq("POST"),
        };

        if is_multipart {
            upload_one_file(task.clone(), index, build_multipart_request).await?
        } else {
            upload_one_file(task.clone(), index, build_stream_request).await?
        };
        task.notify_header_receive();
    }

    info!("task {} upload success", task.task_id());
    Ok(())
}

async fn upload_one_file<F>(
    task: Arc<RequestTask>,
    index: usize,
    build_upload_request: F,
) -> Result<(), TaskError>
where
    F: Fn(Arc<RequestTask>, usize) -> Option<Request>,
{
    info!(
        "begin upload one file, tid: {}, index is {}, sizes {}",
        task.conf.common_data.task_id,
        index,
        task.progress.lock().unwrap().sizes[index]
    );

    let Some(request) = build_upload_request(task.clone(), index) else {
        return Err(TaskError::Failed(Reason::BuildRequestFailed));
    };

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
                if task.timeout_tries.load(Ordering::SeqCst) < 2 {
                    task.timeout_tries.fetch_add(1, Ordering::SeqCst);
                    return Err(TaskError::Waiting(TaskPhase::NeedRetry));
                } else {
                    return Err(TaskError::Failed(Reason::ProtocolError));
                }
            } else {
                task.timeout_tries.store(0, Ordering::SeqCst);
            }
        }
        Err(e) => {
            error!("Task {} {:?}", task.task_id(), e);

            match e.error_kind() {
                ErrorKind::Timeout => return Err(TaskError::Failed(Reason::ContinuousTaskTimeout)),
                ErrorKind::Request => return Err(TaskError::Failed(Reason::RequestError)),
                ErrorKind::Redirect => return Err(TaskError::Failed(Reason::RedirectError)),
                ErrorKind::Connect | ErrorKind::ConnectionUpgrade => {
                    task.network_retry().await?;
                    if e.is_dns_error() {
                        return Err(TaskError::Failed(Reason::Dns));
                    } else if e.is_tls_error() {
                        return Err(TaskError::Failed(Reason::Ssl));
                    } else {
                        return Err(TaskError::Failed(Reason::Tcp));
                    }
                }
                ErrorKind::BodyTransfer => {
                    task.network_retry().await?;
                    return Err(TaskError::Failed(Reason::OthersError));
                }
                ErrorKind::UserAborted => return Err(TaskError::Waiting(TaskPhase::UserAbort)),
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
    task.record_upload_response(index, response).await;
    Ok(())
}

#[cfg(not(feature = "oh"))]
#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};
    use std::net::{TcpListener, TcpStream};
    use std::sync::Arc;

    use once_cell::sync::Lazy;

    use crate::config::{Action, ConfigBuilder, Mode, TaskConfig};
    use crate::info::State;
    use crate::manage::network::Network;
    use crate::manage::task_manager::TaskManagerTx;
    use crate::manage::TaskManager;
    use crate::service::client::{ClientManager, ClientManagerEntry};
    use crate::service::run_count::{RunCountManager, RunCountManagerEntry};
    use crate::task::request_task::{check_config, RequestTask};
    use crate::task::upload::upload_inner;
    const SERVER_ADDR: &str = "127.0.0.1:8989";

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
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            info!("server start {}", SERVER_ADDR);
            std::thread::spawn(|| {
                let listener = TcpListener::bind(SERVER_ADDR).unwrap();
                for stream in listener.incoming() {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    let stream = stream.unwrap();
                    handle_connection(stream);
                }
            });
        })
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        debug!("http request: {:#?}", http_request);
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }

    #[test]
    fn ut_upload_basic() {
        init();
        let file_path = "test_files/ut_upload_basic.txt";

        let file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap();
        file.set_len(100000).unwrap();

        let config = ConfigBuilder::new()
            .action(Action::Upload)
            .method("POST")
            .mode(Mode::BackGround)
            .file_spec(file)
            .url(&format!("http://{}/", SERVER_ADDR))
            .redirect(true)
            .version(1)
            .build();
        let task = build_task(config);

        ylong_runtime::block_on(async {
            upload_inner(task).await.unwrap();
        })
    }
}
