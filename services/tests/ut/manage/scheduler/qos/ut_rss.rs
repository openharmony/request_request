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

fn is_rss_equal(rss1: RssCapacity, rss2: RssCapacity) -> bool {
    rss1.m1() == rss2.m1()
        && rss1.m2() == rss2.m2()
        && rss1.m3() == rss2.m3()
        && rss1.m1_speed() == rss2.m1_speed()
        && rss1.m2_speed() == rss2.m2_speed()
        && rss1.m2_speed() == rss2.m2_speed()
}

// @tc.name: ut_rss_capacity
// @tc.desc: Test RSS capacity initialization with different parameters
// @tc.precon: NA
// @tc.step: 1. Verify QosLevel enum values
//           2. Check RssCapacity initialization for different parameters
//           3. Compare initialized RSS values with expected results
// @tc.expect: RssCapacity is initialized with correct values for each parameter
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_rss_capacity() {
    assert_eq!(QosLevel::High as u64, 0u64);
    assert_eq!(QosLevel::Middle as u64, 800 * 1024u64);
    assert_eq!(QosLevel::Low as u64, 400 * 1024u64);
    assert!(is_rss_equal(
        RssCapacity::new(0),
        RssCapacity(8, 32, 8, QosLevel::High, QosLevel::Middle, QosLevel::Middle,)
    ));
    assert!(is_rss_equal(
        RssCapacity::new(1),
        RssCapacity(8, 32, 8, QosLevel::High, QosLevel::Middle, QosLevel::Middle,)
    ));
    assert!(is_rss_equal(
        RssCapacity::new(2),
        RssCapacity(8, 32, 8, QosLevel::High, QosLevel::Middle, QosLevel::Middle,)
    ));
    assert!(is_rss_equal(
        RssCapacity::new(3),
        RssCapacity(8, 16, 4, QosLevel::High, QosLevel::Middle, QosLevel::Middle,)
    ));
    assert!(is_rss_equal(
        RssCapacity::new(4),
        RssCapacity(4, 16, 4, QosLevel::High, QosLevel::Middle, QosLevel::Middle,)
    ));
    assert!(is_rss_equal(
        RssCapacity::new(5),
        RssCapacity(4, 8, 4, QosLevel::High, QosLevel::Middle, QosLevel::Middle,)
    ));
    assert!(is_rss_equal(
        RssCapacity::new(6),
        RssCapacity(4, 8, 2, QosLevel::High, QosLevel::Low, QosLevel::Low,)
    ));
    assert!(is_rss_equal(
        RssCapacity::new(7),
        RssCapacity(4, 4, 2, QosLevel::High, QosLevel::Low, QosLevel::Low,)
    ));
}