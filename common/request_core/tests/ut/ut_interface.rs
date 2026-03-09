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

use request_core::interface::*;

// @tc.name: ut_interface_construct
// @tc.desc: Test CONSTRUCT constant value
// @tc.precon: NA
// @tc.step: 1. Check CONSTRUCT value
//           2. Verify it equals 0
// @tc.expect: CONSTRUCT equals 0
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_construct() {
    assert_eq!(CONSTRUCT, 0);
}

// @tc.name: ut_interface_pause
// @tc.desc: Test PAUSE constant value
// @tc.precon: NA
// @tc.step: 1. Check PAUSE value
//           2. Verify it equals 1
// @tc.expect: PAUSE equals 1
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_pause() {
    assert_eq!(PAUSE, 1);
}

// @tc.name: ut_interface_query
// @tc.desc: Test QUERY constant value
// @tc.precon: NA
// @tc.step: 1. Check QUERY value
//           2. Verify it equals 2
// @tc.expect: QUERY equals 2
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_query() {
    assert_eq!(QUERY, 2);
}

// @tc.name: ut_interface_query_mime_type
// @tc.desc: Test QUERY_MIME_TYPE constant value
// @tc.precon: NA
// @tc.step: 1. Check QUERY_MIME_TYPE value
//           2. Verify it equals 3
// @tc.expect: QUERY_MIME_TYPE equals 3
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_query_mime_type() {
    assert_eq!(QUERY_MIME_TYPE, 3);
}

// @tc.name: ut_interface_remove
// @tc.desc: Test REMOVE constant value
// @tc.precon: NA
// @tc.step: 1. Check REMOVE value
//           2. Verify it equals 4
// @tc.expect: REMOVE equals 4
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_remove() {
    assert_eq!(REMOVE, 4);
}

// @tc.name: ut_interface_resume
// @tc.desc: Test RESUME constant value
// @tc.precon: NA
// @tc.step: 1. Check RESUME value
//           2. Verify it equals 5
// @tc.expect: RESUME equals 5
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_resume() {
    assert_eq!(RESUME, 5);
}

// @tc.name: ut_interface_start
// @tc.desc: Test START constant value
// @tc.precon: NA
// @tc.step: 1. Check START value
//           2. Verify it equals 6
// @tc.expect: START equals 6
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_start() {
    assert_eq!(START, 6);
}

// @tc.name: ut_interface_stop
// @tc.desc: Test STOP constant value
// @tc.precon: NA
// @tc.step: 1. Check STOP value
//           2. Verify it equals 7
// @tc.expect: STOP equals 7
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_stop() {
    assert_eq!(STOP, 7);
}

// @tc.name: ut_interface_show
// @tc.desc: Test SHOW constant value
// @tc.precon: NA
// @tc.step: 1. Check SHOW value
//           2. Verify it equals 8
// @tc.expect: SHOW equals 8
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_show() {
    assert_eq!(SHOW, 8);
}

// @tc.name: ut_interface_touch
// @tc.desc: Test TOUCH constant value
// @tc.precon: NA
// @tc.step: 1. Check TOUCH value
//           2. Verify it equals 9
// @tc.expect: TOUCH equals 9
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_touch() {
    assert_eq!(TOUCH, 9);
}

// @tc.name: ut_interface_search
// @tc.desc: Test SEARCH constant value
// @tc.precon: NA
// @tc.step: 1. Check SEARCH value
//           2. Verify it equals 10
// @tc.expect: SEARCH equals 10
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_search() {
    assert_eq!(SEARCH, 10);
}

// @tc.name: ut_interface_get_task
// @tc.desc: Test GET_TASK constant value
// @tc.precon: NA
// @tc.step: 1. Check GET_TASK value
//           2. Verify it equals 11
// @tc.expect: GET_TASK equals 11
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_get_task() {
    assert_eq!(GET_TASK, 11);
}

// @tc.name: ut_interface_clear
// @tc.desc: Test CLEAR constant value
// @tc.precon: NA
// @tc.step: 1. Check CLEAR value
//           2. Verify it equals 12
// @tc.expect: CLEAR equals 12
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_clear() {
    assert_eq!(CLEAR, 12);
}

