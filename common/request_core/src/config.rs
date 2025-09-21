// Copyright (c) 2023 Huawei Device Co., Ltd.
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
use std::os::fd::FromRawFd;

use crate::file::FileSpec;

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

pub struct TaskConfigBuilder {
    version: Version,

    url: Option<String>,
    headers: Option<HashMap<String, String>>,

    // network configuration
    enable_metered: Option<bool>,
    enable_roaming: Option<bool>,
    network_type: Option<NetworkConfig>,

    // description of the task
    description: Option<String>,
    title: Option<String>,

    // task config
    background: Option<bool>,

    // file
    file_path: Option<String>,

    method: Option<String>,
    index: Option<i64>,
    begins: Option<i64>,
    ends: Option<i64>,
    files: Option<Vec<FileSpec>>,
    data: Option<Vec<FormItem>>,
    action: Action,
}

impl TaskConfigBuilder {
    pub fn new(version: Version) -> Self {
        TaskConfigBuilder {
            version,
            url: None,
            headers: None,
            enable_metered: None,
            enable_roaming: None,
            network_type: None,
            description: None,
            title: None,
            background: None,
            file_path: None,
            method: None,
            index: None,
            begins: None,
            ends: None,
            files: None,
            data: None,
            action: Action::Download,
        }
    }

    pub fn url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }

    pub fn headers(&mut self, headers: HashMap<String, String>) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    pub fn metered(&mut self, enable: bool) -> &mut Self {
        self.enable_metered = Some(enable);
        self
    }

    pub fn roaming(&mut self, enable: bool) -> &mut Self {
        self.enable_roaming = Some(enable);
        self
    }

    pub fn network_type(&mut self, network_type: NetworkConfig) -> &mut Self {
        self.network_type = Some(network_type);
        self
    }

    pub fn description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }

    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn background(&mut self, background: bool) -> &mut Self {
        self.background = Some(background);
        self
    }

    pub fn file_path(&mut self, file_path: String) -> &mut Self {
        self.file_path = Some(file_path);
        self
    }

    pub fn method(&mut self, method: String) -> &mut Self {
        self.method = Some(method);
        self
    }

    pub fn index(&mut self, index: i64) -> &mut Self {
        self.index = Some(index);
        self
    }

    pub fn begins(&mut self, begins: i64) -> &mut Self {
        self.begins = Some(begins);
        self
    }

    pub fn ends(&mut self, ends: i64) -> &mut Self {
        self.ends = Some(ends);
        self
    }

    pub fn files(&mut self, files: Vec<FileSpec>) -> &mut Self {
        self.files = Some(files);
        self
    }

    pub fn data(&mut self, data: Vec<FormItem>) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn action(&mut self, action: Action) -> &mut Self {
        self.action = action;
        self
    }


    pub fn build(self) -> TaskConfig {
        TaskConfig {
            bundle: "".to_string(),
            bundle_type: 0,
            atomic_account: "".to_string(),
            url: self.url.unwrap_or_default(),
            title: self.title.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
            method: self.method.unwrap_or("GET".to_string()),
            headers: self.headers.unwrap_or_default(),
            data: "".to_string(),
            token: "".to_string(),
            proxy: "".to_string(),
            certificate_pins: "".to_string(),
            extras: HashMap::new(),
            version: self.version,
            form_items: self.data.unwrap_or(vec![]),
            file_specs: self.files.unwrap_or(vec![]),
            body_file_paths: vec![],
            certs_path: vec![],
            common_data: CommonTaskConfig {
                task_id: 0,
                uid: 0,
                token_id: 0,
                action: self.action,
                mode: Mode::FrontEnd,
                cover: false,
                network_config: NetworkConfig::Any,
                metered: self.enable_metered.unwrap_or(false),
                roaming: self.enable_roaming.unwrap_or(false),
                retry: false,
                redirect: true,
                index: self.index.unwrap_or(0i64) as u32,
                begins: self.begins.unwrap_or(0i64) as u64,
                ends: self.ends.unwrap_or(-1),
                gauge: false,
                precise: false,
                priority: 0,
                background: self.background.unwrap_or(false),
                multipart: false,
            },
        }
    }
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
        parcel.write(&0i64)?;
        parcel.write(&0i64)?;
        parcel.write(&0u64)?;
        parcel.write(&0u64)?;
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

#[derive(Clone, Debug)]
pub struct FormItem {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u32)]
pub enum Version {
    API9 = 1,
    API10,
}

impl From<u32> for Version {
    fn from(value: u32) -> Self {
        match value {
            1 => Version::API9,
            2 => Version::API10,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Action {
    /// Download
    Download = 0,
    /// Upload
    Upload,
}

impl From<u32> for Action {
    fn from(value: u32) -> Self {
        match value {
            0 => Action::Download,
            1 => Action::Upload,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Mode {
    /// BackGround
    BackGround = 0,
    /// ForeGround
    FrontEnd,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NetworkConfig {
    /// Any
    Any = 0,
    /// Wifi
    Wifi,
    /// Cellular
    Cellular,
}

impl From<i32> for NetworkConfig {
    fn from(value: i32) -> Self {
        match value {
            0 => NetworkConfig::Any,
            1 => NetworkConfig::Wifi,
            2 => NetworkConfig::Cellular,
            _ => unimplemented!(),
        }
    }
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
