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

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub struct QueueMap<K, V> {
    map: HashMap<K, V>,
    v: VecDeque<K>,
    removed: HashSet<K>,
}

impl<K: Eq + Hash + Clone, V> QueueMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            v: VecDeque::new(),
            removed: HashSet::new(),
        }
    }

    pub fn pop(&mut self) -> Option<V> {
        while let Some(n) = self.v.pop_front() {
            if !self.removed.remove(&n) {
                let ret = self.map.remove(&n);
                self.removed.insert(n);
                return ret;
            }
        }
        None
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.removed.remove(&k);
        self.v.push_back(k.clone());
        self.map.insert(k, v)
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.map.get(k)
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.map.contains_key(k)
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        if let Some(t) = self.map.remove(k) {
            self.removed.insert(k.clone());
            Some(t)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.map.get_mut(k)
    }
}

#[cfg(test)]
mod test {
    use super::QueueMap;

    #[test]
    fn ut_queue_map_fifo() {
        let mut map = QueueMap::new();
        map.insert(1, 1);
        map.insert(2, 2);
        assert!(map.contains_key(&1));
        assert_eq!(map.pop().unwrap(), 1);
    }

    #[test]
    fn ut_queue_map_remove() {
        let mut map = QueueMap::new();
        map.insert(1, 1);
        map.insert(2, 2);
        map.insert(3, 3);
        map.insert(4, 4);
        map.remove(&1);
        map.remove(&2);
        map.remove(&3);
        map.insert(3, 3);
        assert_eq!(map.pop().unwrap(), 3);
        assert_eq!(map.pop().unwrap(), 4);
        map.insert(1, 1);
        map.insert(2, 2);
        assert_eq!(map.pop().unwrap(), 1);
        assert_eq!(map.pop().unwrap(), 2);
        assert!(map.pop().is_none());
    }

    #[test]
    fn ut_queue_map_same_key() {
        let mut map = QueueMap::new();
        map.insert(1, 1);
        map.insert(1, 2);
        assert_eq!(map.pop().unwrap(), 2);
        assert!(map.pop().is_none());
    }

    #[test]
    fn ut_queue_map_insert_get() {
        let mut map = QueueMap::new();
        map.insert(1, 1);
        assert_eq!(1, *map.get(&1).unwrap());
    }
}
