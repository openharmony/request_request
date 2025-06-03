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

use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct TaskSeq(pub NonZeroU64);

impl TaskSeq {
    pub fn next() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        let mut last = NEXT_ID.load(Ordering::Relaxed);
        loop {
            let id = match last.checked_add(1) {
                Some(id) => id,
                None => {
                    error!("Task ID overflow, resetting to 0");
                    0
                }
            };

            match NEXT_ID.compare_exchange_weak(last, id, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => return TaskSeq(NonZeroU64::new(id).unwrap()),
                Err(id) => last = id,
            }
        }
    }
}
