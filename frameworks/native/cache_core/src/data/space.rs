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

//! Resource capacity management for caching system.
//!
//! This module provides functionality for tracking and managing resource capacities
//! within the cache system, including applying for additional resources and releasing
//! unused resources.

/// Manages resource capacities for the caching system.
///
/// This struct tracks total available capacity and currently used capacity,
/// providing methods to apply for additional resources and release unused ones.
pub(crate) struct ResourceManager {
    /// Total available capacity (in bytes)
    pub(super) total_capacity: u64,
    /// Currently used capacity (in bytes)
    pub(super) used_capacity: u64,
}

impl ResourceManager {
    /// Creates a new resource manager with the specified capacity.
    ///
    /// # Parameters
    /// - `capacity`: Total available capacity in bytes
    ///
    /// # Returns
    /// A new ResourceManager instance with the specified capacity and zero used capacity
    pub(crate) fn new(capacity: u64) -> Self {
        Self {
            total_capacity: capacity,
            used_capacity: 0,
        }
    }

    /// Attempts to allocate additional cache space.
    ///
    /// Checks if the requested size can be allocated without exceeding total capacity.
    /// If successful, updates the used capacity and returns true.
    ///
    /// # Parameters
    /// - `apply_size`: Amount of space to allocate in bytes
    ///
    /// # Returns
    /// `true` if allocation succeeded, `false` if insufficient capacity
    pub(crate) fn apply_cache_size(&mut self, apply_size: u64) -> bool {
        if apply_size + self.used_capacity > self.total_capacity {
            return false;
        }
        self.used_capacity += apply_size;
        true
    }

    /// Releases previously allocated resource space.
    ///
    /// Decreases the used capacity by the specified amount.
    ///
    /// # Parameters
    /// - `size`: Amount of space to release in bytes
    pub(super) fn release(&mut self, size: u64) {
        self.used_capacity -= size;
    }

    /// Updates the total capacity of the resource manager.
    ///
    /// # Parameters
    /// - `size`: New total capacity in bytes
    pub(crate) fn change_total_size(&mut self, size: u64) {
        self.total_capacity = size;
    }
}

#[cfg(test)]
mod ut_space {
    // Include test module containing unit tests for ResourceManager
    include!("../../tests/ut/data/ut_space.rs");
}
