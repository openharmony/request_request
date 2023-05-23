/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
//! This create implement the request server register and publish
#![allow(unused_variables)]
extern crate ipc_rust;
extern crate request;
extern crate system_ability_fwk_rust;

use hilog_rust::*;
use ipc_rust::{IRemoteBroker, RemoteObj};
pub use request::{start, stop, RequestService, RequestServiceStub, LOG_LABEL};
use std::ffi::{c_char, CString};
use system_ability_fwk_rust::{define_system_ability, IMethod, ISystemAbility, RSystemAbility};

/// TEST_SERVICE_ID SAID
pub const REQUEST_SERVICE_ID: i32 = 3706;

fn on_start<T: ISystemAbility + IMethod>(ability: &T) {
    info!(LOG_LABEL, "on_start");
    let service =
        RequestServiceStub::new_remote_stub(RequestService).expect("create RequestService failed");
    ability.publish(
        &service.as_object().expect("get request service failed"),
        REQUEST_SERVICE_ID,
    );
    start();
}

fn on_stop<T: ISystemAbility + IMethod>(ability: &T) {
    info!(LOG_LABEL, "on_stop");
    stop();
}

define_system_ability!(sa: SystemAbility(on_start, on_stop),);

#[used]
#[link_section = ".init_array"]
static A: extern "C" fn() = {
    extern "C" fn init() {
        info!(LOG_LABEL, "request service init");
        let system_ability = SystemAbility::new_system_ability(REQUEST_SERVICE_ID, false)
            .expect("create service failed");
        system_ability.register();
    }
    init
};
