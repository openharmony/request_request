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

use crate::file::PermissionToken;
use request_core::config::TaskConfig;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;

#[derive(Default)]
pub struct NativeTaskManager {
    pub(crate) inner: Mutex<NativeTaskManagerInner>,
}

#[derive(Default)]
pub(crate) struct NativeTaskManagerInner {
    pub(crate) tasks: HashMap<u64, Arc<NativeTask>>,
    pub(crate) tids: HashMap<i64, u64>
}

pub struct NativeTask {
    pub config: TaskConfig,
    pub token: Vec<PermissionToken>,
}

impl NativeTaskManager {
    pub fn insert(&self, seq: u64, native_task: NativeTask) {
        self.inner.lock().unwrap().tasks.insert(seq, Arc::new(native_task));
    }

    pub fn remove(&self, seq: &u64) {
        self.inner.lock().unwrap().tasks.remove(seq);
    }

    pub fn bind(&self, task_id: i64, seq: u64) {
        self.inner.lock().unwrap().tids.insert(task_id, seq);
    }

    pub fn remove_task(&self, task_id: &i64) {
        let mut task_map = self.inner.lock().unwrap();
        if let Some(seq) = task_map.tids.remove(task_id) {
            task_map.tasks.remove(&seq);
        }
    }

    pub fn get_by_seq(&self, seq: &u64) -> Option<Arc<NativeTask>> {
        self.inner.lock().unwrap().tasks.get(seq).cloned()
    }

    pub fn get_by_id(&self, task_id: &i64) -> Option<Arc<NativeTask>> {
        let mut task_map = self.inner.lock().unwrap();
        if let Some(seq) = task_map.tids.get(task_id) {
            task_map.tasks.get(seq).cloned()
        } else {
            None
        }
    }
}
