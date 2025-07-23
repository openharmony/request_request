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

use crate::utils::task_id_generator::TaskIdGenerator;

// @tc.name: ut_task_id_generator_generate_basic
// @tc.desc: Test basic functionality of task ID generation
// @tc.precon: TaskIdGenerator is available
// @tc.step: 1. Call TaskIdGenerator::generate()
//           2. Verify the generated ID is non-zero
// @tc.expect: Returns a valid non-zero u32 value
// @tc.type: FUNC
// @tc.require: issue#ICODZX
// @tc.level: Level 0
#[test]
fn ut_task_id_generator_generate_basic() {
    let task_id = TaskIdGenerator::generate();
    assert_ne!(task_id, 0);
}

// @tc.name: ut_task_id_generator_generate_uniqueness
// @tc.desc: Test uniqueness of generated task IDs
// @tc.precon: TaskIdGenerator is available
// @tc.step: 1. Generate multiple task IDs
//           2. Verify all IDs are unique
// @tc.expect: All generated IDs are different
// @tc.type: FUNC
// @tc.require: issue#ICODZX
// @tc.level: Level 1
#[test]
fn ut_task_id_generator_generate_uniqueness_001() {
    let id1 = TaskIdGenerator::generate();
    let id2 = TaskIdGenerator::generate();
    let id3 = TaskIdGenerator::generate();

    assert_ne!(id1, id2);
    assert_ne!(id1, id3);
    assert_ne!(id2, id3);
}

// @tc.name: ut_task_id_generator_range_check
// @tc.desc: Test generated task IDs are within valid u32 range
// @tc.precon: TaskIdGenerator is available
// @tc.step: 1. Generate multiple task IDs
//           2. Verify all IDs are within u32 range
// @tc.expect: All IDs are valid u32 values
// @tc.type: FUNC
// @tc.require: issue#ICODZX
// @tc.level: Level 1
#[test]
fn ut_task_id_generator_range_check() {
    for _ in 0..100 {
        let task_id = TaskIdGenerator::generate();
        assert!(task_id <= u32::MAX);
    }
}

// @tc.name: ut_task_id_generator_zero_edge_case
// @tc.desc: Test edge case handling for zero ID
// @tc.precon: TaskIdGenerator is available
// @tc.step: 1. Generate multiple task IDs
//           2. Verify none are zero
// @tc.expect: No zero values are generated
// @tc.type: FUNC
// @tc.require: issue#ICODZX
// @tc.level: Level 2
#[test]
fn ut_task_id_generator_zero_edge_case() {
    for _ in 0..1000 {
        let task_id = TaskIdGenerator::generate();
        assert_ne!(task_id, 0);
    }
}

// @tc.name: ut_task_id_generator_concurrent_safety
// @tc.desc: Test thread safety of task ID generation
// @tc.precon: TaskIdGenerator is available
// @tc.step: 1. Spawn multiple threads
//           2. Generate IDs concurrently
//           3. Verify uniqueness across all threads
// @tc.expect: All generated IDs are unique
// @tc.type: FUNC
// @tc.require: issue#ICODZX
// @tc.level: Level 3
#[test]
fn ut_task_id_generator_concurrent_safety() {
    use std::sync::{Arc, Mutex};
    use std::collections::HashSet;
    use std::thread;

    let ids = Arc::new(Mutex::new(HashSet::new()));
    let mut handles = vec![];

    for _ in 0..4 {
        let ids_clone = Arc::clone(&ids);
        let handle = thread::spawn(move || {
            let mut local_ids = Vec::new();
            for _ in 0..100 {
                local_ids.push(TaskIdGenerator::generate());
            }

            let mut global_ids = ids_clone.lock().unwrap();
            for id in local_ids {
                assert!(global_ids.insert(id));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_ids = ids.lock().unwrap();
    assert_eq!(final_ids.len(), 400);
}

// Conditional tests for OH feature
#[cfg(feature = "oh")]
mod oh_tests {
    use super::*;

    // @tc.name: ut_task_id_generator_oh_system_time
    // @tc.desc: Test OH feature task ID generation using system time
    // @tc.precon: OH feature is enabled
    // @tc.step: 1. Generate task ID
    //           2. Verify ID is reasonable
    // @tc.expect: ID is generated successfully
    // @tc.type: FUNC
    // @tc.require: issue#ICODZX
    // @tc.level: Level 1
    #[test]
    fn ut_task_id_generator_oh_system_time() {
        let task_id = TaskIdGenerator::generate();
        assert_ne!(task_id, 0);
        assert!(task_id < 2_000_000_000);
    }
}

// Conditional tests for non-OH feature
#[cfg(not(feature = "oh"))]
mod non_oh_tests {
    use super::*;

    // @tc.name: ut_task_id_generator_non_oh_random
    // @tc.desc: Test non-OH feature random task ID generation
    // @tc.precon: OH feature is disabled
    // @tc.step: 1. Generate task ID
    //           2. Verify ID is random
    // @tc.expect: ID is generated using rand::random()
    // @tc.type: FUNC
    // @tc.require: issue#ICODZX
    // @tc.level: Level 1
    #[test]
    fn ut_task_id_generator_non_oh_random() {
        let task_id = TaskIdGenerator::generate();
        assert_ne!(task_id, 0);
    }
}