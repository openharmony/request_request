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

use std::io::Write;
use std::sync::Arc;

use request_utils::task_id::TaskId;

use crate::data::RamCache;
use crate::manage::CacheManager;

// pub(crate) struct Updater {
//     pub(crate) remove_flag: bool
//     pub(crate) seq: usize,
//     pub(crate) handle: TaskHandle,
// }

pub struct Updater {
    task_id: TaskId,
    cache: Option<RamCache>,
    cache_manager: &'static CacheManager,
}

impl Updater {
    pub fn new(task_id: TaskId, cache_manager: &'static CacheManager) -> Self {
        Self {
            task_id,
            cache: None,
            cache_manager,
        }
    }

    pub fn cache_finish(&mut self) -> Arc<RamCache> {
        match self.cache.take() {
            Some(cache) => cache.finish_write(),
            None => Arc::new(RamCache::new(
                self.task_id.clone(),
                self.cache_manager,
                Some(0),
            )),
        }
    }

    pub fn cache_receive<F>(&mut self, data: &[u8], content_length: F)
    where
        F: FnOnce() -> Option<usize>,
    {
        if self.cache.is_none() {
            let content_length = content_length();
            let apply_cache =
                RamCache::new(self.task_id.clone(), self.cache_manager, content_length);
            self.cache = Some(apply_cache)
        }
        self.cache.as_mut().unwrap().write_all(data).unwrap();
    }

    pub fn reset_cache(&mut self) {
        let size = self.cache.as_ref().map(|a| a.size()).unwrap_or(0);
        if size != 0 {
            info!("reset {} cache size {}", self.task_id.brief(), size);
            self.cache.take();
        }
    }
}
