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

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, Cursor, Read, Seek, Write};
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex, Once, RwLock};
use std::thread;

use super::data::Cache;

const DEFAULT_RAM_CACHE_SIZE: usize = 1024 * 1024 * 100;
const DEFAULT_FILE_CACHE_SIZE: usize = 1024 * 1024 * 100;

pub(crate) struct CacheManager {
    caches: Mutex<HashMap<String, Arc<Cache>>>,
    ram_cache_queue: Mutex<VecDeque<String>>,
    file_cache_queue: Mutex<VecDeque<String>>,
    handler: Mutex<Handler>,
}

impl CacheManager {
    fn new() -> Self {
        Self {
            caches: Mutex::new(HashMap::new()),
            ram_cache_queue: Mutex::new(VecDeque::new()),
            file_cache_queue: Mutex::new(VecDeque::new()),
            handler: Mutex::new(Handler::new(DEFAULT_RAM_CACHE_SIZE)),
        }
    }

    pub(crate) fn get_instance() -> &'static Self {
        static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
        &CACHE_MANAGER
    }

    pub(crate) fn apply_for_cache(&self, size: Option<usize>) -> Cache {
        if let Some(size) = size {
            self.apply_ram_size(size);
        }
        Cache::new_ram(size)
    }

    pub(crate) fn update_cache(&self, url: String, cache: Cache) -> bool {
        if !cache.is_valid() {
            return false;
        }
        let cache = Arc::new(cache);
        if let Some(old_cache) = self
            .caches
            .lock()
            .unwrap()
            .insert(url.clone(), cache.clone())
        {
            self.release_cache(old_cache);
        }

        if cache.is_ram() {
            if !cache.known_size() {
                let size = cache.size().unwrap();
                self.apply_ram_size(size);
            }
            self.ram_cache_queue.lock().unwrap().push_back(url);
        } else {
            self.file_cache_queue.lock().unwrap().push_back(url);
        }

        true
    }

    fn release_cache(&self, cache: Arc<Cache>) {
        let size = cache.size().unwrap();
        if cache.is_ram() {
            self.handler.lock().unwrap().release(size);
        }
    }

    pub(crate) fn get_cache(&self, url: String) -> Option<Arc<Cache>> {
        self.caches.lock().unwrap().get(&url).cloned()
    }

    fn release_ram_cache(&self, release_size: usize) {
        let mut released = 0;

        while let Some((url, mut cache)) = self
            .ram_cache_queue
            .lock()
            .unwrap()
            .pop_front()
            .and_then(|x| self.caches.lock().unwrap().get(&x).map(|c| (x, c.clone())))
        {
            let size = cache.size().unwrap();
            released += size;
            thread::spawn(move || {
                let cache = cache.create_file_cache(&url).unwrap();
                CacheManager::get_instance().update_cache(url, cache);
            });
            if released >= release_size {
                break;
            }
        }
    }

    fn apply_ram_size(&self, apply_size: usize) {
        let mut handler = self.handler.lock().unwrap();
        handler.apply_ram_size(apply_size);
        self.release_ram_cache(handler.need_release_size());
    }

    fn update_to_ram() {}
}

struct Handler {
    total_ram: usize,
    used_ram: usize,
}

impl Handler {
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
    use super::*;
    use crate::cache;
    const TEST_URL: &str = "小心猴子";
    const TEST_STRING: &str = "你这猴子真让我欢喜";

    #[test]
    fn ut_handler_size() {
        let mut handler = Handler::new(DEFAULT_RAM_CACHE_SIZE);
        assert!(!handler.apply_ram_size(DEFAULT_RAM_CACHE_SIZE + 1));
        assert!(handler.apply_ram_size(1024));
        assert_eq!(handler.need_release_size(), 0);
        assert!(handler.apply_ram_size(DEFAULT_FILE_CACHE_SIZE));
        assert_eq!(handler.need_release_size(), 1024);
        handler.release(1024);
        assert_eq!(handler.need_release_size(), 0);
        handler.change_total_size(1024);
        assert_eq!(handler.need_release_size(), DEFAULT_RAM_CACHE_SIZE - 1024);
    }

    #[test]
    fn ut_cache_manager_basic() {
        let cache_manager = CacheManager::new();
        let mut cache = cache_manager.apply_for_cache(Some(TEST_STRING.len()));
        cache.write_all(TEST_STRING.as_bytes()).unwrap();

        assert!(cache_manager.update_cache(TEST_URL.to_string(), cache));
        let mut cache = cache_manager.get_cache(TEST_URL.to_string()).unwrap();

        let mut buf = String::new();
        cache.reader().read_to_string(&mut buf);
        assert_eq!(buf, TEST_STRING);
    }

    #[test]
    fn ut_cache_manager_handler() {
        let cache_manager = CacheManager::new();
        assert_eq!(
            cache_manager.handler.lock().unwrap().total_ram,
            DEFAULT_RAM_CACHE_SIZE
        );
        let mut cache = cache_manager.apply_for_cache(Some(TEST_STRING.len()));
        cache.write_all(TEST_STRING.as_bytes()).unwrap();
        assert!(cache_manager.update_cache(TEST_URL.to_string(), cache));
        assert_eq!(
            cache_manager.handler.lock().unwrap().used_ram,
            TEST_STRING.len()
        );

        let mut cache = cache_manager.apply_for_cache(Some(TEST_STRING.len()));
        cache.write_all(TEST_STRING.as_bytes()).unwrap();
        assert!(cache_manager.update_cache(TEST_URL.to_string(), cache));
        assert_eq!(
            cache_manager.handler.lock().unwrap().used_ram,
            TEST_STRING.len()
        );
    }
}
