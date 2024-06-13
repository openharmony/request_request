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

use super::config::{
    Action, CommonTaskConfig, ConfigSet, Network, NetworkInner, TaskConfig, Version,
};
use super::info::{CommonTaskInfo, InfoSet, Mode, TaskInfo, UpdateInfo};
use super::notify::{CommonProgress, EachFileStatus, Progress};
use super::reason::Reason;
use crate::task::info::State;
use crate::utils::c_wrapper::{
    CFileSpec, CFormItem, CStringWrapper, DeleteCFileSpec, DeleteCFormItem, DeleteCStringPtr,
};
use crate::utils::form_item::{FileSpec, FormItem};
use crate::utils::{build_vec, split_string, string_to_hashmap};

#[repr(C)]
pub(crate) struct CTaskConfig {
    pub(crate) bundle: CStringWrapper,
    pub(crate) url: CStringWrapper,
    pub(crate) title: CStringWrapper,
    pub(crate) description: CStringWrapper,
    pub(crate) method: CStringWrapper,
    pub(crate) headers: CStringWrapper,
    pub(crate) data: CStringWrapper,
    pub(crate) token: CStringWrapper,
    pub(crate) proxy: CStringWrapper,
    pub(crate) certificate_pins: CStringWrapper,
    pub(crate) extras: CStringWrapper,
    pub(crate) version: u8,
    pub(crate) form_items_ptr: *const CFormItem,
    pub(crate) form_items_len: u32,
    pub(crate) file_specs_ptr: *const CFileSpec,
    pub(crate) file_specs_len: u32,
    pub(crate) body_file_names_ptr: *const CStringWrapper,
    pub(crate) body_file_names_len: u32,
    pub(crate) certs_path_ptr: *const CStringWrapper,
    pub(crate) certs_path_len: u32,
    pub(crate) common_data: CommonCTaskConfig,
}

#[repr(C)]
pub(crate) struct CommonCTaskConfig {
    pub(crate) task_id: u32,
    pub(crate) uid: u64,
    pub(crate) token_id: u64,
    pub(crate) action: u8,
    pub(crate) mode: u8,
    pub(crate) cover: bool,
    pub(crate) network: u8,
    pub(crate) metered: bool,
    pub(crate) roaming: bool,
    pub(crate) retry: bool,
    pub(crate) redirect: bool,
    pub(crate) index: u32,
    pub(crate) begins: u64,
    pub(crate) ends: i64,
    pub(crate) gauge: bool,
    pub(crate) precise: bool,
    pub(crate) priority: u32,
    pub(crate) background: bool,
}

#[repr(C)]
pub(crate) struct CEachFileStatus {
    pub(crate) path: CStringWrapper,
    pub(crate) reason: u8,
    pub(crate) message: CStringWrapper,
}

impl EachFileStatus {
    pub(crate) fn to_c_struct(&self) -> CEachFileStatus {
        CEachFileStatus {
            path: CStringWrapper::from(&self.path),
            reason: self.reason as u8,
            message: CStringWrapper::from(&self.message),
        }
    }

    pub(crate) fn from_c_struct(c_struct: &CEachFileStatus) -> EachFileStatus {
        EachFileStatus {
            path: c_struct.path.to_string(),
            reason: Reason::from(c_struct.reason),
            message: c_struct.message.to_string(),
        }
    }
}

#[repr(C)]
pub(crate) struct CProgress {
    pub(crate) common_data: CommonProgress,
    pub(crate) sizes: CStringWrapper,
    pub(crate) processed: CStringWrapper,
    pub(crate) extras: CStringWrapper,
}

impl Progress {
    pub(crate) fn to_c_struct(&self, sizes: &str, processed: &str, extras: &str) -> CProgress {
        CProgress {
            common_data: self.common_data.clone(),
            sizes: CStringWrapper::from(sizes),
            processed: CStringWrapper::from(processed),
            extras: CStringWrapper::from(extras),
        }
    }

    pub(crate) fn from_c_struct(c_struct: &CProgress) -> Self {
        Progress {
            common_data: c_struct.common_data.clone(),
            sizes: split_string(&mut c_struct.sizes.to_string())
                .map(|s| s.parse::<i64>().unwrap())
                .collect(),
            processed: split_string(&mut c_struct.processed.to_string())
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
            extras: string_to_hashmap(&mut c_struct.extras.to_string()),
        }
    }
}

