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
use std::ops::Deref;

use crate::manage::database::{Database, TaskQosInfo};
use crate::task::config::Action;
use crate::task::ffi::NetworkInfo;
use crate::task::info::{ApplicationState, Mode};
use crate::utils::c_wrapper::CStringWrapper;

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

    pub(crate) fn change_network(&mut self, network: NetworkInfo) {
        // Firstly, we need to update the status of the tasks waiting to be
        // executed based on changes in network status.
        Database::get_instance().update_on_network_change(network);

        // Then, we need to reload the app based on the current status of all tasks.
        self.inner = reload_all_app_from_database();
    }

    pub(crate) fn change_app_state(&mut self, uid: u64, state: ApplicationState) {
        if let Some(app) = self.inner.iter_mut().find(|app| app.uid == uid) {
            Database::get_instance().update_on_app_state_change(uid, state);

            let tasks = reload_tasks_of_app_from_database(uid);
            if !tasks.is_empty() {
                app.state = state;
                app.tasks = tasks;
            }
        }
    }

    pub(crate) fn insert_task(&mut self, uid: u64, state: ApplicationState, task: TaskQosInfo) {
        let task = Task::from(task);

        if let Some(app) = self.inner.iter_mut().find(|app| app.uid == uid) {
            app.insert(task);
            return;
        }

        let mut app = App::new(uid, state);
        app.insert(task);
        self.inner.binary_insert(app);
    }

    pub(crate) fn remove_task(&mut self, uid: u64, task_id: u32) -> Option<Task> {
        let mut task = None;
        // Remove target task in target app.
        if let Some(app) = self.inner.iter_mut().find(|app| app.uid == uid) {
            task = app.remove(task_id);
        }

        task
    }

    pub(crate) fn contains_task(&mut self, uid: u64, task_id: u32) -> bool {
        if let Some(app) = self.inner.iter().find(|app| app.uid == uid) {
            app.contains(task_id)
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
    state: ApplicationState,
    tasks: Vec<Task>,
}

impl App {
    fn new(uid: u64, state: ApplicationState) -> Self {
        Self {
            uid,
            state,
            tasks: Vec::new(),
        }
    }

    fn from_raw(uid: u64, state: ApplicationState, tasks: Vec<Task>) -> Self {
        Self { uid, state, tasks }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    #[allow(dead_code)]
    pub(crate) fn len(&self) -> usize {
        self.tasks.len()
    }

    fn insert(&mut self, task: Task) {
        self.tasks.binary_insert(task)
    }

    fn remove(&mut self, task_id: u32) -> Option<Task> {
        if let Some((i, _)) = self
            .tasks
            .iter()
            .enumerate()
            .find(|(_, task)| task.task_id == task_id)
        {
            Some(self.tasks.remove(i))
        } else {
            None
        }
    }

    fn contains(&self, task_id: u32) -> bool {
        if self.tasks.iter().any(|task| task.task_id == task_id) {
            return true;
        }
        false
    }
}

impl Deref for App {
    type Target = Vec<Task>;

    fn deref(&self) -> &Self::Target {
        &self.tasks
    }
}

impl Eq for App {}

impl Ord for App {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for App {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl PartialOrd for App {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.state.cmp(&other.state))
    }
}

pub(crate) struct Task {
    task_id: u32,
    mode: Mode,
    action: Action,
    priority: u32,
}

impl Task {
    pub(crate) fn task_id(&self) -> u32 {
        self.task_id
    }

    pub(crate) fn action(&self) -> Action {
        self.action
    }
}

impl From<TaskQosInfo> for Task {
    fn from(value: TaskQosInfo) -> Self {
        Self {
            task_id: value.task_id,
            mode: Mode::from(value.mode),
            action: Action::from(value.action),
            priority: value.priority,
        }
    }
}

impl Eq for Task {}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode && self.priority == other.priority
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(
            self.mode
                .cmp(&other.mode)
                .then(self.priority.cmp(&other.priority)),
        )
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

fn top_bundle() -> String {
    unsafe { GetTopBundleName() }.to_string()
}

fn reload_all_app_from_database() -> Vec<App> {
    let mut inner = Vec::new();
    let top_bundle = top_bundle();
    for (uid, bundle) in reload_app_list_from_database() {
        let state = if top_bundle == bundle {
            ApplicationState::Foreground
        } else {
            ApplicationState::Background
        };
        let mut tasks = reload_tasks_of_app_from_database(uid);
        // Ensure that tasks are arranged in order
        tasks.sort();
        inner.push(App::from_raw(uid, state, tasks));
    }
    inner.sort();
    inner
}

fn reload_tasks_of_app_from_database(uid: u64) -> Vec<Task> {
    Database::get_instance()
        .get_app_task_qos_infos(uid)
        .iter()
        .map(|info| Task {
            task_id: info.task_id,
            mode: Mode::from(info.mode),
            action: Action::from(info.action),
            priority: info.priority,
        })
        .collect()
}

fn reload_app_list_from_database() -> Vec<(u64, String)> {
    Database::get_instance().get_app_infos()
}

#[link(name = "download_server_cxx", kind = "static")]
extern "C" {
    pub(crate) fn GetTopBundleName() -> CStringWrapper;
}

#[cfg(test)]
mod ut_manage_scheduler_qos_apps {
    use super::{App, Task};
    use crate::task::info::{ApplicationState, Mode};

    impl Task {
        fn new(task_id: u32, mode: Mode, priority: u32) -> Self {
            Self {
                action: crate::task::config::Action::Any,
                task_id,
                mode,
                priority,
            }
        }
    }

    #[test]
    fn ut_sorted_app_insert_task() {}

    #[test]
    fn ut_app_insert() {
        let mut app = App::new(1, ApplicationState::Foreground);
        assert!(app.tasks.is_empty());
        assert_eq!(app.uid, 1);
        assert_eq!(app.state, ApplicationState::Foreground);

        app.insert(Task::new(1, Mode::FrontEnd, 0));
        assert_eq!(app.tasks.len(), 1);
        assert_eq!(app.tasks[0].task_id, 1);
        assert_eq!(app.tasks[0].mode, Mode::FrontEnd);
        assert_eq!(app.tasks[0].priority, 0);

        app.insert(Task::new(2, Mode::FrontEnd, 100));
        assert_eq!(app.tasks.len(), 2);
        assert_eq!(app.tasks[0].task_id, 1);
        assert_eq!(app.tasks[1].task_id, 2);

        app.insert(Task::new(3, Mode::FrontEnd, 50));
        assert_eq!(app.tasks.len(), 3);
        assert_eq!(app.tasks[0].task_id, 1);
        assert_eq!(app.tasks[1].task_id, 3);
        assert_eq!(app.tasks[2].task_id, 2);

        app.insert(Task::new(4, Mode::BackGround, 0));
        assert_eq!(app.tasks.len(), 3);
        assert_eq!(app.tasks[0].task_id, 1);
        assert_eq!(app.tasks[1].task_id, 3);
        assert_eq!(app.tasks[2].task_id, 2);
        assert_eq!(app.tasks[3].task_id, 4);

        app.insert(Task::new(5, Mode::BackGround, 100));
        assert_eq!(app.tasks.len(), 3);
        assert_eq!(app.tasks[0].task_id, 1);
        assert_eq!(app.tasks[1].task_id, 3);
        assert_eq!(app.tasks[2].task_id, 2);
        assert_eq!(app.tasks[3].task_id, 4);
        assert_eq!(app.tasks[4].task_id, 5);

        app.insert(Task::new(6, Mode::BackGround, 50));
        assert_eq!(app.tasks.len(), 3);
        assert_eq!(app.tasks[0].task_id, 1);
        assert_eq!(app.tasks[1].task_id, 3);
        assert_eq!(app.tasks[2].task_id, 2);
        assert_eq!(app.tasks[3].task_id, 4);
        assert_eq!(app.tasks[4].task_id, 6);
        assert_eq!(app.tasks[5].task_id, 5);
    }

    #[test]
    fn ut_app_remove() {
        let mut app = App::new(1, ApplicationState::Foreground);
        for i in 0..5 {
            app.insert(Task::new(i, Mode::FrontEnd, i));
        }
        assert_eq!(app.tasks.len(), 5);
        assert_eq!(app.tasks[0].task_id, 1);
        assert_eq!(app.tasks[1].task_id, 2);
        assert_eq!(app.tasks[2].task_id, 3);
        assert_eq!(app.tasks[3].task_id, 4);
        assert_eq!(app.tasks[4].task_id, 5);

        app.remove(6);
        assert_eq!(app.tasks.len(), 5);
        assert_eq!(app.tasks[0].task_id, 1);
        assert_eq!(app.tasks[1].task_id, 2);
        assert_eq!(app.tasks[2].task_id, 3);
        assert_eq!(app.tasks[3].task_id, 4);
        assert_eq!(app.tasks[4].task_id, 5);

        app.remove(3);
        assert_eq!(app.tasks.len(), 4);
        assert_eq!(app.tasks[0].task_id, 1);
        assert_eq!(app.tasks[1].task_id, 2);
        assert_eq!(app.tasks[2].task_id, 4);
        assert_eq!(app.tasks[3].task_id, 5);

        app.remove(1);
        assert_eq!(app.tasks.len(), 3);
        assert_eq!(app.tasks[0].task_id, 2);
        assert_eq!(app.tasks[1].task_id, 4);
        assert_eq!(app.tasks[2].task_id, 5);

        app.remove(4);
        assert_eq!(app.tasks.len(), 2);
        assert_eq!(app.tasks[0].task_id, 2);
        assert_eq!(app.tasks[1].task_id, 5);

        app.remove(5);
        assert_eq!(app.tasks.len(), 1);
        assert_eq!(app.tasks[0].task_id, 2);

        app.remove(2);
    }

    #[test]
    fn ut_app_partial_ord() {
        let app1 = App::new(1, ApplicationState::Foreground);
        let app2 = App::new(2, ApplicationState::Background);
        assert!(app1 < app2);
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
}
