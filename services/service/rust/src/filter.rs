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
pub struct Filter {
    pub bundle: String,
    pub common_data: CommonFilter,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct CommonFilter {
    pub before: i64,
    pub after: i64,
    pub state: u8,
    pub action: u8,
    pub mode: u8
}


#[repr(C)]
pub struct CFilter {
    bundle: CStringWrapper,
    common_data: CommonFilter,
}

impl Filter {
    pub fn to_c_struct(&self) -> CFilter {
        CFilter {
            bundle: CStringWrapper::from(&self.bundle),
            common_data: self.common_data
        }
    }
}

#[repr(C)]
pub struct CVectorWrapper {
    pub ptr: *const u32,
    pub len: u64,
}