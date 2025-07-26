// Copyright (C) 2025 Huawei Device Co., Ltd.
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

use super::LRUCache;

#[derive(Debug, Eq, PartialEq)]
struct Cache {
    data_count: usize,
}

impl Cache {
    pub(crate) fn from_u(init: usize) -> Self {
        Cache { data_count: init }
    }

    pub(crate) fn add(&mut self, num: usize) {
        self.data_count += num
    }
}

// @tc.name: ut_lru_cache_empty
// @tc.desc: Test LRUCache behavior when it's empty
// @tc.precon: NA
// @tc.step: 1. Create a new empty LRUCache instance
//           2. Call get, get_mut, len, contains_key, is_empty, pop and remove
//              methods
//           3. Verify all return values and states
// @tc.expect: All methods return expected values for empty cache (None for get
// operations, 0 for len, true for is_empty)
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
#[test]
fn ut_test_empty() {
    let mut cache = LRUCache::<&str, Cache>::new();
    assert_eq!(None, cache.get(&"key1"));
    assert_eq!(None, cache.get_mut(&"key1"));
    assert_eq!(0, cache.len());
    assert!(!cache.contains_key(&"key1"));
    assert!(cache.is_empty());
    assert_eq!(None, cache.pop());
    assert_eq!(None, cache.remove(&"key1"));
}

// @tc.name: ut_lru_cache_insert
// @tc.desc: Test basic insert and access operations of LRUCache
// @tc.precon: NA
// @tc.step: 1. Create a new LRUCache instance
//           2. Insert a key-value pair
//           3. Verify get, get_mut, len, contains_key and is_empty methods
//           4. Modify value using get_mut
//           5. Verify pop and remove operations
// @tc.expect: Insert succeeds, get operations return correct values, len is 1,
// contains_key returns true, pop returns inserted value
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
#[test]
fn ut_test_insert() {
    let mut cache = LRUCache::new();
    assert_eq!(None, cache.insert("key1", Cache::from_u(0)));
    assert_eq!(Some(&Cache::from_u(0)), cache.get(&"key1"));
    assert_eq!(Some(&mut Cache::from_u(0)), cache.get_mut(&"key1"));
    assert_eq!(
        Some(&mut Cache::from_u(1)),
        cache.get_mut(&"key1").map(|cache| {
            cache.add(1);
            cache
        })
    );
    assert_eq!(1, cache.len());
    assert!(cache.contains_key(&"key1"));
    assert!(!cache.is_empty());
    assert_eq!(Some(Cache::from_u(1)), cache.pop());
    assert_eq!(None, cache.remove(&"key1"));
}

// @tc.name: ut_lru_cache_insert_dump
// @tc.desc: Test LRUCache behavior with multiple insertions and updates
// @tc.precon: NA
// @tc.step: 1. Create a new LRUCache instance
//           2. Insert multiple key-value pairs
//           3. Update an existing key
//           4. Verify cache length and pop behavior
// @tc.expect: All insertions succeed, update returns old value, cache length is
// correct, pop removes oldest entry
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level3
#[test]
fn ut_test_insert_dump() {
    let mut cache = LRUCache::new();
    cache.insert("key0", Cache::from_u(0));
    cache.insert("key1", Cache::from_u(1));
    cache.insert("key2", Cache::from_u(2));
    cache.insert("key3", Cache::from_u(3));
    assert_eq!(
        Some(Cache::from_u(0)),
        cache.insert("key0", Cache::from_u(4))
    );
    assert_eq!(4, cache.len());
    assert_eq!(Some(Cache::from_u(1)), cache.pop());
    assert_eq!(None, cache.get(&"key1"));
    assert_eq!(Some(&Cache::from_u(4)), cache.get(&"key0"));
}

// @tc.name: ut_lru_cache_pop
// @tc.desc: Test pop operation on LRUCache with single entry
// @tc.precon: NA
// @tc.step: 1. Create a new LRUCache instance
//           2. Insert a key-value pair
//           3. Call pop method
//           4. Verify cache state after pop
// @tc.expect: Pop returns the inserted value, cache becomes empty
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
#[test]
fn ut_test_pop() {
    let mut cache = LRUCache::new();
    cache.insert("key1", Cache::from_u(1));
    assert_eq!(Some(Cache::from_u(1)), cache.pop());
    assert_eq!(None, cache.get(&"key1"));
    assert_eq!(None, cache.get_mut(&"key1"));
    assert_eq!(0, cache.len());
    assert!(!cache.contains_key(&"key1"));
    assert!(cache.is_empty());
    assert_eq!(None, cache.pop());
    assert_eq!(None, cache.remove(&"key1"));
}

