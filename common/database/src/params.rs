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

use std::pin::{pin, Pin};

use cxx::{CxxVector, UniquePtr};

use crate::wrapper::ffi::{
    self, BindBlob, BindBool, BindDouble, BindI32, BindI64, BindNull, BindString, GetBlob, GetBool,
    GetDouble, GetI32, GetI64, GetString, IsNull, NewVector, ResultSet, RowEntity, ValueObject,
};

trait ToSql {
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>);
}

pub trait FromSql {
    fn from_sql(index: i32, values: Pin<&mut RowEntity>) -> Self;
}

impl ToSql for i32 {
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindI32(*self, values);
    }
}

impl ToSql for i64 {
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindI64(*self, values);
    }
}

impl ToSql for u32 {
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindI64(*self as i64, values);
    }
}

impl ToSql for u64 {
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindI64(*self as i64, values);
    }
}

impl ToSql for f64 {
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindDouble(*self, values);
    }
}

impl ToSql for bool {
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindBool(*self, values);
    }
}

impl ToSql for String {
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindString(self, values);
    }
}

impl ToSql for str {
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindString(self, values);
    }
}

impl ToSql for [u8] {
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        BindBlob(self, values);
    }
}

impl<T: ?Sized + ToSql> ToSql for &T {
    fn to_sql(&self, values: Pin<&mut CxxVector<ValueObject>>) {
        (*self).to_sql(values);
    }
}

impl<T: ToSql> ToSql for Option<T> {
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
    fn from_sql(index: i32, row: Pin<&mut RowEntity>) -> Self {
        let mut value = 0;
        GetI32(row, index, &mut value);
        value
    }
}

impl FromSql for i64 {
    fn from_sql(index: i32, row: Pin<&mut RowEntity>) -> Self {
        let mut value = 0;
        GetI64(row, index, &mut value);
        value
    }
}

impl FromSql for bool {
    fn from_sql(index: i32, row: Pin<&mut RowEntity>) -> Self {
        let mut value = 0;
        GetI32(row, index, &mut value);
        value == 1
    }
}

impl FromSql for f64 {
    fn from_sql(index: i32, row: Pin<&mut RowEntity>) -> Self {
        let mut value = 0.0;
        GetDouble(row, index, &mut value);
        value
    }
}

impl FromSql for String {
    fn from_sql(index: i32, row: Pin<&mut RowEntity>) -> Self {
        let mut value = String::new();
        GetString(row, index, &mut value);
        value
    }
}

impl FromSql for Vec<u8> {
    fn from_sql(index: i32, row: Pin<&mut RowEntity>) -> Self {
        let mut value = Vec::new();
        GetBlob(row, index, &mut value);
        value
    }
}

impl<T: FromSql> FromSql for Option<T> {
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

struct ParamValues {
    inner: UniquePtr<CxxVector<ValueObject>>,
}

impl ParamValues {
    fn new() -> Self {
        Self { inner: NewVector() }
    }

    fn push<T: ToSql>(&mut self, value: T) {
        unsafe { T::to_sql(&value, self.inner.pin_mut()) };
    }
}

pub trait Params {
    fn into_values_object(self) -> UniquePtr<CxxVector<ValueObject>>;
}

impl Params for () {
    fn into_values_object(self) -> UniquePtr<CxxVector<ValueObject>> {
        NewVector()
    }
}

impl<T: ToSql> Params for T {
    fn into_values_object(self) -> UniquePtr<CxxVector<ValueObject>> {
        let mut values = ParamValues::new();
        values.push(self);
        values.inner
    }
}

macro_rules! single_tuple_impl {
    ($(($field:tt $ftype:ident)),* $(,)?) => {
        impl <$($ftype,) *> Params for ($($ftype,) *) where $($ftype: ToSql,)* {
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
