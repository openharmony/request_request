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

use std::fs::{File, OpenOptions};
use std::io;
use std::os::fd::FromRawFd;
use std::sync::{Arc, Mutex};

use crate::error::{ErrorCode, ServiceError};
use crate::manage::account;
use crate::task::bundle::get_name_and_index;
use crate::task::config::{Action, TaskConfig};
use crate::task::ATOMIC_SERVICE;

/// Container for all files associated with a network task.
/// 
/// Manages the main task files (upload/download targets) and their sizes,
/// as well as any additional body files used for complex requests.
pub(crate) struct AttachedFiles {
    /// Main files for the task (upload sources or download destinations).
    pub(crate) files: Files,
    /// Sizes of the main files in bytes (negative values indicate unknown size).
    pub(crate) sizes: Vec<i64>,
    /// Additional body files for complex request scenarios.
    pub(crate) body_files: Files,
}

impl AttachedFiles {
    /// Opens all files specified in the task configuration.
    /// 
    /// Creates a new `AttachedFiles` instance by opening both the main task files
    /// and any additional body files according to the provided configuration.
    /// 
    /// # Errors
    /// Returns a `ServiceError` if any file fails to open.
    pub(crate) fn open(config: &TaskConfig) -> Result<AttachedFiles, ServiceError> {
        let (files, sizes) = open_task_files(config)?;
        let body_files = open_body_files(config)?;
        Ok(Self {
            files,
            sizes,
            body_files,
        })
    }
}

/// Opens the main task files based on the provided configuration.
/// 
/// Handles both upload and download scenarios, opening files in appropriate modes
/// and collecting their sizes where applicable.
/// 
/// # Errors
/// Returns a `ServiceError` if file opening or metadata retrieval fails.
fn open_task_files(config: &TaskConfig) -> Result<(Files, Vec<i64>), ServiceError> {
    let tid = config.common_data.task_id;
    let uid = config.common_data.uid;

    let mut files = Vec::new();
    let mut sizes = Vec::new();
    // Cache bundle name to avoid redundant calculations for multiple files
    let mut bundle_cache = BundleCache::new(config);

    for (idx, fs) in config.file_specs.iter().enumerate() {
        match config.common_data.action {
            Action::Upload => {
                let file = if fs.is_user_file {
                    // For user-provided files, use the file descriptor directly
                    match fs.fd {
                        Some(fd) => unsafe { File::from_raw_fd(fd) },
                        None => {
                            error!("None user file failed - task_id: {}, idx: {}", tid, idx);
                            sys_event!(
                                ExecFault,
                                DfxCode::SA_ERROR_01,
                                &format!("None user file failed - task_id: {}, idx: {}", tid, idx)
                            );
                            return Err(ServiceError::IoError(io::Error::new(
                                io::ErrorKind::Other,
                                "none user file",
                            )));
                        }
                    }
                } else {
                    // For non-user files, open from the app's storage
                    let bundle_name = bundle_cache.get_value()?;
                    open_file_readonly(uid, &bundle_name, &fs.path)
                        .map_err(ServiceError::IoError)?
                };
                // Get file size for upload progress tracking
                let size = cvt_res_error!(
                    file.metadata()
                        .map(|data| data.len())
                        .map_err(ServiceError::IoError),
                    "Cannot get upload file's size - task_id: {}, idx: {}",
                    tid, idx
                );
                // Use Arc<Mutex<File>> to ensure thread-safe access
                files.push(Arc::new(Mutex::new(file)));
                debug!(
                    "Get file size succeed - task_id: {}, idx: {}, size: {}",
                    tid, idx, size
                );
                sizes.push(size as i64);
            }
            Action::Download => {
                let file = if fs.is_user_file {
                    // For user-provided files, use the file descriptor directly
                    match fs.fd {
                        Some(fd) => unsafe { File::from_raw_fd(fd) },
                        None => {
                            error!("None user file failed - task_id: {}, idx: {}", tid, idx);
                            sys_event!(
                                ExecFault,
                                DfxCode::SA_ERROR_01,
                                &format!("None user file failed - task_id: {}, idx: {}", tid, idx)
                            );
                            return Err(ServiceError::IoError(io::Error::new(
                                io::ErrorKind::Other,
                                "none user file",
                            )));
                        }
                    }
                } else {
                    // For non-user files, open from the app's storage in read-write mode
                    let bundle_name = bundle_cache.get_value()?;
                    open_file_readwrite(uid, &bundle_name, &fs.path)
                        .map_err(ServiceError::IoError)?
                };
                // Use Arc<Mutex<File>> to ensure thread-safe access
                files.push(Arc::new(Mutex::new(file)));
                // Set size to -1 for downloads (unknown size initially)
                sizes.push(-1)
            }
            _ => unreachable!("Action::Any in open_task_files should never reach"),
        }
    }
    Ok((Files::new(files), sizes))
}

