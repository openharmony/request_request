// Copyright (C) 2024 Huawei Device Co., Ltd.
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

//! Provides URL domain policy checking functionality.
//! 
//! This module contains utilities for verifying if an application has permission to access
//! specific URL domains based on domain type policies.

use crate::utils::c_wrapper::CStringWrapper;

/// Checks if an application is allowed to access a specific URL domain.
///
/// This function validates whether the given application has permission to access
/// the specified URL based on the provided domain type policy.
///
/// # Parameters
/// - `app_id`: The identifier of the application requesting access
/// - `domain_type`: The type of domain being checked (e.g., "network", "download", etc.)
/// - `url`: The URL being accessed that needs validation
///
/// # Returns
/// - `Some(true)` if the application is allowed to access the URL
/// - `Some(false)` if the application is explicitly denied access to the URL
/// - `None` if the policy check failed or returned an unexpected result
///
/// # Examples
/// ```rust
/// // Check if app has permission to access a URL
/// let result = check_url_domain("com.example.app", "network", "https://example.com");
/// match result {
///     Some(true) => println!("Access allowed"),
///     Some(false) => println!("Access denied"),
///     None => println!("Policy check failed"),
/// }
/// ```
///
/// # Safety
/// This function calls an unsafe external C function to perform the actual policy check.
/// String conversion to C-compatible format is handled internally.
pub(crate) fn check_url_domain(app_id: &str, domain_type: &str, url: &str) -> Option<bool> {
    // Call external C policy check function with string conversions
    match unsafe { PolicyCheckUrlDomain(app_id.into(), domain_type.into(), url.into()) } {
        // Map C return codes to Rust Option<bool>
        0 => Some(true),   // 0 indicates allowed
        1 => Some(false),  // 1 indicates denied
        _ => None,         // Any other value indicates failure
    }
}

// External C function that performs the actual URL domain policy check.
//
// This is a raw FFI binding to a C function that implements the domain policy validation.
//
// # Safety
// - This function is unsafe because it's an external C function without Rust's safety guarantees.
// - Callers must ensure proper string lifetimes and valid pointers.
//
// # Returns
// - `0`: Access allowed
// - `1`: Access denied
// - Any other value: Error or unexpected result
extern "C" {
    pub(crate) fn PolicyCheckUrlDomain(
        app_id: CStringWrapper,
        domain_type: CStringWrapper,
        url: CStringWrapper,
    ) -> i32;
}
