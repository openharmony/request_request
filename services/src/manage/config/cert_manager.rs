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
//! This module provides functionality to manage and update SSL/TLS certificates
//! used for secure network communications. It includes certificate loading from
//! system locations and user-provided sources, with automatic periodic updates.

use std::sync::{Arc, RwLock};
use std::time::Duration;

use ylong_http_client::Certificate;

use crate::utils::runtime_spawn;

/// Interval in seconds for automatic certificate updates.
const UPDATE_SYSTEM_CERT_INTERVAL_IN_SECS: u64 = 60 * 60;

/// Manages SSL/TLS certificates for secure network communications.
///
/// Provides functionality to load, store, and periodically update certificates
/// from system locations and user-provided sources.
#[derive(Clone)]
pub(crate) struct CertManager {
    /// Thread-safe storage for certificate information.
    info: Arc<RwLock<CertInfo>>,
}

impl CertManager {
    /// Initializes a new certificate manager with automatic update capabilities.
    ///
    /// # Returns
    ///
    /// A new instance of `CertManager` with an active background update task.
    ///
    /// # Notes
    ///
    /// Spawns a background task that periodically updates certificates at
    /// `UPDATE_SYSTEM_CERT_INTERVAL_IN_SECS` intervals.
    pub(crate) fn init() -> Self {
        let info = Arc::new(RwLock::new(CertInfo::default()));
        runtime_spawn(run(info.clone()));
        Self { info }
    }

    /// Retrieves the current certificates.
    ///
    /// # Returns
    ///
    /// An `Option` containing a vector of certificates if available, otherwise `None`.
    pub(crate) fn certificate(&self) -> Option<Vec<Certificate>> {
        self.info.read().unwrap().cert.clone()
    }

    /// Forces an immediate update of the certificates.
    ///
    /// Bypasses the periodic update interval and performs a certificate refresh
    /// immediately.
    pub(crate) fn force_update(&self) {
        update_system_cert(&self.info);
    }
}

/// Internal structure for storing certificate information.
#[derive(Default)]
struct CertInfo {
    /// Stored certificates, if available.
    cert: Option<Vec<Certificate>>,
}

/// Background task that periodically updates certificates.
///
/// # Arguments
///
/// * `info` - Thread-safe reference to certificate storage
///
/// # Notes
///
/// Runs indefinitely, updating certificates at the configured interval.
async fn run(info: Arc<RwLock<CertInfo>>) {
    loop {
        update_system_cert(&info);
        // Sleep for the configured update interval before refreshing certificates
        ylong_runtime::time::sleep(Duration::from_secs(UPDATE_SYSTEM_CERT_INTERVAL_IN_SECS)).await;
    }
}

/// Updates system certificates from various sources.
///
/// # Arguments
///
/// * `info` - Thread-safe reference to certificate storage
///
/// # Notes
///
/// Loads certificates from both user-provided sources and system certificate paths.
/// Returns early if any certificate parsing fails.
fn update_system_cert(info: &Arc<RwLock<CertInfo>>) {
    let mut info = info.write().unwrap();

    let mut certificates = Vec::new();

    // Load user certificates through the C API
    let c_certs_ptr = unsafe { GetUserCertsData() };
    if !c_certs_ptr.is_null() {
        info!("GetUserCertsData valid");
        let certs = unsafe { &*c_certs_ptr };
        // Convert C pointer array to safe Rust slice
        let c_cert_list_ptr =
            unsafe { std::slice::from_raw_parts(certs.cert_data_list, certs.len as usize) };
        for item in c_cert_list_ptr.iter() {
            let cert = unsafe { &**item };
            // Convert certificate data pointer to safe slice
            let cert_slice = unsafe { std::slice::from_raw_parts(cert.data, cert.size as usize) };
            // Parse PEM-encoded certificate
            match Certificate::from_pem(cert_slice) {
                Ok(cert) => {
                    certificates.push(cert);
                }
                Err(e) => {
                    error!("parse security cert path failed, error is {:?}", e);
                    return;
                }
            };
        }
        // Free the allocated C memory
        unsafe { FreeCertDataList(c_certs_ptr) };
    }

    // Load system certificates
    match Certificate::from_path("/system/etc/security/certificates/") {
        Ok(cert) => {
            certificates.push(cert);
        }
        Err(e) => {
            error!("parse security cert path failed, error is {:?}", e);
            return;
        }
    };

    // Update stored certificates
    *info = CertInfo {
        cert: Some(certificates),
    };
}

// C API functions for accessing user certificates
#[cfg(feature = "oh")]
extern "C" {
    /// Retrieves user certificate data from the system.
    ///
    /// # Returns
    ///
    /// A pointer to a structure containing certificate data or NULL if no certificates are available.
    pub(crate) fn GetUserCertsData() -> *const CRequestCerts;
    
    /// Frees memory allocated for certificate data list.
    ///
    /// # Arguments
    ///
    /// * `certs` - Pointer to the certificate data list to free
    pub(crate) fn FreeCertDataList(certs: *const CRequestCerts);
}

/// C-compatible representation of a single certificate.
#[repr(C)]
pub(crate) struct CRequestCert {
    /// Size of the certificate data in bytes.
    pub(crate) size: u32,
    /// Pointer to the certificate data.
    pub(crate) data: *const u8,
}

/// C-compatible representation of a certificate collection.
#[repr(C)]
pub(crate) struct CRequestCerts {
    /// Array of pointers to individual certificates.
    pub(crate) cert_data_list: *const *const CRequestCert,
    /// Number of certificates in the list.
    pub(crate) len: u32,
}

// Unit tests for certificate manager
#[cfg(feature = "oh")]
#[cfg(test)]
mod ut_cert_manager {
    include!("../../../tests/ut/manage/config/ut_cert_manager.rs");
}
