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

pub(crate) use ffi::TaskFilter;

use crate::config::{Action, Mode};
use crate::manage::database::RequestDb;
use crate::manage::TaskManager;
use crate::task::info::State;

impl TaskManager {
    pub(crate) fn search(&self, filter: TaskFilter, method: SearchMethod) -> Vec<u32> {
        info!("Search task by filter: {:?} method: {:?}", filter, method);

        let mut database = RequestDb::get_instance();

        let res = match method {
            SearchMethod::User(uid) => database.search_task(filter, uid),
            SearchMethod::System(bundle_name) => database.system_search_task(filter, bundle_name),
        };
        info!("Search task result: {:?}", res);
        res
    }
}

impl RequestDb {
    pub(crate) fn search_task(&mut self, filter: TaskFilter, uid: u64) -> Vec<u32> {
        let mut sql = format!("SELECT task_id from request_task WHERE uid = {} AND ", uid);
        Self::search_filter(&mut sql, &filter);
        self.query_integer(&sql).unwrap_or_default()
    }

    pub(crate) fn system_search_task(
        &mut self,
        filter: TaskFilter,
        bundle_name: String,
    ) -> Vec<u32> {
        let mut sql = "SELECT task_id from request_task WHERE ".to_string();
        if bundle_name != "*" {
            sql.push_str(&format!("bundle = '{}' AND ", bundle_name));
        }
        Self::search_filter(&mut sql, &filter);
        self.query_integer(&sql).unwrap_or_default()
    }

    fn search_filter(sql: &mut String, filter: &TaskFilter) {
        sql.push_str(&format!(
            "ctime BETWEEN {} AND {} ",
            filter.after, filter.before
        ));
        if filter.state != State::Any.repr {
            sql.push_str(&format!("AND state = {} ", filter.state));
        }
        if filter.action != Action::Any.repr {
            sql.push_str(&format!("AND action = {} ", filter.action));
        }
        if filter.mode != Mode::Any.repr {
            sql.push_str(&format!("AND mode = {} ", filter.mode));
        }
    }
}

#[derive(Debug)]
pub(crate) enum SearchMethod {
    User(u64),
    System(String),
}

#[allow(unreachable_pub)]
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    #[derive(Debug)]
    struct TaskFilter {
        before: i64,
        after: i64,
        state: u8,
        action: u8,
        mode: u8,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::get_current_timestamp;
    use crate::utils::task_id_generator::TaskIdGenerator;

    #[test]
    fn ut_search_user() {
        let mut db = RequestDb::get_instance();
        let task_id = TaskIdGenerator::generate();
        let uid = get_current_timestamp();
        db.execute_sql(&format!(
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
        let mut db = RequestDb::get_instance();
        let task_id = TaskIdGenerator::generate();
        let bundle_name = "com.ohos.app";
        db.execute_sql(&format!(
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
}
