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

#![warn(unused)]

//! File-based cache implementation for task data.
//! 
//! This module provides functionality for managing file-based caches, including:
//! - Directory management for cache storage
//! - File cache creation, restoration, and deletion
//! - Synchronization between RAM and disk storage
//! - Directory observation for cache maintenance
//! 
//! The implementation ensures thread-safe access to cache resources and provides
//! mechanisms for persisting data across application restarts.

use std::collections::hash_map::Entry;
use std::fs::{self, DirEntry, File, OpenOptions};
use std::io::{self, Seek, Write};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Arc, Mutex, Once, OnceLock, Weak};
use std::time::SystemTime;

use request_utils::task_id::TaskId;

use super::ram::RamCache;
use crate::manage::CacheManager;
use crate::spawn;

/// Suffix appended to files that are fully written and finalized.
///
/// This suffix is used to indicate that a cache file has been completely written
/// and is ready for use. Files without this suffix may be incomplete and are considered
/// invalid.
const FINISH_SUFFIX: &str = "_F";

/// Global file store directory manager.
///
/// This static variable manages the directories used for storing cache files. It is
/// initialized on first use through the `init_history_store_dir` and `init_curr_store_dir`
/// functions.
pub(crate) static mut FILE_STORE_DIR: FileStoreDir = FileStoreDir::new();

/// One-time initialization flag for history directory.
///
/// Ensures the history directory is initialized exactly once across all threads.
static INIT_HISTORY: Once = Once::new();

/// One-time initialization flag for current directory.
///
/// Ensures the current directory is initialized exactly once across all threads.
static INIT_CURR: Once = Once::new();

/// Initializes the history directory for cache storage.
///
/// Sets up the history directory with the provided `HistoryDir` instance and
/// starts directory observation using the given spawner function.
///
/// # Parameters
/// - `history`: The history directory to use for cache storage
/// - `spawner`: Function to spawn directory observation process
///
/// # Safety
/// This function is thread-safe and will only initialize the history directory once.
pub fn init_history_store_dir(history: Arc<HistoryDir>, spawner: fn(PathBuf, Arc<HistoryDir>)) {
    INIT_HISTORY.call_once(|| {
        {
            // Get current directory and start observation
            let curr_dir = get_curr_store_dir();
            let mut is_observe = history.is_observe.lock().unwrap();
            spawner(curr_dir, history.clone());
            *is_observe = true;
        }
        // SAFETY: This is the only place where FILE_STORE_DIR is modified concurrently,
        // and it's protected by INIT_HISTORY which ensures it's initialized exactly once.
        unsafe {
            FILE_STORE_DIR.set_history_dir(history, spawner);
        }
    });
}

/// Initializes the current directory for cache storage.
///
/// Sets up the current directory where cache files will be stored.
///
/// # Safety
/// This function is thread-safe and will only initialize the current directory once.
pub fn init_curr_store_dir() {
    INIT_CURR.call_once(|| {
        let curr_dir = get_curr_store_dir();
        // SAFETY: This is the only place where FILE_STORE_DIR's curr field is modified concurrently,
        // and it's protected by INIT_CURR which ensures it's initialized exactly once.
        unsafe {
            FILE_STORE_DIR.set_curr_dir(curr_dir);
        }
    });
}

/// Gets the path to the current cache directory.
///
/// Returns the path to the directory where cache files are stored. On OpenHarmony
/// systems, it uses the application's cache directory, falling back to a default path
/// if that fails. On other systems, it uses the current directory.
///
/// # Returns
/// Path to the cache directory
///
/// # Notes
/// This function creates the directory if it doesn't exist.
pub fn get_curr_store_dir() -> PathBuf {
    #[cfg(feature = "ohos")]
    let mut path = match request_utils::context::get_cache_dir() {
        Some(dir) => PathBuf::from_str(&dir).unwrap(),
        None => {
            error!("get cache dir failed");
            // Fallback to standard cache directory if context retrieval fails
            PathBuf::from_str("/data/storage/el2/base/cache").unwrap()
        }
    };
    #[cfg(not(feature = "ohos"))]
    let mut path = PathBuf::from_str("./").unwrap();

    path.push("preload_caches");
    // Ensure the directory exists
    if let Err(e) = fs::create_dir_all(path.as_path()) {
        error!("create cache dir error {}", e);
    }
    path
}

/// Checks if the history directory has been initialized.
///
/// # Returns
/// `true` if the history directory has been initialized, `false` otherwise
///
/// # Safety
/// This function only performs a read operation on the FILE_STORE_DIR, which is safe.
pub fn is_history_init() -> bool {
    // SAFETY: This is a read-only operation on FILE_STORE_DIR, which is thread-safe.
    unsafe { FILE_STORE_DIR.history.is_some() }
}

