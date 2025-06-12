// Copyright (C) 2025 Huawei Device Co., Ltd.
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

use std::fs::{File, Metadata};
use std::io::{self, Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};

use ylong_runtime::task::JoinHandle;

use crate::task::request_task::RequestTask;

pub(crate) fn runtime_spawn_blocking<F, T>(fut: F) -> JoinHandle<Result<T, io::Error>>
where
    F: FnOnce() -> Result<T, io::Error> + Send + Sync + 'static,
    T: Send + 'static,
{
    ylong_runtime::spawn_blocking(
        Box::new(fut) as Box<dyn FnOnce() -> Result<T, io::Error> + Send + Sync>
    )
}

pub(crate) async fn file_seek(file: Arc<Mutex<File>>, pos: SeekFrom) -> io::Result<u64> {
    runtime_spawn_blocking(move || {
        let mut file = file.lock().unwrap();
        file.flush()?;
        file.seek(pos)
    })
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
}

pub(crate) async fn file_rewind(file: Arc<Mutex<File>>) -> io::Result<()> {
    runtime_spawn_blocking(move || {
        let mut file = file.lock().unwrap();
        file.flush()?;
        file.rewind()
    })
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
}

pub(crate) async fn file_sync_all(file: Arc<Mutex<File>>) -> io::Result<()> {
    runtime_spawn_blocking(move || {
        let mut file = file.lock().unwrap();
        file.flush()?;
        file.sync_all()
    })
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
}

pub(crate) async fn file_metadata(file: Arc<Mutex<File>>) -> io::Result<Metadata> {
    runtime_spawn_blocking(move || {
        let file = file.lock().unwrap();
        file.metadata()
    })
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
}

pub(crate) async fn file_set_len(file: Arc<Mutex<File>>, size: u64) -> io::Result<()> {
    runtime_spawn_blocking(move || {
        let mut file = file.lock().unwrap();
        file.flush()?;
        file.set_len(size)
    })
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
}

pub(crate) async fn file_write_all<'a>(file: Arc<Mutex<File>>, buf: &[u8]) -> io::Result<()> {
    let buf = buf.to_vec();
    runtime_spawn_blocking(move || {
        let mut file = file.lock().unwrap();
        file.write_all(&buf)
    })
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
}

pub(crate) async fn clear_downloaded_file(task: Arc<RequestTask>) -> Result<(), std::io::Error> {
    info!("task {} clear downloaded file", task.task_id());
    runtime_spawn_blocking(move || {
        {
            let file_mutex = task.files.get(0).unwrap();
            let mut file = file_mutex.lock().unwrap();
            file.set_len(0)?;
            file.seek(SeekFrom::Start(0))?;
        }
        {
            let mut progress_guard = task.progress.lock().unwrap();
            progress_guard.common_data.total_processed = 0;
            progress_guard.processed[0] = 0;
        }
        Ok(())
    })
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
}
