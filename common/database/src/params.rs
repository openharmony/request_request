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

//! SQL parameter binding and result extraction utilities.
//! 
//! This module provides traits and implementations for converting between Rust types
//! and SQL database types for parameter binding and result extraction.

use std::pin::Pin;

use cxx::{CxxVector, UniquePtr};

use crate::wrapper::ffi::{
    BindBlob, BindBool, BindDouble, BindI32, BindI64, BindNull, BindString, GetBlob, GetDouble,
    GetI32, GetI64, GetString, IsNull, NewVector, RowEntity, ValueObject,
};

/// Trait for converting Rust types to SQL bind parameters.
/// 
/// Implement this trait for types that can be used as parameters in SQL queries.
trait ToSql {
    /// Converts the value to an SQL bind parameter.
    /// 
    /// # Arguments
    /// 
    /// * `values` - The vector of value objects to append to
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>);
}

/// Trait for converting SQL results to Rust types.
/// 
/// Implement this trait for types that can be constructed from SQL query results.
pub trait FromSql {
    /// Constructs a Rust value from an SQL result row.
    /// 
    /// # Arguments
    /// 
    /// * `index` - Column index in the result row
    /// * `values` - The row entity containing the result data
    fn from_sql(index: i32, values: Pin<&mut RowEntity>) -> Self;
}

impl ToSql for i32 {
    /// Binds an `i32` value to an SQL parameter.
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindI32(*self, values);
    }
}

impl ToSql for i64 {
    /// Binds an `i64` value to an SQL parameter.
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindI64(*self, values);
    }
}

impl ToSql for u32 {
    /// Binds a `u32` value to an SQL parameter as `i64`.
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindI64(*self as i64, values);
    }
}

impl ToSql for u64 {
    /// Binds a `u64` value to an SQL parameter as `i64`.
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindI64(*self as i64, values);
    }
}

impl ToSql for f64 {
    /// Binds an `f64` value to an SQL parameter.
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindDouble(*self, values);
    }
}

impl ToSql for bool {
    /// Binds a `bool` value to an SQL parameter.
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindBool(*self, values);
    }
}

impl ToSql for String {
    /// Binds a `String` value to an SQL parameter.
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindString(self, values);
    }
}

impl ToSql for str {
    /// Binds a string slice to an SQL parameter.
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindString(self, values);
    }
}

impl ToSql for [u8] {
    /// Binds a byte slice as a BLOB to an SQL parameter.
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindBlob(self, values);
    }
}

impl<T: ?Sized + ToSql> ToSql for &T {
    /// Binds a reference to an SQL parameter by delegating to the referenced value.
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        (*self).to_sql(values);
    }
}

impl<T: ToSql> ToSql for Option<T> {
    /// Binds an optional value to an SQL parameter, using NULL for None.
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        match self {
            Some(value) => value.to_sql(values),
            None => {
                BindNull(values);
            }
        }
    }
}

impl FromSql for i32 {
    /// Extracts an `i32` value from an SQL result row.
    fn from_sql(index: i32, row: Pin<&mut RowEntity>) -> Self {
        let mut value = 0;
        GetI32(row, index, &mut value);
        value
    }
}

impl FromSql for i64 {
    /// Extracts an `i64` value from an SQL result row.
    fn from_sql(index: i32, row: Pin<&mut RowEntity>) -> Self {
        let mut value = 0;
        GetI64(row, index, &mut value);
        value
    }
}

impl FromSql for u32 {
    /// Extracts a `u32` value from an SQL result row, converting from `i64`.
    fn from_sql(index: i32, row: Pin<&mut RowEntity>) -> Self {
        let mut value = 0;
        GetI64(row, index, &mut value);
        value as u32
    }
}

impl FromSql for u64 {
    /// Extracts a `u64` value from an SQL result row, converting from `i64`.
    fn from_sql(index: i32, row: Pin<&mut RowEntity>) -> Self {
        let mut value = 0;
        GetI64(row, index, &mut value);
        value as u64
    }
}

impl FromSql for bool {
    /// Extracts a `bool` value from an SQL result row, treating 1 as true.
    fn from_sql(index: i32, row: Pin<&mut RowEntity>) -> Self {
        let mut value = 0;
        GetI32(row, index, &mut value);
        value == 1
    }
}

impl FromSql for f64 {
    /// Extracts an `f64` value from an SQL result row.
    fn from_sql(index: i32, row: Pin<&mut RowEntity>) -> Self {
        let mut value = 0.0;
        GetDouble(row, index, &mut value);
        value
    }
}

impl FromSql for String {
    /// Extracts a `String` value from an SQL result row.
    fn from_sql(index: i32, row: Pin<&mut RowEntity>) -> Self {
        let mut value = String::new();
        GetString(row, index, &mut value);
        value
    }
}

impl FromSql for Vec<u8> {
    /// Extracts a `Vec<u8>` from an SQL BLOB column.
    fn from_sql(index: i32, row: Pin<&mut RowEntity>) -> Self {
        let mut value = Vec::new();
        GetBlob(row, index, &mut value);
        value
    }
}

impl<T: FromSql> FromSql for Option<T> {
    /// Extracts an optional value from an SQL result row.
    /// 
    /// Returns `None` if the column value is NULL, otherwise returns `Some(T)`.
    /// 
    /// # Safety
    /// 
    /// This method uses unsafe code to work with the underlying FFI interface.
    fn from_sql(index: i32, values: Pin<&mut RowEntity>) -> Self {
        unsafe {
            let values = values.get_unchecked_mut();
            if IsNull(Pin::new_unchecked(values), index) {
                None
            } else {
                Some(T::from_sql(index, Pin::new_unchecked(values)))
            }
        }
    }
}

/// Internal helper for collecting SQL parameter values.
/// 
/// Provides methods to create a new parameter collection and add values to it.
struct ParamValues {
    /// Internal vector of SQL value objects
    inner: UniquePtr<CxxVector<ValueObject>>,
}

impl ParamValues {
    /// Creates a new empty parameter collection.
    fn new() -> Self {
        Self { inner: NewVector() }
    }

    /// Adds a value to the parameter collection.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to add, must implement `ToSql`
    fn push<T: ToSql>(&mut self, value: T) {
        T::to_sql(&value, self.inner.pin_mut())
    }
}

/// Trait for types that can be used as SQL query parameters.
/// 
/// Implementations are provided for common types and tuples of up to 16 elements.
pub trait Params {
    /// Converts the value into a vector of SQL value objects for binding.
    fn into_values_object(self) -> UniquePtr<CxxVector<ValueObject>>;
}

impl Params for () {
    /// Creates an empty parameter collection for queries with no parameters.
    fn into_values_object(self) -> UniquePtr<CxxVector<ValueObject>> {
        NewVector()
    }
}

impl<T: ToSql> Params for T {
    /// Converts a single value into a parameter collection.
    fn into_values_object(self) -> UniquePtr<CxxVector<ValueObject>> {
        let mut values = ParamValues::new();
        values.push(self);
        values.inner
    }
}

// Macro for implementing `Params` for tuple types
// This allows using tuples of up to 16 elements as query parameters
macro_rules! single_tuple_impl {
    ($(($field:tt $ftype:ident)),* $(,)?) => {
        impl <$($ftype,) *> Params for ($($ftype,) *) where $($ftype: ToSql,)* {
            /// Converts a tuple of values into a parameter collection.
            fn into_values_object(self) -> UniquePtr<CxxVector<ValueObject>> {
                let mut values = ParamValues::new();
                $({
                    values.push(self.$field);
                })+
                values.inner
            }
        }
    };
}

single_tuple_impl!((0 A));
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