/// Manages directories used for storing cache files.
///
/// This struct keeps track of both the current and history directories used for
/// storing cache files, providing methods to check existence, join paths, and ensure
/// directories are created when needed.
pub struct FileStoreDir {
    /// History directory for file caching
    history: Option<DirObservSpawner>,
    /// Current directory for file caching
    curr: Option<PathBuf>,
}

impl FileStoreDir {
    /// Creates a new empty FileStoreDir.
    ///
    /// Both history and current directories are initialized as None.
    pub const fn new() -> Self {
        Self {
            history: None,
            curr: None,
        }
    }

    /// Sets the history directory for file caching.
    ///
    /// # Parameters
    /// - `history`: The history directory to use
    /// - `spawner`: Function to spawn directory observation process
    pub fn set_history_dir(
        &mut self,
        history: Arc<HistoryDir>,
        spawner: fn(PathBuf, Arc<HistoryDir>),
    ) {
        self.history = Some(DirObservSpawner::new(history, spawner));
    }

    /// Sets the current directory for file caching.
    ///
    /// # Parameters
    /// - `curr`: Path to the current directory
    pub fn set_curr_dir(&mut self, curr: PathBuf) {
        self.curr = Some(curr);
    }

    /// Gets a reference to the current directory path.
    ///
    /// # Safety
    /// This method assumes that curr is not None, which is guaranteed by init_curr_store_dir.
    fn curr(&self) -> &PathBuf {
        self.curr.as_ref().unwrap()
    }

    /// Checks if the directory exists and creates it if necessary.
    ///
    /// Ensures both history and current directories exist, creating them if needed.
    /// Also starts directory observation if the history directory was just created.
    ///
    /// # Returns
    /// `true` if the directories exist (or were created successfully), `false` otherwise
    pub(crate) fn exist(&self) -> bool {
        // Ensure history directory exists
        if let Some(ref history) = self.history {
            if !history.exist() && history.create() {
                history.spawn_observe(self.curr().clone());
            }
        }
        // Ensure current directory exists
        if !self.curr().is_dir() {
            if let Err(e) = fs::create_dir_all(self.curr().as_path()) {
                error!("try create current cache dir error {}", e);
                return false;
            }
        }
        true
    }

    /// Joins a path to the current directory.
    ///
    /// Ensures the directory exists before joining.
    ///
    /// # Parameters
    /// - `path`: Path to join with the current directory
    ///
    /// # Returns
    /// Joined path if the directory exists, None otherwise
    pub(crate) fn join(&self, path: String) -> Option<PathBuf> {
        if self.exist() {
            Some(self.curr().join(path))
        } else {
            None
        }
    }

    /// Gets a reference to the current directory path.
    ///
    /// Ensures the directory exists before returning.
    ///
    /// # Returns
    /// Reference to the current directory path if it exists, None otherwise
    pub(crate) fn as_path(&self) -> Option<&Path> {
        if self.exist() {
            Some(self.curr().as_path())
        } else {
            None
        }
    }
}

/// Manages directory observation for cache maintenance.
///
/// Combines a history directory with a function to spawn directory observation,
/// allowing for automatic monitoring of cache directories.
pub(crate) struct DirObservSpawner {
    /// History directory to observe
    history: Arc<HistoryDir>,
    /// Function to spawn directory observation
    spawner: fn(PathBuf, Arc<HistoryDir>),
}

impl DirObservSpawner {
    /// Creates a new DirObservSpawner.
    ///
    /// # Parameters
    /// - `history`: History directory to observe
    /// - `spawner`: Function to spawn directory observation process
    pub(crate) fn new(history: Arc<HistoryDir>, spawner: fn(PathBuf, Arc<HistoryDir>)) -> Self {
        Self { history, spawner }
    }

    /// Checks if the history directory exists.
    ///
    /// # Returns
    /// `true` if the history directory exists, `false` otherwise
    pub(crate) fn exist(&self) -> bool {
        self.history.exist()
    }

    /// Creates the history directory if it doesn't exist.
    ///
    /// # Returns
    /// `true` if the directory was created successfully, `false` otherwise
    pub fn create(&self) -> bool {
        self.history.create()
    }

    /// Spawns directory observation if not already observing.
    ///
    /// Starts the directory observation process for the history directory.
    ///
    /// # Parameters
    /// - `curr`: Current directory path to pass to the spawner
    pub fn spawn_observe(&self, curr: PathBuf) {
        let mut is_observe = self.history.is_observe.lock().unwrap();
        if !*is_observe {
            // Only spawn observation if not already observing
            (self.spawner)(curr, self.history.clone());
            *is_observe = true;
        }
    }
}

