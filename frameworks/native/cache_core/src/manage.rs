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
//! This module provides the central `CacheManager` that coordinates different
//! cache storage types including RAM-based caches and file-based caches. It
//! handles cache allocation, resource management, and synchronization between
//! different cache types.

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::sync::{Arc, Condvar, Mutex, OnceLock, Weak};

use request_utils::lru::LRUCache;
use request_utils::task_id::TaskId;

use super::data::{
    get_cached_files_info, FileCache, FileCacheInfo, RamCache, SpaceManager, MAX_CACHE_SIZE,
};
use crate::spawn;

/// Default maximum size for RAM-based cache storage (20MB).
const DEFAULT_RAM_CACHE_SIZE: u64 = 1024 * 1024 * 20;

/// Default maximum size for file-based cache storage (100MB).
const DEFAULT_FILE_CACHE_SIZE: u64 = 1024 * 1024 * 100;

pub(crate) struct FileCaches {
    /// File-based cache storage using LRU eviction policy
    pub(crate) files: LRUCache<TaskId, Arc<Mutex<FileCache>>>,

    /// Manages file cache resource allocation and capacity
    pub(crate) file_space: SpaceManager,

    pub(crate) operations: HashMap<TaskId, Arc<Mutex<VecDeque<Arc<NotifyCondition>>>>>,
}

impl FileCaches {
    pub(crate) fn new() -> Self {
        Self {
            files: LRUCache::new(),
            file_space: SpaceManager::new(DEFAULT_FILE_CACHE_SIZE),
            operations: HashMap::new(),
        }
    }

    pub(crate) fn remove(&mut self, task_id: &TaskId) -> Option<OperatingTask> {
        if let Some(file_cache) = self.files.remove(task_id) {
            self.file_space.release(file_cache.lock().unwrap().size());
            let operations = self.get_operations(task_id);
            Some(OperatingTask::new(task_id.clone(), operations))
        } else {
            None
        }
    }

    pub(crate) fn change_total_size(&mut self, size: u64) {
        self.file_space.change_total_size(size);
    }

    pub(crate) fn contains(&self, task_id: &TaskId) -> bool {
        self.files.contains_key(task_id)
    }

    pub(crate) fn try_restore_file(&mut self, info: &FileCacheInfo) -> bool {
        if self.files.contains_key(info.task_id()) {
            return true;
        }
        let size = info.size();
        if self.file_space.apply_cache_size(size) {
            let cache = Arc::new(Mutex::new(FileCache::new(info.task_id().clone(), size)));
            self.files.insert(info.task_id().clone(), cache);
            return true;
        }
        info!(
            "restore file for {} failed, size: {}",
            info.task_id().brief(),
            size
        );
        false
    }

    pub(crate) fn try_apply_size(&mut self, apply: u64) -> (bool, Vec<OperatingTask>) {
        let mut removed = Vec::new();
        if apply > MAX_CACHE_SIZE {
            return (false, removed);
        }
        loop {
            if self.file_space.apply_cache_size(apply) {
                return (true, removed);
            };
            // No cache in caches - eviction failed
            match self.files.pop() {
                Some(cache) => {
                    let index = cache.lock().unwrap();
                    let size = index.size();
                    let id = index.task_id().clone();
                    let operations = self.get_operations(&id);
                    drop(index);
                    self.file_space.release(size);
                    removed.push(OperatingTask::new(id, operations));
                }
                None => {
                    info!("CacheManager apply cache failed");
                    return (false, removed);
                }
            }
        }
    }

    pub(crate) fn task_ids(&self) -> Vec<TaskId> {
        self.files.keys().cloned().collect()
    }

    pub(crate) fn get_operations(
        &mut self,
        task_id: &TaskId,
    ) -> Arc<Mutex<VecDeque<Arc<NotifyCondition>>>> {
        self.operations
            .entry(task_id.clone())
            .or_insert_with(|| Arc::new(Mutex::new(VecDeque::new())))
            .clone()
    }
}

pub struct FileManager {
    pub(crate) caches: Mutex<FileCaches>,

    /// Ensures each file-to-RAM update is performed only once
    pub(crate) update_from_file_once:
        Mutex<HashMap<TaskId, Arc<OnceLock<io::Result<Weak<RamCache>>>>>>,

