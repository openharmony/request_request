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

//! Database interface for relational database operations.
//! 
//! This module provides high-level abstractions for working with relational databases,
//! including opening database connections, executing queries, and handling results.

use cxx::SharedPtr;
use std::pin::Pin;

use crate::config::OpenConfig;
use crate::params::{FromSql, Params};
use crate::wrapper::ffi::{self, Execute, NewRowEntity, Query};
use crate::wrapper::open_rdb_store;

/// Success error code constant.
const E_OK: i32 = 0;

/// Database connection and operation interface.
/// 
/// Provides methods for executing SQL statements and queries on a relational database.
/// Wraps the underlying FFI implementation with safe, idiomatic Rust.
pub struct RdbStore<'a> {
    /// Internal representation of the database store
    inner: RdbStoreInner<'a>,
}

impl<'a> RdbStore<'a> {
    /// Opens a database connection using the provided configuration.
    /// 
    /// # Arguments
    /// 
    /// * `config` - Configuration options for opening the database
    /// 
    /// # Returns
    /// 
    /// Returns `Ok` with a new `RdbStore` instance on success, or `Err` with an error code on failure
    pub fn open(config: OpenConfig) -> Result<Self, i32> {
        let rdb = open_rdb_store(config)?;
        if rdb.is_null() {
            return Err(-1);
        }
        Ok(Self {
            inner: RdbStoreInner::Shared(rdb),
        })
    }

    /// Creates a `RdbStore` from an FFI reference.
    /// 
    /// Used internally to wrap FFI pointers with a safe Rust interface.
    /// 
    /// # Arguments
    /// 
    /// * `ffi` - FFI pointer to an existing database store
    pub fn from_ffi(ffi: Pin<&'a mut ffi::RdbStore>) -> Self {
        Self {
            inner: RdbStoreInner::Ref(ffi),
        }
    }

    /// Executes an SQL statement with optional parameters.
    /// 
    /// Use for statements that modify the database like INSERT, UPDATE, DELETE, etc.
    /// 
    /// # Arguments
    /// 
    /// * `sql` - The SQL statement to execute
    /// * `values` - Parameters to bind to the statement
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or `Err` with an error code on failure
    pub fn execute<P: Params>(&self, sql: &str, values: P) -> Result<(), i32> {
        match Execute(self.inner.pin_mut(), sql, values.into_values_object()) {
            0 => Ok(()),
            err => Err(err),
        }
    }

    /// Executes an SQL query and returns results as a typed iterator.
    /// 
    /// The return type `T` must implement the `FromSql` trait to convert from database rows.
    /// 
    /// # Arguments
    /// 
    /// * `sql` - The SQL query statement
    /// * `values` - Parameters to bind to the query
    /// 
    /// # Returns
    /// 
    /// Returns `Ok` with a `QuerySet` iterator on success, or `Err` with an error code on failure
    /// 
    /// # Safety
    /// 
    /// This method uses unsafe code to handle FFI pointers to the underlying result set.
    pub fn query<T>(&self, sql: &str, values: impl Params) -> Result<QuerySet<T>, i32> {
        let result = Query(self.inner.pin_mut(), sql, values.into_values_object());
        if result.is_null() {
            return Err(-1);
        }
        let ptr = result.as_ref().unwrap() as *const ffi::ResultSet as *mut ffi::ResultSet;

        let mut column_count = 0;
        match unsafe { Pin::new_unchecked(ptr.as_mut().unwrap()).GetColumnCount(&mut column_count) }
        {
            0 => {}
            err => return Err(err),
        };
        Ok(QuerySet {
            inner: result,
            column_count,
            phantom: std::marker::PhantomData,
        })
    }
}

/// Internal representation of a database store.
/// 
/// Provides a unified interface for different ownership models of the underlying FFI store.
enum RdbStoreInner<'a> {
    /// Shared ownership model using a reference-counted pointer
    Shared(SharedPtr<ffi::RdbStore>),
    /// Borrowed reference model using a pinned mutable reference
    Ref(Pin<&'a mut ffi::RdbStore>),
}

impl RdbStoreInner<'_> {
    /// Converts the inner store into a pinned mutable reference.
    /// 
    /// # Safety
    /// 
    /// This method uses unsafe code to convert between pointer types and create mutable references.
    fn pin_mut(&self) -> Pin<&mut ffi::RdbStore> {
        match self {
            Self::Shared(ffi) => {
                let ptr = ffi.as_ref().unwrap() as *const ffi::RdbStore as *mut ffi::RdbStore;
                unsafe { Pin::new_unchecked(ptr.as_mut().unwrap()) }
            }
            Self::Ref(ffi) => {
                let ptr = ffi.as_ref().get_ref() as *const ffi::RdbStore as *mut ffi::RdbStore;
                unsafe { Pin::new_unchecked(ptr.as_mut().unwrap()) }
            }
        }
    }
}

