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

use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::BufReader;
use std::mem::MaybeUninit;
use std::os::unix::fs::MetadataExt;
use std::sync::{Arc, LazyLock, Mutex, Once, OnceLock};
use std::time::Duration;
use std::{io, thread};

use request_utils::queue_map::QueueMap;

use super::data::Cache;
use crate::spawn;

const DEFAULT_RAM_CACHE_SIZE: usize = 1024 * 1024 * 100;
const DEFAULT_FILE_CACHE_SIZE: usize = 1024 * 1024 * 100;

pub(crate) struct CacheManager {
    rams: Mutex<QueueMap<u64, Arc<Cache>>>,
    backup_rams: Mutex<HashMap<u64, Arc<Cache>>>,

    files: Mutex<QueueMap<u64, File>>,

    ram_once: Mutex<HashMap<u64, Arc<OnceLock<Option<Arc<Cache>>>>>>,
    ram_handle: Mutex<Handle>,
}

struct RamHandle {
    ram_caches: Mutex<HashMap<u64, Arc<Cache>>>,
}

impl CacheManager {
    fn new() -> Self {
        Self {
            rams: Mutex::new(QueueMap::new()),
            files: Mutex::new(QueueMap::new()),
            backup_rams: Mutex::new(HashMap::new()),
            ram_once: Mutex::new(HashMap::new()),
            ram_handle: Mutex::new(Handle::new(DEFAULT_RAM_CACHE_SIZE)),
        }
    }

    pub(crate) fn get_instance() -> &'static Self {
        static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
        &CACHE_MANAGER
    }

    pub(crate) fn apply_for_cache(
        &self,
        task_id: u64,
        applied_size: Option<usize>,
    ) -> Result<Cache, ()> {
        if let Some(size) = applied_size {
            if !self.apply_ram_size(size) {
                return Err(());
            }
        }
        Ok(Cache::new(task_id, applied_size))
    }

    pub(crate) fn update_cache(&self, task_id: u64, cache: Arc<Cache>) {
        if let Some(old_cache) = self.rams.lock().unwrap().insert(task_id, cache.clone()) {
            self.release_cache(old_cache);
        }
        info!("{} ram updated", task_id);
        self.ram_once.lock().unwrap().remove(&task_id);
        info!("{} ram once removed", task_id);
        self.backup_rams
            .lock()
            .unwrap()
            .insert(task_id, cache.clone());
        info!("{} ram backup updated", task_id);
        spawn(move || {
            let file = cache.create_file_cache(task_id).unwrap();
            CacheManager::get_instance().update_file(task_id, file);
        });
    }

    pub(crate) fn get_cache(&self, task_id: u64) -> Option<Arc<Cache>> {
        let res = self.rams.lock().unwrap().get(&task_id).cloned();
        res.or_else(|| self.backup_rams.lock().unwrap().get(&task_id).cloned())
            .or_else(|| self.update_ram_from_file(task_id))
    }

    fn update_file(&self, task_id: u64, file: File) {
        info!("{} file updated", task_id);
        self.files.lock().unwrap().insert(task_id, file);
        self.backup_rams.lock().unwrap().remove(&task_id);
    }

    fn update_ram_from_file(&self, task_id: u64) -> Option<Arc<Cache>> {
        info!("{} ram updated from file", task_id);

        let once = match self.ram_once.lock().unwrap().entry(task_id) {
            Entry::Occupied(entry) => entry.into_mut().clone(),
            Entry::Vacant(entry) => {
                let res = self.rams.lock().unwrap().get(&task_id).cloned();
                let res = res.or_else(|| self.backup_rams.lock().unwrap().get(&task_id).cloned());
                if res.is_some() {
                    return res;
                } else {
                    entry.insert(Arc::new(OnceLock::new())).clone()
                }
            }
        };

        let mut ram_once = self.ram_once.lock().unwrap();
        if !ram_once.contains_key(&task_id) {
            let mut res = self.rams.lock().unwrap().get(&task_id).cloned();
            let res = res.or_else(|| self.backup_rams.lock().unwrap().get(&task_id).cloned());
            if res.is_some() {
                return res;
            }
        }
        let once = ram_once
            .entry(task_id)
            .or_insert(Arc::new(OnceLock::new()))
            .clone();
        drop(ram_once);

        let once = self
            .ram_once
            .lock()
            .unwrap()
            .entry(task_id)
            .or_insert(Arc::new(OnceLock::new()))
            .clone();
        once.get_or_init(|| {
            let mut file = self.files.lock().unwrap().remove(&task_id)?;
            let size = file.metadata().unwrap().size();
            let mut cache = Cache::new(task_id, Some(size as usize));
            io::copy(&mut file, &mut cache).unwrap();
            Some(cache.complete_write())
        })
        .clone()
    }

    fn release_cache(&self, cache: Arc<Cache>) {
        self.ram_handle.lock().unwrap().release(cache.size());
    }

    pub(super) fn apply_ram_size(&self, apply_size: usize) -> bool {
        self.ram_handle.lock().unwrap().apply_ram_size(apply_size)
    }

    fn update_to_ram() {}
}

struct Handle {
    total_ram: usize,
    used_ram: usize,
}

