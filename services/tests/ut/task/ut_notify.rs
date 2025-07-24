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
use crate::task::notify::*;
use crate::task::config::{Action, Version};
use crate::task::info::State;
use crate::task::reason::Reason;
use crate::FileSpec;

// @tc.name: ut_subscribe_type_enum_values
// @tc.desc: Test the enum values of SubscribeType
// @tc.precon: NA
// @tc.step: 1. Check the discriminant values of SubscribeType enum variants
//           2. Verify specific values for Complete and FaultOccur
// @tc.expect: Complete has value 0, FaultOccur has value 8, others increment by 1
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 0
#[test]
fn ut_subscribe_type_enum_values() {
    assert_eq!(SubscribeType::Complete as u8, 0);
    assert_eq!(SubscribeType::Fail as u8, 1);
    assert_eq!(SubscribeType::HeaderReceive as u8, 2);
    assert_eq!(SubscribeType::Pause as u8, 3);
    assert_eq!(SubscribeType::Progress as u8, 4);
    assert_eq!(SubscribeType::Remove as u8, 5);
    assert_eq!(SubscribeType::Resume as u8, 6);
    assert_eq!(SubscribeType::FaultOccur as u8, 8);
}

// @tc.name: ut_waiting_cause_enum_values
// @tc.desc: Test the enum values of WaitingCause
// @tc.precon: NA
// @tc.step: 1. Check the discriminant values of WaitingCause enum variants
//           2. Verify values increment from 0
// @tc.expect: TaskQueue has value 0, Network has value 1, etc.
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 0
#[test]
fn ut_waiting_cause_enum_values() {
    assert_eq!(WaitingCause::TaskQueue as u8, 0);
    assert_eq!(WaitingCause::Network as u8, 1);
    assert_eq!(WaitingCause::AppState as u8, 2);
    assert_eq!(WaitingCause::UserState as u8, 3);
}

// @tc.name: ut_each_file_status_create_empty_files
// @tc.desc: Test create_each_file_status with empty file specs
// @tc.precon: NA
// @tc.step: 1. Create empty FileSpec vector
//           2. Call create_each_file_status with index 0
// @tc.expect: Returns empty vector
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_each_file_status_create_empty_files() {
    let file_specs = vec![];
    let result = EachFileStatus::create_each_file_status(&file_specs, 0, Reason::Default);
    assert_eq!(result.len(), 0);
}

// @tc.name: ut_each_file_status_create_single_file
// @tc.desc: Test create_each_file_status with single file
// @tc.precon: NA
// @tc.step: 1. Create single FileSpec
//           2. Call create_each_file_status with index 0
// @tc.expect: Returns vector with single EachFileStatus
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_each_file_status_create_single_file() {
    let file_specs = vec![FileSpec {
        name: "test.txt".to_string(),
        path: "/tmp/test.txt".to_string(),
        file_name: "test.txt".to_string(),
        mime_type: "text/plain".to_string(),
        is_user_file: false,
        fd: None,
    }];
    let result = EachFileStatus::create_each_file_status(&file_specs, 0, Reason::IoError);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].path, "/tmp/test.txt");
    assert_eq!(result[0].reason, Reason::IoError);
    assert_eq!(result[0].message, Reason::IoError.to_str());
}

// @tc.name: ut_each_file_status_create_multiple_files_index_zero
// @tc.desc: Test create_each_file_status with multiple files and index 0
// @tc.precon: NA
// @tc.step: 1. Create multiple FileSpecs
//           2. Call create_each_file_status with index 0
// @tc.expect: All files have IoError reason
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_each_file_status_create_multiple_files_index_zero() {
    let file_specs = vec![
        FileSpec {
            name: "file1.txt".to_string(),
            path: "/tmp/file1.txt".to_string(),
            file_name: "file1.txt".to_string(),
            mime_type: "text/plain".to_string(),
            is_user_file: false,
            fd: None,
        },
        FileSpec {
            name: "file2.txt".to_string(),
            path: "/tmp/file2.txt".to_string(),
            file_name: "file2.txt".to_string(),
            mime_type: "text/plain".to_string(),
            is_user_file: false,
            fd: None,
        },
    ];
    let result = EachFileStatus::create_each_file_status(&file_specs, 0, Reason::NetworkOffline);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].reason, Reason::NetworkOffline);
    assert_eq!(result[1].reason, Reason::NetworkOffline);
}

