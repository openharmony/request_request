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

// Copyright (C) 2023 Huawei Device Co., Ltd.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// limitations under the License.

//! Request ability services implementations.

use std::mem::MaybeUninit;

use crate::manage::task_manager::TaskManagerEntry;
use crate::manage::TaskManager;
use crate::service::client::{ClientManager, ClientManagerEntry};
use crate::service::listener::{AppStateListener, NetworkChangeListener};
use crate::service::runcount::{RunCountManager, RunCountManagerEntry};

pub(crate) static mut PANIC_INFO: Option<String> = None;

static mut RUN_COUNT: MaybeUninit<RunCountManagerEntry> = MaybeUninit::uninit();
static mut TASK_MANAGER: MaybeUninit<TaskManagerEntry> = MaybeUninit::uninit();
static mut APP: MaybeUninit<AppStateListener> = MaybeUninit::uninit();
static mut NETWORK: MaybeUninit<NetworkChangeListener> = MaybeUninit::uninit();
static mut CLIENT_MANAGER: MaybeUninit<ClientManagerEntry> = MaybeUninit::uninit();

pub(crate) struct RequestAbility;

impl RequestAbility {
    pub(crate) fn init() {
        std::panic::set_hook(Box::new(|info| unsafe {
            let trace = std::backtrace::Backtrace::force_capture();
            let mut info = info.to_string();
            info.push_str(trace.to_string().as_str());
            error!("{}", info);
            PANIC_INFO = Some(info);
        }));

        ylong_runtime::builder::RuntimeBuilder::new_multi_thread()
            .worker_num(4)
            .build_global()
            .unwrap();

        unsafe {
            // first init RunCountManager to record Running task count

            RUN_COUNT.write(RunCountManager::init());
            TASK_MANAGER.write(TaskManager::init());
            APP.write(AppStateListener::init());
            NETWORK.write(NetworkChangeListener::init());
            CLIENT_MANAGER.write(ClientManager::init());
            RequestInitServiceHandler();
        };
    }

    pub(crate) fn runcount_manager() -> RunCountManagerEntry {
        unsafe { RUN_COUNT.assume_init_ref().clone() }
    }

    pub(crate) fn task_manager() -> TaskManagerEntry {
        unsafe { TASK_MANAGER.assume_init_ref().clone() }
    }

    pub(crate) fn client_manager() -> ClientManagerEntry {
        unsafe { CLIENT_MANAGER.assume_init_ref().clone() }
    }
}

#[cfg(feature = "oh")]
#[link(name = "request_service_c")]
extern "C" {
    pub(crate) fn RequestInitServiceHandler();
}
