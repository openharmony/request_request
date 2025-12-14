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

//! Configuration types for network tasks.
//!
//! This module provides structures and enums for configuring network operations,
//! including download and upload tasks, with various options for controlling
//! behavior, network preferences, and file handling.

use std::collections::HashMap;
use std::fs::File;
use std::os::fd::{FromRawFd, IntoRawFd, RawFd};

use crate::file::FileSpec;

/// Complete configuration for a network task.
///
/// This struct contains all configuration parameters needed to execute a network task,
/// including network settings, file specifications, metadata, and authentication details.
#[derive(Clone, Debug)]
pub struct TaskConfig {
    /// Bundle identifier associated with the task.
    pub bundle: String,
    /// Type identifier for the bundle.
    pub bundle_type: u32,
    /// Account identifier for atomic operations.
    pub atomic_account: String,
    /// Target URL for the network request.
    pub url: String,
    /// User-facing title for the task.
    pub title: String,
    /// Description of the task purpose.
    pub description: String,
    /// HTTP method to use (e.g., GET, POST).
    pub method: String,
    /// HTTP headers to include in the request.
    pub headers: HashMap<String, String>,
    /// Request body data as a string.
    pub data: String,
    /// Authentication token.
    pub token: String,
    /// Proxy server configuration.
    pub proxy: String,
    /// Certificate pinning configuration.
    pub certificate_pins: String,
    /// Additional configuration parameters.
    pub extras: HashMap<String, String>,
    /// API version to use for compatibility.
    pub version: Version,
    /// Form data items for multi-part requests.
    pub form_items: Vec<FormItem>,
    /// File specifications for upload tasks.
    pub file_specs: Vec<FileSpec>,
    /// Paths to files for request body.
    pub body_file_paths: Vec<String>,
    /// Paths to certificate files.
    pub certs_path: Vec<String>,
    /// Common task configuration parameters.
    pub common_data: CommonTaskConfig,
    pub saveas: String,
    pub overwrite: bool,
    pub notification: Notification,
}

/// Builder for creating a `TaskConfig` with a fluent interface.
///
/// Provides a convenient way to construct a `TaskConfig` instance with
/// selective configuration parameters.
///
/// # Examples
///
/// ```rust
/// let config = TaskConfigBuilder::new(Version::API10)
///     .url("https://example.com".to_string())
///     .title("Example Download".to_string())
///     .description("Download example file".to_string())
///     .background(true)
///     .build();
/// ```
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
    index: Option<i32>,
    begins: Option<i64>,
    ends: Option<i64>,
    files: Option<Vec<FileSpec>>,
    data: Option<Vec<FormItem>>,
    action: Action,
    // notification: Option<Notification>,
}

impl TaskConfigBuilder {
    /// Creates a new `TaskConfigBuilder` with the specified API version.
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
            // notification: None,
        }
    }

    /// Sets the target URL for the task.
    pub fn url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }

    /// Sets HTTP headers for the request.
    pub fn headers(&mut self, headers: HashMap<String, String>) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    /// Sets whether the task should run on metered connections.
    pub fn metered(&mut self, enable: bool) -> &mut Self {
        self.enable_metered = Some(enable);
        self
    }

    /// Sets whether the task should run on roaming connections.
    pub fn roaming(&mut self, enable: bool) -> &mut Self {
        self.enable_roaming = Some(enable);
        self
    }

    /// Sets the network type preference for the task.
    pub fn network_type(&mut self, network_type: NetworkConfig) -> &mut Self {
        self.network_type = Some(network_type);
        self
    }

    /// Sets the description for the task.
    pub fn description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }

    /// Sets the title for the task.
    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    /// Sets whether the task should run in background mode.
    pub fn background(&mut self, background: bool) -> &mut Self {
        self.background = Some(background);
        self
    }

    /// Sets the file path for the task output.
    pub fn file_path(&mut self, file_path: String) -> &mut Self {
        self.file_path = Some(file_path);
        self
    }

    pub fn method(&mut self, method: String) -> &mut Self {
        self.method = Some(method);
        self
    }

    pub fn index(&mut self, index: i32) -> &mut Self {
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

    // pub fn notification(&mut self, notification: Notification) -> &mut Self {
    //     self.notification = Some(notification);
    //     self
    // }

    /// Constructs a `TaskConfig` with the current builder configuration.
    ///
    /// # Notes
    ///
    /// Default values are used for any unspecified fields.
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
                network_config: self.network_type.unwrap_or(NetworkConfig::Any),
                metered: self.enable_metered.unwrap_or(false),
                roaming: self.enable_roaming.unwrap_or(false),
                retry: false,
                redirect: true,
                index: self.index.unwrap_or(0i32) as u32,
                begins: self.begins.unwrap_or(0i64) as u64,
                ends: self.ends.unwrap_or(-1),
                gauge: false,
                precise: false,
                priority: 0,
                background: self.background.unwrap_or(false),
                multipart: false,
                min_speed: MinSpeed {
                    speed: 0,
                    duration: 0,
                },
                timeout: Timeout {
                    connection_timeout: 0,
                    total_timeout: 0,
                },
            },
            saveas: self.file_path.unwrap_or_default(),
            overwrite: false,
            notification: Notification {
                title: None,
                text: None,
            },
            // notification: self.notification.unwrap_or(Notification {
            //     title: "".to_string(),
            //     text: "".to_string(),
            // }),
        }
    }
}

