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

//! This module is responsible for registering and publishing system services.

use std::mem::MaybeUninit;
use std::sync::Mutex;

use hisysevent::{build_number_param, write, EventType};
use samgr::definition::APP_MGR_SERVICE_ID;
use samgr::manage::SystemAbilityManager;
use system_ability_fwk::ability::{Ability, Handler};

use crate::manage::app_state::AppStateListener;
use crate::manage::events::{ScheduleEvent, TaskManagerEvent};
use crate::manage::task_manager::TaskManagerTx;
use crate::manage::{account, SystemConfigManager, TaskManager};
use crate::service::active_counter::ActiveCounter;
use crate::service::client::ClientManager;
use crate::service::run_count::RunCountManager;
use crate::service::RequestServiceStub;
use crate::utils::update_policy;

pub(crate) static mut PANIC_INFO: Option<String> = None;

pub(crate) static mut SYSTEM_CONFIG_MANAGER: MaybeUninit<SystemConfigManager> =
    MaybeUninit::uninit();

/// The structure of `Request System Ability`.
///
/// This structure is responsible for interacting with `System Ability Manager`.
pub struct RequestAbility {
    /// Channel sender used to dispatch schedule and task events to the task manager.
    task_manager: Mutex<Option<TaskManagerTx>>,
    /// Counter tracking in-flight operations, used to decide whether the service can idle.
    active_counter: ActiveCounter,
}

impl RequestAbility {
    /// Creates a new `RequestAbility`.
    pub fn new() -> Self {
        Self {
            active_counter: ActiveCounter::new(),
            task_manager: Mutex::new(None),
        }
    }

    /// Initializes the runtime subsystems and publishes the request service.
    ///
    /// Installs a panic hook, builds the global ylong runtime, and initializes
    /// the run-count manager, client manager, system config manager, and task
    /// manager. Subscribes to the app manager service and finally publishes the
    /// `RequestServiceStub` so that the system ability manager can route
    /// requests to it.
    ///
    /// # Arguments
    ///
    /// * `handler` - System ability handler used to publish the service stub.
    fn init(&self, handler: Handler) {
        info!("ability init");

        // Use a structure to handle panic.
        std::panic::set_hook(Box::new(|info| unsafe {
            let info = info.to_string();
            error!("{}", info);
            PANIC_INFO = Some(info);
        }));

        if let Err(e) = ylong_runtime::builder::RuntimeBuilder::new_multi_thread()
            .worker_num(4)
            .build_global()
        {
            error!("ylong_runtime error: {}", e);
        }
        info!("ylong_runtime init ok");

        let runcount_manager = RunCountManager::init();
        info!("runcount_manager init ok");

        let client_manger = ClientManager::init();
        info!("client_manger init ok");

        // Use methods to handle rather than directly accessing members.
        unsafe { SYSTEM_CONFIG_MANAGER.write(SystemConfigManager::init()) };
        info!("system_config_manager init ok");

        let task_manager = TaskManager::init(
            runcount_manager.clone(),
            client_manger.clone(),
            self.active_counter.clone(),
        );
        *self.task_manager.lock().unwrap() = Some(task_manager.clone());
        info!("task_manager init ok");

        AppStateListener::init(client_manger.clone(), task_manager.clone());

        SystemAbilityManager::subscribe_system_ability(
            APP_MGR_SERVICE_ID,
            |_, _| {
                info!("app manager service init");
                AppStateListener::register();
            },
            |_, _| {
                error!("app manager service died");
            },
        );

        let stub = RequestServiceStub::new(
            handler.clone(),
            task_manager,
            client_manger,
            runcount_manager,
            self.active_counter.clone(),
        );

        info!("ability init succeed");
        if !handler.publish(stub) {
            service_start_fault();
        }
    }
}

