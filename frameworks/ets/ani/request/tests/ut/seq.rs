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

//! Unit tests for the sequence ID generation module.
//!
//! This module tests the thread-safe sequence ID generation functionality
//! used for creating unique task identifiers.

use request_ani::seq::TaskSeq;

// @tc.name: ut_seq_task_id_generation
// @tc.desc: Test that TaskSeq generates unique sequential IDs
// @tc.precon: NA
// @tc.step: 1. Generate two sequential task IDs
//           2. Verify that the IDs are unique
//           3. Verify that the IDs are non-zero
// @tc.expect: Generated IDs should be unique and greater than 0
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_seq_task_id_generation() {
    let task_id1 = TaskSeq::next();
    let task_id2 = TaskSeq::next();

    // Verify that IDs are unique
    assert_ne!(task_id1.0.get(), task_id2.0.get());

    // Verify that IDs are non-zero
    assert!(task_id1.0.get() > 0);
    assert!(task_id2.0.get() > 0);
}

// @tc.name: ut_seq_task_id_sequence
// @tc.desc: Test that TaskSeq generates sequential IDs
// @tc.precon: NA
// @tc.step: 1. Generate multiple sequential task IDs
//           2. Verify that each subsequent ID is greater than the previous
// @tc.expect: IDs should be generated in ascending order
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_seq_task_id_sequence() {
    let id1 = TaskSeq::next().0.get();
    let id2 = TaskSeq::next().0.get();
    let id3 = TaskSeq::next().0.get();

    // Verify sequential ordering
    assert!(id2 > id1);
    assert!(id3 > id2);
}

// @tc.name: ut_seq_task_id_thread_safety
// @tc.desc: Test that TaskSeq generates unique IDs from multiple threads
// @tc.precon: NA
// @tc.step: 1. Spawn multiple threads that generate task IDs
//           2. Collect all generated IDs
//           3. Verify that all IDs are unique
// @tc.expect: All generated IDs should be unique across threads
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_seq_task_id_thread_safety() {
    use std::collections::HashSet;
    use std::thread;

    let num_threads = 10;
    let ids_per_thread = 100;
    let mut handles = vec![];

    for _ in 0..num_threads {
        let handle = thread::spawn(move || {
            let mut ids = Vec::with_capacity(ids_per_thread);
            for _ in 0..ids_per_thread {
                ids.push(TaskSeq::next().0.get());
            }
            ids
        });
        handles.push(handle);
    }

    let mut all_ids = HashSet::new();
    for handle in handles {
        let ids = handle.join().unwrap();
        for id in ids {
            // Verify uniqueness across all threads
            assert!(
                all_ids.insert(id),
                "Duplicate ID {} found across threads",
                id
            );
        }
    }

    // Verify we collected the expected number of unique IDs
    assert_eq!(all_ids.len(), num_threads * ids_per_thread);
}

// @tc.name: ut_seq_task_id_massive_generation
// @tc.desc: Test TaskSeq with massive ID generation
// @tc.precon: NA
// @tc.step: 1. Generate 10000 sequential IDs
//           2. Verify all IDs are unique and in ascending order
// @tc.expect: All IDs should be unique and properly sequenced
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_seq_task_id_massive_generation() {
    use std::collections::HashSet;

    let count = 10000;
    let mut ids = Vec::with_capacity(count);
    let mut id_set = HashSet::with_capacity(count);

    for _ in 0..count {
        let id = TaskSeq::next().0.get();
        ids.push(id);
        id_set.insert(id);
    }

    // Verify uniqueness
    assert_eq!(ids.len(), id_set.len(), "Duplicate IDs found in massive generation");

    // Verify ascending order
    for i in 1..ids.len() {
        assert!(ids[i] > ids[i - 1], "IDs not in ascending order at index {}", i);
    }
}

// @tc.name: ut_seq_task_id_high_contention
// @tc.desc: Test TaskSeq under high thread contention
// @tc.precon: NA
// @tc.step: 1. Spawn 100 threads, each generating 1000 IDs
//           2. Collect all IDs and verify uniqueness
// @tc.expect: All 100000 IDs should be unique
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_seq_task_id_high_contention() {
    use std::collections::HashSet;
    use std::thread;
    use std::sync::mpsc;

    let num_threads = 100;
    let ids_per_thread = 1000;
    let (tx, rx) = mpsc::channel();

    for _ in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut ids = Vec::with_capacity(ids_per_thread);
            for _ in 0..ids_per_thread {
                ids.push(TaskSeq::next().0.get());
            }
            tx.send(ids).unwrap();
        });
    }

    drop(tx); // Drop original sender

    let mut all_ids = HashSet::with_capacity(num_threads * ids_per_thread);
    for ids in rx {
        for id in ids {
            assert!(
                all_ids.insert(id),
                "Duplicate ID {} found under high contention",
                id
            );
        }
    }

    assert_eq!(all_ids.len(), num_threads * ids_per_thread);
}

// @tc.name: ut_seq_task_id_gap_check
// @tc.desc: Test that TaskSeq generates consecutive IDs without gaps
// @tc.precon: NA
// @tc.step: 1. Generate a series of IDs in a single thread
//           2. Verify that IDs are consecutive (id[i] = id[0] + i)
// @tc.expect: IDs should be consecutive without gaps
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_seq_task_id_gap_check() {
    let count = 1000;
    let first_id = TaskSeq::next().0.get();
    
    for i in 1..count {
        let current_id = TaskSeq::next().0.get();
        let expected_id = first_id + i as u64;
        assert_eq!(
            current_id, expected_id,
            "ID gap detected at index {}: expected {}, got {}",
            i, expected_id, current_id
        );
    }
}

// @tc.name: ut_seq_task_id_non_zero_property
// @tc.desc: Verify TaskSeq never generates zero ID
// @tc.precon: NA
// @tc.step: 1. Generate multiple IDs
//           2. Verify all are non-zero
// @tc.expect: No ID should be zero
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_seq_task_id_non_zero_property() {
    for _ in 0..100 {
        let id = TaskSeq::next().0.get();
        assert_ne!(id, 0, "TaskSeq generated zero ID");
    }
}

// @tc.name: ut_seq_task_id_type_safety
// @tc.desc: Verify TaskSeq returns NonZeroU64
// @tc.precon: NA
// @tc.step: 1. Generate TaskSeq
//           2. Verify inner type is NonZeroU64
// @tc.expect: Inner type should be NonZeroU64
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_seq_task_id_type_safety() {
    use std::num::NonZeroU64;
    
    let task_seq = TaskSeq::next();
    let id: NonZeroU64 = task_seq.0;
    
    // Verify we can get the value
    assert!(id.get() > 0);
}
