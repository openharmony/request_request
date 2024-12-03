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
use std::collections::HashMap;
use std::hash::Hash;
use std::ptr;

struct Node<K, V> {
    key: K,
    value: V,
    prev: *mut Node<K, V>,
    next: *mut Node<K, V>,
}

struct LinkedList<K, V> {
    head: *mut Node<K, V>,
    tail: *mut Node<K, V>,
}

impl<K, V> LinkedList<K, V> {
    fn new() -> Self {
        LinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    fn push_front(&mut self, node: *mut Node<K, V>) {
        unsafe {
            (*node).prev = ptr::null_mut();
            (*node).next = self.head;

            if !self.head.is_null() {
                (*self.head).prev = node;
            }
            self.head = node;

            if self.tail.is_null() {
                self.tail = node;
            }
        }
    }

    fn remove(&mut self, node: *mut Node<K, V>) {
        unsafe {
            if !(*node).prev.is_null() {
                (*(*node).prev).next = (*node).next;
            } else {
                self.head = (*node).next;
            }

            if !(*node).next.is_null() {
                (*(*node).next).prev = (*node).prev;
            } else {
                self.tail = (*node).prev;
            }
        }
    }

    fn pop_back(&mut self) -> *mut Node<K, V> {
        if self.tail.is_null() {
            return ptr::null_mut();
        }
        let node = self.tail;
        self.remove(node);
        node
    }
}

impl<K, V> Drop for LinkedList<K, V> {
    fn drop(&mut self) {
        let mut current = self.head;
        while !current.is_null() {
            unsafe {
                let next = (*current).next;
                let _ = Box::from_raw(current);
                current = next;
            }
        }
    }
}

pub struct LRUCache<K, V> {
    map: HashMap<K, *mut Node<K, V>>,
    list: LinkedList<K, V>,
}

impl<K: Hash + Eq + Clone, V> LRUCache<K, V> {
    pub fn new() -> Self {
        LRUCache {
            map: HashMap::new(),
            list: LinkedList::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(&node) = self.map.get(key) {
            self.list.remove(node);
            self.list.push_front(node);
            unsafe {
                return Some(&(*node).value);
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if let Some(&mut node) = self.map.get_mut(key) {
            self.list.remove(node);
            self.list.push_front(node);
            unsafe {
                return Some(&mut (*node).value);
            }
        }
        None
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.map.entry(key) {
            Entry::Occupied(addr) => {
                self.list.remove(*addr.get());
                self.list.push_front(*addr.get());
                unsafe {
                    let old = std::mem::replace(&mut (*(*addr.get())).value, value);
                    Some(old)
                }
            }
            Entry::Vacant(addr) => {
                let new_node = Box::into_raw(Box::new(Node {
                    key: addr.key().clone(),
                    value,
                    prev: ptr::null_mut(),
                    next: ptr::null_mut(),
                }));
                self.list.push_front(new_node);
                addr.insert(new_node);
                None
            }
        }
    }

    pub fn pop(&mut self) -> Option<V> {
        let old_node = self.list.pop_back();
        if !old_node.is_null() {
            unsafe {
                let old_key = (*old_node).key.clone();
                self.map.remove(&old_key);
                let node = Box::from_raw(old_node);
                Some(node.value)
            }
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(node) = self.map.remove(key) {
            self.list.remove(node);
            unsafe {
                let node = Box::from_raw(node);
                return Some(node.value);
            }
        }
        None
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.map.contains_key(k)
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}

impl<K: Eq + Hash + Clone, V> Default for LRUCache<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<K, V> Send for LRUCache<K, V> {}

#[cfg(test)]
mod ut_test {
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
}
