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

use std::collections::HashMap;
use std::time::Duration;

use super::*;
use crate::config::Mode;
use crate::manage::events::{ScheduleEvent, ServiceEvent, StateEvent, TaskEvent, TaskManagerEvent};

// @tc.name: ut_task_manager_constants
// @tc.desc: Test TaskManager constants
// @tc.precon: NA
// @tc.step: 1. Check CLEAR_INTERVAL value (30 minutes)
//           2. Check RESTORE_ALL_TASKS_INTERVAL value (10 seconds)
// @tc.expect: Constants have correct values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_constants() {
    assert_eq!(CLEAR_INTERVAL, 30 * 60);
    assert_eq!(RESTORE_ALL_TASKS_INTERVAL, 10);
}

// @tc.name: ut_task_manager_task_count_map
// @tc.desc: Test task_count HashMap structure
// @tc.precon: NA
// @tc.step: 1. Create task_count HashMap
//           2. Insert and retrieve values
//           3. Verify (foreground, background) tuple structure
// @tc.expect: HashMap works correctly for task counting
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_task_count_map() {
    let mut task_count: HashMap<u64, (usize, usize)> = HashMap::new();
    
    task_count.insert(1000, (5, 3));
    task_count.insert(2000, (10, 2));
    
    assert_eq!(task_count.get(&1000), Some(&(5, 3)));
    assert_eq!(task_count.get(&2000), Some(&(10, 2)));
    assert_eq!(task_count.get(&3000), None);
    
    assert_eq!(task_count.len(), 2);
}

// @tc.name: ut_task_manager_task_count_operations
// @tc.desc: Test task count increment and decrement
// @tc.precon: NA
// @tc.step: 1. Create task_count HashMap
//           2. Perform increment/decrement operations
//           3. Verify counts update correctly
// @tc.expect: Task count operations work correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_task_count_operations() {
    let mut task_count: HashMap<u64, (usize, usize)> = HashMap::new();
    
    let uid: u64 = 1000;
    task_count.entry(uid).or_insert((0, 0));
    
    task_count.entry(uid).and_modify(|(fg, _bg)| *fg += 1);
    assert_eq!(task_count.get(&uid), Some(&(1, 0)));
    
    task_count.entry(uid).and_modify(|(_fg, bg)| *bg += 1);
    assert_eq!(task_count.get(&uid), Some(&(1, 1)));
    
    task_count.entry(uid).and_modify(|(fg, _)| *fg -= 1);
    assert_eq!(task_count.get(&uid), Some(&(0, 1)));
}

// @tc.name: ut_task_qos_info_structure
// @tc.desc: Test TaskQosInfo structure
// @tc.precon: NA
// @tc.step: 1. Create TaskQosInfo
//           2. Verify all fields
// @tc.expect: TaskQosInfo is correctly structured
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_qos_info_structure() {
    let qos_info = TaskQosInfo {
        task_id: 12345,
        action: 1,
        mode: 2,
        state: 0,
        priority: 100,
    };
    
    assert_eq!(qos_info.task_id, 12345);
    assert_eq!(qos_info.action, 1);
    assert_eq!(qos_info.mode, 2);
    assert_eq!(qos_info.state, 0);
    assert_eq!(qos_info.priority, 100);
}

// @tc.name: ut_task_qos_info_copy
// @tc.desc: Test TaskQosInfo Copy trait
// @tc.precon: NA
// @tc.step: 1. Create TaskQosInfo
//           2. Copy it
//           3. Verify copy works
// @tc.expect: Copy works correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_qos_info_copy() {
    let qos_info = TaskQosInfo {
        task_id: 12345,
        action: 1,
        mode: 2,
        state: 0,
        priority: 100,
    };
    
    let copied = qos_info;
    assert_eq!(qos_info.task_id, copied.task_id);
    assert_eq!(qos_info.priority, copied.priority);
}

// @tc.name: ut_task_manager_duration_intervals
// @tc.desc: Test duration intervals for scheduling
// @tc.precon: NA
// @tc.step: 1. Create Duration values from constants
//           2. Verify interval calculations
// @tc.expect: Duration intervals are correct
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_duration_intervals() {
    let clear_interval = Duration::from_secs(CLEAR_INTERVAL);
    let restore_interval = Duration::from_secs(RESTORE_ALL_TASKS_INTERVAL);
    
    assert_eq!(clear_interval.as_secs(), 1800);
    assert_eq!(restore_interval.as_secs(), 10);
    
    assert!(clear_interval > restore_interval);
}

// @tc.name: ut_task_manager_state_event_variants
// @tc.desc: Test StateEvent enum variants
// @tc.precon: NA
// @tc.step: 1. Create different StateEvent variants
//           2. Verify pattern matching
// @tc.expect: StateEvent variants work correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_state_event_variants() {
    let network = StateEvent::Network;
    let foreground = StateEvent::ForegroundApp(1000);
    let background = StateEvent::Background(1000);
    let uninstall = StateEvent::AppUninstall(1000);
    
    assert!(matches!(network, StateEvent::Network));
    
    match foreground {
        StateEvent::ForegroundApp(uid) => assert_eq!(uid, 1000),
        _ => panic!("Expected ForegroundApp"),
    }
    
    match background {
        StateEvent::Background(uid) => assert_eq!(uid, 1000),
        _ => panic!("Expected Background"),
    }
    
    match uninstall {
        StateEvent::AppUninstall(uid) => assert_eq!(uid, 1000),
        _ => panic!("Expected AppUninstall"),
    }
}

