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

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::os::fd::FromRawFd;

pub use ffi::{Action, Mode};

cfg_oh! {
    use ipc::parcel::Serialize;
}

use super::reason::Reason;
use crate::manage::network::{NetworkState, NetworkType};
use crate::utils::c_wrapper::{CFileSpec, CFormItem, CStringWrapper};
use crate::utils::form_item::{FileSpec, FormItem};
use crate::utils::hashmap_to_string;
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    /// Action
    #[derive(Clone, Copy, PartialEq, Debug)]
    #[repr(u8)]
    pub enum Action {
        /// Download
        Download = 0,
        /// Upload
        Upload,
        /// Any
        Any,
    }

    /// Mode
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
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub(crate) enum Version {
    API9 = 1,
    API10,
}

/// NetworkConfig
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

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct CommonTaskConfig {
    pub(crate) task_id: u32,
    pub(crate) uid: u64,
    pub(crate) token_id: u64,
    pub(crate) action: Action,
    pub(crate) mode: Mode,
    pub(crate) cover: bool,
    pub(crate) network_config: NetworkConfig,
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
    pub(crate) multipart: bool,
}

/// task config
#[derive(Clone, Debug)]
pub struct TaskConfig {
    pub(crate) bundle: String,
    pub(crate) bundle_type: u32,
    pub(crate) atomic_account: String,
    pub(crate) url: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) method: String,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) data: String,
    pub(crate) token: String,
    pub(crate) proxy: String,
    pub(crate) certificate_pins: String,
    pub(crate) extras: HashMap<String, String>,
    pub(crate) version: Version,
    pub(crate) form_items: Vec<FormItem>,
    pub(crate) file_specs: Vec<FileSpec>,
    pub(crate) body_file_paths: Vec<String>,
    pub(crate) certs_path: Vec<String>,
    pub(crate) common_data: CommonTaskConfig,
}

impl TaskConfig {
    pub(crate) fn satisfy_network(&self, network: &NetworkState) -> Result<(), Reason> {
        // NetworkConfig::Cellular with NetworkType::Wifi is allowed
        match network {
            NetworkState::Offline => Err(Reason::NetworkOffline),
            NetworkState::Online(info) => match self.common_data.network_config {
                NetworkConfig::Any => Ok(()),
                NetworkConfig::Wifi if info.network_type == NetworkType::Cellular => {
                    Err(Reason::UnsupportedNetworkType)
                }
                _ => {
                    if (self.common_data.roaming || !info.is_roaming)
                        && (self.common_data.metered || !info.is_metered)
                    {
                        Ok(())
                    } else {
                        Err(Reason::UnsupportedNetworkType)
                    }
                }
            },
        }
    }

    pub(crate) fn satisfy_foreground(&self, foreground_abilities: &HashSet<u64>) -> bool {
        self.common_data.mode == Mode::BackGround
            || foreground_abilities.contains(&self.common_data.uid)
    }
}

pub(crate) struct ConfigSet {
    pub(crate) headers: String,
    pub(crate) extras: String,
    pub(crate) form_items: Vec<CFormItem>,
    pub(crate) file_specs: Vec<CFileSpec>,
    pub(crate) body_file_names: Vec<CStringWrapper>,
    pub(crate) certs_path: Vec<CStringWrapper>,
}

impl PartialOrd for Mode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Mode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let me = match *self {
            Mode::FrontEnd => 0,
            Mode::Any => 1,
            Mode::BackGround => 2,
            _ => unreachable!(),
        };
        let other = match *other {
            Mode::FrontEnd => 0,
            Mode::Any => 1,
            Mode::BackGround => 2,
            _ => unreachable!(),
        };
        me.cmp(&other)
    }
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        match value {
            0 => Mode::BackGround,
            1 => Mode::FrontEnd,
            _ => Mode::Any,
        }
    }
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

