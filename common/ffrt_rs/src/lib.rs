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

#![allow(missing_docs)]
mod wrapper;

// Import necessary items from the wrapper module
use wrapper::{ClosureWrapper, FfrtSleep, FfrtSpawn};

/// Spawns a task using the FastFlow Runtime.
/// 
/// Submits a closure to be executed by the FastFlow Runtime thread pool.
/// 
/// # Arguments
/// 
/// * `f` - The closure to execute
/// 
/// # Examples
/// 
/// ```
/// use ffrt_rs::ffrt_spawn;
/// 
/// // Spawn a task that prints a message
/// ffrt_spawn(|| {
///     println!("Task executed by FFRT");
/// });
/// ```
/// 
/// # Safety
/// 
/// This function is safe, but relies on the underlying C++ implementation to correctly handle
/// the provided closure and properly manage thread resources.
pub fn ffrt_spawn<F>(f: F)
where
    F: FnOnce() + 'static,
{
    FfrtSpawn(ClosureWrapper::new(f));
}

/// Suspends the current thread execution for the specified duration using FFRT.
/// 
/// # Arguments
/// 
/// * `ms` - The number of milliseconds to sleep
/// 
/// # Examples
/// 
/// ```
/// use ffrt_rs::ffrt_sleep;
/// 
/// // Sleep for 100 milliseconds
/// ffrt_sleep(100);
/// ```
pub fn ffrt_sleep(ms: u64) {
    FfrtSleep(ms);
}

#[cfg(test)]
mod ut_lib {
    // Include unit tests from the tests directory
    include!("../tests/ut/ut_lib.rs");
}
