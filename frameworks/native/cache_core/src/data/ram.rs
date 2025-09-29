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

use request_utils::task_id::TaskId;

use super::MAX_CACHE_SIZE;
use crate::manage::CacheManager;

const DEFAULT_TRUNK_CAPACITY: usize = 512;

pub struct RamCache {
    pub(super) task_id: TaskId,
    data: Vec<u8>,
    applied: u64,
    handle: &'static CacheManager,
}

impl Drop for RamCache {
    fn drop(&mut self) {
        if self.applied != 0 {
            info!("ram {} released {}", self.task_id.brief(), self.applied);
            self.handle.ram_handle.lock().unwrap().release(self.applied);
        }
    }
}

impl RamCache {
    pub(crate) fn new(task_id: TaskId, handle: &'static CacheManager, size: Option<usize>) -> Self {
        let applied = match size {
            Some(size) => {
                if CacheManager::apply_cache(&handle.ram_handle, &handle.rams, size) {
                    info!("apply ram {} for {}", size, task_id.brief());
                    size as u64
                } else {
                    error!("apply ram {} for {} failed", size, task_id.brief());
                    0
                }
            }
            None => 0,
        };

        Self {
            task_id,
            data: Vec::with_capacity(size.unwrap_or(DEFAULT_TRUNK_CAPACITY)),
            applied,
            handle,
        }
    }

    pub(crate) fn finish_write(mut self) -> Arc<RamCache> {
        let is_cache = self.check_size();
        let me = Arc::new(self);

        if is_cache {
            me.handle.update_ram_cache(me.clone());
        }
        me.handle.update_file_cache(me.task_id.clone(), me.clone());
        me
    }

    pub(crate) fn check_size(&mut self) -> bool {
        match (self.data.len() as u64).cmp(&self.applied) {
            Ordering::Equal => true,
            Ordering::Greater => {
                let diff = self.data.len() - self.applied as usize;
                if self.data.len() > MAX_CACHE_SIZE as usize
                    || !CacheManager::apply_cache(&self.handle.ram_handle, &self.handle.rams, diff)
                {
                    info!(
                        "apply extra ram {} cache for {} failed",
                        diff,
                        self.task_id.brief()
                    );
                    self.handle.ram_handle.lock().unwrap().release(self.applied);
                    self.applied = 0;
                    false
                } else {
                    info!(
                        "apply extra ram {} cache for {} success",
                        diff,
                        self.task_id.brief()
                    );
                    self.applied = self.data.len() as u64;
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

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn cursor(&self) -> Cursor<&[u8]> {
        Cursor::new(&self.data)
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
    pub(crate) fn update_ram_cache(&'static self, cache: Arc<RamCache>) {
        let task_id = cache.task_id().clone();

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
    }
}

#[cfg(test)]
mod ut_ram {
    include!("../../tests/ut/data/ut_ram.rs");
}
