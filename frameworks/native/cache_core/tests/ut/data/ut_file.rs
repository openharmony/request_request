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
use std::sync::{Arc, LazyLock};
use std::time::Duration;
use std::{fs, io};

use request_utils::fastrand::fast_random;
use request_utils::task_id::TaskId;
use request_utils::test::log::init;

use super::{FILE_STORE_DIR, *};
const TEST_STRING: &str = "你这猴子真让我欢喜";
const TEST_STRING_SIZE: usize = TEST_STRING.len();
const TEST_SIZE: u64 = 128;

// @tc.name: ut_cache_file_create
// @tc.desc: Test the creation of file cache
// @tc.precon: NA
// @tc.step: 1. Initialize CacheManager with test size
//           2. Create RamCache with test data
//           3. Call FileCache::try_create method
// @tc.expect: File cache is created successfully
// @tc.type: FUNC
// @tc.require: issue#ICN31I
#[test]
fn ut_cache_file_create() {
    init();
    static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
    CACHE_MANAGER.set_file_cache_size(TEST_SIZE);

    init_curr_store_dir();
    // cache not update
    for _ in 0..1000 {
        let task_id = TaskId::new(fast_random().to_string());
        let mut ram_cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
        ram_cache.write_all(TEST_STRING.as_bytes()).unwrap();
        FileCache::try_create(task_id.clone(), &CACHE_MANAGER, Arc::new(ram_cache)).unwrap();
    }

    // cache update
    for _ in 0..1000 {
        let task_id = TaskId::new(fast_random().to_string());
        let mut ram_cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
        ram_cache.write_all(TEST_STRING.as_bytes()).unwrap();
        let file_cache =
            FileCache::try_create(task_id.clone(), &CACHE_MANAGER, Arc::new(ram_cache)).unwrap();
        CACHE_MANAGER
            .files
            .lock()
            .unwrap()
            .insert(task_id, file_cache);
    }
}

// @tc.name: ut_cache_file_try_new_fail
// @tc.desc: Test failure to create file cache when size exceeds limit
// @tc.precon: NA
// @tc.step: 1. Initialize CacheManager with limited size
//           2. Fill cache until full
//           3. Attempt to create another file cache
// @tc.expect: File cache creation returns None
// @tc.type: FUNC
// @tc.require: issue#ICN31I
#[test]
fn ut_cache_file_try_new_fail() {
    init();
    static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
    CACHE_MANAGER.set_file_cache_size(TEST_SIZE);

    init_curr_store_dir();
    let mut total = TEST_STRING_SIZE as u64;
    let mut v = vec![];
    while total < TEST_SIZE {
        let task_id = TaskId::new(fast_random().to_string());
        let mut ram_cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
        ram_cache.write_all(TEST_STRING.as_bytes()).unwrap();
        v.push(
            FileCache::try_create(task_id.clone(), &CACHE_MANAGER, Arc::new(ram_cache)).unwrap(),
        );
        total += TEST_STRING_SIZE as u64;
    }
    let task_id = TaskId::new(fast_random().to_string());
    let mut ram_cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
    ram_cache.write_all(TEST_STRING.as_bytes()).unwrap();
    assert!(FileCache::try_create(task_id.clone(), &CACHE_MANAGER, Arc::new(ram_cache)).is_none());
    v.pop();
    let task_id = TaskId::new(fast_random().to_string());
    let mut ram_cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
    ram_cache.write_all(TEST_STRING.as_bytes()).unwrap();
    FileCache::try_create(task_id.clone(), &CACHE_MANAGER, Arc::new(ram_cache)).unwrap();
}

// @tc.name: ut_cache_file_drop
// @tc.desc: Test file cache drop and resource release
// @tc.precon: NA
// @tc.step: 1. Create FileCache instance
//           2. Drop the FileCache
//           3. Check used_ram is released
// @tc.expect: used_ram is reset to 0 after drop
// @tc.type: FUNC
// @tc.require: issue#ICN31I
#[test]
fn ut_cache_file_drop() {
    init();
    static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
    CACHE_MANAGER.set_file_cache_size(TEST_SIZE);

    init_curr_store_dir();
    let task_id = TaskId::new(fast_random().to_string());
    let mut ram_cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
    ram_cache.write_all(TEST_STRING.as_bytes()).unwrap();
    let file_cache =
        FileCache::try_create(task_id.clone(), &CACHE_MANAGER, Arc::new(ram_cache)).unwrap();
    assert_eq!(
        CACHE_MANAGER.file_handle.lock().unwrap().used_capacity,
        TEST_STRING_SIZE as u64
    );
    drop(file_cache);
    assert_eq!(CACHE_MANAGER.file_handle.lock().unwrap().used_capacity, 0);
}

