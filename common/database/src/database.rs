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

use std::pin::Pin;

use cxx::SharedPtr;

use crate::config::OpenConfig;
use crate::params::{FromSql, Params};
use crate::wrapper::ffi::{self, Execute, NewRowEntity, Query};
use crate::wrapper::open_rdb_store;

const E_OK: i32 = 0;

/// `RdbStore` ffi wrapper.
pub struct RdbStore<'a> {
    inner: RdbStoreInner<'a>,
}

impl<'a> RdbStore<'a> {
    /// Creates a new `RdbStore`.
    pub fn open(config: OpenConfig) -> Result<Self, i32> {
        let rdb = open_rdb_store(config)?;
        if rdb.is_null() {
            return Err(-1);
        }
        Ok(Self {
            inner: RdbStoreInner::Shared(rdb),
        })
    }

    /// Creates a `RdbStore` from a C structure.
    pub fn from_ffi(ffi: Pin<&'a mut ffi::RdbStore>) -> Self {
        Self {
            inner: RdbStoreInner::Ref(ffi),
        }
    }

    /// Executes a sql statement.
    pub fn execute<P: Params>(&self, sql: &str, values: P) -> Result<(), i32> {
        match Execute(self.inner.pin_mut(), sql, values.into_values_object()) {
            0 => Ok(()),
            err => Err(err),
        }
    }

    /// Queries results with a sql statement.
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

enum RdbStoreInner<'a> {
    Shared(SharedPtr<ffi::RdbStore>),
    Ref(Pin<&'a mut ffi::RdbStore>),
}

impl RdbStoreInner<'_> {
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

/// Query results.
pub struct QuerySet<T> {
    inner: SharedPtr<ffi::ResultSet>,
    column_count: i32,
    phantom: std::marker::PhantomData<T>,
}

impl<T> QuerySet<T> {
    /// Gets the count of the rows in the query result.
    pub fn row_count(&mut self) -> i32 {
        let mut row_count = 0;
        match self.pin_mut().GetRowCount(&mut row_count) {
            0 => row_count,
            _err => 0,
        }
    }

    /// Gets the counts of the columns in the query result.
    pub fn column_count(&self) -> i32 {
        self.column_count
    }

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

#[cfg(test)]
mod ut_database {
    include!("../tests/ut/ut_database.rs");
}