impl From<u8> for NetworkConfig {
    fn from(value: u8) -> Self {
        match value {
            0 => NetworkConfig::Any,
            2 => NetworkConfig::Cellular,
            _ => NetworkConfig::Wifi,
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

    pub(crate) fn contains_user_file(&self) -> bool {
        for specs in self.file_specs.iter() {
            if specs.is_user_file {
                return true;
            }
        }
        false
    }
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self {
            bundle_type: 0,
            atomic_account: "ohosAnonymousUid".to_string(),
            bundle: "xxx".to_string(),
            url: "".to_string(),
            title: "xxx".to_string(),
            description: "xxx".to_string(),
            method: "GET".to_string(),
            headers: Default::default(),
            data: "".to_string(),
            token: "xxx".to_string(),
            proxy: "".to_string(),
            extras: Default::default(),
            version: Version::API10,
            form_items: vec![],
            file_specs: vec![],
            body_file_paths: vec![],
            certs_path: vec![],
            certificate_pins: "".to_string(),
            common_data: CommonTaskConfig {
                task_id: 0,
                uid: 0,
                token_id: 0,
                action: Action::Download,
                mode: Mode::BackGround,
                cover: false,
                network_config: NetworkConfig::Any,
                metered: false,
                roaming: false,
                retry: false,
                redirect: true,
                index: 0,
                begins: 0,
                ends: -1,
                gauge: false,
                precise: false,
                priority: 0,
                background: false,
                multipart: false,
            },
        }
    }
}

/// ConfigBuilder
pub struct ConfigBuilder {
    inner: TaskConfig,
}

impl ConfigBuilder {
    /// Create a new ConfigBuilder
    pub fn new() -> Self {
        Self {
            inner: TaskConfig::default(),
        }
    }
    /// Set url
    pub fn url(&mut self, url: &str) -> &mut Self {
        self.inner.url = url.to_string();
        self
    }

    /// set version
    pub fn version(&mut self, version: u8) -> &mut Self {
        self.inner.version = version.into();
        self
    }

    /// Set title
    pub fn file_spec(&mut self, file: File) -> &mut Self {
        self.inner.file_specs.push(FileSpec::user_file(file));
        self
    }
    /// Set action
    pub fn action(&mut self, action: Action) -> &mut Self {
        self.inner.common_data.action = action;
        self
    }

    /// Set mode
    pub fn mode(&mut self, mode: Mode) -> &mut Self {
        self.inner.common_data.mode = mode;
        self
    }

    /// Set bundle name
    pub fn bundle_name(&mut self, bundle_name: &str) -> &mut Self {
        self.inner.bundle = bundle_name.to_string();
        self
    }

    /// Set uid
    pub fn uid(&mut self, uid: u64) -> &mut Self {
        self.inner.common_data.uid = uid;
        self
    }

    /// set network
    pub fn network(&mut self, network: NetworkConfig) -> &mut Self {
        self.inner.common_data.network_config = network;
        self
    }

    /// Set metered
    pub fn roaming(&mut self, roaming: bool) -> &mut Self {
        self.inner.common_data.roaming = roaming;
        self
    }

    /// set metered
    pub fn metered(&mut self, metered: bool) -> &mut Self {
        self.inner.common_data.metered = metered;
        self
    }

    /// build
    pub fn build(&mut self) -> TaskConfig {
        self.inner.clone()
    }

    /// redirect
    pub fn redirect(&mut self, redirect: bool) -> &mut Self {
        self.inner.common_data.redirect = redirect;
        self
    }

    /// begins
    pub fn begins(&mut self, begins: u64) -> &mut Self {
        self.inner.common_data.begins = begins;
        self
    }

    /// ends
    pub fn ends(&mut self, ends: u64) -> &mut Self {
        self.inner.common_data.ends = ends as i64;
        self
    }

    /// method
    pub fn method(&mut self, metered: &str) -> &mut Self {
        self.inner.method = metered.to_string();
        self
    }

    /// retry
    pub fn retry(&mut self, retry: bool) -> &mut Self {
        self.inner.common_data.retry = retry;
        self
    }
}

#[cfg(feature = "oh")]
impl Serialize for TaskConfig {
    fn serialize(&self, parcel: &mut ipc::parcel::MsgParcel) -> ipc::IpcResult<()> {
        parcel.write(&(self.common_data.action.repr as u32))?;
        parcel.write(&(self.version as u32))?;
        parcel.write(&(self.common_data.mode.repr as u32))?;
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ut_enum_action() {
        assert_eq!(Action::Download.repr, 0);
        assert_eq!(Action::Upload.repr, 1);
        assert_eq!(Action::Any.repr, 2);
    }

    #[test]
    fn ut_enum_mode() {
        assert_eq!(Mode::BackGround.repr, 0);
        assert_eq!(Mode::FrontEnd.repr, 1);
        assert_eq!(Mode::Any.repr, 2);
    }

    #[test]
    fn ut_enum_version() {
        assert_eq!(Version::API9 as u32, 1);
        assert_eq!(Version::API10 as u32, 2);
    }

    #[test]
    fn ut_enum_network_config() {
        assert_eq!(NetworkConfig::Any as u32, 0);
        assert_eq!(NetworkConfig::Wifi as u32, 1);
        assert_eq!(NetworkConfig::Cellular as u32, 2);
    }
}
