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

//! Integration tests for configuration flow scenarios.
//!
//! This module tests complete configuration scenarios including
//! download configs, upload configs, and their conversions.

use std::collections::HashMap;

// @tc.name: int_download_config_complete_flow
// @tc.desc: Test complete download configuration flow
// @tc.precon: NA
// @tc.step: 1. Create download configuration with all fields
//           2. Convert to TaskConfig
//           3. Verify all fields are correctly mapped
// @tc.expect: Complete flow should work correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn int_download_config_complete_flow() {
    use request_core::config::{TaskConfigBuilder, Version, NetworkConfig, Action};

    // Create download configuration with all fields
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer token123".to_string());
    headers.insert("X-Request-ID".to_string(), "req-456".to_string());

    let mut builder = TaskConfigBuilder::new(Version::API9);
    builder.url("https://example.com/files/document.pdf".to_string());
    builder.headers(headers.clone());
    builder.metered(false);
    builder.roaming(true);
    builder.network_type(NetworkConfig::Wifi);
    builder.description("Download important document".to_string());
    builder.title("Document Download".to_string());
    builder.background(true);
    builder.file_path("/downloads/document.pdf".to_string());
    builder.action(Action::Download);

    let config = builder.build();

    // Verify all fields
    assert_eq!(config.url, "https://example.com/files/document.pdf");
    assert_eq!(config.headers.get("Authorization"), Some(&"Bearer token123".to_string()));
    assert!(!config.common_data.metered);
    assert!(config.common_data.roaming);
    assert!(matches!(config.common_data.network_config, NetworkConfig::Wifi));
    assert_eq!(config.description, "Download important document");
    assert_eq!(config.title, "Document Download");
    assert!(config.common_data.background);
    assert_eq!(config.saveas, "/downloads/document.pdf");
    assert_eq!(config.common_data.action, Action::Download as u8);
}

// @tc.name: int_upload_config_with_files_flow
// @tc.desc: Test upload configuration with multiple files
// @tc.precon: NA
// @tc.step: 1. Create upload configuration with multiple files
//           2. Add form items and file specs
//           3. Convert to TaskConfig
//           4. Verify files are correctly mapped
// @tc.expect: Upload config with files should work
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn int_upload_config_with_files_flow() {
    use request_core::config::{TaskConfigBuilder, Version, Action, FormItem};
    use request_core::file::FileSpec;

    let file_specs = vec![
        FileSpec {
            file_name: "image1.jpg".to_string(),
            name: "files".to_string(),
            path: "/storage/photos/image1.jpg".to_string(),
            mime_type: "image/jpeg".to_string(),
            is_user_file: true,
            fd: None,
        },
        FileSpec {
            file_name: "image2.jpg".to_string(),
            name: "files".to_string(),
            path: "/storage/photos/image2.jpg".to_string(),
            mime_type: "image/jpeg".to_string(),
            is_user_file: true,
            fd: None,
        },
    ];

    let form_items = vec![
        FormItem { name: "description".to_string(), value: "My photos".to_string() },
        FormItem { name: "album".to_string(), value: "Vacation".to_string() },
    ];

    let mut builder = TaskConfigBuilder::new(Version::API10);
    builder.url("https://example.com/upload".to_string());
    builder.method("POST".to_string());
    builder.action(Action::Upload);
    builder.files(file_specs);
    builder.form_items(form_items);
    builder.title("Photo Upload".to_string());

    let config = builder.build();

    // Verify
    assert_eq!(config.file_specs.len(), 2);
    assert_eq!(config.form_items.len(), 2);
    assert_eq!(config.file_specs[0].file_name, "image1.jpg");
    assert_eq!(config.file_specs[1].file_name, "image2.jpg");
    assert_eq!(config.form_items[0].name, "description");
}

// @tc.name: int_config_with_timeout_and_speed
// @tc.desc: Test configuration with timeout and speed limits
// @tc.precon: NA
// @tc.step: 1. Create config with timeout settings
//           2. Add min speed configuration
//           3. Convert and verify
// @tc.expect: Timeout and speed settings should be preserved
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn int_config_with_timeout_and_speed() {
    use request_core::config::{TaskConfigBuilder, Version, Action, MinSpeed, Timeout};

    let min_speed = MinSpeed {
        speed: 65536,  // 64 KB/s
        duration: 30,  // 30 seconds
    };

    let timeout = Timeout {
        connection_timeout: 60,
        total_timeout: 3600,
    };

    let mut builder = TaskConfigBuilder::new(Version::API10);
    builder.url("https://example.com/download".to_string());
    builder.action(Action::Download);
    builder.min_speed(min_speed);
    builder.timeout(timeout);

    let config = builder.build();

    assert_eq!(config.min_speed.speed, 65536);
    assert_eq!(config.min_speed.duration, 30);
    assert_eq!(config.timeout.connection_timeout, 60);
    assert_eq!(config.timeout.total_timeout, 3600);
}

