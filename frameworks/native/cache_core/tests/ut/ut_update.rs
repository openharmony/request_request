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

#[cfg(test)]
mod ut_update {
    use super::*;
    use crate::{CacheManager, RamCache};
    use request_utils::task_id::TaskId;
    use std::sync::Arc;

    // Mock CacheManager for testing
    struct MockCacheManager;
    impl CacheManager for MockCacheManager {
        fn get_cache_dir(&self) -> Option<String> {
            Some("./test_cache".to_string())
        }
    }

    impl MockCacheManager {
        fn new() -> &'static Self {
            Box::leak(Box::new(Self))
        }
    }

    // @tc.name: ut_updater_new
    // @tc.desc: Test creation of Updater instance
    // @tc.precon: NA
    // @tc.step: 1. Create TaskId and MockCacheManager
    // 2. Create Updater with new()
    // 3. Verify Updater is properly initialized
    // @tc.expect: Updater instance is created with empty cache
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_updater_new() {
        let task_id = TaskId::new("test_id".to_string());
        let cache_manager = MockCacheManager::new();
        let updater = Updater::new(task_id.clone(), cache_manager);

        assert_eq!(updater.task_id, task_id);
        assert!(updater.cache.is_none());
    }

    // @tc.name: ut_updater_cache_finish_empty
    // @tc.desc: Test cache_finish with empty cache
    // @tc.precon: NA
    // @tc.step: 1. Create Updater with empty cache
    // 2. Call cache_finish()
    // 3. Verify returned RamCache
    // @tc.expect: Returns new RamCache with size 0
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_updater_cache_finish_empty() {
        let task_id = TaskId::new("test_id".to_string());
        let cache_manager = MockCacheManager::new();
        let mut updater = Updater::new(task_id.clone(), cache_manager);

        let result = updater.cache_finish();
        assert_eq!(result.size(), 0);
    }

    // @tc.name: ut_updater_cache_receive_basic
    // @tc.desc: Test basic cache_receive functionality
    // @tc.precon: NA
    // @tc.step: 1. Create Updater instance
    // 2. Call cache_receive with sample data
    // 3. Verify cache is populated
    // @tc.expect: Cache contains received data
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_updater_cache_receive_basic() {
        let task_id = TaskId::new("test_id".to_string());
        let cache_manager = MockCacheManager::new();
        let mut updater = Updater::new(task_id.clone(), cache_manager);
        let test_data = b"test_data";

        updater.cache_receive(test_data, || Some(test_data.len()));
        assert!(updater.cache.is_some());
        assert_eq!(updater.cache.as_ref().unwrap().size(), test_data.len());
    }

    // @tc.name: ut_updater_cache_receive_multiple
    // @tc.desc: Test multiple cache_receive calls
    // @tc.precon: NA
    // @tc.step: 1. Create Updater instance
    // 2. Call cache_receive with multiple data chunks
    // 3. Verify total size matches sum of chunks
    // @tc.expect: Cache size equals sum of all received data
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_updater_cache_receive_multiple() {
        let task_id = TaskId::new("test_id".to_string());
        let cache_manager = MockCacheManager::new();
        let mut updater = Updater::new(task_id.clone(), cache_manager);
        let data1 = b"chunk1";
        let data2 = b"chunk2";

        updater.cache_receive(data1, || Some(data1.len() + data2.len()));
        updater.cache_receive(data2, || None);
        assert_eq!(
            updater.cache.as_ref().unwrap().size(),
            data1.len() + data2.len()
        );
    }

    // @tc.name: ut_updater_reset_cache
    // @tc.desc: Test reset_cache functionality
    // @tc.precon: NA
    // @tc.step: 1. Create Updater with populated cache
    // 2. Call reset_cache()
    // 3. Verify cache is cleared
    // @tc.expect: Cache is None after reset
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_updater_reset_cache() {
        let task_id = TaskId::new("test_id".to_string());
        let cache_manager = MockCacheManager::new();
        let mut updater = Updater::new(task_id.clone(), cache_manager);

        updater.cache_receive(b"test", || Some(4));
        assert!(updater.cache.is_some());

        updater.reset_cache();
        assert!(updater.cache.is_none());
    }

    // @tc.name: ut_updater_cache_receive_empty_data
    // @tc.desc: Test cache_receive with empty data
    // @tc.precon: NA
    // @tc.step: 1. Create Updater instance
    // 2. Call cache_receive with empty data
    // 3. Verify cache behavior
    // @tc.expect: Cache is created but size remains 0
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_updater_cache_receive_empty_data() {
        let task_id = TaskId::new("test_id".to_string());
        let cache_manager = MockCacheManager::new();
        let mut updater = Updater::new(task_id.clone(), cache_manager);

        updater.cache_receive(b"", || Some(0));
        assert!(updater.cache.is_some());
        assert_eq!(updater.cache.as_ref().unwrap().size(), 0);
    }

    // @tc.name: ut_updater_cache_finish_populated
    // @tc.desc: Test cache_finish with populated cache
    // @tc.precon: NA
    // @tc.step: 1. Create Updater and populate cache
    // 2. Call cache_finish()
    // 3. Verify returned RamCache
    // @tc.expect: Returns Arc<RamCache> with correct size
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 3
    #[test]
    fn ut_updater_cache_finish_populated() {
        let task_id = TaskId::new("test_id".to_string());
        let cache_manager = MockCacheManager::new();
        let mut updater = Updater::new(task_id.clone(), cache_manager);
        let test_data = b"test_data";

        updater.cache_receive(test_data, || Some(test_data.len()));
        let result = updater.cache_finish();
        assert_eq!(result.size(), test_data.len());
        assert!(updater.cache.is_none()); // Cache should be taken
    }
}
