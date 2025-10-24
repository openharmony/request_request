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

// Platform-specific imports for OpenHarmony
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

// C++ bridge for exposing Rust types to C++
#[cxx::bridge(namespace = "OHOS::Request")]
mod ffi {
    /// Specifies the type of network task to perform.
    #[derive(Clone, Copy, PartialEq, Debug)]
    #[repr(u8)]
    pub enum Action {
        /// Download action for retrieving data from a server.
        Download = 0,
        /// Upload action for sending data to a server.
        Upload,
        /// Wildcard action that matches any operation type.
        Any,
    }

    /// Determines the execution context for a task.
    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
    #[repr(u8)]
    pub enum Mode {
        /// Task runs in the background with lower priority.
        BackGround = 0,
        /// Task runs in the foreground with higher priority.
        FrontEnd,
        /// Wildcard mode that matches any execution context.
        Any,
    }
}

/// Represents the API version used by the request system.
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub(crate) enum Version {
    /// First API version.
    API9 = 1,
    /// Second API version with additional features.
    API10,
}

/// Specifies the network type required for task execution.
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum NetworkConfig {
    /// Task can run on any available network type.
    Any = 0,
    /// Task requires a Wi-Fi connection.
    Wifi,
    /// Task requires a cellular network connection.
    Cellular,
}

/// Minimum speed requirements for a network task.
/// 
/// If the network speed falls below the specified threshold for the given duration,
/// the task may be paused or rescheduled.
#[derive(Copy, Clone, Debug, Default)]
pub struct MinSpeed {
    /// Minimum acceptable speed in bytes per second.
    pub(crate) speed: i64,
    /// Duration in milliseconds that the speed must be sustained below the threshold
    /// before triggering a response.
    pub(crate) duration: i64,
}

/// Timeout configuration for network operations.
#[derive(Copy, Clone, Debug, Default)]
pub struct Timeout {
    /// Maximum time in milliseconds to wait for a connection to be established.
    pub(crate) connection_timeout: u64,
    /// Maximum time in milliseconds for the entire task to complete.
    pub(crate) total_timeout: u64,
}

/// Core configuration shared by all types of network tasks.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct CommonTaskConfig {
    /// Unique identifier for the task.
    pub(crate) task_id: u32,
    /// User ID associated with the task.
    pub(crate) uid: u64,
    /// Token ID for security verification.
    pub(crate) token_id: u64,
    /// Type of operation (download, upload, etc.).
    pub(crate) action: Action,
    /// Execution context (background, foreground).
    pub(crate) mode: Mode,
    /// Whether to overwrite existing files.
    pub(crate) cover: bool,
    /// Network type requirements.
    pub(crate) network_config: NetworkConfig,
    /// Whether task can run on metered networks.
    pub(crate) metered: bool,
    /// Whether task can run while roaming.
    pub(crate) roaming: bool,
    /// Whether to retry failed operations.
    pub(crate) retry: bool,
    /// Whether to follow HTTP redirects.
    pub(crate) redirect: bool,
    /// Index for ordering related tasks.
    pub(crate) index: u32,
    /// Timestamp for when the task can start.
    pub(crate) begins: u64,
    /// Timestamp for when the task must complete (-1 for no deadline).
    pub(crate) ends: i64,
    /// Whether to enable speed measurement.
    pub(crate) gauge: bool,
    /// Whether to use precise progress tracking.
    pub(crate) precise: bool,
    /// Priority level for task scheduling.
    pub(crate) priority: u32,
    /// Whether task should continue in background.
    pub(crate) background: bool,
    /// Whether to use multipart encoding for uploads.
    pub(crate) multipart: bool,
    /// Minimum speed requirements.
    pub(crate) min_speed: MinSpeed,
    /// Timeout settings for the task.
    pub(crate) timeout: Timeout,
}

