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

use super::*;
use crate::tests::{lock_database, test_init};
use crate::utils::get_current_timestamp;
use crate::utils::task_id_generator::TaskIdGenerator;

#[test]
fn ut_search_user() {
    test_init();
    let _lock = lock_database();
    let db = RequestDb::get_instance();
    let task_id = TaskIdGenerator::generate();
    let uid = get_current_timestamp();
    db.execute(&format!(
        "INSERT INTO request_task (task_id, uid, state, ctime, action, mode) VALUES ({}, {}, {} ,{} ,{} ,{})",
        task_id,
        uid,
        State::Removed.repr,
        get_current_timestamp(),
        Action::Upload.repr,
        Mode::BackGround.repr
    )).unwrap();

    let filter = TaskFilter {
        before: get_current_timestamp() as i64,
        after: get_current_timestamp() as i64 - 200,
        state: State::Completed.repr,
        action: Action::Any.repr,
        mode: Mode::Any.repr,
    };
    let res = db.search_task(filter, uid);
    assert_eq!(res, vec![]);

    let filter = TaskFilter {
        before: get_current_timestamp() as i64,
        after: get_current_timestamp() as i64 - 200,
        state: State::Any.repr,
        action: Action::Download.repr,
        mode: Mode::Any.repr,
    };
    let res = db.search_task(filter, uid);
    assert_eq!(res, vec![]);

    let filter = TaskFilter {
        before: get_current_timestamp() as i64,
        after: get_current_timestamp() as i64 - 200,
        state: State::Any.repr,
        action: Action::Any.repr,
        mode: Mode::FrontEnd.repr,
    };
    let res = db.search_task(filter, uid);
    assert_eq!(res, vec![]);

    let filter = TaskFilter {
        before: get_current_timestamp() as i64,
        after: get_current_timestamp() as i64 - 200,
        state: State::Removed.repr,
        action: Action::Upload.repr,
        mode: Mode::BackGround.repr,
    };
    let res = db.search_task(filter, uid);
    assert_eq!(res, vec![task_id as u32]);

    let filter = TaskFilter {
        before: get_current_timestamp() as i64,
        after: get_current_timestamp() as i64 - 200,
        state: State::Any.repr,
        action: Action::Any.repr,
        mode: Mode::Any.repr,
    };
    let res = db.search_task(filter, uid);
    assert_eq!(res, vec![task_id as u32]);

    let filter = TaskFilter {
        before: get_current_timestamp() as i64,
        after: get_current_timestamp() as i64 - 200,
        state: State::Any.repr,
        action: Action::Upload.repr,
        mode: Mode::BackGround.repr,
    };
    let res = db.search_task(filter, uid);
    assert_eq!(res, vec![task_id as u32]);
}

#[test]
fn ut_search_system() {
    test_init();
    let db = RequestDb::get_instance();
    let _lock = lock_database();
    let task_id = TaskIdGenerator::generate();
    let bundle_name = "com.ohos.app";
    db.execute(&format!(
        "INSERT INTO request_task (task_id, bundle, state, ctime, action, mode) VALUES ({}, '{}' ,{} ,{} ,{}, {})",
        task_id,
        bundle_name,
        State::Removed.repr,
        get_current_timestamp(),
        Action::Download.repr,
        Mode::BackGround.repr
    )).unwrap();

    let filter = TaskFilter {
        before: get_current_timestamp() as i64,
        after: get_current_timestamp() as i64 - 200,
        state: State::Completed.repr,
        action: Action::Any.repr,
        mode: Mode::Any.repr,
    };
    let res = db.system_search_task(filter, bundle_name.to_string());
    assert_eq!(res, vec![]);

    let filter = TaskFilter {
        before: get_current_timestamp() as i64,
        after: get_current_timestamp() as i64 - 200,
        state: State::Any.repr,
        action: Action::Any.repr,
        mode: Mode::Any.repr,
    };
    let res = db.system_search_task(filter, bundle_name.to_string());
    assert_eq!(res, vec![task_id as u32]);

    let filter = TaskFilter {
        before: get_current_timestamp() as i64,
        after: get_current_timestamp() as i64 - 200,
        state: State::Any.repr,
        action: Action::Download.repr,
        mode: Mode::BackGround.repr,
    };
    let res = db.system_search_task(filter, "*".to_string());
    assert_eq!(res, vec![task_id as u32]);
}