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

//! Foreign Function Interface (FFI) bindings for the request system.
//!
//! This module provides FFI bindings to C++ code through the CXX bridge, enabling
//! interaction with native APIs from Rust. It includes types, enums, and functions
//! for accessing system services, logging, and storage functionality.

#![allow(unused)]

use cxx::SharedPtr;
pub use ffi::*;

// CXX bridge module for FFI bindings to C++ code
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {

    // C++ functions and types exposed to Rust
    unsafe extern "C++" {
        include!("request_utils_dataability.h");
        
        #[namespace = "OHOS::AbilityRuntime"]
        type Context = request_utils::wrapper::Context;

        fn DataAbilityOpenFile(context: &SharedPtr<Context>, path: &CxxString) -> i32;
    }
}