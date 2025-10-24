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

//! Foreign Function Interface (FFI) types and conversions for task operations.
//! 
//! This module provides C-compatible structs and conversion methods to bridge
//! between Rust and C code for task configuration, information, and progress updates.

use super::config::{
    Action, CommonTaskConfig, ConfigSet, MinSpeed, Mode, NetworkConfig, TaskConfig, Timeout,
    Version,
};
use super::info::{CommonTaskInfo, InfoSet, TaskInfo, UpdateInfo};
use super::notify::{CommonProgress, Progress};
use crate::task::info::State;
use crate::utils::c_wrapper::{CFileSpec, CFormItem, CStringWrapper};

cfg_oh! {
    use crate::utils::c_wrapper::{DeleteCFileSpec, DeleteCFormItem, DeleteCStringPtr};
}

use crate::utils::form_item::{FileSpec, FormItem};
use crate::utils::{build_vec, split_string, string_to_hashmap};

/// C-compatible representation of task configuration.
///
/// This struct provides a C-ABI compatible interface for passing task configuration
/// between Rust and C code. It contains all necessary fields to configure a download
/// or upload task, including URLs, headers, form data, and network settings.
#[repr(C)]
pub(crate) struct CTaskConfig {
    /// Bundle identifier of the application owning the task.
    pub(crate) bundle: CStringWrapper,
    /// Type of bundle (application type identifier).
    pub(crate) bundle_type: u8,
    /// Atomic account identifier for the task owner.
    pub(crate) atomic_account: CStringWrapper,
    /// URL to download from or upload to.
    pub(crate) url: CStringWrapper,
    /// Human-readable title of the task.
    pub(crate) title: CStringWrapper,
    /// Human-readable description of the task.
    pub(crate) description: CStringWrapper,
    /// HTTP method (GET, POST, etc.).
    pub(crate) method: CStringWrapper,
    /// HTTP headers as a JSON string.
    pub(crate) headers: CStringWrapper,
    /// Request body data.
    pub(crate) data: CStringWrapper,
    /// Authentication token.
    pub(crate) token: CStringWrapper,
    /// Proxy settings.
    pub(crate) proxy: CStringWrapper,
    /// Certificate pins for SSL verification.
    pub(crate) certificate_pins: CStringWrapper,
    /// Additional task-specific data as a JSON string.
    pub(crate) extras: CStringWrapper,
    /// API version identifier.
    pub(crate) version: u8,
    /// Pointer to an array of form items for POST requests.
    pub(crate) form_items_ptr: *const CFormItem,
    /// Length of the form items array.
    pub(crate) form_items_len: u32,
    /// Pointer to an array of file specifications.
    pub(crate) file_specs_ptr: *const CFileSpec,
    /// Length of the file specifications array.
    pub(crate) file_specs_len: u32,
    /// Pointer to an array of file names for request body.
    pub(crate) body_file_names_ptr: *const CStringWrapper,
    /// Length of the body file names array.
    pub(crate) body_file_names_len: u32,
    /// Pointer to an array of certificate paths.
    pub(crate) certs_path_ptr: *const CStringWrapper,
    /// Length of the certificate paths array.
    pub(crate) certs_path_len: u32,
    /// Common configuration data for the task.
    pub(crate) common_data: CommonCTaskConfig,
}

/// C-compatible representation of common task configuration data.
///
/// Contains shared configuration parameters used by all task types, including
/// identifiers, network preferences, scheduling options, and performance settings.
#[repr(C)]
pub(crate) struct CommonCTaskConfig {
    /// Unique identifier for the task.
    pub(crate) task_id: u32,
    /// User ID that owns the task.
    pub(crate) uid: u64,
    /// Token ID for authentication.
    pub(crate) token_id: u64,
    /// Action type identifier (download, upload, etc.).
    pub(crate) action: u8,
    /// Task mode identifier.
    pub(crate) mode: u8,
    /// Whether to overwrite existing files.
    pub(crate) cover: bool,
    /// Network configuration identifier.
    pub(crate) network: u8,
    /// Whether to allow downloads on metered connections.
    pub(crate) metered: bool,
    /// Whether to allow downloads while roaming.
    pub(crate) roaming: bool,
    /// Whether to retry on failure.
    pub(crate) retry: bool,
    /// Whether to follow redirects.
    pub(crate) redirect: bool,
    /// Index of the task in a sequence.
    pub(crate) index: u32,
    /// Start time for scheduling (timestamp in milliseconds).
    pub(crate) begins: u64,
    /// End time for scheduling (timestamp in milliseconds, or -1 for no end).
    pub(crate) ends: i64,
    /// Whether to show progress gauge.
    pub(crate) gauge: bool,
    /// Whether to use precise progress reporting.
    pub(crate) precise: bool,
    /// Task priority level.
    pub(crate) priority: u32,
    /// Whether to run in background.
    pub(crate) background: bool,
    /// Whether to use multipart upload.
    pub(crate) multipart: bool,
    /// Minimum speed requirements for the task.
    pub(crate) min_speed: CMinSpeed,
    /// Timeout settings for the task.
    pub(crate) timeout: CTimeout,
}

