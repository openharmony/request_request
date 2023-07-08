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

use super::c_string_wrapper::*;
#[derive(Clone, Debug)]
pub struct FileSpec {
    pub name: String,
    pub path: String,
    pub file_name: String,
    pub mime_type: String,
}

#[repr(C)]
pub struct CFileSpec {
    pub name: CStringWrapper,
    pub path: CStringWrapper,
    pub file_name: CStringWrapper,
    pub mime_type: CStringWrapper,
}

impl FileSpec {
    pub fn to_c_struct(&self) -> CFileSpec {
        CFileSpec {
            name: CStringWrapper::from(&self.name),
            path: CStringWrapper::from(&self.path),
            file_name: CStringWrapper::from(&self.file_name),
            mime_type: CStringWrapper::from(&self.mime_type),
        }
    }

    pub fn from_c_struct(c_struct: &CFileSpec) -> Self {
        FileSpec {
            name: c_struct.name.to_string(),
            path: c_struct.path.to_string(),
            file_name: c_struct.file_name.to_string(),
            mime_type: c_struct.mime_type.to_string(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct FormItem {
    pub name: String,
    pub value: String,
}

#[repr(C)]
pub struct CFormItem {
    pub name: CStringWrapper,
    pub value: CStringWrapper,
}

impl FormItem {
    pub fn to_c_struct(&self) -> CFormItem {
        CFormItem {
            name: CStringWrapper::from(&self.name),
            value: CStringWrapper::from(&self.value),
        }
    }

    pub fn from_c_struct(c_struct: &CFormItem) -> Self {
        FormItem {
            name: c_struct.name.to_string(),
            value: c_struct.value.to_string(),
        }
    }
}
