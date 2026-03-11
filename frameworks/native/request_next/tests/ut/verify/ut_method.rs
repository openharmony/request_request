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
use request_client::verify::method::MethodVerifier;
use request_client::verify::ConfigVerifier;

fn create_config_with_method(action: Action, method: &str) -> TaskConfig {
    TaskConfigBuilder::new(Version::API10)
        .url("https://example.com/test".to_string())
        .action(action)
        .method(method.to_string())
        .build()
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
    let config = create_config_with_method(Action::Download, "GET");
    let result = verifier.verify(&config);
    assert!(result.is_ok());
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
    let config = create_config_with_method(Action::Download, "POST");
    let result = verifier.verify(&config);
    assert!(result.is_ok());
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
    let config = create_config_with_method(Action::Download, "PUT");
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_method_verifier_download_delete_method
// @tc.desc: Test MethodVerifier with download action and DELETE method
// @tc.precon: NA
// @tc.step: 1. Create MethodVerifier
//           2. Create TaskConfig with download action and DELETE method
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_method_verifier_download_delete_method() {
    let verifier = MethodVerifier {};
    let config = create_config_with_method(Action::Download, "DELETE");
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
    let config = create_config_with_method(Action::Upload, "PUT");
    let result = verifier.verify(&config);
    assert!(result.is_ok());
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
    let config = create_config_with_method(Action::Upload, "POST");
    let result = verifier.verify(&config);
    assert!(result.is_ok());
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
    let config = create_config_with_method(Action::Upload, "GET");
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_method_verifier_upload_delete_method
// @tc.desc: Test MethodVerifier with upload action and DELETE method
// @tc.precon: NA
// @tc.step: 1. Create MethodVerifier
//           2. Create TaskConfig with upload action and DELETE method
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_method_verifier_upload_delete_method() {
    let verifier = MethodVerifier {};
    let config = create_config_with_method(Action::Upload, "DELETE");
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_method_verifier_lowercase_method
// @tc.desc: Test MethodVerifier with lowercase method
// @tc.precon: NA
// @tc.step: 1. Create MethodVerifier
//           2. Create TaskConfig with download action and lowercase 'get'
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_method_verifier_lowercase_method() {
    let verifier = MethodVerifier {};
    let config = create_config_with_method(Action::Download, "get");
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}
