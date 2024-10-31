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

use super::CacheManager;
use crate::agent::CustomCallback;
pub(crate) struct Fetcher {
    task_id: u64,
}

impl Fetcher {
    pub(crate) fn new(task_id: u64) -> Self {
        Self { task_id }
    }

    pub(crate) fn fetch_with_callback(
        &self,
        mut callback: Box<dyn CustomCallback>,
    ) -> Result<(), Box<dyn CustomCallback>> {
        if let Some(cache) = CacheManager::get_instance().get_cache(self.task_id) {
            #[cfg(feature = "ohos")]
            ffrt_rs::ffrt_spawn(move || {
                callback.on_success(cache);
            });
            #[cfg(not(feature = "ohos"))]
            std::thread::spawn(move || {
                callback.on_success(cache);
            });
            Ok(())
        } else {
            Err(callback)
        }
    }
}