/// Represents a history directory for cache storage.
///
/// This struct manages a directory used for storing historical cache data,
/// with a flag to track whether the directory is being observed.
pub struct HistoryDir {
    /// Path to the history directory
    dir: PathBuf,
    /// Mutex-protected flag indicating if the directory is being observed
    pub is_observe: Mutex<bool>,
}

impl HistoryDir {
    /// Creates a new HistoryDir with the specified path.
    ///
    /// # Parameters
    /// - `dir`: Path to the history directory
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir,
            is_observe: Mutex::new(false),
        }
    }

    /// Checks if the history directory exists.
    ///
    /// # Returns
    /// `true` if the directory exists, `false` otherwise
    pub fn exist(&self) -> bool {
        self.dir.is_dir()
    }

    /// Creates the history directory if it doesn't exist.
    ///
    /// # Returns
    /// `true` if the directory was created successfully, `false` otherwise
    pub fn create(&self) -> bool {
        if let Err(e) = fs::create_dir_all(self.dir.as_path()) {
            error!("try create history dir error {}", e);
            false
        } else {
            true
        }
    }

    /// Stops directory observation.
    ///
    /// Sets the observation flag to false, indicating that the directory
    /// is no longer being monitored.
    pub fn stop_observe(&self) {
        let mut is_observe = self.is_observe.lock().unwrap();
        *is_observe = false;
    }

    /// Gets the string representation of the directory path.
    ///
    /// # Returns
    /// String representation of the path if valid UTF-8, None otherwise
    pub fn dir_path(&self) -> Option<&str> {
        self.dir.to_str()
    }
}

/// Represents a file-based cache for a specific task.
///
/// This struct manages a cache file associated with a task ID, handling
/// creation, access, and cleanup of the cache file.
pub(crate) struct FileCache {
    /// ID of the task associated with this cache
    task_id: TaskId,
    /// Reference to the cache manager
    handle: &'static CacheManager,
}

impl Drop for FileCache {
    /// Cleans up the cache file when the FileCache is dropped.
    ///
    /// Removes the cache file from disk and releases the associated memory.
    fn drop(&mut self) {
        // Inner function to handle the actual cleanup with proper error handling
        fn drop_inner(me: &mut FileCache) -> Result<(), io::Error> {
            if let Some(path) = FileCache::path(&me.task_id) {
                let metadata = fs::metadata(&path)?;
                debug!(
                    "try drop file cache {} for task {}",
                    metadata.len(),
                    me.task_id.brief()
                );
                fs::remove_file(path)?;
                // Release the memory used by this cache
                me.handle
                    .file_handle
                    .lock()
                    .unwrap()
                    .release(metadata.len());
            }
            Ok(())
        }

        if let Err(e) = drop_inner(self) {
            // Different logging levels based on error type
            if let Some(2) = e.raw_os_error() {
                // Error 2 is typically "No such file or directory" - not a critical error
                debug!("{} drop file error: {}", self.task_id.brief(), e);
            } else {
                error!("{} drop file error: {}", self.task_id.brief(), e);
            }
        } else {
            info!("{} file drop", self.task_id.brief());
        }
    }
}

impl FileCache {
    /// Attempts to restore a file cache for the given task ID.
    ///
    /// Checks if a cache file exists for the task and attempts to restore it,
    /// applying the cache memory limit before proceeding.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to restore the cache for
    /// - `handle`: Reference to the cache manager
    ///
    /// # Returns
    /// `Some(FileCache)` if successful, `None` if the file doesn't exist or cache can't be applied
    pub(crate) fn try_restore(task_id: TaskId, handle: &'static CacheManager) -> Option<Self> {
        if let Some(path) = Self::path(&task_id) {
            let metadata = fs::metadata(&path).ok()?;
            // Check if we can allocate memory for this cache
            if !CacheManager::apply_cache(
                &handle.file_handle,
                &handle.files,
                metadata.len() as usize,
            ) {
                info!("apply file cache for task {} failed", task_id.brief());
                // Clean up the file if we can't use it
                let _ = fs::remove_file(&path);
                return None;
            }

            Some(Self { task_id, handle })
        } else {
            None
        }
    }

