// Copyright (C) 2023 Huawei Device Co., Ltd.
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

use super::*;
use super::super::files::BundleCache;

// @tc.name: ut_action_to_domain_type
// @tc.desc: Test action_to_domain_type with all action types
// @tc.precon: NA
// @tc.step: 1. Call action_to_domain_type with Download, Upload, Any
// @tc.expect: Returns correct domain type string for each action
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_action_to_domain_type() {
    assert_eq!(action_to_domain_type(Action::Download), "download");
    assert_eq!(action_to_domain_type(Action::Upload), "upload");
    assert_eq!(action_to_domain_type(Action::Any), "");
}

// @tc.name: ut_build_task_proxy_empty
// @tc.desc: Test build_task_proxy with empty proxy
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig with empty proxy
//           2. Call build_task_proxy
// @tc.expect: Returns Ok(None)
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_build_task_proxy_empty() {
    let config = TaskConfig::default();
    let result = build_task_proxy(&config);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

// @tc.name: ut_build_task_proxy_with_value
// @tc.desc: Test build_task_proxy with proxy value
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig with proxy URL
//           2. Call build_task_proxy
// @tc.expect: Returns Ok(Some(Proxy))
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_build_task_proxy_with_value() {
    let mut config = TaskConfig::default();
    config.proxy = "http://proxy.example.com:8080".to_string();

    let result = build_task_proxy(&config);
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

// @tc.name: ut_build_task_certificate_pins_empty
// @tc.desc: Test build_task_certificate_pins with empty pins
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig with empty certificate_pins
//           2. Call build_task_certificate_pins
// @tc.expect: Returns Ok(None)
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_build_task_certificate_pins_empty() {
    let config = TaskConfig::default();
    let result = build_task_certificate_pins(&config);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

// @tc.name: ut_build_task_certs_empty
// @tc.desc: Test build_task_certs with empty paths
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig with empty certs_path
//           2. Call build_task_certs
// @tc.expect: Returns Ok(empty vector)
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_build_task_certs_empty() {
    let config = TaskConfig::default();
    let result = build_task_certs(&config);
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

// @tc.name: ut_bundle_cache_get_value_caching
// @tc.desc: Test BundleCache caches value correctly
// @tc.precon: NA
// @tc.step: 1. Create BundleCache with TaskConfig
//           2. Call get_value multiple times
// @tc.expect: Value is cached after first call
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_bundle_cache_get_value_caching() {
    let config = TaskConfig::default();
    let mut cache = BundleCache::new(&config);

    assert!(cache.value.is_none());

    let result1 = cache.get_value();
    assert!(cache.value.is_some());

    let result2 = cache.get_value();
    assert_eq!(result1, result2);
}

// @tc.name: ut_bundle_cache_new
// @tc.desc: Test BundleCache::new creates cache with None value
// @tc.precon: NA
// @tc.step: 1. Create BundleCache with TaskConfig
//           2. Verify initial value is None
// @tc.expect: value field is None
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_bundle_cache_new() {
    let config = TaskConfig::default();
    let cache = BundleCache::new(&config);

    assert!(cache.value.is_none());
}

// @tc.name: ut_task_config_default_action
// @tc.desc: Test TaskConfig default action is Download
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig using default
//           2. Check action field
// @tc.expect: action is Action::Download
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_config_default_action() {
    let config = TaskConfig::default();
    assert_eq!(config.common_data.action, Action::Download);
}

// @tc.name: ut_task_config_default_mode
// @tc.desc: Test TaskConfig default mode is BackGround
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig using default
//           2. Check mode field
// @tc.expect: mode is Mode::BackGround
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_config_default_mode() {
    let config = TaskConfig::default();
    assert_eq!(config.common_data.mode, Mode::BackGround);
}

// @tc.name: ut_task_config_default_redirect
// @tc.desc: Test TaskConfig default redirect is true
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig using default
//           2. Check redirect field
// @tc.expect: redirect is true
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_config_default_redirect() {
    let config = TaskConfig::default();
    assert!(config.common_data.redirect);
}

// @tc.name: ut_task_config_default_method
// @tc.desc: Test TaskConfig default method is GET
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig using default
//           2. Check method field
// @tc.expect: method is "GET"
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_config_default_method() {
    let config = TaskConfig::default();
    assert_eq!(config.method, "GET");
}

// @tc.name: ut_task_config_default_version
// @tc.desc: Test TaskConfig default version is API10
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig using default
//           2. Check version field
// @tc.expect: version is Version::API10
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_config_default_version() {
    let config = TaskConfig::default();
    assert_eq!(config.version, Version::API10);
}

// @tc.name: ut_task_config_default_ends
// @tc.desc: Test TaskConfig default ends is -1
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig using default
//           2. Check ends field
// @tc.expect: ends is -1 (no deadline)
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_config_default_ends() {
    let config = TaskConfig::default();
    assert_eq!(config.common_data.ends, -1);
}

// @tc.name: ut_task_config_default_empty_collections
// @tc.desc: Test TaskConfig default collections are empty
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig using default
//           2. Check collection fields
// @tc.expect: All collections are empty
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_config_default_empty_collections() {
    let config = TaskConfig::default();

    assert!(config.form_items.is_empty());
    assert!(config.file_specs.is_empty());
    assert!(config.body_file_paths.is_empty());
    assert!(config.certs_path.is_empty());
    assert!(config.headers.is_empty());
    assert!(config.extras.is_empty());
}

// @tc.name: ut_config_builder_url
// @tc.desc: Test ConfigBuilder url method
// @tc.precon: NA
// @tc.step: 1. Create ConfigBuilder and set url
//           2. Build and verify url
// @tc.expect: url is set correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_config_builder_url() {
    let mut builder = ConfigBuilder::new();
    let config = builder.url("http://example.com/file.txt").build();

    assert_eq!(config.url, "http://example.com/file.txt");
}

// @tc.name: ut_config_builder_action
// @tc.desc: Test ConfigBuilder action method
// @tc.precon: NA
// @tc.step: 1. Create ConfigBuilder and set action
//           2. Build and verify action
// @tc.expect: action is set correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_config_builder_action() {
    let mut builder = ConfigBuilder::new();
    let config = builder.action(Action::Upload).build();

    assert_eq!(config.common_data.action, Action::Upload);
}

// @tc.name: ut_config_builder_mode
// @tc.desc: Test ConfigBuilder mode method
// @tc.precon: NA
// @tc.step: 1. Create ConfigBuilder and set mode
//           2. Build and verify mode
// @tc.expect: mode is set correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_config_builder_mode() {
    let mut builder = ConfigBuilder::new();
    let config = builder.mode(Mode::FrontEnd).build();

    assert_eq!(config.common_data.mode, Mode::FrontEnd);
}

// @tc.name: ut_config_builder_network
// @tc.desc: Test ConfigBuilder network method
// @tc.precon: NA
// @tc.step: 1. Create ConfigBuilder and set network
//           2. Build and verify network_config
// @tc.expect: network_config is set correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_config_builder_network() {
    let mut builder = ConfigBuilder::new();
    let config = builder.network(NetworkConfig::Wifi).build();

    assert_eq!(config.common_data.network_config, NetworkConfig::Wifi);
}

