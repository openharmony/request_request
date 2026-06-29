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

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use request_core::config::TaskConfig;

use crate::file::PermissionToken;

/// In-process registry mapping request sequence numbers and task IDs to their
/// native task entries.
#[derive(Default)]
pub struct NativeTaskManager {
    pub(crate) inner: Mutex<NativeTaskManagerInner>,
}

#[derive(Default)]
pub(crate) struct NativeTaskManagerInner {
    pub(crate) tasks: HashMap<u64, Arc<NativeTask>>,
    pub(crate) tids: HashMap<i64, u64>,
}

/// A task held in the native registry together with its granted permissions.
pub struct NativeTask {
    /// Validated configuration for the task.
    pub config: TaskConfig,
    /// Permission tokens granted for the paths used by this task.
    pub token: Vec<PermissionToken>,
}

impl NativeTaskManager {
    /// Inserts a task indexed by its request sequence number.
    pub fn insert(&self, seq: u64, native_task: NativeTask) {
        self.inner
            .lock()
            .unwrap()
            .tasks
            .insert(seq, Arc::new(native_task));
    }

    /// Removes the task associated with the given sequence number.
    pub fn remove(&self, seq: &u64) {
        self.inner.lock().unwrap().tasks.remove(seq);
    }

    /// Binds a service-assigned task ID to the request sequence number that
    /// created it.
    pub fn bind(&self, task_id: i64, seq: u64) {
        self.inner.lock().unwrap().tids.insert(task_id, seq);
    }

    /// Removes a task and its sequence-number binding by task ID.
    pub fn remove_task(&self, task_id: &i64) {
        let mut task_map = self.inner.lock().unwrap();
        if let Some(seq) = task_map.tids.remove(task_id) {
            task_map.tasks.remove(&seq);
        }
    }

    /// Returns the task registered under the given sequence number, if any.
    pub fn get_by_seq(&self, seq: &u64) -> Option<Arc<NativeTask>> {
        self.inner.lock().unwrap().tasks.get(seq).cloned()
    }

    /// Returns the task bound to the given task ID, if any.
    pub fn get_by_id(&self, task_id: &i64) -> Option<Arc<NativeTask>> {
        let mut task_map = self.inner.lock().unwrap();
        if let Some(seq) = task_map.tids.get(task_id) {
            task_map.tasks.get(seq).cloned()
        } else {
            None
        }
    }
}
