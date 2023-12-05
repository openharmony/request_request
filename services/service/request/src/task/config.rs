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
use crate::utils::form_item::{FileSpec, FormItem};

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub(crate) enum Action {
    DownLoad = 0,
    UpLoad,
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

#[derive(Debug)]
pub(crate) struct TaskConfig {
    pub(crate) bundle: String,
    pub(crate) url: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) method: String,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) data: String,
    pub(crate) token: String,
    #[allow(unused)]
    pub(crate) extras: HashMap<String, String>,
    pub(crate) version: Version,
    pub(crate) form_items: Vec<FormItem>,
    pub(crate) file_specs: Vec<FileSpec>,
    pub(crate) body_file_names: Vec<String>,
    pub(crate) certs_path: Vec<String>,
    pub(crate) common_data: CommonTaskConfig,
}

impl From<u8> for Action {
    fn from(value: u8) -> Self {
        match value {
            0 => Action::DownLoad,
            1 => Action::UpLoad,
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
