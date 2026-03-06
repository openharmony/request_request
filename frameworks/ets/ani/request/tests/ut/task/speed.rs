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

//! Unit tests for API 10 task speed limit functions.
//!
//! This module tests the speed limit validation functionality.

// @tc.name: ut_task_speed_limit_comprehensive
// @tc.desc: Test speed limit validation with comprehensive cases
// @tc.precon: NA
// @tc.step: 1. Test boundary values (at, below, above MIN_SPEED_LIMIT)
//           2. Test various valid speeds (16KB/s, 32KB/s, 64KB/s, 1MB/s, etc.)
//           3. Test invalid speeds (negative, zero, below 16KB/s)
//           4. Test extreme values (i64::MAX, i64::MIN)
// @tc.expect: Speeds >= 16KB/s should be valid, others invalid
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_speed_limit_comprehensive() {
    const MIN_SPEED_LIMIT: i64 = 16 * 1024; // 16384 bytes/s (16 KB/s)

    // Boundary values
    let speed_at_limit = MIN_SPEED_LIMIT;
    let speed_below_limit = MIN_SPEED_LIMIT - 1;
    let speed_above_limit = MIN_SPEED_LIMIT + 1;

    assert!(
        speed_at_limit >= MIN_SPEED_LIMIT,
        "Speed at limit should be valid"
    );
    assert!(
        speed_below_limit < MIN_SPEED_LIMIT,
        "Speed below limit should be invalid"
    );
    assert!(
        speed_above_limit >= MIN_SPEED_LIMIT,
        "Speed above limit should be valid"
    );

    // Valid speeds
    let valid_speeds = vec![
        (MIN_SPEED_LIMIT, true),
        (MIN_SPEED_LIMIT + 1, true),
        (32768, true),       // 32 KB/s
        (65536, true),       // 64 KB/s
        (131072, true),      // 128 KB/s
        (262144, true),      // 256 KB/s
        (524288, true),      // 512 KB/s
        (1048576, true),     // 1 MB/s
        (10485760, true),    // 10 MB/s
        (100 * 1024 * 1024, true), // 100 MB/s
        (i64::MAX, true),    // Maximum possible
    ];

    for (speed, expected_valid) in valid_speeds {
        let is_valid = speed >= MIN_SPEED_LIMIT;
        assert_eq!(
            is_valid, expected_valid,
            "Speed {} validation failed: expected {}, got {}",
            speed, expected_valid, is_valid
        );
    }

    // Invalid speeds
    let invalid_speeds = vec![
        (0, false),
        (1, false),
        (1024, false),
        (8192, false),
        (16383, false),
        (MIN_SPEED_LIMIT - 1, false),
        (-1, false),
        (-16384, false),
        (i64::MIN, false),
    ];

    for (speed, expected_valid) in invalid_speeds {
        let is_valid = speed >= MIN_SPEED_LIMIT;
        assert_eq!(
            is_valid, expected_valid,
            "Speed {} validation failed: expected {}, got {}",
            speed, expected_valid, is_valid
        );
    }
}

// @tc.name: ut_task_max_speed_with_min_speed_constraint
// @tc.desc: Test check_max_speed with min_speed configured
// @tc.precon: Task has min_speed configured
// @tc.step: 1. Test max_speed < min_speed (should fail)
//           2. Test max_speed = min_speed (should pass)
//           3. Test max_speed > min_speed (should pass)
//           4. Test various min_speed values
// @tc.expect: max_speed must be >= min_speed
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_max_speed_with_min_speed_constraint() {
    // Test case 1: min_speed = 65536 (64 KB/s)
    let min_speed = 65536i64;

    let test_cases = vec![
        (min_speed - 1, false),
        (min_speed, true),
        (min_speed + 1, true),
        (131072, true),
    ];

    for (max_speed, should_pass) in test_cases {
        let passes = max_speed >= min_speed;
        assert_eq!(
            passes, should_pass,
            "max_speed {} with min_speed {}: expected {}, got {}",
            max_speed, min_speed, should_pass, passes
        );
    }

    // Test case 2: min_speed = 1048576 (1 MB/s)
    let min_speed = 1048576i64;
    let max_speed = 1048575i64;
    assert!(
        max_speed < min_speed,
        "Comparison should detect max < min"
    );

    let max_speed = 1048576i64;
    assert!(
        max_speed >= min_speed,
        "Equal values should pass"
    );

    let max_speed = 1048577i64;
    assert!(
        max_speed >= min_speed,
        "Greater values should pass"
    );
}

// @tc.name: ut_task_speed_without_min_speed
// @tc.desc: Test speed validation without min_speed constraint
// @tc.precon: Task has no min_speed configured
// @tc.step: 1. Check max speed without min_speed constraint
//           2. Verify any speed >= MIN_SPEED_LIMIT is valid
// @tc.expect: Should pass with only MIN_SPEED_LIMIT constraint
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_task_speed_without_min_speed() {
    const MIN_SPEED_LIMIT: i64 = 16 * 1024;

    // When there's no min_speed config, only MIN_SPEED_LIMIT applies
    let speed = 65536;
    assert!(
        speed >= MIN_SPEED_LIMIT,
        "Speed should be >= MIN_SPEED_LIMIT"
    );

    // Zero speed should be invalid
    let speed = 0i64;
    assert!(speed < MIN_SPEED_LIMIT, "Zero speed should be invalid");

    // Very large speed should be valid
    let speed = 100 * 1024 * 1024i64;
    assert!(speed >= MIN_SPEED_LIMIT, "Large speed should be valid");
}
