// Copyright (C) 2024 Huawei Device Co., Ltd.
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

#[cfg(test)]
mod ut_url {
    use super::*;

    #[cfg(not(feature = "ohos"))]
    mod default_hash_tests {
        use super::*;
        use std::collections::HashSet;

        // @tc.name: ut_url_hash_default_basic
        // @tc.desc: Test basic functionality of url_hash with DefaultHasher
        // @tc.precon: NA
        // @tc.step: 1. Call url_hash with "https://example.com"
        // 2. Call url_hash again with the same input
        // @tc.expect: Both calls return the same hash value
        // @tc.type: FUNC
        // @tc.require: issueNumber
        // @tc.level: Level 0
        #[test]
        fn ut_url_hash_default_basic() {
            let input = "https://example.com";
            let result1 = url_hash(input);
            let result2 = url_hash(input);
            assert_eq!(result1, result2);
        }

        // @tc.name: ut_url_hash_default_different_inputs
        // @tc.desc: Test url_hash with different inputs produce different hashes
        // @tc.precon: NA
        // @tc.step: 1. Call url_hash with "https://example.com"
        // 2. Call url_hash with "https://example.org"
        // @tc.expect: Different hash values are returned
        // @tc.type: FUNC
        // @tc.require: issueNumber
        // @tc.level: Level 1
        #[test]
        fn ut_url_hash_default_different_inputs() {
            let result1 = url_hash("https://example.com");
            let result2 = url_hash("https://example.org");
            assert_ne!(result1, result2);
        }

        // @tc.name: ut_url_hash_default_empty_string
        // @tc.desc: Test url_hash with empty string input
        // @tc.precon: NA
        // @tc.step: 1. Call url_hash with empty string
        // @tc.expect: Non-empty hash string is returned
        // @tc.type: FUNC
        // @tc.require: issueNumber
        // @tc.level: Level 2
        #[test]
        fn ut_url_hash_default_empty_string() {
            let result = url_hash("");
            assert!(!result.is_empty());
        }

        // @tc.name: ut_url_hash_default_special_characters
        // @tc.desc: Test url_hash with special characters in URL
        // @tc.precon: NA
        // @tc.step: 1. Call url_hash with "https://example.com/path?query=123#fragment"
        // @tc.expect: Valid hash is generated without panic
        // @tc.type: FUNC
        // @tc.require: issueNumber
        // @tc.level: Level 2
        #[test]
        fn ut_url_hash_default_special_characters() {
            let input = "https://example.com/path?query=123#fragment";
            let result = url_hash(input);
            assert!(!result.is_empty());
        }

        // @tc.name: ut_url_hash_default_long_url
        // @tc.desc: Test url_hash with very long URL input
        // @tc.precon: NA
        // @tc.step: 1. Create a long URL string with 10000 characters
        // 2. Call url_hash with this long URL
        // @tc.expect: Valid hash is generated without panic or memory issues
        // @tc.type: FUNC
        // @tc.require: issueNumber
        // @tc.level: Level 2
        #[test]
        fn ut_url_hash_default_long_url() {
            let long_url = "a".repeat(10000);
            let result = url_hash(&long_url);
            assert!(!result.is_empty());
        }

        // @tc.name: ut_url_hash_default_collision_resistance
        // @tc.desc: Test basic collision resistance of DefaultHasher
        // @tc.precon: NA
        // @tc.step: 1. Generate hashes for 1000 different URLs
        // 2. Check for hash collisions
        // @tc.expect: No collisions occur among generated hashes
        // @tc.type: FUNC
        // @tc.require: issueNumber
        // @tc.level: Level 3
        #[test]
        fn ut_url_hash_default_collision_resistance() {
            let mut hashes = HashSet::new();
            for i in 0..1000 {
                let url = format!("https://example.com/{}", i);
                let hash = url_hash(&url);
                assert!(hashes.insert(hash), "Collision detected at i = {}", i);
            }
        }
    }

    #[cfg(feature = "ohos")]
    mod ohos_hash_tests {
        use super::*;

        // @tc.name: ut_url_hash_ohos_basic
        // @tc.desc: Test basic functionality of SHA256 url_hash
        // @tc.precon: NA
        // @tc.step: 1. Call url_hash with "https://example.com"
        // 2. Verify the returned hash matches known SHA256 value
        // @tc.expect: Hash matches the SHA256 digest of the input string
        // @tc.type: FUNC
        // @tc.require: issueNumber
        // @tc.level: Level 0
        #[test]
        fn ut_url_hash_ohos_basic() {
            let input = "https://example.com";
            let expected_hash = "5f83c7b78d243907e45a3b7b68773841a8b955f6e4d9f2147133f8a000d6b3d0";
            assert_eq!(url_hash(input), expected_hash);
        }

        // @tc.name: ut_url_hash_ohos_empty_string
        // @tc.desc: Test url_hash with empty string input for SHA256
        // @tc.precon: NA
        // @tc.step: 1. Call url_hash with empty string
        // @tc.expect: Hash matches the known SHA256 digest of empty string
        // @tc.type: FUNC
        // @tc.require: issueNumber
        // @tc.level: Level 2
        #[test]
        fn ut_url_hash_ohos_empty_string() {
            let expected_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
            assert_eq!(url_hash(""), expected_hash);
        }
    }
}