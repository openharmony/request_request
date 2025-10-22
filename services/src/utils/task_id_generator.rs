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

//! Provides utilities for generating unique task identifiers.

cfg_oh! {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};
    use crate::manage::database::RequestDb;
}

/// Generator for unique task identifiers.
///
/// This struct provides functionality to generate unique 32-bit identifiers
/// for tasks, with different implementations based on feature flags.
pub(crate) struct TaskIdGenerator;

impl TaskIdGenerator {
    /// Generates a unique task identifier.
    ///
    /// # Feature Dependencies
    /// This function has two different implementations:
    /// - For `oh` feature: Uses system time nanoseconds with an atomic fallback
    /// - For other cases: Uses random number generation
    ///
    /// # Examples
    /// ```rust
    /// // Generate a new unique task ID
    /// let task_id = TaskIdGenerator::generate();
    /// ```
    #[cfg(feature = "oh")]
    pub(crate) fn generate() -> u32 {
        loop {
            debug!("generate task_id");
            // Try to use system time's nanoseconds for unique ID generation
            let task_id = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(time) => time.subsec_nanos(),
                Err(e) => {
                    // Fallback to atomic counter if system time fails
                    static ID: AtomicU32 = AtomicU32::new(0);
                    error!("Generate task id from system time failed {:?}", e);
                    sys_event!(
                        ExecFault,
                        DfxCode::SA_ERROR_00,
                        &format!("Generate task id from system time failed {:?}", e)
                    );
                    // Increment and return atomic counter (relaxed ordering sufficient here)
                    ID.fetch_add(1, Ordering::Relaxed)
                }
            };
            // Ensure generated ID is unique by checking database
            if !RequestDb::get_instance().contains_task(task_id) {
                return task_id;
            }
        }
    }
    
    /// Generates a unique task identifier using random number generation.
    ///
    /// This implementation is used when the `oh` feature is not enabled.
    ///
    /// # Examples
    /// ```rust
    /// // Generate a random task ID
    /// let task_id = TaskIdGenerator::generate();
    /// ```
    #[cfg(not(feature = "oh"))]
    pub(crate) fn generate() -> u32 {
        rand::random()
    }
}
