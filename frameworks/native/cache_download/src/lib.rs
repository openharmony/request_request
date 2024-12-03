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

#![allow(
    unknown_lints,
    static_mut_refs,
    unused,
    stable_features,
    missing_docs,
    clippy::new_without_default
)]
#![feature(lazy_cell)]

#[macro_use]
extern crate request_utils;

#[macro_use]
mod macros;

mod download;
pub mod services;
pub use download::task::Downloader;

cfg_ohos! {
    mod wrapper;
    const TAG: &str = "PreloadNative\0";
    const DOMAIN: u32 = 0xD001C50;
    use ffrt_rs::ffrt_spawn as spawn;
}

cfg_not_ohos! {
    use ylong_runtime::spawn_blocking as spawn;
}