/// C-compatible representation of minimum speed requirements.
///
/// Specifies the minimum download/upload speed threshold and the duration that
/// speed must be maintained before the task is considered to be running too slowly.
#[repr(C)]
pub(crate) struct CMinSpeed {
    /// Minimum speed threshold in bytes per second.
    pub(crate) speed: i64,
    /// Duration in milliseconds to check for minimum speed compliance.
    pub(crate) duration: i64,
}

/// C-compatible representation of timeout settings.
///
/// Contains connection and total timeouts for controlling how long operations
/// can take before being terminated.
#[repr(C)]
pub(crate) struct CTimeout {
    /// Connection timeout in milliseconds.
    pub(crate) connection_timeout: u64,
    /// Total operation timeout in milliseconds.
    pub(crate) total_timeout: u64,
}

/// C-compatible representation of task progress information.
///
/// This struct provides a way to pass progress updates between Rust and C code,
/// including common progress data, file sizes, processed bytes, and extra information.
#[repr(C)]
pub(crate) struct CProgress {
    /// Common progress information (state, percentages, etc.).
    pub(crate) common_data: CommonProgress,
    /// File sizes as a comma-separated string of integers.
    pub(crate) sizes: CStringWrapper,
    /// Processed bytes as a comma-separated string of integers.
    pub(crate) processed: CStringWrapper,
    /// Additional progress information as a JSON string.
    pub(crate) extras: CStringWrapper,
}

impl Progress {
    /// Converts a Rust Progress struct to its C-compatible representation.
    ///
    /// # Arguments
    ///
    /// * `sizes` - Comma-separated string of file sizes.
    /// * `processed` - Comma-separated string of processed bytes.
    /// * `extras` - JSON string with additional progress information.
    ///
    /// # Returns
    ///
    /// Returns a `CProgress` struct ready to be passed across the FFI boundary.
    pub(crate) fn to_c_struct(&self, sizes: &str, processed: &str, extras: &str) -> CProgress {
        CProgress {
            common_data: self.common_data.clone(),
            sizes: CStringWrapper::from(sizes),
            processed: CStringWrapper::from(processed),
            extras: CStringWrapper::from(extras),
        }
    }

    /// Converts a C-compatible Progress struct to a Rust Progress struct.
    ///
    /// # Arguments
    ///
    /// * `c_struct` - The CProgress struct to convert from.
    ///
    /// # Returns
    ///
    /// Returns a Rust `Progress` struct with data parsed from the C representation.
    ///
    /// # Notes
    ///
    /// Parsing errors will result in default values (0 for numbers, empty map for extras).
    pub(crate) fn from_c_struct(c_struct: &CProgress) -> Self {
        Progress {
            common_data: c_struct.common_data.clone(),
            // Parse comma-separated string of sizes into a vector of i64
            sizes: split_string(&mut c_struct.sizes.to_string())
                .map(|s| s.parse::<i64>().unwrap_or_default())
                .collect(),
            // Parse comma-separated string of processed bytes into a vector of usize
            processed: split_string(&mut c_struct.processed.to_string())
                .map(|s| s.parse::<usize>().unwrap_or_default())
                .collect(),
            // Parse JSON string of extras into a HashMap
            extras: string_to_hashmap(&mut c_struct.extras.to_string()),
        }
    }
}

