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

pub(crate) struct Handle {
    pub(super) total_ram: u64,
    pub(super) used_ram: u64,
}

impl Handle {
    pub(crate) fn new(ram_cache_size: u64) -> Self {
        Self {
            total_ram: ram_cache_size,
            used_ram: 0,
        }
    }

    pub(crate) fn apply_cache_size(&mut self, apply_size: u64) -> bool {
        if apply_size + self.used_ram > self.total_ram {
            return false;
        }
        self.used_ram += apply_size;
        true
    }

    pub(super) fn release(&mut self, size: u64) {
        self.used_ram -= size;
    }

    pub(crate) fn change_total_size(&mut self, size: u64) {
        self.total_ram = size;
    }
}

#[cfg(test)]
mod test {
    use super::Handle;
    const TEST_TOTAL_SIZE: u64 = 1024;

    // @tc.name: ut_cache_space_handle_operations
    // @tc.desc: Test Handle struct cache space operations
    // @tc.precon: NA
    // @tc.step: 1. Create Handle instance with TEST_TOTAL_SIZE
    //           2. Apply half of total size
    //           3. Release quarter of total size
    //           4. Change total size to double
    // @tc.expect: used_ram updates correctly after each operation, total_ram is
    // doubled
    // @tc.type: FUNC
    // @tc.require: issue#ICN31I
    // @tc.level: level1
    #[test]
    fn ut_cache_space() {
        let mut handle = Handle::new(TEST_TOTAL_SIZE);
        handle.apply_cache_size(TEST_TOTAL_SIZE / 2);
        assert_eq!(handle.used_ram, TEST_TOTAL_SIZE / 2);
        handle.release(TEST_TOTAL_SIZE / 4);
        assert_eq!(handle.used_ram, TEST_TOTAL_SIZE / 4);
        handle.change_total_size(TEST_TOTAL_SIZE * 2);
        assert_eq!(handle.total_ram, TEST_TOTAL_SIZE * 2);
    }
}
