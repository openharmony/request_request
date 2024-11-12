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

use crate::agent::TaskId;
use crate::cache::CacheManager;

const DEFAULT_TRUNK_CAPACITY: usize = 512;

pub struct RamCache {
    pub(super) task_id: TaskId,
    data: Vec<u8>,
    applied: u64,
    handle: &'static CacheManager,

    is_temp: bool,
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
    pub(crate) fn temp(
        task_id: TaskId,
        handle: &'static CacheManager,
        size: Option<usize>,
    ) -> Self {
        Self {
            task_id,
            data: Vec::with_capacity(size.unwrap_or(DEFAULT_TRUNK_CAPACITY)),
            applied: 0,
            handle,
            is_temp: true,
        }
    }

    pub(crate) fn try_new(
        task_id: TaskId,
        handle: &'static CacheManager,
        size: usize,
    ) -> Option<Self> {
        info!(
            "try apply new ram cache {} for task {}",
            size,
            task_id.brief()
        );

        if !CacheManager::apply_cache(
            &handle.ram_handle,
            &handle.rams,
            |a| RamCache::task_id(a),
            size,
        ) {
            info!("apply ram cache for task {} failed", task_id.brief());
            return None;
        }

        info!("apply ram cache for task {} success", task_id.brief());
        Some(Self {
            task_id,
            data: Vec::with_capacity(size),
            applied: size as u64,
            handle,
            is_temp: false,
        })
    }

    pub(crate) fn finish_write(mut self) -> Arc<RamCache> {
        if self.is_temp || !self.check_size() {
            return Arc::new(self);
        }
        let me = Arc::new(self);
        me.handle.update_cache(me.clone());
        me
    }

    fn check_size(&mut self) -> bool {
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
                    false
                } else {
                    info!(
                        "apply extra ram {} cache for task {} success",
                        diff,
                        self.task_id.brief()
                    );
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

    pub(crate) fn size(&self) -> usize {
        self.data.len()
    }

    pub(crate) fn cursor(&self) -> Cursor<&[u8]> {
        Cursor::new(&self.data)
    }

    pub(super) fn is_temp(&self) -> bool {
        self.is_temp
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
    fn update_cache(&'static self, cache: Arc<RamCache>) {
        self.update_cache_inner(cache.task_id().clone(), cache, false);
    }

    pub(super) fn update_from_file(&'static self, task_id: TaskId, cache: Arc<RamCache>) {
        self.update_cache_inner(task_id, cache, true);
    }

    fn update_cache_inner(&'static self, task_id: TaskId, cache: Arc<RamCache>, from_file: bool) {
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
        info!("{} ram cache updated", task_id.brief());
        if !from_file {
            self.update_file_cache(task_id, cache);
        }
    }
}

#[cfg(test)]
mod test {

    use std::sync::LazyLock;
    use std::thread;
    use std::time::Duration;

    use super::*;
    use crate::init;
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
            let task_id = TaskId::random();
            let mut cache =
                RamCache::try_new(task_id.clone(), &CACHE_MANAGER, TEST_STRING_SIZE).unwrap();
            cache.write_all(TEST_STRING.as_bytes()).unwrap();
        }

        // cache update
        for _ in 0..1000 {
            let task_id = TaskId::random();
            let mut cache =
                RamCache::try_new(task_id.clone(), &CACHE_MANAGER, TEST_STRING_SIZE).unwrap();
            cache.write_all(TEST_STRING.as_bytes()).unwrap();
            CACHE_MANAGER.update_cache_inner(task_id, Arc::new(cache), true);
        }

        // cache update and save to file
        for _ in 0..1000 {
            let task_id = TaskId::random();
            let mut cache =
                RamCache::try_new(task_id.clone(), &CACHE_MANAGER, TEST_STRING_SIZE).unwrap();
            cache.write_all(TEST_STRING.as_bytes()).unwrap();
            cache.finish_write();
            assert!(CACHE_MANAGER.rams.lock().unwrap().contains_key(&task_id));
            thread::sleep(Duration::from_millis(1));
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
            let task_id = TaskId::random();
            v.push(RamCache::try_new(task_id.clone(), &CACHE_MANAGER, TEST_STRING_SIZE).unwrap());
            total += TEST_STRING_SIZE as u64;
        }
        assert!(RamCache::try_new(TaskId::random(), &CACHE_MANAGER, TEST_STRING_SIZE).is_none());
        v.pop();
        RamCache::try_new(TaskId::random(), &CACHE_MANAGER, TEST_STRING_SIZE).unwrap();
    }
    #[test]
    fn ut_cache_ram_drop() {
        init();
        static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
        CACHE_MANAGER.set_ram_cache_size(TEST_SIZE);

        let task_id = TaskId::random();
        let cache = RamCache::try_new(task_id.clone(), &CACHE_MANAGER, TEST_STRING_SIZE).unwrap();
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

        let task_id = TaskId::random();
        let _cache = RamCache::try_new(task_id, &CACHE_MANAGER, TEST_STRING_SIZE).unwrap();
        let task_id = TaskId::random();
        let cache_temp = RamCache::temp(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));

        // temp cache do not apply or release ram size.
        assert_eq!(
            CACHE_MANAGER.ram_handle.lock().unwrap().used_ram,
            TEST_STRING_SIZE as u64
        );
        drop(cache_temp);
        assert_eq!(
            CACHE_MANAGER.ram_handle.lock().unwrap().used_ram,
            TEST_STRING_SIZE as u64
        );

        // temp cache do not update to cache manager.
        let task_id = TaskId::random();
        let cache_temp = RamCache::temp(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
        cache_temp.finish_write();
        assert!(!CACHE_MANAGER.rams.lock().unwrap().contains_key(&task_id));
    }
}