/// C-compatible representation of task information.
///
/// This struct provides a way to pass task details between Rust and C code,
/// including bundle information, URLs, form data, file information, progress updates,
/// and common task metadata.
#[repr(C)]
pub(crate) struct CTaskInfo {
    /// Bundle identifier for the task.
    pub(crate) bundle: CStringWrapper,
    /// URL associated with the task.
    pub(crate) url: CStringWrapper,
    /// Additional task data as a string.
    pub(crate) data: CStringWrapper,
    /// Authentication token for the task.
    pub(crate) token: CStringWrapper,
    /// Pointer to an array of form items.
    pub(crate) form_items_ptr: *const CFormItem,
    /// Length of the form items array.
    pub(crate) form_items_len: u32,
    /// Pointer to an array of file specifications.
    pub(crate) file_specs_ptr: *const CFileSpec,
    /// Length of the file specifications array.
    pub(crate) file_specs_len: u32,
    /// Human-readable title of the task.
    pub(crate) title: CStringWrapper,
    /// Human-readable description of the task.
    pub(crate) description: CStringWrapper,
    /// MIME type of the task content.
    pub(crate) mime_type: CStringWrapper,
    /// Progress information for the task.
    pub(crate) progress: CProgress,
    /// Common task information and metadata.
    pub(crate) common_data: CommonTaskInfo,
    /// Maximum speed achieved by the task (bytes per second).
    pub(crate) max_speed: i64,
    /// Total time elapsed for the task (milliseconds).
    pub(crate) task_time: u64,
}

impl TaskInfo {
    /// Converts a Rust TaskInfo struct to its C-compatible representation.
    ///
    /// # Arguments
    ///
    /// * `info` - InfoSet containing additional task information needed for conversion.
    ///
    /// # Returns
    ///
    /// Returns a `CTaskInfo` struct ready to be passed across the FFI boundary.
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
            common_data: self.common_data,
            max_speed: self.max_speed,
            task_time: self.task_time,
        }
    }

    /// Converts a C-compatible TaskInfo struct to a Rust TaskInfo struct.
    ///
    /// # Arguments
    ///
    /// * `c_struct` - The CTaskInfo struct to convert from.
    ///
    /// # Returns
    ///
    /// Returns a Rust `TaskInfo` struct with data parsed from the C representation.
    ///
    /// # Notes
    ///
    /// Handles special logic for API9 compatibility and for tasks that are not yet completed or failed.
    /// In these cases, the MIME type is preserved; otherwise, it's set to an empty string.
    pub(crate) fn from_c_struct(c_struct: &CTaskInfo) -> Self {
        let progress = Progress::from_c_struct(&c_struct.progress);
        let extras = progress.extras.clone();

        // Removes this logic if api9 and api10 matched.
        let mime_type = if c_struct.common_data.version == Version::API9 as u8
            || (c_struct.progress.common_data.state != State::Completed.repr
                && c_struct.progress.common_data.state != State::Failed.repr)
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
            common_data: c_struct.common_data,
            max_speed: c_struct.max_speed,
            task_time: c_struct.task_time,
        };

        #[cfg(feature = "oh")]
        {
            unsafe { DeleteCFormItem(c_struct.form_items_ptr) };
            unsafe { DeleteCFileSpec(c_struct.file_specs_ptr) };
        }
        task_info
    }
}

/// C-compatible representation of task update information.
///
/// This struct provides a way to pass task update details between Rust and C code,
/// including modification time, update reason, retry attempts, MIME type, and progress.
#[repr(C)]
pub(crate) struct CUpdateInfo {
    /// Last modification timestamp (milliseconds since epoch).
    pub(crate) mtime: u64,
    /// Reason code for the update.
    pub(crate) reason: u8,
    /// Number of retry attempts made.
    pub(crate) tries: u32,
    /// MIME type of the updated content.
    pub(crate) mime_type: CStringWrapper,
    /// Current progress information for the task.
    pub(crate) progress: CProgress,
}