impl ipc::parcel::Serialize for TaskConfig {
    /// Serializes the task configuration to a message parcel.
    ///
    /// # Errors
    ///
    /// Returns `Err` if serialization fails for any reason.
    ///
    /// # Safety
    ///
    /// Uses `unsafe` block when converting raw file descriptors to `File` instances.
    fn serialize(&self, parcel: &mut ipc::parcel::MsgParcel) -> ipc::IpcResult<()> {
        // Serialize common configuration fields
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

        // Write placeholders for future fields
        parcel.write(&self.common_data.min_speed.speed)?;
        parcel.write(&self.common_data.min_speed.duration)?;
        parcel.write(&self.common_data.timeout.connection_timeout)?;
        parcel.write(&self.common_data.timeout.total_timeout)?;

        // Serialize basic string fields
        parcel.write(&self.url)?;
        parcel.write(&self.title)?;
        parcel.write(&self.method)?;
        parcel.write(&self.token)?;
        parcel.write(&self.description)?;
        parcel.write(&self.data)?;
        parcel.write(&self.proxy)?;
        parcel.write(&self.certificate_pins)?;

        // Serialize vector of certificate paths
        parcel.write(&(self.certs_path.len() as u32))?;
        for cert_path in &self.certs_path {
            parcel.write(cert_path)?;
        }

        // Serialize form items
        parcel.write(&(self.form_items.len() as u32))?;
        for form_item in &self.form_items {
            parcel.write(&form_item.name)?;
            parcel.write(&form_item.value)?;
        }

        // Serialize file specifications
        parcel.write(&(self.file_specs.len() as u32))?;
        for file_spec in &self.file_specs {
            parcel.write(&file_spec.name)?;
            parcel.write(&file_spec.path)?;
            parcel.write(&file_spec.file_name)?;
            parcel.write(&file_spec.mime_type)?;
            parcel.write(&file_spec.is_user_file)?;
            // Special handling for user-provided files
            if file_spec.is_user_file {
                // Safety: Assumes the file descriptor is valid and not owned elsewhere
                let file = unsafe { File::from_raw_fd(file_spec.fd.unwrap()) };
                parcel.write_file(file)?;
            }
        }

        // Serialize body file paths
        parcel.write(&(self.body_file_paths.len() as u32))?;
        for body_file_paths in self.body_file_paths.iter() {
            parcel.write(body_file_paths)?;
        }

        // Serialize HTTP headers
        parcel.write(&(self.headers.len() as u32))?;
        for header in self.headers.iter() {
            parcel.write(header.0)?;
            parcel.write(header.1)?;
        }

        // Serialize extra configuration parameters
        parcel.write(&(self.extras.len() as u32))?;
        for extra in self.extras.iter() {
            parcel.write(extra.0)?;
            parcel.write(extra.1)?;
        }

        //Serialize notification fields
        if let Some(title) = &self.notification.title {
            parcel.write(&true)?;
            parcel.write(title)?;
        } else {
            parcel.write(&false)?;
        }

        if let Some(text) = &self.notification.text {
            parcel.write(&true)?;
            parcel.write(text)?;
        } else {
            parcel.write(&false)?;
        }

        parcel.write(&false).unwrap(); //want_agent
        parcel.write(&false).unwrap(); //disable

        // Write gauge configuration based on task settings
        if self.common_data.gauge {
            parcel.write(&3u32).unwrap();
        } else {
            parcel.write(&1u32).unwrap();
        }
        Ok(())
    }
}

