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

use super::QosLevel;

/// Capacity of different `Rss` level, which contains `m1`, `m2`, `m3`.
/// `m1` represents the size of the full-speed zone.
/// `m2` represents the size of the low-speed zone.
/// `m3` represents the size of the fair-adjustment zone.
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
    pub(crate) const LEVEL7: Self = Self(4, 4, 2, QosLevel::High, QosLevel::Low, QosLevel::Low);

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

    pub(crate) fn m1(&self) -> usize {
        self.0
    }

    pub(crate) fn m2(&self) -> usize {
        self.1
    }

    pub(crate) fn m3(&self) -> usize {
        self.2
    }

    pub(crate) fn m1_speed(&self) -> QosLevel {
        self.3
    }

    pub(crate) fn m2_speed(&self) -> QosLevel {
        self.4
    }

    pub(crate) fn m3_speed(&self) -> QosLevel {
        self.5
    }
}

#[cfg(test)]
mod ut_rss {
    include!("../../../../tests/ut/manage/scheduler/qos/ut_rss.rs");
}
