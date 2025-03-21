// Copyright (C) 2025 Huawei Device Co., Ltd.
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

use std::sync::{Arc, Mutex};

use ffi::{NetInfo, NetUnregistration};

use super::Observer;

pub struct NetObserverWrapper {
    inner: Arc<Mutex<Vec<Box<dyn Observer>>>>,
}

impl NetObserverWrapper {
    pub fn new(inner: Arc<Mutex<Vec<Box<dyn Observer>>>>) -> Self {
        Self { inner }
    }
}

impl NetObserverWrapper {
    pub(crate) fn net_available(&self, net_id: i32) {
        let inner = self.inner.lock().unwrap();
        for observer in inner.iter() {
            observer.net_available(net_id);
        }
    }

    pub(crate) fn net_lost(&self, net_id: i32) {
        let inner = self.inner.lock().unwrap();
        for observer in inner.iter() {
            observer.net_lost(net_id);
        }
    }

    pub(crate) fn net_capability_changed(&self, net_id: i32, net_info: NetInfo) {
        let inner = self.inner.lock().unwrap();
        for observer in inner.iter() {
            observer.net_capability_changed(net_id, &net_info);
        }
    }
}

unsafe impl Send for NetUnregistration {}
unsafe impl Sync for NetUnregistration {}

#[cxx::bridge(namespace = "OHOS::Request")]
pub mod ffi {
    #[namespace = "OHOS::NetManagerStandard"]
    #[derive(Debug)]
    #[repr(i32)]
    enum NetCap {
        NET_CAPABILITY_MMS = 0,
        NET_CAPABILITY_SUPL = 1,
        NET_CAPABILITY_DUN = 2,
        NET_CAPABILITY_IA = 3,
        NET_CAPABILITY_XCAP = 4,
        NET_CAPABILITY_BIP = 5,
        NET_CAPABILITY_NOT_METERED = 11,
        NET_CAPABILITY_INTERNET = 12,
        NET_CAPABILITY_NOT_VPN = 15,
        NET_CAPABILITY_VALIDATED = 16,
        NET_CAPABILITY_PORTAL = 17,
        NET_CAPABILITY_INTERNAL_DEFAULT = 18,
        NET_CAPABILITY_CHECKING_CONNECTIVITY = 31,
        NET_CAPABILITY_END = 32,
    }

    #[namespace = "OHOS::NetManagerStandard"]
    #[derive(Debug)]
    #[repr(i32)]
    enum NetBearType {
        BEARER_CELLULAR = 0,
        BEARER_WIFI = 1,
        BEARER_BLUETOOTH = 2,
        BEARER_ETHERNET = 3,
        BEARER_VPN = 4,
        BEARER_WIFI_AWARE = 5,
        BEARER_DEFAULT,
    }

    #[derive(Debug)]
    struct NetInfo {
        caps: Vec<NetCap>,
        bear_types: Vec<NetBearType>,
    }

    extern "Rust" {
        type NetObserverWrapper;

        fn net_available(&self, net_id: i32);
        fn net_lost(&self, net_id: i32);
        fn net_capability_changed(&self, net_id: i32, net_info: NetInfo);
    }

    unsafe extern "C++" {
        include!("net_all_capabilities.h");
        include!("request_utils_network.h");

        #[namespace = "OHOS::NetManagerStandard"]
        type NetCap;
        #[namespace = "OHOS::NetManagerStandard"]
        type NetBearType;

        type NetUnregistration;
        fn unregister(self: &NetUnregistration) -> i32;

        #[allow(unused)]
        fn RegisterNetObserver(
            wrapper: Box<NetObserverWrapper>,
            error: &mut i32,
        ) -> UniquePtr<NetUnregistration>;
    }
}
