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

use std::ffi::c_char;

use cxx::SharedPtr;
pub use ffi::*;

/// Wrapper for the Animation Environment type.
///
/// Provides a static lifetime wrapper around the `ani_rs::AniEnv` type for use in FFI.
#[repr(transparent)]
pub struct AniEnv {
    /// The inner animation environment instance.
    pub inner: ani_rs::AniEnv<'static>,
}

/// Wrapper for the Animation Object type.
///
/// Provides a static lifetime wrapper around the `ani_rs::objects::AniObject` type for use in FFI.
#[repr(transparent)]
pub struct AniObject {
    /// The inner animation object instance.
    pub inner: ani_rs::objects::AniObject<'static>,
}

/// Converts from CXX SharedPtr<ApplicationInfo> to Rust ApplicationInfo.
///
/// Transforms the C++ shared pointer to the native Rust representation by extracting
/// the bundle type information.
impl From<SharedPtr<ffi::ApplicationInfo>> for super::context::ApplicationInfo {
    fn from(value: SharedPtr<ffi::ApplicationInfo>) -> Self {
        super::context::ApplicationInfo {
            bundle_type: BundleType(&value).into(),
        }
    }
}

// CXX bridge module for FFI bindings to C++ code
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    // Import BundleType enum from OHOS::AppExecFwk namespace
    #[namespace = "OHOS::AppExecFwk"]
    #[repr(i32)]
    enum BundleType {
        APP,
        ATOMIC_SERVICE,
        SHARED,
        APP_SERVICE_FWK,
        APP_PLUGIN,
    }

    // Log type enumeration
    #[repr(i32)]
    #[namespace = ""]
    enum LogType {
        // min log type
        LOG_TYPE_MIN = 0,
        // Used by app log.
        LOG_APP = 0,
        // Log to kmsg, only used by init phase.
        LOG_INIT = 1,
        // Used by core service, framework.
        LOG_CORE = 3,
        // Used by kmsg log.
        LOG_KMSG = 4,
        // Not print in release version.
        LOG_ONLY_PRERELEASE = 5,
        // max log type
        LOG_TYPE_MAX,
    }

    // Log level enumeration
    #[repr(i32)]
    #[namespace = ""]
    enum LogLevel {
        // min log level
        LOG_LEVEL_MIN = 0,
        // Designates lower priority log.
        LOG_DEBUG = 3,
        // Designates useful information.
        LOG_INFO = 4,
        // Designates hazardous situations.
        LOG_WARN = 5,
        // Designates very serious errors.
        LOG_ERROR = 6,
        // Designates major fatal anomaly.
        LOG_FATAL = 7,
        // max log level
        LOG_LEVEL_MAX,
    }

    // Rust types exposed to C++
    extern "Rust" {
        type AniEnv;

        type AniObject;
    }

    // C++ functions and types exposed to Rust
    unsafe extern "C++" {
        include!("hilog/log.h");
        include!("request_utils_wrapper.h");
        include!("application_context.h");
        include!("context.h");
        include!("storage_acl.h");
        include!("file_uri.h");

        fn FileUriGetRealPath(uri: &CxxString) -> String;

        #[namespace = "OHOS::AppExecFwk"]
        type BundleType;

        /// Gets the cache directory path.
        ///
        /// Returns the system's cache directory path as a string.
        fn GetCacheDir() -> String;

        fn GetBaseDir() -> String;

        /// Computes the SHA-256 hash of an input string.
        ///
        /// # Parameters
        ///
        /// * `input` - The string to hash
        ///
        /// # Returns
        ///
        /// The SHA-256 hash of the input as a string.
        fn SHA256(input: &str) -> String;

        /// Checks if the given environment is a stage context.
        ///
        /// # Safety
        ///
        /// This function is unsafe because it works with raw pointers that must be valid.
        ///
        /// # Parameters
        ///
        /// * `env` - Pointer to the animation environment
        /// * `ani_object` - Pointer to the animation object
        ///
        /// # Returns
        ///
        /// Returns true if the environment is a stage context.
        /// #Safety
        /// todo
        unsafe fn IsStageContext(env: *mut AniEnv, ani_object: *mut AniObject) -> bool;

        /// Gets the stage mode context from the given environment and object.
        ///
        /// # Safety
        ///
        /// This function is unsafe because it works with raw pointers that must be valid
        /// and may modify the environment pointer.
        ///
        /// # Parameters
        ///
        /// * `env` - Double pointer to the animation environment
        /// * `ani_object` - Pointer to the animation object
        ///
        /// # Returns
        ///
        /// A shared pointer to the context if successful.
        unsafe fn GetStageModeContext(
            env: *mut *mut AniEnv,
            ani_object: *mut AniObject,
        ) -> SharedPtr<Context>;

        /// Gets the bundle name from the context.
        ///
        /// # Parameters
        ///
        /// * `context` - Shared pointer to the context
        ///
        /// # Returns
        ///
        /// The bundle name as a string.
        fn GetBundleName(context: &SharedPtr<Context>) -> String;

        /// Gets the cache directory path from the context.
        ///
        /// # Parameters
        ///
        /// * `context` - Shared pointer to the context
        ///
        /// # Returns
        ///
        /// The cache directory path as a string.
        fn ContextGetCacheDir(context: &SharedPtr<Context>) -> String;

        /// Gets the base directory path from the context.
        ///
        /// # Parameters
        ///
        /// * `context` - Shared pointer to the context
        ///
        /// # Returns
        ///
        /// The base directory path as a string.
        fn ContextGetBaseDir(context: &SharedPtr<Context>) -> String;

        /// Gets the bundle type from application info.
        ///
        /// # Parameters
        ///
        /// * `application_info` - Shared pointer to the application info
        ///
        /// # Returns
        ///
        /// The bundle type.
        fn BundleType(application_info: &SharedPtr<ApplicationInfo>) -> BundleType;

        #[namespace = "OHOS::AbilityRuntime"]
        type Context;

        #[namespace = "OHOS::AppExecFwk"]
        type ApplicationInfo;

        /// Gets the application info from the context.
        ///
        /// # Returns
        ///
        /// A shared pointer to the application info.
        #[namespace = "OHOS::AbilityRuntime"]
        fn GetApplicationInfo(self: &Context) -> SharedPtr<ApplicationInfo>;

        /// Sets access control entries for a target file.
        ///
        /// # Parameters
        ///
        /// * `targetFile` - Path to the file
        /// * `entryTxt` - ACL entry string
        ///
        /// # Returns
        ///
        /// Returns 0 on success, non-zero error code on failure.
        #[namespace = "OHOS::StorageDaemon"]
        fn AclSetAccess(targetFile: &CxxString, entryTxt: &CxxString) -> i32;

        /// Sets default access control entries for a target file.
        ///
        /// # Parameters
        ///
        /// * `targetFile` - Path to the file
        /// * `entryTxt` - Default ACL entry string
        ///
        /// # Returns
        ///
        /// Returns 0 on success, non-zero error code on failure.
        #[namespace = "OHOS::StorageDaemon"]
        fn AclSetDefault(targetFile: &CxxString, entryTxt: &CxxString) -> i32;

        #[namespace = ""]
        type LogType;

        #[namespace = ""]
        type LogLevel;

        fn IsCleartextPermitted(hostname: &CxxString) -> bool;

        fn GetTrustAnchorsForHostName(hostname: &CxxString) -> Vec<String>;

        fn GetCertificatePinsForHostName(hostname: &CxxString) -> String;
    }
}

