// Copyright (C) 2025 Huawei Device Co., Ltd.
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

pub use ffi::*;

// CXX bridge module for FFI bindings to C++ code
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {

    // C++ functions and types exposed to Rust
    unsafe extern "C++" {
        include!("permission_verification.h");

        /// Check if the calling process has INTERNET permission.
        fn CheckInternetPermission() -> bool;
        /// Check if the calling process has GET_NETWORK_INFO permission.
        fn CheckGetNetworkInfoPermission() -> bool;
    }
}