    /// Backup RAM cache storage not subject to LRU eviction
    pub(crate) backup_rams: Mutex<HashMap<TaskId, Arc<RamCache>>>,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            caches: Mutex::new(FileCaches::new()),
            update_from_file_once: Mutex::new(HashMap::new()),
            backup_rams: Mutex::new(HashMap::new()),
        }
    }

    pub fn set_file_cache_size(&self, size: u64) {
        let mut caches = self.caches.lock().unwrap();
        caches.change_total_size(size);
        let (_, removed) = caches.try_apply_size(0);
        let mut remove_others = vec![];
        for task in removed.iter() {
            let handle = send_operation_message(task);
            remove_others.push(handle);
        }
        drop(caches);
        for (index, task) in removed.into_iter().enumerate() {
            execute_file_remove(task, &remove_others[index]);
        }
    }

    pub(crate) fn clear_file_cache(&self, running_tasks: &HashSet<TaskId>) {
        let mut caches = self.caches.lock().unwrap();
        let mut remove_handles = vec![];
        for id in caches.task_ids() {
            if !running_tasks.contains(&id) {
                if let Some(task) = caches.remove(&id) {
                    let handle = send_operation_message(&task);
                    remove_handles.push((handle, task));
                }
            }
        }
        drop(caches);
        for (handle, task) in remove_handles {
            execute_file_remove(task, &handle);
        }
    }

    pub(crate) fn contains(&self, task_id: &TaskId) -> bool {
        self.caches.lock().unwrap().contains(task_id)
            || self.backup_rams.lock().unwrap().contains_key(task_id)
    }

    pub(crate) fn update_ram_from_file(
        &'static self,
        task_id: &TaskId,
        handle: &'static CacheManager,
    ) -> Option<Arc<RamCache>> {
        let mut retry = false;
        // Loop with retry logic for concurrent operations
        loop {
            let ret = self.update_ram_from_file_inner(task_id, &mut retry, handle);
            if !retry || ret.is_some() {
                break ret;
            } else {
                // Clear the once lock to retry
                self.update_from_file_once.lock().unwrap().remove(task_id);
            }
        }
    }

    pub(crate) fn update_ram_from_file_inner(
        &'static self,
        task_id: &TaskId,
        retry: &mut bool,
        handle: &'static CacheManager,
    ) -> Option<Arc<RamCache>> {
        *retry = false;

        // Get or create a OnceLock for this task
        let once = match self
            .update_from_file_once
            .lock()
            .unwrap()
            .entry(task_id.clone())
        {
            Entry::Occupied(entry) => entry.into_mut().clone(),
            Entry::Vacant(entry) => {
                // Check if the cache is already in RAM
                let res = self.backup_rams.lock().unwrap().get(task_id).cloned();
                if res.is_some() {
                    return res;
                } else {
                    // Create a new OnceLock for this task
                    entry.insert(Arc::new(OnceLock::new())).clone()
                }
            }
        };

        // Storage for the result
        let mut ret = None;

        // Use get_or_init to ensure the file is only loaded once
        let res = once.get_or_init(|| {
            debug!("{} ram updated from file", task_id.brief());

            let mut caches = self.caches.lock().unwrap();
            if !caches.files.contains_key(task_id) {
                return Err(io::Error::new(io::ErrorKind::NotFound, "not found"));
            }
            let operating_task =
                OperatingTask::new(task_id.clone(), caches.get_operations(task_id));
            let notify = send_operation_message(&operating_task);
            drop(caches);

            let mut ram = execute_file_read(operating_task, &notify, handle)?;
            // Check if the cache size is valid
            let is_cache = ram.check_size();
            let cache = Arc::new(ram);

            // Update the RAM cache if valid
            if is_cache {
                handle.update_ram_cache(cache.clone());
            }

            // Store the result and return a weak reference
            ret = Some(cache.clone());
            let weak_cache = Arc::downgrade(&cache);
            Ok(weak_cache)
        });

        // If we have a direct result, return it
        if ret.is_some() {
            return ret;
        }
        // Try to upgrade the weak reference
        res.as_ref().ok().and_then(|weak| {
            *retry = true;
            Weak::upgrade(weak)
        })
    }

    pub fn remove(&self, task_id: &TaskId) {
        self.backup_rams.lock().unwrap().remove(task_id);
        let mut caches = self.caches.lock().unwrap();
        if let Some(task) = caches.remove(task_id) {
            let handle = send_operation_message(&task);
            drop(caches);
            execute_file_remove(task, &handle);
        }
        self.update_from_file_once.lock().unwrap().remove(task_id);
    }

    pub(crate) fn try_remove_from_disk(&'static self, task_id: &TaskId) {
        let mut caches = self.caches.lock().unwrap();
        if caches.contains(task_id) {
            return;
        }
        let operating_task = OperatingTask::new(task_id.clone(), caches.get_operations(task_id));
        let notify = send_operation_message(&operating_task);
        drop(caches);
        execute_file_remove(operating_task, &notify);
    }

    pub(crate) fn try_restore_file(&self, info: &FileCacheInfo) -> bool {
        let mut cache = self.caches.lock().unwrap();
        let success = cache.try_restore_file(info);
        if !success {
            let task =
                OperatingTask::new(info.task_id().clone(), cache.get_operations(info.task_id()));
            let notify = send_operation_message(&task);
            drop(cache);
            execute_file_remove(task, &notify);
        }
        success
    }

    pub(super) fn update_file_cache(&'static self, task_id: TaskId, cache: Arc<RamCache>) {
        // Remove any existing update operation for this task
        self.update_from_file_once.lock().unwrap().remove(&task_id);
        // Store backup of RAM cache
        self.backup_rams
            .lock()
            .unwrap()
            .insert(task_id.clone(), cache.clone());

        // Spawn background task to perform the file write
        spawn(move || {
            let mut caches = self.caches.lock().unwrap();
            let mut remove_curr = None;
            if let Some(task) = caches.remove(&task_id) {
                let handle = send_operation_message(&task);
                remove_curr = Some((task, handle));
            }
            let (success, removed) = caches.try_apply_size(cache.size() as u64);
            let mut remove_others = vec![];
            for task in removed.iter() {
                let handle = send_operation_message(task);
                remove_others.push(handle);
            }
            let mut insert_curr = None;
            if success {
                caches.files.insert(
                    task_id.clone(),
                    Arc::new(Mutex::new(FileCache::new(
                        task_id.clone(),
                        cache.size() as u64,
                    ))),
                );
                let task = OperatingTask::new(task_id.clone(), caches.get_operations(&task_id));
                let handle = send_operation_message(&task);
                insert_curr = Some((task, handle));
            }
            drop(caches);
            if let Some((task, handle)) = remove_curr {
                execute_file_remove(task, &handle);
            }
            for (idx, task) in removed.into_iter().enumerate() {
                execute_file_remove(task, &remove_others[idx]);
            }
            if let Some((task, handle)) = insert_curr {
                execute_file_write(task, cache, &handle);
            }
            // Clean up backup RAM cache after file update
            self.backup_rams.lock().unwrap().remove(&task_id);
        });
    }
}

