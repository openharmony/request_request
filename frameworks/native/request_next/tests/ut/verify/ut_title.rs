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

use request_core::config::{TaskConfig, TaskConfigBuilder, Version};
use request_client::verify::{title::TitleVerifier, description::DescriptionVerifier, ConfigVerifier};

// ==================== TitleVerifier Tests ====================

fn create_config_with_title(version: Version, title: &str) -> TaskConfig {
    TaskConfigBuilder::new(version)
        .url("https://example.com/test".to_string())
        .title(title.to_string())
        .build()
}

// @tc.name: ut_title_verifier_api9_long_title
// @tc.desc: Test TitleVerifier with API9 version ignores title length
// @tc.precon: NA
// @tc.step: 1. Create TitleVerifier
//           2. Create TaskConfig with API9 version and long title (300 chars)
//           3. Verify config passes validation
// @tc.expect: Verification passes for API9 with any title length
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_title_verifier_api9_long_title() {
    let verifier = TitleVerifier {};
    let long_title = "a".repeat(300);
    let config = create_config_with_title(Version::API9, &long_title);
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_title_verifier_api10_valid_title
// @tc.desc: Test TitleVerifier with API10 and valid title
// @tc.precon: NA
// @tc.step: 1. Create TitleVerifier
//           2. Create TaskConfig with API10 version and valid title
//           3. Verify config passes validation
// @tc.expect: Verification passes for API10 with valid title
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_title_verifier_api10_valid_title() {
    let verifier = TitleVerifier {};
    let config = create_config_with_title(Version::API10, "Test Download Task");
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_title_verifier_api10_max_length_title
// @tc.desc: Test TitleVerifier with API10 and max length title
// @tc.precon: NA
// @tc.step: 1. Create TitleVerifier
//           2. Create TaskConfig with API10 version and max length title (256 chars)
//           3. Verify config passes validation
// @tc.expect: Verification passes for API10 with max length title
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_title_verifier_api10_max_length_title() {
    let verifier = TitleVerifier {};
    let max_title = "a".repeat(256);
    let config = create_config_with_title(Version::API10, &max_title);
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_title_verifier_api10_exceed_max_length
// @tc.desc: Test TitleVerifier with API10 and title exceeding max length
// @tc.precon: NA
// @tc.step: 1. Create TitleVerifier
//           2. Create TaskConfig with API10 version and title exceeding max length (257 chars)
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_title_verifier_api10_exceed_max_length() {
    let verifier = TitleVerifier {};
    let long_title = "a".repeat(257);
    let config = create_config_with_title(Version::API10, &long_title);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// ==================== DescriptionVerifier Tests ====================

fn create_config_with_description(version: Version, description: &str) -> TaskConfig {
    TaskConfigBuilder::new(version)
        .url("https://example.com/test".to_string())
        .description(description.to_string())
        .build()
}

// @tc.name: ut_description_verifier_api9_long_description
// @tc.desc: Test DescriptionVerifier with API9 version ignores description length
// @tc.precon: NA
// @tc.step: 1. Create DescriptionVerifier
//           2. Create TaskConfig with API9 version and long description (2000 chars)
//           3. Verify config passes validation
// @tc.expect: Verification passes for API9 with any description length
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_description_verifier_api9_long_description() {
    let verifier = DescriptionVerifier {};
    let long_description = "a".repeat(2000);
    let config = create_config_with_description(Version::API9, &long_description);
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_description_verifier_api10_valid_description
// @tc.desc: Test DescriptionVerifier with API10 and valid description
// @tc.precon: NA
// @tc.step: 1. Create DescriptionVerifier
//           2. Create TaskConfig with API10 version and valid description
//           3. Verify config passes validation
// @tc.expect: Verification passes for API10 with valid description
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_description_verifier_api10_valid_description() {
    let verifier = DescriptionVerifier {};
    let config = create_config_with_description(Version::API10, "Test download task description");
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_description_verifier_api10_max_length_description
// @tc.desc: Test DescriptionVerifier with API10 and max length description
// @tc.precon: NA
// @tc.step: 1. Create DescriptionVerifier
//           2. Create TaskConfig with API10 version and max length description (1024 chars)
//           3. Verify config passes validation
// @tc.expect: Verification passes for API10 with max length description
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_description_verifier_api10_max_length_description() {
    let verifier = DescriptionVerifier {};
    let max_description = "a".repeat(1024);
    let config = create_config_with_description(Version::API10, &max_description);
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_description_verifier_api10_exceed_max_length
// @tc.desc: Test DescriptionVerifier with API10 and description exceeding max length
// @tc.precon: NA
// @tc.step: 1. Create DescriptionVerifier
//           2. Create TaskConfig with API10 version and description exceeding max length (1025 chars)
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_description_verifier_api10_exceed_max_length() {
    let verifier = DescriptionVerifier {};
    let long_description = "a".repeat(1025);
    let config = create_config_with_description(Version::API10, &long_description);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}
