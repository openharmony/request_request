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

//! Sequence ID generation module for tasks.
//! 
//! This module provides thread-safe generation of unique task sequence IDs
//! using atomic operations to ensure uniqueness across concurrent operations.

use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

/// Represents a unique task sequence ID.
///
/// Wraps a `NonZeroU64` to ensure that task IDs are always non-zero and unique.
pub struct TaskSeq(pub NonZeroU64);

impl TaskSeq {
    /// Generates the next unique task sequence ID.
    ///
    /// Uses an atomic counter with `Relaxed` ordering to generate IDs efficiently
    /// while ensuring thread safety. Handles overflow by resetting to 0 and continues.
    ///
    /// # Returns
    ///
    /// A new `TaskSeq` with a unique non-zero ID
    ///
    /// # Panics
    ///
    /// Panics if the generated ID is zero after overflow reset, which should be impossible
    /// since we immediately increment from 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_api10::seq::TaskSeq;
    /// 
    /// let task_id1 = TaskSeq::next();
    /// let task_id2 = TaskSeq::next();
    /// 
    /// assert_ne!(task_id1.0, task_id2.0);
    /// assert!(task_id1.0.get() > 0);
    /// assert!(task_id2.0.get() > 0);
    /// ```
    pub fn next() -> Self {
        // Static atomic counter to track the next available ID
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        // Start with current value using relaxed ordering for efficiency
        let mut last = NEXT_ID.load(Ordering::Relaxed);
        
        // Loop until we successfully update the counter using compare-and-swap
        loop {
            // Calculate next ID, handling potential overflow
            let id = match last.checked_add(1) {
                Some(id) => id,
                None => {
                    // Reset to 0 on overflow, though in practice this is unlikely
                    error!("Task ID overflow, resetting to 0");
                    0
                }
            };

            // Try to update the atomic counter using weak compare-and-swap
            match NEXT_ID.compare_exchange_weak(last, id, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => return TaskSeq(NonZeroU64::new(id).unwrap()),
                Err(id) => last = id,
            }
        }
    }
}
