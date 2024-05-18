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

pub(crate) mod listener;

use crate::task::ffi::{GetNetworkInfo, NetworkInfo, UpdateNetworkInfo};

pub(crate) struct NetworkManager;

impl NetworkManager {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn is_online(&self) -> bool {
        unsafe { IsOnline() }
    }

    pub(crate) fn update_network_info(&self) {
        unsafe {
            UpdateNetworkInfo();
        }
    }

    pub(crate) fn get_network_info(&self) -> Option<NetworkInfo> {
        let network_info = unsafe { GetNetworkInfo() };
        if network_info.is_null() {
            info!("get_network_info fail");
            return None;
        }
        let network_info = unsafe { *network_info };
        Some(network_info)
    }
}

#[link(name = "download_server_cxx", kind = "static")]
extern "C" {
    pub(crate) fn IsOnline() -> bool;
}
