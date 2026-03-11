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

use request_core::config::{Action, FormItem, TaskConfig, TaskConfigBuilder, Version};
use request_client::verify::{form_item::FormItemVerifier, token::TokenVerifier, data::DataVerifier, ConfigVerifier};

// ==================== FormItemVerifier Tests ====================

fn create_config_with_form_items(version: Version, action: Action, form_items: Vec<FormItem>) -> TaskConfig {
    TaskConfigBuilder::new(version)
        .url("https://example.com/test".to_string())
        .action(action)
        .data(form_items)
        .build()
}

// @tc.name: ut_form_item_verifier_api9_upload_with_items
// @tc.desc: Test FormItemVerifier with API9 upload and form items
// @tc.precon: NA
// @tc.step: 1. Create FormItemVerifier
//           2. Create TaskConfig with API9 upload action and form items
//           3. Verify config passes validation
// @tc.expect: Verification passes for API9 upload with form items
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_form_item_verifier_api9_upload_with_items() {
    let verifier = FormItemVerifier {};
    let form_items = vec![FormItem { name: "field1".to_string(), value: "value1".to_string() }];
    let config = create_config_with_form_items(Version::API9, Action::Upload, form_items);
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_form_item_verifier_api9_upload_empty_items
// @tc.desc: Test FormItemVerifier with API9 upload and empty form items
// @tc.precon: NA
// @tc.step: 1. Create FormItemVerifier
//           2. Create TaskConfig with API9 upload action and empty form items
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_form_item_verifier_api9_upload_empty_items() {
    let verifier = FormItemVerifier {};
    let config = create_config_with_form_items(Version::API9, Action::Upload, vec![]);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_form_item_verifier_api10_upload_with_items
// @tc.desc: Test FormItemVerifier with API10 upload and form items
// @tc.precon: NA
// @tc.step: 1. Create FormItemVerifier
//           2. Create TaskConfig with API10 upload action and form items
//           3. Verify config passes validation
// @tc.expect: Verification passes for API10 upload with form items
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_form_item_verifier_api10_upload_with_items() {
    let verifier = FormItemVerifier {};
    let form_items = vec![FormItem { name: "field1".to_string(), value: "value1".to_string() }];
    let config = create_config_with_form_items(Version::API10, Action::Upload, form_items);
    assert!(verifier.verify(&config).is_ok());
}

// ==================== TokenVerifier Tests ====================

fn create_config_with_token(version: Version, token: &str) -> TaskConfig {
    let mut config = TaskConfigBuilder::new(version)
        .url("https://example.com/test".to_string())
        .build();
    config.token = token.to_string();
    config
}

// @tc.name: ut_token_verifier_api9_any_token
// @tc.desc: Test TokenVerifier with API9 version ignores token validation
// @tc.precon: NA
// @tc.step: 1. Create TokenVerifier
//           2. Create TaskConfig with API9 version and short token
//           3. Verify config passes validation
// @tc.expect: Verification passes for API9 with any token
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_token_verifier_api9_any_token() {
    let verifier = TokenVerifier {};
    let config = create_config_with_token(Version::API9, "short");
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_token_verifier_api10_valid_token
// @tc.desc: Test TokenVerifier with API10 and valid token
// @tc.precon: NA
// @tc.step: 1. Create TokenVerifier
//           2. Create TaskConfig with API10 version and valid token
//           3. Verify config passes validation
// @tc.expect: Verification passes for API10 with valid token
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_token_verifier_api10_valid_token() {
    let verifier = TokenVerifier {};
    let config = create_config_with_token(Version::API10, "valid_token_12345");
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_token_verifier_api10_below_min_length
// @tc.desc: Test TokenVerifier with API10 and token below minimum length
// @tc.precon: NA
// @tc.step: 1. Create TokenVerifier
//           2. Create TaskConfig with API10 version and token below minimum length (7 chars)
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_token_verifier_api10_below_min_length() {
    let verifier = TokenVerifier {};
    let config = create_config_with_token(Version::API10, "1234567");
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_token_verifier_api10_exceed_max_length
// @tc.desc: Test TokenVerifier with API10 and token exceeding maximum length
// @tc.precon: NA
// @tc.step: 1. Create TokenVerifier
//           2. Create TaskConfig with API10 version and token exceeding maximum length (2049 chars)
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_token_verifier_api10_exceed_max_length() {
    let verifier = TokenVerifier {};
    let long_token = "a".repeat(2049);
    let config = create_config_with_token(Version::API10, &long_token);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// ==================== DataVerifier Tests ====================

fn create_config_with_data(version: Version, action: Action, data: &str) -> TaskConfig {
    let mut config = TaskConfigBuilder::new(version)
        .url("https://example.com/test".to_string())
        .action(action)
        .build();
    config.data = data.to_string();
    config
}

// @tc.name: ut_data_verifier_api10_download_with_data
// @tc.desc: Test DataVerifier with API10 download and data
// @tc.precon: NA
// @tc.step: 1. Create DataVerifier
//           2. Create TaskConfig with API10 download action and data
//           3. Verify config passes validation
// @tc.expect: Verification passes for API10 download with data
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_data_verifier_api10_download_with_data() {
    let verifier = DataVerifier {};
    let config = create_config_with_data(Version::API10, Action::Download, "some data");
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_data_verifier_api10_upload_with_data
// @tc.desc: Test DataVerifier with API10 upload and data
// @tc.precon: NA
// @tc.step: 1. Create DataVerifier
//           2. Create TaskConfig with API10 upload action and data
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_data_verifier_api10_upload_with_data() {
    let verifier = DataVerifier {};
    let config = create_config_with_data(Version::API10, Action::Upload, "some data");
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_data_verifier_api9_upload_with_data
// @tc.desc: Test DataVerifier with API9 upload and data
// @tc.precon: NA
// @tc.step: 1. Create DataVerifier
//           2. Create TaskConfig with API9 upload action and data
//           3. Verify config passes validation
// @tc.expect: Verification passes for API9 upload with data
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_data_verifier_api9_upload_with_data() {
    let verifier = DataVerifier {};
    let config = create_config_with_data(Version::API9, Action::Upload, "some data");
    assert!(verifier.verify(&config).is_ok());
}
