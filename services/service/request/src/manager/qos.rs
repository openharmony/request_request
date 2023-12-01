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

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::sync::Arc;

use crate::task::info::ApplicationState;
use crate::task::RequestTask;

const HIGH_QOS_MAX: usize = 10;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct QosCase {
    uid: u64,
    task_id: u32,
    qos_index: u32,
}

impl PartialOrd for QosCase {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QosCase {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.uid != other.uid {
            return Ordering::Equal;
        }
        self.qos_index.cmp(&other.qos_index)
    }
}

impl QosCase {
    fn new(uid: u64, task_id: u32, qos_index: u32) -> Self {
        Self {
            uid,
            task_id,
            qos_index,
        }
    }
}

#[derive(Debug)]
pub(crate) struct QosQueue {
    foreground_high_qos_cases: Vec<QosCase>,
    foreground_low_qos_cases: HashMap<u64, Vec<QosCase>>,

    background_high_qos_cases: Vec<QosCase>,
    background_low_qos_cases: HashMap<u64, Vec<QosCase>>,

    tasks: HashSet<u32>,

    app_state_map: HashMap<u64, ApplicationState>,
    app_task_count: HashMap<u64, usize>,
}

#[derive(Debug)]
pub(crate) enum Qos {
    High,
    Low,
}

impl QosQueue {
    pub(crate) fn new() -> Self {
        Self {
            foreground_high_qos_cases: Vec::with_capacity(HIGH_QOS_MAX),

            foreground_low_qos_cases: HashMap::new(),

            background_high_qos_cases: Vec::with_capacity(HIGH_QOS_MAX),

            background_low_qos_cases: HashMap::new(),

            tasks: HashSet::new(),

            app_state_map: HashMap::new(),
            app_task_count: HashMap::new(),
        }
    }

    pub(crate) fn insert(
        &mut self,
        task: &Arc<RequestTask>,
        app_state: ApplicationState,
    ) -> Vec<(u32, Qos)> {
        let task_id = task.conf.common_data.task_id;
        let uid = task.conf.common_data.uid;
        let priority = task.conf.common_data.priority;

        if self.tasks.contains(&task_id) {
            error!(
                "Qos insert a task twice, uid:{} task_id:{} priority:{}",
                uid, task_id, priority
            );
            return vec![];
        }

        if app_state == ApplicationState::Terminated {
            error!(
                "Qos insert a terminated task, uid:{} task_id:{} priority:{}",
                uid, task_id, priority
            );
            return vec![];
        }

        debug!(
            "Qos insert a task, uid:{} task_id:{} priority:{}",
            uid, task_id, priority
        );

        self.tasks.insert(task_id);

        match self.app_task_count.get_mut(&uid) {
            Some(count) => *count += 1,
            None => {
                self.app_task_count.insert(uid, 1);
            }
        }

        let case = QosCase::new(uid, task_id, priority);

        match self.app_state_map.get(&uid) {
            Some(state) => {
                if *state != app_state {
                    error!(
                        "Qos app_state_map state:{:?} not eq to inserted app_state:{:?}",
                        state, app_state
                    );

                    let mut qos_changes = self.change_state(uid, app_state);
                    qos_changes.extend(self.insert_inner(case, app_state));
                    qos_changes
                } else {
                    self.insert_inner(case, app_state)
                }
            }
            None => {
                self.app_state_map.insert(uid, app_state);
                self.insert_inner(case, app_state)
            }
        }
    }

    fn insert_inner(&mut self, case: QosCase, state: ApplicationState) -> Vec<(u32, Qos)> {
        match state {
            ApplicationState::Foreground => self.frontground_insert(case, state),

            ApplicationState::Background => self.background_insert(case, state),

            _ => unreachable!(),
        }
    }