/// Prints a log message using HiLog.
///
/// Uses the HiLog system to print log messages with the specified level, domain, tag, and format.
///
/// # Parameters
///
/// * `level` - The log level (DEBUG, INFO, WARN, ERROR, FATAL)
/// * `domain` - The log domain ID
/// * `tag` - The log tag string
/// * `fmt` - The format string for the log message
///
/// # Safety
///
/// This function contains unsafe code to call the C HiLogPrint function with raw pointers.
///
/// # Examples
///
/// ```rust
/// use request_utils::wrapper::{hilog_print, LogLevel};
///
/// // Print an info log message
/// hilog_print(
///     LogLevel::LOG_INFO,
///     0xD001100,
///     "RequestService",
///     "Operation completed successfully".to_string()
/// );
/// ```
pub fn hilog_print(level: LogLevel, domain: u32, tag: &str, mut fmt: String) {
    // Convert Rust string to C string pointer
    let tag = tag.as_ptr() as *const c_char;
    // Ensure format string is null-terminated for C compatibility
    fmt.push('\0');
    unsafe {
        HiLogPrint(
            LogType::LOG_CORE,
            level,
            domain,
            tag,
            fmt.as_ptr() as *const c_char,
        );
    }
}

extern "C" {
    fn HiLogPrint(
        log_type: ffi::LogType,
        level: ffi::LogLevel,
        domain: u32,
        tag: *const c_char,
        fmt: *const c_char,
        ...
    ) -> i32;
}
