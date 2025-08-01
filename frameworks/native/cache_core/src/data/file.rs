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
use std::sync::{Arc, LazyLock, OnceLock, Weak};
use std::time::SystemTime;

use request_utils::task_id::TaskId;

use super::ram::RamCache;
use crate::manage::CacheManager;
use crate::spawn;

const FINISH_SUFFIX: &str = "_F";

static CACHE_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
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
});

pub(crate) struct FileCache {
    task_id: TaskId,
    handle: &'static CacheManager,
}

impl Drop for FileCache {
    fn drop(&mut self) {
        fn drop_inner(me: &mut FileCache) -> Result<(), io::Error> {
            let path = FileCache::path(&me.task_id);
            let metadata = fs::metadata(&path)?;
            info!(
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
        let metadata = fs::metadata(Self::path(&task_id)).ok()?;
        if !CacheManager::apply_cache(
            &handle.file_handle,
            &handle.files,
            FileCache::task_id,
            metadata.len() as usize,
        ) {
            info!("apply file cache for task {} failed", task_id.brief());
            let path = FileCache::path(&task_id);
            let _ = fs::remove_file(path);
            return None;
        }

        Some(Self { task_id, handle })
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
        info!("apply file cache for task {} success", task_id.brief());
        Some(Self { task_id, handle })
    }

    fn create_file(task_id: &TaskId, cache: Arc<RamCache>) -> Result<(), io::Error> {
        let path = Self::path(task_id);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.as_path())?;
        io::copy(&mut cache.cursor(), &mut file)?;
        file.flush()?;
        file.rewind()?;
        let file_name = format!("{}{}", task_id, FINISH_SUFFIX);
        let new_path = CACHE_DIR_PATH.join(file_name);
        fs::rename(path, new_path)?;
        Ok(())
    }

    pub(crate) fn open(&self) -> Result<File, io::Error> {
        OpenOptions::new()
            .read(true)
            .open(Self::path(&self.task_id))
    }

    pub(crate) fn task_id(&self) -> &TaskId {
        &self.task_id
    }

    fn path(task_id: &TaskId) -> PathBuf {
        CACHE_DIR_PATH.join(task_id.to_string() + FINISH_SUFFIX)
    }
}

pub(crate) fn restore_files() -> impl Iterator<Item = TaskId> {
    restore_files_inner(CACHE_DIR_PATH.as_path())
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
            info!("{} ram updated from file", task_id.brief());
            let mut file = self
                .files
                .lock()
                .unwrap()
                .get(task_id)
                .ok_or(io::Error::new(io::ErrorKind::NotFound, "not found"))?
                .open()?;

            let size = file.metadata()?.size();

            let mut cache = RamCache::new(task_id.clone(), self, Some(size as usize));
            io::copy(&mut file, &mut cache).unwrap();

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
