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

//! Cache management and coordination.
//!
//! This module provides the central `CacheManager` that coordinates different cache storage types
//! including RAM-based caches and file-based caches. It handles cache allocation, resource
//! management, and synchronization between different cache types.

use std::collections::{HashMap, HashSet};
use std::io;
use std::sync::{Arc, Mutex, OnceLock, Weak};

use request_utils::lru::LRUCache;
use request_utils::task_id::TaskId;

use super::data::{self, restore_files, FileCache, RamCache};
use crate::data::{init_curr_store_dir, MAX_CACHE_SIZE};

/// Default maximum size for RAM-based cache storage (20MB).
const DEFAULT_RAM_CACHE_SIZE: u64 = 1024 * 1024 * 20;

/// Default maximum size for file-based cache storage (100MB).
const DEFAULT_FILE_CACHE_SIZE: u64 = 1024 * 1024 * 100;

/// Central manager for coordinating different cache types and resources.
///
/// This struct manages RAM-based and file-based caches, handles resource allocation,
/// and provides methods for cache operations across different storage types.
/// It uses LRU (Least Recently Used) eviction policy for managing cache entries.
///
/// # Examples
///
/// ```rust
/// use cache_core::CacheManager;
/// use request_utils::task_id::TaskId;
///
/// // Create a new cache manager
/// let manager = CacheManager::new();
///
/// // Set custom cache sizes
/// manager.set_ram_cache_size(50 * 1024 * 1024); // 50MB
/// manager.set_file_cache_size(200 * 1024 * 1024); // 200MB
/// ```
pub struct CacheManager {
    /// Primary RAM cache storage using LRU eviction policy
    pub(crate) rams: Mutex<LRUCache<TaskId, Arc<RamCache>>>,

    /// Backup RAM cache storage not subject to LRU eviction
    pub(crate) backup_rams: Mutex<HashMap<TaskId, Arc<RamCache>>>,

    /// File-based cache storage using LRU eviction policy
    pub(crate) files: Mutex<LRUCache<TaskId, FileCache>>,

    /// Ensures each file-to-RAM update is performed only once
    pub(crate) update_from_file_once:
        Mutex<HashMap<TaskId, Arc<OnceLock<io::Result<Weak<RamCache>>>>>>,

    /// Manages RAM cache resource allocation and capacity
    pub(crate) ram_handle: Mutex<data::ResourceManager>,

    /// Manages file cache resource allocation and capacity
    pub(crate) file_handle: Mutex<data::ResourceManager>,
}

impl CacheManager {
    /// Creates a new cache manager with default cache sizes.
    ///
    /// Initializes RAM and file caches with default capacities of 20MB and 100MB respectively.
    ///
    /// # Returns
    /// A new CacheManager instance ready for use
    pub fn new() -> Self {
        Self {
            rams: Mutex::new(LRUCache::new()),
            files: Mutex::new(LRUCache::new()),
            backup_rams: Mutex::new(HashMap::new()),
            update_from_file_once: Mutex::new(HashMap::new()),

            ram_handle: Mutex::new(data::ResourceManager::new(DEFAULT_RAM_CACHE_SIZE)),
            file_handle: Mutex::new(data::ResourceManager::new(DEFAULT_FILE_CACHE_SIZE)),
        }
    }

    /// Sets the maximum size for RAM-based caching.
    ///
    /// Adjusts the total capacity for in-memory caching and triggers cache eviction
    /// if the new size requires releasing resources.
    ///
    /// # Parameters
    /// - `size`: New maximum RAM cache size in bytes
    pub fn set_ram_cache_size(&self, size: u64) {
        self.ram_handle.lock().unwrap().change_total_size(size);
        CacheManager::apply_cache(&self.ram_handle, &self.rams, 0);
    }

    /// Sets the maximum size for file-based caching.
    ///
    /// Adjusts the total capacity for file-based caching and triggers cache eviction
    /// if the new size requires releasing resources.
    ///
    /// # Parameters
    /// - `size`: New maximum file cache size in bytes
    pub fn set_file_cache_size(&self, size: u64) {
        self.file_handle.lock().unwrap().change_total_size(size);
        CacheManager::apply_cache(&self.file_handle, &self.files, 0);
    }

    /// Restores cached files from persistent storage.
    ///
    /// Initializes the current storage directory and restores all previously cached files
    /// into the manager's file cache.
    ///
    /// # Safety
    /// Must be called with a `'static self` reference as it may spawn background tasks
    /// that need to reference the manager.
    pub fn restore_files(&'static self) {
        init_curr_store_dir();
        if let Some(task_ids) = restore_files() {
            for task_id in task_ids {
                let Some(file_cache) = FileCache::try_restore(task_id.clone(), self) else {
                    continue;
                };
                self.files.lock().unwrap().insert(task_id, file_cache);
            }
        }
    }

