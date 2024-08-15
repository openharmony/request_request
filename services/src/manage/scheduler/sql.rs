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

use crate::config::Action;
use crate::info::State;
use crate::task::reason::Reason;

pub(super) fn start_task(task_id: u32) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id = {} AND (state = {} OR state = {} OR (action = {} AND (state = {} OR state = {} )))",
        State::Waiting.repr,
        Reason::RunningTaskMeetLimits.repr,
        task_id,
        State::Initialized.repr,
        State::Paused.repr,
        Action::Download.repr,
        State::Failed.repr,
        State::Stopped.repr,
    )
}

pub(super) fn pause_task(task_id: u32) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id = {} AND (state = {} OR state = {} OR state = {})",
        State::Paused.repr,
        Reason::UserOperation.repr,
        task_id,
        State::Running.repr,
        State::Retrying.repr,
        State::Waiting.repr,
    )
}

pub(super) fn stop_task(task_id: u32) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id = {} AND (state = {} OR state = {} OR state = {})",
        State::Stopped.repr,
        Reason::UserOperation.repr,
        task_id,
        State::Running.repr,
        State::Retrying.repr,
        State::Waiting.repr,
    )
}

pub(super) fn remove_task(task_id: u32) -> String {
    format!(
        "UPDATE request_task SET state = {}, reason = {} where task_id = {}",
        State::Removed.repr,
        Reason::UserOperation.repr,
        task_id,
    )
}

#[cfg(all(not(feature = "oh"), test))]
mod test {
    use rusqlite::Connection;

    const CREATE: &'static str = "CREATE TABLE IF NOT EXISTS request_task (task_id INTEGER PRIMARY KEY, uid INTEGER, token_id INTEGER, action INTEGER, mode INTEGER, cover INTEGER, network INTEGER, metered INTEGER, roaming INTEGER, ctime INTEGER, mtime INTEGER, reason INTEGER, gauge INTEGER, retry INTEGER, redirect INTEGER, tries INTEGER, version INTEGER, config_idx INTEGER, begins INTEGER, ends INTEGER, precise INTEGER, priority INTEGER, background INTEGER, bundle TEXT, url TEXT, data TEXT, token TEXT, title TEXT, description TEXT, method TEXT, headers TEXT, config_extras TEXT, mime_type TEXT, state INTEGER, idx INTEGER, total_processed INTEGER, sizes TEXT, processed TEXT, extras TEXT, form_items BLOB, file_specs BLOB, each_file_status BLOB, body_file_names BLOB, certs_paths BLOB)";
    use super::{pause_task, start_task, stop_task};
    use crate::info::State;
    use crate::task::reason::Reason;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

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
}