// @tc.name: ut_each_file_status_create_index_middle
// @tc.desc: Test create_each_file_status with index in middle
// @tc.precon: NA
// @tc.step: 1. Create multiple FileSpecs
//           2. Call create_each_file_status with middle index
// @tc.expect: Files before index have Default reason, others have specified reason
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 2
#[test]
fn ut_each_file_status_create_index_middle() {
    let file_specs = vec![
        FileSpec {
            name: "file1.txt".to_string(),
            path: "/tmp/file1.txt".to_string(),
            file_name: "file1.txt".to_string(),
            mime_type: "text/plain".to_string(),
            is_user_file: false,
            fd: None,
        },
        FileSpec {
            name: "file2.txt".to_string(),
            path: "/tmp/file2.txt".to_string(),
            file_name: "file2.txt".to_string(),
            mime_type: "text/plain".to_string(),
            is_user_file: false,
            fd: None,
        },
        FileSpec {
            name: "file3.txt".to_string(),
            path: "/tmp/file3.txt".to_string(),
            file_name: "file3.txt".to_string(),
            mime_type: "text/plain".to_string(),
            is_user_file: false,
            fd: None,
        },
    ];
    let result = EachFileStatus::create_each_file_status(&file_specs, 2, Reason::RequestError);
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].reason, Reason::Default);
    assert_eq!(result[1].reason, Reason::Default);
    assert_eq!(result[2].reason, Reason::RequestError);
}

// @tc.name: ut_each_file_status_create_index_beyond_length
// @tc.desc: Test create_each_file_status with index beyond file count
// @tc.precon: NA
// @tc.step: 1. Create 2 FileSpecs
//           2. Call create_each_file_status with index 3
// @tc.expect: All files have Default reason
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 2
#[test]
fn ut_each_file_status_create_index_beyond_length() {
    let file_specs = vec![
        FileSpec {
            name: "file1.txt".to_string(),
            path: "/tmp/file1.txt".to_string(),
            file_name: "file1.txt".to_string(),
            mime_type: "text/plain".to_string(),
            is_user_file: false,
            fd: None,
        },
        FileSpec {
            name: "file2.txt".to_string(),
            path: "/tmp/file2.txt".to_string(),
            file_name: "file2.txt".to_string(),
            mime_type: "text/plain".to_string(),
            is_user_file: false,
            fd: None,
        },
    ];
    let result = EachFileStatus::create_each_file_status(&file_specs, 5, Reason::UploadFileError);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].reason, Reason::Default);
    assert_eq!(result[1].reason, Reason::Default);
}

// @tc.name: ut_progress_new_empty_sizes
// @tc.desc: Test Progress::new with empty sizes vector
// @tc.precon: NA
// @tc.step: 1. Call Progress::new with empty vec
// @tc.expect: Creates Progress with empty processed vector and Initialized state
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_progress_new_empty_sizes() {
    let progress = Progress::new(vec![]);
    assert_eq!(progress.sizes.len(), 0);
    assert_eq!(progress.processed.len(), 0);
    assert_eq!(progress.common_data.state, State::Initialized.repr);
    assert_eq!(progress.common_data.index, 0);
    assert_eq!(progress.common_data.total_processed, 0);
    assert!(progress.extras.is_empty());
}

// @tc.name: ut_progress_new_single_size
// @tc.desc: Test Progress::new with single size
// @tc.precon: NA
// @tc.step: 1. Call Progress::new with single size
// @tc.expect: Creates Progress with single processed element initialized to 0
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_progress_new_single_size() {
    let progress = Progress::new(vec![1024]);
    assert_eq!(progress.sizes.len(), 1);
    assert_eq!(progress.sizes[0], 1024);
    assert_eq!(progress.processed.len(), 1);
    assert_eq!(progress.processed[0], 0);
    assert_eq!(progress.common_data.state, State::Initialized.repr);
}

// @tc.name: ut_progress_new_multiple_sizes
// @tc.desc: Test Progress::new with multiple sizes
// @tc.precon: NA
// @tc.step: 1. Call Progress::new with multiple sizes
// @tc.expect: Creates Progress with processed vector matching sizes length
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_progress_new_multiple_sizes() {
    let sizes = vec![1024, 2048, 4096];
    let progress = Progress::new(sizes.clone());
    assert_eq!(progress.sizes, sizes);
    assert_eq!(progress.processed.len(), 3);
    assert_eq!(progress.processed, vec![0, 0, 0]);
}

