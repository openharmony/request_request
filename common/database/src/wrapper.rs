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

//! FFI wrapper for database operations.
//! 
//! This module provides Rust bindings and wrappers around the underlying C++ database API,
//! including callback handling and database store operations.

use std::pin::Pin;

use cxx::SharedPtr;
use ffi::{GetRdbStore, RdbStore};

use crate::config::{OpenCallback, OpenConfig};
use crate::database;

/// Wrapper for database open callbacks.
/// 
/// Provides a bridge between Rust callback implementations and the C++ database API.
pub struct OpenCallbackWrapper {
    /// The underlying Rust callback implementation
    inner: Box<dyn OpenCallback>,
}

impl OpenCallbackWrapper {
    /// Handles database creation callback.
    /// 
    /// # Arguments
    /// 
    /// * `rdb` - The raw RdbStore pointer from the C++ API
    /// 
    /// # Returns
    /// 
    /// Returns 0 on success, otherwise an error code
    fn on_create(&mut self, rdb: Pin<&mut RdbStore>) -> i32 {
        let mut rdb = database::RdbStore::from_ffi(rdb);
        self.inner.on_create(&mut rdb)
    }

    /// Handles database upgrade callback.
    /// 
    /// # Arguments
    /// 
    /// * `rdb` - The raw RdbStore pointer from the C++ API
    /// * `old_version` - The old database version
    /// * `new_version` - The new database version
    /// 
    /// # Returns
    /// 
    /// Returns 0 on success, otherwise an error code
    fn on_upgrade(&mut self, rdb: Pin<&mut RdbStore>, old_version: i32, new_version: i32) -> i32 {
        let mut rdb = database::RdbStore::from_ffi(rdb);
        self.inner.on_upgrade(&mut rdb, old_version, new_version)
    }

    /// Handles database downgrade callback.
    /// 
    /// # Arguments
    /// 
    /// * `rdb` - The raw RdbStore pointer from the C++ API
    /// * `current_version` - The current database version
    /// * `target_version` - The target database version
    /// 
    /// # Returns
    /// 
    /// Returns 0 on success, otherwise an error code
    fn on_downgrade(
        &mut self,
        rdb: Pin<&mut RdbStore>,
        current_version: i32,
        target_version: i32,
    ) -> i32 {
        let mut rdb = database::RdbStore::from_ffi(rdb);
        self.inner
            .on_downgrade(&mut rdb, current_version, target_version)
    }

    /// Handles database open callback.
    /// 
    /// # Arguments
    /// 
    /// * `rdb` - The raw RdbStore pointer from the C++ API
    /// 
    /// # Returns
    /// 
    /// Returns 0 on success, otherwise an error code
    fn on_open(&mut self, rdb: Pin<&mut RdbStore>) -> i32 {
        let mut rdb = database::RdbStore::from_ffi(rdb);
        self.inner.on_open(&mut rdb)
    }

    /// Handles database corruption callback.
    /// 
    /// # Arguments
    /// 
    /// * `database_file` - Path to the corrupt database file
    /// 
    /// # Returns
    /// 
    /// Returns 0 on success, otherwise an error code
    fn on_corrupt(&mut self, database_file: &str) -> i32 {
        self.inner.on_corrupt(database_file)
    }
}

/// Opens an RDB store with the provided configuration.
/// 
/// # Arguments
/// 
/// * `config` - The database opening configuration
/// 
/// # Returns
/// 
/// Returns `Ok` with the shared RDB store pointer on success, or `Err` with an error code on failure
pub(crate) fn open_rdb_store(config: OpenConfig) -> Result<SharedPtr<RdbStore>, i32> {
    let mut code = 0;
    let rdb = GetRdbStore(
        &config.inner,
        config.version,
        Box::new(OpenCallbackWrapper {
            inner: config.callback,
        }),
        &mut code,
    );
    match code {
        0 => Ok(rdb),
        err => Err(err),
    }
}

/// Marks `RdbStore` as safely transferable across thread boundaries.
/// 
/// # Safety
/// 
/// This implementation assumes that the underlying C++ implementation of `RdbStore`
/// is thread-safe when accessed through the FFI boundary.
unsafe impl Send for RdbStore {}

/// Marks `RdbStore` as safely shareable across thread boundaries.
/// 
/// # Safety
/// 
/// This implementation assumes that the underlying C++ implementation of `RdbStore`
/// supports concurrent read operations from multiple threads.
unsafe impl Sync for RdbStore {}

#[allow(unused, missing_docs)]
#[cxx::bridge(namespace = "OHOS::Request")]
pub mod ffi {
    // Database security levels
    #[repr(i32)]
    enum SecurityLevel {
        S1 = 1,
        S2,
        S3,
        S4,
        LAST,
    }

    // SQL column data types
    #[repr(i32)]
    enum ColumnType {
        TYPE_NULL = 0,
        TYPE_INTEGER,
        TYPE_FLOAT,
        TYPE_STRING,
        TYPE_BLOB,
        TYPE_ASSET,
        TYPE_ASSETS,
        TYPE_FLOAT32_ARRAY,
        TYPE_BIGINT,
    }

