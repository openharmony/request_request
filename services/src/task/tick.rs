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

use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, Once};
use std::task::{Context, Waker};

const WAITING_TICK: usize = 20;
pub(crate) const WAITING_TO_WAKE: usize = 3;
pub(crate) const WAITING_TO_TICK: usize = 10;
pub(crate) struct Clock {
    registers: Mutex<HashMap<u32, Waker>>,
    tick: AtomicUsize,
}

impl Clock {
    pub(crate) fn tick(&self) {
        let tick = self.tick.fetch_add(1, Ordering::SeqCst);

        if tick >= WAITING_TICK {
            self.tick.store(0, Ordering::SeqCst);
            self.wake_all();
        }
    }

    pub(crate) fn wake_all(&self) {
        let mut registers = self.registers.lock().unwrap();
        for (_, waker) in registers.drain() {
            waker.wake()
        }
    }

    pub(crate) fn wake_one(&self, task_id: u32) {
        let mut registers = self.registers.lock().unwrap();
        if let Some(waker) = registers.remove(&task_id) {
            waker.wake()
        }
    }

    pub(crate) fn get_instance() -> &'static Self {
        static mut CLOCK: MaybeUninit<Clock> = MaybeUninit::uninit();
        static INIT: Once = Once::new();
        INIT.call_once(|| unsafe {
            CLOCK.write(Clock {
                registers: Mutex::new(HashMap::new()),
                tick: AtomicUsize::new(0),
            });
        });
        unsafe { CLOCK.as_ptr().as_ref().unwrap() }
    }
    pub(crate) fn register(&self, task_id: u32, cx: &mut Context<'_>) {
        self.registers
            .lock()
            .unwrap()
            .insert(task_id, cx.waker().clone());
    }
}
