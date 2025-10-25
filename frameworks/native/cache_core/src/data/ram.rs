// Copyright (C) 2024 Huawei Device Co., Ltd.
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

//! RAM-based caching implementation for task data.
//! 
//! This module provides functionality for in-memory caching of task data,
//! including memory allocation management, data storage, and cleanup mechanisms.
//! The cache implementation manages memory allocation limits and automatically
//! releases resources when no longer needed.

use std::cmp::Ordering;
use std::io::{Cursor, Write};
use std::sync::Arc;

use request_utils::task_id::TaskId;

use super::MAX_CACHE_SIZE;
use crate::manage::CacheManager;

/// Default capacity for new cache vectors when no size is specified.
const DEFAULT_TRUNK_CAPACITY: usize = 512;

/// In-memory cache implementation for task data.
///
/// This struct manages RAM-based storage for task data, including memory allocation
/// tracking and automatic resource cleanup.
pub struct RamCache {
    /// Unique identifier for the task associated with this cache
    pub(super) task_id: TaskId,
    /// Binary data stored in the cache
    data: Vec<u8>,
    /// Amount of memory allocated for this cache (in bytes)
    applied: u64,
    /// Reference to the cache manager controlling this cache
    handle: &'static CacheManager,
}

impl Drop for RamCache {
    /// Releases allocated memory when the cache is dropped.
    ///
    /// Ensures that memory resources are properly released back to the cache manager
    /// when the cache goes out of scope, preventing memory leaks.
    fn drop(&mut self) {
        if self.applied != 0 {
            info!("ram {} released {}", self.task_id.brief(), self.applied);
            self.handle.ram_handle.lock().unwrap().release(self.applied);
        }
    }
}

impl RamCache {
    /// Creates a new RAM cache for the specified task.
    ///
    /// Attempts to allocate the requested amount of memory from the cache manager.
    /// If allocation fails, the cache will be created without reserved memory.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task associated with this cache
    /// - `handle`: Reference to the cache manager
    /// - `size`: Optional size hint for memory allocation
    ///
    /// # Returns
    /// A new RamCache instance with the specified parameters
    pub(crate) fn new(task_id: TaskId, handle: &'static CacheManager, size: Option<usize>) -> Self {
        let applied = match size {
            Some(size) => {
                if CacheManager::apply_cache(&handle.ram_handle, &handle.rams, size) {
                    info!("apply ram {} for {}", size, task_id.brief());
                    size as u64
                } else {
                    error!("apply ram {} for {} failed", size, task_id.brief());
                    0
                }
            }
            None => 0,
        };

        Self {
            task_id,
            data: Vec::with_capacity(size.unwrap_or(DEFAULT_TRUNK_CAPACITY)),
            applied,
            handle,
        }
    }

    /// Finalizes the cache after writing and registers it with the cache manager.
    ///
    /// Checks if the cache size is valid and registers it with both RAM and file cache managers.
    /// Converts the cache to an Arc for shared ownership.
    ///
    /// # Returns
    /// An Arc pointing to the finalized cache
    pub(crate) fn finish_write(mut self) -> Arc<RamCache> {
        let is_cache = self.check_size();
        let me = Arc::new(self);

        if is_cache {
            me.handle.update_ram_cache(me.clone());
        }
        me.handle.update_file_cache(me.task_id.clone(), me.clone());
        me
    }

    /// Checks and adjusts the allocated memory based on current data size.
    ///
    /// Handles three cases:
    /// - If data size equals allocated size: returns true
    /// - If data exceeds allocated size: tries to allocate more memory, returns success status
    /// - If data is smaller than allocated size: releases excess memory, returns true
    ///
    /// # Returns
    /// `true` if the cache is still valid for memory storage, `false` if it exceeds size limits
    pub(crate) fn check_size(&mut self) -> bool {
        match (self.data.len() as u64).cmp(&self.applied) {
            Ordering::Equal => true,
            Ordering::Greater => {
                let diff = self.data.len() - self.applied as usize;
                if self.data.len() > MAX_CACHE_SIZE as usize
                    || !CacheManager::apply_cache(&self.handle.ram_handle, &self.handle.rams, diff)
                {
                    // Exceeds maximum allowed size or failed to allocate additional memory
                    info!(
                        "apply extra ram {} cache for {} failed",
                        diff,
                        self.task_id.brief()
                    );
                    self.handle.ram_handle.lock().unwrap().release(self.applied);
                    self.applied = 0;
                    false
                } else {
                    // Successfully allocated additional memory
                    info!(
                        "apply extra ram {} cache for {} success",
                        diff,
                        self.task_id.brief()
                    );
                    self.applied = self.data.len() as u64;
                    true
                }
            }
            Ordering::Less => {
                // Release excess allocated memory
                self.handle
                    .ram_handle
                    .lock()
                    .unwrap()
                    .release(self.applied - self.data.len() as u64);
                self.applied = self.data.len() as u64;
                true
            }
        }
    }

    /// Returns a reference to the task ID associated with this cache.
    ///
    /// # Returns
    /// Immutable reference to the cache's task ID
    pub(crate) fn task_id(&self) -> &TaskId {
        &self.task_id
    }

    /// Returns the current size of the cached data.
    ///
    /// # Returns
    /// Size of the data in bytes
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Creates a cursor for reading the cached data.
    ///
    /// # Returns
    /// A new cursor positioned at the start of the cached data
    pub fn cursor(&self) -> Cursor<&[u8]> {
        Cursor::new(&self.data)
    }
}

impl Write for RamCache {
    /// Writes data to the cache.
    ///
    /// Delegates to the underlying Vec<u8> implementation.
    ///
    /// # Parameters
    /// - `buf`: Buffer containing the data to write
    ///
    /// # Returns
    /// Number of bytes written on success, or an error
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.write(buf)
    }

    /// Flushes the cache.
    ///
    /// Delegates to the underlying Vec<u8> implementation.
    /// Since this is an in-memory cache, this operation is a no-op.
    ///
    /// # Returns
    /// Ok(()) indicating success
    fn flush(&mut self) -> std::io::Result<()> {
        self.data.flush()
    }
}

impl CacheManager {
    /// Updates the RAM cache for a task.
    ///
    /// Inserts the cache into the manager's collection. If a previous cache exists for
    /// the same task, removes the associated file cache and logs the replacement.
    /// Also removes the task from the update-from-file tracking set.
    ///
    /// # Parameters
    /// - `cache`: The cache to update in the manager
    pub(crate) fn update_ram_cache(&'static self, cache: Arc<RamCache>) {
        let task_id = cache.task_id().clone();

        if self
            .rams
            .lock()
            .unwrap()
            .insert(task_id.clone(), cache.clone())
            .is_some()
        {
            // If there was a previous cache, remove associated file cache
            self.files.lock().unwrap().remove(&task_id);
            info!("{} old caches delete", task_id.brief());
        }
        // Prevent updating from file again for this task
        self.update_from_file_once.lock().unwrap().remove(&task_id);
    }
}

#[cfg(test)]
mod ut_ram {
    // Include test module containing unit tests for RamCache
    include!("../../tests/ut/data/ut_ram.rs");
}