// @tc.name: ut_lru_cache_pop_remaining
// @tc.desc: Test pop operation leaves remaining entries in LRUCache
// @tc.precon: NA
// @tc.step: 1. Create a new LRUCache instance
//           2. Insert multiple key-value pairs
//           3. Call pop method
//           4. Verify cache state and remaining entries
// @tc.expect: Pop removes the oldest entry, remaining entries are accessible,
// cache length is reduced by 1
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level2
#[test]
fn ut_test_pop_remaining() {
    let mut cache = LRUCache::new();
    cache.insert("key0", Cache::from_u(0));
    cache.insert("key1", Cache::from_u(1));
    cache.insert("key2", Cache::from_u(2));
    cache.insert("key3", Cache::from_u(3));
    assert_eq!(Some(Cache::from_u(0)), cache.pop());
    assert_eq!(None, cache.get(&"key0"));
    assert_eq!(None, cache.get_mut(&"key0"));
    assert_eq!(3, cache.len());
    assert!(!cache.contains_key(&"key0"));
    assert!(!cache.is_empty());
}

// @tc.name: ut_lru_cache_remove
// @tc.desc: Test remove operation on LRUCache with single entry
// @tc.precon: NA
// @tc.step: 1. Create a new LRUCache instance
//           2. Insert a key-value pair
//           3. Call remove method with the key
//           4. Verify cache state after removal
// @tc.expect: Remove returns the inserted value, cache becomes empty
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level1
#[test]
fn ut_test_remove() {
    let mut cache = LRUCache::new();
    cache.insert("key1", Cache::from_u(1));
    assert_eq!(Some(Cache::from_u(1)), cache.remove(&"key1"));
    assert_eq!(None, cache.get(&"key1"));
    assert_eq!(None, cache.get_mut(&"key1"));
    assert_eq!(0, cache.len());
    assert!(!cache.contains_key(&"key1"));
    assert!(cache.is_empty());
    assert_eq!(None, cache.pop());
    assert_eq!(None, cache.remove(&"key1"));
}

// @tc.name: ut_lru_cache_remove_remaining
// @tc.desc: Test remove operation leaves remaining entries in LRUCache
// @tc.precon: NA
// @tc.step: 1. Create a new LRUCache instance
//           2. Insert multiple key-value pairs
//           3. Call remove method with one key
//           4. Verify cache state and remaining entries
// @tc.expect: Remove removes specified entry, remaining entries are accessible,
// cache length is reduced by 1
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level2
#[test]
fn ut_test_remove_remaining() {
    let mut cache = LRUCache::new();
    cache.insert("key0", Cache::from_u(0));
    cache.insert("key1", Cache::from_u(1));
    cache.insert("key2", Cache::from_u(2));
    cache.insert("key3", Cache::from_u(3));
    assert_eq!(Some(Cache::from_u(1)), cache.remove(&"key1"));
    assert_eq!(None, cache.get(&"key1"));
    assert_eq!(None, cache.get_mut(&"key1"));
    assert_eq!(3, cache.len());
    assert!(!cache.contains_key(&"key1"));
    assert!(!cache.is_empty());
    assert_eq!(None, cache.remove(&"key1"));
}

// @tc.name: ut_lru_cache_insert_after_pop
// @tc.desc: Test inserting entries after pop operation in LRUCache
// @tc.precon: NA
// @tc.step: 1. Create a new LRUCache instance
//           2. Insert entries and perform pop
//           3. Insert new entry with previously popped key
//           4. Verify cache state and entry accessibility
// @tc.expect: New entry is inserted successfully, can be accessed and modified,
// pop removes correct oldest entry
// @tc.type: FUNC
// @tc.require: issue#ICN31I
// @tc.level: level3
#[test]
fn ut_test_insert_after_pop() {
    let mut cache = LRUCache::new();
    cache.insert("key0", Cache::from_u(0));
    cache.insert("key1", Cache::from_u(1));
    assert_eq!(Some(Cache::from_u(0)), cache.pop());
    cache.insert("key0", Cache::from_u(0));
    assert_eq!(Some(&Cache::from_u(0)), cache.get(&"key0"));
    assert_eq!(Some(&mut Cache::from_u(0)), cache.get_mut(&"key0"));
    assert_eq!(
        Some(&mut Cache::from_u(4)),
        cache.get_mut(&"key0").map(|cache| {
            cache.add(4);
            cache
        })
    );
    assert_eq!(Some(&Cache::from_u(4)), cache.get(&"key0"));
    assert_eq!(2, cache.len());
    assert!(cache.contains_key(&"key0"));
    assert!(!cache.is_empty());
    assert_eq!(Some(Cache::from_u(1)), cache.pop());
}
