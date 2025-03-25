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

use crate::database::RdbStore;
use crate::wrapper::ffi::{NewConfig, RdbStoreConfig, SecurityLevel};

/// Open options of `RDB`.
pub struct OpenConfig {
    pub(crate) inner: UniquePtr<RdbStoreConfig>,
    pub(crate) version: i32,
    pub(crate) callback: Box<dyn OpenCallback>,
}

impl OpenConfig {
    /// Creates a new `OpenConfig`.
    pub fn new(path: &str) -> Self {
        Self {
            inner: NewConfig(path),
            version: 1,
            callback: Box::new(DefaultCallback),
        }
    }

    /// Sets the security level of the database.
    pub fn security_level(&mut self, level: SecurityLevel) -> &mut Self {
        self.inner.pin_mut().SetSecurityLevel(level);
        self
    }

    /// Sets the encrypt status of the database.
    pub fn encrypt_status(&mut self, status: bool) -> &mut Self {
        self.inner.pin_mut().SetEncryptStatus(status);
        self
    }

    /// Sets the bundle name of the database.
    pub fn bundle_name(&mut self, name: &str) -> &mut Self {
        let_cxx_string!(name = name);
        self.inner.pin_mut().SetBundleName(&name);
        self
    }

    /// Sets the open callback of the database.
    pub fn callback(&mut self, callback: Box<dyn OpenCallback>) -> &mut Self {
        self.callback = callback;
        self
    }

    /// Sets the version of the database.
    pub fn version(&mut self, version: i32) -> &mut Self {
        self.version = version;
        self
    }
}

/// Trait for database callbacks.
pub trait OpenCallback {
    /// Callback for creating the database.
    fn on_create(&mut self, _rdb: &mut RdbStore) -> i32 {
        0
    }

    /// Callback for upgrading the database.
    fn on_upgrade(&mut self, _rdb: &mut RdbStore, _old_version: i32, _new_version: i32) -> i32 {
        0
    }

    /// Callback for downgrading the database.
    fn on_downgrade(
        &mut self,
        _rdb: &mut RdbStore,
        _current_version: i32,
        _target_version: i32,
    ) -> i32 {
        0
    }

    /// Callback for opening the database.
    fn on_open(&mut self, _rdb: &mut RdbStore) -> i32 {
        0
    }

    /// Callback when the database is corrupted.
    fn on_corrupt(&mut self, _database_file: &str) -> i32 {
        0
    }
}

struct DefaultCallback;

impl OpenCallback for DefaultCallback {}
