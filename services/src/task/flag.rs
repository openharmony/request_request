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

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// This flag is used to record the number of current rdb database operations.
pub(crate) struct RdbRecording {
    inner: Arc<AtomicUsize>,
}

impl RdbRecording {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub(crate) fn is_recording(&self) -> bool {
        self.count() != 0
    }

    pub(crate) fn count(&self) -> usize {
        self.inner.load(Ordering::SeqCst)
    }
}

impl Clone for RdbRecording {
    fn clone(&self) -> Self {
        self.inner.fetch_add(1, Ordering::SeqCst);
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Drop for RdbRecording {
    fn drop(&mut self) {
        self.inner.fetch_sub(1, Ordering::SeqCst);
    }
}
