// Copyright (C) 2023 Huawei Device Co., Ltd.
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

use crate::task::config::Version;

// Everytime a task need to be started, a permit is required.
// If a task can not get a permit, the task will be changed into `Waiting`
// State.
pub(crate) struct PermitManager {
    counts: HashMap<u64, RunningPermit>,
}

impl PermitManager {
    pub(crate) fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub(crate) fn get_permit(&mut self, uid: u64, version: Version) -> Option<RunningPermit> {
        // If the task belongs to API9, allow it start directly.
        if version == Version::API9 {
            return Some(RunningPermit::new());
        }

        if let Some(permit) = self.counts.get(&uid) {
            return permit.get();
        };
        let new_permit = RunningPermit::new();
        let result = new_permit.get();
        self.counts.insert(uid, new_permit);
        result
    }
}

pub(crate) struct RunningPermit {
    inner: Arc<Mutex<usize>>,
}

impl RunningPermit {
    const PERMIT_LIMIT: usize = 10;

    fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(0)),
        }
    }

    fn get(&self) -> Option<Self> {
        let mut count = self.inner.lock().unwrap();
        if *count == Self::PERMIT_LIMIT {
            return None;
        }
        *count += 1;
        Some(Self {
            inner: self.inner.clone(),
        })
    }
}

impl Drop for RunningPermit {
    fn drop(&mut self) {
        let mut count = self.inner.lock().unwrap();
        if *count >= 1 {
            *count -= 1;
        }
    }
}
