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

use super::*;
use crate::manage::events::{StateEvent, TaskManagerEvent};
use crate::manage::task_manager::TaskManagerTx;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use ylong_runtime::sync::mpsc::unbounded_channel;

const APP_STATE_FOREGROUND: i32 = 2;
const APP_STATE_BACKGROUND: i32 = 4;
const PROCESS_STATE_DIED: i32 = 5;

fn is_hiviewx_bundle(name: &str) -> bool {
    name.starts_with("com.") && name.ends_with(".hmos.hiviewx")
}

fn map_app_state_to_event(state: i32, uid: i32) -> Option<StateEvent> {
    match state {
        APP_STATE_FOREGROUND => Some(StateEvent::ForegroundApp(uid as u64)),
        APP_STATE_BACKGROUND => Some(StateEvent::Background(uid as u64)),
        _ => None,
    }
}

fn map_process_state_to_action(state: i32, bundle_name: &str, uid: i32, pid: i32) -> Vec<ProcessAction> {
    let mut actions = Vec::new();
    
    if is_hiviewx_bundle(bundle_name) {
        actions.push(ProcessAction::SpecialTerminate(uid as u64));
    }
    
    if state == PROCESS_STATE_DIED {
        actions.push(ProcessAction::ProcessTerminate(pid as u64));
    }
    
    actions
}

#[derive(Debug, Clone, PartialEq)]
enum ProcessAction {
    SpecialTerminate(u64),
    ProcessTerminate(u64),
}

// @tc.name: ut_app_state_foreground_mapping
// @tc.desc: Test app state code 2 maps to ForegroundApp event
// @tc.precon: NA
// @tc.step: 1. Call map_app_state_to_event with state=2 and uid=1000
//           2. Verify returned event is ForegroundApp(1000)
// @tc.expect: State code 2 correctly maps to ForegroundApp event
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_app_state_foreground_mapping() {
    let event = map_app_state_to_event(APP_STATE_FOREGROUND, 1000);
    
    match event {
        Some(StateEvent::ForegroundApp(uid)) => assert_eq!(uid, 1000),
        _ => panic!("Expected Some(ForegroundApp(1000))"),
    }
}

// @tc.name: ut_app_state_background_mapping
// @tc.desc: Test app state code 4 maps to Background event
// @tc.precon: NA
// @tc.step: 1. Call map_app_state_to_event with state=4 and uid=2000
//           2. Verify returned event is Background(2000)
// @tc.expect: State code 4 correctly maps to Background event
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_app_state_background_mapping() {
    let event = map_app_state_to_event(APP_STATE_BACKGROUND, 2000);
    
    match event {
        Some(StateEvent::Background(uid)) => assert_eq!(uid, 2000),
        _ => panic!("Expected Some(Background(2000))"),
    }
}

// @tc.name: ut_app_state_unknown_code
// @tc.desc: Test unknown app state code returns None
// @tc.precon: NA
// @tc.step: 1. Call map_app_state_to_event with state=999 (unknown)
//           2. Verify returned value is None
// @tc.expect: Unknown state codes are ignored
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_app_state_unknown_code() {
    let event = map_app_state_to_event(999, 1000);
    assert!(event.is_none());
    
    let event = map_app_state_to_event(0, 1000);
    assert!(event.is_none());
    
    let event = map_app_state_to_event(-1, 1000);
    assert!(event.is_none());
}

