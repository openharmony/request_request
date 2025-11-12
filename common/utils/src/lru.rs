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

//! Least Recently Used (LRU) cache implementation.
//!
//! This module provides an efficient LRU cache implementation using a combination
//! of a hash map for O(1) lookups and a doubly linked list for O(1) insertions and
//! deletions from both ends.

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::ptr;

/// A node in the doubly linked list used to track access order.
struct Node<K, V> {
    /// The key associated with this node.
    key: K,
    /// The value stored in this node.
    value: V,
    /// Pointer to the previous node in the list.
    prev: *mut Node<K, V>,
    /// Pointer to the next node in the list.
    next: *mut Node<K, V>,
}

/// A doubly linked list used to maintain the access order of cache entries.
struct LinkedList<K, V> {
    /// Pointer to the head of the list (most recently used).
    head: *mut Node<K, V>,
    /// Pointer to the tail of the list (least recently used).
    tail: *mut Node<K, V>,
}

impl<K, V> LinkedList<K, V> {
    /// Creates a new empty linked list.
    fn new() -> Self {
        LinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    /// Adds a node to the front of the list (most recently used position).
    ///
    /// # Safety
    ///
    /// Assumes the provided node pointer is valid and not null.
    fn push_front(&mut self, node: *mut Node<K, V>) {
        unsafe {
            // Initialize node's pointers
            (*node).prev = ptr::null_mut();
            (*node).next = self.head;

            // Update head's previous pointer if head exists
            if !self.head.is_null() {
                (*self.head).prev = node;
            }
            // Update head to new node
            self.head = node;

            // If list was empty, set tail to new node
            if self.tail.is_null() {
                self.tail = node;
            }
        }
    }

    /// Removes a node from the list.
    ///
    /// # Safety
    ///
    /// Assumes the provided node pointer is valid, not null, and is part of this list.
    fn remove(&mut self, node: *mut Node<K, V>) {
        unsafe {
            // Update previous node's next pointer or head
            if !(*node).prev.is_null() {
                (*(*node).prev).next = (*node).next;
            } else {
                self.head = (*node).next;
            }

            // Update next node's previous pointer or tail
            if !(*node).next.is_null() {
                (*(*node).next).prev = (*node).prev;
            } else {
                self.tail = (*node).prev;
            }
        }
    }

    /// Removes and returns the node at the back of the list (least recently used).
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
    /// Drops the linked list and all its nodes.
    ///
    /// Properly deallocates all nodes in the list to prevent memory leaks.
    fn drop(&mut self) {
        let mut current = self.head;
        while !current.is_null() {
            unsafe {
                let next = (*current).next;
                // Convert raw pointer back to Box and drop it
                let _ = Box::from_raw(current);
                current = next;
            }
        }
    }
}

/// A Least Recently Used (LRU) cache.
///
/// Provides efficient O(1) lookups, insertions, and deletions while maintaining
/// the access order of elements. When items are accessed or inserted, they are
/// moved to the front (most recently used position). The cache does not have a
/// fixed capacity and will grow indefinitely unless items are manually removed
/// using `pop()` or `remove()`. When items are removed due to being least recently
/// used, the `pop()` method returns them.
///
/// # Examples
///
/// ```rust
/// use request_utils::lru::LRUCache;
///
/// // Create a new LRU cache
/// let mut cache = LRUCache::new();
///
/// // Insert items
/// cache.insert("key1", "value1");
/// cache.insert("key2", "value2");
///
/// // Access an item (moves it to most recently used)
/// assert_eq!(cache.get(&"key1"), Some(&"value1"));
///
/// // Remove the least recently used item
/// assert_eq!(cache.pop(), Some("value2"));
///
/// // Remove a specific item
/// assert_eq!(cache.remove(&"key1"), Some("value1"));
/// assert!(cache.is_empty());
/// ```
///
/// # Safety
///
/// This implementation uses unsafe pointer operations to manage the linked list.
/// The pointers are carefully managed to avoid dangling pointers or memory leaks.
pub struct LRUCache<K, V> {
    /// Map from keys to nodes for O(1) lookups.
    map: HashMap<K, *mut Node<K, V>>,
    /// Linked list to track access order.
    list: LinkedList<K, V>,
}

impl<K: Hash + Eq + Clone, V> LRUCache<K, V> {
    /// Creates a new empty LRU cache.
    pub fn new() -> Self {
        LRUCache {
            map: HashMap::new(),
            list: LinkedList::new(),
        }
    }

    /// Returns a reference to the value corresponding to the key if it exists.
    ///
    /// Moves the accessed key to the most recently used position.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_utils::lru::LRUCache;
    ///
    /// let mut cache = LRUCache::new();
    /// cache.insert(1, "one");
    ///
    /// assert_eq!(cache.get(&1), Some(&"one"));
    /// assert_eq!(cache.get(&2), None);
    /// ```
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(&node) = self.map.get(key) {
            // Move to front (most recently used)
            self.list.remove(node);
            self.list.push_front(node);
            unsafe {
                return Some(&(*node).value);
            }
        }
        None
    }