    // Rust interface exposed to C++
    extern "Rust" {
        type OpenCallbackWrapper;
        fn on_create(&mut self, rdb: Pin<&mut RdbStore>) -> i32;
        fn on_upgrade(
            &mut self,
            rdb: Pin<&mut RdbStore>,
            old_version: i32,
            new_version: i32,
        ) -> i32;
        fn on_downgrade(
            &mut self,
            rdb: Pin<&mut RdbStore>,
            current_version: i32,
            target_version: i32,
        ) -> i32;
        fn on_open(&mut self, rdb: Pin<&mut RdbStore>) -> i32;
        fn on_corrupt(&mut self, database_file: &str) -> i32;
    }

    // C++ interface imported to Rust
    unsafe extern "C++" {
        // Include necessary C++ headers
        include!("rdb_store.h");
        include!("result_set.h");
        include!("remote_result_set.h");
        include!("wrapper.h");
        
        // C++ types from the NativeRdb namespace
        #[namespace = "OHOS::NativeRdb"]
        type RdbStoreConfig;
        #[namespace = "OHOS::NativeRdb"]
        type RdbStore;
        #[namespace = "OHOS::NativeRdb"]
        type ValueObject;
        #[namespace = "OHOS::NativeRdb"]
        type SecurityLevel;
        #[namespace = "OHOS::NativeRdb"]
        type StorageMode;
        #[namespace = "OHOS::NativeRdb"]
        type ResultSet;
        #[namespace = "OHOS::NativeRdb"]
        type RowEntity;
        #[namespace = "OHOS::NativeRdb"]
        type ColumnType;

        // ResultSet operations
        #[namespace = "OHOS::NativeRdb"]
        fn GetColumnType(
            self: Pin<&mut ResultSet>,
            column_index: i32,
            column_type: Pin<&mut ColumnType>,
        ) -> i32;
        
        #[namespace = "OHOS::NativeRdb"]
        fn GetColumnCount(self: Pin<&mut ResultSet>, count: &mut i32) -> i32;
        
        #[namespace = "OHOS::NativeRdb"]
        fn GetRowCount(self: Pin<&mut ResultSet>, count: &mut i32) -> i32;
        
        #[namespace = "OHOS::NativeRdb"]
        fn GoToNextRow(self: Pin<&mut ResultSet>) -> i32;
        
        #[namespace = "OHOS::NativeRdb"]
        fn GetRow(self: Pin<&mut ResultSet>, row: Pin<&mut RowEntity>) -> i32;

        // Factory functions
        fn NewVector() -> UniquePtr<CxxVector<ValueObject>>;
        fn NewConfig(path: &str) -> UniquePtr<RdbStoreConfig>;
        fn NewRowEntity() -> UniquePtr<RowEntity>;

        // SQL parameter binding functions
        fn BindI32(value: i32, values: Pin<&mut CxxVector<ValueObject>>);
        fn BindI64(value: i64, values: Pin<&mut CxxVector<ValueObject>>);
        fn BindBool(value: bool, values: Pin<&mut CxxVector<ValueObject>>);
        fn BindDouble(value: f64, values: Pin<&mut CxxVector<ValueObject>>);
        fn BindString(value: &str, values: Pin<&mut CxxVector<ValueObject>>);
        fn BindBlob(value: &[u8], values: Pin<&mut CxxVector<ValueObject>>);
        fn BindNull(values: Pin<&mut CxxVector<ValueObject>>);

        // SQL result extraction functions
        fn GetI32(row: Pin<&mut RowEntity>, index: i32, value: &mut i32) -> i32;
        fn GetI64(row: Pin<&mut RowEntity>, index: i32, value: &mut i64) -> i32;
        fn GetBool(row: Pin<&mut RowEntity>, index: i32, value: &mut bool) -> i32;
        fn GetDouble(row: Pin<&mut RowEntity>, index: i32, value: &mut f64) -> i32;
        fn GetString(row: Pin<&mut RowEntity>, index: i32, value: &mut String) -> i32;
        fn GetBlob(row: Pin<&mut RowEntity>, index: i32, value: &mut Vec<u8>) -> i32;
        fn IsNull(row: Pin<&mut RowEntity>, index: i32) -> bool;
        // SQL execution functions
        fn Execute(
            rdb: Pin<&mut RdbStore>,
            sql: &str,
            values: UniquePtr<CxxVector<ValueObject>>,
        ) -> i32;

        fn Query(
            rdb: Pin<&mut RdbStore>,
            sql: &str,
            values: UniquePtr<CxxVector<ValueObject>>,
        ) -> SharedPtr<ResultSet>;

        // RDB store configuration and creation
        fn SetSecurityLevel(self: Pin<&mut RdbStoreConfig>, level: SecurityLevel);
        fn SetEncryptStatus(self: Pin<&mut RdbStoreConfig>, status: bool);
        fn SetBundleName(self: Pin<&mut RdbStoreConfig>, bundleName: &CxxString) -> i32;
        fn GetRdbStore(
            config: &RdbStoreConfig,
            version: i32,
            openCallback: Box<OpenCallbackWrapper>,
            errCode: &mut i32,
        ) -> SharedPtr<RdbStore>;
    }
}
