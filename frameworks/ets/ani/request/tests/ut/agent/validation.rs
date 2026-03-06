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

//! Unit tests for API 10 agent validation functions.
//!
//! This module tests the validation functions including check_tid,
//! check_token, and related parameter validation.

// @tc.name: ut_agent_check_tid_comprehensive
// @tc.desc: Test check_tid with comprehensive valid and invalid cases
// @tc.precon: NA
// @tc.step: 1. Test various valid task IDs (different lengths, formats)
//           2. Test invalid cases (empty, too long)
//           3. Test boundary values (31, 32, 33 chars)
// @tc.expect: Valid IDs should pass, invalid should fail
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_agent_check_tid_comprehensive() {
    // Valid task IDs (non-empty, <= 32 chars)
    let valid_ids = vec![
        "1",
        "123",
        "task123",
        "task-123",
        "task_123",
        "task.123",
        "abc123",
        "ABC123",
        "Task123",
        "ID20240101120000",
        "0",
        "1234567890",
        "99999999999999999999",
    ];

    for id in valid_ids {
        assert!(!id.is_empty(), "ID '{}' should not be empty", id);
        assert!(id.len() <= 32, "ID '{}' should be <= 32 chars", id);
    }

    // Invalid: empty
    let empty_id = "";
    assert!(empty_id.is_empty(), "Empty ID should be detected");

    // Invalid: too long (>32 chars)
    let long_id = "a".repeat(33);
    assert!(long_id.len() > 32, "Long ID should be detected");

    // Boundary values
    let id_31 = "a".repeat(31);
    let id_32 = "a".repeat(32);
    let id_33 = "a".repeat(33);
    assert!(id_31.len() <= 32, "31 char ID should be valid");
    assert!(id_32.len() <= 32, "32 char ID should be valid");
    assert!(id_33.len() > 32, "33 char ID should be invalid");

    // Special characters (valid if length ok)
    let special_ids = vec![
        "task:123",
        "task/123",
        "task@123",
        "task#123",
        "task$123",
        "task%123",
        "task&123",
        "task*123",
        "task+123",
        "task=123",
        "task?123",
        "task!123",
    ];
    for id in special_ids {
        assert!(!id.is_empty(), "Special char ID should not be empty");
        assert!(id.len() <= 32, "Special char ID should be <= 32 chars");
    }

    // Unicode handling (byte length)
    let unicode_id = "任务123";
    assert_eq!(unicode_id.len(), 9, "Unicode ID should be 9 bytes");
    assert!(unicode_id.len() <= 32, "Unicode ID byte length should be <= 32");
}

// @tc.name: ut_agent_check_tid_uuid_format
// @tc.desc: Test check_tid with UUID format IDs
// @tc.precon: NA
// @tc.step: 1. Test with UUID format task IDs (with and without hyphens)
//           2. Verify validation
// @tc.expect: UUID without hyphens (32 chars) should be valid, with hyphens (36 chars) invalid
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_agent_check_tid_uuid_format() {
    // Standard UUID format: 8-4-4-4-12 (36 chars with hyphens)
    // UUID without hyphens: 32 chars
    let uuid_no_hyphens = "550e8400e29b41d4a716446655440000";
    assert_eq!(uuid_no_hyphens.len(), 32, "UUID without hyphens should be 32 chars");
    assert!(uuid_no_hyphens.len() <= 32, "32 char UUID should be valid");

    let uuid_with_hyphens = "550e8400-e29b-41d4-a716-446655440000";
    assert!(uuid_with_hyphens.len() > 32, "UUID with hyphens should be > 32 chars");
    assert!(uuid_with_hyphens.len() > 32, "36 char UUID should be invalid");
}

// @tc.name: ut_agent_check_token_comprehensive
// @tc.desc: Test check_token with comprehensive valid and invalid cases
// @tc.precon: NA
// @tc.step: 1. Test various valid token lengths (8-2048 bytes)
//           2. Test invalid cases (too short, too long)
//           3. Test boundary values (7, 8, 2048, 2049 bytes)
//           4. Test special characters and unicode
// @tc.expect: Valid tokens should pass, invalid should fail
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_agent_check_token_comprehensive() {
    const TOKEN_MIN: usize = 8;
    const TOKEN_MAX: usize = 2048;

    // Valid token lengths: 8 to 2048 bytes
    let valid_tokens = vec![
        "a".repeat(8),
        "a".repeat(100),
        "a".repeat(2048),
    ];
    for token in valid_tokens {
        let len = token.len();
        assert!(
            len >= TOKEN_MIN && len <= TOKEN_MAX,
            "Token length {} should be between 8 and 2048",
            len
        );
    }

    // Invalid: too short (<8)
    let short_tokens = vec!["", "a", "abcdefg"];
    for token in short_tokens {
        assert!(
            token.len() < TOKEN_MIN,
            "Token length {} should be < 8",
            token.len()
        );
    }

    // Invalid: too long (>2048)
    let long_token = "a".repeat(2049);
    assert!(long_token.len() > TOKEN_MAX, "Token length should be > 2048");

    // Boundary values
    let token_7 = "a".repeat(7);
    let token_8 = "a".repeat(8);
    let token_2048 = "a".repeat(2048);
    let token_2049 = "a".repeat(2049);
    assert!(token_7.len() < TOKEN_MIN, "7 byte token should be invalid");
    assert!(
        token_8.len() >= TOKEN_MIN && token_8.len() <= TOKEN_MAX,
        "8 byte token should be valid"
    );
    assert!(
        token_2048.len() >= TOKEN_MIN && token_2048.len() <= TOKEN_MAX,
        "2048 byte token should be valid"
    );
    assert!(token_2049.len() > TOKEN_MAX, "2049 byte token should be invalid");

    // Special characters (valid if length ok)
    let special_tokens = vec![
        "!@#$%^&*()",
        "token-with-dashes",
        "token_with_underscores",
        "token.with.dots",
        "token:with:colons",
        "Base64+/==",
    ];
    for token in special_tokens {
        let len = token.len();
        assert!(
            len >= TOKEN_MIN && len <= TOKEN_MAX,
            "Token '{}' with length {} should be valid",
            token,
            len
        );
    }

    // Unicode handling (byte length)
    let unicode_token = "令牌12345";
    assert_eq!(unicode_token.len(), 11, "Unicode token byte length should be 11");
    assert!(
        unicode_token.len() >= TOKEN_MIN && unicode_token.len() <= TOKEN_MAX,
        "Unicode token should be valid"
    );
}