    /// Fetches a cache entry by task ID.
    ///
    /// Retrieves a RAM cache for the given task ID, checking primary RAM cache, backup RAM cache,
    /// and falling back to loading from file cache if necessary.
    ///
    /// # Parameters
    /// - `task_id`: The task ID to fetch
    ///
    /// # Returns
    /// `Some(Arc<RamCache>)` if found, `None` otherwise
    ///
    /// # Safety
    /// Must be called with a `'static self` reference as it may load from file cache.
    pub fn fetch(&'static self, task_id: &TaskId) -> Option<Arc<RamCache>> {
        self.get_cache(task_id)
    }

    /// Removes a cache entry by task ID.
    ///
    /// Removes the entry from all cache storage types (file, backup RAM, and primary RAM cache),
    /// and clears any pending file-to-RAM update operations for the task.
    ///
    /// # Parameters
    /// - `task_id`: The task ID to remove
    pub fn remove(&self, task_id: TaskId) {
        self.files.lock().unwrap().remove(&task_id);
        self.backup_rams.lock().unwrap().remove(&task_id);
        self.rams.lock().unwrap().remove(&task_id);
        self.update_from_file_once.lock().unwrap().remove(&task_id);
    }

    /// Checks if a cache entry exists for the given task ID.
    ///
    /// Checks all cache storage types (file, backup RAM, and primary RAM cache).
    ///
    /// # Parameters
    /// - `task_id`: The task ID to check
    ///
    /// # Returns
    /// `true` if the task ID exists in any cache, `false` otherwise
    pub fn contains(&self, task_id: &TaskId) -> bool {
        self.files.lock().unwrap().contains_key(task_id)
            || self.backup_rams.lock().unwrap().contains_key(task_id)
            || self.rams.lock().unwrap().contains_key(task_id)
    }

    /// Internal method to get a cache entry with fallback logic.
    ///
    /// First checks the primary RAM cache, then the backup RAM cache, and finally
    /// attempts to load from file cache if necessary.
    ///
    /// # Parameters
    /// - `task_id`: The task ID to retrieve
    ///
    /// # Returns
    /// `Some(Arc<RamCache>)` if found through any cache source, `None` otherwise
    pub(crate) fn get_cache(&'static self, task_id: &TaskId) -> Option<Arc<RamCache>> {
        let res = self.rams.lock().unwrap().get(task_id).cloned();
        res.or_else(|| self.backup_rams.lock().unwrap().get(task_id).cloned())
            .or_else(|| self.update_ram_from_file(task_id))
    }

    /// Clears memory cache entries not associated with running tasks.
    pub fn clear_memory_cache(&self, running_tasks: &HashSet<TaskId>) {
        let ram_keys = self
            .rams
            .lock()
            .unwrap()
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        let key_to_remove = ram_keys
            .into_iter()
            .filter(|task_id| !running_tasks.contains(task_id))
            .collect::<Vec<_>>();
        let mut rams_to_remove = Vec::with_capacity(key_to_remove.len());
        // Do not delete the data of Arc during the time when the lock is held to reduce the time when the lock is held
        {
            let mut rams = self.rams.lock().unwrap();
            for key in key_to_remove {
                rams_to_remove.push(rams.remove(&key));
            }
        }
    }

    /// Clears file cache entries not associated with running tasks.
    pub fn clear_file_cache(&self, running_tasks: &HashSet<TaskId>) {
        let file_keys = self
            .files
            .lock()
            .unwrap()
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        let key_to_remove = file_keys
            .into_iter()
            .filter(|task_id| !running_tasks.contains(task_id))
            .collect::<Vec<_>>();
        let mut files_to_remove = Vec::with_capacity(key_to_remove.len());
        // Do not delete the data of FileCache during the time when the lock is held to reduce the time when the lock is held
        {
            let mut files = self.files.lock().unwrap();
            for key in key_to_remove {
                files_to_remove.push(files.remove(&key));
            }
        }
    }

    /// Attempts to allocate cache space, evicting entries if necessary.
    ///
    /// Tries to apply for the requested cache size, and if insufficient space is available,
    /// evicts the least recently used entries until enough space is freed or all entries
    /// have been evicted.
    ///
    /// # Type Parameters
    /// - `T`: The cache value type, can be either `RamCache` or `FileCache`
    ///
    /// # Parameters
    /// - `handle`: Resource manager controlling the cache capacity
    /// - `caches`: LRU cache to potentially evict entries from
    /// - `size`: Amount of space to allocate in bytes
    ///
    /// # Returns
    /// `true` if allocation succeeded, `false` if insufficient space even after eviction
    pub(super) fn apply_cache<T>(
        handle: &Mutex<data::ResourceManager>,
        caches: &Mutex<LRUCache<TaskId, T>>,
        size: usize,
    ) -> bool {
        loop {
            if size > MAX_CACHE_SIZE as usize {
                return false;
            }
            if handle.lock().unwrap().apply_cache_size(size as u64) {
                return true;
            };
            // No cache in caches - eviction failed
            if caches.lock().unwrap().pop().is_none() {
                info!("CacheManager release cache failed");
                return false;
            }
        }
    }
}

#[cfg(test)]
mod ut_manage {
    // Include test module containing unit tests for CacheManager
    include!("../tests/ut/ut_manage.rs");
}
