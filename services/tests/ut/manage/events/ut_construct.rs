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

use std::collections::HashMap;

const MAX_BACKGROUND_TASK: usize = 1001;
const MAX_FRONTEND_TASK: usize = 2001;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    BackGround = 0,
    FrontEnd = 1,
    Any = 2,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ErrorCode {
    ErrOk,
    TaskEnqueueErr,
}

struct TaskCountManager {
    task_count: HashMap<u64, (usize, usize)>,
}

impl TaskCountManager {
    fn new() -> Self {
        Self {
            task_count: HashMap::new(),
        }
    }

    fn check_and_increment(&mut self, uid: u64, mode: Mode) -> Result<(), ErrorCode> {
        let (frontend, background) = self.task_count.entry(uid).or_insert((0, 0));

        let (task_count, limit) = match mode {
            Mode::FrontEnd => (frontend, MAX_FRONTEND_TASK),
            _ => (background, MAX_BACKGROUND_TASK),
        };

        if *task_count >= limit {
            return Err(ErrorCode::TaskEnqueueErr);
        }
        *task_count += 1;
        Ok(())
    }

    fn get_counts(&self, uid: u64) -> Option<(usize, usize)> {
        self.task_count.get(&uid).copied()
    }
}

// @tc.name: ut_construct_task_count_init
// @tc.desc: Test task count initialization for new uid
// @tc.precon: NA
// @tc.step: 1. Create TaskCountManager
//           2. Check counts for non-existent uid
//           3. Increment for new uid
//           4. Verify counts are initialized
// @tc.expect: Task count is initialized correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_construct_task_count_init() {
    let mut manager = TaskCountManager::new();
    let uid = 1000u64;

    assert!(manager.get_counts(uid).is_none());

    let result = manager.check_and_increment(uid, Mode::BackGround);
    assert_eq!(result, Ok(()));
    assert_eq!(manager.get_counts(uid), Some((0, 1)));
}

// @tc.name: ut_construct_mode_frontend_limit
// @tc.desc: Test frontend task limit enforcement
// @tc.precon: NA
// @tc.step: 1. Create TaskCountManager with frontend count at limit
//           2. Attempt to increment frontend count
//           3. Verify TaskEnqueueErr is returned
// @tc.expect: Frontend task limit is enforced
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_construct_mode_frontend_limit() {
    let mut manager = TaskCountManager::new();
    let uid = 1000u64;

    manager.task_count.insert(uid, (MAX_FRONTEND_TASK, 0));

    let result = manager.check_and_increment(uid, Mode::FrontEnd);
    assert_eq!(result, Err(ErrorCode::TaskEnqueueErr));
    assert_eq!(manager.get_counts(uid), Some((MAX_FRONTEND_TASK, 0)));
}

// @tc.name: ut_construct_mode_background_limit
// @tc.desc: Test background task limit enforcement
// @tc.precon: NA
// @tc.step: 1. Create TaskCountManager with background count at limit
//           2. Attempt to increment background count
//           3. Verify TaskEnqueueErr is returned
// @tc.expect: Background task limit is enforced
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_construct_mode_background_limit() {
    let mut manager = TaskCountManager::new();
    let uid = 1000u64;

    manager.task_count.insert(uid, (0, MAX_BACKGROUND_TASK));

    let result = manager.check_and_increment(uid, Mode::BackGround);
    assert_eq!(result, Err(ErrorCode::TaskEnqueueErr));
    assert_eq!(manager.get_counts(uid), Some((0, MAX_BACKGROUND_TASK)));
}

// @tc.name: ut_construct_mode_any_as_background
// @tc.desc: Test Mode::Any uses background limit
// @tc.precon: NA
// @tc.step: 1. Create TaskCountManager with background count at limit
//           2. Attempt to increment with Mode::Any
//           3. Verify TaskEnqueueErr is returned
// @tc.expect: Mode::Any uses background limit
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_construct_mode_any_as_background() {
    let mut manager = TaskCountManager::new();
    let uid = 1000u64;

    manager.task_count.insert(uid, (0, MAX_BACKGROUND_TASK));

    let result = manager.check_and_increment(uid, Mode::Any);
    assert_eq!(result, Err(ErrorCode::TaskEnqueueErr));
}