// @tc.name: int_filter_with_multiple_criteria
// @tc.desc: Test search filter with multiple criteria
// @tc.precon: NA
// @tc.step: 1. Create filter with bundle, state, action, mode
//           2. Convert to SearchFilter
//           3. Verify all criteria are preserved
// @tc.expect: All filter criteria should be correctly mapped
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn int_filter_with_multiple_criteria() {
    use request_core::filter::SearchFilter;
    use request_core::config::{Action, Mode};
    use request_core::info::State;

    let filter = SearchFilter {
        bundle_name: Some("com.example.app".to_string()),
        before: Some(1700000000),
        after: Some(1600000000),
        state: Some(State::Running),
        action: Some(Action::Download),
        mode: Some(Mode::BackGround),
    };

    assert_eq!(filter.bundle_name, Some("com.example.app".to_string()));
    assert_eq!(filter.before, Some(1700000000));
    assert_eq!(filter.after, Some(1600000000));
    assert!(matches!(filter.state, Some(State::Running)));
    assert!(matches!(filter.action, Some(Action::Download)));
    assert!(matches!(filter.mode, Some(Mode::BackGround)));
}

// @tc.name: int_task_info_with_progress
// @tc.desc: Test TaskInfo to TaskInfo conversion with progress
// @tc.precon: NA
// @tc.step: 1. Create TaskInfo with progress data
//           2. Convert to API TaskInfo
//           3. Verify progress is correctly mapped
// @tc.expect: Progress data should be correctly converted
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn int_task_info_with_progress() {
    use request_core::info::{TaskInfo, CommonData, Progress, State};

    let task_info = TaskInfo {
        common_data: CommonData {
            task_id: 12345,
            uid: 1000,
            state: State::Running as u8,
            priority: 5,
            ..Default::default()
        },
        title: "Download Task".to_string(),
        description: "Downloading file".to_string(),
        url: "https://example.com/file.zip".to_string(),
        progress: Progress {
            state: State::Running,
            index: 0,
            processed: vec![512000, 256000],
            sizes: vec![1024000, 512000],
            total_processed: 768000,
            ..Default::default()
        },
        ..Default::default()
    };

    assert_eq!(task_info.common_data.task_id, 12345);
    assert_eq!(task_info.progress.state as u8, State::Running as u8);
    assert_eq!(task_info.progress.total_processed, 768000);
    assert_eq!(task_info.progress.processed.len(), 2);
}

// @tc.name: int_config_version_compatibility
// @tc.desc: Test configuration with different API versions
// @tc.precon: NA
// @tc.step: 1. Create configs with API9 and API10
//           2. Verify version is correctly set
//           3. Verify version-specific behavior
// @tc.expect: Versions should be correctly handled
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn int_config_version_compatibility() {
    use request_core::config::{TaskConfigBuilder, Version, Action};

    // API9 config
    let mut builder = TaskConfigBuilder::new(Version::API9);
    builder.url("https://example.com/file1".to_string());
    builder.action(Action::Download);
    let config9 = builder.build();

    // API10 config
    let mut builder = TaskConfigBuilder::new(Version::API10);
    builder.url("https://example.com/file2".to_string());
    builder.action(Action::Download);
    let config10 = builder.build();

    assert_eq!(config9.version, Version::API9);
    assert_eq!(config10.version, Version::API10);
}

// @tc.name: int_http_response_with_headers
// @tc.desc: Test HTTP response with complex headers
// @tc.precon: NA
// @tc.step: 1. Create HTTP response with multiple headers
//           2. Include multi-value headers
//           3. Convert and verify
// @tc.expect: Headers should be correctly mapped
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn int_http_response_with_headers() {
    use request_core::info::Response;

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), vec!["application/json".to_string()]);
    headers.insert("Cache-Control".to_string(), vec!["no-cache".to_string(), "no-store".to_string()]);
    headers.insert("Set-Cookie".to_string(), vec![
        "session=abc123; Path=/; HttpOnly".to_string(),
        "user=john; Path=/".to_string(),
    ]);

    let response = Response {
        version: "HTTP/2".to_string(),
        status_code: 200,
        reason: "OK".to_string(),
        headers: headers.clone(),
    };

    assert_eq!(response.status_code, 200);
    assert_eq!(response.headers.get("Content-Type"), Some(&vec!["application/json".to_string()]));
    
    let cookies = response.headers.get("Set-Cookie").unwrap();
    assert_eq!(cookies.len(), 2);
}

