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

use request_core::config::{Action, TaskConfig, TaskConfigBuilder, Version};
use request_core::file::FileSpec;
use request_client::verify::{file_spec::FileSpecVerifier, method::MethodVerifier, ConfigVerifier};

fn create_download_config() -> TaskConfig {
    TaskConfigBuilder::new(Version::API10)
        .url("https://example.com/test".to_string())
        .action(Action::Download)
        .build()
}

fn create_upload_config_with_files(file_count: usize) -> TaskConfig {
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
        .files(files)
        .build()
}

// @tc.name: ut_file_spec_verifier_download_success
// @tc.desc: Test FileSpecVerifier with download action
// @tc.precon: NA
// @tc.step: 1. Create FileSpecVerifier
//           2. Create TaskConfig with download action
//           3. Verify config passes validation
// @tc.expect: Verification passes for download action
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_file_spec_verifier_download_success() {
    let verifier = FileSpecVerifier {};
    let config = create_download_config();
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_file_spec_verifier_upload_with_files
// @tc.desc: Test FileSpecVerifier with upload action and files
// @tc.precon: NA
// @tc.step: 1. Create FileSpecVerifier
//           2. Create TaskConfig with upload action and 1 file
//           3. Verify config passes validation
// @tc.expect: Verification passes for upload with files
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_file_spec_verifier_upload_with_files() {
    let verifier = FileSpecVerifier {};
    let config = create_upload_config_with_files(1);
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_file_spec_verifier_upload_without_files
// @tc.desc: Test FileSpecVerifier with upload action but no files
// @tc.precon: NA
// @tc.step: 1. Create FileSpecVerifier
//           2. Create TaskConfig with upload action but no files
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_file_spec_verifier_upload_without_files() {
    let verifier = FileSpecVerifier {};
    let config = TaskConfigBuilder::new(Version::API10)
        .url("https://example.com/upload".to_string())
        .action(Action::Upload)
        .build();
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_method_verifier_download_get
// @tc.desc: Test MethodVerifier with download action and GET method
// @tc.precon: NA
// @tc.step: 1. Create MethodVerifier
//           2. Create TaskConfig with download action and GET method
//           3. Verify config passes validation
// @tc.expect: Verification passes for download with GET
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_method_verifier_download_get() {
    let verifier = MethodVerifier {};
    let mut config = create_download_config();
    config.method = "GET".to_string();
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_method_verifier_download_post
// @tc.desc: Test MethodVerifier with download action and POST method
// @tc.precon: NA
// @tc.step: 1. Create MethodVerifier
//           2. Create TaskConfig with download action and POST method
//           3. Verify config passes validation
// @tc.expect: Verification passes for download with POST
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_method_verifier_download_post() {
    let verifier = MethodVerifier {};
    let mut config = create_download_config();
    config.method = "POST".to_string();
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_method_verifier_download_invalid_method
// @tc.desc: Test MethodVerifier with download action and invalid method
// @tc.precon: NA
// @tc.step: 1. Create MethodVerifier
//           2. Create TaskConfig with download action and PUT method
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_method_verifier_download_invalid_method() {
    let verifier = MethodVerifier {};
    let mut config = create_download_config();
    config.method = "PUT".to_string();
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_method_verifier_upload_put
// @tc.desc: Test MethodVerifier with upload action and PUT method
// @tc.precon: NA
// @tc.step: 1. Create MethodVerifier
//           2. Create TaskConfig with upload action and PUT method
//           3. Verify config passes validation
// @tc.expect: Verification passes for upload with PUT
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_method_verifier_upload_put() {
    let verifier = MethodVerifier {};
    let mut config = create_upload_config_with_files(1);
    config.method = "PUT".to_string();
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_method_verifier_upload_post
// @tc.desc: Test MethodVerifier with upload action and POST method
// @tc.precon: NA
// @tc.step: 1. Create MethodVerifier
//           2. Create TaskConfig with upload action and POST method
//           3. Verify config passes validation
// @tc.expect: Verification passes for upload with POST
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_method_verifier_upload_post() {
    let verifier = MethodVerifier {};
    let mut config = create_upload_config_with_files(1);
    config.method = "POST".to_string();
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_method_verifier_upload_invalid_method
// @tc.desc: Test MethodVerifier with upload action and invalid method
// @tc.precon: NA
// @tc.step: 1. Create MethodVerifier
//           2. Create TaskConfig with upload action and GET method
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_method_verifier_upload_invalid_method() {
    let verifier = MethodVerifier {};
    let mut config = create_upload_config_with_files(1);
    config.method = "GET".to_string();
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}
