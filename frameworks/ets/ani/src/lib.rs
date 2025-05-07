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

use ani_rs::AniVm;

pub mod api10;
pub mod api9;
mod error;
mod file_control;
mod listener;
pub mod middle_layer;
pub mod proxy;
mod unfinished;
mod wrapper;

#[macro_use]
extern crate request_utils;

const REQUEST_NAMESPACE: &CStr = cstr(b"L@ohos/request/request;\0");
const AGENT_NAMESPACE: &CStr = cstr(b"L@ohos/request/request/agent;\0");
const AGENT_TASK: &CStr = cstr(b"L@ohos/request/request/agent/TaskInner;\0");

const TAG: &str = "RequestAni\0";
const DOMAIN: u32 = 0xD001C50;

pub(crate) const fn cstr(input: &[u8]) -> &CStr {
    unsafe { CStr::from_bytes_with_nul_unchecked(input) }
}

#[no_mangle]
pub extern "C" fn ANI_Constructor(vm: AniVm, result: *mut u32) -> c_uint {
    unsafe {
        let env = vm.get_env().unwrap();
        let name_space = env.find_namespace(REQUEST_NAMESPACE).unwrap();
        let methods = [(
            api9::download::DOWNLOAD_SYNC,
            api9::download::download_sync as _,
        )];

        env.bind_namespace_functions(name_space, &methods).unwrap();

        let agent = env.find_namespace(AGENT_NAMESPACE).unwrap();
        let methods = [(api10::task::CREATE_SYNC, api10::task::create_sync as _)];
        env.bind_namespace_functions(agent, &methods).unwrap();

        let class = env.find_class(AGENT_TASK).unwrap();
        let methods = [
            (api10::task::START_SYNC, api10::task::start_sync as _),
            (api10::task::ON, api10::task::on as _),
        ];
        env.bind_class_methods(class, &methods).unwrap();

        *result = 1;
    };
    0
}