#[repr(C)]
pub(crate) struct CTaskInfo {
    pub(crate) bundle: CStringWrapper,
    pub(crate) url: CStringWrapper,
    pub(crate) data: CStringWrapper,
    pub(crate) token: CStringWrapper,
    pub(crate) form_items_ptr: *const CFormItem,
    pub(crate) form_items_len: u32,
    pub(crate) file_specs_ptr: *const CFileSpec,
    pub(crate) file_specs_len: u32,
    pub(crate) title: CStringWrapper,
    pub(crate) description: CStringWrapper,
    pub(crate) mime_type: CStringWrapper,
    pub(crate) progress: CProgress,
    pub(crate) each_file_status_ptr: *const CEachFileStatus,
    pub(crate) each_file_status_len: u32,
    pub(crate) common_data: CommonTaskInfo,
}

impl TaskInfo {
    pub(crate) fn to_c_struct(&self, info: &InfoSet) -> CTaskInfo {
        CTaskInfo {
            bundle: CStringWrapper::from(&self.bundle),
            url: CStringWrapper::from(&self.url),
            data: CStringWrapper::from(&self.data),
            token: CStringWrapper::from(&self.token),
            form_items_ptr: info.form_items.as_ptr(),
            form_items_len: info.form_items.len() as u32,
            file_specs_ptr: info.file_specs.as_ptr(),
            file_specs_len: info.file_specs.len() as u32,
            title: CStringWrapper::from(&self.title),
            description: CStringWrapper::from(&self.description),
            mime_type: CStringWrapper::from(&self.mime_type),
            progress: self
                .progress
                .to_c_struct(&info.sizes, &info.processed, &info.extras),
            each_file_status_ptr: info.each_file_status.as_ptr(),
            each_file_status_len: info.each_file_status.len() as u32,
            common_data: self.common_data,
        }
    }

    pub(crate) fn from_c_struct(c_struct: &CTaskInfo) -> Self {
        let progress = Progress::from_c_struct(&c_struct.progress);
        let extras = progress.extras.clone();

        // Removes this logic if api9 and api10 matched.
        let mime_type = if c_struct.common_data.version == Version::API9 as u8
            || (c_struct.progress.common_data.state != State::Completed as u8
                && c_struct.progress.common_data.state != State::Failed as u8)
        {
            c_struct.mime_type.to_string()
        } else {
            String::new()
        };

        let task_info = TaskInfo {
            bundle: c_struct.bundle.to_string(),
            url: c_struct.url.to_string(),
            data: c_struct.data.to_string(),
            token: c_struct.token.to_string(),
            form_items: build_vec(
                c_struct.form_items_ptr,
                c_struct.form_items_len as usize,
                FormItem::from_c_struct,
            ),
            file_specs: build_vec(
                c_struct.file_specs_ptr,
                c_struct.file_specs_len as usize,
                FileSpec::from_c_struct,
            ),
            title: c_struct.title.to_string(),
            description: c_struct.description.to_string(),
            mime_type,
            progress,
            extras,
            each_file_status: build_vec(
                c_struct.each_file_status_ptr,
                c_struct.each_file_status_len as usize,
                EachFileStatus::from_c_struct,
            ),
            common_data: c_struct.common_data,
        };

        unsafe { DeleteCFormItem(c_struct.form_items_ptr) };
        unsafe { DeleteCFileSpec(c_struct.file_specs_ptr) };
        unsafe { DeleteCEachFileStatus(c_struct.each_file_status_ptr) };
        task_info
    }
}

#[repr(C)]
pub(crate) struct CUpdateInfo {
    pub(crate) mtime: u64,
    pub(crate) reason: u8,
    pub(crate) tries: u32,
    pub(crate) mime_type: CStringWrapper,
    pub(crate) progress: CProgress,
    pub(crate) each_file_status_ptr: *const CEachFileStatus,
    pub(crate) each_file_status_len: u32,
}

impl UpdateInfo {
    pub(crate) fn to_c_struct(
        &self,
        sizes: &str,
        processed: &str,
        extras: &str,
        each_file_status: &Vec<CEachFileStatus>,
    ) -> CUpdateInfo {
        CUpdateInfo {
            mtime: self.mtime,
            reason: self.reason,
            tries: self.tries,
            mime_type: CStringWrapper::from(self.mime_type.as_str()),
            progress: self.progress.to_c_struct(sizes, processed, extras),
            each_file_status_ptr: each_file_status.as_ptr(),
            each_file_status_len: each_file_status.len() as u32,
        }
    }
}

