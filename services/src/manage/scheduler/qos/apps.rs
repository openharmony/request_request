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

    pub(crate) fn sort(&mut self, foreground_abilities: &HashSet<u64>, top_user: u64) {
        self.inner.sort_by(|a, b| {
            (a.uid / 200000 == top_user)
                .cmp(&(b.uid / 200000 == top_user))
                .then(
                    foreground_abilities
                        .contains(&a.uid)
                        .cmp(&(foreground_abilities.contains(&b.uid))),
                )
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

    fn get_app_mut(&mut self, uid: u64) -> Option<&mut App> {
        self.inner.iter_mut().find(|app| app.uid == uid)
    }

    pub(crate) fn remove_task(&mut self, uid: u64, task_id: u32) -> bool {
        match self.get_app_mut(uid) {
            // Remove target task in target app.
            Some(app) => app.remove(task_id),
            None => false,
        }
    }

    pub(crate) fn task_set_mode(&mut self, uid: u64, task_id: u32, mode: Mode) -> bool {
        match self.get_app_mut(uid) {
            // Remove target task in target app.
            Some(app) => app.task_set_mode(task_id, mode),
            None => false,
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
    pub(crate) tasks: Vec<Task>,
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

    fn insert(&mut self, task: Task) {
        self.tasks.binary_insert(task)
    }

    fn get_task_mut(&mut self, task_id: u32) -> Option<(usize, &mut Task)> {
        self.tasks
            .iter_mut()
            .enumerate()
            .find(|(_, task)| task.task_id == task_id)
    }

    fn remove(&mut self, task_id: u32) -> bool {
        match self.get_task_mut(task_id) {
            Some((index, _task)) => {
                self.tasks.remove(index);
                // Remove do not need to resort tasks.
                true
            }
            None => false,
        }
    }

    fn resort_tasks(&mut self) {
        self.tasks.sort();
    }

    fn task_set_mode(&mut self, task_id: u32, mode: Mode) -> bool {
        match self.get_task_mut(task_id) {
            Some((_index, task)) => {
                task.set_mode(mode);
                self.resort_tasks();
                true
            }
            None => false,
        }
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

    pub(crate) fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
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
mod ut_apps {
    include!("../../../../tests/ut/manage/scheduler/qos/ut_apps.rs");
}