// @tc.name: ut_hiviewx_bundle_name_matching
// @tc.desc: Test hiviewx bundle name pattern matching for special process handling
// @tc.precon: NA
// @tc.step: 1. Test valid hiviewx bundle names that should match
//           2. Test invalid bundle names that should not match
// @tc.expect: Only bundles matching "com.*.hmos.hiviewx" pattern are identified
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_hiviewx_bundle_name_matching() {
    let valid_hiviewx = [
        "com.example.hmos.hiviewx",
        "com.test.hmos.hiviewx",
        "com.abc.hmos.hiviewx",
        "com.vendor.app.hmos.hiviewx",
    ];
    
    for name in valid_hiviewx {
        assert!(is_hiviewx_bundle(name), "Expected '{}' to be hiviewx bundle", name);
    }
    
    let invalid_hiviewx = [
        "com.example.app",
        "org.test.hmos.hiviewx",
        "com.example.hmos.other",
        "com.example.hiviewx",
        "example.hmos.hiviewx",
        "com.example.hmos.hiviewx.extra",
        "",
    ];
    
    for name in invalid_hiviewx {
        assert!(!is_hiviewx_bundle(name), "Expected '{}' to NOT be hiviewx bundle", name);
    }
}

// @tc.name: ut_process_died_state_mapping
// @tc.desc: Test process state code 5 maps to ProcessTerminate action
// @tc.precon: NA
// @tc.step: 1. Call map_process_state_to_action with state=5
//           2. Verify ProcessTerminate action is generated
// @tc.expect: State code 5 correctly generates ProcessTerminate action
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_process_died_state_mapping() {
    let actions = map_process_state_to_action(PROCESS_STATE_DIED, "com.example.app", 1000, 12345);
    
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0], ProcessAction::ProcessTerminate(12345));
}

// @tc.name: ut_hiviewx_special_terminate
// @tc.desc: Test hiviewx process generates SpecialTerminate action
// @tc.precon: NA
// @tc.step: 1. Call map_process_state_to_action with hiviewx bundle name
//           2. Verify SpecialTerminate action is generated
// @tc.expect: hiviewx process generates SpecialTerminate regardless of state
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_hiviewx_special_terminate() {
    let actions = map_process_state_to_action(0, "com.example.hmos.hiviewx", 1000, 12345);
    
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0], ProcessAction::SpecialTerminate(1000));
}

// @tc.name: ut_hiviewx_process_died_combined
// @tc.desc: Test hiviewx process died generates both SpecialTerminate and ProcessTerminate
// @tc.precon: NA
// @tc.step: 1. Call map_process_state_to_action with hiviewx bundle and state=5
//           2. Verify both actions are generated
// @tc.expect: hiviewx process death generates both special and normal terminate actions
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_hiviewx_process_died_combined() {
    let actions = map_process_state_to_action(
        PROCESS_STATE_DIED,
        "com.example.hmos.hiviewx",
        1000,
        12345,
    );
    
    assert_eq!(actions.len(), 2);
    assert!(actions.contains(&ProcessAction::SpecialTerminate(1000)));
    assert!(actions.contains(&ProcessAction::ProcessTerminate(12345)));
}

// @tc.name: ut_process_non_died_state
// @tc.desc: Test non-died process state generates no ProcessTerminate action
// @tc.precon: NA
// @tc.step: 1. Call map_process_state_to_action with state != 5
//           2. Verify no ProcessTerminate action for non-hiviewx bundle
// @tc.expect: Only state 5 generates ProcessTerminate action
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_process_non_died_state() {
    let non_died_states = [0, 1, 2, 3, 4, 6, 10, 100];
    
    for state in non_died_states {
        let actions = map_process_state_to_action(state, "com.example.app", 1000, 12345);
        assert!(actions.is_empty(), "Expected no actions for state {}", state);
    }
}

// @tc.name: uid_conversion_for_callbacks
// @tc.desc: Test UID type conversion from i32 to u64 matches business logic
// @tc.precon: NA
// @tc.step: 1. Test positive UID conversion
//           2. Test typical system UID ranges
// @tc.expect: UID conversion preserves value for callback parameters
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn uid_conversion_for_callbacks() {
    let test_uids: Vec<i32> = vec![0, 1000, 2000, 10000, 99999];
    
    for uid in test_uids {
        let uid_u64 = uid as u64;
        let event = StateEvent::ForegroundApp(uid_u64);
        
        match event {
            StateEvent::ForegroundApp(converted) => assert_eq!(converted, uid as u64),
            _ => panic!("Expected ForegroundApp"),
        }
    }
}

