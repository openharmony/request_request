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
use request_client::verify::{proxy::ProxyVerifier, timeout::TimeoutVerifier, ConfigVerifier};

// ==================== ProxyVerifier Tests ====================

fn create_config_with_proxy(version: Version, proxy: &str) -> TaskConfig {
    let mut config = TaskConfigBuilder::new(version)
        .url("https://example.com/test".to_string())
        .build();
    config.proxy = proxy.to_string();
    config
}

// @tc.name: ut_proxy_verifier_api9_any_proxy
// @tc.desc: Test ProxyVerifier with API9 version ignores proxy validation
// @tc.precon: NA
// @tc.step: 1. Create ProxyVerifier
//           2. Create TaskConfig with API9 version and invalid proxy
//           3. Verify config passes validation
// @tc.expect: Verification passes for API9 with any proxy
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_proxy_verifier_api9_any_proxy() {
    let verifier = ProxyVerifier {};
    let config = create_config_with_proxy(Version::API9, "invalid_proxy");
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_proxy_verifier_empty_proxy
// @tc.desc: Test ProxyVerifier with empty proxy
// @tc.precon: NA
// @tc.step: 1. Create ProxyVerifier
//           2. Create TaskConfig with API10 version and empty proxy
//           3. Verify config passes validation
// @tc.expect: Verification passes for empty proxy
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_proxy_verifier_empty_proxy() {
    let verifier = ProxyVerifier {};
    let config = create_config_with_proxy(Version::API10, "");
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_proxy_verifier_valid_proxy
// @tc.desc: Test ProxyVerifier with valid proxy
// @tc.precon: NA
// @tc.step: 1. Create ProxyVerifier
//           2. Create TaskConfig with API10 version and valid proxy
//           3. Verify config passes validation
// @tc.expect: Verification passes for valid proxy
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_proxy_verifier_valid_proxy() {
    let verifier = ProxyVerifier {};
    let config = create_config_with_proxy(Version::API10, "http://127.0.0.1:8080");
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_proxy_verifier_missing_http_prefix
// @tc.desc: Test ProxyVerifier with missing http:// prefix
// @tc.precon: NA
// @tc.step: 1. Create ProxyVerifier
//           2. Create TaskConfig with API10 version and proxy without http:// prefix
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_proxy_verifier_missing_http_prefix() {
    let verifier = ProxyVerifier {};
    let config = create_config_with_proxy(Version::API10, "127.0.0.1:8080");
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_proxy_verifier_missing_port
// @tc.desc: Test ProxyVerifier with missing port
// @tc.precon: NA
// @tc.step: 1. Create ProxyVerifier
//           2. Create TaskConfig with API10 version and proxy without port
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_proxy_verifier_missing_port() {
    let verifier = ProxyVerifier {};
    let config = create_config_with_proxy(Version::API10, "http://127.0.0.1");
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_proxy_verifier_non_numeric_port
// @tc.desc: Test ProxyVerifier with non-numeric port
// @tc.precon: NA
// @tc.step: 1. Create ProxyVerifier
//           2. Create TaskConfig with API10 version and proxy with non-numeric port
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_proxy_verifier_non_numeric_port() {
    let verifier = ProxyVerifier {};
    let config = create_config_with_proxy(Version::API10, "http://127.0.0.1:abc");
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_proxy_verifier_max_length
// @tc.desc: Test ProxyVerifier with proxy exceeding max length
// @tc.precon: NA
// @tc.step: 1. Create ProxyVerifier
//           2. Create TaskConfig with API10 version and proxy exceeding max length
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_proxy_verifier_max_length() {
    let verifier = ProxyVerifier {};
    let long_proxy = format!("http://{}", "a".repeat(600));
    let config = create_config_with_proxy(Version::API10, &long_proxy);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// ==================== TimeoutVerifier Tests ====================

fn create_config_with_timeout(connection_timeout: u64, total_timeout: u64) -> TaskConfig {
    TaskConfigBuilder::new(Version::API10)
        .url("https://example.com/test".to_string())
        .timeout(request_core::config::Timeout {
            connection_timeout,
            total_timeout,
        })
        .build()
}

// @tc.name: ut_timeout_verifier_valid_timeout
// @tc.desc: Test TimeoutVerifier with valid timeout values
// @tc.precon: NA
// @tc.step: 1. Create TimeoutVerifier
//           2. Create TaskConfig with valid timeout values
//           3. Verify config passes validation
// @tc.expect: Verification passes for valid timeout values
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_timeout_verifier_valid_timeout() {
    let verifier = TimeoutVerifier {};
    let config = create_config_with_timeout(60, 3600);
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_timeout_verifier_zero_connection_timeout
// @tc.desc: Test TimeoutVerifier with zero connection timeout
// @tc.precon: NA
// @tc.step: 1. Create TimeoutVerifier
//           2. Create TaskConfig with zero connection timeout
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_timeout_verifier_zero_connection_timeout() {
    let verifier = TimeoutVerifier {};
    let config = create_config_with_timeout(0, 3600);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_timeout_verifier_zero_total_timeout
// @tc.desc: Test TimeoutVerifier with zero total timeout
// @tc.precon: NA
// @tc.step: 1. Create TimeoutVerifier
//           2. Create TaskConfig with zero total timeout
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_timeout_verifier_zero_total_timeout() {
    let verifier = TimeoutVerifier {};
    let config = create_config_with_timeout(1, 0);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_timeout_verifier_max_total_timeout
// @tc.desc: Test TimeoutVerifier with maximum total timeout
// @tc.precon: NA
// @tc.step: 1. Create TimeoutVerifier
//           2. Create TaskConfig with maximum total timeout (604800)
//           3. Verify config passes validation
// @tc.expect: Verification passes for maximum total timeout
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_timeout_verifier_max_total_timeout() {
    let verifier = TimeoutVerifier {};
    let config = create_config_with_timeout(1, 604800);
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_timeout_verifier_exceed_max_total_timeout
// @tc.desc: Test TimeoutVerifier with total timeout exceeding max
// @tc.precon: NA
// @tc.step: 1. Create TimeoutVerifier
//           2. Create TaskConfig with total timeout exceeding max (604801)
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_timeout_verifier_exceed_max_total_timeout() {
    let verifier = TimeoutVerifier {};
    let config = create_config_with_timeout(1, 604801);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}
