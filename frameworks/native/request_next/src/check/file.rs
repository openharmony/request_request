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

//! File path validation and permission management for downloads.
//!
//! This module provides functions for validating download paths, converting between
//! different path formats, and setting appropriate file permissions for downloaded
//! content across different API versions.

// Standard library imports
use std::error::Error;
use std::fmt::Display;
use std::fs::{self, OpenOptions};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

// External dependencies
use request_core::config::Version;
use request_utils::context::Context;
use request_utils::storage;

// Constants for path validation
const MAX_FILE_PATH_LENGTH: usize = 4096; // Maximum allowed path length in bytes

// Path prefixes for validation and conversion
const ABSOLUTE_PREFIX: &str = "/";
const RELATIVE_PREFIX: &str = "./";
const FILE_PREFIX: &str = "file://";
const INTERNAL_PREFIX: &str = "internal://";

// Valid storage areas for API 10
const AREA1: &str = "/data/storage/el1/base";
const AREA2: &str = "/data/storage/el2/base";
const AREA5: &str = "/data/storage/el5/base";

// ACL permission strings for service account access
const SA_PERMISSION_RWX: &str = "g:3815:rwx"; // Read, write, execute permissions
const SA_PERMISSION_X: &str = "g:3815:x";     // Execute-only permissions
const SA_PERMISSION_CLEAN: &str = "g:3815:---"; // No permissions

/// Gets a validated download path with appropriate permissions.
///
/// Validates and converts the provided path string based on the API version,
/// checking for existing files if overwrite is disabled, and sets appropriate
/// permissions on the file and parent directories.
///
/// # Parameters
/// - `version`: API version to determine path handling logic
/// - `context`: Application context for accessing storage directories
/// - `save_as`: Path string to validate and convert
/// - `overwrite`: Whether to allow overwriting existing files
///
/// # Returns
/// A valid `PathBuf` for the download if successful, or a `DownloadPathError` if validation fails
///
/// # Errors
/// - `DownloadPathError::EmptyPath`: If the path is empty
/// - `DownloadPathError::TooLongPath`: If the path exceeds the maximum length
/// - `DownloadPathError::InvalidPath`: If the path is not in an allowed storage area
/// - `DownloadPathError::AlreadyExists`: If the file already exists and overwrite is false
/// - `DownloadPathError::CreateFile`: If file creation fails
/// - `DownloadPathError::SetPermission`: If setting file permissions fails
/// - `DownloadPathError::AclAccess`: If setting ACL permissions fails
pub fn get_download_path(
    version: Version,
    context: &Context,
    saveas: &str,
    overwrite: bool,
) -> Result<PathBuf, DownloadPathError> {
    // Convert path according to API version rules
    let path = convert_path(version, context, saveas)?;

    // Check for existing file if overwrite is disabled
    if !overwrite && path.exists() {
        return Err(DownloadPathError::AlreadyExists);
    }

    // Set appropriate file permissions
    set_file_permission(&path, &context)?;

    Ok(path)
}

/// Converts a path string to a `PathBuf` based on API version rules.
///
/// Handles different path conversion logic for API 9 and API 10, validating
/// paths against appropriate constraints for each version.
///
/// # Parameters
/// - `version`: API version determining conversion rules
/// - `context`: Application context for accessing directories
/// - `save_as`: Path string to convert
///
/// # Returns
/// A `PathBuf` representing the converted path, or a `DownloadPathError` on validation failure
pub fn convert_path(
    version: Version,
    context: &Context,
    saveas: &str,
) -> Result<PathBuf, DownloadPathError> {
    match version {
        Version::API9 => {
            // Handle absolute paths directly for API 9
            if let Some(0) = saveas.find(ABSOLUTE_PREFIX) {
                if saveas.len() == ABSOLUTE_PREFIX.len() {
                    return Err(DownloadPathError::EmptyPath);
                }
                return Ok(PathBuf::from(saveas));
            } else {
                // Handle internal cache paths for API 9
                const INTERNAL_PATTERN: &str = "internal://cache/";
                let file_name = match saveas.find(INTERNAL_PATTERN) {
                    Some(0) => saveas.split_at(INTERNAL_PATTERN.len()).1,
                    _ => saveas,
                };
                if file_name.is_empty() {
                    return Err(DownloadPathError::EmptyPath);
                }
                let cache_dir = context.get_cache_dir();

                // Validate path length to prevent buffer overflows
                if cache_dir.len() + file_name.len() + 1 > MAX_FILE_PATH_LENGTH {
                    return Err(DownloadPathError::TooLongPath);
                }
                Ok(PathBuf::from(cache_dir).join(file_name))
            }
        }

        Version::API10 => {
            // Convert to absolute path using API 10 rules
            let absolute_path = convert_to_absolute_path(&context, saveas)?;

            // Validate path is within allowed storage areas for API 10
            if !absolute_path.starts_with(AREA1)
                && !absolute_path.starts_with(AREA2)
                && !absolute_path.starts_with(AREA5)
            {
                return Err(DownloadPathError::InvalidPath);
            }
            Ok(absolute_path)
        }
    }
}

