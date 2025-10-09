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

//! netstack_rs is used to provide a rust interface to the netstack library.

#![warn(
    missing_docs,
    clippy::redundant_static_lifetimes,
    clippy::enum_variant_names,
    clippy::clone_on_copy,
    clippy::unused_async
)]
#![deny(unused_must_use)]
#![allow(missing_docs, clippy::new_without_default)]

/// Request.
pub mod request;
/// create and manage requests.
pub mod task;

/// response from the server.
pub mod response;

pub mod error;
mod wrapper;

pub mod info;

use hilog_rust::{HiLogLabel, LogType};

pub(crate) const LOG_LABEL: HiLogLabel = HiLogLabel {
    log_type: LogType::LogCore,
    domain: 0xD001C50,
    tag: "PreloadNative",
};
