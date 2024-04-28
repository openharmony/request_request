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

use crate::utils::c_wrapper::CStringWrapper;

#[derive(Clone)]
pub(crate) struct SystemProxyManager;

impl SystemProxyManager {
    pub(crate) fn init() -> Self {
        unsafe {
            RegisterProxySubscriber();
        }
        Self
    }

    pub(crate) fn host(&self) -> String {
        unsafe { GetHost() }.to_string()
    }

    pub(crate) fn port(&self) -> String {
        unsafe { GetPort() }.to_string()
    }

    pub(crate) fn exlist(&self) -> String {
        unsafe { GetExclusionList() }.to_string()
    }
}

#[link(name = "download_server_cxx", kind = "static")]
extern "C" {
    pub(crate) fn RegisterProxySubscriber();
    pub(crate) fn GetHost() -> CStringWrapper;
    pub(crate) fn GetPort() -> CStringWrapper;
    pub(crate) fn GetExclusionList() -> CStringWrapper;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_system_proxy_manager() {
        let system_proxy_manager = SystemProxyManager::init();
        let host = system_proxy_manager.host();
        let port = system_proxy_manager.port();
        let exlist = system_proxy_manager.exlist();
        println!("host: {}, port: {}, exlist: {}", host, port, exlist);
    }
}
