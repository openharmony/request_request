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

//! Unit tests for API 10 bridge module.
//!
//! This module tests the type conversions and data transformations between
//! the API 10 ETS interface and the request core functionality.

use std::collections::HashMap;

// @tc.name: ut_api10_enum_values_consistency
// @tc.desc: Test core enum values consistency for API10 bridge
// @tc.precon: NA
// @tc.step: 1. Verify Action enum values
//           2. Verify Mode enum values
//           3. Verify NetworkConfig enum values
//           4. Verify State enum values
// @tc.expect: All enum values should match expected API10 constants
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_api10_enum_values_consistency() {
    use request_core::config::{Action, Mode, NetworkConfig};
    use request_core::info::State;

    // Test Action enum values
    assert_eq!(Action::Download as u8, 0);
    assert_eq!(Action::Upload as u8, 1);

    // Test Mode enum values
    assert_eq!(Mode::BackGround as u8, 0);
    assert_eq!(Mode::FrontEnd as u8, 1);

    // Test NetworkConfig enum values
    assert_eq!(NetworkConfig::Any as u8, 0);
    assert_eq!(NetworkConfig::Wifi as u8, 1);
    assert_eq!(NetworkConfig::Cellular as u8, 2);

    // Test State enum values
    assert_eq!(State::Initialized as u8, 0x00);
    assert_eq!(State::Waiting as u8, 0x10);
    assert_eq!(State::Running as u8, 0x20);
    assert_eq!(State::Retrying as u8, 0x21);
    assert_eq!(State::Paused as u8, 0x30);
    assert_eq!(State::Stopped as u8, 0x31);
    assert_eq!(State::Completed as u8, 0x40);
    assert_eq!(State::Failed as u8, 0x41);
    assert_eq!(State::Removed as u8, 0x50);
}

// @tc.name: ut_api10_faults_enum_consistency
// @tc.desc: Test Faults enum values consistency for error handling
// @tc.precon: NA
// @tc.step: 1. Verify all Faults enum values
// @tc.expect: Faults enum values should match expected constants
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_api10_faults_enum_consistency() {
    use request_core::info::Faults;

    assert_eq!(Faults::Others as u8, 0xFF);
    assert_eq!(Faults::Disconnected as u8, 0x00);
    assert_eq!(Faults::Timeout as u8, 0x10);
    assert_eq!(Faults::Protocol as u8, 0x20);
    assert_eq!(Faults::Param as u8, 0x30);
    assert_eq!(Faults::Fsio as u8, 0x40);
    assert_eq!(Faults::Dns as u8, 0x50);
    assert_eq!(Faults::Tcp as u8, 0x60);
    assert_eq!(Faults::Ssl as u8, 0x70);
    assert_eq!(Faults::Redirect as u8, 0x80);
    assert_eq!(Faults::LowSpeed as u8, 0x90);
}

// @tc.name: ut_api10_config_defaults
// @tc.desc: Test Config default values conversion for API10
// @tc.precon: NA
// @tc.step: 1. Create TaskConfigBuilder with minimal fields
//           2. Build TaskConfig
//           3. Verify default values
// @tc.expect: TaskConfig should have correct default values for API10
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_api10_config_defaults() {
    use request_core::config::{TaskConfigBuilder, Version, Action, NetworkConfig};

    let mut builder = TaskConfigBuilder::new(Version::API10);
    builder.url("https://example.com/api".to_string());
    builder.action(Action::Download);

    let config = builder.build();

    assert_eq!(config.url, "https://example.com/api");
    assert_eq!(config.version, Version::API10);
    assert_eq!(config.common_data.action, Action::Download as u8);
    assert_eq!(config.common_data.network_config, NetworkConfig::Any);
    assert!(!config.common_data.metered);
    assert!(config.common_data.roaming);
    assert!(config.common_data.retry);
    assert!(config.common_data.redirect);
}