/// Converts various path formats to absolute paths for API 10.
///
/// Handles absolute, file://, internal://, and relative paths, converting them
/// to appropriate absolute paths with validation.
///
/// # Parameters
/// - `context`: Application context for resolving relative paths
/// - `path`: Path string in various formats
///
/// # Returns
/// An absolute `PathBuf`, or a `DownloadPathError` on validation failure
fn convert_to_absolute_path(context: &Context, path: &str) -> Result<PathBuf, DownloadPathError> {
    // Handle absolute paths
    if let Some(0) = path.find(ABSOLUTE_PREFIX) {
        if path.len() == ABSOLUTE_PREFIX.len() {
            return Err(DownloadPathError::EmptyPath);
        }
        return Ok(PathBuf::from(path));
    }

    // Handle file:// scheme with bundle name validation
    if let Some(0) = path.find(FILE_PREFIX) {
        let path = path.split_at(FILE_PREFIX.len()).1;
        if path.is_empty() {
            return Err(DownloadPathError::EmptyPath);
        }
        // Validate path has bundle name and path parts
        let Some(index) = path.find('/') else {
            return Err(DownloadPathError::InvalidPath);
        };
        let (bundle_name, path) = path.split_at(index);
        // Ensure bundle name matches the application
        if bundle_name != context.get_bundle_name() {
            return Err(DownloadPathError::BundleNameNotMap);
        }
        return Ok(PathBuf::from(path));
    }

    // Handle internal:// paths
    if let Some(0) = path.find(INTERNAL_PREFIX) {
        let path = path.split_at(INTERNAL_PREFIX.len()).1;
        if path.is_empty() {
            return Err(DownloadPathError::EmptyPath);
        }
        let cache_dir = context.get_cache_dir();
        return Ok(PathBuf::from(cache_dir).join(path));
    }

    // Handle relative paths
    let path = if let Some(0) = path.find(RELATIVE_PREFIX) {
        path.split_at(RELATIVE_PREFIX.len()).1
    } else {
        path
    };

    if path.is_empty() {
        return Err(DownloadPathError::EmptyPath);
    }
    let cache_dir = context.get_cache_dir();

    Ok(PathBuf::from(cache_dir).join(path))
}

/// Sets appropriate permissions on a file and its parent directories.
///
/// Creates the file if it doesn't exist, sets standard permissions, and configures
/// ACL permissions for the service account to access the file and its parent directories.
///
/// # Parameters
/// - `path`: Path to the file to configure permissions for
/// - `context`: Application context
///
/// # Returns
/// `Ok(())` if permissions are successfully set, or a `DownloadPathError` on failure
///
/// # Errors
/// - `DownloadPathError::CreateFile`: If file creation fails
/// - `DownloadPathError::SetPermission`: If setting standard file permissions fails
/// - `DownloadPathError::AclAccess`: If setting ACL permissions fails
pub fn set_file_permission(path: &PathBuf, context: &Context) -> Result<(), DownloadPathError> {
    // Create the file if it doesn't exist
    let _ = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&path)
        .map_err(|e| DownloadPathError::CreateFile(e))?;

    // Set read/write/execute permissions for all users
    let perm = fs::Permissions::from_mode(0o777);
    if let Err(e) = fs::set_permissions(&path, perm) {
        return Err(DownloadPathError::SetPermission(e));
    }

    // Log the base directory for debugging
    let base_dir = context.get_base_dir();
    info!("Base directory: {:?}", base_dir);

    // Set execute permissions on parent directories for traversal
    let mut path_clone = path.clone();

    // Process parent directories up to a reasonable limit
    while path_clone.to_string_lossy().to_string().len() >= 10 {
        info!("Current path: {:?}", path_clone);
        if let Err(e) =
            storage::acl_set_access(&path_clone.to_string_lossy().to_string(), SA_PERMISSION_X)
        {
            info!("");
        }
        path_clone.pop();
    }

    // Set read/write/execute permissions on the file for the service account
    info!("Setting ACL access for path: {:?}", path);
    if let Err(e) = storage::acl_set_access(&path.to_string_lossy().to_string(), SA_PERMISSION_RWX)
    {
        return Err(DownloadPathError::AclAccess(e));
    }

    Ok(())
}

/// Error types for download path validation and permission operations.
#[derive(Debug)]
pub enum DownloadPathError {
    /// The provided path is empty
    EmptyPath,
    /// The path exceeds the maximum allowed length
    TooLongPath,
    /// The path is not in an allowed storage area
    InvalidPath,
    /// The bundle name in the path doesn't match the application
    BundleNameNotMap,
    /// The file already exists and overwrite is disabled
    AlreadyExists,
    /// File creation failed with an IO error
    CreateFile(std::io::Error),
    /// Setting file permissions failed with an IO error
    SetPermission(std::io::Error),
    /// Setting ACL access permissions failed
    AclAccess(i32),
}

impl Error for DownloadPathError {}

impl Display for DownloadPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self))
    }
}