/// Represents a form field in a multi-part request.
#[derive(Clone, Debug)]
pub struct FormItem {
    /// Name of the form field.
    pub name: String,
    /// Value of the form field.
    pub value: String,
}

/// API version identifier for task configuration.
///
/// Used to ensure compatibility across different versions of the API.
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u32)]
pub enum Version {
    /// API version 9.
    API9 = 1,
    /// API version 10.
    API10,
}

impl From<u32> for Version {
    /// Converts a raw integer to a `Version` enum variant.
    ///
    /// # Panics
    ///
    /// Panics if the provided value does not correspond to a known version.
    fn from(value: u32) -> Self {
        match value {
            1 => Version::API9,
            2 => Version::API10,
            _ => unimplemented!(),
        }
    }
}

/// Type of network operation to perform.
#[derive(Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Action {
    /// Download content from a remote server.
    Download = 0,
    /// Upload content to a remote server.
    Upload,
}

impl From<u32> for Action {
    /// Converts a raw integer to an `Action` enum variant.
    ///
    /// # Panics
    ///
    /// Panics if the provided value does not correspond to a known action.
    fn from(value: u32) -> Self {
        match value {
            0 => Action::Download,
            1 => Action::Upload,
            _ => unimplemented!(),
        }
    }
}

/// Execution mode for a network task.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Mode {
    /// Task runs in the background, possibly with lower priority.
    BackGround = 0,
    /// Task runs in the foreground, typically for user-initiated operations.
    FrontEnd,
}

impl From<u32> for Mode {
    fn from(value: u32) -> Self {
        match value {
            0 => Mode::BackGround,
            1 => Mode::FrontEnd,
            _ => unimplemented!(),
        }
    }
}

/// Network type configuration for task execution.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NetworkConfig {
    /// Task can run on any available network.
    Any = 0,
    /// Task should only run on WiFi networks.
    Wifi,
    /// Task should only run on cellular networks.
    Cellular,
}

impl From<i32> for NetworkConfig {
    /// Converts a raw integer to a `NetworkConfig` enum variant.
    ///
    /// # Panics
    ///
    /// Panics if the provided value does not correspond to a known network configuration.
    fn from(value: i32) -> Self {
        match value {
            0 => NetworkConfig::Any,
            1 => NetworkConfig::Wifi,
            2 => NetworkConfig::Cellular,
            _ => unimplemented!(),
        }
    }
}

/// task min speed
#[derive(Copy, Clone, Debug, Default)]
pub struct MinSpeed {
    pub speed: i64,
    pub duration: i64,
}

/// task Timeout
#[derive(Copy, Clone, Debug, Default)]
pub struct Timeout {
    pub connection_timeout: u64,
    pub total_timeout: u64,
}

/// Common configuration parameters for network tasks.
///
/// Contains general task settings that apply to both download and upload operations.
#[derive(Clone, Debug)]
pub struct CommonTaskConfig {
    /// Unique identifier for the task.
    pub task_id: u32,
    /// User identifier associated with the task.
    pub uid: u64,
    /// Token identifier for authentication.
    pub token_id: u64,
    /// Type of operation to perform.
    pub action: Action,
    /// Execution mode (foreground or background).
    pub mode: Mode,
    /// Whether to overwrite existing files.
    pub cover: bool,
    /// Network type preference.
    pub network_config: NetworkConfig,
    /// Whether to allow execution on metered connections.
    pub metered: bool,
    /// Whether to allow execution while roaming.
    pub roaming: bool,
    /// Whether to retry on failure.
    pub retry: bool,
    /// Whether to follow redirects.
    pub redirect: bool,
    /// Index of the task in a batch.
    pub index: u32,
    /// Start position for range requests (in bytes).
    pub begins: u64,
    /// End position for range requests (in bytes).
    pub ends: i64,
    /// Whether to enable progress gauge updates.
    pub gauge: bool,
    /// Whether to use precise progress reporting.
    pub precise: bool,
    /// Task priority level.
    pub priority: u32,
    /// Whether to run as a background task.
    pub background: bool,
    /// Whether to use multi-part form encoding.
    pub multipart: bool,
    /// the min speed
    pub min_speed: MinSpeed,
    /// the timeout of task
    pub timeout: Timeout,
}

