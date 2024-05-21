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

mod apps;
mod direction;
mod rss;

use apps::SortedApps;
pub(crate) use direction::{QosChanges, QosDirection, QosLevel};
pub(crate) use rss::RssCapacity;

use crate::manage::database::TaskQosInfo;
use crate::task::config::Action;
use crate::task::ffi::NetworkInfo;
use crate::task::info::ApplicationState;

pub(crate) struct Qos {
    // should not pub
    pub(crate) apps: SortedApps,
    capacity: RssCapacity,
}

impl Qos {
    pub(crate) fn new() -> Self {
        Self {
            apps: SortedApps::init(),
            capacity: RssCapacity::LEVEL0,
        }
    }

    // qos 里包含upload和download，通过empty确认哪些需要更新。
    pub(crate) fn start_task(
        &mut self,
        uid: u64,
        state: ApplicationState,
        task: TaskQosInfo,
    ) -> QosChanges {
        // Only tasks that can run automatically can be added to the qos queue.
        self.apps.insert_task(uid, state, task);
        self.reschedule(Action::from(task.action))
    }

    pub(crate) fn finish_task(&mut self, uid: u64, task_id: u32) -> QosChanges {
        if let Some(task) = self.apps.remove_task(uid, task_id) {
            self.reschedule(task.action())
        } else {
            self.reschedule(Action::Any)
        }
    }

    pub(crate) fn change_network(&mut self, network: NetworkInfo) -> QosChanges {
        self.apps.change_network(network);
        self.reschedule(Action::Any)
    }

    pub(crate) fn change_app_state(&mut self, uid: u64, state: ApplicationState) -> QosChanges {
        self.apps.change_app_state(uid, state);
        self.reschedule(Action::Any)
    }

    pub(crate) fn change_rss(&mut self, rss: RssCapacity) -> QosChanges {
        self.capacity = rss;
        self.reschedule(Action::Any)
    }

    pub(crate) fn contains_task(&mut self, uid: u64, task_id: u32) -> bool {
        self.apps.contains_task(uid, task_id)
    }
}

impl Qos {
    // Reschedule qos queue and get directions.
    pub(crate) fn reschedule(&mut self, action: Action) -> QosChanges {
        let mut changes = QosChanges::new();
        match action {
            Action::Any => {
                changes.download = Some(self.reschedule_inner(Action::Download));
                changes.upload = Some(self.reschedule_inner(Action::Upload));
            }
            Action::Download => {
                changes.download = Some(self.reschedule_inner(Action::Download));
            }
            Action::Upload => {
                changes.upload = Some(self.reschedule_inner(Action::Upload));
            }
        }
        changes
    }

    fn reschedule_inner(&mut self, action: Action) -> Vec<QosDirection> {
        let m1 = self.capacity.m1();
        let m1_speed = self.capacity.m1_speed();
        let m2 = self.capacity.m2();
        let m2_speed = self.capacity.m2_speed();
        let m3 = self.capacity.m3();
        let m3_speed = self.capacity.m3_speed();

        let mut count = 0;
        let mut app_i = 0;
        let mut task_i = 0;

        let mut qos_vec = Vec::new();

        for (i, task) in self.apps.iter().enumerate().flat_map(|(i, app)| {
            if !app.is_empty() {
                app_i = i;
            }
            app.iter().enumerate()
        }) {
            if task.action() != action {
                continue;
            }
            if count < m1 {
                qos_vec.push(QosDirection::new(task.task_id(), m1_speed));
            } else if count < m1 + m2 {
                qos_vec.push(QosDirection::new(task.task_id(), m2_speed));
            }
            count += 1;
            if count == m1 + m2 {
                task_i = i;
                break;
            }
        }

        // Here if the number of all uncompleted tasks is less than `m1 + m2`,
        // we don not need to adjust `m3` position.
        if count < m1 + m2 {
            return qos_vec;
        }

        // The filtering logic for fair position is executed as follows:
        // Each app will take turns taking one task to execute until the
        // fair position is filled.
        let mut i = 0;

        loop {
            let mut no_tasks_left = true;

            for tasks in self.apps.iter().skip(app_i + 1).map(|app| &app[..]) {
                let task = match tasks.get(i) {
                    Some(task) => {
                        no_tasks_left = false;
                        task
                    }
                    None => continue,
                };

                if task.action() != action {
                    continue;
                }

                if count < m1 + m2 + m3 {
                    qos_vec.push(QosDirection::new(task.task_id(), m3_speed));
                } else {
                    return qos_vec;
                }

                count += 1;
            }

            if no_tasks_left {
                break;
            }
            i += 1;
        }

        // supplement fair position with remaining tasks
        for task in self
            .apps
            .iter()
            .skip(app_i)
            .take(1)
            .flat_map(|app| app.iter().skip(task_i + 1))
        {
            if task.action() != action {
                continue;
            }

            if count < m1 + m2 + m3 {
                qos_vec.push(QosDirection::new(task.task_id(), m3_speed));
            } else {
                return qos_vec;
            }

            count += 1;
        }

        qos_vec
    }
}
