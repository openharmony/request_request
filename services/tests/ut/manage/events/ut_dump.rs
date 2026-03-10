// Copyright (C) 2025 Huawei Device Co., Ltd.
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

use std::sync::{Arc, Mutex};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Download = 0,
    Upload,
    Any,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Initialized = 0x00,
    Waiting = 0x10,
    Running = 0x20,
    Retrying = 0x21,
    Paused = 0x30,
    Stopped = 0x31,
    Completed = 0x40,
    Failed = 0x41,
    Removed = 0x50,
    Any = 0x61,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reason {
    Default = 0,
    TaskSurvivalOneMonth = 1,
    RunningTaskMeetLimits = 4,
    UserOperation = 5,
    AppBackgroundOrTerminate = 6,
}

struct TaskStatus {
    state: State,
    reason: Reason,
}

struct TaskConfig {
    task_id: u32,
    action: Action,
}

struct Task {
    conf: TaskConfig,
    status: Arc<Mutex<TaskStatus>>,
}

impl Task {
    fn task_id(&self) -> u32 {
        self.conf.task_id
    }
}

struct DumpOneInfo {
    task_id: u32,
    action: Action,
    state: State,
    reason: Reason,
}

struct DumpAllEachInfo {
    task_id: u32,
    action: Action,
    state: State,
    reason: Reason,
}

struct DumpAllInfo {
    vec: Vec<DumpAllEachInfo>,
}

struct Scheduler {
    tasks: Vec<Task>,
}

impl Scheduler {
    fn tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.iter()
    }
}

struct TaskManager {
    scheduler: Scheduler,
}

impl TaskManager {
    fn query_one_task(&self, task_id: u32) -> Option<DumpOneInfo> {
        self.scheduler
            .tasks()
            .find(|task| task.task_id() == task_id)
            .map(|task| {
                let status = task
                    .status
                    .lock()
                    .expect("Failed to lock task status in query_one_task");
                DumpOneInfo {
                    task_id: task.conf.task_id,
                    action: task.conf.action,
                    state: status.state,
                    reason: status.reason,
                }
            })
    }

    fn query_all_task(&self) -> DumpAllInfo {
        DumpAllInfo {
            vec: self
                .scheduler
                .tasks()
                .map(|task| {
                    let status = task
                        .status
                        .lock()
                        .expect("Failed to lock task status in query_all_task");
                    DumpAllEachInfo {
                        task_id: task.conf.task_id,
                        action: task.conf.action,
                        state: status.state,
                        reason: status.reason,
                    }
                })
                .collect(),
        }
    }
}

fn create_task(task_id: u32, action: Action, state: State, reason: Reason) -> Task {
    Task {
        conf: TaskConfig { task_id, action },
        status: Arc::new(Mutex::new(TaskStatus { state, reason })),
    }
}

// @tc.name: ut_dump_query_one_task_found
// @tc.desc: Test query_one_task returns correct info for existing task
// @tc.precon: NA
// @tc.step: 1. Create TaskManager with tasks
//           2. Query existing task by id
//           3. Verify returned DumpOneInfo
// @tc.expect: Correct DumpOneInfo is returned
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_dump_query_one_task_found() {
    let manager = TaskManager {
        scheduler: Scheduler {
            tasks: vec![
                create_task(1, Action::Download, State::Running, Reason::Default),
                create_task(2, Action::Upload, State::Waiting, Reason::Default),
            ],
        },
    };

    let result = manager.query_one_task(1);
    assert!(result.is_some());
    let info = result.unwrap();
    assert_eq!(info.task_id, 1);
    assert_eq!(info.action, Action::Download);
    assert_eq!(info.state, State::Running);
    assert_eq!(info.reason, Reason::Default);
}

// @tc.name: ut_dump_query_one_task_not_found
// @tc.desc: Test query_one_task returns None for non-existent task
// @tc.precon: NA
// @tc.step: 1. Create TaskManager with tasks
//           2. Query non-existent task id
// @tc.expect: None is returned
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_dump_query_one_task_not_found() {
    let manager = TaskManager {
        scheduler: Scheduler {
            tasks: vec![create_task(1, Action::Download, State::Running, Reason::Default)],
        },
    };

    let result = manager.query_one_task(999);
    assert!(result.is_none());
}

// @tc.name: ut_dump_query_all_task
// @tc.desc: Test query_all_task returns all task info
// @tc.precon: NA
// @tc.step: 1. Create TaskManager with multiple tasks
//           2. Query all tasks
//           3. Verify all tasks are included
// @tc.expect: All task info is returned
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_dump_query_all_task() {
    let manager = TaskManager {
        scheduler: Scheduler {
            tasks: vec![
                create_task(1, Action::Download, State::Running, Reason::Default),
                create_task(2, Action::Upload, State::Completed, Reason::UserOperation),
            ],
        },
    };

    let result = manager.query_all_task();
    assert_eq!(result.vec.len(), 2);
    assert_eq!(result.vec[0].task_id, 1);
    assert_eq!(result.vec[1].task_id, 2);
}

// @tc.name: ut_dump_state_repr_values
// @tc.desc: Test State enum repr values match source
// @tc.precon: NA
// @tc.step: 1. Check State discriminant values
// @tc.expect: State repr values match source code
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_dump_state_repr_values() {
    assert_eq!(State::Initialized as u8, 0x00);
    assert_eq!(State::Running as u8, 0x20);
    assert_eq!(State::Completed as u8, 0x40);
    assert_eq!(State::Failed as u8, 0x41);
}

// @tc.name: ut_dump_action_repr_values
// @tc.desc: Test Action enum repr values match source
// @tc.precon: NA
// @tc.step: 1. Check Action discriminant values
// @tc.expect: Action repr values match source code
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_dump_action_repr_values() {
    assert_eq!(Action::Download as u8, 0);
    assert_eq!(Action::Upload as u8, 1);
}

// @tc.name: ut_dump_reason_repr_values
// @tc.desc: Test Reason enum repr values match source
// @tc.precon: NA
// @tc.step: 1. Check Reason discriminant values
// @tc.expect: Reason repr values match source code
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_dump_reason_repr_values() {
    assert_eq!(Reason::Default as i32, 0);
    assert_eq!(Reason::UserOperation as i32, 5);
}