impl Ability for RequestAbility {
    /// Handles system ability start triggered by an on-demand reason.
    ///
    /// When the reason is a user-removed event, the tasks belonging to that
    /// user are cleaned up. Afterwards the service is initialized and the
    /// update policy is applied.
    ///
    /// # Arguments
    ///
    /// * `reason` - The on-demand reason that triggered the start.
    /// * `handler` - System ability handler used to initialize the service.
    fn on_start_with_reason(
        &self,
        reason: system_ability_fwk::cxx_share::SystemAbilityOnDemandReason,
        handler: Handler,
    ) {
        info!("on_start_with_reason: {:?}", reason);
        if reason.name == "usual.event.USER_REMOVED" {
            match reason.value.parse::<i32>() {
                Ok(user_id) => {
                    account::remove_account_tasks(user_id);
                }
                Err(e) => {
                    error!("on_start_with_reason err {}", e);
                }
            }
        }
        self.init(handler);
        const INIT_POLICY: bool = false;
        let _ = update_policy(INIT_POLICY);
    }

    /// Handles the system ability becoming active.
    ///
    /// Sends a restart-countdown schedule event to the task manager so that
    /// pending retry timers are refreshed.
    ///
    /// # Arguments
    ///
    /// * `reason` - The on-demand reason that activated the service.
    fn on_active(&self, reason: system_ability_fwk::cxx_share::SystemAbilityOnDemandReason) {
        info!("on_active: {:?}", reason);
        if let Some(task_manager) = self.task_manager.lock().unwrap().as_ref() {
            task_manager.send_event(TaskManagerEvent::Schedule(ScheduleEvent::RestartCountDown));
        }
    }

    /// Handles the system ability entering idle, deciding whether to shut down.
    ///
    /// Rejects idle (returns `-1`) while there are active operations, so that
    /// in-flight tasks are not interrupted. When idle, a shutdown schedule
    /// event is sent to the task manager and `0` is returned.
    ///
    /// # Arguments
    ///
    /// * `reason` - The on-demand reason that prompted the idle check.
    ///
    /// # Returns
    ///
    /// `0` if the service may idle and shut down, or `-1` to reject idle.
    fn on_idle(&self, reason: system_ability_fwk::cxx_share::SystemAbilityOnDemandReason) -> i32 {
        if self.active_counter.is_active() {
            info!("remote is busy reject idle, reason: {:?}", reason);
            -1
        } else {
            info!("remote not busy accept idle, reason: {:?}", reason);
            if let Some(task_manager) = self.task_manager.lock().unwrap().as_ref() {
                task_manager.send_event(TaskManagerEvent::Schedule(ScheduleEvent::Shutdown));
            }
            0
        }
    }

    /// Handles device resource level changes reported by the system.
    ///
    /// Forwards the reported level to the task manager as a device event so
    /// that running tasks can react to resource constraints.
    ///
    /// # Arguments
    ///
    /// * `change_type` - The type of resource level change.
    /// * `level` - The new resource level.
    /// * `action` - The action associated with the change.
    fn on_device_level_changed(&self, change_type: i32, level: i32, action: String) {
        info!(
            "on_device_level_changed type {} level {} action {}",
            change_type, level, action
        );
        if let Some(task_manager) = self.task_manager.lock().unwrap().as_ref() {
            task_manager.send_event(TaskManagerEvent::Device(level));
        }
    }
}

#[cfg(not(test))]
#[used]
#[link_section = ".init_array"]
static A: extern "C" fn() = {
    #[link_section = ".text.startup"]
    // Early initializer placed in the `.init_array` section so it runs before
    // `main`; builds and registers the request system ability with samgr.
    extern "C" fn init() {
        info!("begin request service init");
        if let Some(system_ability) = RequestAbility::new()
            .build_system_ability(samgr::definition::DOWNLOAD_SERVICE_ID, false)
        {
            system_ability.register();
            info!("request service inited");
        } else {
            info!("request service inited error");
        }
    }
    init
};

// TODO: Use `SysEvent` instead.
/// Reports a service start fault through the HiSysEvent framework.
///
/// Emits a `SERVICE_START_FAULT` fault event tagged with the publish-failure
/// error code so that service start failures are visible to diagnostics.
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
