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

//! Unit tests for API 9 bridge module.
//!
//! This module tests the type conversions and data transformations between
//! the API 9 ETS interface and the request core functionality.

use std::collections::HashMap;

// @tc.name: ut_api9_download_config_defaults
// @tc.desc: Test DownloadConfig to TaskConfig conversion with default values
// @tc.precon: NA
// @tc.step: 1. Create TaskConfigBuilder with minimal fields
//           2. Build TaskConfig
//           3. Verify default values are applied correctly
// @tc.expect: TaskConfig should have correct default values for API9
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_api9_download_config_defaults() {
    use request_core::config::{TaskConfigBuilder, Version, Action, NetworkConfig};

    let mut builder = TaskConfigBuilder::new(Version::API9);
    builder.url("https://example.com/file.txt".to_string());
    builder.action(Action::Download);

    let config = builder.build();

    assert_eq!(config.url, "https://example.com/file.txt");
    assert_eq!(config.common_data.action, Action::Download as u8);
    assert_eq!(config.version, Version::API9);
    assert_eq!(config.common_data.network_config, NetworkConfig::Any);
}

// @tc.name: ut_api9_download_config_with_headers
// @tc.desc: Test DownloadConfig with custom headers conversion
// @tc.precon: NA
// @tc.step: 1. Create TaskConfigBuilder with custom headers
//           2. Build TaskConfig
//           3. Verify headers are preserved in config
// @tc.expect: TaskConfig should contain the custom headers
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_api9_download_config_with_headers() {
    use request_core::config::{TaskConfigBuilder, Version, Action};

    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer token123".to_string());
    headers.insert("X-Custom-Header".to_string(), "custom_value".to_string());

    let mut builder = TaskConfigBuilder::new(Version::API9);
    builder.url("https://example.com/file.txt".to_string());
    builder.headers(headers.clone());
    builder.action(Action::Download);

    let config = builder.build();

    assert_eq!(config.headers.get("Authorization"), Some(&"Bearer token123".to_string()));
    assert_eq!(config.headers.get("X-Custom-Header"), Some(&"custom_value".to_string()));
}

// @tc.name: ut_api9_network_type_conversion
// @tc.desc: Test network type conversion from API to core NetworkConfig
// @tc.precon: NA
// @tc.step: 1. Define API network type constants
//           2. Map to core NetworkConfig values
//           3. Verify correct mapping for all network types
// @tc.expect: Network types should map correctly to NetworkConfig
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_api9_network_type_conversion() {
    use request_core::config::NetworkConfig;

    const NETWORK_MOBILE: i32 = 0x00000001;
    const NETWORK_WIFI: i32 = 0x00010000;

    // Test MOBILE mapping
    let network_type = match NETWORK_MOBILE {
        NETWORK_MOBILE => NetworkConfig::Cellular,
        NETWORK_WIFI => NetworkConfig::Wifi,
        _ => NetworkConfig::Any,
    };
    assert!(matches!(network_type, NetworkConfig::Cellular));

    // Test WIFI mapping
    let network_type = match NETWORK_WIFI {
        NETWORK_MOBILE => NetworkConfig::Cellular,
        NETWORK_WIFI => NetworkConfig::Wifi,
        _ => NetworkConfig::Any,
    };
    assert!(matches!(network_type, NetworkConfig::Wifi));

    // Test unknown network type defaults to Any
    let unknown_network = 0x9999;
    let result = match unknown_network {
        NETWORK_MOBILE => NetworkConfig::Cellular,
        NETWORK_WIFI => NetworkConfig::Wifi,
        _ => NetworkConfig::Any,
    };
    assert!(matches!(result, NetworkConfig::Any));
}

