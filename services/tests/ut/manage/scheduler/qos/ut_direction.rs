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

// @tc.name: ut_qos_changes_new
// @tc.desc: Test QosChanges creation with new()
// @tc.precon: NA
// @tc.step: 1. Create QosChanges using new()
//           2. Verify both download and upload fields are None
// @tc.expect: QosChanges is created with None fields for download and upload
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_qos_changes_new() {
    let changes = QosChanges::new();
    assert!(changes.download.is_none());
    assert!(changes.upload.is_none());
}

// @tc.name: ut_qos_direction_new
// @tc.desc: Test QosDirection creation with new()
// @tc.precon: NA
// @tc.step: 1. Create QosDirection with uid, task_id and QosLevel
//           2. Verify all accessor methods return correct values
// @tc.expect: QosDirection is created correctly with all fields accessible
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_qos_direction_new() {
    let uid = 1001u64;
    let task_id = 12345u32;
    let direction = QosLevel::High;
    
    let qos = QosDirection::new(uid, task_id, direction);
    
    assert_eq!(qos.uid(), uid);
    assert_eq!(qos.task_id(), task_id);
    assert_eq!(qos.direction(), QosLevel::High);
}

// @tc.name: ut_qos_direction_low
// @tc.desc: Test QosDirection with Low priority level
// @tc.precon: NA
// @tc.step: 1. Create QosDirection with Low level
//           2. Verify direction returns Low
// @tc.expect: QosDirection correctly stores Low priority
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_qos_direction_low() {
    let qos = QosDirection::new(2001, 54321, QosLevel::Low);
    
    assert_eq!(qos.uid(), 2001);
    assert_eq!(qos.task_id(), 54321);
    assert_eq!(qos.direction(), QosLevel::Low);
}

// @tc.name: ut_qos_direction_middle
// @tc.desc: Test QosDirection with Middle priority level
// @tc.precon: NA
// @tc.step: 1. Create QosDirection with Middle level
//           2. Verify direction returns Middle
// @tc.expect: QosDirection correctly stores Middle priority
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_qos_direction_middle() {
    let qos = QosDirection::new(3001, 11111, QosLevel::Middle);
    
    assert_eq!(qos.uid(), 3001);
    assert_eq!(qos.task_id(), 11111);
    assert_eq!(qos.direction(), QosLevel::Middle);
}

// @tc.name: ut_qos_level_values
// @tc.desc: Test QosLevel enum discriminant values
// @tc.precon: NA
// @tc.step: 1. Check each QosLevel discriminant value
//           2. Verify expected speeds: High=0 (unlimited), Low=400KB/s, Middle=800KB/s
// @tc.expect: QosLevel has correct speed values as discriminants
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_qos_level_values() {
    assert_eq!(QosLevel::High as i32, 0);
    assert_eq!(QosLevel::Low as i32, 400 * 1024);
    assert_eq!(QosLevel::Middle as i32, 800 * 1024);
}

// @tc.name: ut_qos_level_equality
// @tc.desc: Test QosLevel equality and inequality comparison
// @tc.precon: NA
// @tc.step: 1. Create different QosLevel values
//           2. Compare for equality and inequality
// @tc.expect: Equality comparison works correctly for all variants
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_qos_level_equality() {
    assert_eq!(QosLevel::High, QosLevel::High);
    assert_eq!(QosLevel::Low, QosLevel::Low);
    assert_eq!(QosLevel::Middle, QosLevel::Middle);
    
    assert_ne!(QosLevel::High, QosLevel::Low);
    assert_ne!(QosLevel::Low, QosLevel::Middle);
    assert_ne!(QosLevel::Middle, QosLevel::High);
}

// @tc.name: ut_qos_level_copy
// @tc.desc: Test QosLevel Copy trait implementation
// @tc.precon: NA
// @tc.step: 1. Create QosLevel value
//           2. Copy it to another variable
//           3. Verify both values are equal
// @tc.expect: Copy trait works correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_qos_level_copy() {
    let level = QosLevel::Middle;
    let copied = level;
    
    assert_eq!(level, copied);
}

// @tc.name: ut_qos_level_clone
// @tc.desc: Test QosLevel Clone trait implementation
// @tc.precon: NA
// @tc.step: 1. Create QosLevel value
//           2. Clone it to another variable
//           3. Verify both values are equal
// @tc.expect: Clone trait works correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_qos_level_clone() {
    let level = QosLevel::Low;
    let cloned = level.clone();
    
    assert_eq!(level, cloned);
}

