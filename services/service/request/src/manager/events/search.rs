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

use crate::manager::TaskManager;
use crate::utils::c_wrapper::{CFilter, CVectorWrapper, DeleteCVectorWrapper};
use crate::utils::filter::Filter;

impl TaskManager {
    pub(crate) fn search(&self, filter: Filter) -> Vec<u32> {
        debug!("TaskManager search a task, filter:{:?}", filter);

        let mut vec = Vec::<u32>::new();
        let c_vector_wrapper = unsafe { Search(filter.to_c_struct()) };
        if c_vector_wrapper.ptr.is_null() || c_vector_wrapper.len == 0 {
            error!("c_vector_wrapper is null");
            return vec;
        }
        let slice = unsafe {
            std::slice::from_raw_parts(c_vector_wrapper.ptr, c_vector_wrapper.len as usize)
        };
        for item in slice.iter() {
            vec.push(*item);
        }
        debug!("c_vector_wrapper is not null");
        unsafe { DeleteCVectorWrapper(c_vector_wrapper.ptr) };
        vec
    }
}

extern "C" {

    pub(crate) fn Search(filter: CFilter) -> CVectorWrapper;
}
