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

use super::{App, Task};
use crate::manage::database::RequestDb;
use crate::task::config::Mode;
use crate::tests::{lock_database, test_init};
use crate::utils::get_current_timestamp;
use crate::utils::task_id_generator::TaskIdGenerator;
impl Task {
    fn new(task_id: u32, mode: Mode, priority: u32) -> Self {
        Self {
            uid: 0,
            action: crate::task::config::Action::Any,
            task_id,
            mode,
            priority,
        }
    }
}

// @tc.name: ut_app_insert
// @tc.desc: Test inserting tasks into App with priority ordering
// @tc.precon: NA
// @tc.step: 1. Create new App instance
//           2. Insert tasks with different priorities and modes
//           3. Verify task order after each insertion
// @tc.expect: Tasks are inserted and ordered correctly by priority and mode
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_app_insert() {
    let mut app = App::new(1);
    assert!(app.tasks.is_empty());
    assert_eq!(app.uid, 1);

    app.insert(Task::new(1, Mode::FrontEnd, 0));
    assert_eq!(app.tasks[0].task_id, 1);
    assert_eq!(app.tasks[0].mode, Mode::FrontEnd);
    assert_eq!(app.tasks[0].priority, 0);

    app.insert(Task::new(2, Mode::FrontEnd, 100));
    assert_eq!(app.tasks[0].task_id, 1);
    assert_eq!(app.tasks[1].task_id, 2);

    app.insert(Task::new(3, Mode::FrontEnd, 50));
    assert_eq!(app.tasks[0].task_id, 1);
    assert_eq!(app.tasks[1].task_id, 3);
    assert_eq!(app.tasks[2].task_id, 2);

    app.insert(Task::new(4, Mode::BackGround, 0));
    assert_eq!(app.tasks[0].task_id, 1);
    assert_eq!(app.tasks[1].task_id, 3);
    assert_eq!(app.tasks[2].task_id, 2);
    assert_eq!(app.tasks[3].task_id, 4);

    app.insert(Task::new(5, Mode::BackGround, 100));
    assert_eq!(app.tasks[0].task_id, 1);
    assert_eq!(app.tasks[1].task_id, 3);
    assert_eq!(app.tasks[2].task_id, 2);
    assert_eq!(app.tasks[3].task_id, 4);
    assert_eq!(app.tasks[4].task_id, 5);

    app.insert(Task::new(6, Mode::BackGround, 50));
    assert_eq!(app.tasks[0].task_id, 1);
    assert_eq!(app.tasks[1].task_id, 3);
    assert_eq!(app.tasks[2].task_id, 2);
    assert_eq!(app.tasks[3].task_id, 4);
    assert_eq!(app.tasks[4].task_id, 6);
    assert_eq!(app.tasks[5].task_id, 5);
}

// @tc.name: ut_app_remove
// @tc.desc: Test removing tasks from App
// @tc.precon: NA
// @tc.step: 1. Create App instance with multiple tasks
//           2. Remove specific tasks by ID
//           3. Verify remaining tasks' order
// @tc.expect: Specified tasks are removed and remaining tasks maintain correct order
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_app_remove() {
    let mut app = App::new(1);
    for i in 0..5 {
        app.insert(Task::new(i, Mode::FrontEnd, i));
    }
    assert_eq!(app.tasks[0].task_id, 0);
    assert_eq!(app.tasks[1].task_id, 1);
    assert_eq!(app.tasks[2].task_id, 2);
    assert_eq!(app.tasks[3].task_id, 3);
    assert_eq!(app.tasks[4].task_id, 4);

    app.remove(3);
    assert_eq!(app.tasks[0].task_id, 0);
    assert_eq!(app.tasks[1].task_id, 1);
    assert_eq!(app.tasks[2].task_id, 2);
    assert_eq!(app.tasks[3].task_id, 4);

    app.remove(1);
    assert_eq!(app.tasks[0].task_id, 0);
    assert_eq!(app.tasks[1].task_id, 2);
    assert_eq!(app.tasks[2].task_id, 4);

    app.remove(4);
    assert_eq!(app.tasks[0].task_id, 0);
    assert_eq!(app.tasks[1].task_id, 2);

    app.remove(0);
    assert_eq!(app.tasks[0].task_id, 2);
}

// @tc.name: ut_task_partial_ord
// @tc.desc: Test task ordering based on priority and mode
// @tc.precon: NA
// @tc.step: 1. Create tasks with different modes and priorities
//           2. Compare task order using partial ordering
// @tc.expect: Tasks are ordered correctly according to priority and mode rules
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_partial_ord() {
    let task1 = Task::new(1, Mode::FrontEnd, 0);
    let task2 = Task::new(2, Mode::FrontEnd, 1);
    let task3 = Task::new(3, Mode::BackGround, 0);
    let task4 = Task::new(4, Mode::BackGround, 1);
    assert!(task1 < task2);
    assert!(task1 < task3);
    assert!(task1 < task4);
    assert!(task2 < task3);
    assert!(task2 < task4);
    assert!(task3 < task4);
}

// @tc.name: ut_database_app_info
// @tc.desc: Test retrieving app information from database
// @tc.precon: NA
// @tc.step: 1. Initialize test database
//           2. Insert test tasks with different UIDs
//           3. Query app information
//           4. Verify correct UIDs are returned
// @tc.expect: Database returns correct app information for inserted tasks
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_database_app_info() {
    test_init();
    let db = RequestDb::get_instance();
    let _lock = lock_database();
    let uid = get_current_timestamp();

    for i in 0..10 {
        db.execute(&format!(
            "INSERT INTO request_task (task_id, uid, bundle) VALUES ({}, {}, '{}')",
            TaskIdGenerator::generate(),
            uid + i / 5,
            "test_bundle",
        ))
        .unwrap();
    }
    let v = db.get_app_infos();
    assert_eq!(v.iter().filter(|a| **a == uid).count(), 1);
    assert_eq!(v.iter().filter(|a| **a == uid + 1).count(), 1);
}