// @tc.name: ut_qos_level_debug
// @tc.desc: Test QosLevel Debug trait implementation
// @tc.precon: NA
// @tc.step: 1. Create each QosLevel variant
//           2. Format with Debug trait
//           3. Verify output contains variant name
// @tc.expect: Debug output is correct for all variants
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_qos_level_debug() {
    let high = QosLevel::High;
    assert!(format!("{:?}", high).contains("High"));
    
    let low = QosLevel::Low;
    assert!(format!("{:?}", low).contains("Low"));
    
    let middle = QosLevel::Middle;
    assert!(format!("{:?}", middle).contains("Middle"));
}

// @tc.name: ut_qos_direction_debug
// @tc.desc: Test QosDirection Debug trait implementation
// @tc.precon: NA
// @tc.step: 1. Create QosDirection with all fields
//           2. Format with Debug trait
//           3. Verify output contains all field names
// @tc.expect: Debug output shows uid, task_id, and direction
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_qos_direction_debug() {
    let qos = QosDirection::new(1001, 99999, QosLevel::High);
    
    let debug_str = format!("{:?}", qos);
    assert!(debug_str.contains("uid"));
    assert!(debug_str.contains("task_id"));
    assert!(debug_str.contains("direction"));
}

// @tc.name: ut_qos_changes_with_download
// @tc.desc: Test QosChanges with download changes
// @tc.precon: NA
// @tc.step: 1. Create QosChanges with download changes
//           2. Verify download field contains correct data
// @tc.expect: Download changes are stored correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_qos_changes_with_download() {
    let download_changes = vec![
        QosDirection::new(1001, 1, QosLevel::High),
        QosDirection::new(1001, 2, QosLevel::Low),
    ];
    
    let changes = QosChanges {
        download: Some(download_changes),
        upload: None,
    };
    
    assert!(changes.download.is_some());
    assert!(changes.upload.is_none());
    
    let downloads = changes.download.unwrap();
    assert_eq!(downloads.len(), 2);
    assert_eq!(downloads[0].task_id(), 1);
    assert_eq!(downloads[0].direction(), QosLevel::High);
    assert_eq!(downloads[1].task_id(), 2);
    assert_eq!(downloads[1].direction(), QosLevel::Low);
}

// @tc.name: ut_qos_changes_with_upload
// @tc.desc: Test QosChanges with upload changes
// @tc.precon: NA
// @tc.step: 1. Create QosChanges with upload changes
//           2. Verify upload field contains correct data
// @tc.expect: Upload changes are stored correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_qos_changes_with_upload() {
    let upload_changes = vec![
        QosDirection::new(2001, 10, QosLevel::Middle),
    ];
    
    let changes = QosChanges {
        download: None,
        upload: Some(upload_changes),
    };
    
    assert!(changes.download.is_none());
    assert!(changes.upload.is_some());
    
    let uploads = changes.upload.unwrap();
    assert_eq!(uploads.len(), 1);
    assert_eq!(uploads[0].uid(), 2001);
    assert_eq!(uploads[0].task_id(), 10);
    assert_eq!(uploads[0].direction(), QosLevel::Middle);
}

// @tc.name: ut_qos_changes_with_both
// @tc.desc: Test QosChanges with both download and upload changes
// @tc.precon: NA
// @tc.step: 1. Create QosChanges with both download and upload changes
//           2. Verify both fields contain correct data
// @tc.expect: Both download and upload changes are stored correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_qos_changes_with_both() {
    let download_changes = vec![
        QosDirection::new(1001, 1, QosLevel::High),
    ];
    let upload_changes = vec![
        QosDirection::new(1001, 2, QosLevel::Low),
        QosDirection::new(1002, 3, QosLevel::Middle),
    ];
    
    let changes = QosChanges {
        download: Some(download_changes),
        upload: Some(upload_changes),
    };
    
    assert!(changes.download.is_some());
    assert!(changes.upload.is_some());
    assert_eq!(changes.download.as_ref().unwrap().len(), 1);
    assert_eq!(changes.upload.as_ref().unwrap().len(), 2);
}

// @tc.name: ut_qos_level_ordering
// @tc.desc: Test QosLevel speed ordering
// @tc.precon: NA
// @tc.step: 1. Compare QosLevel values as integers
//           2. Verify High < Low < Middle in terms of speed limit
// @tc.expect: Speed limits are ordered correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_qos_level_ordering() {
    let high_speed = QosLevel::High as i32;
    let low_speed = QosLevel::Low as i32;
    let middle_speed = QosLevel::Middle as i32;
    
    assert!(high_speed < low_speed);
    assert!(low_speed < middle_speed);
    assert!(high_speed < middle_speed);
}
