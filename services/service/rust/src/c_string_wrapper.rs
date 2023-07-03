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
 
use std::ffi::{c_char, CStr};
use super::request_binding::*;
#[repr(C)]
pub struct CStringWrapper {
    c_str: *const c_char,
    len: u32,
}

impl CStringWrapper {
    pub fn from(s: &String) -> Self {
        let c_str = s.as_ptr() as *const c_char;
        let len = s.as_bytes().len() as u32;
        CStringWrapper { c_str, len }
    }

    pub fn to_string(&self) -> String {
        if self.c_str.is_null() || self.len == 0 {
            return String::new();
        }
        let c_str = unsafe { CStr::from_ptr(self.c_str) };
        let str_slice = c_str.to_str().unwrap();
        let str = str_slice.to_string();
        unsafe { DeleteChar(self.c_str) };
        str
    }
}
