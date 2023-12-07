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

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::task::{Context, Waker};

const WAITING_TICK: usize = 20;

pub(crate) struct Clock {
    registers: Mutex<Vec<Waker>>,
    tick: AtomicUsize,
}

impl Clock {
    pub(crate) fn tick(&mut self) {
        let tick = self.tick.fetch_add(1, Ordering::SeqCst);

        if tick >= WAITING_TICK {
            self.tick.store(0, Ordering::SeqCst);
            self.wake_all();
        }
    }

    pub(crate) fn wake_all(&mut self) {
        let mut registers = self.registers.lock().unwrap();
        while let Some(waker) = registers.pop() {
            waker.wake()
        }
    }

    pub(crate) fn get_instance() -> &'static mut Self {
        static mut CLOCK: Clock = Clock {
            registers: Mutex::new(Vec::new()),
            tick: AtomicUsize::new(0),
        };
        unsafe { &mut CLOCK }
    }
    pub(crate) fn register(&mut self, cx: &mut Context<'_>) {
        self.registers.lock().unwrap().push(cx.waker().clone());
    }
}

#[cfg(test)]
mod test {

    use std::future::Future;
    use std::task::Poll;

    use super::*;
    struct TestFuture(Option<()>);
    impl Future for TestFuture {
        type Output = ();
        fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let me = self.get_mut();
            if me.0.take().is_none() {
                Clock::get_instance().register(cx);
                me.0 = Some(());
                println!("hello");
                Poll::Pending
            } else {
                println!("world");
                Poll::Ready(())
            }
        }
    }

    #[test]
    fn tick_tesk() {
        let join_handle = ylong_runtime::spawn(TestFuture(None));
        assert!(!join_handle.is_finished());
        let _ = ylong_runtime::spawn(async {
            loop {
                Clock::get_instance().tick();
            }
        });
    }
}
