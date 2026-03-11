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
use request_client::verify::url::{get_hostname_from_url, UrlVerifier};
use request_client::verify::ConfigVerifier;

fn create_config_with_url(url: &str) -> TaskConfig {
    let mut config = TaskConfigBuilder::new(Version::API10).build();
    config.url = url.to_string();
    config
}

// @tc.name: ut_url_verifier_valid_https_url
// @tc.desc: Test UrlVerifier with valid HTTPS URL
// @tc.precon: NA
// @tc.step: 1. Create UrlVerifier
//           2. Create TaskConfig with valid HTTPS URL
//           3. Verify config passes validation
// @tc.expect: Verification passes for valid HTTPS URL
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_url_verifier_valid_https_url() {
    let verifier = UrlVerifier {};
    let config = create_config_with_url("https://example.com/path/to/file");
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_url_verifier_valid_http_url
// @tc.desc: Test UrlVerifier with valid HTTP URL
// @tc.precon: NA
// @tc.step: 1. Create UrlVerifier
//           2. Create TaskConfig with valid HTTP URL
//           3. Verify config passes validation
// @tc.expect: Verification passes for valid HTTP URL
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_url_verifier_valid_http_url() {
    let verifier = UrlVerifier {};
    let config = create_config_with_url("http://example.com/path/to/file");
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_url_verifier_invalid_scheme
// @tc.desc: Test UrlVerifier with invalid URL scheme
// @tc.precon: NA
// @tc.step: 1. Create UrlVerifier
//           2. Create TaskConfig with FTP URL scheme
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_url_verifier_invalid_scheme() {
    let verifier = UrlVerifier {};
    let config = create_config_with_url("ftp://example.com/file");
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_url_verifier_empty_url
// @tc.desc: Test UrlVerifier with empty URL
// @tc.precon: NA
// @tc.step: 1. Create UrlVerifier
//           2. Create TaskConfig with empty URL
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_url_verifier_empty_url() {
    let verifier = UrlVerifier {};
    let config = create_config_with_url("");
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_url_verifier_exceed_max_length
// @tc.desc: Test UrlVerifier with URL exceeding max length
// @tc.precon: NA
// @tc.step: 1. Create UrlVerifier
//           2. Create TaskConfig with URL exceeding max length
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_url_verifier_exceed_max_length() {
    let verifier = UrlVerifier {};
    let long_url = format!("https://example.com/{}", "a".repeat(8192));
    let config = create_config_with_url(&long_url);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_get_hostname_from_url_standard
// @tc.desc: Test get_hostname_from_url with standard URL
// @tc.precon: NA
// @tc.step: 1. Call get_hostname_from_url with standard HTTPS URL
//           2. Verify hostname is extracted correctly
// @tc.expect: Hostname is extracted as "example.com"
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_get_hostname_from_url_standard() {
    assert_eq!(get_hostname_from_url("https://example.com/path"), "example.com");
}

// @tc.name: ut_get_hostname_from_url_with_port
// @tc.desc: Test get_hostname_from_url with URL containing port
// @tc.precon: NA
// @tc.step: 1. Call get_hostname_from_url with URL containing port
//           2. Verify hostname is extracted correctly without port
// @tc.expect: Hostname is extracted as "example.com"
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_get_hostname_from_url_with_port() {
    assert_eq!(get_hostname_from_url("https://example.com:8080/path"), "example.com");
}

// @tc.name: ut_get_hostname_from_url_empty
// @tc.desc: Test get_hostname_from_url with empty URL
// @tc.precon: NA
// @tc.step: 1. Call get_hostname_from_url with empty URL
//           2. Verify empty string is returned
// @tc.expect: Empty string is returned
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_get_hostname_from_url_empty() {
    assert_eq!(get_hostname_from_url(""), "");
}

// @tc.name: ut_get_hostname_from_url_ip_address
// @tc.desc: Test get_hostname_from_url with IP address
// @tc.precon: NA
// @tc.step: 1. Call get_hostname_from_url with IP address URL
//           2. Verify IP address is extracted correctly
// @tc.expect: IP address is extracted as "192.168.1.1"
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_get_hostname_from_url_ip_address() {
    assert_eq!(get_hostname_from_url("https://192.168.1.1:8080/path"), "192.168.1.1");
}
