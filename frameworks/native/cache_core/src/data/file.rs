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

const FINISH_SUFFIX: &str = "_F";

pub(crate) static mut FILE_STORE_DIR: FileStoreDir = FileStoreDir::new();

static INIT_HISTORY: Once = Once::new();

static INIT_CURR: Once = Once::new();

pub fn init_history_store_dir(history: Arc<HistoryDir>, spawner: fn(PathBuf, Arc<HistoryDir>)) {
    INIT_HISTORY.call_once(|| unsafe {
        FILE_STORE_DIR.set_history_dir(history, spawner);
    });
}

pub fn init_curr_store_dir(curr: PathBuf) {
    INIT_CURR.call_once(|| unsafe {
        FILE_STORE_DIR.set_curr_dir(curr);
    });
}

pub fn get_curr_store_dir() -> PathBuf {
    #[cfg(feature = "ohos")]
    let mut path = match request_utils::context::get_cache_dir() {
        Some(dir) => PathBuf::from_str(&dir).unwrap(),
        None => {
            error!("get cache dir failed");
            PathBuf::from_str("/data/storage/el2/base/cache").unwrap()
        }
    };
    #[cfg(not(feature = "ohos"))]
    let mut path = PathBuf::from_str("./").unwrap();

    path.push("preload_caches");
    if let Err(e) = fs::create_dir_all(path.as_path()) {
        error!("create cache dir error {}", e);
    }
    path
}

pub struct FileStoreDir {
    history: Option<DirObservSpawner>,
    curr: Option<PathBuf>,
}

impl FileStoreDir {
    pub const fn new() -> Self {
        Self {
            history: None,
            curr: None,
        }
    }

    pub fn set_history_dir(
        &mut self,
        history: Arc<HistoryDir>,
        spawner: fn(PathBuf, Arc<HistoryDir>),
    ) {
        self.history = Some(DirObservSpawner::new(history, spawner));
    }

    pub fn set_curr_dir(&mut self, curr: PathBuf) {
        self.curr = Some(curr);
    }

    // SAFETY: curr is guaranteed to be Some.
    fn curr(&self) -> &PathBuf {
        self.curr.as_ref().unwrap()
    }

    pub(crate) fn exist(&self) -> bool {
        if let Some(ref history) = self.history {
            if !history.exist() && history.create() {
                history.spawn_observe(self.curr().clone());
            }
        }
        if !self.curr().is_dir() {
            if let Err(e) = fs::create_dir_all(self.curr().as_path()) {
                error!("try create current cache dir error {}", e);
                return false;
            }
        }
        true
    }

    pub(crate) fn join(&self, path: String) -> Option<PathBuf> {
        if self.exist() {
            Some(self.curr().join(path))
        } else {
            None
        }
    }

    pub(crate) fn as_path(&self) -> Option<&Path> {
        if self.exist() {
            Some(self.curr().as_path())
        } else {
            None
        }
    }
}

pub(crate) struct DirObservSpawner {
    history: Arc<HistoryDir>,
    spawner: fn(PathBuf, Arc<HistoryDir>),
}

impl DirObservSpawner {
    pub(crate) fn new(history: Arc<HistoryDir>, spawner: fn(PathBuf, Arc<HistoryDir>)) -> Self {
        Self { history, spawner }
    }

    pub(crate) fn exist(&self) -> bool {
        self.history.exist()
    }

    pub fn create(&self) -> bool {
        self.history.create()
    }

    pub fn spawn_observe(&self, curr: PathBuf) {
        let mut is_observe = self.history.is_observe.lock().unwrap();
        if !*is_observe {
            (self.spawner)(curr, self.history.clone());
            *is_observe = true;
        }
    }
}

pub struct HistoryDir {
    dir: PathBuf,
    pub is_observe: Mutex<bool>,
}

impl HistoryDir {
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir,
            is_observe: Mutex::new(false),
        }
    }

    pub fn exist(&self) -> bool {
        self.dir.is_dir()
    }

    pub fn create(&self) -> bool {
        if let Err(e) = fs::create_dir_all(self.dir.as_path()) {
            error!("try create history dir error {}", e);
            false
        } else {
            true
        }
    }

    pub fn stop_observe(&self) {
        let mut is_observe = self.is_observe.lock().unwrap();
        *is_observe = false;
    }

    pub fn dir_path(&self) -> Option<&str> {
        self.dir.to_str()
    }
}

pub(crate) struct FileCache {
    task_id: TaskId,
    handle: &'static CacheManager,
}

impl Drop for FileCache {
    fn drop(&mut self) {
        fn drop_inner(me: &mut FileCache) -> Result<(), io::Error> {
            if let Some(path) = FileCache::path(&me.task_id) {
                let metadata = fs::metadata(&path)?;
                debug!(
                    "try drop file cache {} for task {}",
                    metadata.len(),
                    me.task_id.brief()
                );
                fs::remove_file(path)?;
                me.handle
                    .file_handle
                    .lock()
                    .unwrap()
                    .release(metadata.len());
            }
            Ok(())
        }

        if let Err(e) = drop_inner(self) {
            error!("{} drop file cache error: {}", self.task_id, e);
        } else {
            info!("{} file cache drop", self.task_id.brief());
        }
    }
}

impl FileCache {
    pub(crate) fn try_restore(task_id: TaskId, handle: &'static CacheManager) -> Option<Self> {
        if let Some(path) = Self::path(&task_id) {
            let metadata = fs::metadata(&path).ok()?;
            if !CacheManager::apply_cache(
                &handle.file_handle,
                &handle.files,
                FileCache::task_id,
                metadata.len() as usize,
            ) {
                info!("apply file cache for task {} failed", task_id.brief());
                let _ = fs::remove_file(&path);
                return None;
            }

            Some(Self { task_id, handle })
        } else {
            None
        }
    }

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

