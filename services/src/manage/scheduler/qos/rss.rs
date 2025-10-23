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

//! RSS memory capacity-based QoS resource allocation.
//! 
//! This module defines the `RssCapacity` struct which manages the allocation of network
//! resources based on system memory pressure. Different RSS levels adjust the number of
//! tasks that can run at various priority levels, ensuring that network operations
//! adapt to changing system conditions.

use super::QosLevel;

/// Memory capacity configuration for QoS scheduling based on RSS levels.
///
/// This struct defines the task allocation limits and associated QoS levels for
/// different resource pressure scenarios. It implements a three-tiered task
/// prioritization system with varying capacities based on system memory conditions.
///
/// # Fields
///
/// The tuple contains the following values in order:
/// 1. `m1` - Size of the full-speed zone (highest priority tasks)
/// 2. `m2` - Size of the medium-speed zone (medium priority tasks)
/// 3. `m3` - Size of the fair-adjustment zone (lowest priority tasks)
/// 4. QoS level for m1 tasks
/// 5. QoS level for m2 tasks
/// 6. QoS level for m3 tasks
#[derive(PartialEq, Eq)]
pub(crate) struct RssCapacity(usize, usize, usize, QosLevel, QosLevel, QosLevel);

impl RssCapacity {
    pub(crate) const LEVEL0: Self =
        Self(8, 32, 8, QosLevel::High, QosLevel::Middle, QosLevel::Middle);
    pub(crate) const LEVEL1: Self =
        Self(8, 32, 8, QosLevel::High, QosLevel::Middle, QosLevel::Middle);
    pub(crate) const LEVEL2: Self =
        Self(8, 32, 8, QosLevel::High, QosLevel::Middle, QosLevel::Middle);
    pub(crate) const LEVEL3: Self =
        Self(8, 16, 4, QosLevel::High, QosLevel::Middle, QosLevel::Middle);
    pub(crate) const LEVEL4: Self =
        Self(4, 16, 4, QosLevel::High, QosLevel::Middle, QosLevel::Middle);
    pub(crate) const LEVEL5: Self =
        Self(4, 8, 4, QosLevel::High, QosLevel::Middle, QosLevel::Middle);
    pub(crate) const LEVEL6: Self = Self(4, 8, 2, QosLevel::High, QosLevel::Low, QosLevel::Low);
    
    /// Capacity configuration for level 7 memory pressure (highest).
    ///
    /// Most restrictive configuration: 4 high-priority tasks,
    /// 4 medium-priority tasks, and 2 low-priority tasks with reduced QoS.
    pub(crate) const LEVEL7: Self = Self(4, 4, 2, QosLevel::High, QosLevel::Low, QosLevel::Low);

    /// Creates a new `RssCapacity` instance based on the specified memory pressure level.
    ///
    /// # Arguments
    ///
    /// * `level` - Memory pressure level (0-7), where 0 is minimal pressure and 7 is highest.
    ///
    /// # Returns
    ///
    /// A `RssCapacity` instance configured for the specified pressure level.
    ///
    /// # Panics
    ///
    /// Panics if `level` is outside the valid range of 0-7.
    pub(crate) fn new(level: i32) -> Self {
        match level {
            0 => Self::LEVEL0,
            1 => Self::LEVEL1,
            2 => Self::LEVEL2,
            3 => Self::LEVEL3,
            4 => Self::LEVEL4,
            5 => Self::LEVEL5,
            6 => Self::LEVEL6,
            7 => Self::LEVEL7,
            _ => unreachable!(),
        }
    }

    /// Returns the maximum number of tasks allowed in the full-speed (highest priority) zone.
    pub(crate) fn m1(&self) -> usize {
        self.0
    }

    /// Returns the maximum number of tasks allowed in the medium-speed (medium priority) zone.
    pub(crate) fn m2(&self) -> usize {
        self.1
    }

    /// Returns the maximum number of tasks allowed in the fair-adjustment (lowest priority) zone.
    pub(crate) fn m3(&self) -> usize {
        self.2
    }

    /// Returns the QoS level assigned to tasks in the full-speed zone.
    pub(crate) fn m1_speed(&self) -> QosLevel {
        self.3
    }

    /// Returns the QoS level assigned to tasks in the medium-speed zone.
    pub(crate) fn m2_speed(&self) -> QosLevel {
        self.4
    }

    /// Returns the QoS level assigned to tasks in the fair-adjustment zone.
    pub(crate) fn m3_speed(&self) -> QosLevel {
        self.5
    }
}

#[cfg(test)]
mod ut_rss {
    include!("../../../../tests/ut/manage/scheduler/qos/ut_rss.rs");
}
