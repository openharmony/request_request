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

use super::RequestDb;
use crate::config::{Action, Mode};
use crate::task::info::State;
use crate::tests::{lock_database, test_init};
use crate::utils::get_current_timestamp;
use crate::utils::task_id_generator::TaskIdGenerator;

#[test]
fn ut_database_base() {
    test_init();
    let _lock = lock_database();

    let task_id = TaskIdGenerator::generate();
    let db = RequestDb::get_instance();
    db.execute(&format!(
        "INSERT INTO request_task (task_id, bundle) VALUES ({}, 'example_bundle')",
        task_id
    ))
    .unwrap();

    let tasks =
        db.query_integer("SELECT task_id FROM request_task WHERE bundle = 'example_bundle'");
    assert!(tasks.contains(&task_id));
}

#[test]
fn ut_database_contains_task() {
    test_init();
    let _lock = lock_database();
    let task_id = TaskIdGenerator::generate();
    let db = RequestDb::get_instance();
    db.execute(&format!(
        "INSERT INTO request_task (task_id, bundle) VALUES ({}, 'example_bundle')",
        task_id
    ))
    .unwrap();

    assert!(db.contains_task(task_id));
}

#[test]
fn ut_database_query_task_token_id() {
    test_init();
    let _lock = lock_database();

    let task_id = TaskIdGenerator::generate();
    let token_id = 123456789;
    let db = RequestDb::get_instance();
    db.execute(&format!(
        "INSERT INTO request_task (task_id, token_id) VALUES ({}, {})",
        task_id, token_id
    ))
    .unwrap();

    assert_eq!(db.query_task_token_id(task_id).unwrap(), token_id);
}

#[test]
fn ut_database_app_task_qos_info() {
    test_init();
    let _lock = lock_database();
    let task_id = TaskIdGenerator::generate();
    let db = RequestDb::get_instance();
    let priority = get_current_timestamp() as u32;
    db.execute(&format!(
        "INSERT INTO request_task (task_id, action, mode, state, priority) VALUES ({}, {}, {}, {}, {})",
        task_id,
        Action::Download.repr,
        Mode::FrontEnd.repr,
        State::Completed.repr,
        priority,
    ))
    .unwrap();

    let info = db.get_task_qos_info(task_id).unwrap();
    assert_eq!(info.task_id, task_id);
    assert_eq!(info.action, Action::Download.repr);
    assert_eq!(info.mode, Mode::FrontEnd.repr);
    assert_eq!(info.state, State::Completed.repr);
    assert_eq!(info.priority, priority);
}