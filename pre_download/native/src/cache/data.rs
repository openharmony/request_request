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

use std::fs::{File, OpenOptions};
use std::io::{self, Cursor, Read, Seek, Write};
use std::sync::Arc;

use super::CacheManager;

pub struct Cache {
    task_id: u64,
    data: Vec<u8>,
    ram_applied: bool,
}

impl Cache {
    pub(super) fn new(task_id: u64, applied_size: Option<usize>) -> Self {
        let (data, ram_applied) = match applied_size {
            Some(size) => (Vec::with_capacity(size), true),
            None => (Vec::new(), false),
        };
        Self {
            task_id,
            data,
            ram_applied,
        }
    }

    pub(crate) fn cursor(&self) -> Cursor<&[u8]> {
        Cursor::new(&self.data)
    }

    pub(crate) fn complete_write(self) -> Arc<Self> {
        if !self.ram_applied {
            CacheManager::get_instance().apply_ram_size(self.data.len());
        }
        let me = Arc::new(self);
        CacheManager::get_instance().update_cache(me.task_id, me.clone());
        me
    }

    pub(super) fn create_file_cache(&self, task_id: u64) -> Result<File, io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(true)
            .open(task_id.to_string())?;
        io::copy(&mut self.cursor(), &mut file)?;
        file.rewind()?;
        Ok(file)
    }

    pub(crate) fn size(&self) -> usize {
        self.data.len()
    }
}

impl Write for Cache {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.data.flush()
    }
}

#[cfg(test)]
mod test {

    use request_utils::fastrand::fast_random;

    use super::*;
    const TEST_STRING: &str = "你这猴子真让我欢喜";

    #[test]
    fn ut_cache_write_read() {
        let task_id = fast_random();

        let mut cache = Cache::new(task_id, Some(TEST_STRING.len()));
        cache.write_all(TEST_STRING.as_bytes()).unwrap();
        let mut buf = String::new();
        cache.cursor().read_to_string(&mut buf).unwrap();
        assert_eq!(buf, TEST_STRING);

        let mut buf = String::new();
        cache.cursor().read_to_string(&mut buf).unwrap();
        assert_eq!(buf, TEST_STRING);
    }
}
