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

use std::mem::MaybeUninit;

use super::task_manager::TaskManagerTx;
use crate::service::client::ClientManagerEntry;

pub(crate) struct AppStateListener {
    client_manager: ClientManagerEntry,
    task_manager: TaskManagerTx,
}

static mut APP_STATE_LISTENER: MaybeUninit<AppStateListener> = MaybeUninit::uninit();

impl AppStateListener {
    pub(crate) fn init(client_manager: ClientManagerEntry, task_manager: TaskManagerTx) {
        info!("AppStateListener prepares to be inited");
        unsafe {
            APP_STATE_LISTENER.write(AppStateListener {
                client_manager,
                task_manager,
            });
            #[cfg(feature = "oh")]
            {
                RegisterAPPStateCallback(app_state_change_callback);
                RegisterProcessStateCallback(process_state_change_callback);
            }
        }
        info!("AppStateListener is inited");
    }
}

extern "C" fn app_state_change_callback(uid: i32, state: i32, _pid: i32) {
    if state == 2 {
        unsafe {
            APP_STATE_LISTENER
                .assume_init_ref()
                .task_manager
                .notify_foreground_app_change(uid as u64)
        };
    } else if state == 4 {
        unsafe {
            APP_STATE_LISTENER
                .assume_init_ref()
                .task_manager
                .notify_app_background(uid as u64)
        };
    }
}

extern "C" fn process_state_change_callback(uid: i32, state: i32, pid: i32) {
    debug!(
        "Receives process change notify, uid is {}, pid is {}, state is {}",
        uid, pid, state
    );
    if state == 5 {
        info!(
            "Receives process died notify, uid is {}, pid is {}",
            uid, pid
        );
        unsafe {
            APP_STATE_LISTENER
                .assume_init_ref()
                .client_manager
                .notify_process_terminate(pid as u64)
        };
    }
}

#[cfg(feature = "oh")]
extern "C" {
    fn RegisterAPPStateCallback(f: extern "C" fn(i32, i32, i32));
    fn RegisterProcessStateCallback(f: extern "C" fn(i32, i32, i32));
}