// @tc.name: ut_config_builder_roaming
// @tc.desc: Test ConfigBuilder roaming method
// @tc.precon: NA
// @tc.step: 1. Create ConfigBuilder and set roaming
//           2. Build and verify roaming
// @tc.expect: roaming is set correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_config_builder_roaming() {
    let mut builder = ConfigBuilder::new();
    let config = builder.roaming(true).build();

    assert!(config.common_data.roaming);
}

// @tc.name: ut_config_builder_metered
// @tc.desc: Test ConfigBuilder metered method
// @tc.precon: NA
// @tc.step: 1. Create ConfigBuilder and set metered
//           2. Build and verify metered
// @tc.expect: metered is set correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_config_builder_metered() {
    let mut builder = ConfigBuilder::new();
    let config = builder.metered(true).build();

    assert!(config.common_data.metered);
}

// @tc.name: ut_config_builder_redirect
// @tc.desc: Test ConfigBuilder redirect method
// @tc.precon: NA
// @tc.step: 1. Create ConfigBuilder and set redirect
//           2. Build and verify redirect
// @tc.expect: redirect is set correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_config_builder_redirect() {
    let mut builder = ConfigBuilder::new();
    let config = builder.redirect(false).build();

    assert!(!config.common_data.redirect);
}

// @tc.name: ut_config_builder_retry
// @tc.desc: Test ConfigBuilder retry method
// @tc.precon: NA
// @tc.step: 1. Create ConfigBuilder and set retry
//           2. Build and verify retry
// @tc.expect: retry is set correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_config_builder_retry() {
    let mut builder = ConfigBuilder::new();
    let config = builder.retry(true).build();

    assert!(config.common_data.retry);
}

// @tc.name: ut_config_builder_method
// @tc.desc: Test ConfigBuilder method method
// @tc.precon: NA
// @tc.step: 1. Create ConfigBuilder and set method
//           2. Build and verify method
// @tc.expect: method is set correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_config_builder_method() {
    let mut builder = ConfigBuilder::new();
    let config = builder.method("POST").build();

    assert_eq!(config.method, "POST");
}

// @tc.name: ut_config_builder_version
// @tc.desc: Test ConfigBuilder version method
// @tc.precon: NA
// @tc.step: 1. Create ConfigBuilder and set version
//           2. Build and verify version
// @tc.expect: version is set correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_config_builder_version() {
    let mut builder = ConfigBuilder::new();
    let config = builder.version(1).build();

    assert_eq!(config.version, Version::API9);
}

// @tc.name: ut_config_builder_bundle_name
// @tc.desc: Test ConfigBuilder bundle_name method
// @tc.precon: NA
// @tc.step: 1. Create ConfigBuilder and set bundle_name
//           2. Build and verify bundle
// @tc.expect: bundle is set correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_config_builder_bundle_name() {
    let mut builder = ConfigBuilder::new();
    let config = builder.bundle_name("com.example.app").build();

    assert_eq!(config.bundle, "com.example.app");
}

