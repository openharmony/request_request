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

use std::collections::HashMap;
use std::io;
use std::mem::MaybeUninit;
use std::sync::{Arc, Mutex, Once, OnceLock, Weak};

use request_utils::lru::LRUCache;

use super::data::{self, FileCache, RamCache};
use crate::agent::TaskId;

cfg_not_test! {
    const DEFAULT_RAM_CACHE_SIZE: u64 =  1024 * 1024 * 20;
    const DEFAULT_FILE_CACHE_SIZE: u64 = 1024 * 1024 * 100;
    use super::data::restore_files;
}

cfg_test! {
    const DEFAULT_RAM_CACHE_SIZE: u64 = 1024 *100;
    const DEFAULT_FILE_CACHE_SIZE: u64 = 1024 * 100;
}

pub(crate) struct CacheManager {
    pub(super) rams: Mutex<LRUCache<TaskId, Arc<RamCache>>>,
    pub(super) backup_rams: Mutex<HashMap<TaskId, Arc<RamCache>>>,
    pub(super) files: Mutex<LRUCache<TaskId, FileCache>>,

    pub(super) update_from_file_once:
        Mutex<HashMap<TaskId, Arc<OnceLock<io::Result<Weak<RamCache>>>>>>,
    pub(super) ram_handle: Mutex<data::Handle>,
    pub(super) file_handle: Mutex<data::Handle>,
}

impl CacheManager {
    pub(super) fn new() -> Self {
        Self {
            rams: Mutex::new(LRUCache::new()),
            files: Mutex::new(LRUCache::new()),
            backup_rams: Mutex::new(HashMap::new()),
            update_from_file_once: Mutex::new(HashMap::new()),

            ram_handle: Mutex::new(data::Handle::new(DEFAULT_RAM_CACHE_SIZE)),
            file_handle: Mutex::new(data::Handle::new(DEFAULT_FILE_CACHE_SIZE)),
        }
    }

    pub(crate) fn get_instance() -> &'static Self {
        static mut CACHE_MANAGER: MaybeUninit<CacheManager> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();
        ONCE.call_once(|| unsafe {
            CACHE_MANAGER.write(CacheManager::new());
        });
        unsafe { CACHE_MANAGER.assume_init_ref() }
    }

    #[cfg(not(test))]
    pub(crate) fn init(&'static self) {
        for task_id in restore_files() {
            let Some(file_cache) = FileCache::try_restore(task_id.clone(), self) else {
                continue;
            };
            self.files.lock().unwrap().insert(task_id, file_cache);
        }
    }

    pub(crate) fn remove(&'static self, task_id: TaskId) {
        self.files.lock().unwrap().remove(&task_id);
        self.backup_rams.lock().unwrap().remove(&task_id);
        self.rams.lock().unwrap().remove(&task_id);
        self.update_from_file_once.lock().unwrap().remove(&task_id);
    }

    pub(crate) fn get_cache(&'static self, task_id: &TaskId) -> Option<Arc<RamCache>> {
        let res = self.rams.lock().unwrap().get(task_id).cloned();
        res.or_else(|| self.backup_rams.lock().unwrap().get(task_id).cloned())
            .or_else(|| self.update_ram_from_file(task_id))
    }

    pub(super) fn apply_cache<T>(
        handle: &Mutex<data::Handle>,
        caches: &Mutex<LRUCache<TaskId, T>>,
        task_id: fn(&T) -> &TaskId,
        size: usize,
    ) -> bool {
        loop {
            if handle.lock().unwrap().apply_cache_size(size as u64) {
                return true;
            };

            match caches.lock().unwrap().pop() {
                Some(cache) => {
                    info!("CacheManager release cache {}", task_id(&cache).brief());
                }
                None => {
                    info!("CacheManager release cache failed");
                    return false;
                }
            }
        }
    }

    pub(crate) fn set_ram_cache_size(&'static self, size: u64) {
        self.ram_handle.lock().unwrap().change_total_size(size);
    }

    pub(crate) fn set_file_cache_size(&'static self, size: u64) {
        self.file_handle.lock().unwrap().change_total_size(size);
    }
}

#[cfg(test)]
mod test {
    use std::io::{Read, Write};
    use std::sync::LazyLock;
    use std::thread;
    use std::time::Duration;

    use super::*;
    use crate::init;
    const TEST_STRING: &str = "你这猴子真让我欢喜";
    const TEST_STRING_SIZE: usize = TEST_STRING.len();

    #[test]
    fn ut_cache_manager_update_file() {
        init();
        let task_id = TaskId::random();
        static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);

