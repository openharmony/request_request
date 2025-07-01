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

//! Active Counter

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct ActiveCounter {
    count: Arc<AtomicU32>,
}

impl ActiveCounter {
    pub(crate) fn new() -> Self {
        Self {
            count: Arc::new(AtomicU32::new(0)),
        }
    }

    pub(crate) fn increment(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn decrement(&self) {
        self.count.fetch_sub(1, Ordering::Relaxed);
    }

    pub(crate) fn is_active(&self) -> bool {
        let count = self.count.load(Ordering::Relaxed);
        info!("active count: {}", count);
        count > 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ut_active_counter() {
        let counter = ActiveCounter::new();
        assert!(!counter.is_active());
        counter.increment();
        assert!(counter.is_active());
        counter.decrement();
        assert!(!counter.is_active());
    }
}