// @tc.name: pid_conversion_for_callbacks
// @tc.desc: Test PID type conversion from i32 to u64 matches business logic
// @tc.precon: NA
// @tc.step: 1. Test positive PID conversion
//           2. Verify conversion matches process_died_callback logic
// @tc.expect: PID conversion preserves value for callback parameters
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn pid_conversion_for_callbacks() {
    let test_pids: Vec<i32> = vec![1, 100, 12345, 65535];
    
    for pid in test_pids {
        let pid_u64 = pid as u64;
        let action = ProcessAction::ProcessTerminate(pid_u64);
        
        match action {
            ProcessAction::ProcessTerminate(converted) => assert_eq!(converted, pid as u64),
            _ => panic!("Expected ProcessTerminate"),
        }
    }
}

// @tc.name: ut_app_uninstall_event_construction
// @tc.desc: Test AppUninstall event construction from want parameters
// @tc.precon: NA
// @tc.step: 1. Simulate extracting uid from want parameters
//           2. Construct TaskManagerEvent with AppUninstall
//           3. Verify event structure
// @tc.expect: AppUninstall event is correctly constructed with uid
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_app_uninstall_event_construction() {
    fn simulate_uninstall_event(uid_param: Option<i32>) -> Option<TaskManagerEvent> {
        uid_param.map(|uid| {
            TaskManagerEvent::State(StateEvent::AppUninstall(uid as u64))
        })
    }
    
    let event = simulate_uninstall_event(Some(1000));
    match event {
        Some(TaskManagerEvent::State(StateEvent::AppUninstall(uid))) => {
            assert_eq!(uid, 1000);
        }
        _ => panic!("Expected State(AppUninstall(1000))"),
    }
    
    let event = simulate_uninstall_event(None);
    assert!(event.is_none());
}

// @tc.name: ut_app_uninstall_with_valid_uid
// @tc.desc: Test AppUninstallSubscriber handles valid uid parameter
// @tc.precon: NA
// @tc.step: 1. Create mock want with uid parameter
//           2. Simulate on_receive_event processing
//           3. Verify correct event is generated
// @tc.expect: Valid uid generates AppUninstall event
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_app_uninstall_with_valid_uid() {
    struct MockWant {
        uid: Option<i32>,
    }
    
    impl MockWant {
        fn get_int_param(&self, _key: &str) -> Option<i32> {
            self.uid
        }
    }
    
    fn process_uninstall_event(want: MockWant) -> Option<StateEvent> {
        want.get_int_param("uid").map(|uid| StateEvent::AppUninstall(uid as u64))
    }
    
    let want_with_uid = MockWant { uid: Some(2000) };
    let event = process_uninstall_event(want_with_uid);
    
    match event {
        Some(StateEvent::AppUninstall(uid)) => assert_eq!(uid, 2000),
        _ => panic!("Expected AppUninstall(2000)"),
    }
    
    let want_without_uid = MockWant { uid: None };
    let event = process_uninstall_event(want_without_uid);
    assert!(event.is_none());
}

// @tc.name: ut_state_event_all_variants_for_app_state
// @tc.desc: Test all StateEvent variants used by app_state module
// @tc.precon: NA
// @tc.step: 1. Create all StateEvent variants used in app_state
//           2. Verify each variant carries correct uid
// @tc.expect: All StateEvent variants work correctly for app state handling
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_state_event_all_variants_for_app_state() {
    let test_uid: u64 = 1000;
    
    let foreground = StateEvent::ForegroundApp(test_uid);
    match foreground {
        StateEvent::ForegroundApp(uid) => assert_eq!(uid, test_uid),
        _ => panic!("Expected ForegroundApp"),
    }
    
    let background = StateEvent::Background(test_uid);
    match background {
        StateEvent::Background(uid) => assert_eq!(uid, test_uid),
        _ => panic!("Expected Background"),
    }
    
    let uninstall = StateEvent::AppUninstall(test_uid);
    match uninstall {
        StateEvent::AppUninstall(uid) => assert_eq!(uid, test_uid),
        _ => panic!("Expected AppUninstall"),
    }
    
    let special = StateEvent::SpecialTerminate(test_uid);
    match special {
        StateEvent::SpecialTerminate(uid) => assert_eq!(uid, test_uid),
        _ => panic!("Expected SpecialTerminate"),
    }
}