impl UpdateInfo {
    /// Converts a Rust UpdateInfo struct to its C-compatible representation.
    ///
    /// # Arguments
    ///
    /// * `sizes` - Comma-separated string of file sizes.
    /// * `processed` - Comma-separated string of processed bytes.
    /// * `extras` - JSON string with additional progress information.
    ///
    /// # Returns
    ///
    /// Returns a `CUpdateInfo` struct ready to be passed across the FFI boundary.
    pub(crate) fn to_c_struct(&self, sizes: &str, processed: &str, extras: &str) -> CUpdateInfo {
        CUpdateInfo {
            mtime: self.mtime,
            reason: self.reason,
            tries: self.tries,
            mime_type: CStringWrapper::from(self.mime_type.as_str()),
            progress: self.progress.to_c_struct(sizes, processed, extras),
        }
    }
}

impl TaskConfig {
    /// Converts a Rust TaskConfig struct to its C-compatible representation.
    ///
    /// # Arguments
    ///
    /// * `task_id` - Unique identifier for the task.
    /// * `uid` - User ID associated with the task.
    /// * `set` - Additional configuration data needed for conversion.
    ///
    /// # Returns
    ///
    /// Returns a `CTaskConfig` struct ready to be passed across the FFI boundary.
    pub(crate) fn to_c_struct(&self, task_id: u32, uid: u64, set: &ConfigSet) -> CTaskConfig {
        CTaskConfig {
            // Basic task identifiers and metadata
            bundle: CStringWrapper::from(&self.bundle),
            bundle_type: self.bundle_type as u8, // Convert u32 to u8 for C compatibility
            atomic_account: CStringWrapper::from(&self.atomic_account),
            url: CStringWrapper::from(&self.url),
            title: CStringWrapper::from(&self.title),
            description: CStringWrapper::from(&self.description),
            
            // Request configuration
            method: CStringWrapper::from(&self.method),
            headers: CStringWrapper::from(&set.headers), // Headers from ConfigSet
            data: CStringWrapper::from(&self.data),
            token: CStringWrapper::from(&self.token),
            extras: CStringWrapper::from(&set.extras), // Extras from ConfigSet
            proxy: CStringWrapper::from(&self.proxy),
            certificate_pins: CStringWrapper::from(&self.certificate_pins),

            // Version information
            version: self.version as u8, // Convert Version enum to u8

            // Array pointers and lengths for dynamic data
            form_items_ptr: set.form_items.as_ptr(), // Pointer to form items array
            form_items_len: set.form_items.len() as u32, // Length of form items array
            file_specs_ptr: set.file_specs.as_ptr(), // Pointer to file specifications array
            file_specs_len: set.file_specs.len() as u32, // Length of file specifications array
            body_file_names_ptr: set.body_file_names.as_ptr(), // Pointer to body file names array
            body_file_names_len: set.body_file_names.len() as u32, // Length of body file names array
            certs_path_ptr: set.certs_path.as_ptr(), // Pointer to certificate paths array
            certs_path_len: set.certs_path.len() as u32, // Length of certificate paths array

            // Common task configuration data
            common_data: CommonCTaskConfig {
                // Task identification
                task_id,
                uid,
                token_id: self.common_data.token_id,

                // Action and mode identifiers
                action: self.common_data.action.repr, // Raw representation of Action enum
                mode: self.common_data.mode.repr, // Raw representation of Mode enum

                // Task behavior flags
                cover: self.common_data.cover,
                network: self.common_data.network_config as u8, // Convert NetworkConfig to u8
                metered: self.common_data.metered,
                roaming: self.common_data.roaming,
                retry: self.common_data.retry,
                redirect: self.common_data.redirect,

                // Task scheduling and metadata
                index: self.common_data.index,
                begins: self.common_data.begins,
                ends: self.common_data.ends,

                // Progress tracking
                gauge: self.common_data.gauge,
                precise: self.common_data.precise,

                // Task execution parameters
                priority: self.common_data.priority,
                background: self.common_data.background,
                multipart: self.common_data.multipart,

                // Speed and timeout configurations
                min_speed: CMinSpeed {
                    speed: self.common_data.min_speed.speed,
                    duration: self.common_data.min_speed.duration,
                },
                timeout: CTimeout {
                    connection_timeout: self.common_data.timeout.connection_timeout,
                    total_timeout: self.common_data.timeout.total_timeout,
                },
            },
        }
    }