    /// Attempts to create a new file cache from RAM cache data.
    ///
    /// Writes the contents of the RAM cache to a file and creates a new FileCache instance.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to create the cache for
    /// - `handle`: Reference to the cache manager
    /// - `cache`: RAM cache to write to disk
    ///
    /// # Returns
    /// `Some(FileCache)` if successful, `None` if the file can't be created or cache can't be applied
    pub(crate) fn try_create(
        task_id: TaskId,
        handle: &'static CacheManager,
        cache: Arc<RamCache>,
    ) -> Option<Self> {
        let size = cache.size();
        debug!(
            "try apply new file cache {} for task {}",
            size,
            task_id.brief()
        );

        // Check if we can allocate memory for this cache
        if !CacheManager::apply_cache(&handle.file_handle, &handle.files, size) {
            info!("apply file cache for task {} failed", task_id.brief());
            return None;
        }

        // Try to create the file cache
        if let Err(e) = Self::create_file(&task_id, cache) {
            error!("create file cache error: {}", e);
            // Release memory if creation fails
            handle.file_handle.lock().unwrap().release(size as u64);
            return None;
        }
        Some(Self { task_id, handle })
    }

    /// Creates a cache file and writes the contents of the RAM cache to it.
    ///
    /// Writes data to a temporary file and then renames it with the finish suffix
    /// to indicate it's complete.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to create the file for
    /// - `cache`: RAM cache to write to disk
    ///
    /// # Returns
    /// `Ok(())` if successful, `Err(io::Error)` if any file operation fails
    fn create_file(task_id: &TaskId, cache: Arc<RamCache>) -> Result<(), io::Error> {
        if let Some(path) = Self::path(task_id) {
            // Create the file and write cache contents
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path.as_path())?;
            io::copy(&mut cache.cursor(), &mut file)?;
            file.flush()?;
            file.rewind()?;
            
            // Rename to indicate the file is complete
            let file_name = format!("{}{}", task_id, FINISH_SUFFIX);
            if let Some(new_path) = unsafe { FILE_STORE_DIR.join(file_name) } {
                fs::rename(path, new_path)?;
                return Ok(());
            }
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "cache store dir not created.",
        ))
    }

    /// Opens the cache file for reading.
    ///
    /// # Returns
    /// `Ok(File)` if successful, `Err(io::Error)` if the file can't be opened
    pub(crate) fn open(&self) -> Result<File, io::Error> {
        if let Some(path) = Self::path(&self.task_id) {
            OpenOptions::new().read(true).open(path)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "cache store dir not created.",
            ))
        }
    }

    /// Gets the path to the cache file for the given task ID.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to get the path for
    ///
    /// # Returns
    /// Path to the cache file if the directory exists, None otherwise
    fn path(task_id: &TaskId) -> Option<PathBuf> {
        // SAFETY: This is a read-only operation that joins a path
        unsafe { FILE_STORE_DIR.join(task_id.to_string() + FINISH_SUFFIX) }
    }
}

/// Restores all valid cache files from the current directory.
///
/// Scans the current cache directory for valid cache files and returns an iterator
/// over the task IDs of those files.
///
/// # Returns
/// Iterator over task IDs if the directory exists, None otherwise
pub(crate) fn restore_files() -> Option<impl Iterator<Item = TaskId>> {
    // SAFETY: This is a read-only operation to get the path
    unsafe { FILE_STORE_DIR.as_path() }.map(restore_files_inner)
}

/// Restores all valid cache files from the given directory.
///
/// Scans the directory for valid cache files, filters out incomplete files,
/// sorts them by modification time, and returns an iterator over the task IDs.
///
/// # Parameters
/// - `path`: Path to the directory to scan
///
/// # Returns
/// Iterator over task IDs of valid cache files
pub(crate) fn restore_files_inner(path: &Path) -> impl Iterator<Item = TaskId> {
    // Function to extract just the TaskId from (TaskId, SystemTime) pairs
    let closure = |(path, _)| path;

    // Read the directory contents
    let files = match fs::read_dir(path) {
        Ok(files) => files,
        Err(e) => {
            error!("read dir error {}", e);
            // Return empty iterator if directory can't be read
            return vec![].into_iter().map(closure);
        }
    };
    
    // Process and filter the directory entries
    let mut v = files
        .into_iter()
        .filter_map(|entry| match filter_map_entry(entry, path) {
            Ok((path, time)) => Some((path, time)),
            Err(e) => {
                error!("restore file error {}", e);
                None
            }
        })
        .collect::<Vec<_>>();
    
    // Sort by modification time
    v.sort_by_key(|(_, time)| *time);
    // Extract and return just the TaskIds
    v.into_iter().map(closure)
}

