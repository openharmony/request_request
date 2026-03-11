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

use request_core::config::{Action, TaskConfig, TaskConfigBuilder, Timeout, Version};
use request_core::file::FileSpec;
use request_client::verify::{TaskConfigVerifier, ConfigVerifier};

fn create_valid_download_config() -> TaskConfig {
    TaskConfigBuilder::new(Version::API10)
        .url("https://example.com/test".to_string())
        .action(Action::Download)
        .timeout(Timeout { connection_timeout: 60, total_timeout: 3600 })
        .build()
}

fn create_valid_upload_config() -> TaskConfig {
    let files = vec![FileSpec {
        name: "file".to_string(),
        path: "/tmp/file".to_string(),
        file_name: "file.txt".to_string(),
        mime_type: "text/plain".to_string(),
        is_user_file: false,
        fd: None,
    }];
    TaskConfigBuilder::new(Version::API10)
        .url("https://example.com/upload".to_string())
        .action(Action::Upload)
        .method("PUT".to_string())
        .files(files)
        .timeout(Timeout { connection_timeout: 60, total_timeout: 3600 })
        .build()
}

// @tc.name: ut_task_config_verifier_get_instance
// @tc.desc: Test TaskConfigVerifier singleton instance
// @tc.precon: NA
// @tc.step: 1. Get TaskConfigVerifier instance twice
//           2. Verify both instances are the same pointer
// @tc.expect: Both instances point to the same singleton
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_config_verifier_get_instance() {
    let instance1 = TaskConfigVerifier::get_instance();
    let instance2 = TaskConfigVerifier::get_instance();
    assert!(std::ptr::eq(instance1, instance2));
}

// @tc.name: ut_task_config_verifier_valid_download_config
// @tc.desc: Test TaskConfigVerifier with valid download config
// @tc.precon: NA
// @tc.step: 1. Get TaskConfigVerifier instance
//           2. Create valid download config
//           3. Verify config passes validation
// @tc.expect: Verification passes for valid download config
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_config_verifier_valid_download_config() {
    let verifier = TaskConfigVerifier::get_instance();
    let config = create_valid_download_config();
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_task_config_verifier_valid_upload_config
// @tc.desc: Test TaskConfigVerifier with valid upload config
// @tc.precon: NA
// @tc.step: 1. Get TaskConfigVerifier instance
//           2. Create valid upload config with files
//           3. Verify config passes validation
// @tc.expect: Verification passes for valid upload config
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_config_verifier_valid_upload_config() {
    let verifier = TaskConfigVerifier::get_instance();
    let config = create_valid_upload_config();
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_task_config_verifier_invalid_url
// @tc.desc: Test TaskConfigVerifier with invalid URL
// @tc.precon: NA
// @tc.step: 1. Get TaskConfigVerifier instance
//           2. Create config with invalid URL
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_config_verifier_invalid_url() {
    let verifier = TaskConfigVerifier::get_instance();
    let mut config = create_valid_download_config();
    config.url = "invalid_url".to_string();
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_task_config_verifier_invalid_method_download
// @tc.desc: Test TaskConfigVerifier with invalid method for download
// @tc.precon: NA
// @tc.step: 1. Get TaskConfigVerifier instance
//           2. Create download config with PUT method
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_config_verifier_invalid_method_download() {
    let verifier = TaskConfigVerifier::get_instance();
    let mut config = create_valid_download_config();
    config.method = "PUT".to_string();
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_task_config_verifier_upload_without_files
// @tc.desc: Test TaskConfigVerifier with upload but no files
// @tc.precon: NA
// @tc.step: 1. Get TaskConfigVerifier instance
//           2. Create upload config without files
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_config_verifier_upload_without_files() {
    let verifier = TaskConfigVerifier::get_instance();
    let mut config = TaskConfigBuilder::new(Version::API10)
        .url("https://example.com/upload".to_string())
        .action(Action::Upload)
        .method("PUT".to_string())
        .timeout(Timeout { connection_timeout: 60, total_timeout: 3600 })
        .build();
    config.file_specs = vec![];
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_task_config_verifier_invalid_timeout
// @tc.desc: Test TaskConfigVerifier with invalid timeout
// @tc.precon: NA
// @tc.step: 1. Get TaskConfigVerifier instance
//           2. Create config with zero connection timeout
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_config_verifier_invalid_timeout() {
    let verifier = TaskConfigVerifier::get_instance();
    let config = TaskConfigBuilder::new(Version::API10)
        .url("https://example.com/test".to_string())
        .action(Action::Download)
        .timeout(Timeout { connection_timeout: 0, total_timeout: 3600 })
        .build();
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_task_config_verifier_long_title
// @tc.desc: Test TaskConfigVerifier with title exceeding max length
// @tc.precon: NA
// @tc.step: 1. Get TaskConfigVerifier instance
//           2. Create config with title exceeding max length (257 chars)
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_config_verifier_long_title() {
    let verifier = TaskConfigVerifier::get_instance();
    let mut config = create_valid_download_config();
    config.title = "a".repeat(257);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_task_config_verifier_api9_config
// @tc.desc: Test TaskConfigVerifier with API9 config
// @tc.precon: NA
// @tc.step: 1. Get TaskConfigVerifier instance
//           2. Create config with API9 version
//           3. Verify config passes validation
// @tc.expect: Verification passes for API9 config
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_task_config_verifier_api9_config() {
    let verifier = TaskConfigVerifier::get_instance();
    let config = TaskConfigBuilder::new(Version::API9)
        .url("https://example.com/test".to_string())
        .action(Action::Download)
        .timeout(Timeout { connection_timeout: 60, total_timeout: 3600 })
        .build();
    assert!(verifier.verify(&config).is_ok());
}