// @tc.name: ut_cache_file_content
// @tc.desc: Test file cache content integrity
// @tc.precon: NA
// @tc.step: 1. Create FileCache with test data
//           2. Open the cache file
//           3. Read and verify content
// @tc.expect: Read content matches original test string
// @tc.type: FUNC
// @tc.require: issue#ICN31I
#[test]
fn ut_cache_file_content() {
    init();
    static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
    CACHE_MANAGER.set_file_cache_size(TEST_SIZE);

    init_curr_store_dir();
    let task_id = TaskId::new(fast_random().to_string());
    let mut ram_cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
    ram_cache.write_all(TEST_STRING.as_bytes()).unwrap();
    let file_cache =
        FileCache::try_create(task_id.clone(), &CACHE_MANAGER, Arc::new(ram_cache)).unwrap();
    let mut file = file_cache.open().unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    assert_eq!(buf, TEST_STRING);
}

// @tc.name: ut_cache_file_restore_files
// @tc.desc: Test file cache restoration functionality
// @tc.precon: NA
// @tc.step: 1. Create test directory with sample files
//           2. Call restore_files_inner function
//           3. Verify restored task IDs and cleanup
// @tc.expect: Only finished files are restored in correct order
// @tc.type: FUNC
// @tc.require: issue#ICN31I
#[test]
fn ut_cache_file_restore_files() {
    init();
    const TEST_DIR: &str = "restore_test";

    // The first to create are the first to come out
    init_curr_store_dir();
    let path = unsafe { FILE_STORE_DIR.join(String::from(TEST_DIR)).unwrap() };

    fs::create_dir_all(&path).unwrap();
    for i in 0..10 {
        // not finished will not come out and will be deleted
        let path = if i % 2 == 0 {
            path.join(format!("{}{}", i, FINISH_SUFFIX))
        } else {
            path.join(format!("{}", i))
        };
        fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(path)
            .unwrap();
        std::thread::sleep(Duration::from_millis(10));
    }
    for (i, file) in restore_files_inner(path.as_path()).enumerate() {
        assert_eq!(file.to_string(), (i * 2).to_string());
    }
    for i in 0..5 {
        let path = path.join(format!("{}", i));
        assert!(fs::metadata(path).is_err_and(|e| e.kind() == io::ErrorKind::NotFound));
    }
    fs::remove_dir_all(&path).unwrap();
}

// @tc.name: ut_cache_file_update_ram_from_file
// @tc.desc: Test updating RAM cache from file
// @tc.precon: NA
// @tc.step: 1. Create and store FileCache
//           2. Spawn multiple threads to update RAM from file
//           3. Verify all threads successfully retrieve cache
// @tc.expect: All threads return valid cache data
// @tc.type: FUNC
// @tc.require: issue#ICN31I
#[test]
fn ut_cache_file_update_ram_from_file() {
    init();
    static CACHE_MANAGER: LazyLock<CacheManager> = LazyLock::new(CacheManager::new);
    CACHE_MANAGER.set_file_cache_size(TEST_SIZE);

    init_curr_store_dir();
    let task_id = TaskId::new(fast_random().to_string());
    let mut ram_cache = RamCache::new(task_id.clone(), &CACHE_MANAGER, Some(TEST_STRING_SIZE));
    ram_cache.write_all(TEST_STRING.as_bytes()).unwrap();
    let file_cache =
        FileCache::try_create(task_id.clone(), &CACHE_MANAGER, Arc::new(ram_cache)).unwrap();
    CACHE_MANAGER
        .files
        .lock()
        .unwrap()
        .insert(task_id.clone(), file_cache);

    let mut v = vec![];
    for _ in 0..1000 {
        let task_id = task_id.clone();
        v.push(std::thread::spawn(move || {
            let Some(_) = CACHE_MANAGER.update_ram_from_file(&task_id) else {
                return false;
            };
            true
        }))
    }
    for j in v {
        assert!(j.join().unwrap());
    }
}