impl Handle {
    fn new(ram_cache_size: usize) -> Self {
        Self {
            total_ram: ram_cache_size,
            used_ram: 0,
        }
    }

    fn apply_ram_size(&mut self, apply_size: usize) -> bool {
        if apply_size > self.total_ram {
            return false;
        }
        self.used_ram += apply_size;
        true
    }

    fn need_release_size(&self) -> usize {
        self.used_ram.saturating_sub(self.total_ram)
    }

    fn release(&mut self, size: usize) {
        self.used_ram -= size;
    }

    fn change_total_size(&mut self, size: usize) {
        self.total_ram = size;
    }
}

#[cfg(test)]
mod test {
    use std::io::{Read, Write};
    use std::thread;
    use std::time::Duration;

    use request_utils::fastrand::fast_random;

    use super::*;
    use crate::init;
    const TEST_STRING: &str = "你这猴子真让我欢喜";

    #[test]
    fn ut_handle_size() {
        let mut handle = Handle::new(DEFAULT_RAM_CACHE_SIZE);
        assert!(!handle.apply_ram_size(DEFAULT_RAM_CACHE_SIZE + 1));
        assert!(handle.apply_ram_size(1024));
        assert_eq!(handle.need_release_size(), 0);
        assert!(handle.apply_ram_size(DEFAULT_FILE_CACHE_SIZE));
        assert_eq!(handle.need_release_size(), 1024);
        handle.release(1024);
        assert_eq!(handle.need_release_size(), 0);
        handle.change_total_size(1024);
        assert_eq!(handle.need_release_size(), DEFAULT_RAM_CACHE_SIZE - 1024);
    }

    #[test]
    fn ut_cache_manager_basic() {
        let task_id = fast_random();
        let cache_manager = CacheManager::get_instance();

        let mut cache = cache_manager
            .apply_for_cache(task_id, Some(TEST_STRING.len()))
            .unwrap();
        cache.write_all(TEST_STRING.as_bytes()).unwrap();
        let cache = Arc::new(cache);
        let mut buf = String::new();
        cache.cursor().read_to_string(&mut buf).unwrap();
        assert_eq!(buf, TEST_STRING);
    }

    #[test]
    fn ut_cache_manager_update() {
        init();
        let task_id = fast_random();
        let cache_manager = CacheManager::get_instance();

        let mut cache = cache_manager
            .apply_for_cache(task_id, Some(TEST_STRING.len()))
            .unwrap();
        cache.write_all(TEST_STRING.as_bytes()).unwrap();
        let cache = Arc::new(cache);
        cache_manager.update_cache(task_id, cache);

        let cache = cache_manager.get_cache(task_id).unwrap();
        let mut buf = String::new();
        cache.cursor().read_to_string(&mut buf).unwrap();
        assert_eq!(buf, TEST_STRING);
    }

    #[test]
    fn ut_cache_manager_file() {
        init();
        let task_id = fast_random();
        let cache_manager = CacheManager::get_instance();

        let mut cache = cache_manager
            .apply_for_cache(task_id, Some(TEST_STRING.len()))
            .unwrap();
        cache.write_all(TEST_STRING.as_bytes()).unwrap();
        let cache = Arc::new(cache);
        cache_manager.update_cache(task_id, cache);
        thread::sleep(Duration::from_millis(100));
        let mut file = cache_manager
            .files
            .lock()
            .unwrap()
            .remove(&task_id)
            .unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        assert_eq!(buf, TEST_STRING);
    }

    #[test]
    fn ut_cache_manager_ram_backup() {
        init();
        let task_id = fast_random();
        let cache_manager = CacheManager::get_instance();

        let mut cache = cache_manager
            .apply_for_cache(task_id, Some(TEST_STRING.len()))
            .unwrap();
        cache.write_all(TEST_STRING.as_bytes()).unwrap();
        let cache = Arc::new(cache);
        cache_manager.update_cache(task_id, cache);
        assert!(cache_manager
            .backup_rams
            .lock()
            .unwrap()
            .contains_key(&task_id));
        thread::sleep(Duration::from_millis(100));
        assert!(!cache_manager
            .backup_rams
            .lock()
            .unwrap()
            .contains_key(&task_id));
    }

    #[test]
    fn ut_cache_manager_cache_from_file() {
        init();
        let task_id = fast_random();
        let cache_manager = CacheManager::get_instance();

        let mut cache = cache_manager
            .apply_for_cache(task_id, Some(TEST_STRING.len()))
            .unwrap();
        cache.write_all(TEST_STRING.as_bytes()).unwrap();
        let cache = Arc::new(cache);
        cache_manager.update_cache(task_id, cache);
        thread::sleep(Duration::from_millis(100));
        cache_manager.rams.lock().unwrap().remove(&task_id);

        let mut v = vec![];
        for _ in 0..1024 {
            v.push(thread::spawn(move || {
                let cache = CacheManager::get_instance().get_cache(task_id).unwrap();
                let mut buf = String::new();
                cache.cursor().read_to_string(&mut buf).unwrap();
                buf == TEST_STRING
            }));
        }
        for t in v {
            assert!(t.join().unwrap());
        }
    }
}
