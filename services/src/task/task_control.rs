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

//! Task control utilities for asynchronous operations.
//! 
//! This module provides utility functions for spawning blocking operations in async context
//! and performing file operations in a thread-safe manner, primarily used for HTTP request tasks.

use std::fs::{File, Metadata};
use std::io::{self, Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};

use ylong_runtime::task::JoinHandle;

use crate::task::request_task::RequestTask;

/// Spawns a blocking operation that returns a result.
/// 
/// This function wraps `ylong_runtime::spawn_blocking` to provide a consistent interface
/// for spawning blocking operations that return `Result<T, io::Error>`.
/// 
/// # Type Parameters
/// 
/// * `F` - A function that performs the blocking operation.
/// * `T` - The success type of the operation's result.
/// 
/// # Arguments
/// 
/// * `fut` - The blocking function to spawn.
/// 
/// # Returns
/// 
/// A `JoinHandle` for the spawned blocking task.
pub(crate) fn runtime_spawn_blocking<F, T>(fut: F) -> JoinHandle<Result<T, io::Error>>
where
    F: FnOnce() -> Result<T, io::Error> + Send + Sync + 'static,
    T: Send + 'static,
{
    ylong_runtime::spawn_blocking(
        Box::new(fut) as Box<dyn FnOnce() -> Result<T, io::Error> + Send + Sync>
    )
}

/// Moves the file cursor to the specified position asynchronously.
/// 
/// # Arguments
/// 
/// * `file` - A thread-safe reference to the file.
/// * `pos` - The position to seek to.
/// 
/// # Returns
/// 
/// The new position of the file cursor.
/// 
/// # Errors
/// 
/// Returns an error if the seek operation fails or if the blocking task fails.
pub(crate) async fn file_seek(file: Arc<Mutex<File>>, pos: SeekFrom) -> io::Result<u64> {
    runtime_spawn_blocking(move || {
        let mut file = file.lock().unwrap();
        file.flush()?; // Ensure all pending writes are committed before seeking
        file.seek(pos)
    })
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
}

/// Moves the file cursor to the beginning asynchronously.
/// 
/// # Arguments
/// 
/// * `file` - A thread-safe reference to the file.
/// 
/// # Returns
/// 
/// `Ok(())` if the operation succeeds.
/// 
/// # Errors
/// 
/// Returns an error if the rewind operation fails or if the blocking task fails.
pub(crate) async fn file_rewind(file: Arc<Mutex<File>>) -> io::Result<()> {
    runtime_spawn_blocking(move || {
        let mut file = file.lock().unwrap();
        file.flush()?; // Ensure all pending writes are committed before rewinding
        file.rewind()
    })
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
}

/// Synchronizes all file data and metadata to disk asynchronously.
/// 
/// # Arguments
/// 
/// * `file` - A thread-safe reference to the file.
/// 
/// # Returns
/// 
/// `Ok(())` if the operation succeeds.
/// 
/// # Errors
/// 
/// Returns an error if the sync operation fails or if the blocking task fails.
pub(crate) async fn file_sync_all(file: Arc<Mutex<File>>) -> io::Result<()> {
    runtime_spawn_blocking(move || {
        let mut file = file.lock().unwrap();
        file.flush()?; // Ensure all pending writes are committed
        file.sync_all() // Sync both data and metadata to disk
    })
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
}

/// Retrieves file metadata asynchronously.
/// 
/// # Arguments
/// 
/// * `file` - A thread-safe reference to the file.
/// 
/// # Returns
/// 
/// The file's metadata.
/// 
/// # Errors
/// 
/// Returns an error if the metadata operation fails or if the blocking task fails.
pub(crate) async fn file_metadata(file: Arc<Mutex<File>>) -> io::Result<Metadata> {
    runtime_spawn_blocking(move || {
        let file = file.lock().unwrap();
        file.metadata()
    })
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
}

/// Sets the length of a file asynchronously.
/// 
/// # Arguments
/// 
/// * `file` - A thread-safe reference to the file.
/// * `size` - The new size of the file in bytes.
/// 
/// # Returns
/// 
/// `Ok(())` if the operation succeeds.
/// 
/// # Errors
/// 
/// Returns an error if the resize operation fails or if the blocking task fails.
pub(crate) async fn file_set_len(file: Arc<Mutex<File>>, size: u64) -> io::Result<()> {
    runtime_spawn_blocking(move || {
        let mut file = file.lock().unwrap();
        file.flush()?; // Ensure all pending writes are committed before resizing
        file.set_len(size)
    })
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
}

/// Writes all bytes from a buffer to a file asynchronously.
/// 
/// # Arguments
/// 
/// * `file` - A thread-safe reference to the file.
/// * `buf` - The buffer containing the bytes to write.
/// 
/// # Returns
/// 
/// `Ok(())` if the operation succeeds.
/// 
/// # Errors
/// 
/// Returns an error if the write operation fails or if the blocking task fails.
pub(crate) async fn file_write_all<'a>(file: Arc<Mutex<File>>, buf: &[u8]) -> io::Result<()> {
    // Clone the buffer to move it into the blocking task
    let buf = buf.to_vec();
    runtime_spawn_blocking(move || {
        let mut file = file.lock().unwrap();
        file.write_all(&buf)
    })
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
}

/// Clears a downloaded file and resets its progress tracking.
/// 
/// This function truncates the first file in the task to zero length and resets
/// the progress tracking information.
/// 
/// # Arguments
/// 
/// * `task` - The request task containing the file to clear.
/// 
/// # Returns
/// 
/// `Ok(())` if the operation succeeds.
/// 
/// # Errors
/// 
/// Returns an error if there are no files in the task or if the file operations fail.
pub(crate) async fn clear_downloaded_file(task: Arc<RequestTask>) -> Result<(), std::io::Error> {
    info!("task {} clear downloaded file", task.task_id());
    runtime_spawn_blocking(move || {
        // Clear the file content
        {
            let file_mutex = if let Some(mutex) = task.files.get(0) {
                mutex
            } else {
                error!("clear_downloaded_file err, 1no file in the `task`");
                return Err(io::Error::new(io::ErrorKind::Other,
                                          "clear_downloaded_file err, 1no file in the `task`"
                ));
            };

            let mut file = file_mutex.lock().unwrap();
            file.set_len(0)?; // Truncate the file to zero length
            file.seek(SeekFrom::Start(0))?; // Reset file position
        }
        
        // Reset progress tracking
        {
            let mut progress_guard = task.progress.lock().unwrap();
            progress_guard.common_data.total_processed = 0;
            if let Some(elem) = progress_guard.processed.get_mut(0) {
                *elem = 0; // Reset individual file progress
            } else {
                info!("Failed to get a process size from an empty vector in Progress");
            }
        }
        Ok(())
    })
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
}
