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

pub struct QueueMap<N, T> {
    map: HashMap<N, T>,
    v: VecDeque<N>,
    removed: HashSet<N>,
}

impl<N: Eq + Hash + Clone, T> QueueMap<N, T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            v: VecDeque::new(),
            removed: HashSet::new(),
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        while let Some(n) = self.v.pop_front() {
            if !self.removed.remove(&n) {
                let ret = self.map.remove(&n);
                self.removed.insert(n);
                return ret;
            }
        }
        None
    }

    pub fn push_back(&mut self, n: N, t: T) {
        self.removed.remove(&n);
        self.v.push_back(n.clone());
        self.map.insert(n, t);
    }

    pub fn contains_key(&self, n: &N) -> bool {
        self.map.contains_key(n)
    }

    pub fn remove(&mut self, n: &N) -> Option<T> {
        if let Some(t) = self.map.remove(n) {
            self.removed.insert(n.clone());
            Some(t)
        } else {
            None
        }
    }

    pub fn get(&self, n: &N) -> Option<&T> {
        self.map.get(n)
    }
}

#[cfg(test)]
mod test {
    use super::QueueMap;

    #[test]
    fn ut_queue_map_fifo() {
        let mut map = QueueMap::new();
        map.push_back(1, 1);
        map.push_back(2, 2);
        assert!(map.contains_key(&1));
        assert_eq!(map.pop_front().unwrap(), 1);
    }

    #[test]
    fn ut_queue_map_remove() {
        let mut map = QueueMap::new();
        map.push_back(1, 1);
        map.push_back(2, 2);
        map.push_back(3, 3);
        map.push_back(4, 4);
        map.remove(&1);
        map.remove(&2);
        map.remove(&3);
        map.push_back(3, 3);
        assert_eq!(map.pop_front().unwrap(), 3);
        assert_eq!(map.pop_front().unwrap(), 4);
        map.push_back(1, 1);
        map.push_back(2, 2);
        assert_eq!(map.pop_front().unwrap(), 1);
        assert_eq!(map.pop_front().unwrap(), 2);
        assert!(map.pop_front().is_none());
    }

    #[test]
    fn ut_queue_map_same_key() {
        let mut map = QueueMap::new();
        map.push_back(1, 1);
        map.push_back(1, 2);
        assert_eq!(map.pop_front().unwrap(), 2);
        assert!(map.pop_front().is_none());
    }
}
