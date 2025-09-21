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

use std::error::Error;
use std::fmt::Display;
use std::fs::{self, OpenOptions};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use request_core::config::Version;
use request_utils::context::Context;
use request_utils::storage;

const MAX_FILE_PATH_LENGTH: usize = 4096;

const ABSOLUTE_PREFIX: &str = "/";
const RELATIVE_PREFIX: &str = "./";
const FILE_PREFIX: &str = "file://";
const INTERNAL_PREFIX: &str = "internal://";

const AREA1: &str = "/data/storage/el1/base";
const AREA2: &str = "/data/storage/el2/base";
const AREA5: &str = "/data/storage/el5/base";

const SA_PERMISSION_RWX: &str = "g:3815:rwx";
const SA_PERMISSION_X: &str = "g:3815:x";
const SA_PERMISSION_CLEAN: &str = "g:3815:---";

pub fn get_download_path(
    version: Version,
    context: &Context,
    save_as: &str,
    overwrite: bool,
) -> Result<PathBuf, DownloadPathError> {
    let path = convert_path(version, context, save_as)?;
    if !overwrite && path.exists() {
        return Err(DownloadPathError::AlreadyExists);
    }

    set_file_permission(&path, &context)?;

    Ok(path)
}

pub fn convert_path(
    version: Version,
    context: &Context,
    save_as: &str,
) -> Result<PathBuf, DownloadPathError> {
    match version {
        Version::API9 => {
            if let Some(0) = save_as.find(ABSOLUTE_PREFIX) {
                if save_as.len() == ABSOLUTE_PREFIX.len() {
                    return Err(DownloadPathError::EmptyPath);
                }
                return Ok(PathBuf::from(save_as));
            } else {
                const INTERNAL_PATTERN: &str = "internal://cache/";
                let file_name = match save_as.find(INTERNAL_PATTERN) {
                    Some(0) => save_as.split_at(INTERNAL_PATTERN.len()).1,
                    _ => save_as,
                };
                if file_name.is_empty() {
                    return Err(DownloadPathError::EmptyPath);
                }
                let cache_dir = context.get_cache_dir();

                if cache_dir.len() + file_name.len() + 1 > MAX_FILE_PATH_LENGTH {
                    return Err(DownloadPathError::TooLongPath);
                }
                Ok(PathBuf::from(cache_dir).join(file_name))
            }
        }

        Version::API10 => {
            let absolute_path = convert_to_absolute_path(&context, save_as)?;
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

fn convert_to_absolute_path(context: &Context, path: &str) -> Result<PathBuf, DownloadPathError> {
    if let Some(0) = path.find(ABSOLUTE_PREFIX) {
        if path.len() == ABSOLUTE_PREFIX.len() {
            return Err(DownloadPathError::EmptyPath);
        }
        return Ok(PathBuf::from(path));
    }

    if let Some(0) = path.find(FILE_PREFIX) {
        let path = path.split_at(FILE_PREFIX.len()).1;
        if path.is_empty() {
            return Err(DownloadPathError::EmptyPath);
        }
        let Some(index) = path.find('/') else {
            return Err(DownloadPathError::InvalidPath);
        };
        let (bundle_name, path) = path.split_at(index);
        if bundle_name != context.get_bundle_name() {
            return Err(DownloadPathError::BundleNameNotMap);
        }
        return Ok(PathBuf::from(path));
    }

    if let Some(0) = path.find(INTERNAL_PREFIX) {
        let path = path.split_at(INTERNAL_PREFIX.len()).1;
        if path.is_empty() {
            return Err(DownloadPathError::EmptyPath);
        }
        let cache_dir = context.get_cache_dir();
        return Ok(PathBuf::from(cache_dir).join(path));
    }

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

pub fn set_file_permission(path: &PathBuf, context: &Context) -> Result<(), DownloadPathError> {
    let _ = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&path)
        .map_err(|e| DownloadPathError::CreateFile(e))?;

    let perm = fs::Permissions::from_mode(0o777);
    if let Err(e) = fs::set_permissions(&path, perm) {
        return Err(DownloadPathError::SetPermission(e));
    }

    let base_dir = context.get_base_dir();
    info!("Base directory: {:?}", base_dir);

    let mut path_clone = path.clone();

    while path_clone.to_string_lossy().to_string().len() >= 10 {
        info!("Current path: {:?}", path_clone);
        if let Err(e) =
            storage::acl_set_access(&path_clone.to_string_lossy().to_string(), SA_PERMISSION_X)
        {
            info!("");
        }
        path_clone.pop();
    }

    info!("Setting ACL access for path: {:?}", path);
    if let Err(e) = storage::acl_set_access(&path.to_string_lossy().to_string(), SA_PERMISSION_RWX)
    {
        return Err(DownloadPathError::AclAccess(e));
    }

    Ok(())
}

#[derive(Debug)]
pub enum DownloadPathError {
    EmptyPath,
    TooLongPath,
    InvalidPath,
    BundleNameNotMap,
    AlreadyExists,
    CreateFile(std::io::Error),
    SetPermission(std::io::Error),
    AclAccess(i32),
}

impl Error for DownloadPathError {}
impl Display for DownloadPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self))
    }
}
