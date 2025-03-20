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

pub(crate) use ffi::{FfrtSleep, FfrtSpawn};

pub struct ClosureWrapper {
    inner: Option<Box<dyn FnOnce()>>,
}

impl ClosureWrapper {
    pub fn new<F>(f: F) -> Box<Self>
    where
        F: FnOnce() + 'static,
    {
        Box::new(Self {
            inner: Some(Box::new(f)),
        })
    }

    pub fn run(&mut self) {
        if let Some(f) = self.inner.take() {
            f();
        }
    }
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type ClosureWrapper;
        fn run(self: &mut ClosureWrapper);
    }

    unsafe extern "C++" {
        include!("wrapper.h");
        fn FfrtSpawn(closure: Box<ClosureWrapper>);
        fn FfrtSleep(ms: u64);
    }
}
