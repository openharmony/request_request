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

use request_core::config::{MinSpeed, TaskConfig, TaskConfigBuilder, Version};
use request_client::verify::{min_speed::MinSpeedVerifier, index::IndexVerifier, ConfigVerifier};
use request_core::config::Action;
use request_core::file::FileSpec;

// ==================== MinSpeedVerifier Tests ====================

fn create_config_with_min_speed(speed: i64, duration: i64) -> TaskConfig {
    TaskConfigBuilder::new(Version::API10)
        .url("https://example.com/test".to_string())
        .min_speed(MinSpeed { speed, duration })
        .build()
}

// @tc.name: ut_min_speed_verifier_valid_zero
// @tc.desc: Test MinSpeedVerifier with zero values
// @tc.precon: NA
// @tc.step: 1. Create MinSpeedVerifier
//           2. Create TaskConfig with min_speed speed=0 and duration=0
//           3. Verify config passes validation
// @tc.expect: Verification passes for zero min_speed values
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_min_speed_verifier_valid_zero() {
    let verifier = MinSpeedVerifier {};
    let config = create_config_with_min_speed(0, 0);
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_min_speed_verifier_valid_positive
// @tc.desc: Test MinSpeedVerifier with positive values
// @tc.precon: NA
// @tc.step: 1. Create MinSpeedVerifier
//           2. Create TaskConfig with min_speed speed=1024 and duration=60
//           3. Verify config passes validation
// @tc.expect: Verification passes for positive min_speed values
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_min_speed_verifier_valid_positive() {
    let verifier = MinSpeedVerifier {};
    let config = create_config_with_min_speed(1024, 60);
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_min_speed_verifier_negative_speed
// @tc.desc: Test MinSpeedVerifier with negative speed
// @tc.precon: NA
// @tc.step: 1. Create MinSpeedVerifier
//           2. Create TaskConfig with min_speed speed=-1 and duration=60
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_min_speed_verifier_negative_speed() {
    let verifier = MinSpeedVerifier {};
    let config = create_config_with_min_speed(-1, 60);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_min_speed_verifier_negative_duration
// @tc.desc: Test MinSpeedVerifier with negative duration
// @tc.precon: NA
// @tc.step: 1. Create MinSpeedVerifier
//           2. Create TaskConfig with min_speed speed=1024 and duration=-1
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_min_speed_verifier_negative_duration() {
    let verifier = MinSpeedVerifier {};
    let config = create_config_with_min_speed(1024, -1);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// ==================== IndexVerifier Tests ====================

fn create_download_config_with_index(index: i32) -> TaskConfig {
    TaskConfigBuilder::new(Version::API10)
        .url("https://example.com/test".to_string())
        .action(Action::Download)
        .index(index)
        .build()
}

fn create_upload_config_with_index(index: i32, file_count: usize) -> TaskConfig {
    let files: Vec<FileSpec> = (0..file_count)
        .map(|i| FileSpec {
            name: format!("file_{}", i),
            path: format!("/tmp/file_{}", i),
            file_name: format!("file_{}", i),
            mime_type: "application/octet-stream".to_string(),
            is_user_file: false,
            fd: None,
        })
        .collect();
    TaskConfigBuilder::new(Version::API10)
        .url("https://example.com/upload".to_string())
        .action(Action::Upload)
        .index(index)
        .files(files)
        .build()
}

// @tc.name: ut_index_verifier_download_zero_index
// @tc.desc: Test IndexVerifier with download action and index=0
// @tc.precon: NA
// @tc.step: 1. Create IndexVerifier
//           2. Create TaskConfig with download action and index=0
//           3. Verify config passes validation
// @tc.expect: Verification passes for download with index=0
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_index_verifier_download_zero_index() {
    let verifier = IndexVerifier {};
    let config = create_download_config_with_index(0);
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_index_verifier_download_non_zero_index
// @tc.desc: Test IndexVerifier with download action and non-zero index
// @tc.precon: NA
// @tc.step: 1. Create IndexVerifier
//           2. Create TaskConfig with download action and index=1
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_index_verifier_download_non_zero_index() {
    let verifier = IndexVerifier {};
    let config = create_download_config_with_index(1);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_index_verifier_upload_valid_index
// @tc.desc: Test IndexVerifier with upload action and valid index
// @tc.precon: NA
// @tc.step: 1. Create IndexVerifier
//           2. Create TaskConfig with upload action, index=2 and 5 files
//           3. Verify config passes validation
// @tc.expect: Verification passes for upload with valid index
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_index_verifier_upload_valid_index() {
    let verifier = IndexVerifier {};
    let config = create_upload_config_with_index(2, 5);
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_index_verifier_upload_index_exceeds_file_count
// @tc.desc: Test IndexVerifier with upload action and index exceeds file count
// @tc.precon: NA
// @tc.step: 1. Create IndexVerifier
//           2. Create TaskConfig with upload action, index=4 and 3 files
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_index_verifier_upload_index_exceeds_file_count() {
    let verifier = IndexVerifier {};
    let config = create_upload_config_with_index(4, 3);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}