/// Filters and processes a directory entry to extract task ID and modification time.
///
/// Validates that the entry is a file with the correct suffix, extracts the task ID,
/// and retrieves the modification time.
///
/// # Parameters
/// - `entry`: Directory entry to process
/// - `path`: Base directory path
///
/// # Returns
/// `Ok((TaskId, SystemTime))` if the entry is a valid cache file, `Err(io::Error)` otherwise
fn filter_map_entry(
    entry: Result<DirEntry, io::Error>,
    path: &Path,
) -> Result<(TaskId, SystemTime), io::Error> {
    // Get the file name and validate it
    let file_name = entry?.file_name();
    let file_name = file_name.to_str().ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("invalid file name {:?}", file_name),
    ))?;
    
    // Check for the finish suffix to ensure the file is complete
    if !file_name.ends_with(FINISH_SUFFIX) {
        // Remove incomplete files
        let _ = fs::remove_file(path.join(file_name));
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("incomplete file {}", file_name),
        ));
    }
    
    // Extract the task ID from the file name
    let task_id = TaskId::new(file_name.trim_end_matches(FINISH_SUFFIX).to_string());
    let path = path.join(file_name);
    // Get the modification time
    let time = fs::metadata(path)?.modified()?;
    Ok((task_id, time))
}

impl CacheManager {
    /// Updates the file cache for a given task with data from RAM.
    ///
    /// This method creates a new file cache from RAM cache data in a background task,
    /// ensuring that the operation doesn't block the calling thread.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to update
    /// - `cache`: RAM cache containing the data to write to disk
    pub(super) fn update_file_cache(&'static self, task_id: TaskId, cache: Arc<RamCache>) {
        // Remove any existing update operation for this task
        self.update_from_file_once.lock().unwrap().remove(&task_id);
        
        // Spawn background task to perform the file write
        spawn(move || {
            // Store backup of RAM cache
            self.backup_rams
                .lock()
                .unwrap()
                .insert(task_id.clone(), cache.clone());
            
            // Remove any existing file cache
            self.files.lock().unwrap().remove(&task_id);
            
            // Create new file cache
            if let Some(file_cache) = FileCache::try_create(task_id.clone(), self, cache) {
                info!("{} file cache updated", task_id.brief());
                self.files
                    .lock()
                    .unwrap()
                    .insert(task_id.clone(), file_cache);
            };
            
            // Clean up backup
            self.backup_rams.lock().unwrap().remove(&task_id);
        });
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
    /// `Some(Arc<RamCache>)` if successful, `None` if the file doesn't exist or can't be read
    pub(crate) fn update_ram_from_file(&'static self, task_id: &TaskId) -> Option<Arc<RamCache>> {
        let mut retry = false;
        // Loop with retry logic for concurrent operations
        loop {
            let ret = self.update_ram_from_file_inner(task_id, &mut retry);
            if !retry || ret.is_some() {
                break ret;
            } else {
                // Clear the once lock to retry
                self.update_from_file_once.lock().unwrap().remove(task_id);
            }
        }
    }

    /// Internal implementation of updating RAM cache from file cache.
    ///
    /// Uses OnceLock to ensure only one thread loads the file at a time,
    /// returning the same result to all waiting threads.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to update
    /// - `retry`: Flag set to true if a retry is needed
    ///
    /// # Returns
    /// `Some(Arc<RamCache>)` if successful, `None` if the file doesn't exist or can't be read
    pub(crate) fn update_ram_from_file_inner(
        &'static self,
        task_id: &TaskId,
        retry: &mut bool,
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
                let res = self.rams.lock().unwrap().get(task_id).cloned();
                let res = res.or_else(|| self.backup_rams.lock().unwrap().get(task_id).cloned());
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
            
            // Open the file
            let mut file = self
                .files
                .lock()
                .unwrap()
                .get(task_id)
                .ok_or(io::Error::new(io::ErrorKind::NotFound, "not found"))?
                .open()
                .map_err(|e| {
                    error!(
                        "task {:?} update ram open file fail {:?}",
                        task_id.brief(),
                        e
                    );
                    e
                })?;

            // Get file size for buffer allocation
            let size = file.metadata()?.size();

            // Create and populate the RAM cache
            let mut cache = RamCache::new(task_id.clone(), self, Some(size as usize));
            io::copy(&mut file, &mut cache).map_err(|e| {
                error!(
                    "task {:?} copy file to cache failed {:?}",
                    task_id.brief(),
                    e
                );
                e
            })?;

            // Check if the cache size is valid
            let is_cache = cache.check_size();
            let cache = Arc::new(cache);

            // Update the RAM cache if valid
            if is_cache {
                self.update_ram_cache(cache.clone());
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
}

#[cfg(test)]
mod ut_file {
    // Include unit tests
    include!("../../tests/ut/data/ut_file.rs");
}
