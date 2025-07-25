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

pub(crate) struct ResourceManager {
    pub(super) total_capacity: u64,
    pub(super) used_capacity: u64,
}

impl ResourceManager {
    pub(crate) fn new(capacity: u64) -> Self {
        Self {
            total_capacity: capacity,
            used_capacity: 0,
        }
    }

    pub(crate) fn apply_cache_size(&mut self, apply_size: u64) -> bool {
        if apply_size + self.used_capacity > self.total_capacity {
            return false;
        }
        self.used_capacity += apply_size;
        true
    }

    pub(super) fn release(&mut self, size: u64) {
        self.used_capacity -= size;
    }

    pub(crate) fn change_total_size(&mut self, size: u64) {
        self.total_capacity = size;
    }
}

#[cfg(test)]
mod ut_space {
    include!("../../tests/ut/data/ut_space.rs");
}
