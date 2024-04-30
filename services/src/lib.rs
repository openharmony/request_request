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

#![warn(missing_docs, unreachable_pub)]
#![warn(clippy::redundant_clone, clippy::redundant_static_lifetimes)]
#![cfg_attr(not(feature = "oh"), allow(unused))]
#![warn(clippy::enum_variant_names)]
#![feature(io_error_other)]

#[macro_use]
mod macros;

#[cfg(not(feature = "oh"))]
#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[cfg(feature = "oh")]
#[macro_use]
mod hilog;

mod error;
mod manage;
mod task;
mod utils;

cfg_oh! {
    mod init;
    mod trace;
    mod sys_event;
    mod service;

    /// hilog label
    pub const LOG_LABEL: hilog_rust::HiLogLabel = hilog_rust::HiLogLabel {
        log_type: hilog_rust::LogType::LogCore,
        domain: 0xD001C50,
        tag: "RequestService",
    };
}

cfg_not_oh! {
    fn test_set_up() {
        use std::sync::Once;
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .write_style(env_logger::WriteStyle::Always)
            .init();;
        })
    }
}