// @tc.name: ut_config_builder_uid
// @tc.desc: Test ConfigBuilder uid method
// @tc.precon: NA
// @tc.step: 1. Create ConfigBuilder and set uid
//           2. Build and verify uid
// @tc.expect: uid is set correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_config_builder_uid() {
    let mut builder = ConfigBuilder::new();
    let config = builder.uid(200000).build();

    assert_eq!(config.common_data.uid, 200000);
}

// @tc.name: ut_config_builder_begins
// @tc.desc: Test ConfigBuilder begins method
// @tc.precon: NA
// @tc.step: 1. Create ConfigBuilder and set begins
//           2. Build and verify begins
// @tc.expect: begins is set correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_config_builder_begins() {
    let mut builder = ConfigBuilder::new();
    let config = builder.begins(1234567890).build();

    assert_eq!(config.common_data.begins, 1234567890);
}

// @tc.name: ut_config_builder_ends
// @tc.desc: Test ConfigBuilder ends method
// @tc.precon: NA
// @tc.step: 1. Create ConfigBuilder and set ends
//           2. Build and verify ends
// @tc.expect: ends is set correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_config_builder_ends() {
    let mut builder = ConfigBuilder::new();
    let config = builder.ends(9999999999).build();

    assert_eq!(config.common_data.ends, 9999999999);
}

// @tc.name: ut_config_builder_chained
// @tc.desc: Test ConfigBuilder with chained method calls
// @tc.precon: NA
// @tc.step: 1. Create ConfigBuilder with multiple chained calls
//           2. Build and verify all values
// @tc.expect: All values are set correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_config_builder_chained() {
    let config = ConfigBuilder::new()
        .url("http://example.com")
        .action(Action::Upload)
        .mode(Mode::FrontEnd)
        .network(NetworkConfig::Wifi)
        .roaming(true)
        .metered(true)
        .redirect(false)
        .retry(true)
        .method("POST")
        .bundle_name("com.test.app")
        .uid(200000)
        .build();

    assert_eq!(config.url, "http://example.com");
    assert_eq!(config.common_data.action, Action::Upload);
    assert_eq!(config.common_data.mode, Mode::FrontEnd);
    assert_eq!(config.common_data.network_config, NetworkConfig::Wifi);
    assert!(config.common_data.roaming);
    assert!(config.common_data.metered);
    assert!(!config.common_data.redirect);
    assert!(config.common_data.retry);
    assert_eq!(config.method, "POST");
    assert_eq!(config.bundle, "com.test.app");
    assert_eq!(config.common_data.uid, 200000);
}

// @tc.name: ut_task_config_contains_user_file_false
// @tc.desc: Test contains_user_file returns false when no user files
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig with no user files
//           2. Call contains_user_file
// @tc.expect: Returns false
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_config_contains_user_file_false() {
    let config = TaskConfig::default();
    assert!(!config.contains_user_file());
}

// @tc.name: ut_task_config_build_config_set
// @tc.desc: Test build_config_set creates ConfigSet
// @tc.precon: NA
// @tc.step: 1. Create TaskConfig
//           2. Call build_config_set
// @tc.expect: Returns ConfigSet with correct values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_config_build_config_set() {
    let config = TaskConfig::default();
    let config_set = config.build_config_set();

    assert_eq!(config_set.headers, "{}");
    assert_eq!(config_set.extras, "{}");
    assert!(config_set.form_items.is_empty());
    assert!(config_set.file_specs.is_empty());
    assert!(config_set.body_file_names.is_empty());
    assert!(config_set.certs_path.is_empty());
}

// @tc.name: ut_mode_ord
// @tc.desc: Test Mode Ord trait implementation
// @tc.precon: NA
// @tc.step: 1. Compare different Mode values
// @tc.expect: FrontEnd < Any < BackGround
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_mode_ord() {
    assert!(Mode::FrontEnd < Mode::Any);
    assert!(Mode::Any < Mode::BackGround);
    assert!(Mode::FrontEnd < Mode::BackGround);
}

// @tc.name: ut_mode_from_u8
// @tc.desc: Test Mode From<u8> conversion
// @tc.precon: NA
// @tc.step: 1. Convert u8 values to Mode
// @tc.expect: Correct Mode values returned
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_mode_from_u8() {
    assert_eq!(Mode::from(0), Mode::BackGround);
    assert_eq!(Mode::from(1), Mode::FrontEnd);
    assert_eq!(Mode::from(2), Mode::Any);
    assert_eq!(Mode::from(255), Mode::Any);
}

// @tc.name: ut_action_from_u8
// @tc.desc: Test Action From<u8> conversion
// @tc.precon: NA
// @tc.step: 1. Convert u8 values to Action
// @tc.expect: Correct Action values returned
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_action_from_u8() {
    assert_eq!(Action::from(0), Action::Download);
    assert_eq!(Action::from(1), Action::Upload);
    assert_eq!(Action::from(2), Action::Any);
    assert_eq!(Action::from(255), Action::Any);
}
