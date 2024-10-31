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

//! # Pre-download native

#![allow(missing_docs)]
#![allow(stable_features)]
#![feature(lazy_cell)]
#![allow(unused)]
#[macro_use]
extern crate request_utils;

mod agent;
pub use agent::{CustomCallback, DownloadAgent};

mod cache;
mod download;

cfg_ohos! {
    mod wrapper;
    const TAG: &str = "PreDownloadNative\0";
    const DOMAIN: u32 = 0xD001C50;
    use ffrt_rs::ffrt_spawn as spawn;
}

cfg_not_ohos! {
    use std::thread::spawn as spawn;
}

cfg_test! {
    #[cfg(not(feature = "ohos"))]
    fn init() {
        let _ = env_logger::builder().is_test(true).format_timestamp_millis().try_init();
    }

    #[cfg(feature = "ohos")]
    fn init() {}

    const TEST_URL: &str =
        "http://www.baidu.com";
}
