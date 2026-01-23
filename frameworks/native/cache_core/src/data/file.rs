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
//! This module provides functionality for managing file-based caches,
//! including:
//! - Directory management for cache storage
//! - File cache creation, restoration, and deletion
//! - Synchronization between RAM and disk storage
//! - Directory observation for cache maintenance
//!
//! The implementation ensures thread-safe access to cache resources and
//! provides mechanisms for persisting data across application restarts.

use std::fs::{self, DirEntry, File, OpenOptions};
use std::io::{self, Seek, Write};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Arc, Mutex, Once};
use std::time::SystemTime;

use request_utils::task_id::TaskId;

use super::ram::RamCache;
use crate::manage::CacheManager;

/// Suffix appended to files that are fully written and finalized.
///
/// This suffix is used to indicate that a cache file has been completely
/// written and is ready for use. Files without this suffix may be incomplete
/// and are considered invalid.
const FINISH_SUFFIX: &str = "_F";

/// Global file store directory manager.
///
/// This static variable manages the directories used for storing cache files.
/// It is initialized on first use through the `init_history_store_dir` and
/// `init_curr_store_dir` functions.
pub(crate) static mut FILE_STORE_DIR: FileStoreDir = FileStoreDir::new();

/// One-time initialization flag for history directory.
///
/// Ensures the history directory is initialized exactly once across all
/// threads.
static INIT_HISTORY: Once = Once::new();

/// One-time initialization flag for current directory.
///
/// Ensures the current directory is initialized exactly once across all
/// threads.
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
/// This function is thread-safe and will only initialize the history directory
/// once.
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
        // and it's protected by INIT_HISTORY which ensures it's initialized exactly
        // once.
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
/// This function is thread-safe and will only initialize the current directory
/// once.
pub fn init_curr_store_dir() {
    INIT_CURR.call_once(|| {
        let curr_dir = get_curr_store_dir();
        // SAFETY: This is the only place where FILE_STORE_DIR's curr field is modified
        // concurrently, and it's protected by INIT_CURR which ensures it's
        // initialized exactly once.
        unsafe {
            FILE_STORE_DIR.set_curr_dir(curr_dir);
        }
    });
}

/// Gets the path to the current cache directory.
///
/// Returns the path to the directory where cache files are stored. On
/// OpenHarmony systems, it uses the application's cache directory, falling back
/// to a default path if that fails. On other systems, it uses the current
/// directory.
///
/// # Returns
/// Path to the cache directory
///
/// # Notes
/// This function creates the directory if it doesn't exist.
pub fn get_curr_store_dir() -> PathBuf {
    #[cfg(feature = "ohos")]
    let mut path = match request_application::application::wrapper::get_cache_dir() {
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
/// This function only performs a read operation on the FILE_STORE_DIR, which is
/// safe.
pub fn is_history_init() -> bool {
    // SAFETY: This is a read-only operation on FILE_STORE_DIR, which is
    // thread-safe.
    unsafe { FILE_STORE_DIR.history.is_some() }
}

/// Manages directories used for storing cache files.
///
/// This struct keeps track of both the current and history directories used for
/// storing cache files, providing methods to check existence, join paths, and
/// ensure directories are created when needed.
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
    /// This method assumes that curr is not None, which is guaranteed by
    /// init_curr_store_dir.
    fn curr(&self) -> &PathBuf {
        self.curr.as_ref().unwrap()
    }

    /// Checks if the directory exists and creates it if necessary.
    ///
    /// Ensures both history and current directories exist, creating them if
    /// needed. Also starts directory observation if the history directory
    /// was just created.
    ///
    /// # Returns
    /// `true` if the directories exist (or were created successfully), `false`
    /// otherwise
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
    size: u64,
    /// ID of the task associated with this cache
    task_id: TaskId,
}

impl FileCache {
    pub(crate) fn new(task_id: TaskId, size: u64) -> Self {
        Self { size, task_id }
    }

    pub(crate) fn size(&self) -> u64 {
        self.size
    }

    pub(crate) fn task_id(&self) -> &TaskId {
        &self.task_id
    }

    /// Releases the file cache resource.
    ///
    /// Removes the cache file from disk and releases the associated memory.
    ///
    /// # Returns
    /// Amount of memory released (in bytes)
    pub(crate) fn remove_file(task_id: &TaskId) {
        if let Some(path) = Self::path(task_id) {
            if let Err(e) = fs::remove_file(path) {
                // Different logging levels based on error type
                if let Some(2) = e.raw_os_error() {
                    // Error 2 is typically "No such file or directory" - not a critical error
                    debug!("{} drop file error: {}", task_id.brief(), e);
                } else {
                    error!("{} drop file error: {}", task_id.brief(), e);
                }
            }
        }
    }

    /// Creates a cache file and writes the contents of the RAM cache to it.
    ///
    /// Writes data to a temporary file and then renames it with the finish
    /// suffix to indicate it's complete.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to create the file for
    /// - `cache`: RAM cache to write to disk
    ///
    /// # Returns
    /// `Ok(())` if successful, `Err(io::Error)` if any file operation fails
    pub(crate) fn create_file(task_id: &TaskId, cache: Arc<RamCache>) -> Result<(), io::Error> {
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
            return Ok(());
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "cache store dir not created.",
        ))
    }

    pub(crate) fn open(task_id: &TaskId) -> Result<File, io::Error> {
        if let Some(path) = Self::path(task_id) {
            OpenOptions::new().read(true).open(path)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "file not found."))
        }
    }

    pub(crate) fn read(task_id: &TaskId, handle: &'static CacheManager) -> io::Result<RamCache> {
        let mut file = Self::open(task_id).map_err(|e| {
            error!("{:?} open file failed {:?}", task_id.brief(), e);
            e
        })?;
        let size = file.metadata()?.size();
        FileCache::copy_file_to_cache(task_id, handle, &mut file, Some(size as usize))
    }

    pub(crate) fn read_but_not_cache(
        task_id: &TaskId,
        handle: &'static CacheManager,
    ) -> io::Result<RamCache> {
        let mut file = Self::open(task_id).map_err(|e| {
            error!("{:?} file open failed {:?}", task_id.brief(), e);
            e
        })?;
        FileCache::copy_file_to_cache(task_id, handle, &mut file, None)
    }

    fn copy_file_to_cache(
        task_id: &TaskId,
        handle: &'static CacheManager,
        file: &mut File,
        size: Option<usize>,
    ) -> io::Result<RamCache> {
        let mut cache = RamCache::new(task_id.clone(), handle, size);
        io::copy(file, &mut cache).map_err(|e| {
            error!("{:?} copy file failed {:?}", task_id.brief(), e);
            e
        })?;
        Ok(cache)
    }

    /// Gets the path to the cache file for the given task ID.
    ///
    /// # Parameters
    /// - `task_id`: ID of the task to get the path for
    ///
    /// # Returns
    /// Path to the cache file if the directory exists, None otherwise
    pub(crate) fn path(task_id: &TaskId) -> Option<PathBuf> {
        // SAFETY: This is a read-only operation that joins a path
        unsafe { FILE_STORE_DIR.join(task_id.to_string() + FINISH_SUFFIX) }
    }
}

