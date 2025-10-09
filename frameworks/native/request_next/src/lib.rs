// Copyright (C) 2025 Huawei Device Co., Ltd.
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

#![feature(lazy_cell)]
pub mod check;
pub mod client;
mod proxy;
pub use client::RequestClient;
mod listen;
pub use listen::Callback;

#[macro_use]
extern crate request_utils;

use hilog_rust::{HiLogLabel, LogType};

pub(crate) const LOG_LABEL: HiLogLabel = HiLogLabel {
    log_type: LogType::LogCore,
    domain: 0xD001C50,
    tag: "RequestNative",
};
