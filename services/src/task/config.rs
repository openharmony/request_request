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
use std::os::fd::{FromRawFd, IntoRawFd, RawFd};

pub use ffi::{Action, Mode};
use ipc::IpcStatusCode;

cfg_oh! {
    use ipc::parcel::Serialize;
    use ipc::parcel::Deserialize;
}

use super::reason::Reason;
use super::ATOMIC_SERVICE;
use crate::manage::account::GetOhosAccountUid;
use crate::manage::network::{NetworkState, NetworkType};
use crate::utils::c_wrapper::{CFileSpec, CFormItem, CStringWrapper};
use crate::utils::form_item::{FileSpec, FormItem};
use crate::utils::{hashmap_to_string, query_calling_bundle};

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

/// task min speed
#[derive(Copy, Clone, Debug, Default)]
pub struct MinSpeed {
    pub(crate) speed: i64,
    pub(crate) duration: i64,
}

/// task Timeout
#[derive(Copy, Clone, Debug, Default)]
pub struct Timeout {
    pub(crate) connection_timeout: u64,
    pub(crate) total_timeout: u64,
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
    pub(crate) min_speed: MinSpeed,
    pub(crate) timeout: Timeout,
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
        self.to_usize().cmp(&other.to_usize())
    }
}

impl Mode {
    fn to_usize(self) -> usize {
        match self {
            Mode::FrontEnd => 0,
            Mode::Any => 1,
            Mode::BackGround => 2,
            _ => unreachable!(),
        }
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
                min_speed: MinSpeed::default(),
                timeout: Timeout::default(),
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
        parcel.write(&self.common_data.min_speed.speed)?;
        parcel.write(&self.common_data.min_speed.duration)?;
        parcel.write(&self.common_data.timeout.connection_timeout)?;
        parcel.write(&self.common_data.timeout.total_timeout)?;
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

#[cfg(feature = "oh")]
impl Deserialize for TaskConfig {
    fn deserialize(parcel: &mut ipc::parcel::MsgParcel) -> ipc::IpcResult<Self> {
        let action: u32 = parcel.read()?;
        let action: Action = Action::from(action as u8);
        let version: u32 = parcel.read()?;
        let version: Version = Version::from(version as u8);
        let mode: u32 = parcel.read()?;
        let mode: Mode = Mode::from(mode as u8);
        let bundle_type: u32 = parcel.read()?;
        let cover: bool = parcel.read()?;
        let network: u32 = parcel.read()?;
        let network_config = NetworkConfig::from(network as u8);
        let metered: bool = parcel.read()?;
        let roaming: bool = parcel.read()?;
        let retry: bool = parcel.read()?;
        let redirect: bool = parcel.read()?;
        let background: bool = parcel.read()?;
        let multipart: bool = parcel.read()?;
        let index: u32 = parcel.read()?;
        let begins: i64 = parcel.read()?;
        let ends: i64 = parcel.read()?;
        let gauge: bool = parcel.read()?;
        let precise: bool = parcel.read()?;
        let priority: u32 = parcel.read()?;
        let min_speed: i64 = parcel.read()?;
        let min_duration: i64 = parcel.read()?;
        let connection_timeout: u64 = parcel.read()?;
        let total_timeout: u64 = parcel.read()?;
        let url: String = parcel.read()?;
        let title: String = parcel.read()?;
        let method: String = parcel.read()?;
        let token: String = parcel.read()?;
        let description: String = parcel.read()?;
        let data_base: String = parcel.read()?;
        let proxy: String = parcel.read()?;
        let certificate_pins: String = parcel.read()?;
        let bundle = query_calling_bundle();
        let uid = ipc::Skeleton::calling_uid();
        let token_id = ipc::Skeleton::calling_full_token_id();
        let certs_path_size: u32 = parcel.read()?;
        if certs_path_size > parcel.readable() as u32 {
            error!("deserialize failed: certs_path_size too large");
            sys_event!(
                ExecFault,
                DfxCode::INVALID_IPC_MESSAGE_A00,
                "deserialize failed: certs_path_size too large"
            );
            return Err(IpcStatusCode::Failed);
        }
        let mut certs_path = Vec::new();
        for _ in 0..certs_path_size {
            let cert_path: String = parcel.read()?;
            certs_path.push(cert_path);
        }

        let form_size: u32 = parcel.read()?;
        if form_size > parcel.readable() as u32 {
            error!("deserialize failed: form_size too large");
            sys_event!(
                ExecFault,
                DfxCode::INVALID_IPC_MESSAGE_A00,
                "deserialize failed: form_size too large"
            );
            return Err(IpcStatusCode::Failed);
        }
        let mut form_items = Vec::new();
        for _ in 0..form_size {
            let name: String = parcel.read()?;
            let value: String = parcel.read()?;
            form_items.push(FormItem { name, value });
        }

        let file_size: u32 = parcel.read()?;
        if file_size > parcel.readable() as u32 {
            error!("deserialize failed: file_specs size too large");
            sys_event!(
                ExecFault,
                DfxCode::INVALID_IPC_MESSAGE_A00,
                "deserialize failed: file_specs size too large"
            );
            return Err(IpcStatusCode::Failed);
        }
        let mut file_specs: Vec<FileSpec> = Vec::new();
        for _ in 0..file_size {
            let name: String = parcel.read()?;
            let path: String = parcel.read()?;
            let file_name: String = parcel.read()?;
            let mime_type: String = parcel.read()?;
            let is_user_file: bool = parcel.read()?;
            let mut fd: Option<RawFd> = None;
            if is_user_file {
                let raw_fd = unsafe { parcel.read_raw_fd() };
                if raw_fd < 0 {
                    error!("Failed to open user file, fd: {}", raw_fd);
                    sys_event!(
                        ExecFault,
                        DfxCode::INVALID_IPC_MESSAGE_A00,
                        "deserialize failed: failed to open user file"
                    );
                    return Err(IpcStatusCode::Failed);
                }
                let ipc_fd = unsafe { File::from_raw_fd(raw_fd) };
                fd = Some(ipc_fd.into_raw_fd());
            }
            file_specs.push(FileSpec {
                name,
                path,
                file_name,
                mime_type,
                is_user_file,
                fd,
            });
        }

        // Response bodies fd.
        let body_file_size: u32 = parcel.read()?;
        if body_file_size > parcel.readable() as u32 {
            error!("deserialize failed: body_file size too large");
            sys_event!(
                ExecFault,
                DfxCode::INVALID_IPC_MESSAGE_A00,
                "deserialize failed: body_file size too large"
            );
            return Err(IpcStatusCode::Failed);
        }

        let mut body_file_paths: Vec<String> = Vec::new();
        for _ in 0..body_file_size {
            let file_name: String = parcel.read()?;
            body_file_paths.push(file_name);
        }

        let header_size: u32 = parcel.read()?;
        if header_size > parcel.readable() as u32 {
            error!("deserialize failed: header size too large");
            sys_event!(
                ExecFault,
                DfxCode::INVALID_IPC_MESSAGE_A00,
                "deserialize failed: header size too large"
            );
            return Err(IpcStatusCode::Failed);
        }
        let mut headers: HashMap<String, String> = HashMap::new();
        for _ in 0..header_size {
            let key: String = parcel.read()?;
            let value: String = parcel.read()?;
            headers.insert(key, value);
        }

        let extras_size: u32 = parcel.read()?;
        if extras_size > parcel.readable() as u32 {
            error!("deserialize failed: extras size too large");
            sys_event!(
                ExecFault,
                DfxCode::INVALID_IPC_MESSAGE_A00,
                "deserialize failed: extras size too large"
            );
            return Err(IpcStatusCode::Failed);
        }
        let mut extras: HashMap<String, String> = HashMap::new();
        for _ in 0..extras_size {
            let key: String = parcel.read()?;
            let value: String = parcel.read()?;
            extras.insert(key, value);
        }

        let atomic_account = if bundle_type == ATOMIC_SERVICE {
            GetOhosAccountUid()
        } else {
            "".to_string()
        };

        let task_config = TaskConfig {
            bundle,
            bundle_type,
            atomic_account,
            url,
            title,
            description,
            method,
            headers,
            data: data_base,
            token,
            proxy,
            certificate_pins,
            extras,
            version,
            form_items,
            file_specs,
            body_file_paths,
            certs_path,
            common_data: CommonTaskConfig {
                task_id: 0,
                uid,
                token_id,
                action,
                mode,
                cover,
                network_config,
                metered,
                roaming,
                retry,
                redirect,
                index,
                begins: begins as u64,
                ends,
                gauge,
                precise,
                priority,
                background,
                multipart,
                min_speed: MinSpeed {
                    speed: min_speed,
                    duration: min_duration,
                },
                timeout: Timeout {
                    connection_timeout,
                    total_timeout,
                },
            },
        };
        Ok(task_config)
    }
}

#[cfg(test)]
mod ut_config {
    include!("../../tests/ut/task/ut_config.rs");
}
