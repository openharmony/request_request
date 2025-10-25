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

//! Conditional compilation utility macros.
//! 
//! This module provides macros for conditional compilation based on build
//! configurations, enabling code to be included or excluded depending on
//! whether the code is being compiled in test mode or with specific features.

/// Conditionally includes items only when compiled in test mode.
///
/// Wraps the provided items with `#[cfg(test)]`, causing them to be included
/// only when the test configuration is active.
///
/// # Examples
///
/// ```rust
/// use request_utils::cfg_test;
///
/// cfg_test! {
///     fn test_helper() -> u32 {
///         42
///     }
/// }
///
/// #[test]
/// fn test_using_helper() {
///     // Available only in test mode
///     assert_eq!(test_helper(), 42);
/// }
/// ```
#[macro_export]
macro_rules! cfg_test {
    ($($item:item)*) => {
        $(
            #[cfg(test)]
            $item
        )*
    }
}

/// Conditionally includes items only when not compiled in test mode.
///
/// Wraps the provided items with `#[cfg(not(test))]`, causing them to be included
/// only when the test configuration is not active.
///
/// # Examples
///
/// ```rust
/// use request_utils::cfg_not_test;
///
/// cfg_not_test! {
///     // This function is only available in non-test builds
///     fn production_only() -> &'static str {
///         "Production mode"
///     }
/// }
///
/// ```
#[macro_export]
macro_rules! cfg_not_test {
    ($($item:item)*) => {
        $(
            #[cfg(not(test))]
            $item
        )*
    }
}

/// Conditionally includes items only when the "ohos" feature is enabled.
///
/// Wraps the provided items with `#[cfg(feature = "ohos")]`, causing them to be
/// included only when the "ohos" feature flag is enabled during compilation.
///
/// # Examples
///
/// ```rust
/// use request_utils::cfg_ohos;
///
/// cfg_ohos! {
///     fn ohos_specific_function() -> &'static str {
///         "OHOS specific implementation"
///     }
/// }
///
/// // Usage with conditional compilation check
/// #[cfg(feature = "ohos")]
/// fn use_ohos_function() {
///     println!("{}", ohos_specific_function());
/// }
/// ```
#[macro_export]
macro_rules! cfg_ohos {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "ohos")]
            $item
        )*
    }
}

/// Conditionally includes items only when the "ohos" feature is not enabled.
///
/// Wraps the provided items with `#[cfg(not(feature = "ohos"))]`, causing them to be
/// included only when the "ohos" feature flag is not enabled during compilation.
///
/// # Examples
///
/// ```rust
/// use request_utils::cfg_not_ohos;
///
/// cfg_not_ohos! {
///     fn non_ohos_implementation() -> &'static str {
///         "Standard implementation"
///     }
/// }
///
/// // Usage with conditional compilation check
/// #[cfg(not(feature = "ohos"))]
/// fn use_standard_function() {
///     println!("{}", non_ohos_implementation());
/// }
/// ```
#[macro_export]
macro_rules! cfg_not_ohos {
    ($($item:item)*) => {
        $(
            #[cfg(not(feature = "ohos"))]
            $item
        )*
    }
}
