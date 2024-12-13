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

use std::cmp::Ordering;
use std::io::{Cursor, Write};
use std::sync::Arc;

use request_utils::task_id::TaskId;

use crate::manage::CacheManager;

const DEFAULT_TRUNK_CAPACITY: usize = 512;

pub struct RamCache {
    pub(super) task_id: TaskId,
    data: Vec<u8>,
    applied: u64,
    handle: &'static CacheManager,
}

impl Drop for RamCache {
    fn drop(&mut self) {
        if self.applied != 0 {
            info!(
                "ram cache {} released {}",
                self.task_id.brief(),
                self.applied
            );
            self.handle.ram_handle.lock().unwrap().release(self.applied);
        }
    }
}

impl RamCache {
    pub(crate) fn new(task_id: TaskId, handle: &'static CacheManager, size: Option<usize>) -> Self {
        let applied = match size {
            Some(size) => {
                if CacheManager::apply_cache(
                    &handle.ram_handle,
                    &handle.rams,
                    |a| RamCache::task_id(a),
                    size,
                ) {
                    info!(
                        "apply ram cache {} for task {} success",
                        size,
                        task_id.brief()
                    );
                    size as u64
                } else {
                    error!(
                        "apply ram cache {} for task {} failed",
                        size,
                        task_id.brief()
                    );
                    0
                }
            }
            None => 0,
        };

        Self {
            task_id,
            data: Vec::with_capacity(size.unwrap_or(DEFAULT_TRUNK_CAPACITY)),
            applied,
            handle,
        }
    }

    pub(crate) fn finish_write(mut self) -> Arc<RamCache> {
        let is_cache = self.check_size();
        let me = Arc::new(self);

        if is_cache {
            me.handle.update_ram_cache(me.clone());
        }
        me.handle.update_file_cache(me.task_id.clone(), me.clone());
        me
    }

    pub(crate) fn check_size(&mut self) -> bool {
        match (self.data.len() as u64).cmp(&self.applied) {
            Ordering::Equal => true,
            Ordering::Greater => {
                let diff = self.data.len() - self.applied as usize;
                if !CacheManager::apply_cache(
                    &self.handle.ram_handle,
                    &self.handle.rams,
                    |a| RamCache::task_id(a),
                    diff,
                ) {
                    info!(
                        "apply extra ram {} cache for task {} failed",
                        diff,
                        self.task_id.brief()
                    );
                    self.handle.ram_handle.lock().unwrap().release(self.applied);
                    self.applied = 0;
                    false
                } else {
                    info!(
                        "apply extra ram {} cache for task {} success",
                        diff,
                        self.task_id.brief()
                    );
                    self.applied = self.data.len() as u64;
                    true
                }
            }
            Ordering::Less => {
                self.handle
                    .ram_handle
                    .lock()
                    .unwrap()
                    .release(self.applied - self.data.len() as u64);
                self.applied = self.data.len() as u64;
                true
            }
        }
    }

    pub(crate) fn task_id(&self) -> &TaskId {
        &self.task_id
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn cursor(&self) -> Cursor<&[u8]> {
        Cursor::new(&self.data)
    }
}

impl Write for RamCache {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.data.flush()
    }
}

impl CacheManager {
    pub(crate) fn update_ram_cache(&'static self, cache: Arc<RamCache>) {
        let task_id = cache.task_id().clone();

        if self
            .rams
            .lock()
            .unwrap()
            .insert(task_id.clone(), cache.clone())
            .is_some()
        {
            self.files.lock().unwrap().remove(&task_id);
            info!("{} old caches delete", task_id.brief());
        }
        self.update_from_file_once.lock().unwrap().remove(&task_id);
    }
}

#[cfg(test)]
mod test {

    use std::sync::LazyLock;
    use std::thread;
    use std::time::Duration;

    use request_utils::fastrand::fast_random;
    use request_utils::test::log::init;

    use super::*;

    const TEST_STRING: &str = "你这猴子真让我欢喜";
    const TEST_STRING_SIZE: usize = TEST_STRING.len();
    const TEST_SIZE: u64 = 128;

    #[test]
    fn ut_cache_ram_try_new() {
        init();
        static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
        CACHE_MANAGER.set_ram_cache_size(TEST_SIZE);

        // cache not update
        for _ in 0..1000 {
            let task_id = TaskId::new(fast_random().to_string());
            let mut cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
            cache.write_all(TEST_STRING.as_bytes()).unwrap();
        }

        // cache update
        for _ in 0..1000 {
            let task_id = TaskId::new(fast_random().to_string());
            let mut cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));

            cache.write_all(TEST_STRING.as_bytes()).unwrap();
            CACHE_MANAGER.update_ram_cache(Arc::new(cache));
        }

        // cache update and save to file
        for _ in 0..1000 {
            let task_id = TaskId::new(fast_random().to_string());
            let mut cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
            cache.write_all(TEST_STRING.as_bytes()).unwrap();
            cache.finish_write();
            assert!(CACHE_MANAGER.rams.lock().unwrap().contains_key(&task_id));
            thread::sleep(Duration::from_millis(5));
        }
    }

    #[test]
    fn ut_cache_ram_try_new_fail() {
        init();
        static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
        CACHE_MANAGER.set_ram_cache_size(TEST_SIZE);

        let mut total = TEST_STRING_SIZE as u64;
        let mut v = vec![];
        while total < TEST_SIZE {
            let task_id = TaskId::new(fast_random().to_string());
            v.push(RamCache::new(
                task_id.clone(),
                &CACHE_MANAGER,
                Some(TEST_STRING_SIZE),
            ));
            total += TEST_STRING_SIZE as u64;
        }
        assert_eq!(
            RamCache::new(
                TaskId::new(fast_random().to_string()),
                &CACHE_MANAGER,
                Some(TEST_STRING_SIZE)
            )
            .applied,
            0
        );
        v.pop();
        RamCache::new(
            TaskId::new(fast_random().to_string()),
            &CACHE_MANAGER,
            Some(TEST_STRING_SIZE),
        );
    }
    #[test]
    fn ut_cache_ram_drop() {
        init();
        static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
        CACHE_MANAGER.set_ram_cache_size(TEST_SIZE);

        let task_id = TaskId::new(fast_random().to_string());
        let cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
        assert_eq!(
            CACHE_MANAGER.ram_handle.lock().unwrap().used_ram,
            TEST_STRING_SIZE as u64
        );
        drop(cache);
        assert_eq!(CACHE_MANAGER.ram_handle.lock().unwrap().used_ram, 0);
    }

    #[test]
    fn ut_cache_ram_temp() {
        init();
        static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
        CACHE_MANAGER.set_ram_cache_size(TEST_SIZE);
    }
}
