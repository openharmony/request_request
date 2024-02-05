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
use ipc_rust::{IRemoteBroker, RemoteObj};
use system_ability_fwk_rust::{IMethod, ISystemAbility, RSystemAbility};

use crate::{start, stop, RequestService, RequestServiceStub};

/// TEST_SERVICE_ID SAID
const REQUEST_SERVICE_ID: i32 = 3706;

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

struct SystemAbility {
    r_system_ability: RSystemAbility<SystemAbility>,
}

impl SystemAbility {
    /// Create a SystemAbility object
    #[allow(dead_code)]
    fn new_system_ability(said: i32, run_on_create: bool) -> Option<Self> {
        let r_system_ability = RSystemAbility::new(said, run_on_create);
        match r_system_ability {
            Some(r_system_ability) => Some(SystemAbility { r_system_ability }),
            None => {
                error!("RSystemAbility::new failed");
                None
            }
        }
    }
}

impl ISystemAbility for SystemAbility {
    /// Callback to deal safwk onstart for this system_ability
    fn on_start(&self) {
        info!("on_start");
        let service = match RequestServiceStub::new_remote_stub(RequestService) {
            Some(service) => service,
            None => {
                service_start_fault();
                panic!("create RequestService failed");
            }
        };

        let object = match service.as_object() {
            Some(object) => object,
            None => {
                service_start_fault();
                panic!("get request service failed");
            }
        };

        self.publish(&object, REQUEST_SERVICE_ID);
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

impl IMethod for SystemAbility {
    /// Call RSystemAbility<SystemAbility> register_ability
    fn register(&self) {
        self.r_system_ability.register_ability(self);
    }

    /// Call RSystemAbility<SystemAbility> publish
    fn publish(&self, service: &RemoteObj, said: i32) {
        self.r_system_ability.publish(service, said);
    }
}

#[used]
#[link_section = ".init_array"]
static A: extern "C" fn() = {
    extern "C" fn init() {
        info!("request service init");
        let system_ability = SystemAbility::new_system_ability(REQUEST_SERVICE_ID, false)
            .expect("create service failed");
        system_ability.register();
    }
    init
};
