// Copyright (c) 2023 Huawei Device Co., Ltd.
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
mod ut_context {
    use super::*;
    use mockall::automock;
    use mockall::Sequence;

    pub mod wrapper {
        pub static mut GetCacheDir: fn() -> String = || String::new();
    }
    // Mock the external GetCacheDir function
    #[automock]
    trait CacheDirProvider {
        fn get_cache_dir() -> String;
    }

    // @tc.name: ut_get_cache_dir_non_empty
    // @tc.desc: Test get_cache_dir returns Some when cache directory exists
    // @tc.precon: NA
    // @tc.step: 1. Mock GetCacheDir to return non-empty string
    // 2. Call get_cache_dir() function
    // 3. Verify returned value is Some with expected directory
    // @tc.expect: get_cache_dir returns Some("/test/cache/dir")
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_get_cache_dir_non_empty() {
        let expected_dir = String::from("/test/cache/dir");
        super::wrapper::GetCacheDir = || expected_dir.clone();

        let result = get_cache_dir();
        assert_eq!(result, Some(expected_dir));
    }

    // @tc.name: ut_get_cache_dir_empty
    // @tc.desc: Test get_cache_dir returns None when cache directory is empty
    // @tc.precon: NA
    // @tc.step: 1. Mock GetCacheDir to return empty string
    // 2. Call get_cache_dir() function
    // 3. Verify returned value is None
    // @tc.expect: get_cache_dir returns None
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_get_cache_dir_empty() {
        super::wrapper::GetCacheDir = || String::new();

        let result = get_cache_dir();
        assert_eq!(result, None);
    }

    // @tc.name: ut_get_cache_dir_whitespace
    // @tc.desc: Test get_cache_dir handles whitespace-only directory
    // @tc.precon: NA
    // @tc.step: 1. Mock GetCacheDir to return whitespace string
    // 2. Call get_cache_dir() function
    // 3. Verify returned value is None
    // @tc.expect: Whitespace-only directory is treated as empty and returns None
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 3
    #[test]
    fn ut_get_cache_dir_whitespace() {
        super::wrapper::GetCacheDir = || String::from("   ");

        let result = get_cache_dir();
        assert_eq!(result, None);
    }
}
