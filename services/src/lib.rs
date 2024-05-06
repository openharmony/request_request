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

#![feature(io_error_other)]
#![warn(
    missing_docs,
    unreachable_pub,
    clippy::redundant_clone,
    clippy::redundant_static_lifetimes,
    clippy::enum_variant_names
)]
#[macro_use]
mod hilog;

mod error;
mod init;
mod manage;
mod service;
mod sys_event;
mod task;
mod trace;
mod utils;

const LOG_LABEL: hilog_rust::HiLogLabel = hilog_rust::HiLogLabel {
    log_type: hilog_rust::LogType::LogCore,
    domain: 0xD001C50,
    tag: "RequestService",
};
