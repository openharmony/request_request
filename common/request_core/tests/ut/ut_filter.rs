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

use request_core::filter::SearchFilter;
use request_core::config::{Action, Mode};
use request_core::info::State;

// @tc.name: ut_search_filter_new
// @tc.desc: Test SearchFilter creation with new()
// @tc.precon: NA
// @tc.step: 1. Create SearchFilter using new()
//           2. Verify all fields are None
// @tc.expect: All fields are None after creation
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_search_filter_new() {
    let filter = SearchFilter::new();
    
    assert!(filter.bundle_name.is_none());
    assert!(filter.before.is_none());
    assert!(filter.after.is_none());
    assert!(filter.state.is_none());
    assert!(filter.action.is_none());
    assert!(filter.mode.is_none());
}

// @tc.name: ut_search_filter_set_bundle_name
// @tc.desc: Test setting bundle_name field
// @tc.precon: NA
// @tc.step: 1. Create SearchFilter
//           2. Set bundle_name field
//           3. Verify field is set correctly
// @tc.expect: bundle_name is set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_search_filter_set_bundle_name() {
    let mut filter = SearchFilter::new();
    filter.bundle_name = Some("com.example.app".to_string());
    
    assert_eq!(filter.bundle_name, Some("com.example.app".to_string()));
}

// @tc.name: ut_search_filter_set_before
// @tc.desc: Test setting before timestamp field
// @tc.precon: NA
// @tc.step: 1. Create SearchFilter
//           2. Set before field
//           3. Verify field is set correctly
// @tc.expect: before is set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_search_filter_set_before() {
    let mut filter = SearchFilter::new();
    filter.before = Some(1628092800);
    
    assert_eq!(filter.before, Some(1628092800));
}

// @tc.name: ut_search_filter_set_after
// @tc.desc: Test setting after timestamp field
// @tc.precon: NA
// @tc.step: 1. Create SearchFilter
//           2. Set after field
//           3. Verify field is set correctly
// @tc.expect: after is set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_search_filter_set_after() {
    let mut filter = SearchFilter::new();
    filter.after = Some(1628092800);
    
    assert_eq!(filter.after, Some(1628092800));
}

// @tc.name: ut_search_filter_set_state
// @tc.desc: Test setting state field
// @tc.precon: NA
// @tc.step: 1. Create SearchFilter
//           2. Set state field
//           3. Verify field is set correctly
// @tc.expect: state is set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_search_filter_set_state() {
    let mut filter = SearchFilter::new();
    filter.state = Some(State::Completed);
    
    assert_eq!(filter.state, Some(State::Completed));
}

// @tc.name: ut_search_filter_set_action
// @tc.desc: Test setting action field
// @tc.precon: NA
// @tc.step: 1. Create SearchFilter
//           2. Set action field
//           3. Verify field is set correctly
// @tc.expect: action is set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_search_filter_set_action() {
    let mut filter = SearchFilter::new();
    filter.action = Some(Action::Download);
    
    assert_eq!(filter.action, Some(Action::Download));
}

// @tc.name: ut_search_filter_set_mode
// @tc.desc: Test setting mode field
// @tc.precon: NA
// @tc.step: 1. Create SearchFilter
//           2. Set mode field
//           3. Verify field is set correctly
// @tc.expect: mode is set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_search_filter_set_mode() {
    let mut filter = SearchFilter::new();
    filter.mode = Some(Mode::FrontEnd);
    
    assert_eq!(filter.mode, Some(Mode::FrontEnd));
}

// @tc.name: ut_search_filter_set_multiple_fields
// @tc.desc: Test setting multiple fields
// @tc.precon: NA
// @tc.step: 1. Create SearchFilter
//           2. Set multiple fields
//           3. Verify all fields are set correctly
// @tc.expect: All fields are set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_search_filter_set_multiple_fields() {
    let mut filter = SearchFilter::new();
    filter.bundle_name = Some("com.example.app".to_string());
    filter.state = Some(State::Running);
    filter.action = Some(Action::Download);
    
    assert_eq!(filter.bundle_name, Some("com.example.app".to_string()));
    assert_eq!(filter.state, Some(State::Running));
    assert_eq!(filter.action, Some(Action::Download));
    assert!(filter.before.is_none());
    assert!(filter.after.is_none());
    assert!(filter.mode.is_none());
}

// @tc.name: ut_search_filter_all_fields
// @tc.desc: Test setting all fields
// @tc.precon: NA
// @tc.step: 1. Create SearchFilter
//           2. Set all fields
//           3. Verify all fields are set correctly
// @tc.expect: All fields are set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_search_filter_all_fields() {
    let mut filter = SearchFilter::new();
    filter.bundle_name = Some("com.test.app".to_string());
    filter.before = Some(1700000000);
    filter.after = Some(1600000000);
    filter.state = Some(State::Completed);
    filter.action = Some(Action::Upload);
    filter.mode = Some(Mode::BackGround);
    
    assert_eq!(filter.bundle_name, Some("com.test.app".to_string()));
    assert_eq!(filter.before, Some(1700000000));
    assert_eq!(filter.after, Some(1600000000));
    assert_eq!(filter.state, Some(State::Completed));
    assert_eq!(filter.action, Some(Action::Upload));
    assert_eq!(filter.mode, Some(Mode::BackGround));
}

// @tc.name: ut_search_filter_time_range
// @tc.desc: Test SearchFilter with time range
// @tc.precon: NA
// @tc.step: 1. Create SearchFilter with before and after
//           2. Verify time range is valid
// @tc.expect: Time range is set correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_search_filter_time_range() {
    let mut filter = SearchFilter::new();
    filter.after = Some(1000);
    filter.before = Some(2000);
    
    assert!(filter.before.unwrap() > filter.after.unwrap());
}

// @tc.name: ut_search_filter_different_states
// @tc.desc: Test SearchFilter with different states
// @tc.precon: NA
// @tc.step: 1. Create SearchFilter with different states
//           2. Verify each state is set correctly
// @tc.expect: All states work correctly
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_search_filter_different_states() {
    let states = [
        State::Initialized,
        State::Waiting,
        State::Running,
        State::Retrying,
        State::Paused,
        State::Stopped,
        State::Completed,
        State::Failed,
        State::Removed,
    ];
    
    for state in states {
        let mut filter = SearchFilter::new();
        filter.state = Some(state.clone());
        assert_eq!(filter.state, Some(state));
    }
}
