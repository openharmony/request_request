// Copyright (C) 2025 Huawei Device Co., Ltd.
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
use std::fs::File;
use std::os::fd::{FromRawFd, RawFd};

pub mod interface;

#[derive(Clone, Debug)]
pub struct FormItem {
    pub name: String,
    pub value: String,
}

/// File Spec
#[derive(Clone, Debug)]
pub struct FileSpec {
    /// Name
    pub name: String,
    /// path
    pub path: String,
    /// file_name
    pub file_name: String,
    /// mime_type
    pub mime_type: String,
    /// is_user_file
    pub is_user_file: bool,
    /// Only for user file.
    pub fd: Option<RawFd>,
}

impl FileSpec {
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
            path: "".to_owned(),
            file_name: "".to_owned(),
            mime_type: "".to_owned(),
            is_user_file: false,
            fd: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum Version {
    API9 = 1,
    API10,
}

/// task config
#[derive(Clone, Debug)]
pub struct TaskConfig {
    pub bundle: String,
    pub bundle_type: u32,
    pub atomic_account: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub data: String,
    pub token: String,
    pub proxy: String,
    pub certificate_pins: String,
    pub extras: HashMap<String, String>,
    pub version: Version,
    pub form_items: Vec<FormItem>,
    pub file_specs: Vec<FileSpec>,
    pub body_file_paths: Vec<String>,
    pub certs_path: Vec<String>,
    pub common_data: CommonTaskConfig,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    /// Download
    Download = 0,
    /// Upload
    Upload,
    /// Any
    Any,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[repr(u8)]
pub enum Mode {
    /// BackGround
    BackGround = 0,
    /// ForeGround
    FrontEnd,
    /// Any
    Any,
}
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum NetworkConfig {
    /// Any
    Any = 0,
    /// Wifi
    Wifi,
    /// Cellular
    Cellular,
}

#[derive(Clone, Debug)]
pub struct CommonTaskConfig {
    pub task_id: u32,
    pub uid: u64,
    pub token_id: u64,
    pub action: Action,
    pub mode: Mode,
    pub cover: bool,
    pub network_config: NetworkConfig,
    pub metered: bool,
    pub roaming: bool,
    pub retry: bool,
    pub redirect: bool,
    pub index: u32,
    pub begins: u64,
    pub ends: i64,
    pub gauge: bool,
    pub precise: bool,
    pub priority: u32,
    pub background: bool,
    pub multipart: bool,
}

impl ipc::parcel::Serialize for TaskConfig {
    fn serialize(&self, parcel: &mut ipc::parcel::MsgParcel) -> ipc::IpcResult<()> {
        parcel.write(&(self.common_data.action.clone() as u32))?;
        parcel.write(&(self.version as u32))?;
        parcel.write(&(self.common_data.mode as u32))?;
        parcel.write(&self.bundle_type)?;
        parcel.write(&self.common_data.cover)?;
        parcel.write(&(self.common_data.network_config as u32))?;
        parcel.write(&(self.common_data.metered))?;
        parcel.write(&self.common_data.roaming)?;
        parcel.write(&(self.common_data.retry))?;
        parcel.write(&(self.common_data.redirect))?;
        parcel.write(&(self.common_data.background))?;
        parcel.write(&(self.common_data.multipart))?;
        parcel.write(&self.common_data.index)?;
        parcel.write(&(self.common_data.begins as i64))?;
        parcel.write(&self.common_data.ends)?;
        parcel.write(&self.common_data.gauge)?;
        parcel.write(&self.common_data.precise)?;
        parcel.write(&self.common_data.priority)?;
        parcel.write(&self.url)?;
        parcel.write(&self.title)?;
        parcel.write(&self.method)?;
        parcel.write(&self.token)?;
        parcel.write(&self.description)?;
        parcel.write(&self.data)?;
        parcel.write(&self.proxy)?;
        parcel.write(&self.certificate_pins)?;

        parcel.write(&(self.certs_path.len() as u32))?;
        for cert_path in &self.certs_path {
            parcel.write(cert_path)?;
        }

        parcel.write(&(self.form_items.len() as u32))?;
        for form_item in &self.form_items {
            parcel.write(&form_item.name)?;
            parcel.write(&form_item.value)?;
        }
        parcel.write(&(self.file_specs.len() as u32))?;
        for file_spec in &self.file_specs {
            parcel.write(&file_spec.name)?;
            parcel.write(&file_spec.path)?;
            parcel.write(&file_spec.file_name)?;
            parcel.write(&file_spec.mime_type)?;
            parcel.write(&file_spec.is_user_file)?;
            if file_spec.is_user_file {
                let file = unsafe { File::from_raw_fd(file_spec.fd.unwrap()) };
                parcel.write_file(file)?;
            }
        }

        parcel.write(&(self.body_file_paths.len() as u32))?;
        for body_file_paths in self.body_file_paths.iter() {
            parcel.write(body_file_paths)?;
        }
        parcel.write(&(self.headers.len() as u32))?;
        for header in self.headers.iter() {
            parcel.write(header.0)?;
            parcel.write(header.1)?;
        }

        parcel.write(&(self.extras.len() as u32))?;
        for extra in self.extras.iter() {
            parcel.write(extra.0)?;
            parcel.write(extra.1)?;
        }

        Ok(())
    }
}