pub struct OperatingTask {
    pub(crate) task_id: TaskId,
    pub(crate) operations: Arc<Mutex<VecDeque<Arc<NotifyCondition>>>>,
}

impl OperatingTask {
    pub fn new(task_id: TaskId, operations: Arc<Mutex<VecDeque<Arc<NotifyCondition>>>>) -> Self {
        Self {
            task_id,
            operations,
        }
    }

    pub fn task_id(&self) -> &TaskId {
        &self.task_id
    }
}

pub struct NotifyHandle {
    is_first: bool,
    handle: Arc<NotifyCondition>,
}

impl Clone for NotifyHandle {
    fn clone(&self) -> Self {
        Self {
            is_first: self.is_first,
            handle: self.handle.clone(),
        }
    }
}

impl NotifyHandle {
    pub fn new(is_first: bool, handle: Arc<NotifyCondition>) -> NotifyHandle {
        Self { is_first, handle }
    }

    pub fn is_first(&self) -> bool {
        self.is_first
    }

    pub fn handle(&self) -> &Arc<NotifyCondition> {
        &self.handle
    }
}

pub struct NotifyCondition {
    available: Mutex<bool>,
    condvar: Condvar,
}

impl NotifyCondition {
    pub fn new() -> Self {
        Self {
            available: Mutex::new(false),
            condvar: Condvar::new(),
        }
    }

    pub fn wait(&self) {
        let mut available = self.available.lock().unwrap();
        while !*available {
            available = self.condvar.wait(available).unwrap();
        }
        *available = false;
    }

    pub fn notify(&self) {
        *(self.available.lock().unwrap()) = true;
        self.condvar.notify_one();
    }
}