    fn frontground_insert(&mut self, case: QosCase, state: ApplicationState) -> Vec<(u32, Qos)> {
        if self.foreground_high_qos_cases.len() < HIGH_QOS_MAX {
            let mut qos_changes = Vec::new();
            qos_changes.push((case.task_id, Qos::High));

            self.foreground_high_qos_cases.push(case);

            if self.background_high_qos_cases.len() + self.foreground_high_qos_cases.len()
                > HIGH_QOS_MAX
            {
                self.background_high_qos_cases.sort();
                let down_grade_case = self.background_high_qos_cases.pop().unwrap();

                qos_changes.push((down_grade_case.task_id, Qos::Low));

                match self.background_low_qos_cases.get_mut(&down_grade_case.uid) {
                    Some(low_qos_cases) => {
                        low_qos_cases.push(down_grade_case);
                    }
                    None => {
                        let mut low_qos_cases = Vec::new();
                        let uid = down_grade_case.uid;
                        low_qos_cases.push(down_grade_case);
                        self.background_low_qos_cases.insert(uid, low_qos_cases);
                    }
                }
            }
            qos_changes
        } else {
            self.contest_insert(case, state)
        }
    }

    fn background_insert(&mut self, case: QosCase, state: ApplicationState) -> Vec<(u32, Qos)> {
        if self.background_high_qos_cases.len() + self.foreground_high_qos_cases.len()
            < HIGH_QOS_MAX
        {
            let task_id = case.task_id;
            self.background_high_qos_cases.push(case);
            vec![(task_id, Qos::High)]
        } else {
            self.contest_insert(case, state)
        }
    }

    fn contest_insert(&mut self, mut case: QosCase, state: ApplicationState) -> Vec<(u32, Qos)> {
        let high_qos_cases = match state {
            ApplicationState::Foreground => &mut self.foreground_high_qos_cases,
            ApplicationState::Background => &mut self.background_high_qos_cases,
            ApplicationState::Terminated => unreachable!(),
        };

        let low_qos_cases = match state {
            ApplicationState::Foreground => &mut self.foreground_low_qos_cases,
            ApplicationState::Background => &mut self.background_low_qos_cases,
            ApplicationState::Terminated => unreachable!(),
        };

        let mut qos_changes = Vec::new();

        let mut down_grade_case = &case;
        let mut swap_case_index_opt = None;
        for (i, swap_case) in high_qos_cases
            .iter()
            .enumerate()
            .filter(|(_, swap)| swap.uid == case.uid)
        {
            if down_grade_case.qos_index < swap_case.qos_index {
                down_grade_case = swap_case;
                swap_case_index_opt = Some(i)
            }
        }

        if let Some(i) = swap_case_index_opt {
            qos_changes.push((case.task_id, Qos::High));
            mem::swap(&mut case, high_qos_cases.get_mut(i).unwrap());
        }

        qos_changes.push((case.task_id, Qos::Low));
        match low_qos_cases.get_mut(&case.uid) {
            Some(cases) => {
                cases.push(case);
            }
            None => {
                let mut cases = Vec::new();
                let uid = case.uid;
                cases.push(case);
                low_qos_cases.insert(uid, cases);
            }
        }

        qos_changes
    }

    pub(crate) fn remove(&mut self, uid: u64, task_id: u32) -> Vec<(u32, Qos)> {
        let state = match self.app_state_map.get(&uid) {
            None => {
                error!("Qos can not find app_state, uid:{}", uid);
                return vec![];
            }
            Some(state) => state,
        };

        if !self.tasks.remove(&task_id) {
            debug!("Qos remove task_id:{} that not exist", task_id);
            return vec![];
        }

        debug!("Qos remove uid:{} task_id:{}", uid, task_id);

        match self.app_task_count.get_mut(&uid) {
            Some(count) => {
                *count -= 1;
                if *count == 0 {
                    self.app_task_count.remove(&uid);
                }
            }
            None => {
                error!(
                    "Qos remove task_id:{}, but uid:{} count task 0",
                    task_id, uid
                );
            }
        }

        match state {
            ApplicationState::Foreground => self.foreground_remove(uid, task_id),
            ApplicationState::Background => self.background_remove(uid, task_id),
            ApplicationState::Terminated => unreachable!(),
        }
    }

