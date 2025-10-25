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

//! System Ability state management for download service.
//! 
//! This module defines the state management for the download service's System Ability (SA).
//! It provides functionality to track whether the service is ready or invalid, and to
//! attempt to load the System Ability with automatic retry logic.

// Standard library dependencies
use std::sync::Arc;
use std::time::{self, Instant};

// IPC and service management dependencies
use ipc::remote::RemoteObj;
use samgr::definition::DOWNLOAD_SERVICE_ID;
use samgr::manage::SystemAbilityManager;

pub(crate) enum SaState {
    /// The System Ability is ready to use with the provided remote object.
    Ready(Arc<RemoteObj>),
    
    /// The System Ability is invalid, with the timestamp when it became invalid.
    Invalid(time::Instant),
}

impl SaState {
    /// Attempts to load the download service System Ability with retry logic.
    ///
    /// Tries to load the System Ability up to 5 times with a 5-second delay between attempts.
    /// Returns `SaState::Ready` if successful, or `SaState::Invalid` if all attempts fail.
    ///
    /// # Returns
    /// - `SaState::Ready` with an `Arc<RemoteObj>` if the System Ability is successfully loaded
    /// - `SaState::Invalid` with the current time if all 5 attempts to load fail
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use request_next::proxy::state::SaState;
    ///
    /// fn example() {
    ///     // Attempt to load the download service System Ability
    ///     let state = SaState::update();
    ///     
    ///     match state {
    ///         SaState::Ready(remote) => println!("System Ability loaded successfully"),
    ///         SaState::Invalid(timestamp) => println!("Failed to load System Ability"),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// This method uses a fixed retry count of 5 and a 5-second delay between retries. If all
    /// attempts fail, it records the current time as the point when the state became invalid.
    pub(crate) fn update() -> Self {
        // Try to load the System Ability up to 5 times with retries
        for _ in 0..5 {
            match SystemAbilityManager::load_system_ability(DOWNLOAD_SERVICE_ID, 1000) {
                Some(remote) => {
                    // Successfully loaded, return Ready state with the remote object
                    return SaState::Ready(Arc::new(remote));
                }
                None => {
                    // Failed to load, wait 5 seconds before retrying
                    std::thread::sleep(std::time::Duration::from_millis(5000));
                    error!("request systemAbility load failed, retrying...");
                }
            }
        }
        // All retries failed, return Invalid state with current timestamp
        SaState::Invalid(Instant::now())
    }
}
