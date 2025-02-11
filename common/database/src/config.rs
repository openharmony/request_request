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

use cxx::{let_cxx_string, UniquePtr};

pub use crate::wrapper::ffi::SecurityLevel;
use crate::wrapper::ffi::{NewConfig, RdbStoreConfig};
use crate::RdbStore;

pub struct OpenConfig {
    pub(crate) inner: UniquePtr<RdbStoreConfig>,
    pub(crate) version: i32,
    pub(crate) callback: Box<dyn OpenCallback>,
}

struct DefaultCallback;
impl OpenCallback for DefaultCallback {}

pub trait OpenCallback {
    fn on_create(&mut self, rdb: &mut RdbStore) -> i32 {
        0
    }
    fn on_upgrade(&mut self, rdb: &mut RdbStore, old_version: i32, new_version: i32) -> i32 {
        0
    }
    fn on_downgrade(
        &mut self,
        rdb: &mut RdbStore,
        current_version: i32,
        target_version: i32,
    ) -> i32 {
        0
    }
    fn on_open(&mut self, rdb: &mut RdbStore) -> i32 {
        0
    }
    fn on_corrupt(&mut self, database_file: &str) -> i32 {
        0
    }
}

impl OpenConfig {
    pub fn new(path: &str) -> Self {
        Self {
            inner: NewConfig(path),
            version: 1,
            callback: Box::new(DefaultCallback),
        }
    }

    pub fn security_level(&mut self, level: SecurityLevel) -> &mut Self {
        self.inner.pin_mut().SetSecurityLevel(level);
        self
    }

    pub fn encrypt_status(&mut self, status: bool) -> &mut Self {
        self.inner.pin_mut().SetEncryptStatus(status);
        self
    }

    pub fn bundle_name(&mut self, name: &str) -> &mut Self {
        let_cxx_string!(name = name);
        self.inner.pin_mut().SetBundleName(&name);
        self
    }

    pub fn callback(&mut self, callback: Box<dyn OpenCallback>) -> &mut Self {
        self.callback = callback;
        self
    }

    pub fn version(&mut self, version: i32) -> &mut Self {
        self.version = version;
        self
    }
}
