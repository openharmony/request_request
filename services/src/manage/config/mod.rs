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

mod cert_manager;
mod system_proxy;

use cert_manager::CertManager;
use system_proxy::SystemProxyManager;
use ylong_http_client::Certificate;

#[derive(Clone)]
pub(crate) struct SystemConfigManager {
    cert: CertManager,
    proxy: SystemProxyManager,
}

impl SystemConfigManager {
    pub(crate) fn init() -> Self {
        Self {
            cert: CertManager::init(),
            proxy: SystemProxyManager::init(),
        }
    }

    pub(crate) fn system_config(&self) -> SystemConfig {
        let mut certs = self.cert.certificate();

        if certs.is_none() {
            self.cert.force_update();
            certs = self.cert.certificate();
        }

        SystemConfig {
            proxy_host: self.proxy.host(),
            proxy_port: self.proxy.port(),
            proxy_exlist: self.proxy.exlist(),
            certs,
        }
    }
}

pub(crate) struct SystemConfig {
    pub(crate) proxy_host: String,
    pub(crate) proxy_port: String,
    pub(crate) proxy_exlist: String,
    pub(crate) certs: Option<Vec<Certificate>>,
}
