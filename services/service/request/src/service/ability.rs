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

use std::hint;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicU8, Ordering};

use crate::manager::task_manager::TaskManagerEntry;
use crate::manager::TaskManager;
use crate::service::listener::{AppStateListener, NetworkChangeListener};
use crate::service::notify::{NotifyEntry, NotifyManager};

static mut REQUEST_ABILITY: MaybeUninit<RequestAbility> = MaybeUninit::uninit();
static STATE: AtomicU8 = AtomicU8::new(RequestAbility::NOT_INITED);

pub(crate) struct RequestAbility {
    manager: TaskManagerEntry,
    notify: NotifyEntry,
    app: AppStateListener,
    network: NetworkChangeListener,
}

impl RequestAbility {
    const NOT_INITED: u8 = 0;
    const INITIALIZING: u8 = 1;
    const RUNNING: u8 = 2;
    const STOPPING: u8 = 3;
    const STOPPED: u8 = 4;

    // `init` must have been called before calling `get_instance`.
    pub(crate) fn get_instance() -> &'static Self {
        loop {
            match STATE.load(Ordering::SeqCst) {
                Self::RUNNING | Self::STOPPED => return unsafe { &*REQUEST_ABILITY.as_ptr() },
                _ => hint::spin_loop(),
            }
        }
    }

    pub(crate) fn init() {
        if STATE
            .compare_exchange(
                Self::NOT_INITED,
                Self::INITIALIZING,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .is_ok()
        {
            unsafe {
                REQUEST_ABILITY.write(Self {
                    manager: TaskManager::init(),
                    notify: NotifyManager::init(),
                    app: AppStateListener::init(),
                    network: NetworkChangeListener::init(),
                });
                RequestInitServiceHandler();
            };
            STATE.store(Self::RUNNING, Ordering::SeqCst);
        }
    }

    pub(crate) fn stop() {
        if STATE
            .compare_exchange(
                Self::RUNNING,
                Self::STOPPING,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .is_ok()
        {
            unsafe {
                let ability = REQUEST_ABILITY.assume_init_ref();
                // After entries shutdown, the `rx`s of these channels will be dropped.
                ability.notify.shutdown();
                ability.app.shutdown();
                ability.network.shutdown();
            };
            STATE.store(Self::STOPPED, Ordering::SeqCst);
        }
    }

    pub(crate) fn notify() -> NotifyEntry {
        Self::get_instance().notify.clone()
    }

    pub(crate) fn task_manager() -> TaskManagerEntry {
        Self::get_instance().manager.clone()
    }
}

extern "C" {
    pub(crate) fn RequestInitServiceHandler();
}
