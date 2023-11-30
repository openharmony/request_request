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

use std::collections::HashMap;
use super::{c_string_wrapper::*, enumration::*, form_item::*, progress::*, utils::*, request_binding::*};
#[derive(Debug)]
pub struct TaskInfo {
    pub bundle: String,
    pub url: String,
    pub data: String,
    pub token: String,
    pub form_items: Vec<FormItem>,
    pub file_specs: Vec<FileSpec>,
    pub title: String,
    pub description: String,
    pub mime_type: String,
    pub progress: Progress,
    pub extras: HashMap<String, String>,
    pub each_file_status: Vec<EachFileStatus>,
    pub common_data: CommonTaskInfo,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CommonTaskInfo {
    pub task_id: u32,
    pub uid: u64,
    pub action: u8,
    pub mode: u8,
    pub ctime: u64,
    pub mtime: u64,
    pub reason: u8,
    pub gauge: bool,
    pub retry: bool,
    pub tries: u32,
    pub version: u8,
}

#[derive(Debug)]
pub struct EachFileStatus {
    pub path: String,
    pub reason: Reason,
    pub message: String,
}

#[derive(Debug)]
pub struct NotifyData {
    pub progress: Progress,
    pub action: Action,
    pub version: Version,
    pub each_file_status: Vec<EachFileStatus>,
    pub task_id: u32,
    pub uid: u64,
    pub bundle: String,
}

#[repr(C)]
pub struct CEachFileStatus {
    pub path: CStringWrapper,
    pub reason: u8,
    pub message: CStringWrapper,
}

impl EachFileStatus {
    pub fn to_c_struct(&self) -> CEachFileStatus {
        CEachFileStatus {
            path: CStringWrapper::from(&self.path),
            reason: self.reason as u8,
            message: CStringWrapper::from(&self.message),
        }
    }

    pub fn from_c_struct(c_struct: &CEachFileStatus) -> EachFileStatus {
        EachFileStatus {
            path: c_struct.path.to_string(),
            reason: Reason::from(c_struct.reason),
            message: c_struct.message.to_string(),
        }
    }
}

#[repr(C)]
pub struct CTaskInfo {
    pub bundle: CStringWrapper,
    pub url: CStringWrapper,
    pub data: CStringWrapper,
    pub token: CStringWrapper,
    pub form_items_ptr: *const CFormItem,
    pub form_items_len: u32,
    pub file_specs_ptr: *const CFileSpec,
    pub file_specs_len: u32,
    pub title: CStringWrapper,
    pub description: CStringWrapper,
    pub mime_type: CStringWrapper,
    pub progress: CProgress,
    pub each_file_status_ptr: *const CEachFileStatus,
    pub each_file_status_len: u32,
    pub common_data: CommonTaskInfo,
}

pub struct InfoSet {
    pub form_items: Vec<CFormItem>,
    pub file_specs: Vec<CFileSpec>,
    pub sizes: String,
    pub processed: String,
    pub extras: String,
    pub each_file_status: Vec<CEachFileStatus>,
}

impl TaskInfo {
    pub fn build_info_set(&self) -> InfoSet {
        InfoSet {
            form_items: self.form_items.iter().map(|x| x.to_c_struct()).collect(),
            file_specs: self.file_specs.iter().map(|x| x.to_c_struct()).collect(),
            sizes: format!("{:?}", self.progress.sizes),
            processed: format!("{:?}", self.progress.processed),
            extras: hashmap_to_string(&self.extras),
            each_file_status: self
                .each_file_status
                .iter()
                .map(|x| x.to_c_struct())
                .collect(),
        }
    }

    fn build_vec<A, B, C>(ptr: *const A, len: usize, func: C) -> Vec<B>
    where
        C: Fn(&A) -> B,
    {
        if ptr.is_null() || len == 0 {
            return Vec::<B>::new();
        }
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
        slice.iter().map(|x| func(x)).collect()
    }

    pub fn to_c_struct(&self, info: &InfoSet) -> CTaskInfo {
        CTaskInfo {
            bundle: CStringWrapper::from(&self.bundle),
            url: CStringWrapper::from(&self.url),
            data: CStringWrapper::from(&self.data),
            token: CStringWrapper::from(&self.token),
            form_items_ptr: info.form_items.as_ptr() as *const CFormItem,
            form_items_len: info.form_items.len() as u32,
            file_specs_ptr: info.file_specs.as_ptr() as *const CFileSpec,
            file_specs_len: info.file_specs.len() as u32,
            title: CStringWrapper::from(&self.title),
            description: CStringWrapper::from(&self.description),
            mime_type: CStringWrapper::from(&self.mime_type),
            progress: self.progress.to_c_struct(&info.sizes, &info.processed, &info.extras),
            each_file_status_ptr: info.each_file_status.as_ptr() as *const CEachFileStatus,
            each_file_status_len: info.each_file_status.len() as u32,
            common_data: self.common_data,
        }
    }

    pub fn from_c_struct(c_struct: &CTaskInfo) -> Self {
        let progress = Progress::from_c_struct(&c_struct.progress);
        let extras = progress.extras.clone();
        let task_info = TaskInfo {
            bundle: c_struct.bundle.to_string(),
            url: c_struct.url.to_string(),
            data: c_struct.data.to_string(),
            token: c_struct.token.to_string(),
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
            title: c_struct.title.to_string(),
            description: c_struct.description.to_string(),
            mime_type: c_struct.mime_type.to_string(),
            progress,
            extras,
            each_file_status: Self::build_vec(
                c_struct.each_file_status_ptr, 
                c_struct.each_file_status_len as usize, 
                EachFileStatus::from_c_struct),
            common_data: c_struct.common_data,
        };
        unsafe { DeleteCFormItem(c_struct.form_items_ptr) };
        unsafe { DeleteCFileSpec(c_struct.file_specs_ptr) };
        unsafe { DeleteCEachFileStatus(c_struct.each_file_status_ptr) };
        task_info
    }
}

pub struct UpdateInfo {
    pub mtime: u64,
    pub reason: u8,
    pub tries: u32,
    pub progress: Progress,
    pub each_file_status: Vec<EachFileStatus>,
}

#[repr(C)]
pub struct CUpdateInfo {
    pub mtime: u64,
    pub reason: u8,
    pub tries: u32,
    pub progress: CProgress,
    pub each_file_status_ptr: *const CEachFileStatus,
    pub each_file_status_len: u32,
}

impl UpdateInfo {
    pub fn to_c_struct(
        &self,
        sizes: &String,
        processed: &String,
        extras: &String,
        each_file_status: &Vec<CEachFileStatus>,
    ) -> CUpdateInfo {
        CUpdateInfo {
            mtime: self.mtime,
            reason: self.reason,
            tries: self.tries,
            progress: self.progress.to_c_struct(sizes, processed, extras),
            each_file_status_ptr: each_file_status.as_ptr() as *const CEachFileStatus,
            each_file_status_len: each_file_status.len() as u32,
        }
    }
}