#[repr(C)]
pub(crate) struct RequestTaskMsg {
    pub(crate) task_id: u32,
    pub(crate) uid: i32,
    pub(crate) action: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub(crate) struct NetworkInfo {
    pub(crate) network_type: NetworkInner,
    pub(crate) is_metered: bool,
    pub(crate) is_roaming: bool,
}

impl TaskConfig {
    pub(crate) fn to_c_struct(&self, task_id: u32, uid: u64, set: &ConfigSet) -> CTaskConfig {
        CTaskConfig {
            bundle: CStringWrapper::from(&self.bundle),
            url: CStringWrapper::from(&self.url),
            title: CStringWrapper::from(&self.title),
            description: CStringWrapper::from(&self.description),
            method: CStringWrapper::from(&self.method),
            headers: CStringWrapper::from(&set.headers),
            data: CStringWrapper::from(&self.data),
            token: CStringWrapper::from(&self.token),
            extras: CStringWrapper::from(&set.extras),
            proxy: CStringWrapper::from(&self.proxy),
            certificate_pins: CStringWrapper::from(&self.certificate_pins),
            version: self.version as u8,
            form_items_ptr: set.form_items.as_ptr(),
            form_items_len: set.form_items.len() as u32,
            file_specs_ptr: set.file_specs.as_ptr(),
            file_specs_len: set.file_specs.len() as u32,
            body_file_names_ptr: set.body_file_names.as_ptr(),
            body_file_names_len: set.body_file_names.len() as u32,
            certs_path_ptr: set.certs_path.as_ptr(),
            certs_path_len: set.certs_path.len() as u32,
            common_data: CommonCTaskConfig {
                task_id,
                uid,
                token_id: self.common_data.token_id,
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
                priority: self.common_data.priority,
                background: self.common_data.background,
            },
        }
    }

    pub(crate) fn from_c_struct(c_struct: &CTaskConfig) -> Self {
        let task_config: TaskConfig = TaskConfig {
            bundle: c_struct.bundle.to_string(),
            url: c_struct.url.to_string(),
            title: c_struct.title.to_string(),
            description: c_struct.description.to_string(),
            method: c_struct.method.to_string(),
            headers: string_to_hashmap(&mut c_struct.headers.to_string()),
            data: c_struct.data.to_string(),
            token: c_struct.token.to_string(),
            extras: string_to_hashmap(&mut c_struct.extras.to_string()),
            proxy: c_struct.proxy.to_string(),
            certificate_pins: c_struct.certificate_pins.to_string(),
            version: Version::from(c_struct.version),
            form_items: build_vec(
                c_struct.form_items_ptr,
                c_struct.form_items_len as usize,
                FormItem::from_c_struct,
            ),
            file_specs: build_vec(
                c_struct.file_specs_ptr,
                c_struct.file_specs_len as usize,
                FileSpec::from_c_struct,
            ),
            body_file_paths: build_vec(
                c_struct.body_file_names_ptr,
                c_struct.body_file_names_len as usize,
                CStringWrapper::to_string,
            ),
            certs_path: build_vec(
                c_struct.certs_path_ptr,
                c_struct.certs_path_len as usize,
                CStringWrapper::to_string,
            ),
            common_data: CommonTaskConfig {
                task_id: c_struct.common_data.task_id,
                uid: c_struct.common_data.uid,
                token_id: c_struct.common_data.token_id,
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
                priority: c_struct.common_data.priority,
                background: c_struct.common_data.background,
            },
        };
        unsafe { DeleteCFormItem(c_struct.form_items_ptr) };
        unsafe { DeleteCFileSpec(c_struct.file_specs_ptr) };
        unsafe { DeleteCStringPtr(c_struct.body_file_names_ptr) };
        unsafe { DeleteCStringPtr(c_struct.certs_path_ptr) };
        task_config
    }
}

pub(crate) fn publish_event(bundle: &str, task_id: u32, state: State) {
    let len = bundle.len();
    unsafe {
        PublishStateChangeEvents(
            bundle.as_ptr() as *const c_char,
            len as u32,
            task_id,
            state as i32,
        );
    }
}

#[link(name = "download_server_cxx", kind = "static")]
extern "C" {
    pub(crate) fn GetNetworkInfo() -> *const NetworkInfo;
    pub(crate) fn DeleteCEachFileStatus(ptr: *const CEachFileStatus);
    pub(crate) fn UpdateNetworkInfo();
    pub(crate) fn PublishStateChangeEvents(
        bundle_name: *const c_char,
        bundle_name_len: u32,
        task_id: u32,
        state: i32,
    );

    pub(crate) fn RequestBackgroundNotify(
        msg: RequestTaskMsg,
        wrapped_path: CStringWrapper,
        wrapped_file_name: CStringWrapper,
        percent: u32,
    );
}