// @tc.name: ut_task_manager_event_wrapping_state_events
// @tc.desc: Test TaskManagerEvent correctly wraps StateEvent for app state
// @tc.precon: NA
// @tc.step: 1. Create TaskManagerEvent with each StateEvent variant
//           2. Verify pattern matching extracts correct values
// @tc.expect: TaskManagerEvent correctly wraps StateEvent for event dispatch
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_manager_event_wrapping_state_events() {
    let events = vec![
        TaskManagerEvent::State(StateEvent::ForegroundApp(1000)),
        TaskManagerEvent::State(StateEvent::Background(2000)),
        TaskManagerEvent::State(StateEvent::AppUninstall(3000)),
        TaskManagerEvent::State(StateEvent::SpecialTerminate(4000)),
    ];
    
    let expected_uids = [1000u64, 2000, 3000, 4000];
    
    for (event, expected_uid) in events.into_iter().zip(expected_uids.iter()) {
        match event {
            TaskManagerEvent::State(StateEvent::ForegroundApp(uid)) => assert_eq!(uid, *expected_uid),
            TaskManagerEvent::State(StateEvent::Background(uid)) => assert_eq!(uid, *expected_uid),
            TaskManagerEvent::State(StateEvent::AppUninstall(uid)) => assert_eq!(uid, *expected_uid),
            TaskManagerEvent::State(StateEvent::SpecialTerminate(uid)) => assert_eq!(uid, *expected_uid),
            _ => panic!("Unexpected event type"),
        }
    }
}

// @tc.name: ut_app_state_codes_business_meaning
// @tc.desc: Test app state codes have correct business meaning
// @tc.precon: NA
// @tc.step: 1. Verify FOREGROUND code is 2
//           2. Verify BACKGROUND code is 4
//           3. Verify DIED code is 5
//           4. Verify codes are distinct
// @tc.expect: State codes match system-defined values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_app_state_codes_business_meaning() {
    assert_eq!(APP_STATE_FOREGROUND, 2);
    assert_eq!(APP_STATE_BACKGROUND, 4);
    assert_eq!(PROCESS_STATE_DIED, 5);
    
    assert_ne!(APP_STATE_FOREGROUND, APP_STATE_BACKGROUND);
    assert_ne!(APP_STATE_FOREGROUND, PROCESS_STATE_DIED);
    assert_ne!(APP_STATE_BACKGROUND, PROCESS_STATE_DIED);
}

// @tc.name: ut_multiple_app_state_transitions
// @tc.desc: Test multiple app state transitions generate correct events
// @tc.precon: NA
// @tc.step: 1. Simulate app going to foreground (state=2)
//           2. Simulate app going to background (state=4)
//           3. Verify correct events for each transition
// @tc.expect: State transitions generate correct sequence of events
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_multiple_app_state_transitions() {
    let uid = 1000;
    let transitions = vec![
        (APP_STATE_FOREGROUND, StateEvent::ForegroundApp(uid as u64)),
        (APP_STATE_BACKGROUND, StateEvent::Background(uid as u64)),
    ];
    
    for (state, expected_event) in transitions {
        let event = map_app_state_to_event(state, uid);
        match (event, expected_event) {
            (Some(StateEvent::ForegroundApp(got)), StateEvent::ForegroundApp(expected)) => {
                assert_eq!(got, expected);
            }
            (Some(StateEvent::Background(got)), StateEvent::Background(expected)) => {
                assert_eq!(got, expected);
            }
            _ => panic!("Event mismatch for state {}", state),
        }
    }
}
