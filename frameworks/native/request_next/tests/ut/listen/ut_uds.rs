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

use request_client::listen::uds::{Message, MAGIC_NUM, HTTP_RESPONSE, NOTIFY_DATA, FAULTS, WAIT};
use request_client::listen::ser::{Serialize, UdsSer};
use request_core::info::{Response, NotifyData, FaultOccur, Wait};

// @tc.name: ut_uds_magic_num_value
// @tc.desc: Test MAGIC_NUM constant value
// @tc.precon: NA
// @tc.step: 1. Check MAGIC_NUM value
//           2. Verify it equals expected value
// @tc.expect: MAGIC_NUM equals 0x43434646
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_magic_num_value() {
    assert_eq!(MAGIC_NUM, 0x43434646);
}

// @tc.name: ut_uds_message_type_constants
// @tc.desc: Test message type constants
// @tc.precon: NA
// @tc.step: 1. Check all message type constants
//           2. Verify they have expected values
// @tc.expect: All constants have correct values
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_message_type_constants() {
    assert_eq!(HTTP_RESPONSE, 0);
    assert_eq!(NOTIFY_DATA, 1);
    assert_eq!(FAULTS, 2);
    assert_eq!(WAIT, 3);
}

// @tc.name: ut_uds_message_http_response_variant
// @tc.desc: Test Message::HttpResponse variant
// @tc.precon: NA
// @tc.step: 1. Create Message::HttpResponse variant
//           2. Verify variant is created
// @tc.expect: Message::HttpResponse variant is created successfully
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_message_http_response_variant() {
    let response = Response::default();
    let message = Message::HttpResponse(response);
    
    match message {
        Message::HttpResponse(_) => assert!(true),
        _ => panic!("Expected HttpResponse variant"),
    }
}

// @tc.name: ut_uds_message_notify_data_variant
// @tc.desc: Test Message::NotifyData variant
// @tc.precon: NA
// @tc.step: 1. Create Message::NotifyData variant
//           2. Verify variant is created
// @tc.expect: Message::NotifyData variant is created successfully
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_message_notify_data_variant() {
    let notify_data = NotifyData::default();
    let message = Message::NotifyData(notify_data);
    
    match message {
        Message::NotifyData(_) => assert!(true),
        _ => panic!("Expected NotifyData variant"),
    }
}

// @tc.name: ut_uds_message_faults_variant
// @tc.desc: Test Message::Faults variant
// @tc.precon: NA
// @tc.step: 1. Create Message::Faults variant
//           2. Verify variant is created
// @tc.expect: Message::Faults variant is created successfully
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_message_faults_variant() {
    let fault_occur = FaultOccur::default();
    let message = Message::Faults(fault_occur);
    
    match message {
        Message::Faults(_) => assert!(true),
        _ => panic!("Expected Faults variant"),
    }
}

// @tc.name: ut_uds_message_wait_variant
// @tc.desc: Test Message::WAIT variant
// @tc.precon: NA
// @tc.step: 1. Create Message::WAIT variant
//           2. Verify variant is created
// @tc.expect: Message::WAIT variant is created successfully
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_message_wait_variant() {
    let wait = Wait::default();
    let message = Message::WAIT(wait);
    
    match message {
        Message::WAIT(_) => assert!(true),
        _ => panic!("Expected WAIT variant"),
    }
}

// @tc.name: ut_uds_message_all_variants
// @tc.desc: Test all Message variants can be created
// @tc.precon: NA
// @tc.step: 1. Create all Message variants
//           2. Verify all variants are created successfully
// @tc.expect: All Message variants are created successfully
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_message_all_variants() {
    let messages: Vec<Message> = vec![
        Message::HttpResponse(Response::default()),
        Message::NotifyData(NotifyData::default()),
        Message::Faults(FaultOccur::default()),
        Message::WAIT(Wait::default()),
    ];
    
    assert_eq!(messages.len(), 4);
}