/// Complete configuration for a network task.
/// 
/// Contains all necessary parameters to execute a download or upload operation,
/// including network preferences, file specifications, authentication details,
/// and execution constraints.
#[derive(Clone, Debug)]
pub struct TaskConfig {
    /// Bundle name of the requesting application.
    pub(crate) bundle: String,
    /// Type identifier for the bundle.
    pub(crate) bundle_type: u32,
    /// Atomic account associated with the task.
    pub(crate) atomic_account: String,
    /// Target URL for the network operation.
    pub(crate) url: String,
    /// Human-readable title for the task.
    pub(crate) title: String,
    /// Detailed description of the task.
    pub(crate) description: String,
    /// HTTP method to use (GET, POST, etc.).
    pub(crate) method: String,
    /// HTTP headers to include in the request.
    pub(crate) headers: HashMap<String, String>,
    /// Request body data.
    pub(crate) data: String,
    /// Authentication token.
    pub(crate) token: String,
    /// Proxy server configuration.
    pub(crate) proxy: String,
    /// Certificate pins for secure connections.
    pub(crate) certificate_pins: String,
    /// Additional custom parameters.
    pub(crate) extras: HashMap<String, String>,
    /// API version compatibility indicator.
    pub(crate) version: Version,
    /// Form data items for upload requests.
    pub(crate) form_items: Vec<FormItem>,
    /// File specifications for upload/download operations.
    pub(crate) file_specs: Vec<FileSpec>,
    /// Paths to body files for complex requests.
    pub(crate) body_file_paths: Vec<String>,
    /// Paths to custom certificates.
    pub(crate) certs_path: Vec<String>,
    /// Core configuration shared across task types.
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
                    // Check roaming and metered status constraints
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

    /// Determines if a task satisfies foreground execution requirements.
    /// 
    /// A task can run in the foreground if it's configured for background execution
    /// or if its associated UID is in the set of active foreground abilities.
    pub(crate) fn satisfy_foreground(&self, foreground_abilities: &HashSet<u64>) -> bool {
        self.common_data.mode == Mode::BackGround
            || foreground_abilities.contains(&self.common_data.uid)
    }
}

/// Internal representation of a task configuration optimized for C FFI.
/// 
/// Converts high-level Rust types into C-compatible representations
/// for interoperability with native code.
pub(crate) struct ConfigSet {
    /// HTTP headers serialized as a string.
    pub(crate) headers: String,
    /// Extra parameters serialized as a string.
    pub(crate) extras: String,
    /// Form items in C-compatible format.
    pub(crate) form_items: Vec<CFormItem>,
    /// File specifications in C-compatible format.
    pub(crate) file_specs: Vec<CFileSpec>,
    /// Body file names wrapped for C compatibility.
    pub(crate) body_file_names: Vec<CStringWrapper>,
    /// Certificate paths wrapped for C compatibility.
    pub(crate) certs_path: Vec<CStringWrapper>,
}

impl PartialOrd for Mode {
    /// Compares two execution modes to determine their relative priority.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Mode {
    /// Compares two execution modes to determine their relative priority.
    /// 
    /// Ordering is based on execution priority: FrontEnd > Any > BackGround
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_usize().cmp(&other.to_usize())
    }
}

impl Mode {
    /// Converts a Mode enum to a numeric priority value.
    fn to_usize(self) -> usize {
        match self {
            Mode::FrontEnd => 0,  // Highest priority
            Mode::Any => 1,
            Mode::BackGround => 2, // Lowest priority
            _ => unreachable!(),
        }
    }
}

impl From<u8> for Mode {
    /// Converts a raw u8 value to a Mode enum.
    /// 
    /// Maps numeric identifiers to their corresponding execution modes.
    /// Values outside the defined range default to Mode::Any.
    fn from(value: u8) -> Self {
        match value {
            0 => Mode::BackGround,
            1 => Mode::FrontEnd,
            _ => Mode::Any, // Default for unknown values
        }
    }
}

impl From<u8> for Action {
    /// Converts a raw u8 value to an Action enum.
    /// 
    /// Maps numeric identifiers to their corresponding action types.
    /// Values outside the defined range default to Action::Any.
    fn from(value: u8) -> Self {
        match value {
            0 => Action::Download,
            1 => Action::Upload,
            _ => Action::Any, // Default for unknown values
        }
    }
}

