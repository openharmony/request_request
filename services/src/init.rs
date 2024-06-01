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

use std::mem::MaybeUninit;
use std::sync::Mutex;

use hisysevent::{build_number_param, write, EventType};
use samgr::definition::DOWNLOAD_SERVICE_ID;
use system_ability_fwk::ability::{Ability, Handler};

use crate::manage::network::listener::NetworkChangeListener;
use crate::manage::{SystemConfigManager, TaskManager};
use crate::service::client::ClientManager;
use crate::service::runcount::RunCountManager;

pub(crate) static mut PANIC_INFO: Option<String> = None;
use crate::manage::events::TaskManagerEvent;
use crate::manage::task_manager::TaskManagerTx;
use crate::service::RequestServiceStub;

/// TEST_SERVICE_ID SAID
pub(crate) static mut SYSTEM_CONFIG_MANAGER: MaybeUninit<SystemConfigManager> =
    MaybeUninit::uninit();

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

struct RequestAbility {
    task_manager: Mutex<Option<TaskManagerTx>>,
}

impl RequestAbility {
    fn new() -> Self {
        Self {
            task_manager: Mutex::new(None),
        }
    }
}

impl Ability for RequestAbility {
    /// Callback to deal safwk onstart for this system_ability
    fn on_start(&self, handler: Handler) {
        info!("ability init");

        std::panic::set_hook(Box::new(|info| unsafe {
            let info = info.to_string();
            error!("{}", info);
            PANIC_INFO = Some(info);
        }));

        ylong_runtime::builder::RuntimeBuilder::new_multi_thread()
            .worker_num(4)
            .build_global()
            .unwrap();
        info!("ylong_runtime init succeed");

        let runcount_manager = RunCountManager::init();
        info!("runcount_manager init succeed");

        let client_manger = ClientManager::init();
        info!("client_manger init succeed");

        let task_manager = TaskManager::init(runcount_manager.clone(), client_manger.clone());

        *self.task_manager.lock().unwrap() = Some(task_manager.clone());

        info!("task_manager init succeed");

        NetworkChangeListener::init(task_manager.clone());
        info!("network_change_listener init succeed");

        unsafe { SYSTEM_CONFIG_MANAGER.write(SystemConfigManager::init()) };

        info!("system_config_manager init succeed");

        unsafe {
            RequestInitServiceHandler();
        }

        let stub = RequestServiceStub::new(task_manager, client_manger, runcount_manager);

        info!("ability init succeed");
        if !handler.publish(stub) {
            service_start_fault();
        }
        info!("ability publish succeed");
    }

    fn on_device_level_changed(&self, change_type: i32, level: i32, action: String) {
        info!(
            "on_device_level_changed change_type: {}, level: {}, action: {}",
            change_type, level, action
        );
        if let Some(task_manager) = self.task_manager.lock().unwrap().as_ref() {
            task_manager.send_event(TaskManagerEvent::Device(level));
        }
    }
}

#[used]
#[link_section = ".init_array"]
static A: extern "C" fn() = {
    extern "C" fn init() {
        info!("begin request service init");
        let system_ability = RequestAbility::new()
            .build_system_ability(DOWNLOAD_SERVICE_ID, false)
            .unwrap();
        system_ability.register();
        info!("request service inited");
    }
    init
};

#[link(name = "download_server_cxx", kind = "static")]
extern "C" {
    pub(crate) fn RequestInitServiceHandler();
}
