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

#![allow(unused)]

use std::ffi::c_char;

use cxx::SharedPtr;
pub use ffi::*;

#[repr(transparent)]
pub struct AniEnv {
    pub inner: ani_rs::AniEnv<'static>,
}

#[repr(transparent)]
pub struct AniObject {
    pub inner: ani_rs::objects::AniObject<'static>,
}

impl From<SharedPtr<ffi::ApplicationInfo>> for super::context::ApplicationInfo {
    fn from(value: SharedPtr<ffi::ApplicationInfo>) -> Self {
        super::context::ApplicationInfo {
            bundle_type: BundleType(&value).into(),
        }
    }
}

#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {

    #[namespace = "OHOS::AppExecFwk"]
    #[repr(i32)]
    enum BundleType {
        APP,
        ATOMIC_SERVICE,
        SHARED,
        APP_SERVICE_FWK,
        APP_PLUGIN,
    }

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

    // Log level
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

    extern "Rust" {
        type AniEnv;

        type AniObject;
    }

    unsafe extern "C++" {
        include!("hilog/log.h");
        include!("request_utils_wrapper.h");
        include!("application_context.h");
        include!("context.h");
        include!("storage_acl.h");

        #[namespace = "OHOS::AppExecFwk"]
        type BundleType;

        fn GetCacheDir() -> String;

        fn SHA256(input: &str) -> String;

        unsafe fn IsStageContext(env: *mut AniEnv, ani_object: *mut AniObject) -> bool;

        unsafe fn GetStageModeContext(
            env: *mut *mut AniEnv,
            ani_object: *mut AniObject,
        ) -> SharedPtr<Context>;

        fn GetBundleName(context: &SharedPtr<Context>) -> String;

        fn ContextGetCacheDir(context: &SharedPtr<Context>) -> String;
        fn ContextGetBaseDir(context: &SharedPtr<Context>) -> String;

        fn BundleType(application_info: &SharedPtr<ApplicationInfo>) -> BundleType;

        #[namespace = "OHOS::AbilityRuntime"]
        type Context;

        #[namespace = "OHOS::AppExecFwk"]
        type ApplicationInfo;

        #[namespace = "OHOS::AbilityRuntime"]
        fn GetApplicationInfo(self: &Context) -> SharedPtr<ApplicationInfo>;

        #[namespace = "OHOS::StorageDaemon"]
        fn AclSetAccess(targetFile: &CxxString, entryTxt: &CxxString) -> i32;

        #[namespace = "OHOS::StorageDaemon"]
        fn AclSetDefault(targetFile: &CxxString, entryTxt: &CxxString) -> i32;

        #[namespace = ""]
        type LogType;

        #[namespace = ""]
        type LogLevel;

    }
}

pub fn hilog_print(level: LogLevel, domain: u32, tag: &str, mut fmt: String) {
    let tag = tag.as_ptr() as *const c_char;
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