/// Central manager for coordinating different cache types and resources.
///
/// This struct manages RAM-based and file-based caches, handles resource
/// allocation, and provides methods for cache operations across different
/// storage types. It uses LRU (Least Recently Used) eviction policy for
/// managing cache entries.
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

    /// Manages RAM cache resource allocation and capacity
    pub(crate) ram_handle: Mutex<SpaceManager>,

    /// Manages file cache resource allocation and capacity
    pub(crate) file_manager: FileManager,
}

impl CacheManager {
    /// Creates a new cache manager with default cache sizes.
    ///
    /// Initializes RAM and file caches with default capacities of 20MB and
    /// 100MB respectively.
    ///
    /// # Returns
    /// A new CacheManager instance ready for use
    pub fn new() -> Self {
        Self {
            rams: Mutex::new(LRUCache::new()),
            ram_handle: Mutex::new(SpaceManager::new(DEFAULT_RAM_CACHE_SIZE)),
            file_manager: FileManager::new(),
        }
    }

    /// Sets the maximum size for RAM-based caching.
    ///
    /// Adjusts the total capacity for in-memory caching and triggers cache
    /// eviction if the new size requires releasing resources.
    ///
    /// # Parameters
    /// - `size`: New maximum RAM cache size in bytes
    pub fn set_ram_cache_size(&self, size: u64) {
        self.ram_handle.lock().unwrap().change_total_size(size);
        CacheManager::apply_cache(&self.ram_handle, &self.rams, 0);
    }

    /// Sets the maximum size for file-based caching.
    ///
    /// Adjusts the total capacity for file-based caching and triggers cache
    /// eviction if the new size requires releasing resources.
    ///
    /// # Parameters
    /// - `size`: New maximum file cache size in bytes
    pub fn set_file_cache_size(&self, size: u64) {
        self.file_manager.set_file_cache_size(size);
    }

    /// Restores all valid cache files from the given directory.
    ///
    /// Scans the directory for valid cache files, filters out incomplete files,
    /// sorts them by modification time, and returns an iterator over the task
    /// IDs.
    ///
    /// # Parameters
    /// - `path`: Path to the directory to scan
    ///
    /// # Returns
    /// Iterator over task IDs of valid cache files
    pub fn build_cached_files_index(&'static self) {
        if let Some(file_info) = get_cached_files_info() {
            let mut is_continue = true;
            for info in file_info {
                if !is_continue {
                    self.file_manager.try_remove_from_disk(info.task_id());
                    continue;
                }
                if !self.file_manager.try_restore_file(&info) {
                    is_continue = false;
                    continue;
                }
            }
        }
    }

    /// Fetches a cache entry by task ID.
    ///
    /// Retrieves a RAM cache for the given task ID, checking primary RAM cache,
    /// backup RAM cache, and falling back to loading from file cache if
    /// necessary.
    ///
    /// # Parameters
    /// - `task_id`: The task ID to fetch
    ///
    /// # Returns
    /// `Some(Arc<RamCache>)` if found, `None` otherwise
    ///
    /// # Safety
    /// Must be called with a `'static self` reference as it may load from file
    /// cache.
    pub fn fetch(&'static self, task_id: &TaskId) -> Option<Arc<RamCache>> {
        self.get_cache(task_id)
    }

    /// Removes a cache entry by task ID.
    ///
    /// Removes the entry from all cache storage types (file, backup RAM, and
    /// primary RAM cache), and clears any pending file-to-RAM update
    /// operations for the task.
    ///
    /// # Parameters
    /// - `task_id`: The task ID to remove
    pub fn remove(&self, task_id: TaskId) {
        self.file_manager.remove(&task_id);
        self.rams.lock().unwrap().remove(&task_id);
    }

    /// Checks if a cache entry exists for the given task ID.
    ///
    /// Checks all cache storage types (file, backup RAM, and primary RAM
    /// cache).
    ///
    /// # Parameters
    /// - `task_id`: The task ID to check
    ///
    /// # Returns
    /// `true` if the task ID exists in any cache, `false` otherwise
    pub fn contains(&self, task_id: &TaskId) -> bool {
        self.file_manager.contains(task_id) || self.rams.lock().unwrap().contains_key(task_id)
    }

