// Copyright (C) 2023 Huawei Device Co., Ltd.
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

use std::ffi::c_char;
use std::slice;

use crate::utils::filter::{CommonFilter, Filter};
use crate::utils::form_item::{FileSpec, FormItem};

#[derive(Clone, Debug)]
#[repr(C)]
pub(crate) struct CStringWrapper {
    c_str: *const c_char,
    len: u32,
}

impl From<&str> for CStringWrapper {
    fn from(value: &str) -> Self {
        let c_str = value.as_ptr() as *const c_char;
        let len = value.len() as u32;
        CStringWrapper { c_str, len }
    }
}

impl From<&String> for CStringWrapper {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ToString for CStringWrapper {
    fn to_string(&self) -> String {
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

#[repr(C)]
pub(crate) struct CFilter {
    bundle: CStringWrapper,
    common_data: CommonFilter,
}

impl Filter {
    pub(crate) fn to_c_struct(&self) -> CFilter {
        CFilter {
            bundle: CStringWrapper::from(&self.bundle),
            common_data: self.common_data,
        }
    }
}

#[repr(C)]
pub(crate) struct CVectorWrapper {
    pub(crate) ptr: *const u32,
    pub(crate) len: u64,
}

#[repr(C)]
pub(crate) struct CFileSpec {
    pub(crate) name: CStringWrapper,
    pub(crate) path: CStringWrapper,
    pub(crate) file_name: CStringWrapper,
    pub(crate) mime_type: CStringWrapper,
    pub(crate) is_user_file: bool,
}

impl FileSpec {
    pub(crate) fn to_c_struct(&self) -> CFileSpec {
        CFileSpec {
            name: CStringWrapper::from(&self.name),
            path: CStringWrapper::from(&self.path),
            file_name: CStringWrapper::from(&self.file_name),
            mime_type: CStringWrapper::from(&self.mime_type),
            is_user_file: self.is_user_file,
        }
    }

    pub(crate) fn from_c_struct(c_struct: &CFileSpec) -> Self {
        FileSpec {
            name: c_struct.name.to_string(),
            path: c_struct.path.to_string(),
            file_name: c_struct.file_name.to_string(),
            mime_type: c_struct.mime_type.to_string(),
            is_user_file: c_struct.is_user_file,
            fd: None,
        }
    }
}

#[repr(C)]
pub(crate) struct CFormItem {
    pub(crate) name: CStringWrapper,
    pub(crate) value: CStringWrapper,
}

impl FormItem {
    pub(crate) fn to_c_struct(&self) -> CFormItem {
        CFormItem {
            name: CStringWrapper::from(&self.name),
            value: CStringWrapper::from(&self.value),
        }
    }

    pub(crate) fn from_c_struct(c_struct: &CFormItem) -> Self {
        FormItem {
            name: c_struct.name.to_string(),
            value: c_struct.value.to_string(),
        }
    }
}

#[link(name = "download_server_cxx", kind = "static")]
extern "C" {
    pub(crate) fn DeleteChar(ptr: *const c_char);
    pub(crate) fn DeleteCFormItem(ptr: *const CFormItem);
    pub(crate) fn DeleteCFileSpec(ptr: *const CFileSpec);
    pub(crate) fn DeleteCStringPtr(ptr: *const CStringWrapper);
}
