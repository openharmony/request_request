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

//! Proxy interface for communicating with the download service.
//! 
//! This module provides a singleton proxy implementation that handles communication with
//! the download service through IPC. It manages service state, provides access to remote
//! objects, and serves as the foundation for all service interactions.

// Submodules
mod notification; // Handles notification-related functionality
mod query; // Provides task query capabilities
mod state; // Manages service state tracking
mod task; // Implements task management operations
mod uds; // Handles Unix Domain Socket communication

/// Service token identifier for the download request service.
///
/// Used to identify and connect to the download service through the IPC mechanism.
const SERVICE_TOKEN: &str = "OHOS.Download.RequestServiceInterface";

// Standard library imports
use std::sync::{Arc, LazyLock, Mutex};

// External dependencies
use ipc::remote::RemoteObj;
use request_core::error_code::EXCEPTION_SERVICE;

// Local dependencies
use state::SaState;

/// Proxy for interacting with the download service through IPC.
///
/// Implements the singleton pattern to provide a single point of access to the
/// download service. Manages connection state and provides methods to obtain
/// the remote service object for IPC calls.
pub struct RequestProxy {
    /// Service state protected by a mutex for thread safety
    remote: Mutex<SaState>,
}

impl RequestProxy {
    /// Returns the singleton instance of `RequestProxy`.
    ///
    /// Creates the instance on first call using `LazyLock` for thread-safe initialization.
    /// Subsequent calls return the same instance.
    ///
    /// # Returns
    /// A static reference to the singleton `RequestProxy` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use request_next::proxy::RequestProxy;
    ///
    /// // Get the singleton instance
    /// let proxy = RequestProxy::get_instance();
    ///
    /// // Verify pointer equality (both references point to the same instance)
    /// let proxy2 = RequestProxy::get_instance();
    /// assert!(std::ptr::eq(proxy, proxy2));
    /// ```
    pub fn get_instance() -> &'static Self {
        static REQUEST_PROXY: LazyLock<RequestProxy> = LazyLock::new(|| RequestProxy {
            remote: Mutex::new(SaState::update()),
        });
        &REQUEST_PROXY
    }

    /// Retrieves the remote service object for IPC communication.
    ///
    /// Checks if the service state is ready. If not, attempts to reconnect if the
    /// last failure occurred more than 5 seconds ago.
    ///
    /// # Returns
    /// A `Result` containing either:
    /// - `Ok(Arc<RemoteObj>)`: The remote service object for making IPC calls
    /// - `Err(i32)`: Error code if the service could not be accessed
    ///
    /// # Errors
    /// - Returns `EXCEPTION_SERVICE` if the service is not available or cannot be reconnected
    ///
    /// # Safety
    /// This method is marked as `pub(crate)` to restrict access to the module's internal API.
    pub(crate) fn remote(&self) -> Result<Arc<RemoteObj>, i32> {
        let mut remote = self.remote.lock().unwrap();
        match *remote {
            // If service is ready, return the remote object
            SaState::Ready(ref obj) => return Ok(obj.clone()),
            // If service is invalid, attempt to reconnect after a delay
            SaState::Invalid(ref time) => {
                // Only attempt reconnection after 5 seconds to prevent excessive reconnection attempts
                if time.elapsed().as_secs() > 5 {
                    *remote = SaState::update();
                    if let SaState::Ready(ref obj) = *remote {
                        return Ok(obj.clone());
                    }
                }
            }
        }
        // Log error and return exception code
        error!("request systemAbility load failed");
        Err(EXCEPTION_SERVICE)
    }
}
