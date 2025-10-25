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

//! Configuration macros for feature-gated conditional compilation.
//! 
//! This module defines convenience macros for conditional compilation based on
//! feature flags, making it easier to manage platform-specific and backend-specific code.

/// Conditionally includes code when the "ylong" feature is enabled.
///
/// Applies `#[cfg(feature = "ylong")]` to all provided items, enabling them only when
/// the "ylong" feature flag is set.
///
/// # Examples
///
/// ```rust
/// cfg_ylong! {
///     fn ylong_specific_function() {
///         // Implementation specific to ylong backend
///     }
///     
///     struct YlongSpecificStruct {
///         // Fields specific to ylong implementation
///     }
/// }
/// ```
macro_rules! cfg_ylong {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "ylong")]
            $item
        )*
    }
}

/// Conditionally includes code when the "netstack" feature is enabled.
///
/// Applies `#[cfg(feature = "netstack")]` to all provided items, enabling them only when
/// the "netstack" feature flag is set.
///
/// # Examples
///
/// ```rust
/// cfg_netstack! {
///     fn netstack_specific_function() {
///         // Implementation specific to netstack backend
///     }
///     
///     struct NetstackSpecificStruct {
///         // Fields specific to netstack implementation
///     }
/// }
/// ```
macro_rules! cfg_netstack {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "netstack")]
            $item
        )*
    }
}
