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

#[cfg(test)]
mod ut_log {
    use super::*;
    use std::env;

    // @tc.name: ut_log_init_non_ohos_feature
    // @tc.desc: Test log initialization when ohos feature is not enabled
    // @tc.precon: NA
    // @tc.step: 1. Set RUST_LOG environment variable
    // 2. Call init() function
    // 3. Verify logger was initialized
    // @tc.expect: Logger initializes successfully without errors
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_log_init_non_ohos_feature() {
        env::set_var("RUST_LOG", "debug");
        init();
        // Verify by checking if logger is initialized
        // Since env_logger doesn't provide a direct check, we'll log a message
        log::debug!("Test log message");
    }

    // @tc.name: ut_log_init_ohos_feature
    // @tc.desc: Test log initialization when ohos feature is enabled
    // @tc.precon: NA
    // @tc.step: 1. Call init() function with ohos feature
    // @tc.expect: Empty function executes without errors
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[cfg(feature = "ohos")]
    #[test]
    fn ut_log_init_ohos_feature() {
        init();
    }

    // @tc.name: ut_log_init_multiple_calls
    // @tc.desc: Test multiple init() calls don't cause issues
    // @tc.precon: NA
    // @tc.step: 1. Call init() function
    // 2. Call init() function again
    // @tc.expect: Second call doesn't panic or cause errors
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 2
    #[test]
    fn ut_log_init_multiple_calls() {
        init();
        init(); // Should not panic
    }
}