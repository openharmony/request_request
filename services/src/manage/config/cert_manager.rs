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

use std::fs::File;
use std::io::Read;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

use ylong_http_client::Certificate;

const UPDATE_SYSTEM_CERT_INTERVAL_IN_SECS: u64 = 60 * 60;

#[derive(Clone)]
pub(crate) struct CertManager {
    info: Arc<RwLock<CertInfo>>,
}

impl CertManager {
    pub(crate) fn init() -> Self {
        let info = Arc::new(RwLock::new(CertInfo::default()));
        ylong_runtime::spawn(run(info.clone()));
        Self { info }
    }

    pub(crate) fn certificate(&self) -> Option<Vec<Certificate>> {
        self.info.read().unwrap().cert.clone()
    }

    pub(crate) fn force_update(&self) {
        update_system_cert(&self.info);
    }
}

#[derive(Default)]
struct CertInfo {
    time: Option<SystemTime>,
    cert: Option<Vec<Certificate>>,
}

async fn run(info: Arc<RwLock<CertInfo>>) {
    loop {
        update_system_cert(&info);
        ylong_runtime::time::sleep(Duration::from_secs(UPDATE_SYSTEM_CERT_INTERVAL_IN_SECS)).await;
    }
}

// Try use `async` func to read file.
fn update_system_cert(info: &Arc<RwLock<CertInfo>>) {
    let mut info = info.write().unwrap();

    let mut file = match File::open("/etc/ssl/certs/cacert.pem") {
        Ok(file) => file,
        Err(e) => {
            error!("open cacert.pem failed, error is {:?}", e);
            return;
        }
    };

    let modified = match file.metadata().and_then(|meta| meta.modified()) {
        Ok(modified) => Some(modified),
        Err(e) => {
            error!("open cacert.pem failed, error is {:?}", e);
            return;
        }
    };

    // If the certificate file has not been updated, there is no need to update
    // `CertInfo`.
    if info.time == modified {
        return;
    }

    let mut buf = Vec::new();
    if let Err(e) = file.read_to_end(&mut buf) {
        error!("read cacert.pem failed, error is {:?}", e);
        return;
    }

    let mut cert_from_pem = match Certificate::from_pem(&buf) {
        Ok(cert) => CertInfo {
            time: modified,
            cert: Some(vec![cert]),
        },
        Err(e) => {
            error!("parse cacert.pem failed, error is {:?}", e);
            return;
        }
    };

    let c_certs_ptr = unsafe { GetUserCertsData() };
    if !c_certs_ptr.is_null() {
        error!("GetUserCertsData is not ptr");
        let certs = unsafe { &*c_certs_ptr };
        let c_cert_list_ptr =
            unsafe { std::slice::from_raw_parts(certs.cert_data_list, certs.len as usize) };
        for (_, item) in c_cert_list_ptr.iter().enumerate() {
            let cert = unsafe { &**item };
            let cert_slice = unsafe { std::slice::from_raw_parts(cert.data, cert.size as usize) };
            cert_from_pem = match Certificate::from_pem(cert_slice) {
                Ok(cert) => {
                    cert_from_pem.cert.as_mut().unwrap().push(cert);
                    cert_from_pem
                }
                Err(e) => {
                    error!("parse security cert path failed, error is {:?}", e);
                    return;
                }
            };
        }
        unsafe { FreeCertDataList(c_certs_ptr) };
    }

    let cert = match Certificate::from_path("/system/etc/security/certificates/") {
        Ok(cert) => {
            cert_from_pem.cert.as_mut().unwrap().push(cert);
            cert_from_pem
        }
        Err(e) => {
            error!("parse security cert path failed, error is {:?}", e);
            return;
        }
    };

    *info = cert;
}

#[link(name = "download_server_cxx", kind = "static")]
extern "C" {
    pub(crate) fn GetUserCertsData() -> *const CRequestCerts;
    pub(crate) fn FreeCertDataList(certs: *const CRequestCerts);
}

#[repr(C)]
pub(crate) struct CRequestCert {
    pub(crate) size: u32,
    pub(crate) data: *const u8,
}

#[repr(C)]
pub(crate) struct CRequestCerts {
    pub(crate) cert_data_list: *const *const CRequestCert,
    pub(crate) len: u32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cert_manager() {
        let cert_manager = CertManager::init();
        let cert = cert_manager.certificate();
        if cert.is_none() {
            cert_manager.force_update();
        }
        assert!(cert_manager.certificate().is_some());
    }
}