        // update cache
        let mut cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
        cache.write_all(TEST_STRING.as_bytes()).unwrap();
        cache.finish_write();
        thread::sleep(Duration::from_millis(100));

        // files contain cache
        let mut file = CACHE_MANAGER
            .files
            .lock()
            .unwrap()
            .remove(&task_id)
            .unwrap()
            .open()
            .unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        assert_eq!(buf, TEST_STRING);

        // backup caches removed for file exist
        assert!(!CACHE_MANAGER
            .backup_rams
            .lock()
            .unwrap()
            .contains_key(&task_id));
    }

    #[test]
    fn ut_cache_manager_get() {
        init();
        let task_id = TaskId::random();
        static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);

        let mut cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));

        cache.write_all(TEST_STRING.as_bytes()).unwrap();
        cache.finish_write();

        let cache = CACHE_MANAGER.get_cache(&task_id).unwrap();
        let mut buf = String::new();
        cache.cursor().read_to_string(&mut buf).unwrap();
        assert_eq!(buf, TEST_STRING);
    }

    #[test]
    fn ut_cache_manager_cache_from_file() {
        init();
        let task_id = TaskId::random();

        static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
        let mut cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
        cache.write_all(TEST_STRING.as_bytes()).unwrap();
        cache.finish_write();

        thread::sleep(Duration::from_millis(100));
        CACHE_MANAGER.rams.lock().unwrap().remove(&task_id);

        let mut v = vec![];
        for _ in 0..1 {
            let task_id = task_id.clone();
            v.push(crate::spawn(move || {
                let cache = CACHE_MANAGER.get_cache(&task_id).unwrap();
                let mut buf = String::new();
                cache.cursor().read_to_string(&mut buf).unwrap();
                buf == TEST_STRING
            }));
        }
        for t in v {
            assert!(t.join().unwrap());
        }
    }

    #[test]
    fn ut_cache_manager_cache_from_file_clean() {
        init();
        let task_id = TaskId::random();
        static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);

        let mut cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
        cache.write_all(TEST_STRING.as_bytes()).unwrap();
        cache.finish_write();
        thread::sleep(Duration::from_millis(100));
        CACHE_MANAGER.rams.lock().unwrap().remove(&task_id);

        CACHE_MANAGER.get_cache(&task_id).unwrap();
        assert!(CACHE_MANAGER.rams.lock().unwrap().contains_key(&task_id));
        assert!(!CACHE_MANAGER
            .backup_rams
            .lock()
            .unwrap()
            .contains_key(&task_id));
        assert!(!CACHE_MANAGER
            .update_from_file_once
            .lock()
            .unwrap()
            .contains_key(&task_id));
    }

    #[test]
    fn ut_cache_manager_update_same() {
        init();
        let task_id = TaskId::random();
        static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);

        let mut cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));

        cache.write_all(TEST_STRING.as_bytes()).unwrap();
        cache.finish_write();

        let mut test_string = TEST_STRING.to_string();
        test_string.push_str(TEST_STRING);

        let mut cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(test_string.len()));
        cache.write_all(test_string.as_bytes()).unwrap();
        cache.finish_write();

        let cache = CACHE_MANAGER.get_cache(&task_id).unwrap();
        let mut buf = String::new();
        cache.cursor().read_to_string(&mut buf).unwrap();
        assert_eq!(buf, test_string);

        CACHE_MANAGER.rams.lock().unwrap().remove(&task_id);

        let mut buf = String::new();
        cache.cursor().read_to_string(&mut buf).unwrap();
        assert_eq!(buf, test_string);
    }

    // #[allow(unused)]
    // fn ut_cache_manager_multi_write_read() {
    //     init();
    //     static CACHE_MANAGER: LazyLock<CacheManager> =
    // LazyLock::new(CacheManager::new);     let size =
    // TEST_STRING.bytes().len();

    //     for _ in 0..10 {
    //         spawn(move || loop {
    //             let task_id = TaskId::new(fast_random().to_string());
    //             let mut cache = CACHE_MANAGER
    //                 .apply_for_cache(&task_id, Some(size as u64))
    //                 .unwrap();
    //             cache.write_all(TEST_STRING.as_bytes()).unwrap();
    //             CACHE_MANAGER.update_cache(task_id.clone(), Arc::new(cache));
    //             let cache = CACHE_MANAGER.get_cache(&task_id).unwrap();
    //             let mut buf = String::new();
    //             cache.cursor().read_to_string(&mut buf).unwrap();
    //             assert_eq!(buf, TEST_STRING);
    //         });
    //     }
    //     std::thread::sleep(Duration::from_secs(1000));
    // }
}
