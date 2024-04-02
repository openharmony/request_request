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

use std::collections::HashMap;

use super::info::Mode;
use crate::utils::c_wrapper::{CFileSpec, CFormItem, CStringWrapper};
use crate::utils::form_item::{FileSpec, FormItem};
use crate::utils::hashmap_to_string;

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub(crate) enum Action {
    Download = 0,
    Upload,
    Any,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub(crate) enum Version {
    API9 = 1,
    API10,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub(crate) enum Network {
    Any = 0,
    Wifi,
    Cellular,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct CommonTaskConfig {
    pub(crate) task_id: u32,
    pub(crate) uid: u64,
    pub(crate) token_id: u64,
    pub(crate) action: Action,
    pub(crate) mode: Mode,
    pub(crate) cover: bool,
    pub(crate) network: Network,
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

#[derive(Clone, Debug)]
pub(crate) struct TaskConfig {
    pub(crate) bundle: String,
    pub(crate) url: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) method: String,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) data: String,
    pub(crate) token: String,
    pub(crate) proxy: String,
    #[allow(unused)]
    pub(crate) extras: HashMap<String, String>,
    pub(crate) version: Version,
    pub(crate) form_items: Vec<FormItem>,
    pub(crate) file_specs: Vec<FileSpec>,
    pub(crate) body_file_paths: Vec<String>,
    pub(crate) certs_path: Vec<String>,
    pub(crate) common_data: CommonTaskConfig,
}

pub(crate) struct ConfigSet {
    pub(crate) headers: String,
    pub(crate) extras: String,
    pub(crate) form_items: Vec<CFormItem>,
    pub(crate) file_specs: Vec<CFileSpec>,
    pub(crate) body_file_names: Vec<CStringWrapper>,
    pub(crate) certs_path: Vec<CStringWrapper>,
}

impl From<u8> for Action {
    fn from(value: u8) -> Self {
        match value {
            0 => Action::Download,
            1 => Action::Upload,
            _ => Action::Any,
        }
    }
}

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        match value {
            2 => Version::API10,
            _ => Version::API9,
        }
    }
}

impl From<u8> for Network {
    fn from(value: u8) -> Self {
        match value {
            0 => Network::Any,
            2 => Network::Cellular,
            _ => Network::Wifi,
        }
    }
}

impl TaskConfig {
    pub(crate) fn build_config_set(&self) -> ConfigSet {
        ConfigSet {
            headers: hashmap_to_string(&self.headers),
            extras: hashmap_to_string(&self.extras),
            form_items: self.form_items.iter().map(|x| x.to_c_struct()).collect(),
            file_specs: self.file_specs.iter().map(|x| x.to_c_struct()).collect(),
            body_file_names: self
                .body_file_paths
                .iter()
                .map(CStringWrapper::from)
                .collect(),
            certs_path: self.certs_path.iter().map(CStringWrapper::from).collect(),
        }
    }
}