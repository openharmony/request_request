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
//! This create implement the request server register and publish

use hisysevent::{build_number_param, write, EventType};
use samgr::definition::DOWNLOAD_SERVICE_ID;
use system_ability_fwk::ability::{Ability, Handler};

use crate::service::RequestServiceStub;
use crate::{start, stop};

/// TEST_SERVICE_ID SAID

fn service_start_fault() {
    const DOMAIN: &str = "REQUEST";
    const SERVICE_START_FAULT: &str = "SERVICE_START_FAULT";
    const ERROR_INFO: &str = "ERROR_INFO";
    const DOWNLOAD_PUBLISH_FAIL: i32 = -1;

    write(
        DOMAIN,
        SERVICE_START_FAULT,
        EventType::Fault,
        &[build_number_param!(ERROR_INFO, DOWNLOAD_PUBLISH_FAIL)],
    );
}

struct RequestAbility;

impl Ability for RequestAbility {
    /// Callback to deal safwk onstart for this system_ability
    fn on_start(&self, handler: Handler) {
        info!("on_start");

        if !handler.publish(RequestServiceStub) {
            service_start_fault();
        }
        start();
        info!("on_start succeed");
    }

    /// Callback to deal safwk onstop for this system_ability
    fn on_stop(&self) {
        info!("on_stop");
        stop();
        info!("on_stop succeed");
    }
}

#[used]
#[link_section = ".init_array"]
static A: extern "C" fn() = {
    extern "C" fn init() {
        info!("begin request service init");
        let system_ability = RequestAbility
            .build_system_ability(DOWNLOAD_SERVICE_ID, false)
            .unwrap();
        system_ability.register();
        info!("request service inited");
    }
    init
};