/// Iterator over database query results.
/// 
/// Provides methods to access metadata and iterate over rows, converting each row to type `T`.
/// 
/// # Type Parameters
/// 
/// * `T` - The target type that each row will be converted to
pub struct QuerySet<T> {
    /// Internal FFI result set pointer
    inner: SharedPtr<ffi::ResultSet>,
    /// Number of columns in the result set
    column_count: i32,
    /// Type marker for the target row conversion type
    phantom: std::marker::PhantomData<T>,
}

impl<T> QuerySet<T> {
    /// Gets the number of rows in the query result.
    /// 
    /// Returns 0 if an error occurs while retrieving the row count.
    pub fn row_count(&mut self) -> i32 {
        let mut row_count = 0;
        match self.pin_mut().GetRowCount(&mut row_count) {
            0 => row_count,
            _err => 0,
        }
    }

    /// Gets the number of columns in the query result.
    pub fn column_count(&self) -> i32 {
        self.column_count
    }

    /// Converts the inner result set into a pinned mutable reference.
    /// 
    /// # Safety
    /// 
    /// This method uses unsafe code to convert between pointer types and create mutable references.
    fn pin_mut(&mut self) -> Pin<&mut ffi::ResultSet> {
        let ptr = self.inner.as_ref().unwrap() as *const ffi::ResultSet as *mut ffi::ResultSet;
        unsafe { Pin::new_unchecked(ptr.as_mut().unwrap()) }
    }
}

impl<T> Iterator for QuerySet<T>
where
    T: FromSql,
{
    type Item = T;

    /// Advances to the next row and converts it to type `T`.
    /// 
    /// Returns `None` when there are no more rows or when an error occurs.
    fn next(&mut self) -> Option<Self::Item> {
        let mut row = NewRowEntity();
        if self.pin_mut().GoToNextRow() != E_OK {
            return None;
        };
        if self.pin_mut().GetRow(row.pin_mut()) != E_OK {
            return None;
        }
        Some(T::from_sql(0, row.pin_mut()))
    }
}

macro_rules! single_tuple_impl {
    ($(($field:tt $ftype:ident)),* $(,)?) => {
        impl <$($ftype,) *> Iterator for QuerySet<($($ftype,) *)> where $($ftype: FromSql,)* {
            type Item = ($($ftype,) *);
            fn next(&mut self) -> Option<Self::Item> {
                let mut row = NewRowEntity();
                if self.pin_mut().GoToNextRow() != E_OK {
                    return None;
                };
                if (self.pin_mut().GetRow(row.pin_mut()) != E_OK) {
                    return None;
                }
                Some(($({
                    $ftype::from_sql($field,row.pin_mut())
                }), *))

            }
        }
    };
}

single_tuple_impl!((0 A), (1 B));
single_tuple_impl!((0 A), (1 B), (2 C));
single_tuple_impl!((0 A), (1 B), (2 C), (3 D));
single_tuple_impl!((0 A), (1 B), (2 C), (3 D), (4 E));
single_tuple_impl!((0 A), (1 B), (2 C), (3 D), (4 E), (5 F));
single_tuple_impl!((0 A), (1 B), (2 C), (3 D), (4 E), (5 F), (6 G));
single_tuple_impl!((0 A), (1 B), (2 C), (3 D), (4 E), (5 F), (6 G), (7 H));
single_tuple_impl!((0 A), (1 B), (2 C), (3 D), (4 E), (5 F), (6 G), (7 H), (8 I));
single_tuple_impl!((0 A), (1 B), (2 C), (3 D), (4 E), (5 F), (6 G), (7 H), (8 I), (9 J));
single_tuple_impl!((0 A), (1 B), (2 C), (3 D), (4 E), (5 F), (6 G), (7 H), (8 I), (9 J), (10 K));
single_tuple_impl!((0 A), (1 B), (2 C), (3 D), (4 E), (5 F), (6 G), (7 H), (8 I), (9 J), (10 K), (11 L));
single_tuple_impl!((0 A), (1 B), (2 C), (3 D), (4 E), (5 F), (6 G), (7 H), (8 I), (9 J), (10 K), (11 L), (12 M));
single_tuple_impl!((0 A), (1 B), (2 C), (3 D), (4 E), (5 F), (6 G), (7 H), (8 I), (9 J), (10 K), (11 L), (12 M), (13 N));
single_tuple_impl!((0 A), (1 B), (2 C), (3 D), (4 E), (5 F), (6 G), (7 H), (8 I), (9 J), (10 K), (11 L), (12 M), (13 N), (14 O));
single_tuple_impl!((0 A), (1 B), (2 C), (3 D), (4 E), (5 F), (6 G), (7 H), (8 I), (9 J), (10 K), (11 L), (12 M), (13 N), (14 O), (15 P));

// Unit tests for the database module
#[cfg(test)]
mod ut_database {
    include!("../tests/ut/ut_database.rs");
}
