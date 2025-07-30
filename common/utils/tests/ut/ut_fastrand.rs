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
mod ut_fastrand {
    use super::*;
    use std::thread;

    // @tc.name: ut_fast_random_non_zero
    // @tc.desc: Verify fast_random returns non-zero value
    // @tc.precon: NA
    // @tc.step: 1. Call fast_random() function
    // @tc.expect: Returned value is non-zero u64
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 0
    #[test]
    fn ut_fast_random_non_zero() {
        let result = fast_random();
        assert_ne!(result, 0, "Random value should not be zero");
    }

    // @tc.name: ut_fast_random_consecutive_different
    // @tc.desc: Verify consecutive calls produce different values
    // @tc.precon: NA
    // @tc.step: 1. Call fast_random() twice
    // 2. Compare the two results
    // @tc.expect: Two different u64 values are returned
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 1
    #[test]
    fn ut_fast_random_consecutive_different() {
        let first = fast_random();
        let second = fast_random();
        assert_ne!(first, second, "Consecutive calls should return different values");
    }

    // @tc.name: ut_fast_random_thread_isolation
    // @tc.desc: Verify thread-local RNG isolation
    // @tc.precon: NA
    // @tc.step: 1. Spawn two threads
    // 2. Each thread calls fast_random() once
    // 3. Compare results from both threads
    // @tc.expect: Different values from each thread's RNG
    // @tc.type: FUNC
    // @tc.require: issueNumber
    // @tc.level: Level 3
    #[test]
    fn ut_fast_random_thread_isolation() {
        let handle1 = thread::spawn(|| fast_random());
        let handle2 = thread::spawn(|| fast_random());

        let result1 = handle1.join().unwrap();
        let result2 = handle2.join().unwrap();

        assert_ne!(result1, result2, "Thread-local RNGs should produce different sequences");
    }
}