/// Opens additional body files specified in the task configuration.
/// 
/// These files are typically used for complex request scenarios requiring
/// additional data beyond the main task files.
/// 
/// # Errors
/// Returns a `ServiceError` if any body file fails to open.
fn open_body_files(config: &TaskConfig) -> Result<Files, ServiceError> {
    let tid = config.common_data.task_id;
    let uid = config.common_data.uid;
    let mut bundle_cache = BundleCache::new(config);
    let mut body_files = Vec::new();
    
    for (idx, path) in config.body_file_paths.iter().enumerate() {
        let bundle_name = bundle_cache.get_value()?;
        let file = open_file_readwrite(uid, &bundle_name, path).map_err(|e| {
            error!("Open body_file failed - task_id: {}, idx: {}", tid, idx);
            sys_event!(
                ExecFault,
                DfxCode::SA_ERROR_02,
                &format!("Open body_file failed - task_id: {}, idx: {}", tid, idx)
            );
            ServiceError::IoError(e)
        })?;
        body_files.push(Arc::new(Mutex::new(file)))
    }
    
    Ok(Files::new(body_files))
}

/// Opens a file in read-write mode at the specified path.
/// 
/// Converts the provided path using the UID and bundle name, then opens the file
/// with read and append permissions.
/// 
/// # Errors
/// Returns an `io::Error` if the file cannot be opened.
fn open_file_readwrite(uid: u64, bundle_name: &str, path: &str) -> io::Result<File> {
    Ok(cvt_res_error!(
        OpenOptions::new()
            .read(true)
            .append(true)
            .open(convert_path(uid, bundle_name, path)),
        "open_file_readwrite failed"
    ))
}

/// Opens a file in read-only mode at the specified path.
/// 
/// Converts the provided path using the UID and bundle name, then opens the file
/// with read-only permissions.
/// 
/// # Errors
/// Returns an `io::Error` if the file cannot be opened.
fn open_file_readonly(uid: u64, bundle_name: &str, path: &str) -> io::Result<File> {
    Ok(cvt_res_error!(
        OpenOptions::new()
            .read(true)
            .open(convert_path(uid, bundle_name, path)),
        "open_file_readonly failed"
    ))
}

/// Converts a relative path to an absolute path based on the user and bundle.
/// 
/// Transforms paths by replacing "storage" with "app" and "base" with a path
/// containing the user's UUID and bundle name.
/// 
/// # Examples
/// ```
/// # use crate::task::files::convert_path;
/// let path = convert_path(200000, "example.bundle", "storage/base/file.txt");
/// assert!(path.contains("app/1/base/example.bundle/file.txt"));
/// ```
pub(crate) fn convert_path(uid: u64, bundle_name: &str, path: &str) -> String {
    let uuid = get_uuid_from_uid(uid);
    let base_replace = format!("{}/base/{}", uuid, bundle_name);
    path.replacen("storage", "app", 1)
        .replacen("base", &base_replace, 1)
}

/// Extracts the UUID from a UID by dividing by 200000.
/// 
/// This is a standard way to map user identifiers to unique account identifiers.
fn get_uuid_from_uid(uid: u64) -> u64 {
    uid / 200000
}

/// Checks if a task's account matches the current foreground account.
/// 
/// Returns `true` if the task's account matches the foreground account or if
/// the task's account is 0 (indicating it can run under any account).
/// 
/// # Examples
/// ```
/// # use crate::task::files::check_current_account;
/// // Assuming foreground account is 1
/// assert!(check_current_account(200000)); // task_account is 1
/// assert!(check_current_account(0));     // account 0 can run under any account
/// assert!(!check_current_account(400000)); // task_account is 2
/// ```
pub(crate) fn check_current_account(task_uid: u64) -> bool {
    let task_account = get_uuid_from_uid(task_uid);
    let (foreground_account, _active_accounts) = account::query_active_accounts();
    // 0 account_id tasks can run under other account_ids
    let b = (task_account == 0) || (task_account == foreground_account);
    if !b {
        info!(
            "check_current_account: {}, {}",
            foreground_account, task_account
        );
    }
    b
}

