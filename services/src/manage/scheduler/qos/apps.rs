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

use std::cmp;
use std::collections::HashSet;
use std::ops::Deref;

use crate::manage::database::{RequestDb, TaskQosInfo};
use crate::task::config::{Action, Mode};

/// List of sorted apps.
pub(crate) struct SortedApps {
    inner: Vec<App>,
}

impl SortedApps {
    pub(crate) fn init() -> Self {
        Self {
            inner: reload_all_app_from_database(),
        }
    }

    pub(crate) fn sort(&mut self, top_uid: Option<u64>, top_user: u64) {
        self.inner.sort_by(|a, b| {
            (a.uid / 200000 == top_user)
                .cmp(&(b.uid / 200000 == top_user))
                .then((Some(a.uid) == top_uid).cmp(&(Some(b.uid) == top_uid)))
        })
    }

    pub(crate) fn reload_all_tasks(&mut self) {
        self.inner = reload_all_app_from_database();
    }

    pub(crate) fn insert_task(&mut self, uid: u64, task: TaskQosInfo) {
        let task = Task {
            uid,
            task_id: task.task_id,
            mode: Mode::from(task.mode),
            action: Action::from(task.action),
            priority: task.priority,
        };

        if let Some(app) = self.inner.iter_mut().find(|app| app.uid == uid) {
            app.insert(task);
            return;
        }

        let mut app = App::new(uid);
        app.insert(task);
        self.inner.push(app);
    }

    pub(crate) fn remove_task(&mut self, uid: u64, task_id: u32) -> bool {
        // Remove target task in target app.
        if let Some(app) = self.inner.iter_mut().find(|app| app.uid == uid) {
            app.remove(task_id)
        } else {
            false
        }
    }
}

impl Deref for SortedApps {
    type Target = Vec<App>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// An independent app.
pub(crate) struct App {
    pub(crate) uid: u64,
    tasks: Vec<Task>,
}

impl App {
    fn new(uid: u64) -> Self {
        Self {
            uid,
            tasks: Vec::new(),
        }
    }

    fn from_raw(uid: u64, tasks: Vec<Task>) -> Self {
        Self { uid, tasks }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    fn insert(&mut self, task: Task) {
        self.tasks.binary_insert(task)
    }

    fn remove(&mut self, task_id: u32) -> bool {
        if let Some((i, _)) = self
            .tasks
            .iter()
            .enumerate()
            .find(|(_, task)| task.task_id == task_id)
        {
            self.tasks.remove(i);
            true
        } else {
            false
        }
    }
}

impl Deref for App {
    type Target = Vec<Task>;

    fn deref(&self) -> &Self::Target {
        &self.tasks
    }
}

pub(crate) struct Task {
    uid: u64,
    task_id: u32,
    mode: Mode,
    action: Action,
    priority: u32,
}

impl Task {
    pub(crate) fn uid(&self) -> u64 {
        self.uid
    }

    pub(crate) fn task_id(&self) -> u32 {
        self.task_id
    }

    pub(crate) fn action(&self) -> Action {
        self.action
    }
}

impl Eq for Task {}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.mode
            .cmp(&other.mode)
            .then(self.priority.cmp(&other.priority))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode && self.priority == other.priority
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

trait Binary<T: Ord> {
    fn binary_insert(&mut self, value: T);
}

impl<T: Ord> Binary<T> for Vec<T> {
    fn binary_insert(&mut self, value: T) {
        match self.binary_search(&value) {
            Ok(n) => self.insert(n, value),
            Err(n) => self.insert(n, value),
        }
    }
}

fn reload_all_app_from_database() -> Vec<App> {
    let mut inner = Vec::new();
    for uid in reload_app_list_from_database() {
        let mut tasks = reload_tasks_of_app_from_database(uid);
        // Ensure that tasks are arranged in order
        tasks.sort();
        inner.push(App::from_raw(uid, tasks));
    }
    inner
}

fn reload_tasks_of_app_from_database(uid: u64) -> Vec<Task> {
    RequestDb::get_instance()
        .get_app_task_qos_infos(uid)
        .iter()
        .map(|info| Task {
            uid,
            task_id: info.task_id,
            mode: Mode::from(info.mode),
            action: Action::from(info.action),
            priority: info.priority,
        })
        .collect()
}

fn reload_app_list_from_database() -> HashSet<u64> {
    RequestDb::get_instance()
        .get_app_infos()
        .into_iter()
        .collect()
}

impl RequestDb {
    fn get_app_infos(&self) -> Vec<u64> {
        let sql = "SELECT DISTINCT uid FROM request_task";
        self.query_integer(sql)
    }
}

#[cfg(feature = "oh")]
#[cfg(test)]
mod ut_manage_scheduler_qos_apps {
    use super::{App, Task};
    use crate::manage::database::RequestDb;
    use crate::task::config::Mode;
    use crate::tests::test_init;
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

    #[test]
    fn ut_database_app_info() {
        test_init();
        let db = RequestDb::get_instance();
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
}
