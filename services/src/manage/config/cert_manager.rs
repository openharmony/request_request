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

//! Certificate manager for handling SSL/TLS certificates in the request service.
//!
//! This module provides functionality to load SSL/TLS certificates on-demand
//! from system locations and user-provided sources.

use std::path::Path;

use ylong_http_client::Certificate;

/// System certificate path for system-trusted CA certificates
const SYSTEM_CERT_PATH: &str = "/system/etc/security/certificates/";
/// User certificate path template for user-trusted CA certificates
const USER_CERT_PATH_PREFIX: &str =
    "/data/service/el1/public/cert_manager_service/certificates/user_open/";
/// Constant for converting uid to user id
const UID_TO_USER_ID_DIVISOR: u64 = 200000;

/// Loads certificates from system paths and user paths on demand.
///
/// This function loads certificates from:
/// 1. System certificate path: /system/etc/security/certificates/
/// 2. User 0 certificate path (for app-managed certificates)
/// 3. Current user certificate path (only if developermode is enabled and uid is provided)
///
/// # Arguments
///
/// * `uid` - The uid from TaskConfig, used to calculate user id
///
/// # Returns
///
/// A vector of loaded certificates, or an empty vector if no certificates are found.
pub(crate) fn load_certificates_from_paths(uid: u64, tid: u32) -> Vec<Certificate> {
    let mut certificates = Vec::new();

    // Load system certificates
    add_certificate_from_path(SYSTEM_CERT_PATH, &mut certificates, tid);

    // Load user 0 certificates (app-managed certificates)
    let user_0_path = format!("{}0/", USER_CERT_PATH_PREFIX);
    add_certificate_from_path(&user_0_path, &mut certificates, tid);

    // Check developermode and load current user certificates if enabled
    if is_developermode_enabled() {
        let user_id = uid / UID_TO_USER_ID_DIVISOR;
        if user_id > 0 {
            let user_path = format!("{}{}/", USER_CERT_PATH_PREFIX, user_id);
            add_certificate_from_path(&user_path, &mut certificates, tid);
        }
    }

    certificates
}

fn add_certificate_from_path(certs_dir: &str, certificates: &mut Vec<Certificate>, tid: u32) {
    let path = Path::new(certs_dir);
    if path.is_dir() {
        match Certificate::from_path(certs_dir) {
            Ok(cert) => {
                certificates.push(cert);
            }
            Err(e) => {
                error!("{} failed to load certificates: {:?}", tid, e);
            }
        }
    }
}

/// Checks if developermode is enabled by calling C++ function.
///
/// # Returns
///
/// `true` if developermode is enabled, `false` otherwise.
fn is_developermode_enabled() -> bool {
    unsafe { IsDevelopermodeEnabled() }
}

extern "C" {
    /// C++ function to check if developermode is enabled
    fn IsDevelopermodeEnabled() -> bool;
}

#[cfg(feature = "oh")]
#[cfg(test)]
mod ut_cert_manager {
    use super::*;

    include!("../../../tests/ut/manage/config/ut_cert_manager.rs");
}
