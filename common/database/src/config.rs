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

//! Database configuration utilities.
//! 
//! This module provides configuration options for opening and managing databases,
//! including security settings, callbacks, and versioning.

use cxx::{let_cxx_string, UniquePtr};

use crate::database::RdbStore;
use crate::wrapper::ffi::{NewConfig, RdbStoreConfig, SecurityLevel};

/// Configuration options for opening an RDB database.
/// 
/// Provides a builder-style API for configuring various aspects of database
/// initialization including security settings, bundle information, versioning,
/// and lifecycle callbacks.
pub struct OpenConfig {
    /// Internal C++ configuration object
    pub(crate) inner: UniquePtr<RdbStoreConfig>,
    /// Database version number
    pub(crate) version: i32,
    /// Callback handler for database lifecycle events
    pub(crate) callback: Box<dyn OpenCallback>,
}

impl OpenConfig {
    /// Creates a new database configuration with default settings.
    /// 
    /// Sets up a configuration with version 1 and default callback.
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to the database file
    pub fn new(path: &str) -> Self {
        Self {
            inner: NewConfig(path),
            version: 1,
            callback: Box::new(DefaultCallback),
        }
    }

    /// Sets the security level for the database.
    /// 
    /// # Arguments
    /// 
    /// * `level` - Desired security level
    /// 
    /// # Returns
    /// 
    /// Returns `self` for method chaining
    pub fn security_level(&mut self, level: SecurityLevel) -> &mut Self {
        self.inner.pin_mut().SetSecurityLevel(level);
        self
    }

    /// Enables or disables database encryption.
    /// 
    /// # Arguments
    /// 
    /// * `status` - Whether encryption should be enabled (`true`) or disabled (`false`)
    /// 
    /// # Returns
    /// 
    /// Returns `self` for method chaining
    pub fn encrypt_status(&mut self, status: bool) -> &mut Self {
        self.inner.pin_mut().SetEncryptStatus(status);
        self
    }

    /// Sets the bundle name associated with the database.
    /// 
    /// # Arguments
    /// 
    /// * `name` - Bundle name to associate with the database
    /// 
    /// # Returns
    /// 
    /// Returns `self` for method chaining
    pub fn bundle_name(&mut self, name: &str) -> &mut Self {
        let_cxx_string!(name = name);
        self.inner.pin_mut().SetBundleName(&name);
        self
    }

    /// Sets the callback handler for database lifecycle events.
    /// 
    /// # Arguments
    /// 
    /// * `callback` - Custom callback implementation for database events
    /// 
    /// # Returns
    /// 
    /// Returns `self` for method chaining
    pub fn callback(&mut self, callback: Box<dyn OpenCallback>) -> &mut Self {
        self.callback = callback;
        self
    }

    /// Sets the database version number.
    /// 
    /// # Arguments
    /// 
    /// * `version` - Database schema version number
    /// 
    /// # Returns
    /// 
    /// Returns `self` for method chaining
    pub fn version(&mut self, version: i32) -> &mut Self {
        self.version = version;
        self
    }
}

/// Trait for handling database lifecycle events.
/// 
/// Implement this trait to customize database creation, migration, and corruption handling.
pub trait OpenCallback {
    /// Called when the database is first created.
    /// 
    /// Use this to create tables and initialize schema.
    /// 
    /// # Arguments
    /// 
    /// * `_rdb` - The newly created database instance
    /// 
    /// # Returns
    /// 
    /// Returns 0 on success, or a non-zero error code on failure
    fn on_create(&mut self, _rdb: &mut RdbStore) -> i32 {
        0
    }

    /// Called when the database version needs to be upgraded.
    /// 
    /// # Arguments
    /// 
    /// * `_rdb` - The database instance being upgraded
    /// * `_old_version` - The current database version
    /// * `_new_version` - The target database version
    /// 
    /// # Returns
    /// 
    /// Returns 0 on success, or a non-zero error code on failure
    fn on_upgrade(&mut self, _rdb: &mut RdbStore, _old_version: i32, _new_version: i32) -> i32 {
        0
    }

    /// Called when the database version needs to be downgraded.
    /// 
    /// # Arguments
    /// 
    /// * `_rdb` - The database instance being downgraded
    /// * `_current_version` - The current database version
    /// * `_target_version` - The target database version
    /// 
    /// # Returns
    /// 
    /// Returns 0 on success, or a non-zero error code on failure
    fn on_downgrade(
        &mut self,
        _rdb: &mut RdbStore,
        _current_version: i32,
        _target_version: i32,
    ) -> i32 {
        0
    }

    /// Called when the database is successfully opened.
    /// 
    /// # Arguments
    /// 
    /// * `_rdb` - The opened database instance
    /// 
    /// # Returns
    /// 
    /// Returns 0 on success, or a non-zero error code on failure
    fn on_open(&mut self, _rdb: &mut RdbStore) -> i32 {
        0
    }

    /// Called when the database is corrupted.
    /// 
    /// # Arguments
    /// 
    /// * `_database_file` - Path to the corrupted database file
    /// 
    /// # Returns
    /// 
    /// Returns 0 on success, or a non-zero error code on failure
    fn on_corrupt(&mut self, _database_file: &str) -> i32 {
        0
    }
}

/// Default implementation of `OpenCallback` that performs no operations.
/// 
/// Provides empty implementations for all callback methods that return 0 (success).
struct DefaultCallback;

impl OpenCallback for DefaultCallback {}