    fn foreground_remove(&mut self, uid: u64, task_id: u32) -> Vec<(u32, Qos)> {
        let mut qos_changes = vec![];

        for i in 0..self.foreground_high_qos_cases.len() {
            if self.foreground_high_qos_cases[i].task_id == task_id {
                self.foreground_high_qos_cases.remove(i);
                for low_qos_cases in self.foreground_low_qos_cases.values_mut() {
                    low_qos_cases.sort_by(|a, b| b.qos_index.cmp(&a.qos_index));
                    if let Some(case) = low_qos_cases.pop() {
                        qos_changes.push((case.task_id, Qos::High));
                        self.foreground_high_qos_cases.push(case);
                        return qos_changes;
                    }
                }

                for low_qos_cases in self.background_low_qos_cases.values_mut() {
                    low_qos_cases.sort_by(|a, b| b.qos_index.cmp(&a.qos_index));
                    if let Some(case) = low_qos_cases.pop() {
                        qos_changes.push((case.task_id, Qos::High));
                        self.background_high_qos_cases.push(case);
                        return qos_changes;
                    }
                }
                return qos_changes;
            }
        }

        if let Some(set) = self.foreground_low_qos_cases.get_mut(&uid) {
            for i in 0..set.len() {
                if set[i].task_id == task_id {
                    set.remove(i);
                }
            }
        }

        qos_changes
    }
    fn background_remove(&mut self, uid: u64, task_id: u32) -> Vec<(u32, Qos)> {
        let mut qos_changes = vec![];

        for i in 0..self.background_high_qos_cases.len() {
            if self.background_high_qos_cases[i].task_id == task_id {
                self.background_high_qos_cases.remove(i);

                for low_qos_cases in self.background_low_qos_cases.values_mut() {
                    low_qos_cases.sort_by(|a, b| b.qos_index.cmp(&a.qos_index));
                    if let Some(case) = low_qos_cases.pop() {
                        qos_changes.push((case.task_id, Qos::High));
                        self.background_high_qos_cases.push(case);
                        return qos_changes;
                    }
                }
                return qos_changes;
            }
        }

        if let Some(set) = self.background_low_qos_cases.get_mut(&uid) {
            for i in 0..set.len() {
                if set[i].task_id == task_id {
                    set.remove(i);
                }
            }
        }

        qos_changes
    }

    pub(crate) fn change_state(
        &mut self,
        uid: u64,
        new_state: ApplicationState,
    ) -> Vec<(u32, Qos)> {
        let state = match self.app_state_map.get(&uid) {
            None => return vec![],
            Some(state) => {
                if *state == new_state {
                    error!("Qos change state with the same state");
                    return vec![];
                } else {
                    *state
                }
            }
        };
        debug!(
            "Qos change state uid:{}, state:{:?}, new_state:{:?}",
            uid, state, new_state
        );

        self.app_state_map.insert(uid, new_state);

        match new_state {
            ApplicationState::Foreground => self.state_turn_to_foreground(uid),
            ApplicationState::Background => self.state_turn_to_background(uid),
            ApplicationState::Terminated => self.state_turn_to_terminated(uid, state),
        }
    }

    fn state_turn_to_foreground(&mut self, uid: u64) -> Vec<(u32, Qos)> {
        let mut qos_changes = vec![];
        let high_qos_cases = self
            .background_high_qos_cases
            .iter()
            .cloned()
            .filter(|case| case.uid == uid);

        self.foreground_high_qos_cases.extend(high_qos_cases);

        self.background_high_qos_cases
            .retain(|case| case.uid != uid);

        if let Some(mut low_qos_cases) = self.background_low_qos_cases.remove(&uid) {
            low_qos_cases.sort_by(|a, b| b.qos_index.cmp(&a.qos_index));
            while self.foreground_high_qos_cases.len() < HIGH_QOS_MAX {
                if let Some(case) = low_qos_cases.pop() {
                    qos_changes.extend(self.frontground_insert(case, ApplicationState::Foreground));
                } else {
                    break;
                }
            }
            if !low_qos_cases.is_empty() {
                self.foreground_low_qos_cases.insert(uid, low_qos_cases);
            }
        }
        qos_changes
    }

