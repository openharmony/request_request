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

use std::io::{Read, Write};
use std::sync::LazyLock;
use std::thread;
use std::time::Duration;

use request_utils::fastrand::fast_random;
use request_utils::test::log::init;

use super::*;
const TEST_STRING: &str = "你这猴子真让我欢喜";
const TEST_STRING_SIZE: usize = TEST_STRING.len();

// @tc.name: ut_cache_manager_update_file
// @tc.desc: Test cache manager updates file cache from RAM
// @tc.precon: NA
// @tc.step: 1. Create RamCache with test data
//           2. Call finish_write method
//           3. Verify file cache content
// @tc.expect: File cache contains the same data as RAM cache
// @tc.type: FUNC
// @tc.require: issue#ICN31I
#[test]
fn ut_cache_manager_update_file() {
    init();
    let task_id = TaskId::new(fast_random().to_string());
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

// @tc.name: ut_cache_manager_get
// @tc.desc: Test cache manager retrieves cache data
// @tc.precon: NA
// @tc.step: 1. Create and populate RamCache
//           2. Call get_cache method
//           3. Verify retrieved data matches original
// @tc.expect: Cache data is successfully retrieved and matches
// @tc.type: FUNC
// @tc.require: issue#ICN31I
#[test]
fn ut_cache_manager_get() {
    init();
    let task_id = TaskId::new(fast_random().to_string());
    static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);

    let mut cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));

    cache.write_all(TEST_STRING.as_bytes()).unwrap();
    cache.finish_write();

    let cache = CACHE_MANAGER.get_cache(&task_id).unwrap();
    let mut buf = String::new();
    cache.cursor().read_to_string(&mut buf).unwrap();
    assert_eq!(buf, TEST_STRING);
}

// @tc.name: ut_cache_manager_cache_from_file
// @tc.desc: Test cache manager retrieves cache from file
// @tc.precon: NA
// @tc.step: 1. Create file cache with test data
//           2. Remove RAM cache
//           3. Call get_cache and verify data
// @tc.expect: Cache data is successfully retrieved from file
// @tc.type: FUNC
// @tc.require: issue#ICN31I
#[test]
fn ut_cache_manager_cache_from_file() {
    init();
    let task_id = TaskId::new(fast_random().to_string());

    static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
    let mut cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
    cache.write_all(TEST_STRING.as_bytes()).unwrap();
    cache.finish_write();

    thread::sleep(Duration::from_millis(100));
    CACHE_MANAGER.rams.lock().unwrap().remove(&task_id);

    let mut v = vec![];
    for _ in 0..1 {
        let task_id = task_id.clone();
        v.push(std::thread::spawn(move || {
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

// @tc.name: ut_cache_manager_cache_from_file_clean
// @tc.desc: Test cache manager cleans up temporary data after file retrieval
// @tc.precon: NA
// @tc.step: 1. Create and store file cache
//           2. Retrieve cache from file
//           3. Verify temporary caches are removed
// @tc.expect: backup_rams and update_from_file_once are empty
// @tc.type: FUNC
// @tc.require: issue#ICN31I
#[test]
fn ut_cache_manager_cache_from_file_clean() {
    init();
    let task_id = TaskId::new(fast_random().to_string());
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

// @tc.name: ut_cache_manager_update_same
// @tc.desc: Test cache manager updates existing cache with new data
// @tc.precon: NA
// @tc.step: 1. Create initial cache with test data
//           2. Update cache with new data
//           3. Verify retrieved data matches updated content
// @tc.expect: Updated cache data is successfully stored and retrieved
// @tc.type: FUNC
// @tc.require: issue#ICN31I
#[test]
fn ut_cache_manager_update_same() {
    init();
    let task_id = TaskId::new(fast_random().to_string());
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