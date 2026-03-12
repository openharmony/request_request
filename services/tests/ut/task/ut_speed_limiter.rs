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

use super::*;

// @tc.name: ut_speed_limiter_default
// @tc.desc: Test SpeedLimiter default initialization
// @tc.precon: NA
// @tc.step: 1. Create SpeedLimiter using default
//           2. Verify all fields are initialized to default values
// @tc.expect: All fields are zero/None
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_speed_limiter_default() {
    let limiter = SpeedLimiter::default();

    assert_eq!(limiter.last_time, 0);
    assert_eq!(limiter.last_size, 0);
    assert_eq!(limiter.speed_limit, 0);
    assert!(limiter.sleep.is_none());
}

// @tc.name: ut_speed_limiter_update_speed_limit_same
// @tc.desc: Test update_speed_limit with same value
// @tc.precon: NA
// @tc.step: 1. Create SpeedLimiter and set speed_limit
//           2. Update with same speed_limit value
// @tc.expect: State is not reset when same value
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_speed_limiter_update_speed_limit_same() {
    let mut limiter = SpeedLimiter::default();
    limiter.speed_limit = 1000;
    limiter.last_time = 5000;
    limiter.last_size = 500;

    limiter.update_speed_limit(1000);

    assert_eq!(limiter.last_time, 5000);
    assert_eq!(limiter.last_size, 500);
    assert_eq!(limiter.speed_limit, 1000);
}

// @tc.name: ut_speed_limiter_update_speed_limit_different
// @tc.desc: Test update_speed_limit with different value
// @tc.precon: NA
// @tc.step: 1. Create SpeedLimiter and set speed_limit
//           2. Update with different speed_limit value
// @tc.expect: State is reset when value changes
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_speed_limiter_update_speed_limit_different() {
    let mut limiter = SpeedLimiter::default();
    limiter.speed_limit = 1000;
    limiter.last_time = 5000;
    limiter.last_size = 500;
    limiter.sleep = Some(Box::pin(ylong_runtime::time::sleep(std::time::Duration::from_millis(100))));

    limiter.update_speed_limit(2000);

    assert_eq!(limiter.last_time, 0);
    assert_eq!(limiter.last_size, 0);
    assert_eq!(limiter.speed_limit, 2000);
    assert!(limiter.sleep.is_none());
}

// @tc.name: ut_speed_limiter_update_speed_limit_zero
// @tc.desc: Test update_speed_limit with zero value
// @tc.precon: NA
// @tc.step: 1. Create SpeedLimiter with non-zero speed_limit
//           2. Update with zero value
// @tc.expect: State is reset when value changes to zero
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_speed_limiter_update_speed_limit_zero() {
    let mut limiter = SpeedLimiter::default();
    limiter.speed_limit = 1000;
    limiter.last_time = 5000;
    limiter.last_size = 500;

    limiter.update_speed_limit(0);

    assert_eq!(limiter.last_time, 0);
    assert_eq!(limiter.last_size, 0);
    assert_eq!(limiter.speed_limit, 0);
}

// @tc.name: ut_speed_limiter_update_speed_limit_from_zero
// @tc.desc: Test update_speed_limit from zero to non-zero
// @tc.precon: NA
// @tc.step: 1. Create SpeedLimiter with zero speed_limit
//           2. Update with non-zero value
// @tc.expect: speed_limit is updated
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_speed_limiter_update_speed_limit_from_zero() {
    let mut limiter = SpeedLimiter::default();

    limiter.update_speed_limit(3000);

    assert_eq!(limiter.speed_limit, 3000);
    assert_eq!(limiter.last_time, 0);
    assert_eq!(limiter.last_size, 0);
}

// @tc.name: ut_speed_limiter_poll_check_limit_disabled
// @tc.desc: Test poll_check_limit when speed limiting is disabled
// @tc.precon: NA
// @tc.step: 1. Create SpeedLimiter with speed_limit = 0
//           2. Call poll_check_limit
// @tc.expect: Returns Ready(Ok(()))
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_speed_limiter_poll_check_limit_disabled() {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    let mut limiter = SpeedLimiter::default();
    limiter.speed_limit = 0;

    fn dummy_raw_waker() -> RawWaker {
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            dummy_raw_waker()
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    let waker = unsafe { Waker::from_raw(dummy_raw_waker()) };
    let mut cx = Context::from_waker(&waker);

    let result = limiter.poll_check_limit(&mut cx, 1000, 500);

    assert!(matches!(result, Poll::Ready(Ok(()))));
}

// @tc.name: ut_speed_limiter_poll_check_limit_first_call
// @tc.desc: Test poll_check_limit on first call (last_time = 0)
// @tc.precon: NA
// @tc.step: 1. Create SpeedLimiter with speed_limit > 0
//           2. Call poll_check_limit with last_time = 0
// @tc.expect: Initializes timing and returns Ready(Ok(()))
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_speed_limiter_poll_check_limit_first_call() {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    let mut limiter = SpeedLimiter::default();
    limiter.speed_limit = 1000;
    limiter.last_time = 0;

    fn dummy_raw_waker() -> RawWaker {
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            dummy_raw_waker()
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    let waker = unsafe { Waker::from_raw(dummy_raw_waker()) };
    let mut cx = Context::from_waker(&waker);

    let result = limiter.poll_check_limit(&mut cx, 1000, 500);

    assert!(matches!(result, Poll::Ready(Ok(()))));
    assert_eq!(limiter.last_time, 1000);
    assert_eq!(limiter.last_size, 500);
}

// @tc.name: ut_speed_limiter_poll_check_limit_interval_elapsed
// @tc.desc: Test poll_check_limit when measurement interval has elapsed
// @tc.precon: NA
// @tc.step: 1. Create SpeedLimiter with last_time set
//           2. Call poll_check_limit with time > interval
// @tc.expect: Resets timing window
// @tc.type: FUNC
// @tc.require: issues#ICN16H
#[test]
fn ut_speed_limiter_poll_check_limit_interval_elapsed() {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    let mut limiter = SpeedLimiter::default();
    limiter.speed_limit = 1000;
    limiter.last_time = 0;
    limiter.last_size = 0;

    fn dummy_raw_waker() -> RawWaker {
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            dummy_raw_waker()
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    let waker = unsafe { Waker::from_raw(dummy_raw_waker()) };
    let mut cx = Context::from_waker(&waker);

    let result = limiter.poll_check_limit(&mut cx, 2000, 1500);

    assert!(matches!(result, Poll::Ready(Ok(()))));
    assert_eq!(limiter.last_time, 2000);
    assert_eq!(limiter.last_size, 1500);
}