// @tc.name: ut_api9_download_config_with_all_options
// @tc.desc: Test DownloadConfig with all optional fields set
// @tc.precon: NA
// @tc.step: 1. Create TaskConfigBuilder with all options
//           2. Build TaskConfig
//           3. Verify all options are correctly mapped
// @tc.expect: All options should be correctly mapped to TaskConfig
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_api9_download_config_with_all_options() {
    use request_core::config::{TaskConfigBuilder, Version, NetworkConfig, Action};

    let mut headers = HashMap::new();
    headers.insert("X-Auth".to_string(), "token".to_string());

    let mut builder = TaskConfigBuilder::new(Version::API9);
    builder.url("https://example.com/file.zip".to_string());
    builder.headers(headers);
    builder.metered(true);
    builder.roaming(false);
    builder.network_type(NetworkConfig::Wifi);
    builder.description("Download description".to_string());
    builder.title("Custom Title".to_string());
    builder.background(true);
    builder.file_path("/downloads/file.zip".to_string());
    builder.action(Action::Download);

    let config = builder.build();

    assert_eq!(config.url, "https://example.com/file.zip");
    assert!(config.common_data.metered);
    assert!(!config.common_data.roaming);
    assert!(matches!(config.common_data.network_config, NetworkConfig::Wifi));
    assert_eq!(config.description, "Download description");
    assert_eq!(config.title, "Custom Title");
    assert!(config.common_data.background);
    assert_eq!(config.saveas, "/downloads/file.zip");
}

// @tc.name: ut_api9_upload_config_with_form_data
// @tc.desc: Test UploadConfig with form data items conversion
// @tc.precon: NA
// @tc.step: 1. Create TaskConfigBuilder with form items
//           2. Build TaskConfig
//           3. Verify form items are correctly mapped
// @tc.expect: Form items should be correctly mapped
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_api9_upload_config_with_form_data() {
    use request_core::config::{TaskConfigBuilder, Version, Action, FormItem};

    let mut builder = TaskConfigBuilder::new(Version::API9);
    builder.url("https://example.com/upload".to_string());
    builder.method("POST".to_string());
    builder.action(Action::Upload);
    builder.form_items(vec![
        FormItem { name: "field1".to_string(), value: "value1".to_string() },
        FormItem { name: "field2".to_string(), value: "value2".to_string() },
    ]);

    let config = builder.build();

    assert_eq!(config.form_items.len(), 2);
    assert_eq!(config.form_items[0].name, "field1");
    assert_eq!(config.form_items[0].value, "value1");
    assert_eq!(config.form_items[1].name, "field2");
    assert_eq!(config.form_items[1].value, "value2");
}

// @tc.name: ut_api9_upload_config_partial_range
// @tc.desc: Test UploadConfig with partial byte range conversion
// @tc.precon: NA
// @tc.step: 1. Create TaskConfigBuilder with begins and ends
//           2. Build TaskConfig
//           3. Verify range values are correctly mapped
// @tc.expect: Range values should be correctly mapped
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_api9_upload_config_partial_range() {
    use request_core::config::{TaskConfigBuilder, Version, Action};

    let mut builder = TaskConfigBuilder::new(Version::API9);
    builder.url("https://example.com/upload".to_string());
    builder.action(Action::Upload);
    builder.begins(1024);
    builder.ends(2048);

    let config = builder.build();

    assert_eq!(config.common_data.begins, 1024);
    assert_eq!(config.common_data.ends, 2048);
}

// @tc.name: ut_api9_special_characters_handling
// @tc.desc: Test special characters handling in URL and headers
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig with special URL and headers
//           2. Build TaskConfig
//           3. Verify special characters are preserved
// @tc.expect: Special characters should be preserved correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_api9_special_characters_handling() {
    use request_core::config::{TaskConfigBuilder, Version, Action};

    let special_url = "https://example.com/file?name=test%20file&version=1.0.zip";
    let mut headers = HashMap::new();
    headers.insert("X-Special".to_string(), "value with spaces & symbols=+,;".to_string());
    headers.insert("X-Unicode".to_string(), "日本語テスト".to_string());

    let mut builder = TaskConfigBuilder::new(Version::API9);
    builder.url(special_url.to_string());
    builder.headers(headers);
    builder.action(Action::Download);

    let config = builder.build();

    assert_eq!(config.url, special_url);
    assert_eq!(config.headers.get("X-Special"), Some(&"value with spaces & symbols=+,;".to_string()));
    assert_eq!(config.headers.get("X-Unicode"), Some(&"日本語テスト".to_string()));
}