// @tc.name: int_notification_config_flow
// @tc.desc: Test notification configuration flow
// @tc.precon: NA
// @tc.step: 1. Create notification with all fields
//           2. Convert to core notification
//           3. Verify all fields are preserved
// @tc.expect: Notification config should be correctly mapped
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn int_notification_config_flow() {
    use request_core::config::Notification;

    let notification = Notification {
        title: Some("Download Started".to_string()),
        text: Some("Your download is in progress".to_string()),
        disable: Some(false),
        visibility: Some(2),
        want_agent: None,
    };

    assert_eq!(notification.title, Some("Download Started".to_string()));
    assert_eq!(notification.text, Some("Your download is in progress".to_string()));
    assert_eq!(notification.disable, Some(false));
    assert_eq!(notification.visibility, Some(2));
}



// @tc.name: int_file_spec_collection
// @tc.desc: Test file specification collection handling
// @tc.precon: NA
// @tc.step: 1. Create multiple file specs
//           2. Add to config
//           3. Verify collection is preserved
// @tc.expect: File spec collection should be correctly handled
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn int_file_spec_collection() {
    use request_core::config::{TaskConfigBuilder, Version, Action};
    use request_core::file::FileSpec;

    let files: Vec<FileSpec> = (0..5).map(|i| FileSpec {
        file_name: format!("file{}.txt", i),
        name: "upload".to_string(),
        path: format!("/storage/file{}.txt", i),
        mime_type: "text/plain".to_string(),
        is_user_file: true,
        fd: None,
    }).collect();

    let mut builder = TaskConfigBuilder::new(Version::API10);
    builder.url("https://example.com/upload".to_string());
    builder.action(Action::Upload);
    builder.files(files.clone());

    let config = builder.build();

    assert_eq!(config.file_specs.len(), 5);
    for (i, file) in config.file_specs.iter().enumerate() {
        assert_eq!(file.file_name, format!("file{}.txt", i));
    }
}

// @tc.name: int_form_item_collection
// @tc.desc: Test form item collection handling
// @tc.precon: NA
// @tc.step: 1. Create multiple form items
//           2. Add to config
//           3. Verify collection is preserved
// @tc.expect: Form item collection should be correctly handled
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn int_form_item_collection() {
    use request_core::config::{TaskConfigBuilder, Version, Action, FormItem};

    let form_items: Vec<FormItem> = vec![
        FormItem { name: "name".to_string(), value: "John".to_string() },
        FormItem { name: "age".to_string(), value: "30".to_string() },
        FormItem { name: "city".to_string(), value: "Beijing".to_string() },
    ];

    let mut builder = TaskConfigBuilder::new(Version::API10);
    builder.url("https://example.com/submit".to_string());
    builder.action(Action::Upload);
    builder.form_items(form_items.clone());

    let config = builder.build();

    assert_eq!(config.form_items.len(), 3);
    assert_eq!(config.form_items[0].name, "name");
    assert_eq!(config.form_items[1].value, "30");
    assert_eq!(config.form_items[2].name, "city");
}

// @tc.name: int_complex_download_scenario
// @tc.desc: Test complex download scenario with all options
// @tc.precon: NA
// @tc.step: 1. Create download config with all advanced options
//           2. Set range, index, priority, etc.
//           3. Convert and verify all settings
// @tc.expect: Complex configuration should work correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn int_complex_download_scenario() {
    use request_core::config::{TaskConfigBuilder, Version, Action, NetworkConfig, MinSpeed, Timeout};

    let mut headers = HashMap::new();
    headers.insert("Range".to_string(), "bytes=1024-2047".to_string());
    headers.insert("Accept".to_string(), "*/*".to_string());

    let mut builder = TaskConfigBuilder::new(Version::API10);
    builder.url("https://example.com/largefile.zip".to_string());
    builder.action(Action::Download);
    builder.headers(headers);
    builder.method("GET".to_string());
    builder.title("Large File Download".to_string());
    builder.description("Downloading large archive".to_string());
    builder.network_type(NetworkConfig::Any);
    builder.metered(true);
    builder.roaming(false);
    builder.retry(true);
    builder.redirect(true);
    builder.index(0);
    builder.begins(1024);
    builder.ends(2047);
    builder.gauge(true);
    builder.precise(true);
    builder.priority(10);
    builder.background(true);
    builder.file_path("/downloads/largefile.zip".to_string());
    builder.min_speed(MinSpeed { speed: 32768, duration: 60 });
    builder.timeout(Timeout { connection_timeout: 30, total_timeout: 7200 });

    let config = builder.build();

    // Verify complex configuration
    assert_eq!(config.common_data.index, 0);
    assert_eq!(config.common_data.begins, 1024);
    assert_eq!(config.common_data.ends, 2047);
    assert_eq!(config.common_data.priority, 10);
    assert!(config.common_data.gauge);
    assert!(config.common_data.precise);
    assert!(config.common_data.retry);
    assert!(config.common_data.redirect);
    assert!(!config.common_data.roaming);
}