/// Caches bundle name resolution for a task configuration.
/// 
/// This struct avoids redundant bundle name conversions by caching the result
/// of the first conversion for subsequent use.
pub(crate) struct BundleCache<'a> {
    /// Reference to the task configuration used for bundle name resolution.
    config: &'a TaskConfig,
    /// Cached result of the bundle name conversion.
    value: Option<Result<String, ServiceError>>,
}

impl<'a> BundleCache<'a> {
    /// Creates a new bundle cache for the given task configuration.
    /// 
    /// Initializes the cache with no precomputed value.
    pub(crate) fn new(config: &'a TaskConfig) -> Self {
        Self {
            config,
            value: None,
        }
    }

    /// Gets the resolved bundle name, using the cached value if available.
    /// 
    /// If the bundle name has already been resolved, returns the cached value.
    /// Otherwise, resolves the bundle name and caches the result.
    /// 
    /// # Errors
    /// Returns a `ServiceError` if the bundle name cannot be resolved.
    pub(crate) fn get_value(&mut self) -> Result<String, ServiceError> {
        let ret = match &self.value {
            // Return cached success result
            Some(ret) => match ret {
                Ok(name) => Ok(name.to_owned()),
                // For cached errors, retry the conversion
                Err(_e) => convert_bundle_name(self.config),
            },
            // If no cache, perform the conversion
            None => convert_bundle_name(self.config),
        };
        
        // Update cache with the new result
        self.value = Some(ret.clone());
        ret
    }
}

/// Converts a bundle name based on the task configuration.
/// 
/// Handles both atomic service bundles and regular application bundles,
/// applying appropriate formatting for each type.
/// 
/// # Errors
/// Returns a `ServiceError` if the bundle name cannot be converted.
fn convert_bundle_name(config: &TaskConfig) -> Result<String, ServiceError> {
    let is_account = config.bundle_type == ATOMIC_SERVICE;
    let bundle_name = config.bundle.as_str();
    
    if is_account {
        // Format for atomic service bundles
        let atomic_account = config.atomic_account.as_str();
        Ok(format!("+auid-{}+{}", atomic_account, bundle_name))
    } else {
        // Handle regular application bundles with possible clone indices
        let uid = config.common_data.uid;
        check_app_clone_bundle_name(uid, bundle_name)
    }
}

/// Checks for app clone bundle names and applies appropriate formatting.
/// 
/// If the app is a clone (indicated by an index > 0), formats the bundle name
/// to include the clone index.
/// 
/// # Errors
/// Returns a `ServiceError` if the bundle name and index cannot be retrieved.
fn check_app_clone_bundle_name(uid: u64, bundle_name: &str) -> Result<String, ServiceError> {
    let mut ret_name = bundle_name.to_string();
    
    if let Some((index, name)) = get_name_and_index(uid as i32) {
        // Log mismatch between provided and retrieved bundle names
        if bundle_name != name {
            info!("bundle name not matching. {:?}, {:?}", bundle_name, name);
        }
        
        // For clone apps, append the clone index to the bundle name
        if index > 0 {
            ret_name = format!("+clone-{}+{}", index, bundle_name);
        }
        
        return Ok(ret_name);
    }
    
    info!("can not get bundle name and index.");
    Err(ServiceError::ErrorCode(ErrorCode::Other))
}

/// Thread-safe collection of file handles.
/// 
/// Provides a safe interface to access multiple files concurrently,
/// using `Arc<Mutex<File>>` to ensure thread-safe file operations.
pub(crate) struct Files(Vec<Arc<Mutex<File>>>);

impl Files {
    /// Creates a new file collection from a vector of file handles.
    fn new(files: Vec<Arc<Mutex<File>>>) -> Self {
        Self(files)
    }

    /// Returns the number of files in the collection.
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    /// Gets a file handle at the specified index, if it exists.
    /// 
    /// Returns a clone of the `Arc<Mutex<File>>` if the index is valid,
    /// allowing thread-safe access to the file.
    pub(crate) fn get(&self, index: usize) -> Option<Arc<Mutex<File>>> {
        self.0.get(index).cloned()
    }
}
