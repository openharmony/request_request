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

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time;

use super::*;

// @tc.name: ut_ffrt_spawn
// @tc.desc: Test ffrt_spawn function with atomic counter
// @tc.precon: NA
// @tc.step: 1. Create atomic counter
//           2. Spawn task to increment counter
//           3. Wait for task completion
//           4. Verify counter value
// @tc.expect: Counter should be incremented to 1
// @tc.type: FUNC
// @tc.require: issues#ICN31I
#[test]
fn ut_ffrt_spawn() {
    let flag = Arc::new(AtomicUsize::new(0));
    let flag_clone = flag.clone();
    ffrt_spawn(move || {
        flag_clone.fetch_add(1, Ordering::SeqCst);
    });
    std::thread::sleep(std::time::Duration::from_millis(100));
    assert_eq!(flag.load(Ordering::SeqCst), 1);
}

// @tc.name: ut_ffrt_sleep
// @tc.desc: Test ffrt_sleep function with delayed counter increment
// @tc.precon: NA
// @tc.step: 1. Create atomic counter
//           2. Spawn task with sleep and counter increment
//           3. Wait for sleep duration
//           4. Verify counter value
// @tc.expect: Counter should be incremented to 1 after sleep
// @tc.type: FUNC
// @tc.require: issues#ICN31I
#[test]
fn ut_ffrt_sleep() {
    let flag = Arc::new(AtomicUsize::new(0));
    let flag_clone = flag.clone();
    ffrt_spawn(move || {
        ffrt_sleep(1000);
        flag_clone.fetch_add(1, Ordering::SeqCst);
    });
    for _ in 0..9 {
        std::thread::sleep(time::Duration::from_millis(100));
    }
    std::thread::sleep(time::Duration::from_millis(500));
    assert_eq!(flag.load(Ordering::SeqCst), 1);
}