// @tc.name: ut_api10_config_with_custom_values
// @tc.desc: Test Config with custom values conversion
// @tc.precon: NA
// @tc.step: 1. Create TaskConfigBuilder with custom values
//           2. Build TaskConfig
//           3. Verify custom values are preserved
// @tc.expect: TaskConfig should have correct custom values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_api10_config_with_custom_values() {
    use request_core::config::{TaskConfigBuilder, Version, Action, NetworkConfig, Mode};

    let mut headers = HashMap::new();
    headers.insert("X-API-Key".to_string(), "secret_key".to_string());

    let mut builder = TaskConfigBuilder::new(Version::API10);
    builder.url("https://example.com/upload".to_string());
    builder.action(Action::Upload);
    builder.method("POST".to_string());
    builder.headers(headers);
    builder.title("Custom Upload".to_string());
    builder.description("Test upload description".to_string());
    builder.network_type(NetworkConfig::Wifi);
    builder.metered(true);
    builder.roaming(false);
    builder.retry(false);
    builder.redirect(false);

    let config = builder.build();

    assert_eq!(config.url, "https://example.com/upload");
    assert_eq!(config.common_data.action, Action::Upload as u8);
    assert_eq!(config.method, "POST");
    assert_eq!(config.title, "Custom Upload");
    assert_eq!(config.description, "Test upload description");
    assert_eq!(config.common_data.network_config, NetworkConfig::Wifi);
    assert!(config.common_data.metered);
    assert!(!config.common_data.roaming);
    assert!(!config.common_data.retry);
    assert!(!config.common_data.redirect);
}

// @tc.name: ut_api10_config_method_variations
// @tc.desc: Test Config method handling for download vs upload actions
// @tc.precon: NA
// @tc.step: 1. Create config with Download action
//           2. Create config with Upload action
//           3. Verify default methods are set correctly
// @tc.expect: Methods should default correctly based on action
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_api10_config_method_variations() {
    use request_core::config::{TaskConfigBuilder, Version, Action};

    // Download should default to GET
    let mut builder = TaskConfigBuilder::new(Version::API10);
    builder.url("https://example.com/file".to_string());
    builder.action(Action::Download);
    let config = builder.build();
    assert_eq!(config.method, "GET");

    // Upload should default to PUT
    let mut builder = TaskConfigBuilder::new(Version::API10);
    builder.url("https://example.com/upload".to_string());
    builder.action(Action::Upload);
    let config = builder.build();
    assert_eq!(config.method, "PUT");
}

// @tc.name: ut_api10_config_boundary_values
// @tc.desc: Test Config with boundary values for priority and index
// @tc.precon: NA
// @tc.step: 1. Test priority with 0 and max value
//           2. Test index with 0 and large value
// @tc.expect: Boundary values should be handled correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_api10_config_boundary_values() {
    use request_core::config::{TaskConfigBuilder, Version, Action};

    // Test with priority 0
    let mut builder = TaskConfigBuilder::new(Version::API10);
    builder.url("https://example.com".to_string());
    builder.action(Action::Download);
    builder.priority(0);
    let config = builder.build();
    assert_eq!(config.common_data.priority, 0);

    // Test with max u32
    let mut builder = TaskConfigBuilder::new(Version::API10);
    builder.url("https://example.com".to_string());
    builder.action(Action::Download);
    builder.priority(u32::MAX);
    let config = builder.build();
    assert_eq!(config.common_data.priority, u32::MAX);

    // Test with index 0
    let mut builder = TaskConfigBuilder::new(Version::API10);
    builder.url("https://example.com".to_string());
    builder.action(Action::Download);
    builder.index(0);
    let config = builder.build();
    assert_eq!(config.common_data.index, 0);

    // Test with large index
    let mut builder = TaskConfigBuilder::new(Version::API10);
    builder.url("https://example.com".to_string());
    builder.action(Action::Download);
    builder.index(999999);
    let config = builder.build();
    assert_eq!(config.common_data.index, 999999);
}

// @tc.name: ut_api10_config_long_strings
// @tc.desc: Test Config with long string values
// @tc.precon: NA
// @tc.step: 1. Create TaskConfigBuilder with long URL, title, description
//           2. Build TaskConfig
//           3. Verify values are preserved
// @tc.expect: Long strings should be handled
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_api10_config_long_strings() {
    use request_core::config::{TaskConfigBuilder, Version, Action};

    let long_url = format!("https://example.com/{}", "path".repeat(200));
    let long_title = "T".repeat(1000);
    let long_desc = "D".repeat(2000);

    let mut builder = TaskConfigBuilder::new(Version::API10);
    builder.url(long_url.clone());
    builder.title(long_title.clone());
    builder.description(long_desc.clone());
    builder.action(Action::Download);

    let config = builder.build();

    assert_eq!(config.url.len(), long_url.len());
    assert_eq!(config.title, long_title);
    assert_eq!(config.description, long_desc);
}
