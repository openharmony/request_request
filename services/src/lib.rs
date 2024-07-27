// Copyright (C) 2023 Huawei Device Co., Ltd.
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

//! This create implement the request proxy and stub
#![cfg_attr(feature = "oh", feature(io_error_other))]
#![cfg_attr(test, feature(future_join))]
#![cfg_attr(test, allow(clippy::redundant_clone))]
#![allow(unreachable_pub, clippy::new_without_default)]
#![warn(
    missing_docs,
    clippy::redundant_static_lifetimes,
    clippy::enum_variant_names,
    clippy::clone_on_copy,
    clippy::unused_async
)]
#[macro_use]
mod macros;

#[cfg(not(feature = "oh"))]
#[macro_use]
extern crate log;

cfg_oh! {
    #[macro_use]
    mod hilog;
    mod trace;
    pub mod ability;
    mod sys_event;
    pub use service::interface;
    pub use utils::form_item::FileSpec;
}

mod error;
mod manage;
mod service;
mod task;
mod utils;
pub use task::{config, info};

cfg_oh! {
#[cfg(not(test))]
const LOG_LABEL: hilog_rust::HiLogLabel = hilog_rust::HiLogLabel {
    log_type: hilog_rust::LogType::LogCore,
    domain: 0xD001C50,
    tag: "RequestService",
};

#[cfg(test)]
const LOG_LABEL: hilog_rust::HiLogLabel = hilog_rust::HiLogLabel {
    log_type: hilog_rust::LogType::LogCore,
    domain: 0xD001C50,
    tag: "RequestUtTest",
};

#[cfg(test)]
mod tests {

    /// test init
    pub(crate) fn test_init() {
        unsafe { SetAccessTokenPermission() };
    }

    pub(crate) static DB_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    extern "C" {
        fn SetAccessTokenPermission();
    }
}
}
