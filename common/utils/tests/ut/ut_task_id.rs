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
mod ut_task_id {
    use super::*;

    // @tc.name: ut_task_id_new_basic
    // @tc.desc: Test basic creation of TaskId with new()
    // @tc.precon: NA
    // @tc.step: 1. Create TaskId with new("test_hash".to_string())
    // 2. Create another TaskId with same hash
    // 3. Compare the two TaskId instances
    // @tc.expect: Both TaskId instances are equal
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_task_id_new_basic() {
        let task1 = TaskId::new("test_hash".to_string());
        let task2 = TaskId::new("test_hash".to_string());
        assert_eq!(task1, task2);
    }

    // @tc.name: ut_task_id_clone
    // @tc.desc: Verify Clone trait implementation
    // @tc.precon: NA
    // @tc.step: 1. Create TaskId instance
    // 2. Clone the instance
    // 3. Compare original and cloned instances
    // @tc.expect: Cloned TaskId is equal to original
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_task_id_clone() {
        let original = TaskId::new("clone_test".to_string());
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    // @tc.name: ut_task_id_from_url_non_empty
    // @tc.desc: Verify from_url generates non-empty hash
    // @tc.precon: NA
    // @tc.step: 1. Call from_url with valid URL
    // 2. Check the generated hash is non-empty
    // @tc.expect: TaskId contains non-empty hash string
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_task_id_from_url_non_empty() {
        let task_id = TaskId::from_url("https://example.com");
        assert!(!task_id.to_string().is_empty());
    }

    // @tc.name: ut_task_id_brief_normal_case
    // @tc.desc: Test brief() with standard hash length
    // @tc.precon: NA
    // @tc.step: 1. Create TaskId with 8-character hash
    // 2. Call brief() method
    // @tc.expect: Returns first 2 characters of hash
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_task_id_brief_normal_case() {
        let task_id = TaskId::new("12345678".to_string());
        assert_eq!(task_id.brief(), "12");
    }

    // @tc.name: ut_task_id_brief_short_hash
    // @tc.desc: Test brief() with hash length less than 4
    // @tc.precon: NA
    // @tc.step: 1. Create TaskId with 3-character hash
    // 2. Call brief() method
    // @tc.expect: Returns empty string without panic
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_task_id_brief_short_hash() {
        let task_id = TaskId::new("123".to_string());
        assert_eq!(task_id.brief(), "");
    }

    // @tc.name: ut_task_id_brief_empty_hash
    // @tc.desc: Test brief() with empty hash string
    // @tc.precon: NA
    // @tc.step: 1. Create TaskId with empty hash
    // 2. Call brief() method
    // @tc.expect: Returns empty string without panic
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_task_id_brief_empty_hash() {
        let task_id = TaskId::new(String::new());
        assert_eq!(task_id.brief(), "");
    }

    // @tc.name: ut_task_id_brief_exact_4_chars
    // @tc.desc: Test brief() with exactly 4-character hash
    // @tc.precon: NA
    // @tc.step: 1. Create TaskId with 4-character hash
    // 2. Call brief() method
    // @tc.expect: Returns first 1 character of hash
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_task_id_brief_exact_4_chars() {
        let task_id = TaskId::new("1234".to_string());
        assert_eq!(task_id.brief(), "1");
    }

    // @tc.name: ut_task_id_brief_non_multiple_of_four
    // @tc.desc: Test brief() with hash length not divisible by 4
    // @tc.precon: NA
    // @tc.step: 1. Create TaskId with 5-character hash
    // 2. Call brief() method
    // @tc.expect: Returns first 1 character of hash (5/4=1)
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_task_id_brief_non_multiple_of_four() {
        let task_id = TaskId::new("12345".to_string());
        assert_eq!(task_id.brief(), "1");
    }

    // @tc.name: ut_task_id_display_trait
    // @tc.desc: Verify Display trait implementation
    // @tc.precon: NA
    // @tc.step: 1. Create TaskId with known hash
    // 2. Convert to string using to_string()
    // @tc.expect: String matches original hash
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_task_id_display_trait() {
        let expected_hash = "display_test_hash".to_string();
        let task_id = TaskId::new(expected_hash.clone());
        assert_eq!(task_id.to_string(), expected_hash);
    }

    // @tc.name: ut_task_id_hash_consistency
    // @tc.desc: Verify Hash trait consistency for equal TaskIds
    // @tc.precon: NA
    // @tc.step: 1. Create two equal TaskId instances
    // 2. Insert both into a HashMap
    // 3. Verify HashMap contains only one entry
    // @tc.expect: HashMap has single entry with count 2
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 3
    #[test]
    fn ut_task_id_hash_consistency() {
        use std::collections::HashMap;

        let task1 = TaskId::new("hash_test".to_string());
        let task2 = TaskId::new("hash_test".to_string());

        let mut map = HashMap::new();
        *map.entry(task1).or_insert(0) += 1;
        *map.entry(task2).or_insert(0) += 1;

        assert_eq!(map.len(), 1, "HashMap should contain one entry");
        assert_eq!(map.values().next(), Some(&2), "Entry count should be 2");
    }

    // @tc.name: ut_task_id_long_hash
    // @tc.desc: Test TaskId with very long hash string
    // @tc.precon: NA
    // @tc.step: 1. Create TaskId with 1000-character hash
    // 2. Call brief() method
    // 3. Verify brief returns first 250 characters
    // @tc.expect: brief() returns first 250 characters of hash
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 3
    #[test]
    fn ut_task_id_long_hash() {
        let long_hash = "a".repeat(1000);
        let task_id = TaskId::new(long_hash.clone());
        assert_eq!(task_id.brief(), &long_hash[..250]);
    }

    // @tc.name: ut_task_id_from_url_empty
    // @tc.desc: Test from_url with empty URL string
    // @tc.precon: NA
    // @tc.step: 1. Call from_url with empty string
    // 2. Check the generated hash
    // @tc.expect: TaskId contains consistent hash for empty input
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_task_id_from_url_empty() {
        let task_id = TaskId::from_url("");
        let result = task_id.to_string();
        // Verify consistent result for empty input
        assert_eq!(result, TaskId::from_url("").to_string());
    }

    // @tc.name: ut_task_id_from_url_same_consistent
    // @tc.desc: Verify same URL produces consistent hash
    // @tc.precon: NA
    // @tc.step: 1. Create two TaskIds from same URL
    // 2. Compare their string representations
    // @tc.expect: Both TaskIds have identical hash strings
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_task_id_from_url_same_consistent() {
        let task1 = TaskId::from_url("https://example.com");
        let task2 = TaskId::from_url("https://example.com");
        assert_eq!(task1.to_string(), task2.to_string());
    }

    // @tc.name: ut_task_id_from_url_special_chars
    // @tc.desc: Test from_url with URL containing special characters
    // @tc.precon: NA
    // @tc.step: 1. Call from_url with URL containing special characters
    // 2. Check the generated hash is non-empty
    // @tc.expect: TaskId contains non-empty hash string
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_task_id_from_url_special_chars() {
        let task_id = TaskId::from_url("https://example.com/path?query=123#fragment");
        assert!(!task_id.to_string().is_empty());
    }

    // @tc.name: ut_task_id_from_url_different_urls
    // @tc.desc: Verify different URLs generate different TaskIds
    // @tc.precon: NA
    // @tc.step: 1. Create two TaskIds from different URLs
    // 2. Compare the two TaskId instances
    // @tc.expect: TaskId instances are not equal
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 3
    #[test]
    fn ut_task_id_from_url_different_urls() {
        let task1 = TaskId::from_url("https://example.com");
        let task2 = TaskId::from_url("https://example.org");
        assert_ne!(task1, task2);
    }
}