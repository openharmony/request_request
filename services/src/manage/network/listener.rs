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

use crate::manage::events::TaskManagerEvent;
use crate::manage::task_manager::TaskManagerTx;

pub(crate) struct NetworkChangeListener {
    task_manager: TaskManagerTx,
}

static mut NETWORK_CHANGE_LISTENER: MaybeUninit<NetworkChangeListener> = MaybeUninit::uninit();

impl NetworkChangeListener {
    pub(crate) fn init(task_manager: TaskManagerTx) {
        info!("NetworkChangeListener prepares to be inited");

        unsafe {
            NETWORK_CHANGE_LISTENER.write(NetworkChangeListener { task_manager });
            RegisterNetworkCallback(network_change_callback);
        }
        info!("NetworkChangeListener is inited");
    }
}

extern "C" fn network_change_callback() {
    info!("Receives network change callback");
    unsafe {
        NETWORK_CHANGE_LISTENER
            .assume_init_ref()
            .task_manager
            .send_event(TaskManagerEvent::network_change())
    };
}

#[link(name = "download_server_cxx", kind = "static")]
extern "C" {
    fn RegisterNetworkCallback(f: extern "C" fn());
}
