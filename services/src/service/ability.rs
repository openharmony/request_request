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

static mut REQUEST_ABILITY: MaybeUninit<RequestAbility> = MaybeUninit::uninit();

pub(crate) static mut PANIC_INFO: Option<String> = None;

pub(crate) struct RequestAbility {
    runcount: RunCountManagerEntry,
    manager: TaskManagerEntry,
    app: AppStateListener,
    network: NetworkChangeListener,
    client_manager: ClientManagerEntry,
}

impl RequestAbility {
    // `init` must have been called before calling `get_instance`.
    pub(crate) fn get_instance() -> &'static Self {
        unsafe { &*REQUEST_ABILITY.as_ptr() }
    }

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
            REQUEST_ABILITY.write(Self {
                // first init RunCountManager to record Running task count
                runcount: RunCountManager::init(),
                manager: TaskManager::init(),
                app: AppStateListener::init(),
                network: NetworkChangeListener::init(),
                client_manager: ClientManager::init(),
            });
            RequestInitServiceHandler();
        };
    }

    pub(crate) fn stop() {
        unsafe {
            let ability = REQUEST_ABILITY.assume_init_ref();
            // After entries shutdown, the `rx`s of these channels will be dropped.
            ability.app.shutdown();
            ability.network.shutdown();
            ability.runcount.shutdown();
        };
    }

    pub(crate) fn runcount_manager() -> RunCountManagerEntry {
        Self::get_instance().runcount.clone()
    }

    pub(crate) fn task_manager() -> TaskManagerEntry {
        Self::get_instance().manager.clone()
    }

    pub(crate) fn client_manager() -> ClientManagerEntry {
        Self::get_instance().client_manager.clone()
    }
}

#[cfg(feature = "oh")]
#[link(name = "request_service_c")]
extern "C" {
    pub(crate) fn RequestInitServiceHandler();
}