//deserialize by service file stub.rs function serialize_task_config
impl ipc::parcel::Deserialize for TaskConfig {
    fn deserialize(parcel: &mut ipc::parcel::MsgParcel) -> ipc::IpcResult<Self> {
        // deserialize common configuration fields
        let action_repr = parcel.read::<u32>()?;
        let action = Action::from(action_repr);
        let mode_repr = parcel.read::<u32>()?;
        let mode = Mode::from(mode_repr);
        let bundle_type: u32 = parcel.read()?;
        let cover: bool = parcel.read()?;
        let network: u32 = parcel.read()?;
        let metered: bool = parcel.read()?;
        let roaming: bool = parcel.read()?;
        let retry: bool = parcel.read()?;
        let redirect: bool = parcel.read()?;
        let index: u32 = parcel.read()?;
        let begins: i64 = parcel.read()?;
        let ends: i64 = parcel.read()?;
        let gauge: bool = parcel.read()?;
        let precise: bool = parcel.read()?;
        let priority: u32 = parcel.read()?;
        let background: bool = parcel.read()?;
        let multipart: bool = parcel.read()?;
        let bundle: String = parcel.read()?;
        let url: String = parcel.read()?;
        let title: String = parcel.read()?;
        let description: String = parcel.read()?;
        let method: String = parcel.read()?;

        // deserialize HashMap：headers
        let headers_len = parcel.read::<u32>()? as usize;
        let mut headers = HashMap::with_capacity(headers_len);
        for _ in 0..headers_len {
            let k = parcel.read::<String>()?;
            let v = parcel.read::<String>()?;
            headers.insert(k, v);
        }
        let data = parcel.read::<String>()?;
        let token = parcel.read::<String>()?;

        // deserialize HashMap：extras
        let extras_len = parcel.read::<u32>()? as usize;
        let mut extras = HashMap::with_capacity(extras_len);
        for _ in 0..extras_len {
            let k = parcel.read::<String>()?;
            let v = parcel.read::<String>()?;
            extras.insert(k, v);
        }
        let version = parcel.read::<u32>()?;

        // deserialize form_items
        let form_len = parcel.read::<u32>()? as usize;
        let mut form_items = Vec::with_capacity(form_len);
        for _ in 0..form_len {
            let name = parcel.read::<String>()?;
            let value = parcel.read::<String>()?;
            form_items.push(FormItem { name, value });
        }

        // deserialize file_specs
        let file_specs_len = parcel.read::<u32>()? as usize;
        let mut file_specs = Vec::with_capacity(file_specs_len);
        for _ in 0..file_specs_len {
            let name = parcel.read::<String>()?;
            let path = parcel.read::<String>()?;
            let file_name = parcel.read::<String>()?;
            let mime_type = parcel.read::<String>()?;
            file_specs.push(FileSpec { name, path, file_name, mime_type, is_user_file: false, fd: None });
        }

        // deserialize body_file_names
        let body_file_names_len = parcel.read::<u32>()? as usize;
        let mut body_file_names = Vec::with_capacity(body_file_names_len);
        for _ in 0..body_file_names_len {
            let name = parcel.read::<String>()?;
            body_file_names.push(name);
        }
        
        // deserialize min_speed
        let min_speed_speed = parcel.read::<i64>()?;
        let min_speed_duration = parcel.read::<i64>()?;

        Ok(TaskConfig {
            bundle,
            bundle_type,
            atomic_account: "".to_string(),
            url,
            title,
            description,
            method,
            headers,
            data,
            token,
            proxy: "".to_string(),
            certificate_pins: "".to_string(),
            extras,
            version: version.into(),
            form_items,
            file_specs,
            body_file_paths: vec![],
            certs_path: vec![],
            common_data: CommonTaskConfig {
                task_id: 0, uid: 0, token_id: 0, action, mode, cover, network_config: NetworkConfig::Any,
                metered, roaming, retry, redirect, index, begins: begins as u64, ends,
                gauge, precise, priority, background, multipart,
                min_speed: MinSpeed{ speed: min_speed_speed, duration: min_speed_duration },
                timeout: Timeout{connection_timeout: 0, total_timeout: 0}
            },
            saveas: "".to_string(),
            overwrite: cover,
            notification: Notification {
                title: None,
                text: None,
            },
        })
    }
}

#[derive(Clone, Debug)]
pub struct Notification {
    pub title: Option<String>,
    pub text: Option<String>,
}
