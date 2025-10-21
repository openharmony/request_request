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

//! Module for tracking and reporting task completion statistics.
//! 
//! This module provides utilities to count completed and failed tasks, and
//! report these statistics when tasks are unloaded. It uses a thread-safe
//! singleton pattern to maintain the count state across the application.

use std::sync::{Arc, Mutex, Once};

/// Internal struct for tracking request task statistics.
///
/// This struct maintains counts of completed and failed tasks, along with a
/// state flag indicating whether any tasks have been recorded since the last
/// report.
struct RequestTaskCount {
    /// Number of successfully completed tasks.
    completed_task_count: i32,
    /// Number of failed tasks.
    failed_task_count: i32,
    /// Flag indicating whether task counts have been updated since last report.
    load_state: bool,
}

impl RequestTaskCount {
    /// Gets a thread-safe singleton instance of `RequestTaskCount`.
    ///
    /// This method implements the singleton pattern with thread safety,
    /// ensuring that only one instance of `RequestTaskCount` exists
    /// throughout the application lifecycle.
    ///
    /// # Safety
    ///
    /// This method uses `unsafe` code to access and initialize the static
    /// instance. The `Once` mechanism guarantees that initialization happens
    /// exactly once, and subsequent accesses are safe.
    ///
    /// # Returns
    ///
    /// Returns a reference-counted pointer to a mutex-wrapped `RequestTaskCount`
    /// instance.
    fn get_instance() -> Arc<Mutex<RequestTaskCount>> {
        // Static storage for the singleton instance
        static mut TASK_COUNT: Option<Arc<Mutex<RequestTaskCount>>> = None;
        // Ensures the initialization happens exactly once
        static ONCE: Once = Once::new();
        
        ONCE.call_once(|| {
            // Initialize the singleton instance with default values
            unsafe {
                TASK_COUNT = Some(Arc::new(Mutex::new(RequestTaskCount {
                    completed_task_count: 0,
                    failed_task_count: 0,
                    load_state: false,
                })))
            };
        });

        // Return a clone of the Arc to increment the reference count
        unsafe { TASK_COUNT.as_ref().unwrap().clone() }
    }
}

/// Increments the count of successfully completed tasks.
///
/// This function safely increments the completed task counter and sets the
/// load state flag to indicate that statistics have been updated.
///
/// # Examples
///
/// ```rust
/// // Call when a task successfully completes
/// task_complete_add();
/// ```
pub(crate) fn task_complete_add() {
    let instance = RequestTaskCount::get_instance();
    // Lock the mutex to ensure thread safety while updating the count
    let mut task_count = instance.lock().unwrap();
    task_count.completed_task_count += 1;
    // Mark that we have new data to report
    task_count.load_state = true;
}

/// Increments the count of failed tasks.
///
/// This function safely increments the failed task counter and sets the
/// load state flag to indicate that statistics have been updated.
///
/// # Examples
///
/// ```rust
/// // Call when a task fails to complete
/// task_fail_add();
/// ```
pub(crate) fn task_fail_add() {
    let instance = RequestTaskCount::get_instance();
    // Lock the mutex to ensure thread safety while updating the count
    let mut task_count = instance.lock().unwrap();
    task_count.failed_task_count += 1;
    // Mark that we have new data to report
    task_count.load_state = true;
}

/// Reports task statistics and resets counters.
///
/// This function checks if there are new statistics to report (using the
/// load_state flag). If so, it logs the completed and failed task counts
/// using the sys_event macro, then resets all counters and the state flag.
///
/// # Notes
///
/// Statistics are only reported if the load_state flag is true, indicating
/// that task counts have been updated since the last report.
///
/// # Examples
///
/// ```rust
/// // Call when unloading tasks to report collected statistics
/// task_unload();
/// ```
pub(crate) fn task_unload() {
    let instance = RequestTaskCount::get_instance();
    // Lock the mutex to ensure thread safety while reading and resetting
    let mut task_count = instance.lock().unwrap();
    
    // Only report and reset if we have new data
    if task_count.load_state {
        // Capture current counts for reporting
        let completed = task_count.completed_task_count;
        let failed = task_count.failed_task_count;
        
        // Report statistics via system event
        sys_event!(
            ExecError,
            DfxCode::TASK_STATISTICS,
            &format!("Task Completed {}, failed {}", completed, failed)
        );
        
        // Reset counters and state flag
        task_count.completed_task_count = 0;
        task_count.failed_task_count = 0;
        task_count.load_state = false;
    }
}
