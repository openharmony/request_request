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

use rusqlite::Connection;

const CREATE: &'static str = "CREATE TABLE IF NOT EXISTS request_task (task_id INTEGER PRIMARY KEY, uid INTEGER, token_id INTEGER, action INTEGER, mode INTEGER, cover INTEGER, network INTEGER, metered INTEGER, roaming INTEGER, ctime INTEGER, mtime INTEGER, reason INTEGER, gauge INTEGER, retry INTEGER, redirect INTEGER, tries INTEGER, version INTEGER, config_idx INTEGER, begins INTEGER, ends INTEGER, precise INTEGER, priority INTEGER, background INTEGER, bundle TEXT, url TEXT, data TEXT, token TEXT, title TEXT, description TEXT, method TEXT, headers TEXT, config_extras TEXT, mime_type TEXT, state INTEGER, idx INTEGER, total_processed INTEGER, sizes TEXT, processed TEXT, extras TEXT, form_items BLOB, file_specs BLOB, each_file_status BLOB, body_file_names BLOB, certs_paths BLOB)";
use super::{pause_task, start_task, stop_task};
use crate::info::State;
use crate::task::reason::Reason;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

// @tc.name: ut_start_pause_start
// @tc.desc: Test task start-pause-start sequence
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Create in-memory database
//           3. Insert test task
//           4. Start task and verify state
//           5. Pause task and verify state
//           6. Attempt to start again and verify state
// @tc.expect: Task transitions through start-pause states correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_start_pause_start() {
    init();

    let db = Connection::open_in_memory().unwrap();
    db.execute(
        &CREATE,
        (), // empty list of parameters.
    )
    .unwrap();

    let task_id: u32 = rand::random();
    db.execute(
        &format!(
            "INSERT INTO request_task (task_id, state) VALUES ({}, {})",
            task_id,
            State::Initialized.repr,
        ),
        (),
    )
    .unwrap();
    db.execute(&start_task(task_id), ()).unwrap();
    let mut stmt = db
        .prepare(&format!(
            "SELECT state from request_task where task_id = {}",
            task_id,
        ))
        .unwrap();
    let mut row = stmt
        .query_map([], |row| Ok(row.get::<_, u8>(0).unwrap()))
        .unwrap();
    let state = row.next().unwrap().unwrap();
    assert_eq!(state, State::Running.repr);
    db.execute(&pause_task(task_id), ()).unwrap();

    let mut stmt = db
        .prepare(&format!(
            "SELECT state from request_task where task_id = {}",
            task_id,
        ))
        .unwrap();
    let mut row = stmt
        .query_map([], |row| Ok(row.get::<_, u8>(0).unwrap()))
        .unwrap();
    let state = row.next().unwrap().unwrap();
    assert_eq!(state, State::Paused.repr);

    db.execute(&start_task(task_id), ()).unwrap();

    let mut stmt = db
        .prepare(&format!(
            "SELECT state from request_task where task_id = {}",
            task_id,
        ))
        .unwrap();
    let mut row = stmt
        .query_map([], |row| Ok(row.get::<_, u8>(0).unwrap()))
        .unwrap();
    let state = row.next().unwrap().unwrap();
    assert_eq!(state, State::Paused.repr);
}

// @tc.name: ut_pause
// @tc.desc: Test task pause functionality across different states
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Create in-memory database
//           3. Insert tasks in various states
//           4. Pause tasks and verify state transitions
// @tc.expect: Tasks in running, retrying, or waiting state transition to paused state
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_pause() {
    init();

    let db = Connection::open_in_memory().unwrap();
    db.execute(
        &CREATE,
        (), // empty list of parameters.
    )
    .unwrap();
    let states = [State::Running, State::Retrying, State::Waiting];
    let mut tasks = vec![];
    for state in states.iter() {
        let task_id: u32 = rand::random();
        tasks.push(task_id);
        db.execute(
            &format!(
                "INSERT INTO request_task (task_id, state) VALUES ({}, {})",
                task_id, state.repr,
            ),
            (),
        )
        .unwrap();
    }
    for task_id in tasks.iter() {
        db.execute(&pause_task(*task_id), ()).unwrap();
    }
    let mut stmt = db
        .prepare(&format!(
            "SELECT task_id from request_task where state = {} AND reason = {}",
            State::Paused.repr,
            Reason::UserOperation.repr
        ))
        .unwrap();
    let rows = stmt.query_map([], |row| Ok(row.get(0).unwrap())).unwrap();
    let mut res: Vec<u32> = rows.map(|r| r.unwrap()).collect();
    res.sort();
    tasks.sort();
    assert_eq!(tasks, res);
}

// @tc.name: ut_stop
// @tc.desc: Test task stop functionality across different states
// @tc.precon: NA
// @tc.step: 1. Initialize test environment
//           2. Create in-memory database
//           3. Insert tasks in various states
//           4. Stop tasks and verify state transitions
// @tc.expect: Tasks in running, retrying, or waiting state transition to stopped state
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_stop() {
    init();

    let db = Connection::open_in_memory().unwrap();
    db.execute(
        &CREATE,
        (), // empty list of parameters.
    )
    .unwrap();
    let states = [State::Running, State::Retrying, State::Waiting];
    let mut tasks = vec![];
    for state in states.iter() {
        let task_id: u32 = rand::random();
        tasks.push(task_id);
        db.execute(
            &format!(
                "INSERT INTO request_task (task_id, state) VALUES ({}, {})",
                task_id, state.repr,
            ),
            (),
        )
        .unwrap();
    }
    for task_id in tasks.iter() {
        db.execute(&&stop_task(*task_id), ()).unwrap();
    }
    let mut stmt = db
        .prepare(&format!(
            "SELECT task_id from request_task where state = {} AND reason = {}",
            State::Stopped.repr,
            Reason::UserOperation.repr
        ))
        .unwrap();
    let rows = stmt.query_map([], |row| Ok(row.get(0).unwrap())).unwrap();
    let mut res: Vec<u32> = rows.map(|r| r.unwrap()).collect();
    res.sort();
    tasks.sort();
    assert_eq!(tasks, res);
}