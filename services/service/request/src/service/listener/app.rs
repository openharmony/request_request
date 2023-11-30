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

use crate::manager::events::EventMessage;
use crate::service::ability::RequestAbility;
use crate::task::info::ApplicationState;

pub(crate) struct AppStateListener;

impl AppStateListener {
    pub(crate) fn init() -> Self {
        info!("AppStateListener prepares to be inited");
        unsafe {
            RegisterAPPStateCallback(app_state_change_callback);
        }
        info!("AppStateListener is inited");
        Self
    }

    pub(crate) fn shutdown(&self) {
        // Considers remove the callback.
        info!("AppStateListener is stopped");
    }
}

extern "C" fn app_state_change_callback(uid: i32, state: i32) {
    info!("Receives app state change callback");

    let state = match state {
        2 => ApplicationState::Foreground,
        4 => ApplicationState::Background,
        5 => ApplicationState::Terminated,
        _ => return,
    };

    RequestAbility::task_manager().send_event(EventMessage::app_state_change(uid as u64, state));
}

extern "C" {
    fn RegisterAPPStateCallback(f: extern "C" fn(i32, i32));
}
