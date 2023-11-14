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

use super::{c_string_wrapper::*, enumration::*, form_item::*, utils::*, request_binding::*};
use std::collections::HashMap;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CommonTaskConfig {
    pub task_id: u32,
    pub uid: u64,
    pub action: Action,
    pub mode: Mode,
    pub cover: bool,
    pub network: Network,
    pub metered: bool,
    pub roaming: bool,
    pub retry: bool,
    pub redirect: bool,
    pub index: u32,
    pub begins: u64,
    pub ends: i64,
    pub gauge: bool,
    pub precise: bool,
    pub background: bool,
}

#[derive(Debug)]
pub struct TaskConfig {
    pub bundle: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub data: String,
    pub token: String,
    pub extras: HashMap<String, String>,
    pub version: Version,
    pub form_items: Vec<FormItem>,
    pub file_specs: Vec<FileSpec>,
    pub body_file_names: Vec<String>,
    pub common_data: CommonTaskConfig,
}

#[repr(C)]
pub struct CommonCTaskConfig {
    pub task_id: u32,
    pub uid: u64,
    pub action: u8,
    pub mode: u8,
    pub cover: bool,
    pub network: u8,
    pub metered: bool,
    pub roaming: bool,
    pub retry: bool,
    pub redirect: bool,
    pub index: u32,
    pub begins: u64,
    pub ends: i64,
    pub gauge: bool,
    pub precise: bool,
    pub background: bool,
}

#[repr(C)]
pub struct CTaskConfig {
    pub bundle: CStringWrapper,
    pub url: CStringWrapper,
    pub title: CStringWrapper,
    pub description: CStringWrapper,
    pub method: CStringWrapper,
    pub headers: CStringWrapper,
    pub data: CStringWrapper,
    pub token: CStringWrapper,
    pub extras: CStringWrapper,
    pub version: u8,
    pub form_items_ptr: *const CFormItem,
    pub form_items_len: u32,
    pub file_specs_ptr: *const CFileSpec,
    pub file_specs_len: u32,
    pub body_file_names_ptr: *const CStringWrapper,
    pub body_file_names_len: u32,
    pub common_data: CommonCTaskConfig,
}

impl TaskConfig {
    fn build_vec<A, B, C>(ptr: *const A, len: usize, func: C) -> Vec<B> where C: Fn(&A) -> B,
    {
        if ptr.is_null() || len == 0 {
            return Vec::<B>::new();
        }
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
        slice.iter().map(|x| func(x)).collect()
    }

    pub fn to_c_struct(&self, task_id: u32, uid: u64) -> CTaskConfig {
        let form_items: Vec<CFormItem> = self.form_items.iter().map(|x| x.to_c_struct()).collect();
        let file_specs: Vec<CFileSpec> = self.file_specs.iter().map(|x| x.to_c_struct()).collect();
        let body_file_names: Vec<CStringWrapper> = self.body_file_names.iter()
            .map(|x| CStringWrapper::from(x)).collect();
        CTaskConfig {
            bundle: CStringWrapper::from(&self.bundle),
            url: CStringWrapper::from(&self.url),
            title: CStringWrapper::from(&self.title),
            description: CStringWrapper::from(&self.description),
            method: CStringWrapper::from(&self.method),
            headers: CStringWrapper::from(&hashmap_to_string(&self.headers)),
            data: CStringWrapper::from(&self.data),
            token: CStringWrapper::from(&self.token),
            extras: CStringWrapper::from(&hashmap_to_string(&self.extras)),
            version: self.version as u8,
            form_items_ptr: form_items.as_ptr() as *const CFormItem,
            form_items_len: form_items.len() as u32,
            file_specs_ptr: file_specs.as_ptr() as *const CFileSpec,
            file_specs_len: file_specs.len() as u32,
            body_file_names_ptr: body_file_names.as_ptr() as *const CStringWrapper,
            body_file_names_len: body_file_names.len() as u32,
            common_data: CommonCTaskConfig {
                task_id,
                uid,
                action: self.common_data.action as u8,
                mode: self.common_data.mode as u8,
                cover: self.common_data.cover,
                network: self.common_data.network as u8,
                metered: self.common_data.metered,
                roaming: self.common_data.roaming,
                retry: self.common_data.retry,
                redirect: self.common_data.redirect,
                index: self.common_data.index,
                begins: self.common_data.begins,
                ends: self.common_data.ends,
                gauge: self.common_data.gauge,
                precise: self.common_data.precise,
                background: self.common_data.background,
            },
        }
    }

    pub fn from_c_struct(c_struct: &CTaskConfig) -> Self {
        let task_config = TaskConfig {
            bundle: c_struct.bundle.to_string(),
            url: c_struct.url.to_string(),
            title: c_struct.title.to_string(),
            description: c_struct.description.to_string(),
            method: c_struct.method.to_string(),
            headers: string_to_hashmap(&mut c_struct.headers.to_string()),
            data: c_struct.data.to_string(),
            token: c_struct.token.to_string(),
            extras: string_to_hashmap(&mut c_struct.extras.to_string()),
            version: Version::from(c_struct.version),
            form_items: Self::build_vec(
                c_struct.form_items_ptr,
                c_struct.form_items_len as usize,
                FormItem::from_c_struct,
            ),
            file_specs: Self::build_vec(
                c_struct.file_specs_ptr,
                c_struct.file_specs_len as usize,
                FileSpec::from_c_struct,
            ),
            body_file_names: Self::build_vec(
                c_struct.body_file_names_ptr,
                c_struct.body_file_names_len as usize,
                CStringWrapper::to_string,
            ),
            common_data: CommonTaskConfig {
                task_id: c_struct.common_data.task_id,
                uid: c_struct.common_data.uid,
                action: Action::from(c_struct.common_data.action),
                mode: Mode::from(c_struct.common_data.mode),
                cover: c_struct.common_data.cover,
                network: Network::from(c_struct.common_data.network),
                metered: c_struct.common_data.metered,
                roaming: c_struct.common_data.roaming,
                retry: c_struct.common_data.retry,
                redirect: c_struct.common_data.redirect,
                index: c_struct.common_data.index,
                begins: c_struct.common_data.begins,
                ends: c_struct.common_data.ends,
                gauge: c_struct.common_data.gauge,
                precise: c_struct.common_data.precise,
                background: c_struct.common_data.background,
            },
        };
        unsafe { DeleteCFormItem(c_struct.form_items_ptr) };
        unsafe { DeleteCFileSpec(c_struct.file_specs_ptr) };
        unsafe { DeleteCStringPtr(c_struct.body_file_names_ptr) };
        task_config
    }
}