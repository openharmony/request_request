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

pub(crate) struct Cache {
    data: CacheData,
    known_size: bool,
    size: Option<usize>,
}

enum CacheData {
    Ram(Vec<u8>),
    File(File),
}

impl Cache {
    pub(super) fn new_ram(size: Option<usize>) -> Self {
        if let Some(size) = size {
            Self {
                data: CacheData::Ram(Vec::with_capacity(size)),
                known_size: true,
                size: Some(size),
            }
        } else {
            Self {
                data: CacheData::Ram(Vec::new()),
                known_size: false,
                size: None,
            }
        }
    }

    pub(crate) fn reader(&self) -> CacheReader {
        match &self.data {
            CacheData::Ram(v) => CacheReader {
                inner: ReaderData::Cursor(Cursor::new(v)),
            },
            CacheData::File(f) => {
                let mut file = f.try_clone().unwrap();
                file.rewind().unwrap();
                CacheReader {
                    inner: ReaderData::File(file),
                }
            }
        }
    }

    pub(crate) fn update_cache_size(&mut self) -> bool {
        if self.size.is_none() {
            self.size = Some(self.data.size());
            true
        } else {
            false
        }
    }

    pub(super) fn turn_to_file(&mut self) -> bool {
        todo!()
    }

    pub(super) fn is_ram(&self) -> bool {
        match &self.data {
            CacheData::Ram(_) => true,
            CacheData::File(_) => false,
        }
    }

    pub(super) fn is_valid(&self) -> bool {
        if let Some(size) = self.size {
            return size == self.data.size();
        }
        false
    }

    pub(super) fn size(&self) -> Option<usize> {
        self.size
    }

    pub(super) fn known_size(&self) -> bool {
        self.known_size
    }

    pub(super) fn create_file_cache(&self, path: &str) -> Result<Cache, io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        io::copy(&mut self.reader(), &mut file);
        file.rewind()?;
        Ok(Cache {
            data: CacheData::File(file),
            known_size: self.known_size,
            size: self.size,
        })
    }
}

impl CacheData {
    fn size(&self) -> usize {
        match self {
            CacheData::Ram(v) => v.len(),
            CacheData::File(f) => f.metadata().unwrap().len() as usize,
        }
    }
}

impl Write for Cache {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.data {
            CacheData::Ram(ref mut v) => v.write(buf),
            CacheData::File(ref mut f) => f.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self.data {
            CacheData::Ram(ref mut v) => v.flush(),
            CacheData::File(ref mut f) => f.flush(),
        }
    }
}

pub(crate) struct CacheReader<'a> {
    inner: ReaderData<'a>,
}

enum ReaderData<'a> {
    Cursor(Cursor<&'a [u8]>),
    File(File),
}

impl<'a> Read for CacheReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.inner {
            ReaderData::Cursor(c) => c.read(buf),
            ReaderData::File(f) => f.read(buf),
        }
    }
}

impl<'a> Seek for CacheReader<'a> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match &mut self.inner {
            ReaderData::Cursor(c) => c.seek(pos),
            ReaderData::File(f) => f.seek(pos),
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::read;

    use super::*;
    const TEST_URL: &str = "小心猴子";
    const TEST_STRING: &str = "你这猴子真让我欢喜";

    #[test]
    fn ut_cache_write_read() {
        let mut cache = Cache::new_ram(Some(TEST_STRING.len()));
        cache.write_all(TEST_STRING.as_bytes()).unwrap();
        let mut buf = String::new();
        cache.reader().read_to_string(&mut buf);
        assert_eq!(buf, TEST_STRING);

        let mut buf = String::new();
        cache.reader().read_to_string(&mut buf);
        assert_eq!(buf, TEST_STRING);

        let new_cache = cache.create_file_cache(TEST_URL).unwrap();
        assert!(new_cache.is_valid());

        let mut buf = String::new();
        new_cache.reader().read_to_string(&mut buf).unwrap();
        assert_eq!(buf, TEST_STRING);
    }
}
