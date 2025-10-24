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

//! Internal FFI wrapper implementation for FFRT.
//! 
//! This module provides the low-level bindings and wrapper types needed to interface
//! with the C++ FastFlow Runtime library.

// Import FFI functions from the C++ interface
pub(crate) use ffi::{FfrtSleep, FfrtSpawn};

/// Wrapper for closures to be passed across the FFI boundary.
/// 
/// This struct manages the lifetime and execution of closures that need to be
/// executed by the C++ FFRT library.
pub struct ClosureWrapper {
    /// The wrapped closure, stored as an Option to allow take() during execution
    inner: Option<Box<dyn FnOnce()>>,
}

impl ClosureWrapper {
    /// Creates a new boxed ClosureWrapper containing the provided closure.
    /// 
    /// # Arguments
    /// 
    /// * `f` - The closure to wrap, which must be 'static as it will be passed to C++
    /// 
    /// # Returns
    /// 
    /// Returns a boxed ClosureWrapper instance
    pub fn new<F>(f: F) -> Box<Self>
    where
        F: FnOnce() + 'static,
    {
        Box::new(Self {
            inner: Some(Box::new(f)),
        })
    }

    /// Executes the wrapped closure and consumes it.
    /// 
    /// This method is called by the C++ side to execute the Rust closure.
    /// After execution, the closure is removed from the wrapper to ensure it's
    /// only called once.
    pub fn run(&mut self) {
        if let Some(f) = self.inner.take() {
            f();
        }
    }
}

// CXX bridge for FFI between Rust and C++ FFRT components
#[cxx::bridge]
mod ffi {
    // Rust interface exposed to C++
    extern "Rust" {
        type ClosureWrapper;
        fn run(self: &mut ClosureWrapper);
    }

    // C++ interface imported to Rust
    unsafe extern "C++" {
        // Include the C++ header defining the FFRT interface
        include!("wrapper.h");
        
        // FFRT API functions
        fn FfrtSpawn(closure: Box<ClosureWrapper>);
        fn FfrtSleep(ms: u64);
    }
}
