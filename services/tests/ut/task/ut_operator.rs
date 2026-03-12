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
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

fn create_safe_waker(wake_count: Arc<AtomicUsize>) -> Waker {
    fn clone(data: *const ()) -> RawWaker {
        let arc = unsafe { Arc::from_raw(data as *const AtomicUsize) };
        let cloned = arc.clone();
        std::mem::forget(arc);
        RawWaker::new(Arc::into_raw(cloned) as *const (), &VTABLE)
    }

    fn wake(data: *const ()) {
        let arc = unsafe { Arc::from_raw(data as *const AtomicUsize) };
        arc.fetch_add(1, Ordering::SeqCst);
    }

    fn wake_by_ref(data: *const ()) {
        let arc = unsafe { Arc::from_raw(data as *const AtomicUsize) };
        arc.fetch_add(1, Ordering::SeqCst);
        std::mem::forget(arc);
    }

    fn drop_waker(data: *const ()) {
        unsafe { Arc::from_raw(data as *const AtomicUsize) };
    }

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop_waker);

    let raw = RawWaker::new(Arc::into_raw(wake_count) as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw) }
}

// @tc.name: ut_speed_limiter
// @tc.desc: Test SpeedLimiter default values and update_speed_limit
// @tc.precon: NA
// @tc.step: 1. Test default values
//           2. Test update_speed_limit resets state when value changes
//           3. Test update_speed_limit keeps state when same value
// @tc.expect: SpeedLimiter behaves correctly
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_speed_limiter() {
    let limiter = SpeedLimiter::default();
    assert_eq!(limiter.last_time, 0);
    assert_eq!(limiter.last_size, 0);
    assert_eq!(limiter.speed_limit, 0);
    assert!(limiter.sleep.is_none());

    let mut limiter = SpeedLimiter::default();
    limiter.last_time = 1000;
    limiter.last_size = 500;
    limiter.update_speed_limit(2000);
    assert_eq!(limiter.speed_limit, 2000);
    assert_eq!(limiter.last_time, 0);
    assert_eq!(limiter.last_size, 0);

    let mut limiter = SpeedLimiter::default();
    limiter.speed_limit = 2000;
    limiter.last_time = 1000;
    limiter.last_size = 500;
    limiter.update_speed_limit(2000);
    assert_eq!(limiter.speed_limit, 2000);
    assert_eq!(limiter.last_time, 1000);
    assert_eq!(limiter.last_size, 500);
}

// @tc.name: ut_speed_limiter_poll_check_limit
// @tc.desc: Test SpeedLimiter poll_check_limit with safe waker
// @tc.precon: NA
// @tc.step: 1. Test with zero speed limit
//           2. Test initialization on first call
// @tc.expect: poll_check_limit works correctly with safe waker
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_speed_limiter_poll_check_limit() {
    let wake_count = Arc::new(AtomicUsize::new(0));
    let waker = create_safe_waker(wake_count.clone());
    let mut cx = Context::from_waker(&waker);

    let mut limiter = SpeedLimiter::default();
    limiter.speed_limit = 0;
    let result = limiter.poll_check_limit(&mut cx, 1000, 500);
    assert!(matches!(result, Poll::Ready(Ok(()))));

    let mut limiter = SpeedLimiter::default();
    limiter.speed_limit = 1000;
    limiter.last_time = 0;
    let _ = limiter.poll_check_limit(&mut cx, 5000, 1000);
    assert_eq!(limiter.last_time, 5000);
    assert_eq!(limiter.last_size, 1000);
}

// @tc.name: ut_min_speed_timeout_defaults
// @tc.desc: Test MinSpeed and Timeout default values
// @tc.precon: NA
// @tc.step: 1. Create MinSpeed using default
//           2. Create Timeout using default
// @tc.expect: Both have zero values
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_min_speed_timeout_defaults() {
    let min_speed = MinSpeed::default();
    assert_eq!(min_speed.speed, 0);
    assert_eq!(min_speed.duration, 0);

    let timeout = Timeout::default();
    assert_eq!(timeout.connection_timeout, 0);
    assert_eq!(timeout.total_timeout, 0);
}
