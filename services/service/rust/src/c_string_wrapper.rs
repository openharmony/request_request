/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::{slice, ffi::c_char};
use super::request_binding::*;
#[repr(C)]
pub struct CStringWrapper {
    c_str: *const c_char,
    len: u32,
}

impl CStringWrapper {
    pub fn from(s: &str) -> Self {
        let c_str = s.as_ptr() as *const c_char;
        let len = s.len() as u32;
        CStringWrapper { c_str, len }
    }

    pub fn to_string(&self) -> String {
        if self.c_str.is_null() || self.len == 0 {
            unsafe { DeleteChar(self.c_str) };
            return String::new();
        }
        let bytes = unsafe { slice::from_raw_parts(self.c_str as *const u8, self.len as usize) };
        let str = unsafe { String::from_utf8_unchecked(bytes.to_vec()) };
        unsafe { DeleteChar(self.c_str) };
        str
    }
}