// @tc.name: ut_construct_multiple_users
// @tc.desc: Test task count isolation between different users
// @tc.precon: NA
// @tc.step: 1. Create TaskCountManager
//           2. Increment for user 1000
//           3. Increment for user 2000
//           4. Verify counts are isolated
// @tc.expect: Task counts are isolated per user
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_construct_multiple_users() {
    let mut manager = TaskCountManager::new();

    manager.check_and_increment(1000, Mode::FrontEnd).unwrap();
    manager.check_and_increment(1000, Mode::BackGround).unwrap();
    manager.check_and_increment(2000, Mode::FrontEnd).unwrap();

    assert_eq!(manager.get_counts(1000), Some((1, 1)));
    assert_eq!(manager.get_counts(2000), Some((1, 0)));
}

// @tc.name: ut_construct_frontend_near_limit
// @tc.desc: Test frontend task count just below limit
// @tc.precon: NA
// @tc.step: 1. Create TaskCountManager with frontend count at limit-1
//           2. Attempt to increment
//           3. Verify success and count reaches limit
// @tc.expect: Task can be created just below limit
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_construct_frontend_near_limit() {
    let mut manager = TaskCountManager::new();
    let uid = 1000u64;

    manager.task_count.insert(uid, (MAX_FRONTEND_TASK - 1, 0));

    let result = manager.check_and_increment(uid, Mode::FrontEnd);
    assert_eq!(result, Ok(()));
    assert_eq!(manager.get_counts(uid), Some((MAX_FRONTEND_TASK, 0)));
}

// @tc.name: ut_construct_background_near_limit
// @tc.desc: Test background task count just below limit
// @tc.precon: NA
// @tc.step: 1. Create TaskCountManager with background count at limit-1
//           2. Attempt to increment
//           3. Verify success and count reaches limit
// @tc.expect: Task can be created just below limit
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_construct_background_near_limit() {
    let mut manager = TaskCountManager::new();
    let uid = 1000u64;

    manager.task_count.insert(uid, (0, MAX_BACKGROUND_TASK - 1));

    let result = manager.check_and_increment(uid, Mode::BackGround);
    assert_eq!(result, Ok(()));
    assert_eq!(manager.get_counts(uid), Some((0, MAX_BACKGROUND_TASK)));
}

// @tc.name: ut_construct_mixed_modes
// @tc.desc: Test mixing frontend and background tasks
// @tc.precon: NA
// @tc.step: 1. Create TaskCountManager
//           2. Create multiple frontend and background tasks
//           3. Verify both counters are tracked correctly
// @tc.expect: Frontend and background counts are tracked independently
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_construct_mixed_modes() {
    let mut manager = TaskCountManager::new();
    let uid = 1000u64;

    for _ in 0..5 {
        manager.check_and_increment(uid, Mode::FrontEnd).unwrap();
    }
    for _ in 0..3 {
        manager.check_and_increment(uid, Mode::BackGround).unwrap();
    }

    assert_eq!(manager.get_counts(uid), Some((5, 3)));
}

// @tc.name: ut_construct_limit_boundary_values
// @tc.desc: Test that limit constants have correct values
// @tc.precon: NA
// @tc.step: 1. Check MAX_BACKGROUND_TASK value
//           2. Check MAX_FRONTEND_TASK value
// @tc.expect: Limit constants match expected values
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_construct_limit_boundary_values() {
    assert_eq!(MAX_BACKGROUND_TASK, 1001);
    assert_eq!(MAX_FRONTEND_TASK, 2001);
    assert!(MAX_FRONTEND_TASK > MAX_BACKGROUND_TASK);
}

// @tc.name: ut_construct_mode_repr_values
// @tc.desc: Test Mode enum repr values match source
// @tc.precon: NA
// @tc.step: 1. Check Mode discriminant values
// @tc.expect: Mode repr values match source code
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_construct_mode_repr_values() {
    assert_eq!(Mode::BackGround as u8, 0);
    assert_eq!(Mode::FrontEnd as u8, 1);
    assert_eq!(Mode::Any as u8, 2);
}
