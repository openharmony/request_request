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

use super::AppStateManagerTx;
use crate::service::client::ClientManagerEntry;
use crate::task::info::ApplicationState;

pub(crate) struct AppStateListener {
    client_manager: ClientManagerEntry,
    app_state_manager: AppStateManagerTx,
}

static mut APP_STATE_LISTENER: MaybeUninit<AppStateListener> = MaybeUninit::uninit();

impl AppStateListener {
    pub(crate) fn init(client_manager: ClientManagerEntry, app_state_manager: AppStateManagerTx) {
        info!("AppStateListener prepares to be inited");
        unsafe {
            APP_STATE_LISTENER.write(AppStateListener {
                client_manager,
                app_state_manager,
            });
            RegisterAPPStateCallback(app_state_change_callback);
            RegisterProcessStateCallback(process_state_change_callback);
        }

        info!("AppStateListener is inited");
    }
}

extern "C" fn app_state_change_callback(uid: i32, state: i32, pid: i32) {
    info!(
        "Receives app state change callback, uid is {}, pid is {}, state is {}",
        uid, pid, state
    );
    let state = match state {
        2 => ApplicationState::Foreground,
        4 => ApplicationState::Background,
        5 => ApplicationState::Terminated,
        _ => return,
    };

    unsafe {
        APP_STATE_LISTENER
            .assume_init_ref()
            .app_state_manager
            .change_app_state(uid as u64, state)
    };
}

extern "C" fn process_state_change_callback(uid: i32, state: i32, pid: i32) {
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

#[link(name = "download_server_cxx", kind = "static")]
extern "C" {
    fn RegisterAPPStateCallback(f: extern "C" fn(i32, i32, i32));
    fn RegisterProcessStateCallback(f: extern "C" fn(i32, i32, i32));
}