impl From<u8> for Version {
    /// Converts a raw u8 value to a Version enum.
    /// 
    /// Maps numeric identifiers to their corresponding API versions.
    /// Defaults to API9 for unsupported values.
    fn from(value: u8) -> Self {
        match value {
            2 => Version::API10,
            _ => Version::API9, // Default to earliest version for compatibility
        }
    }
}

impl From<u8> for NetworkConfig {
    /// Converts a raw u8 value to a NetworkConfig enum.
    /// 
    /// Maps numeric identifiers to their corresponding network configurations.
    /// Defaults to Wifi for unsupported values.
    fn from(value: u8) -> Self {
        match value {
            0 => NetworkConfig::Any,
            2 => NetworkConfig::Cellular,
            _ => NetworkConfig::Wifi, // Default for unknown values
        }
    }
}

impl TaskConfig {
    /// Creates a C-compatible configuration set from the current task config.
    /// 
    /// Transforms Rust-native types into formats suitable for FFI interactions
    /// with C/C++ code, including stringifying hash maps and converting vectors
    /// to C-compatible representations.
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

    /// Checks if the task configuration includes any user files.
    /// 
    /// Returns true if any file specification in the configuration
    /// is marked as a user file, false otherwise.
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
    /// Creates a default task configuration with sensible initial values.
    /// 
    /// Sets up a basic download task with common defaults that can be
    /// customized through the ConfigBuilder.
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

/// Builder pattern for constructing TaskConfig instances.
/// 
/// Provides a fluent interface for incrementally configuring network tasks
/// with method chaining for improved readability and usability.
pub struct ConfigBuilder {
    inner: TaskConfig,
}

impl ConfigBuilder {
    /// Creates a new builder with default task configuration.
    pub fn new() -> Self {
        Self {
            inner: TaskConfig::default(),
        }
    }

    /// Sets the target URL for the network operation.
    pub fn url(&mut self, url: &str) -> &mut Self {
        self.inner.url = url.to_string();
        self
    }

    /// Sets the API version compatibility level.
    pub fn version(&mut self, version: u8) -> &mut Self {
        self.inner.version = version.into();
        self
    }

    /// Adds a user file to the task configuration.
    pub fn file_spec(&mut self, file: File) -> &mut Self {
        self.inner.file_specs.push(FileSpec::user_file(file));
        self
    }

    /// Sets the operation type (download or upload).
    pub fn action(&mut self, action: Action) -> &mut Self {
        self.inner.common_data.action = action;
        self
    }

    /// Sets the execution context (background or foreground).
    pub fn mode(&mut self, mode: Mode) -> &mut Self {
        self.inner.common_data.mode = mode;
        self
    }

    /// Sets the name of the bundle requesting the task.
    pub fn bundle_name(&mut self, bundle_name: &str) -> &mut Self {
        self.inner.bundle = bundle_name.to_string();
        self
    }

    /// Sets the user ID associated with the task.
    pub fn uid(&mut self, uid: u64) -> &mut Self {
        self.inner.common_data.uid = uid;
        self
    }

    /// Sets the network type requirements for the task.
    pub fn network(&mut self, network: NetworkConfig) -> &mut Self {
        self.inner.common_data.network_config = network;
        self
    }

    /// Sets whether the task can run while roaming.
    pub fn roaming(&mut self, roaming: bool) -> &mut Self {
        self.inner.common_data.roaming = roaming;
        self
    }

    /// Sets whether the task can run on metered networks.
    pub fn metered(&mut self, metered: bool) -> &mut Self {
        self.inner.common_data.metered = metered;
        self
    }

    /// Constructs the final TaskConfig from the builder's current state.
    pub fn build(&mut self) -> TaskConfig {
        self.inner.clone()
    }

    /// Sets whether to follow HTTP redirects.
    pub fn redirect(&mut self, redirect: bool) -> &mut Self {
        self.inner.common_data.redirect = redirect;
        self
    }

    /// Sets the earliest time the task can start (timestamp in milliseconds).
    pub fn begins(&mut self, begins: u64) -> &mut Self {
        self.inner.common_data.begins = begins;
        self
    }

    /// Sets the latest time the task must complete (timestamp in milliseconds).
    pub fn ends(&mut self, ends: u64) -> &mut Self {
        self.inner.common_data.ends = ends as i64;
        self
    }