    /// Internal method to get a cache entry with fallback logic.
    ///
    /// First checks the primary RAM cache, then the backup RAM cache, and
    /// finally attempts to load from file cache if necessary.
    ///
    /// # Parameters
    /// - `task_id`: The task ID to retrieve
    ///
    /// # Returns
    /// `Some(Arc<RamCache>)` if found through any cache source, `None`
    /// otherwise
    pub(crate) fn get_cache(&'static self, task_id: &TaskId) -> Option<Arc<RamCache>> {
        let res = self.rams.lock().unwrap().get(task_id).cloned();
        res.or_else(|| {
            self.file_manager
                .backup_rams
                .lock()
                .unwrap()
                .get(task_id)
                .cloned()
        })
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
        // Do not delete the data of Arc during the time when the lock is held to reduce
        // the time when the lock is held
        {
            let mut rams = self.rams.lock().unwrap();
            for key in key_to_remove {
                rams_to_remove.push(rams.remove(&key));
            }
        }
    }

    /// Clears file cache entries not associated with running tasks.
    pub fn clear_file_cache(&self, running_tasks: &HashSet<TaskId>) {
        self.file_manager.clear_file_cache(running_tasks);
    }

    pub fn read_task_local_file(&'static self, task_id: &TaskId) -> Option<RamCache> {
        FileCache::read_but_not_cache(task_id, self).ok()
    }

    pub(super) fn update_file_cache(&'static self, task_id: TaskId, cache: Arc<RamCache>) {
        self.file_manager.update_file_cache(task_id, cache);
    }

    /// Updates the RAM cache from the file cache for a given task.
    ///
    /// Reads data from the file cache and loads it into RAM, with retry logic
    /// to handle concurrent access scenarios.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to update
    ///
    /// # Returns
    /// `Some(Arc<RamCache>)` if successful, `None` if the file doesn't exist or
    /// can't be read
    pub(crate) fn update_ram_from_file(&'static self, task_id: &TaskId) -> Option<Arc<RamCache>> {
        self.file_manager.update_ram_from_file(task_id, self)
    }

    /// Attempts to allocate cache space, evicting entries if necessary.
    ///
    /// Tries to apply for the requested cache size, and if insufficient space
    /// is available, evicts the least recently used entries until enough
    /// space is freed or all entries have been evicted.
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
    /// `true` if allocation succeeded, `false` if insufficient space even after
    /// eviction
    pub(super) fn apply_cache<T>(
        handle: &Mutex<SpaceManager>,
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

    pub(crate) fn update_ram_cache(&'static self, cache: Arc<RamCache>) {
        let task_id = cache.task_id().clone();

        self.rams
            .lock()
            .unwrap()
            .insert(task_id.clone(), cache.clone());
    }
}

pub fn send_operation_message(task: &OperatingTask) -> NotifyHandle {
    let pair = Arc::new(NotifyCondition::new());
    let pair2 = pair.clone();
    let mut is_first = false;
    let mut operations = task.operations.lock().unwrap();
    if operations.is_empty() {
        is_first = true;
    }
    operations.push_back(pair);
    NotifyHandle::new(is_first, pair2)
}

pub fn execute_file_remove(task: OperatingTask, notify: &NotifyHandle) {
    let is_first = notify.is_first();
    let pair = notify.handle();
    if is_first {
        FileCache::remove_file(task.task_id());
    } else {
        pair.wait();
        FileCache::remove_file(task.task_id());
    }
    notify_next_operation(task);
}

pub fn execute_file_read(
    task: OperatingTask,
    notify: &NotifyHandle,
    handle: &'static CacheManager,
) -> Result<RamCache, io::Error> {
    let is_first = notify.is_first();
    let pair = notify.handle();
    let ram = if is_first {
        FileCache::read(task.task_id(), handle)
    } else {
        pair.wait();
        FileCache::read(task.task_id(), handle)
    };
    notify_next_operation(task);
    ram
}

pub fn execute_file_write(task: OperatingTask, ram: Arc<RamCache>, notify: &NotifyHandle) {
    let is_first = notify.is_first();
    let pair = notify.handle();
    if is_first {
        if let Err(e) = FileCache::create_file(task.task_id(), ram) {
            error!("{} create file error: {}", task.task_id().brief(), e);
        }
    } else {
        pair.wait();
        if let Err(e) = FileCache::create_file(task.task_id(), ram) {
            error!("{} create file error: {}", task.task_id().brief(), e);
        }
    }
    notify_next_operation(task);
}

pub fn notify_next_operation(task: OperatingTask) {
    let mut operations = task.operations.lock().unwrap();
    operations.pop_front();
    if let Some(message) = operations.get_mut(0) {
        message.notify();
    }
}

#[cfg(test)]
mod ut_manage {
    // Include test module containing unit tests for CacheManager
    include!("../tests/ut/ut_manage.rs");
}
