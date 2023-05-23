/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use hilog_rust::*;
use std::{
    ffi::{c_char, CString},
    file,
    option::Option,
};

/// hilog label
pub const LOG_LABEL: HiLogLabel = HiLogLabel {
    log_type: LogType::LogCore,
    domain: 0xD001C00,
    tag: "RequestService",
};

/// print debug log
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => ({
        debug!(LOG_LABEL, "[{}:{}]:{}",file!().rsplit('/').collect::<Vec<&str>>().first().unwrap(),
               line!(), format!($($arg)*));
    })
}
/// print info log
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => ({
        info!(LOG_LABEL, "[{}:{}]:{}", file!().rsplit('/').collect::<Vec<&str>>().first().unwrap(),
              line!(), format!($($arg)*));
    })
}
/// print warn log
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => ({
         warn!(LOG_LABEL, "[{}:{}]:{}", file!().rsplit('/').collect::<Vec<&str>>().first().unwrap(),
               line!(), format!($($arg)*));
    })
}
/// print error log
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => ({
         error!(LOG_LABEL, "[{}:{}]:{}", file!().rsplit('/').collect::<Vec<&str>>().first().unwrap(),
                line!(), format!($($arg)*));
    })
}
/// print fatal log
#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)*) => ({
         fatal!(LOG_LABEL, "[{}:{}]:{}", file!().rsplit('/').collect::<Vec<&str>>().first().unwrap(),
                line!(), format!($($arg)*));
    })
}