// @tc.name: ut_progress_is_finish_empty_sizes
// @tc.desc: Test Progress::is_finish with empty sizes
// @tc.precon: NA
// @tc.step: 1. Create Progress with empty sizes
//           2. Call is_finish
// @tc.expect: Returns true (empty sum equals empty sum)
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_progress_is_finish_empty_sizes() {
    let progress = Progress::new(vec![]);
    assert!(progress.is_finish());
}

// @tc.name: ut_progress_is_finish_single_complete
// @tc.desc: Test Progress::is_finish with single file complete
// @tc.precon: NA
// @tc.step: 1. Create Progress with single size
//           2. Set processed to match size
// @tc.expect: Returns true
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_progress_is_finish_single_complete() {
    let mut progress = Progress::new(vec![1024]);
    progress.processed[0] = 1024;
    assert!(progress.is_finish());
}

// @tc.name: ut_progress_is_finish_single_incomplete
// @tc.desc: Test Progress::is_finish with single file incomplete
// @tc.precon: NA
// @tc.step: 1. Create Progress with single size
//           2. Set processed less than size
// @tc.expect: Returns false
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 2
#[test]
fn ut_progress_is_finish_single_incomplete() {
    let mut progress = Progress::new(vec![1024]);
    progress.processed[0] = 512;
    assert!(!progress.is_finish());
}

// @tc.name: ut_progress_is_finish_negative_size
// @tc.desc: Test Progress::is_finish with negative size
// @tc.precon: NA
// @tc.step: 1. Create Progress with negative size
//           2. Set processed to any value
// @tc.expect: Returns false due to negative size check
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 2
#[test]
fn ut_progress_is_finish_negative_size() {
    let mut progress = Progress::new(vec![-1]);
    progress.processed[0] = 0;
    assert!(!progress.is_finish());
}

// @tc.name: ut_progress_is_finish_mixed_negative_positive
// @tc.desc: Test Progress::is_finish with mixed negative and positive sizes
// @tc.precon: NA
// @tc.step: 1. Create Progress with mixed sizes
//           2. Set processed to match positive sizes
// @tc.expect: Returns false due to negative size
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 2
#[test]
fn ut_progress_is_finish_mixed_negative_positive() {
    let mut progress = Progress::new(vec![1024, -1, 2048]);
    progress.processed[0] = 1024;
    progress.processed[1] = 0;
    progress.processed[2] = 2048;
    assert!(!progress.is_finish());
}

// @tc.name: ut_progress_is_finish_multiple_complete
// @tc.desc: Test Progress::is_finish with multiple files complete
// @tc.precon: NA
// @tc.step: 1. Create Progress with multiple sizes
//           2. Set processed to match all sizes
// @tc.expect: Returns true
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_progress_is_finish_multiple_complete() {
    let mut progress = Progress::new(vec![1024, 2048, 4096]);
    progress.processed[0] = 1024;
    progress.processed[1] = 2048;
    progress.processed[2] = 4096;
    assert!(progress.is_finish());
}

// @tc.name: ut_progress_is_finish_multiple_partial
// @tc.desc: Test Progress::is_finish with multiple files partially complete
// @tc.precon: NA
// @tc.step: 1. Create Progress with multiple sizes
//           2. Set processed to partial values
// @tc.expect: Returns false
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 2
#[test]
fn ut_progress_is_finish_multiple_partial() {
    let mut progress = Progress::new(vec![1024, 2048, 4096]);
    progress.processed[0] = 1024;
    progress.processed[1] = 1000;
    progress.processed[2] = 4096;
    assert!(!progress.is_finish());
}

// @tc.name: ut_progress_is_finish_zero_size_complete
// @tc.desc: Test Progress::is_finish with zero size files
// @tc.precon: NA
// @tc.step: 1. Create Progress with zero sizes
//           2. Set processed to zero
// @tc.expect: Returns true (0 == 0)
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 2
#[test]
fn ut_progress_is_finish_zero_size_complete() {
    let mut progress = Progress::new(vec![0, 0, 0]);
    progress.processed[0] = 0;
    progress.processed[1] = 0;
    progress.processed[2] = 0;
    assert!(progress.is_finish());
}

