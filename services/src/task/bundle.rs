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

//! Provides functionality to retrieve application bundle information.
//! 
//! This module offers utilities to get application name and index information
//! from the system bundle service using FFI calls to C++ code.

#[allow(unused)]
#[cxx::bridge(namespace = "OHOS::Request")]
/// FFI bridge for communicating with C++ bundle service.
/// 
/// This module defines the interface between Rust and C++ for bundle information retrieval.
mod ffi {
    /// Application information structure returned from C++ bundle service.
    /// 
    /// Contains status flag, index, and name of an application.
    struct AppInfo {
        /// Indicates if the operation was successful.
        ret: bool,
        /// Application index identifier.
        index: i32,
        /// Application name.
        name: String,
    }

    unsafe extern "C++" {
        include!("bundle.h");
        /// Retrieves application name and index based on UID.
        /// 
        /// # Safety
        /// This is an unsafe external C++ function without Rust's safety guarantees.
        fn GetNameAndIndex(uid: i32) -> AppInfo;
    }
}

/// Retrieves application index and name for a given user ID.
/// 
/// Queries the system bundle service to get the application's index and name
/// associated with the provided UID.
///
/// # Parameters
/// - `uid`: The user ID to query for application information
///
/// # Returns
/// - `Some((i32, String))` containing the application index and name if successful
/// - `None` if the application information could not be retrieved
///
/// # Examples
/// ```rust
/// // Get application info for UID 1000
/// let result = get_name_and_index(1000);
/// match result {
///     Some((index, name)) => println!("App found: {} (index: {})", name, index),
///     None => println!("Failed to get app info"),
/// }
/// ```
///
/// # Safety
/// This function calls an unsafe external C++ function through the FFI bridge,
/// but handles the unsafe operation internally.
pub(crate) fn get_name_and_index(uid: i32) -> Option<(i32, String)> {
    // Call the C++ function to get application info
    let app_info = ffi::GetNameAndIndex(uid);
    
    // Convert the result based on the success flag
    match app_info.ret {
        true => Some((app_info.index, app_info.name)),
        false => None,
    }
}
