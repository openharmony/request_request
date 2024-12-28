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

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use ylong_http_client::HttpClientError;
use ylong_runtime::time::{sleep, Sleep};

#[derive(Default)]
pub(crate) struct SpeedLimiter {
    pub(crate) last_time: u64,
    pub(crate) last_size: u64,
    pub(crate) speed_limit: u64,
    pub(crate) sleep: Option<Pin<Box<Sleep>>>,
}

impl SpeedLimiter {
    pub(crate) fn update_speed_limit(&mut self, speed_limit: u64) {
        if self.speed_limit != speed_limit {
            self.last_size = 0;
            self.last_time = 0;
            self.sleep = None;
            self.speed_limit = speed_limit;
        }
    }

    pub(crate) fn poll_check_limit(
        &mut self,
        cx: &mut Context<'_>,
        current_time: u64,
        current_size: u64,
    ) -> Poll<Result<(), HttpClientError>> {
        const SPEED_LIMIT_INTERVAL: u64 = 1000;
        self.sleep = None;
        if self.speed_limit != 0 {
            if self.last_time == 0 || current_time - self.last_time >= SPEED_LIMIT_INTERVAL {
                // get the init time and size, for speed caculate
                self.last_time = current_time;
                self.last_size = current_size;
            } else if current_time - self.last_time < SPEED_LIMIT_INTERVAL
                && ((current_size - self.last_size) >= self.speed_limit)
            {
                // sleep until wakeup_time if needed or speed limit conditions are met
                let limit_time = (current_size - self.last_size) * SPEED_LIMIT_INTERVAL
                    / self.speed_limit
                    - (current_time - self.last_time);
                self.sleep = Some(Box::pin(sleep(Duration::from_millis(limit_time))));
            }
        }

        if self.sleep.is_some() && Pin::new(self.sleep.as_mut().unwrap()).poll(cx).is_pending() {
            return Poll::Pending;
        }
        Poll::Ready(Ok(()))
    }
}