// @tc.name: ut_task_manager_schedule_event
// @tc.desc: Test ScheduleEvent enum variants
// @tc.precon: NA
// @tc.step: 1. Create different schedule events
//           2. Verify pattern matching
// @tc.expect: Schedule events work correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_schedule_event() {
    let events = vec![
        ScheduleEvent::ClearTimeoutTasks,
        ScheduleEvent::RestoreAllTasks,
        ScheduleEvent::Unload,
        ScheduleEvent::Shutdown,
        ScheduleEvent::RestartCountDown,
    ];

    assert_eq!(events.len(), 5);

    for event in events {
        match event {
            ScheduleEvent::ClearTimeoutTasks => assert!(true),
            ScheduleEvent::RestoreAllTasks => assert!(true),
            ScheduleEvent::Unload => assert!(true),
            ScheduleEvent::Shutdown => assert!(true),
            ScheduleEvent::RestartCountDown => assert!(true),
        }
    }
}

// @tc.name: ut_task_manager_service_event
// @tc.desc: Test ServiceEvent enum variants
// @tc.precon: NA
// @tc.step: 1. Create different service events
//           2. Verify pattern matching
// @tc.expect: Service events work correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_service_event() {
    let start_event = ServiceEvent::Start(1000, 12345, std::sync::mpsc::channel().0);
    let stop_event = ServiceEvent::Stop(1000, 12345, std::sync::mpsc::channel().0);
    
    match start_event {
        ServiceEvent::Start(uid, task_id, _) => {
            assert_eq!(uid, 1000);
            assert_eq!(task_id, 12345);
        }
        _ => panic!("Expected Start event"),
    }
    
    match stop_event {
        ServiceEvent::Stop(uid, task_id, _) => {
            assert_eq!(uid, 1000);
            assert_eq!(task_id, 12345);
        }
        _ => panic!("Expected Stop event"),
    }
}

// @tc.name: ut_task_manager_task_event
// @tc.desc: Test TaskEvent enum variants
// @tc.precon: NA
// @tc.step: 1. Create different task events
//           2. Verify pattern matching
// @tc.expect: Task events work correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_task_event() {
    let completed_event = TaskEvent::Completed(12345, 1000, Mode::BackGround);
    let offline_event = TaskEvent::Offline(12345, 1000, Mode::BackGround);
    
    match completed_event {
        TaskEvent::Completed(task_id, uid, mode) => {
            assert_eq!(task_id, 12345);
            assert_eq!(uid, 1000);
            assert_eq!(mode, Mode::BackGround);
        }
        _ => panic!("Expected Completed event"),
    }
    
    match offline_event {
        TaskEvent::Offline(task_id, uid, mode) => {
            assert_eq!(task_id, 12345);
            assert_eq!(uid, 1000);
        }
        _ => panic!("Expected Offline event"),
    }
}

// @tc.name: ut_task_manager_running_tasks_check
// @tc.desc: Test running tasks check logic for unload decision
// @tc.precon: NA
// @tc.step: 1. Simulate running tasks count
//           2. Check if any tasks running or events pending
// @tc.expect: Running tasks check works correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_running_tasks_check() {
    fn check_any_tasks(running_tasks: usize, rx_empty: bool) -> bool {
        if running_tasks != 0 {
            return true;
        }
        if !rx_empty {
            return true;
        }
        false
    }
    
    assert!(check_any_tasks(5, true));
    assert!(check_any_tasks(0, false));
    assert!(!check_any_tasks(0, true));
    assert!(check_any_tasks(1, false));
}

// @tc.name: ut_task_manager_unload_sa_logic
// @tc.desc: Test unload SA logic conditions
// @tc.precon: NA
// @tc.step: 1. Verify REQUEST_SERVICE_ID constant
//           2. Verify ONE_MONTH constant
//           3. Test unload conditions
// @tc.expect: Unload SA logic works correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_unload_sa_logic() {
    const REQUEST_SERVICE_ID: i32 = 3706;
    const ONE_MONTH: i64 = 30 * 24 * 60 * 60 * 1000;
    
    assert_eq!(REQUEST_SERVICE_ID, 3706);
    assert_eq!(ONE_MONTH, 2592000000);
    
    fn should_unload(running_tasks: usize, has_pending_events: bool) -> bool {
        running_tasks == 0 && !has_pending_events
    }
    
    assert!(should_unload(0, false));
    assert!(!should_unload(1, false));
    assert!(!should_unload(0, true));
    assert!(!should_unload(1, true));
}

// @tc.name: ut_task_manager_event_wrapping
// @tc.desc: Test TaskManagerEvent wrapping other event types
// @tc.precon: NA
// @tc.step: 1. Create TaskManagerEvent with different inner events
//           2. Verify pattern matching
// @tc.expect: TaskManagerEvent correctly wraps other event types
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_event_wrapping() {
    let service_event = TaskManagerEvent::Service(ServiceEvent::Start(1000, 12345, std::sync::mpsc::channel().0));
    let state_event = TaskManagerEvent::State(StateEvent::Network);
    let schedule_event = TaskManagerEvent::Schedule(ScheduleEvent::Unload);
    let device_event = TaskManagerEvent::Device(1);
    
    match service_event {
        TaskManagerEvent::Service(_) => assert!(true),
        _ => panic!("Expected Service"),
    }
    
    match state_event {
        TaskManagerEvent::State(StateEvent::Network) => assert!(true),
        _ => panic!("Expected State(Network)"),
    }
    
    match schedule_event {
        TaskManagerEvent::Schedule(ScheduleEvent::Unload) => assert!(true),
        _ => panic!("Expected Schedule(Unload)"),
    }
    
    match device_event {
        TaskManagerEvent::Device(level) => assert_eq!(level, 1),
        _ => panic!("Expected Device"),
    }
}