    fn state_turn_to_background(&mut self, uid: u64) -> Vec<(u32, Qos)> {
        let mut qos_changes = vec![];

        if let Some(low_qos_cases) = self.foreground_low_qos_cases.remove(&uid) {
            self.background_low_qos_cases.insert(uid, low_qos_cases);
        }

        let mut high_qos_cases = self
            .foreground_high_qos_cases
            .iter()
            .cloned()
            .filter(|case| case.uid == uid)
            .collect::<Vec<_>>();

        self.foreground_high_qos_cases
            .retain(|case| case.uid != uid);

        'a: for low_qos_cases in self.foreground_low_qos_cases.values_mut() {
            low_qos_cases.sort_by(|a, b| b.qos_index.cmp(&a.qos_index));
            while self.foreground_high_qos_cases.len() < HIGH_QOS_MAX {
                if let Some(case) = low_qos_cases.pop() {
                    qos_changes.push((case.task_id, Qos::High));
                    self.foreground_high_qos_cases.push(case);
                } else {
                    break 'a;
                }
            }
        }

        self.foreground_low_qos_cases
            .retain(|_, cases| !cases.is_empty());

        if self.foreground_high_qos_cases.len() < 10 {
            high_qos_cases.sort_by(|a, b| b.qos_index.cmp(&a.qos_index));

            while self.background_high_qos_cases.len() + self.foreground_high_qos_cases.len()
                < HIGH_QOS_MAX
            {
                if let Some(case) = high_qos_cases.pop() {
                    self.background_high_qos_cases.push(case);
                } else {
                    break;
                }
            }
        }
        qos_changes.extend(high_qos_cases.iter().map(|case| (case.task_id, Qos::Low)));

        if !high_qos_cases.is_empty() {
            match self.background_low_qos_cases.get_mut(&uid) {
                Some(low_qos_cases) => {
                    low_qos_cases.extend(high_qos_cases);
                }
                None => {
                    let low_qos_cases = high_qos_cases.into_iter().collect();
                    self.background_low_qos_cases.insert(uid, low_qos_cases);
                }
            }
        }
        qos_changes
    }
    fn state_turn_to_terminated(
        &mut self,
        uid: u64,
        old_state: ApplicationState,
    ) -> Vec<(u32, Qos)> {
        let mut qos_changes = vec![];

        self.app_state_map.remove(&uid);
        self.app_task_count.remove(&uid);

        match old_state {
            ApplicationState::Background => {
                self.background_high_qos_cases
                    .iter()
                    .filter(|case| case.uid == uid)
                    .for_each(|case| {
                        self.tasks.remove(&case.task_id);
                    });

                self.background_high_qos_cases
                    .retain(|case| case.uid != uid);

                'a: for low_qos_cases in self.background_low_qos_cases.values_mut() {
                    low_qos_cases.sort_by(|a, b| b.qos_index.cmp(&a.qos_index));
                    while self.background_high_qos_cases.len()
                        + self.foreground_high_qos_cases.len()
                        < HIGH_QOS_MAX
                    {
                        if let Some(case) = low_qos_cases.pop() {
                            qos_changes.push((case.task_id, Qos::High));
                            self.background_high_qos_cases.push(case);
                        } else {
                            break 'a;
                        }
                    }
                }

                if let Some(remove_tasks) = self.background_low_qos_cases.remove(&uid) {
                    remove_tasks.into_iter().for_each(|case| {
                        self.tasks.remove(&case.task_id);
                    })
                }
            }
            ApplicationState::Foreground => {
                self.foreground_high_qos_cases
                    .iter()
                    .filter(|case| case.uid == uid)
                    .for_each(|case| {
                        self.tasks.remove(&case.task_id);
                    });

                self.foreground_high_qos_cases
                    .retain(|case| case.uid != uid);

                'a: for low_qos_cases in self.foreground_low_qos_cases.values_mut() {
                    low_qos_cases.sort_by(|a, b| b.qos_index.cmp(&a.qos_index));
                    while self.foreground_high_qos_cases.len() < HIGH_QOS_MAX {
                        if let Some(case) = low_qos_cases.pop() {
                            qos_changes.push((case.task_id, Qos::High));
                            self.foreground_high_qos_cases.push(case);
                        } else {
                            break 'a;
                        }
                    }
                }

                if let Some(remove_tasks) = self.foreground_low_qos_cases.remove(&uid) {
                    remove_tasks.into_iter().for_each(|case| {
                        self.tasks.remove(&case.task_id);
                    })
                }
            }
            _ => unreachable!(),
        }

        qos_changes
    }
}
