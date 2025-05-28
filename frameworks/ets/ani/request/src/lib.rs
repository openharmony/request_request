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

use std::ffi::{c_uint, c_void, CStr};

use ani_rs::{ani_constructor, AniVm};
use serde::de;

pub mod api10;
pub mod api9;

#[macro_use]
extern crate request_utils;

const TAG: &str = "RequestAni\0";
const DOMAIN: u32 = 0xD001C50;

ani_constructor!(
    namespace "L@ohos/request/request"
    [
        "downloadSync": api9::download::download_sync,
    ]
    namespace "L@ohos/request/request/agent"
    [
        "createSync": api10::task::create_sync,
    ]
    class "L@ohos/request/request/agent/TaskInner"
    [
        "startSync" : api10::task::start_sync,
        "on" : api10::task::on,
    ]
);