// @tc.name: ut_notify_data_creation
// @tc.desc: Test NotifyData struct creation with all fields
// @tc.precon: NA
// @tc.step: 1. Create all required components
//           2. Create NotifyData instance
// @tc.expect: All fields are correctly initialized
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_notify_data_creation() {
    let bundle = "com.example.app".to_string();
    let progress = Progress::new(vec![1024, 2048]);
    let action = Action::Download;
    let version = Version::V1;
    let file_specs = vec![
        FileSpec {
            name: "file1.txt".to_string(),
            path: "/tmp/file1.txt".to_string(),
            file_name: "file1.txt".to_string(),
            mime_type: "text/plain".to_string(),
            is_user_file: false,
            fd: None,
        },
    ];
    let each_file_status = EachFileStatus::create_each_file_status(&file_specs, 0, Reason::Default);
    let task_id = 12345;
    let uid = 1000;

    let notify_data = NotifyData {
        bundle: bundle.clone(),
        progress: progress.clone(),
        action,
        version,
        each_file_status,
        task_id,
        uid,
    };

    assert_eq!(notify_data.bundle, bundle);
    assert_eq!(notify_data.progress.sizes, vec![1024, 2048]);
    assert_eq!(notify_data.action, action);
    assert_eq!(notify_data.version, version);
    assert_eq!(notify_data.each_file_status.len(), 1);
    assert_eq!(notify_data.task_id, task_id);
    assert_eq!(notify_data.uid, uid);
}

// @tc.name: ut_common_progress_default_values
// @tc.desc: Test CommonProgress default initialization
// @tc.precon: NA
// @tc.step: 1. Create CommonProgress with default values
// @tc.expect: All fields are correctly initialized
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_common_progress_default_values() {
    let common_progress = CommonProgress {
        state: State::Running.repr,
        index: 5,
        total_processed: 1024,
    };
    assert_eq!(common_progress.state, State::Running.repr);
    assert_eq!(common_progress.index, 5);
    assert_eq!(common_progress.total_processed, 1024);
}

// @tc.name: ut_each_file_status_clone
// @tc.desc: Test EachFileStatus clone implementation
// @tc.precon: NA
// @tc.step: 1. Create EachFileStatus instance
//           2. Clone the instance
// @tc.expect: Cloned instance has same values
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_each_file_status_clone() {
    let original = EachFileStatus {
        path: "/tmp/test.txt".to_string(),
        reason: Reason::IoError,
        message: "IO Error".to_string(),
    };
    let cloned = original.clone();
    assert_eq!(cloned.path, "/tmp/test.txt");
    assert_eq!(cloned.reason, Reason::IoError);
    assert_eq!(cloned.message, "IO Error");
}

// @tc.name: ut_progress_clone
// @tc.desc: Test Progress clone implementation
// @tc.precon: NA
// @tc.step: 1. Create Progress instance with data
//           2. Clone the instance
// @tc.expect: Cloned instance has same values
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_progress_clone() {
    let mut original = Progress::new(vec![1024, 2048]);
    original.common_data.state = State::Running.repr;
    original.common_data.index = 1;
    original.common_data.total_processed = 1024;
    original.processed[0] = 1024;
    original.processed[1] = 512;
    original.extras.insert("key".to_string(), "value".to_string());

    let cloned = original.clone();
    assert_eq!(cloned.sizes, vec![1024, 2048]);
    assert_eq!(cloned.processed, vec![1024, 512]);
    assert_eq!(cloned.common_data.state, State::Running.repr);
    assert_eq!(cloned.common_data.index, 1);
    assert_eq!(cloned.common_data.total_processed, 1024);
    assert_eq!(cloned.extras.get("key"), Some(&"value".to_string()));
}