        if !CacheManager::apply_cache(&handle.file_handle, &handle.files, FileCache::task_id, size)
        {
            info!("apply file cache for task {} failed", task_id.brief());
            return None;
        }

        if let Err(e) = Self::create_file(&task_id, cache) {
            error!("create file cache error: {}", e);
            handle.file_handle.lock().unwrap().release(size as u64);
            return None;
        }
        Some(Self { task_id, handle })
    }

    fn create_file(task_id: &TaskId, cache: Arc<RamCache>) -> Result<(), io::Error> {
        if let Some(path) = Self::path(task_id) {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path.as_path())?;
            io::copy(&mut cache.cursor(), &mut file)?;
            file.flush()?;
            file.rewind()?;
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

    pub(crate) fn task_id(&self) -> &TaskId {
        &self.task_id
    }

    fn path(task_id: &TaskId) -> Option<PathBuf> {
        unsafe { FILE_STORE_DIR.join(task_id.to_string() + FINISH_SUFFIX) }
    }
}

pub(crate) fn restore_files() -> Option<impl Iterator<Item = TaskId>> {
    unsafe { FILE_STORE_DIR.as_path() }.map(restore_files_inner)
}

pub(crate) fn restore_files_inner(path: &Path) -> impl Iterator<Item = TaskId> {
    let closure = |(path, _)| path;

    let files = match fs::read_dir(path) {
        Ok(files) => files,
        Err(e) => {
            error!("read dir error {}", e);
            return vec![].into_iter().map(closure);
        }
    };
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
    v.sort_by_key(|(_, time)| *time);
    v.into_iter().map(closure)
}

fn filter_map_entry(
    entry: Result<DirEntry, io::Error>,
    path: &Path,
) -> Result<(TaskId, SystemTime), io::Error> {
    let file_name = entry?.file_name();
    let file_name = file_name.to_str().ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("invalid file name {:?}", file_name),
    ))?;
    if !file_name.ends_with(FINISH_SUFFIX) {
        let _ = fs::remove_file(path.join(file_name));
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("incomplete file {}", file_name),
        ));
    }
    let task_id = TaskId::new(file_name.trim_end_matches(FINISH_SUFFIX).to_string());
    let path = path.join(file_name);
    let time = fs::metadata(path)?.modified()?;
    Ok((task_id, time))
}

impl CacheManager {
    pub(super) fn update_file_cache(&'static self, task_id: TaskId, cache: Arc<RamCache>) {
        self.update_from_file_once.lock().unwrap().remove(&task_id);
        spawn(move || {
            self.backup_rams
                .lock()
                .unwrap()
                .insert(task_id.clone(), cache.clone());
            self.files.lock().unwrap().remove(&task_id);
            if let Some(file_cache) = FileCache::try_create(task_id.clone(), self, cache) {
                info!("{} file cache updated", task_id.brief());
                self.files
                    .lock()
                    .unwrap()
                    .insert(task_id.clone(), file_cache);
            };
            self.backup_rams.lock().unwrap().remove(&task_id);
        });
    }

    pub(crate) fn update_ram_from_file(&'static self, task_id: &TaskId) -> Option<Arc<RamCache>> {
        let mut retry = false;
        loop {
            let ret = self.update_ram_from_file_inner(task_id, &mut retry);
            if !retry || ret.is_some() {
                break ret;
            } else {
                self.update_from_file_once.lock().unwrap().remove(task_id);
            }
        }
    }

    pub(crate) fn update_ram_from_file_inner(
        &'static self,
        task_id: &TaskId,
        retry: &mut bool,
    ) -> Option<Arc<RamCache>> {
        *retry = false;
        let once = match self
            .update_from_file_once
            .lock()
            .unwrap()
            .entry(task_id.clone())
        {
            Entry::Occupied(entry) => entry.into_mut().clone(),
            Entry::Vacant(entry) => {
                let res = self.rams.lock().unwrap().get(task_id).cloned();
                let res = res.or_else(|| self.backup_rams.lock().unwrap().get(task_id).cloned());
                if res.is_some() {
                    return res;
                } else {
                    entry.insert(Arc::new(OnceLock::new())).clone()
                }
            }
        };

        let mut ret = None;
        let res = once.get_or_init(|| {
            debug!("{} ram updated from file", task_id.brief());
            let mut file = self
                .files
                .lock()
                .unwrap()
                .get(task_id)
                .ok_or(io::Error::new(io::ErrorKind::NotFound, "not found"))?
                .open()?;

            let size = file.metadata()?.size();

            let mut cache = RamCache::new(task_id.clone(), self, Some(size as usize));
            io::copy(&mut file, &mut cache).map_err(|e| {
                error!("copy file to cache failed {:?}", e);
                e
            })?;

            let is_cache = cache.check_size();
            let cache = Arc::new(cache);

            if is_cache {
                self.update_ram_cache(cache.clone());
            }

            ret = Some(cache.clone());
            let weak_cache = Arc::downgrade(&cache);
            Ok(weak_cache)
        });

        if ret.is_some() {
            return ret;
        }
        let res = match res {
            Err(e) => {
                info!("{} ram update from file failed {}", task_id.brief(), e);
                None
            }
            Ok(weak) => {
                *retry = true;
                Weak::upgrade(weak)
            }
        };
        res
    }
}

#[cfg(test)]
mod ut_file {
    include!("../../tests/ut/data/ut_file.rs");
}
