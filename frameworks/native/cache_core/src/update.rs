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

//! Cache update and synchronization operations.
//! 
//! This module provides the `Updater` struct for managing cache updates, including
//! receiving data, finalizing cache entries, and resetting cache state. It handles
//! the synchronization between incoming data and the caching system.

use std::io::Write;
use std::sync::Arc;

use request_utils::task_id::TaskId;

use crate::data::RamCache;
use crate::manage::CacheManager;

// Previous version of Updater struct (commented out)
// pub(crate) struct Updater {
//     pub(crate) remove_flag: bool
//     pub(crate) seq: usize,
//     pub(crate) handle: TaskHandle,
// }

/// Manages cache updates for a specific task.
///
/// This struct handles the process of receiving data, storing it in a RAM cache,
/// and finalizing the cache entry. It provides methods for incremental data
/// updates and cache lifecycle management.
///
/// # Examples
///
/// ```rust
/// use cache_core::{CacheManager, Updater};
/// use request_utils::task_id::TaskId;
///
/// // Create a cache manager and updater
/// let manager = CacheManager::new();
/// let task_id = TaskId::new("example_task".to_string());
/// let mut updater = Updater::new(task_id, &manager);
///
/// // Receive data in chunks
/// updater.cache_receive(b"Hello, ", || Some(13));
/// updater.cache_receive(b"world!", || None);
///
/// // Finalize the cache
/// let final_cache = updater.cache_finish();
/// ```
pub struct Updater {
    /// Unique identifier for the task being updated
    task_id: TaskId,
    
    /// Optional RAM cache for storing received data
    cache: Option<RamCache>,
    
    /// Reference to the global cache manager
    cache_manager: &'static CacheManager,
}

impl Updater {
    /// Creates a new updater for the specified task.
    ///
    /// Initializes an updater with the given task ID and cache manager reference,
    /// ready to receive data for caching.
    ///
    /// # Parameters
    /// - `task_id`: Unique identifier for the task
    /// - `cache_manager`: Static reference to the cache manager
    ///
    /// # Returns
    /// A new Updater instance
    pub fn new(task_id: TaskId, cache_manager: &'static CacheManager) -> Self {
        Self {
            task_id,
            cache: None,
            cache_manager,
        }
    }

    /// Finalizes the cache and returns an Arc-wrapped RamCache.
    ///
    /// Completes the write operation on the cache and returns it wrapped in an Arc
    /// for shared ownership. If no cache was created, initializes an empty one.
    ///
    /// # Returns
    /// An Arc-wrapped RamCache instance containing the cached data
    pub fn cache_finish(&mut self) -> Arc<RamCache> {
        match self.cache.take() {
            Some(cache) => cache.finish_write(),
            None => Arc::new(RamCache::new(
                self.task_id.clone(),
                self.cache_manager,
                Some(0),
            )),
        }
    }

    /// Receives and caches a chunk of data.
    ///
    /// Initializes the cache if it doesn't exist yet, using the provided content length
    /// for allocation. Writes the data to the cache and logs any errors.
    ///
    /// # Type Parameters
    /// - `F`: Function that returns the optional content length
    ///
    /// # Parameters
    /// - `data`: Data chunk to write to the cache
    /// - `content_length`: Function that provides the total content length (if known)
    pub fn cache_receive<F>(&mut self, data: &[u8], content_length: F)
    where
        F: FnOnce() -> Option<usize>,
    {
        // Initialize cache on first data reception
        if self.cache.is_none() {
            let content_length = content_length();
            let apply_cache = 
                RamCache::new(self.task_id.clone(), self.cache_manager, content_length);
            self.cache = Some(apply_cache)
        }
        
        // Write data to cache and log errors without panicking
        if let Err(e) = self.cache.as_mut().unwrap().write_all(data) {
            error!("{} cache write error: {}", self.task_id.brief(), e);
        };
    }

    /// Resets the cache, releasing its resources.
    ///
    /// Takes ownership of the current cache if it contains data, effectively
    /// clearing it and releasing associated resources.
    pub fn reset_cache(&mut self) {
        let size = self.cache.as_ref().map(|a| a.size()).unwrap_or(0);
        if size != 0 {
            info!("reset {} cache size {}", self.task_id.brief(), size);
            self.cache.take();
        }
    }
}