    /// Converts a C-compatible TaskConfig struct to a Rust TaskConfig struct.
    ///
    /// # Arguments
    ///
    /// * `c_struct` - The CTaskConfig struct to convert from.
    ///
    /// # Returns
    ///
    /// Returns a Rust `TaskConfig` struct with data parsed from the C representation.
    ///
    /// # Safety
    ///
    /// Under the `oh` feature flag, this function calls C functions to deallocate
    /// pointers. Callers must ensure that pointers are only passed once to prevent
    /// double-free errors.
    pub(crate) fn from_c_struct(c_struct: &CTaskConfig) -> Self {
        let task_config: TaskConfig = TaskConfig {
            // Basic task identifiers and metadata
            bundle: c_struct.bundle.to_string(),
            bundle_type: c_struct.bundle_type as u32, // Convert u8 back to u32
            atomic_account: c_struct.atomic_account.to_string(),
            url: c_struct.url.to_string(),
            title: c_struct.title.to_string(),
            description: c_struct.description.to_string(),

            // Request configuration
            method: c_struct.method.to_string(),
            // Parse headers from JSON string into HashMap
            headers: string_to_hashmap(&mut c_struct.headers.to_string()),
            data: c_struct.data.to_string(),
            token: c_struct.token.to_string(),
            // Parse extras from JSON string into HashMap
            extras: string_to_hashmap(&mut c_struct.extras.to_string()),
            proxy: c_struct.proxy.to_string(),
            certificate_pins: c_struct.certificate_pins.to_string(),

            // Version information - convert u8 back to Version enum
            version: Version::from(c_struct.version),

            // Convert C arrays to Rust vectors using build_vec helper function
            form_items: build_vec(
                c_struct.form_items_ptr,
                c_struct.form_items_len as usize,
                FormItem::from_c_struct, // Conversion function for each element
            ),
            file_specs: build_vec(
                c_struct.file_specs_ptr,
                c_struct.file_specs_len as usize,
                FileSpec::from_c_struct, // Conversion function for each element
            ),
            body_file_paths: build_vec(
                c_struct.body_file_names_ptr,
                c_struct.body_file_names_len as usize,
                CStringWrapper::to_string, // Conversion function for each element
            ),
            certs_path: build_vec(
                c_struct.certs_path_ptr,
                c_struct.certs_path_len as usize,
                CStringWrapper::to_string, // Conversion function for each element
            ),

            // Common task configuration data
            common_data: CommonTaskConfig {
                // Task identification
                task_id: c_struct.common_data.task_id,
                uid: c_struct.common_data.uid,
                token_id: c_struct.common_data.token_id,

                // Convert raw representations to proper enums
                action: Action::from(c_struct.common_data.action),
                mode: Mode::from(c_struct.common_data.mode),

                // Task behavior flags
                cover: c_struct.common_data.cover,
                network_config: NetworkConfig::from(c_struct.common_data.network),
                metered: c_struct.common_data.metered,
                roaming: c_struct.common_data.roaming,
                retry: c_struct.common_data.retry,
                redirect: c_struct.common_data.redirect,

                // Task scheduling and metadata
                index: c_struct.common_data.index,
                begins: c_struct.common_data.begins,
                ends: c_struct.common_data.ends,

                // Progress tracking
                gauge: c_struct.common_data.gauge,
                precise: c_struct.common_data.precise,

                // Task execution parameters
                priority: c_struct.common_data.priority,
                background: c_struct.common_data.background,
                multipart: c_struct.common_data.multipart,

                // Convert C-specific structs to Rust structs
                min_speed: MinSpeed {
                    speed: c_struct.common_data.min_speed.speed,
                    duration: c_struct.common_data.min_speed.duration,
                },
                timeout: Timeout {
                    connection_timeout: c_struct.common_data.timeout.connection_timeout,
                    total_timeout: c_struct.common_data.timeout.total_timeout,
                },
            },
        };

        // Under OH feature flag, free C-allocated memory to prevent memory leaks
        #[cfg(feature = "oh")]
        {
            // Deallocate form items array
            unsafe { DeleteCFormItem(c_struct.form_items_ptr) };
            // Deallocate file specifications array
            unsafe { DeleteCFileSpec(c_struct.file_specs_ptr) };
            // Deallocate body file names array
            unsafe { DeleteCStringPtr(c_struct.body_file_names_ptr) };
            // Deallocate certificate paths array
            unsafe { DeleteCStringPtr(c_struct.certs_path_ptr) };
        }

        task_config
    }
}
