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

use crate::task::info::ApplicationState::{self, Background, Foreground, Terminated};
use crate::task::info::Mode;
use crate::task::RequestTask;
// The smaller the value of index, the higher the task priority.
// The smaller the value of Qoscase, the higher the task priority.
// Tasks with different uid have equal priority
#[derive(Debug, Hash, Clone)]
pub(crate) struct QosCase {
    uid: u64,
    task_id: u32,
    mode: Mode,
    priority: u32,
    qos: Option<Qos>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct QosChange {
    pub(crate) task_id: u32,
    pub(crate) new_qos: Qos,
}

impl QosChange {
    fn new(task_id: u32, new_qos: Qos) -> Self {
        Self { task_id, new_qos }
    }
}

impl QosCase {
    fn new(uid: u64, task_id: u32, mode: Mode, priority: u32) -> Self {
        Self {
            uid,
            task_id,
            mode,
            priority,
            qos: None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct QosQueue {
    high_qos_max: usize,

    foreground_high_qos_cases: Vec<QosCase>,
    // bool for sorted
    foreground_low_qos_cases: HashMap<u64, SortQueue>,

    background_high_qos_cases: Vec<QosCase>,

    // bool for sorted
    background_low_qos_cases: HashMap<u64, SortQueue>,

    tasks: HashSet<u32>,

    app_state_map: HashMap<u64, ApplicationState>,
    app_high_qos_count: HashMap<u64, usize>,
}

#[derive(Debug)]
struct SortQueue {
    cases: Vec<QosCase>,
    sorted: bool,
}

impl SortQueue {
    fn push(&mut self, case: QosCase) {
        self.cases.push(case);
        self.sorted = false;
    }

    fn pop_highest_qos(&mut self) -> Option<QosCase> {
        if !self.sorted {
            self.cases.sort_by(|me, other| {
                let res = other.mode.cmp(&me.mode);
                if res == Ordering::Equal {
                    other.priority.cmp(&me.priority)
                } else {
                    res
                }
            });
            self.sorted = true;
        }
        self.cases.pop()
    }

    fn len(&self) -> usize {
        self.cases.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn remove_by_id(&mut self, task_id: u32) {
        for i in 0..self.cases.len() {
            if self.cases[i].task_id == task_id {
                self.cases.remove(i);
                break;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) enum Qos {
    High,
    Low,
}

impl QosQueue {
    pub(crate) fn new(high_qos_max: usize) -> Self {
        Self {
            high_qos_max,

            foreground_high_qos_cases: Vec::with_capacity(high_qos_max),

            foreground_low_qos_cases: HashMap::new(),

            background_high_qos_cases: Vec::with_capacity(high_qos_max),

            background_low_qos_cases: HashMap::new(),

            tasks: HashSet::new(),

            app_state_map: HashMap::new(),
            app_high_qos_count: HashMap::new(),
        }
    }

    pub(crate) fn insert(
        &mut self,
        task: &Arc<RequestTask>,
        state: ApplicationState,
    ) -> Vec<QosChange> {
        let uid = task.conf.common_data.uid;
        let task_id = task.conf.common_data.task_id;
        let priority = task.conf.common_data.priority;
        let mode = task.conf.common_data.mode;
        if self.tasks.contains(&task_id) {
            error!("Qos insert task {} twice", task_id);
            return vec![];
        }

        if state == Terminated {
            error!("Qos insert a terminated task {}", task_id);
            return vec![];
        }

        let case = QosCase::new(uid, task_id, mode, priority);
        info!(
            "Qos insert {:?} task {} priority {} mode {:?}",
            state, case.task_id, case.priority, case.mode
        );
        self.insert_inner(case, state)
    }

    pub(crate) fn change_state(&mut self, uid: u64, new_state: ApplicationState) -> Vec<QosChange> {
        let state = match self.app_state_map.get_mut(&uid) {
            Some(old_state) => {
                if old_state == &new_state {
                    error!("Qos change state uid {} {:?} to the same", uid, old_state);
                    return vec![];
                } else {
                    info!(
                        "Qos change state uid {} form {:?} to {:?}",
                        uid, old_state, new_state
                    );
                    let old = *old_state;
                    *old_state = new_state;
                    old
                }
            }
            None => {
                error!(
                    "Qos change state uid {} {:?}, that has no tasks",
                    uid, new_state
                );
                return vec![];
            }
        };

        let res = match new_state {
            Foreground => self.state_turn_to_foreground(uid),
            Background => self.state_turn_to_background(uid),
            Terminated => self.state_turn_to_terminated(uid, state),
        };

        info!("Qos change state finished");

        res
    }

    pub(crate) fn remove(&mut self, uid: u64, task_id: u32) -> Vec<QosChange> {
        if !self.tasks.remove(&task_id) {
            debug!("Qos remove task_id:{} that not exist", task_id);
            return vec![];
        }

        let state = *self.app_state_map.get(&uid).unwrap();

        info!("Qos remove {:?} task {} uid {}", state, task_id, uid);

        let res = match state {
            Foreground => {
                let res = self.foreground_remove(uid, task_id);

                if let Some(case) = self.foreground_low_qos_cases.get(&uid) {
                    if case.is_empty() {
                        self.foreground_low_qos_cases.remove(&uid);
                    }
                }
                if *self.app_high_qos_count.get(&uid).unwrap() == 0
                    && self.foreground_low_qos_cases.get(&uid).is_none()
                {
                    self.app_high_qos_count.remove(&uid);
                    self.app_state_map.remove(&uid);
                }

                res
            }
            Background => {
                let res = self.background_remove(uid, task_id);

                if let Some(case) = self.background_low_qos_cases.get(&uid) {
                    if case.is_empty() {
                        self.background_low_qos_cases.remove(&uid);
                    }
                }
                if *self.app_high_qos_count.get(&uid).unwrap() == 0
                    && self.background_low_qos_cases.get(&uid).is_none()
                {
                    self.app_high_qos_count.remove(&uid);
                    self.app_state_map.remove(&uid);
                }

                res
            }
            Terminated => unreachable!(),
        };
        info!("Qos remove finished");
        res
    }

    fn insert_inner(&mut self, case: QosCase, insert_state: ApplicationState) -> Vec<QosChange> {
        self.tasks.insert(case.task_id);

        if self.app_high_qos_count.get(&case.uid).is_none() {
            self.app_high_qos_count.insert(case.uid, 0);
        }

        let res = match self.app_state_map.get(&case.uid) {
            Some(state) => {
                if *state != insert_state {
                    error!(
                        "Qos app_state_map state:{:?} not eq to inserted app_state:{:?}",
                        state, insert_state
                    );

                    let mut qos_changes = self.change_state(case.uid, insert_state);
                    qos_changes.extend(match insert_state {
                        Foreground => self.foreground_insert(case),
                        Background => self.background_insert(case),
                        Terminated => unreachable!(),
                    });

                    qos_changes
                } else {
                    match insert_state {
                        Foreground => self.foreground_insert(case),
                        Background => self.background_insert(case),
                        Terminated => unreachable!(),
                    }
                }
            }
            None => {
                self.app_state_map.insert(case.uid, insert_state);
                match insert_state {
                    Foreground => self.foreground_insert(case),
                    Background => self.background_insert(case),
                    Terminated => unreachable!(),
                }
            }
        };
        info!("Qos insert task finished");
        res
    }

    fn foreground_insert(&mut self, mut case: QosCase) -> Vec<QosChange> {
        let mut qos_changes = Vec::new();
        if self.foreground_high_qos_cases.len() < self.high_qos_max {
            change_qos(
                &mut self.app_high_qos_count,
                &mut qos_changes,
                &mut case,
                Qos::High,
            );
            self.push_high_qos(case, Foreground);

            if self.foreground_high_qos_cases.len() + self.background_high_qos_cases.len()
                > self.high_qos_max
            {
                self.move_one_high_qos_to_low(&mut qos_changes, Background);
            }
        } else {
            self.contest_insert(&mut qos_changes, case, Foreground);
        }
        qos_changes
    }

    fn background_insert(&mut self, mut case: QosCase) -> Vec<QosChange> {
        let mut qos_changes = vec![];
        if self.background_high_qos_cases.len() + self.foreground_high_qos_cases.len()
            < self.high_qos_max
        {
            change_qos(
                &mut self.app_high_qos_count,
                &mut qos_changes,
                &mut case,
                Qos::High,
            );
            self.push_high_qos(case, Background);
        } else {
            self.contest_insert(&mut qos_changes, case, Background)
        }
        qos_changes
    }

    fn move_one_high_qos_to_low(
        &mut self,
        qos_changes: &mut Vec<QosChange>,
        state: ApplicationState,
    ) {
        let mut down_grade_case = self.pop_high_qos(state).unwrap();
        change_qos(
            &mut self.app_high_qos_count,
            qos_changes,
            &mut down_grade_case,
            Qos::Low,
        );
        self.push_low_qos(down_grade_case, state);
    }

    fn push_high_qos(&mut self, case: QosCase, state: ApplicationState) {
        debug!("Qos task {} push to {:?} High Qos", case.task_id, state);
        match state {
            Foreground => {
                self.foreground_high_qos_cases.push(case);
            }
            Background => {
                self.background_high_qos_cases.push(case);
            }
            Terminated => unreachable!(),
        }
    }

    fn push_low_qos(&mut self, case: QosCase, state: ApplicationState) {
        debug!("Qos task {} push to {:?} Low Qos", case.task_id, state);

        let low_qos_cases = match state {
            Foreground => &mut self.foreground_low_qos_cases,
            Background => &mut self.background_low_qos_cases,
            Terminated => unreachable!(),
        };

        match low_qos_cases.get_mut(&case.uid) {
            Some(cases) => {
                cases.push(case);
            }
            None => {
                let mut cases = Vec::new();
                let uid = case.uid;
                cases.push(case);

                low_qos_cases.insert(
                    uid,
                    SortQueue {
                        cases,
                        sorted: true,
                    },
                );
            }
        }
    }

    fn pop_low_qos(&mut self, state: ApplicationState) -> Option<QosCase> {
        let low_qos_cases = match state {
            Foreground => &mut self.foreground_low_qos_cases,
            Background => &mut self.background_low_qos_cases,
            Terminated => unreachable!(),
        };
        let mut remove_uid = None;
        let res = low_qos_cases
            .iter_mut()
            .min_by(|me, other| {
                self.app_high_qos_count
                    .get(me.0)
                    .unwrap()
                    .cmp(self.app_high_qos_count.get(other.0).unwrap())
            })
            .map(|(uid, queue)| {
                let res = queue.pop_highest_qos().unwrap();
                if queue.is_empty() {
                    remove_uid = Some(*uid);
                }
                debug!("Qos pop task {} from {:?} Low Qos", res.task_id, state);
                res
            });
        if let Some(uid) = remove_uid {
            low_qos_cases.remove(&uid);
        }
        res
    }

    fn pop_high_qos(&mut self, state: ApplicationState) -> Option<QosCase> {
        let high_qos_cases = match state {
            Foreground => &mut self.foreground_high_qos_cases,
            Background => &mut self.background_high_qos_cases,
            Terminated => unreachable!(),
        };

        high_qos_cases.sort_by(|me, other| {
            if me.uid == other.uid {
                let res = me.mode.cmp(&other.mode);
                if res == Ordering::Equal {
                    me.priority.cmp(&other.priority)
                } else {
                    res
                }
            } else {
                self.app_high_qos_count
                    .get(&me.uid)
                    .unwrap()
                    .cmp(self.app_high_qos_count.get(&other.uid).unwrap())
            }
        });
        high_qos_cases.pop()
    }

    fn contest_insert(
        &mut self,
        qos_changes: &mut Vec<QosChange>,
        mut case: QosCase,
        state: ApplicationState,
    ) {
        if *self.app_high_qos_count.get(&case.uid).unwrap() == 0
            && (state == Foreground || !self.background_high_qos_cases.is_empty())
        {
            self.move_one_high_qos_to_low(qos_changes, state);

            change_qos(
                &mut self.app_high_qos_count,
                qos_changes,
                &mut case,
                Qos::High,
            );
            self.push_high_qos(case, state);
            return;
        }

        let high_qos_cases = match state {
            Foreground => &mut self.foreground_high_qos_cases,
            Background => &mut self.background_high_qos_cases,
            Terminated => unreachable!(),
        };

        let mut down_grade_case = &case;
        let mut swap_case_index_opt = None;
        for (i, swap_case) in high_qos_cases.iter().enumerate() {
            // For task with different uid, the uid with only one task will win the contest.
            if down_grade_case.uid == swap_case.uid
                && (down_grade_case.mode < swap_case.mode
                    || down_grade_case.priority < swap_case.priority)
            {
                down_grade_case = swap_case;
                swap_case_index_opt = Some(i)
            }
        }

        if let Some(i) = swap_case_index_opt {
            change_qos(
                &mut self.app_high_qos_count,
                qos_changes,
                &mut case,
                Qos::High,
            );
            mem::swap(&mut case, high_qos_cases.get_mut(i).unwrap());
            change_qos(
                &mut self.app_high_qos_count,
                qos_changes,
                &mut case,
                Qos::Low,
            );
            self.push_low_qos(case, state);
        } else {
            change_qos(
                &mut self.app_high_qos_count,
                qos_changes,
                &mut case,
                Qos::Low,
            );
            self.push_low_qos(case, state);
        }
    }

    fn foreground_remove(&mut self, uid: u64, task_id: u32) -> Vec<QosChange> {
        let mut qos_changes = vec![];
        for i in 0..self.foreground_high_qos_cases.len() {
            if self.foreground_high_qos_cases[i].task_id == task_id {
                *self.app_high_qos_count.get_mut(&uid).unwrap() -= 1;
                self.foreground_high_qos_cases.remove(i);

                if let Some(mut case) = self.pop_low_qos(Foreground) {
                    change_qos(
                        &mut self.app_high_qos_count,
                        &mut qos_changes,
                        &mut case,
                        Qos::High,
                    );
                    self.push_high_qos(case, Foreground);
                } else if let Some(mut case) = self.pop_low_qos(Background) {
                    change_qos(
                        &mut self.app_high_qos_count,
                        &mut qos_changes,
                        &mut case,
                        Qos::High,
                    );
                    self.push_high_qos(case, Background);
                }

                return qos_changes;
            }
        }

        self.foreground_low_qos_cases
            .get_mut(&uid)
            .unwrap()
            .remove_by_id(task_id);

        qos_changes
    }

    fn background_remove(&mut self, uid: u64, task_id: u32) -> Vec<QosChange> {
        let mut qos_changes = vec![];

        for i in 0..self.background_high_qos_cases.len() {
            if self.background_high_qos_cases[i].task_id == task_id {
                *self.app_high_qos_count.get_mut(&uid).unwrap() -= 1;
                self.background_high_qos_cases.remove(i);

                if let Some(mut case) = self.pop_low_qos(Background) {
                    change_qos(
                        &mut self.app_high_qos_count,
                        &mut qos_changes,
                        &mut case,
                        Qos::High,
                    );
                    self.push_high_qos(case, Background);
                }

                return qos_changes;
            }
        }

        self.background_low_qos_cases
            .get_mut(&uid)
            .unwrap()
            .remove_by_id(task_id);

        qos_changes
    }

    fn state_turn_to_foreground(&mut self, uid: u64) -> Vec<QosChange> {
        let mut qos_changes = vec![];

        let change_state_cases = self
            .background_high_qos_cases
            .iter()
            .cloned()
            .filter(|case| case.uid == uid);
        self.foreground_high_qos_cases.extend(change_state_cases);
        self.background_high_qos_cases
            .retain(|case| case.uid != uid);

        if let Some(mut low_qos_cases) = self.background_low_qos_cases.remove(&uid) {
            while self.foreground_high_qos_cases.len() < self.high_qos_max
                || self.app_high_qos_count.get(&uid).unwrap() == &0
            {
                if let Some(case) = low_qos_cases.pop_highest_qos() {
                    qos_changes.extend(self.foreground_insert(case));
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

    fn state_turn_to_background(&mut self, uid: u64) -> Vec<QosChange> {
        let mut qos_changes = vec![];

        if self.foreground_high_qos_cases.len() + self.background_high_qos_cases.len()
            < self.high_qos_max
        {
            let change_state_cases = self
                .foreground_high_qos_cases
                .iter()
                .cloned()
                .filter(|case| case.uid == uid);
            self.background_high_qos_cases.extend(change_state_cases);
            self.foreground_high_qos_cases
                .retain(|case| case.uid != uid);
            return qos_changes;
        }

        // move changes_state_cases to background_low_qos_cases
        let mut change_state_cases = self
            .foreground_high_qos_cases
            .iter()
            .cloned()
            .filter(|case| case.uid == uid)
            .map(|mut case| {
                change_qos(
                    &mut self.app_high_qos_count,
                    &mut qos_changes,
                    &mut case,
                    Qos::Low,
                );
                case
            })
            .collect::<Vec<_>>();

        self.foreground_high_qos_cases
            .retain(|case| case.uid != uid);

        if let Some(cases) = self.foreground_low_qos_cases.remove(&uid) {
            change_state_cases.extend(cases.cases);
        }

        self.background_low_qos_cases.insert(
            uid,
            SortQueue {
                cases: change_state_cases,
                sorted: false,
            },
        );

        // filling high qos task vacancies
        if self.foreground_high_qos_cases.len() < self.high_qos_max
            && !self.foreground_low_qos_cases.is_empty()
        {
            self.high_qos_vacant(&mut qos_changes, Foreground);
        }

        if self.foreground_high_qos_cases.len() + self.background_high_qos_cases.len()
            < self.high_qos_max
            && !self.background_low_qos_cases.is_empty()
        {
            self.high_qos_vacant(&mut qos_changes, Background);
        }

        let mut changes_filter = HashMap::new();
        for change in qos_changes.into_iter() {
            if let std::collections::hash_map::Entry::Vacant(e) =
                changes_filter.entry(change.task_id)
            {
                e.insert(change.new_qos);
            } else {
                changes_filter.remove(&change.task_id);
            }
        }
        changes_filter
            .into_iter()
            .map(|change| QosChange::new(change.0, change.1))
            .collect()
    }

    fn state_turn_to_terminated(
        &mut self,
        uid: u64,
        old_state: ApplicationState,
    ) -> Vec<QosChange> {
        let mut qos_changes = vec![];

        self.app_state_map.remove(&uid);
        self.app_high_qos_count.remove(&uid);

        let (high_qos_cases, low_qos_cases) = match old_state {
            Foreground => (
                &mut self.foreground_high_qos_cases,
                &mut self.foreground_low_qos_cases,
            ),
            Background => (
                &mut self.background_high_qos_cases,
                &mut self.background_low_qos_cases,
            ),
            Terminated => unreachable!(),
        };

        // remove
        high_qos_cases
            .iter()
            .filter(|case| case.uid == uid)
            .for_each(|case| {
                self.tasks.remove(&case.task_id);
            });

        high_qos_cases.retain(|case| case.uid != uid);

        if let Some(remove_tasks) = low_qos_cases.remove(&uid) {
            remove_tasks.cases.into_iter().for_each(|case| {
                self.tasks.remove(&case.task_id);
            })
        }

        // vacant
        if self.foreground_high_qos_cases.len() < self.high_qos_max {
            self.high_qos_vacant(&mut qos_changes, Foreground);
        }
        if self.foreground_high_qos_cases.len() + self.background_high_qos_cases.len()
            < self.high_qos_max
        {
            self.high_qos_vacant(&mut qos_changes, Background);
        }

        qos_changes
    }

    fn high_qos_vacant(&mut self, qos_changes: &mut Vec<QosChange>, state: ApplicationState) {
        let mut high_count =
            self.foreground_high_qos_cases.len() + self.background_high_qos_cases.len();

        let mut v = match state {
            Foreground => {
                debug!("Qos foreground high qos vacant");
                self.foreground_low_qos_cases.iter_mut().collect::<Vec<_>>()
            }
            Background => {
                debug!("Qos background high qos vacant");
                self.background_low_qos_cases.iter_mut().collect::<Vec<_>>()
            }
            Terminated => unreachable!(),
        };

        if v.is_empty() {
            return;
        }

        v.sort_by(|me, other| {
            let res = self
                .app_high_qos_count
                .get(me.0)
                .unwrap()
                .cmp(self.app_high_qos_count.get(other.0).unwrap());
            if res == Ordering::Equal {
                other.1.len().cmp(&me.1.len())
            } else {
                res
            }
        });

        let mut empty_uid = vec![];
        // push the tasks in map
        while !v.is_empty() && high_count < self.high_qos_max {
            for (_, cases) in v.iter_mut() {
                if high_count >= self.high_qos_max {
                    break;
                }
                if let Some(mut case) = cases.pop_highest_qos() {
                    change_qos(
                        &mut self.app_high_qos_count,
                        qos_changes,
                        &mut case,
                        Qos::High,
                    );
                    high_count += 1;
                    debug!("Qos task {} push to {:?} High Qos", case.task_id, state);
                    match state {
                        Foreground => {
                            self.foreground_high_qos_cases.push(case);
                        }
                        Background => {
                            self.background_high_qos_cases.push(case);
                        }
                        Terminated => unreachable!(),
                    }
                } else {
                    break;
                }
            }
            for i in 0..v.len() {
                if v[i].1.is_empty() {
                    empty_uid.push(*v[i].0);
                    v.remove(i);
                    break;
                }
            }
        }

        match state {
            Foreground => {
                for uid in empty_uid.iter() {
                    self.foreground_low_qos_cases.remove(uid);
                }
            }
            Background => {
                for uid in empty_uid.iter() {
                    self.background_low_qos_cases.remove(uid);
                }
            }
            Terminated => unreachable!(),
        }
    }
}

fn change_qos(
    app_high_qos_count: &mut HashMap<u64, usize>,
    qos_changes: &mut Vec<QosChange>,
    case: &mut QosCase,
    new_qos: Qos,
) {
    match case.qos.take() {
        Some(old_qos) => {
            if old_qos == new_qos {
                error!(
                    "Qos change task {} qos {:?} with the same",
                    case.task_id, old_qos
                );
                return;
            }
            let count = app_high_qos_count.get_mut(&case.uid).unwrap();
            match new_qos {
                Qos::High => *count += 1,
                Qos::Low => *count -= 1,
            }

            case.qos = Some(new_qos.clone());
            debug!("Qos task {} change to {:?} Qos", case.task_id, new_qos);
            qos_changes.push(QosChange::new(case.task_id, new_qos));
        }
        None => {
            if new_qos == Qos::High {
                let count = app_high_qos_count.get_mut(&case.uid).unwrap();
                *count += 1;
            }
            case.qos = Some(new_qos.clone());
            debug!("Qos task {} change to {:?} Qos", case.task_id, new_qos);
            qos_changes.push(QosChange::new(case.task_id, new_qos));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // QosQueue Situation
    // FrontHigh FrontLow BackHigh BackLow situation_index
    //    N         N        N        N         0
    //    N         N        Y        N         2
    //    N         N        Y        Y         3
    //    Y         N        N        N         8
    //    Y         N        N        Y         9
    //    Y         N        Y        N         10
    //    Y         N        Y        Y         11
    //    Y         Y        N        N         12
    //    Y         Y        N        Y         13
    //    Y         Y        Y        N         14
    //    Y         Y        Y        Y         15

    // Impossible QosQueue Situation
    // FrontHigh FrontLow BackHigh BackLow situation_index
    //    N         N        N        Y         1
    //    N         Y        N        N         4
    //    N         Y        N        Y         5
    //    N         Y        Y        N         6
    //    N         Y        Y        Y         7

    fn check_empty(queue: &QosQueue) {
        assert!(queue.foreground_high_qos_cases.is_empty());
        assert!(queue.foreground_low_qos_cases.is_empty());
        assert!(queue.background_high_qos_cases.is_empty());
        assert!(queue.background_low_qos_cases.is_empty());
        assert!(queue.tasks.is_empty());
        assert!(queue.app_high_qos_count.is_empty());
        assert!(queue.app_state_map.is_empty());
    }
    #[test]
    fn ut_qos_init_test() {
        #[cfg(not(feature = "oh"))]
        crate::test_set_up();
        let mode = Mode::FrontEnd;

        let mut queue = QosQueue::new(4);
        let case = QosCase::new(0, 0, mode, 0);
        queue.insert_inner(case, Foreground);
        assert_eq!(queue.app_state_map.get(&0).unwrap(), &Foreground);
        assert_eq!(queue.app_high_qos_count.get(&0).unwrap(), &1);
        queue.remove(0, 0);
        check_empty(&queue);

        let case = QosCase::new(0, 0, mode, 0);
        queue.insert_inner(case, Background);
        assert_eq!(queue.app_state_map.get(&0).unwrap(), &Background);
        assert_eq!(queue.app_high_qos_count.get(&0).unwrap(), &1);
        queue.remove(0, 0);
    }

    #[test]
    fn ut_qos_test_basic() {
        #[cfg(not(feature = "oh"))]
        crate::test_set_up();
        let mode = Mode::FrontEnd;

        let mut queue = QosQueue::new(4);
        let case = QosCase::new(0, 0, mode, 0);

        // Foreground insert then change to background and remove.
        let qos_changes = queue.insert_inner(case.clone(), Foreground);
        assert_eq!(qos_changes[0].task_id, 0);
        assert_eq!(qos_changes[0].new_qos, Qos::High);
        assert_eq!(queue.foreground_high_qos_cases[0].task_id, case.task_id);
        assert_eq!(queue.foreground_high_qos_cases[0].qos, Some(Qos::High));

        let qos_changes = queue.change_state(0, Background);
        assert!(qos_changes.is_empty());
        assert!(queue.foreground_high_qos_cases.is_empty());
        assert_eq!(queue.background_high_qos_cases.len(), 1);

        let qos_changes = queue.remove(case.uid, case.task_id);
        assert!(qos_changes.is_empty());

        check_empty(&queue);

        // Background insert and remove.
        let qos_changes = queue.insert_inner(case.clone(), Background);
        assert_eq!(qos_changes[0].task_id, 0);
        assert_eq!(qos_changes[0].new_qos, Qos::High);
        assert_eq!(queue.background_high_qos_cases[0].task_id, case.task_id);
        assert_eq!(queue.background_high_qos_cases[0].qos, Some(Qos::High));
        let qos_changes = queue.remove(case.uid, case.task_id);
        assert!(qos_changes.is_empty());

        check_empty(&queue);

        // Foreground insert then change to terminated
        queue.insert_inner(case, Foreground);
        let qos_changes = queue.change_state(0, Terminated);
        assert!(qos_changes.is_empty());

        check_empty(&queue);
    }

    #[test]
    fn ut_qos_test_map_basic() {
        #[cfg(not(feature = "oh"))]
        crate::test_set_up();
        let mode = Mode::FrontEnd;

        let mut queue = QosQueue::new(1);
        let case_0 = QosCase::new(0, 0, mode, 0);
        let case_1 = QosCase::new(0, 1, mode, 1);

        // insert to foreground low qos map then remove all
        queue.insert_inner(case_0.clone(), Foreground);
        let qos_changes = queue.insert_inner(case_1.clone(), Foreground);
        assert_eq!(queue.foreground_low_qos_cases.get(&0).unwrap().len(), 1);
        assert_eq!(qos_changes[0].task_id, 1);
        assert_eq!(qos_changes[0].new_qos, Qos::Low);
        let qos_changes = queue.remove(0, 0);
        assert_eq!(queue.foreground_high_qos_cases.len(), 1);
        assert_eq!(qos_changes[0].task_id, 1);
        assert_eq!(qos_changes[0].new_qos, Qos::High);
        queue.remove(0, 1);

        check_empty(&queue);

        // insert to background low qos map then remove all
        queue.insert_inner(case_0.clone(), Background);
        let qos_changes = queue.insert_inner(case_1.clone(), Background);
        assert_eq!(queue.background_low_qos_cases.get(&0).unwrap().len(), 1);
        assert_eq!(qos_changes[0].task_id, 1);
        assert_eq!(qos_changes[0].new_qos, Qos::Low);
        let qos_changes = queue.remove(0, 0);
        assert_eq!(queue.background_high_qos_cases.len(), 1);
        assert_eq!(qos_changes[0].task_id, 1);
        assert_eq!(qos_changes[0].new_qos, Qos::High);
        queue.remove(0, 1);

        check_empty(&queue);

        // insert to background low qos map then change state
        queue.insert_inner(case_0.clone(), Background);
        queue.insert_inner(case_1.clone(), Background);
        let qos_changes = queue.change_state(0, Foreground);
        assert!(qos_changes.is_empty());
        queue.change_state(0, Terminated);
        assert!(qos_changes.is_empty());

        check_empty(&queue);

        // insert to background low qos map then change state
        queue.insert_inner(case_0, Foreground);
        queue.insert_inner(case_1, Foreground);
        let qos_changes = queue.change_state(0, Background);
        assert!(qos_changes.is_empty());
        queue.change_state(0, Terminated);
        assert!(qos_changes.is_empty());

        check_empty(&queue);
    }

    #[test]
    fn ut_qos_test_vacant() {
        #[cfg(not(feature = "oh"))]
        crate::test_set_up();

        let mode = Mode::FrontEnd;
        let mut queue = QosQueue::new(3);
        let case_0_0 = QosCase::new(0, 0, mode, 0);
        let case_0_1 = QosCase::new(0, 1, mode, 1);
        let case_0_2 = QosCase::new(0, 2, mode, 2);
        let case_0_3 = QosCase::new(0, 3, mode, 3);
        let case_1_4 = QosCase::new(1, 4, mode, 0);
        let case_1_5 = QosCase::new(1, 5, mode, 1);
        let case_1_6 = QosCase::new(1, 6, mode, 2);
        queue.insert_inner(case_0_0, Foreground);
        queue.insert_inner(case_0_1, Foreground);
        queue.insert_inner(case_0_2, Foreground);
        queue.insert_inner(case_0_3, Foreground);
        assert_eq!(*queue.app_high_qos_count.get(&0).unwrap(), 3);

        queue.insert_inner(case_1_4, Background);
        queue.insert_inner(case_1_5, Background);
        queue.insert_inner(case_1_6, Background);
        assert_eq!(*queue.app_high_qos_count.get(&1).unwrap(), 0);
        let qos_changes = queue.change_state(1, Foreground);
        assert_eq!(qos_changes[0].task_id, 2);
        assert_eq!(qos_changes[0].new_qos, Qos::Low);
        assert_eq!(qos_changes[1].task_id, 4);
        assert_eq!(qos_changes[1].new_qos, Qos::High);

        let qos_changes = queue.change_state(0, Background);
        assert_eq!(qos_changes.len(), 4);
        assert!(qos_changes.contains(&QosChange {
            task_id: 5,
            new_qos: Qos::High
        }));

        assert!(qos_changes.contains(&QosChange {
            task_id: 6,
            new_qos: Qos::High
        }));

        assert!(qos_changes.contains(&QosChange {
            task_id: 0,
            new_qos: Qos::Low
        }));
        assert!(qos_changes.contains(&QosChange {
            task_id: 1,
            new_qos: Qos::Low
        }));

        let qos_changes = queue.change_state(1, Background);
        assert_eq!(qos_changes.len(), 4);
        assert!(qos_changes.contains(&QosChange {
            task_id: 0,
            new_qos: Qos::High
        }));
        assert!(qos_changes.contains(&QosChange {
            task_id: 1,
            new_qos: Qos::High
        }));
        assert!(qos_changes.contains(&QosChange {
            task_id: 5,
            new_qos: Qos::Low
        }));
        assert!(qos_changes.contains(&QosChange {
            task_id: 6,
            new_qos: Qos::Low
        }));

        let qos_changes = queue.change_state(1, Terminated);
        assert_eq!(qos_changes.len(), 1);
        assert!(qos_changes.contains(&QosChange {
            task_id: 2,
            new_qos: Qos::High
        }));
        queue.change_state(0, Terminated);
        check_empty(&queue)
    }

    #[test]
    fn ut_test_pop_high() {
        #[cfg(not(feature = "oh"))]
        crate::test_set_up();

        let states = [Foreground, Background];

        for state in states {
            let mut queue = QosQueue::new(6);

            let mode = Mode::FrontEnd;

            let case_0_0 = QosCase::new(0, 0, mode, 0);
            let case_2_5 = QosCase::new(2, 5, mode, 0);
            let case_0_1 = QosCase::new(0, 1, Mode::BackGround, 1);
            let case_0_2 = QosCase::new(0, 2, mode, 2);
            let case_1_3 = QosCase::new(1, 3, mode, 0);
            let case_1_4 = QosCase::new(1, 4, mode, 0);
            queue.insert_inner(case_0_0, state);
            queue.insert_inner(case_2_5, state);
            queue.insert_inner(case_0_1, state);
            queue.insert_inner(case_0_2, state);
            queue.insert_inner(case_1_3, state);
            queue.insert_inner(case_1_4, state);

            assert_eq!(queue.pop_high_qos(state).unwrap().task_id, 1);
            *queue.app_high_qos_count.get_mut(&0).unwrap() -= 1;
            assert!(
                queue.pop_high_qos(state).unwrap().task_id == 2
                    || queue.pop_high_qos(state).unwrap().task_id == 3
            );
        }
    }

    #[test]
    fn ut_test_contest() {
        #[cfg(not(feature = "oh"))]
        crate::test_set_up();
        let mut queue = QosQueue::new(2);

        let mode = Mode::FrontEnd;

        let case_0_0 = QosCase::new(0, 0, mode, 0);
        let case_0_1 = QosCase::new(0, 1, mode, 1);
        let case_0_2 = QosCase::new(0, 2, mode, 2);
        let case_1_3 = QosCase::new(1, 3, mode, 0);
        let case_2_4 = QosCase::new(2, 4, mode, 0);

        queue.insert_inner(case_0_2, Foreground);
        queue.insert_inner(case_1_3, Background);
        let qos_changes = queue.insert_inner(case_0_1, Foreground);
        assert_eq!(qos_changes.len(), 2);
        assert_eq!(qos_changes[0].task_id, 1);
        assert_eq!(qos_changes[0].new_qos, Qos::High);
        assert_eq!(qos_changes[1].task_id, 3);
        assert_eq!(qos_changes[1].new_qos, Qos::Low);

        let qos_changes = queue.insert_inner(case_0_0, Foreground);
        assert_eq!(qos_changes.len(), 2);
        assert_eq!(qos_changes[0].task_id, 0);
        assert_eq!(qos_changes[0].new_qos, Qos::High);
        assert_eq!(qos_changes[1].task_id, 2);
        assert_eq!(qos_changes[1].new_qos, Qos::Low);

        let qos_changes = queue.insert_inner(case_2_4, Foreground);
        assert_eq!(qos_changes.len(), 2);
        assert_eq!(qos_changes[0].task_id, 1);
        assert_eq!(qos_changes[0].new_qos, Qos::Low);
        assert_eq!(qos_changes[1].task_id, 4);
        assert_eq!(qos_changes[1].new_qos, Qos::High);
        queue.remove(0, 0);
        queue.remove(0, 1);
        queue.remove(0, 2);
        queue.remove(1, 3);
        queue.remove(2, 4);
        check_empty(&queue);
    }

    #[test]
    fn ut_qos_mode_ord_test() {
        assert!(Mode::FrontEnd < Mode::Any);
        assert!(Mode::Any < Mode::BackGround);
    }
}