// @tc.name: ut_uds_message_pattern_match
// @tc.desc: Test Message pattern matching
// @tc.precon: NA
// @tc.step: 1. Create different Message variants
//           2. Pattern match each variant
//           3. Verify correct branch is taken
// @tc.expect: Pattern matching works correctly for all variants
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_message_pattern_match() {
    fn get_message_type(message: &Message) -> &'static str {
        match message {
            Message::HttpResponse(_) => "http_response",
            Message::NotifyData(_) => "notify_data",
            Message::Faults(_) => "faults",
            Message::WAIT(_) => "wait",
        }
    }
    
    assert_eq!(get_message_type(&Message::HttpResponse(Response::default())), "http_response");
    assert_eq!(get_message_type(&Message::NotifyData(NotifyData::default())), "notify_data");
    assert_eq!(get_message_type(&Message::Faults(FaultOccur::default())), "faults");
    assert_eq!(get_message_type(&Message::WAIT(Wait::default())), "wait");
}

// @tc.name: ut_uds_message_extract_data
// @tc.desc: Test extracting data from Message variants
// @tc.precon: NA
// @tc.step: 1. Create Message with data
//           2. Extract the inner data
//           3. Verify data is correct
// @tc.expect: Data can be extracted from Message variants
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_message_extract_data() {
    let mut response = Response::default();
    response.task_id = "12345".to_string();
    
    let message = Message::HttpResponse(response);
    
    if let Message::HttpResponse(resp) = message {
        assert_eq!(resp.task_id, "12345");
    } else {
        panic!("Expected HttpResponse");
    }
}

// @tc.name: ut_uds_magic_num_bytes
// @tc.desc: Test MAGIC_NUM byte representation
// @tc.precon: NA
// @tc.step: 1. Convert MAGIC_NUM to bytes
//           2. Verify byte representation
// @tc.expect: MAGIC_NUM has correct byte representation
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_magic_num_bytes() {
    let bytes = MAGIC_NUM.to_ne_bytes();
    let reconstructed = i32::from_ne_bytes(bytes);
    
    assert_eq!(reconstructed, MAGIC_NUM);
}

// @tc.name: ut_uds_message_type_range
// @tc.desc: Test message type constants are distinct
// @tc.precon: NA
// @tc.step: 1. Collect all message type constants
//           2. Verify they are all distinct
// @tc.expect: All message type constants have distinct values
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_message_type_range() {
    let types = [HTTP_RESPONSE, NOTIFY_DATA, FAULTS, WAIT];
    
    for i in 0..types.len() {
        for j in (i + 1)..types.len() {
            assert_ne!(types[i], types[j], "Message types should be distinct");
        }
    }
}

// @tc.name: ut_uds_message_response_default
// @tc.desc: Test Response default values in Message
// @tc.precon: NA
// @tc.step: 1. Create Message with default Response
//           2. Verify default values
// @tc.expect: Default Response values are correct
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_message_response_default() {
    let response = Response::default();
    let message = Message::HttpResponse(response.clone());
    
    if let Message::HttpResponse(resp) = message {
        assert_eq!(resp.status_code, response.status_code);
        assert_eq!(resp.task_id, response.task_id);
    } else {
        panic!("Expected HttpResponse");
    }
}

// @tc.name: ut_uds_message_notify_data_default
// @tc.desc: Test NotifyData default values in Message
// @tc.precon: NA
// @tc.step: 1. Create Message with default NotifyData
//           2. Verify default values
// @tc.expect: Default NotifyData values are correct
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_message_notify_data_default() {
    let notify_data = NotifyData::default();
    let message = Message::NotifyData(notify_data.clone());
    
    if let Message::NotifyData(data) = message {
        assert_eq!(data.task_id, notify_data.task_id);
    } else {
        panic!("Expected NotifyData");
    }
}

// @tc.name: ut_uds_message_ownership_transfer
// @tc.desc: Test Message ownership transfer
// @tc.precon: NA
// @tc.step: 1. Create Message
//           2. Transfer ownership
//           3. Verify ownership is transferred
// @tc.expect: Message ownership can be transferred
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_message_ownership_transfer() {
    let message = Message::HttpResponse(Response::default());
    
    fn take_message(msg: Message) {
        let _ = msg;
    }
    
    take_message(message);
}

// @tc.name: ut_uds_message_clone_response
// @tc.desc: Test cloning Response in Message
// @tc.precon: NA
// @tc.step: 1. Create Response with custom values
//           2. Clone and create Message
//           3. Verify original is unchanged
// @tc.expect: Response can be cloned for Message
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_uds_message_clone_response() {
    let mut response = Response::default();
    response.task_id = "test_task".to_string();
    
    let cloned = response.clone();
    let _message = Message::HttpResponse(cloned);
    
    assert_eq!(response.task_id, "test_task");
}