// @tc.name: ut_interface_open_channel
// @tc.desc: Test OPEN_CHANNEL constant value
// @tc.precon: NA
// @tc.step: 1. Check OPEN_CHANNEL value
//           2. Verify it equals 13
// @tc.expect: OPEN_CHANNEL equals 13
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_open_channel() {
    assert_eq!(OPEN_CHANNEL, 13);
}

// @tc.name: ut_interface_subscribe
// @tc.desc: Test SUBSCRIBE constant value
// @tc.precon: NA
// @tc.step: 1. Check SUBSCRIBE value
//           2. Verify it equals 14
// @tc.expect: SUBSCRIBE equals 14
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_subscribe() {
    assert_eq!(SUBSCRIBE, 14);
}

// @tc.name: ut_interface_unsubscribe
// @tc.desc: Test UNSUBSCRIBE constant value
// @tc.precon: NA
// @tc.step: 1. Check UNSUBSCRIBE value
//           2. Verify it equals 15
// @tc.expect: UNSUBSCRIBE equals 15
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_unsubscribe() {
    assert_eq!(UNSUBSCRIBE, 15);
}

// @tc.name: ut_interface_group_operations
// @tc.desc: Test group-related constants
// @tc.precon: NA
// @tc.step: 1. Check CREATE_GROUP, ATTACH_GROUP, DELETE_GROUP values
//           2. Verify they equal 18, 19, 20
// @tc.expect: Group constants have correct values
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_group_operations() {
    assert_eq!(CREATE_GROUP, 18);
    assert_eq!(ATTACH_GROUP, 19);
    assert_eq!(DELETE_GROUP, 20);
}

// @tc.name: ut_interface_set_max_speed
// @tc.desc: Test SET_MAX_SPEED constant value
// @tc.precon: NA
// @tc.step: 1. Check SET_MAX_SPEED value
//           2. Verify it equals 21
// @tc.expect: SET_MAX_SPEED equals 21
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_set_max_speed() {
    assert_eq!(SET_MAX_SPEED, 21);
}

// @tc.name: ut_interface_set_mode
// @tc.desc: Test SET_MODE constant value
// @tc.precon: NA
// @tc.step: 1. Check SET_MODE value
//           2. Verify it equals 100
// @tc.expect: SET_MODE equals 100
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_set_mode() {
    assert_eq!(SET_MODE, 100);
}

// @tc.name: ut_interface_disable_task_notification
// @tc.desc: Test DISABLE_TASK_NOTIFICATION constant value
// @tc.precon: NA
// @tc.step: 1. Check DISABLE_TASK_NOTIFICATION value
//           2. Verify it equals 101
// @tc.expect: DISABLE_TASK_NOTIFICATION equals 101
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_disable_task_notification() {
    assert_eq!(DISABLE_TASK_NOTIFICATION, 101);
}

// @tc.name: ut_interface_all_distinct
// @tc.desc: Test all interface constants are distinct
// @tc.precon: NA
// @tc.step: 1. Collect all constants
//           2. Verify they are all distinct
// @tc.expect: All constants have unique values
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_interface_all_distinct() {
    let codes = [
        CONSTRUCT, PAUSE, QUERY, QUERY_MIME_TYPE, REMOVE, RESUME, START, STOP,
        SHOW, TOUCH, SEARCH, GET_TASK, CLEAR, OPEN_CHANNEL, SUBSCRIBE, UNSUBSCRIBE,
        SUB_RUN_COUNT, UNSUB_RUN_COUNT, CREATE_GROUP, ATTACH_GROUP, DELETE_GROUP,
        SET_MAX_SPEED, SET_MODE, DISABLE_TASK_NOTIFICATION,
    ];
    
    for i in 0..codes.len() {
        for j in (i + 1)..codes.len() {
            assert_ne!(codes[i], codes[j], "Interface codes should be distinct");
        }
    }
}