// @tc.name: ut_notify_data_clone
// @tc.desc: Test NotifyData clone implementation
// @tc.precon: NA
// @tc.step: 1. Create NotifyData instance
//           2. Clone the instance
// @tc.expect: Cloned instance has same values
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 1
#[test]
fn ut_notify_data_clone() {
    let file_specs = vec![FileSpec {
        name: "test.txt".to_string(),
        path: "/tmp/test.txt".to_string(),
        file_name: "test.txt".to_string(),
        mime_type: "text/plain".to_string(),
        is_user_file: false,
        fd: None,
    }];
    let each_file_status = EachFileStatus::create_each_file_status(&file_specs, 0, Reason::Default);

    let original = NotifyData {
        bundle: "com.test.app".to_string(),
        progress: Progress::new(vec![1024]),
        action: Action::Upload,
        version: Version::V2,
        each_file_status,
        task_id: 999,
        uid: 1001,
    };

    let cloned = original.clone();
    assert_eq!(cloned.bundle, "com.test.app");
    assert_eq!(cloned.action, Action::Upload);
    assert_eq!(cloned.version, Version::V2);
    assert_eq!(cloned.task_id, 999);
    assert_eq!(cloned.uid, 1001);
    assert_eq!(cloned.each_file_status.len(), 1);
}

// @tc.name: ut_progress_large_values
// @tc.desc: Test Progress with large file sizes
// @tc.precon: NA
// @tc.step: 1. Create Progress with large size values
//           2. Test is_finish with large values
// @tc.expect: Handles large values correctly
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 3
#[test]
fn ut_progress_large_values() {
    let large_size = i64::MAX / 2;
    let mut progress = Progress::new(vec![large_size]);
    progress.processed[0] = large_size as usize;
    assert!(progress.is_finish());
}

// @tc.name: ut_progress_edge_case_zero_processed
// @tc.desc: Test Progress with zero processed and non-zero sizes
// @tc.precon: NA
// @tc.step: 1. Create Progress with non-zero sizes
//           2. Keep processed as zero
// @tc.expect: is_finish returns false
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 2
#[test]
fn ut_progress_edge_case_zero_processed() {
    let progress = Progress::new(vec![1024, 2048]);
    assert!(!progress.is_finish());
}

// @tc.name: ut_each_file_status_empty_path
// @tc.desc: Test EachFileStatus with empty path
// @tc.precon: NA
// @tc.step: 1. Create FileSpec with empty path
//           2. Create EachFileStatus
// @tc.expect: Handles empty path correctly
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 2
#[test]
fn ut_each_file_status_empty_path() {
    let file_specs = vec![FileSpec {
        name: "".to_string(),
        path: "".to_string(),
        file_name: "".to_string(),
        mime_type: "".to_string(),
        is_user_file: false,
        fd: None,
    }];
    let result = EachFileStatus::create_each_file_status(&file_specs, 0, Reason::Default);
    assert_eq!(result[0].path, "");
    assert_eq!(result[0].message, Reason::Default.to_str());
}

// @tc.name: ut_progress_extras_hashmap
// @tc.desc: Test Progress extras HashMap functionality
// @tc.precon: NA
// @tc.step: 1. Create Progress instance
//           2. Insert multiple key-value pairs into extras
//           3. Verify HashMap operations
// @tc.expect: HashMap operations work correctly
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 2
#[test]
fn ut_progress_extras_hashmap() {
    let mut progress = Progress::new(vec![1024]);
    progress.extras.insert("key1".to_string(), "value1".to_string());
    progress.extras.insert("key2".to_string(), "value2".to_string());

    assert_eq!(progress.extras.len(), 2);
    assert_eq!(progress.extras.get("key1"), Some(&"value1".to_string()));
    assert_eq!(progress.extras.get("key2"), Some(&"value2".to_string()));
    assert!(progress.extras.contains_key("key1"));
    assert!(!progress.extras.contains_key("nonexistent"));
}

// @tc.name: ut_each_file_status_unicode_path
// @tc.desc: Test EachFileStatus with Unicode characters in path
// @tc.precon: NA
// @tc.step: 1. Create FileSpec with Unicode path
//           2. Create EachFileStatus
// @tc.expect: Handles Unicode path correctly
// @tc.type: FUNC
// @tc.require: issue#ICOHJ2
// @tc.level: Level 3
#[test]
fn ut_each_file_status_unicode_path() {
    let file_specs = vec![FileSpec {
        name: "测试.txt".to_string(),
        path: "/tmp/测试文件.txt".to_string(),
        file_name: "测试.txt".to_string(),
        mime_type: "text/plain".to_string(),
        is_user_file: false,
        fd: None,
    }];
    let result = EachFileStatus::create_each_file_status(&file_specs, 0, Reason::Default);
    assert_eq!(result[0].path, "/tmp/测试文件.txt");
}