pub(crate) struct FileCacheInfo {
    task_id: TaskId,
    time: SystemTime,
    size: u64,
}

impl FileCacheInfo {
    pub(crate) fn new(task_id: TaskId, time: SystemTime, size: u64) -> Self {
        Self {
            task_id,
            time,
            size,
        }
    }
    pub(crate) fn task_id(&self) -> &TaskId {
        &self.task_id
    }
    pub(crate) fn time(&self) -> SystemTime {
        self.time
    }
    pub(crate) fn size(&self) -> u64 {
        self.size
    }
}

pub(crate) fn get_cached_files_info() -> Option<impl Iterator<Item = FileCacheInfo>> {
    // SAFETY: This is a read-only operation to get the path
    unsafe { FILE_STORE_DIR.as_path() }.map(get_info_from_path)
}

pub(crate) fn get_info_from_path(path: &Path) -> impl Iterator<Item = FileCacheInfo> {
    // Read the directory contents
    let files = match fs::read_dir(path) {
        Ok(files) => files,
        Err(e) => {
            error!("restore read dir error {}", e);
            // Return empty iterator if directory can't be read
            return vec![].into_iter();
        }
    };
    // Process and filter the directory entries
    let mut v = files
        .into_iter()
        .filter_map(|entry| match get_entry_file_info(entry) {
            Ok(info) => Some(info),
            Err(e) => {
                error!("restore file error {}", e);
                None
            }
        })
        .collect::<Vec<_>>();

    v.sort_by_key(|info| info.time());
    v.into_iter()
}

pub(crate) fn get_entry_file_info(entry: io::Result<DirEntry>) -> Result<FileCacheInfo, io::Error> {
    let entry = entry?;
    // Get the file name and validate it
    let file_name = entry.file_name();
    let file_name = file_name.to_str().ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("invalid file name {:?}", file_name),
    ))?;

    // Check for the finish suffix to ensure the file is complete
    if !file_name.ends_with(FINISH_SUFFIX) {
        // Remove incomplete files
        let _ = fs::remove_file(entry.path());
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("incomplete file {}", file_name),
        ));
    }

    // Extract the task ID from the file name
    let task_id = TaskId::new(file_name.trim_end_matches(FINISH_SUFFIX).to_string());
    // Get the modification time
    let time = entry.metadata()?.modified()?;
    let size = entry.metadata()?.len();
    Ok(FileCacheInfo::new(task_id, time, size))
}

#[cfg(test)]
mod ut_file {
    // Include unit tests
    include!("../../tests/ut/data/ut_file.rs");
}
