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

//! Core request service crate.
//!
//! Aggregates the request subsystem's configuration types, error codes,
//! file handling, filters, runtime info, and external interfaces.

pub mod config;
pub mod error_code;
pub mod file;
pub mod filter;
pub mod info;
pub mod interface;