    /// Sets the HTTP method to use for the request.
    pub fn method(&mut self, method: &str) -> &mut Self {
        self.inner.method = method.to_string();
        self
    }

    /// Sets whether failed operations should be retried.
    pub fn retry(&mut self, retry: bool) -> &mut Self {
        self.inner.common_data.retry = retry;
        self
    }
}

#[cfg(feature = "oh")]
impl Serialize for TaskConfig {
    fn serialize(&self, parcel: &mut ipc::parcel::MsgParcel) -> ipc::IpcResult<()> {
        // Write primitive configuration values
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

        // Write speed and timeout configurations
        parcel.write(&self.common_data.min_speed.speed)?;
        parcel.write(&self.common_data.min_speed.duration)?;
        parcel.write(&self.common_data.timeout.connection_timeout)?;
        parcel.write(&self.common_data.timeout.total_timeout)?;

        // Write string fields
        parcel.write(&self.url)?;
        parcel.write(&self.title)?;
        parcel.write(&self.method)?;
        parcel.write(&self.token)?;
        parcel.write(&self.description)?;
        parcel.write(&self.data)?;
        parcel.write(&self.proxy)?;
        parcel.write(&self.certificate_pins)?;

        // Write certificate paths
        parcel.write(&(self.certs_path.len() as u32))?;
        for cert_path in &self.certs_path {
            parcel.write(cert_path)?;
        }

        // Write form items
        parcel.write(&(self.form_items.len() as u32))?;
        for form_item in &self.form_items {
            parcel.write(&form_item.name)?;
            parcel.write(&form_item.value)?;
        }
        
        // Write file specifications with special handling for user files
        parcel.write(&(self.file_specs.len() as u32))?;
        for file_spec in &self.file_specs {
            parcel.write(&file_spec.name)?;
            parcel.write(&file_spec.path)?;
            parcel.write(&file_spec.file_name)?;
            parcel.write(&file_spec.mime_type)?;
            parcel.write(&file_spec.is_user_file)?;
            if file_spec.is_user_file {
                // Safety: If is_user_file is true, the `fd` must be valid
                let file = unsafe { File::from_raw_fd(file_spec.fd.unwrap()) };
                parcel.write_file(file)?;
            }
        }

        // Write body file paths
        parcel.write(&(self.body_file_paths.len() as u32))?;
        for body_file_path in self.body_file_paths.iter() {
            parcel.write(body_file_path)?;
        }
        
        // Write headers map
        parcel.write(&(self.headers.len() as u32))?;
        for header in self.headers.iter() {
            parcel.write(header.0)?;
            parcel.write(header.1)?;
        }

        // Write extras map
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
        // Read primitive configuration values
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

        // Read speed and timeout configurations
        let min_speed: i64 = parcel.read()?;
        let min_duration: i64 = parcel.read()?;
        let connection_timeout: u64 = parcel.read()?;
        let total_timeout: u64 = parcel.read()?;

        // Read string fields
        let url: String = parcel.read()?;
        let title: String = parcel.read()?;
        let method: String = parcel.read()?;
        let token: String = parcel.read()?;
        let description: String = parcel.read()?;
        let data_base: String = parcel.read()?;
        let proxy: String = parcel.read()?;
        let certificate_pins: String = parcel.read()?;

        // Get caller information from IPC context
        let bundle = query_calling_bundle();
        let uid = ipc::Skeleton::calling_uid();
        let token_id = ipc::Skeleton::calling_full_token_id();

        // Read certificate paths with size validation
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

        // Read form items with size validation
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

        // Read file specifications with size validation and special handling for user files
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
                // Safety: Assumes the IPC system provides a valid file descriptor
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
                // Safety: Transfers ownership of the raw file descriptor
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

        // Read body file paths with size validation
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

        // Read headers map with size validation
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

        // Read extras map with size validation
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

        // Determine atomic account based on bundle type
        let atomic_account = if bundle_type == ATOMIC_SERVICE {
            GetOhosAccountUid()
        } else {
            "".to_string()
        };

        // Construct the final TaskConfig
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
