// Copyright (C) 2023 Huawei Device Co., Ltd.
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

use std::sync::{Arc, LazyLock};
use std::thread;
use std::time::Duration;

use request_utils::fastrand::fast_random;
use request_utils::test::log::init;

use super::*;

const TEST_STRING: &str = "你这猴子真让我欢喜";
const TEST_STRING_SIZE: usize = TEST_STRING.len();
const TEST_SIZE: u64 = 128;

// @tc.name: ut_cache_ram_try_new
// @tc.desc: Test RamCache creation and update functionality
// @tc.precon: NA
// @tc.step: 1. Initialize CacheManager with specified RAM size
//           2. Create multiple RamCache instances
//           3. Write test data to cache
//           4. Verify cache updates correctly
// @tc.expect: Cache is created and updated successfully
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
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

// @tc.name: ut_cache_ram_try_new_fail
// @tc.desc: Test RamCache creation failure when exceeding capacity
// @tc.precon: NA
// @tc.step: 1. Initialize CacheManager with limited RAM size
//           2. Fill cache to maximum capacity
//           3. Attempt to create additional RamCache instance
// @tc.expect: New cache creation fails when exceeding capacity
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level2
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

// @tc.name: ut_cache_ram_drop
// @tc.desc: Test RamCache memory release on drop
// @tc.precon: NA
// @tc.step: 1. Create RamCache instance
//           2. Verify initial used RAM
//           3. Drop the cache instance
//           4. Verify RAM is released
// @tc.expect: Used RAM returns to zero after drop
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
#[test]
fn ut_cache_ram_drop() {
    init();
    static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
    CACHE_MANAGER.set_ram_cache_size(TEST_SIZE);

    let task_id = TaskId::new(fast_random().to_string());
    let cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
    assert_eq!(
        CACHE_MANAGER.ram_handle.lock().unwrap().used_capacity,
        TEST_STRING_SIZE as u64
    );
    drop(cache);
    assert_eq!(CACHE_MANAGER.ram_handle.lock().unwrap().used_capacity, 0);
}

// @tc.name: ut_cache_ram_temp
// @tc.desc: Test temporary RamCache functionality
// @tc.precon: NA
// @tc.step: 1. Initialize CacheManager with specified RAM size
// @tc.expect: Cache manager initializes without errors
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
#[test]
fn ut_cache_ram_temp() {
    init();
    static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
    CACHE_MANAGER.set_ram_cache_size(TEST_SIZE);
}