    /// Returns a mutable reference to the value corresponding to the key if it exists.
    ///
    /// Moves the accessed key to the most recently used position.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_utils::lru::LRUCache;
    ///
    /// let mut cache = LRUCache::new();
    /// cache.insert(1, 10);
    ///
    /// if let Some(value) = cache.get_mut(&1) {
    ///     *value += 5;
    /// }
    ///
    /// assert_eq!(cache.get(&1), Some(&15));
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if let Some(&mut node) = self.map.get_mut(key) {
            // Move to front (most recently used)
            self.list.remove(node);
            self.list.push_front(node);
            unsafe {
                return Some(&mut (*node).value);
            }
        }
        None
    }

    /// Inserts a key-value pair into the cache.
    ///
    /// If the key already exists, updates the value and moves the key to the most
    /// recently used position. Returns the previous value if the key existed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_utils::lru::LRUCache;
    ///
    /// let mut cache = LRUCache::new();
    /// assert_eq!(cache.insert(1, "one"), None);
    /// assert_eq!(cache.insert(1, "ONE"), Some("one"));
    /// assert_eq!(cache.get(&1), Some(&"ONE"));
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.map.entry(key) {
            Entry::Occupied(addr) => {
                // Key exists, update value and move to front
                self.list.remove(*addr.get());
                self.list.push_front(*addr.get());
                unsafe {
                    let old = std::mem::replace(&mut (*(*addr.get())).value, value);
                    Some(old)
                }
            }
            Entry::Vacant(addr) => {
                // Key doesn't exist, create new node
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

    /// Removes and returns the least recently used item.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_utils::lru::LRUCache;
    ///
    /// let mut cache = LRUCache::new();
    /// cache.insert(1, "one");
    /// cache.insert(2, "two");
    /// cache.get(&1); // Makes key 2 the least recently used
    ///
    /// assert_eq!(cache.pop(), Some("two"));
    /// assert_eq!(cache.pop(), Some("one"));
    /// assert_eq!(cache.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<V> {
        let old_node = self.list.pop_back();
        if !old_node.is_null() {
            unsafe {
                // Remove from map and deallocate node
                let old_key = (*old_node).key.clone();
                self.map.remove(&old_key);
                let node = Box::from_raw(old_node);
                Some(node.value)
            }
        } else {
            None
        }
    }

    /// Removes and returns the value associated with the key if it exists.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_utils::lru::LRUCache;
    ///
    /// let mut cache = LRUCache::new();
    /// cache.insert(1, "one");
    ///
    /// assert_eq!(cache.remove(&1), Some("one"));
    /// assert_eq!(cache.remove(&2), None);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(node) = self.map.remove(key) {
            self.list.remove(node);
            unsafe {
                // Deallocate node
                let node = Box::from_raw(node);
                return Some(node.value);
            }
        }
        None
    }

    /// Returns `true` if the cache contains the specified key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_utils::lru::LRUCache;
    ///
    /// let mut cache = LRUCache::new();
    /// cache.insert(1, "one");
    ///
    /// assert!(cache.contains_key(&1));
    /// assert!(!cache.contains_key(&2));
    /// ```
    pub fn contains_key(&self, k: &K) -> bool {
        self.map.contains_key(k)
    }

    /// Returns `true` if the cache contains no elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_utils::lru::LRUCache;
    ///
    /// let mut cache = LRUCache::new();
    /// assert!(cache.is_empty());
    ///
    /// cache.insert(1, "one");
    /// assert!(!cache.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns the number of elements in the cache.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_utils::lru::LRUCache;
    ///
    /// let mut cache = LRUCache::new();
    /// assert_eq!(cache.len(), 0);
    ///
    /// cache.insert(1, "one");
    /// cache.insert(2, "two");
    /// assert_eq!(cache.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns an iterator over the keys of the cache.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_utils::lru::LRUCache;
    ///
    /// let mut cache = LRUCache::new();
    /// cache.insert(1, "one");
    /// cache.insert(2, "two");
    ///
    /// let keys: Vec<_> = cache.keys().collect();
    /// assert_eq!(keys, vec![&1, &2]);
    /// ```
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.map.keys()
    }
}

impl<K: Eq + Hash + Clone, V> Default for LRUCache<K, V> {
    /// Creates a new empty LRU cache.
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<K, V> Send for LRUCache<K, V> {
    // Safety: The cache's internal pointers are not exposed outside the cache
    // and are managed in a way that ensures thread safety for the Send trait.
    // The pointers are only accessed within methods that have exclusive mutable
    // access to the cache, and all modifications to the pointers are properly
    // synchronized through Rust's ownership system.
}

#[cfg(test)]
mod ut_lru {
    include!("../tests/ut/ut_lru.rs");
}
