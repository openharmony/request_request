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

use request_core::config::{Notification, TaskConfig, TaskConfigBuilder, Version};
use request_client::verify::{notification::NotificationVerifier, ConfigVerifier};

fn create_config_with_notification(version: Version, notification: Notification) -> TaskConfig {
    let mut config = TaskConfigBuilder::new(version)
        .url("https://example.com/test".to_string())
        .build();
    config.notification = notification;
    config
}

// @tc.name: ut_notification_verifier_api9_with_notification
// @tc.desc: Test NotificationVerifier with API9 version ignores notification validation
// @tc.precon: NA
// @tc.step: 1. Create NotificationVerifier
//           2. Create TaskConfig with API9 version and notification with long title
//           3. Verify config passes validation
// @tc.expect: Verification passes for API9 with any notification
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_notification_verifier_api9_with_notification() {
    let verifier = NotificationVerifier {};
    let notification = Notification {
        title: Some("a".repeat(2000)),
        text: None,
        disable: None,
        visibility: Some(0),
        want_agent: None,
    };
    let config = create_config_with_notification(Version::API9, notification);
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_notification_verifier_api10_empty_notification
// @tc.desc: Test NotificationVerifier with API10 and empty notification
// @tc.precon: NA
// @tc.step: 1. Create NotificationVerifier
//           2. Create TaskConfig with API10 version and empty notification
//           3. Verify config passes validation
// @tc.expect: Verification passes for API10 with empty notification
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_notification_verifier_api10_empty_notification() {
    let verifier = NotificationVerifier {};
    let notification = Notification {
        title: None,
        text: None,
        disable: None,
        visibility: None,
        want_agent: None,
    };
    let config = create_config_with_notification(Version::API10, notification);
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_notification_verifier_api10_valid_title
// @tc.desc: Test NotificationVerifier with API10 and valid title
// @tc.precon: NA
// @tc.step: 1. Create NotificationVerifier
//           2. Create TaskConfig with API10 version and notification with valid title
//           3. Verify config passes validation
// @tc.expect: Verification passes for API10 with valid title
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_notification_verifier_api10_valid_title() {
    let verifier = NotificationVerifier {};
    let notification = Notification {
        title: Some("Download Notification".to_string()),
        text: None,
        disable: None,
        visibility: None,
        want_agent: None,
    };
    let config = create_config_with_notification(Version::API10, notification);
    assert!(verifier.verify(&config).is_ok());
}

// @tc.name: ut_notification_verifier_api10_title_exceed_max
// @tc.desc: Test NotificationVerifier with API10 and title exceeding max length
// @tc.precon: NA
// @tc.step: 1. Create NotificationVerifier
//           2. Create TaskConfig with API10 version and notification with title exceeding max length
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_notification_verifier_api10_title_exceed_max() {
    let verifier = NotificationVerifier {};
    let notification = Notification {
        title: Some("a".repeat(1025)),
        text: None,
        disable: None,
        visibility: None,
        want_agent: None,
    };
    let config = create_config_with_notification(Version::API10, notification);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_notification_verifier_api10_text_exceed_max
// @tc.desc: Test NotificationVerifier with API10 and text exceeding max length
// @tc.precon: NA
// @tc.step: 1. Create NotificationVerifier
//           2. Create TaskConfig with API10 version and notification with text exceeding max length
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_notification_verifier_api10_text_exceed_max() {
    let verifier = NotificationVerifier {};
    let notification = Notification {
        title: None,
        text: Some("a".repeat(3073)),
        disable: None,
        visibility: None,
        want_agent: None,
    };
    let config = create_config_with_notification(Version::API10, notification);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_notification_verifier_api10_empty_want_agent
// @tc.desc: Test NotificationVerifier with API10 and empty want_agent
// @tc.precon: NA
// @tc.step: 1. Create NotificationVerifier
//           2. Create TaskConfig with API10 version and notification with empty want_agent
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_notification_verifier_api10_empty_want_agent() {
    let verifier = NotificationVerifier {};
    let notification = Notification {
        title: None,
        text: None,
        disable: None,
        visibility: None,
        want_agent: Some("".to_string()),
    };
    let config = create_config_with_notification(Version::API10, notification);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_notification_verifier_api10_visibility_zero
// @tc.desc: Test NotificationVerifier with API10 and visibility=0
// @tc.precon: NA
// @tc.step: 1. Create NotificationVerifier
//           2. Create TaskConfig with API10 version and notification with visibility=0
//           3. Verify config fails validation
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_notification_verifier_api10_visibility_zero() {
    let verifier = NotificationVerifier {};
    let notification = Notification {
        title: None,
        text: None,
        disable: None,
        visibility: Some(0),
        want_agent: None,
    };
    let config = create_config_with_notification(Version::API10, notification);
    let result = verifier.verify(&config);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 401);
}

// @tc.name: ut_notification_verifier_api10_visibility_valid
// @tc.desc: Test NotificationVerifier with API10 and valid visibility values
// @tc.precon: NA
// @tc.step: 1. Create NotificationVerifier
//           2. Create TaskConfig with API10 version and notification with visibility 1, 2, 3
//           3. Verify config passes validation for each
// @tc.expect: Verification passes for valid visibility values
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_notification_verifier_api10_visibility_valid() {
    let verifier = NotificationVerifier {};
    for vis in [1, 2, 3] {
        let notification = Notification {
            title: None,
            text: None,
            disable: None,
            visibility: Some(vis),
            want_agent: None,
        };
        let config = create_config_with_notification(Version::API10, notification);
        assert!(verifier.verify(&config).is_ok());
    }
}

// @tc.name: ut_notification_verifier_api10_visibility_invalid
// @tc.desc: Test NotificationVerifier with API10 and invalid visibility values
// @tc.precon: NA
// @tc.step: 1. Create NotificationVerifier
//           2. Create TaskConfig with API10 version and notification with visibility 4, 5, 7
//           3. Verify config fails validation for each
// @tc.expect: Verification fails with error code 401
// @tc.type: FUNC
// @tc.require: issueNumber
#[test]
fn ut_notification_verifier_api10_visibility_invalid() {
    let verifier = NotificationVerifier {};
    for vis in [4, 5, 7] {
        let notification = Notification {
            title: None,
            text: None,
            disable: None,
            visibility: Some(vis),
            want_agent: None,
        };
        let config = create_config_with_notification(Version::API10, notification);
        let result = verifier.verify(&config);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), 401);
    }
}
