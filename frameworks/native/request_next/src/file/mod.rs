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

//! File path resolution and permission management for tasks.
//!
//! Exposes `FileManager` and `PermissionManager` which validate task file
//! paths, resolve them to absolute locations, and grant the associated
//! access permissions.

mod permission;

use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::io::IntoRawFd;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use cxx::let_cxx_string;
/// Re-exports of the permission manager and its permission tokens.
pub use permission::{PermissionManager, PermissionToken};
use request_core::config::{Action, Mode, TaskConfig, Version};
use request_core::file::FileSpec;
use request_utils::context::Context;
use request_utils::storage;

const DOCS_PREFIX: &str = "file://docs/";
const MEDIA_PREFIX: &str = "file://media/";
const ABSOLUTE_PREFIX: &str = "/";
const INTERNAL_PATTERN: &str = "internal://cache/";
const MAX_FILE_PATH_LENGTH: usize = 4096;
const FILE_PREFIX: &str = "file://";
const INTERNAL_PREFIX: &str = "internal://";
const RELATIVE_PREFIX: &str = "./";
const AREA1: &str = "/data/storage/el1/base";
const AREA2: &str = "/data/storage/el2/base";
const AREA5: &str = "/data/storage/el5/base";
const CERTS_PATH: &str = "/data/storage/el2/base/.ohos/.request/.certs";

/// Manager that validates task file paths and grants the associated file
/// permissions.
pub struct FileManager {
    /// Underlying permission manager used to issue and track permission tokens.
    pub permission_manager: PermissionManager,
}

impl FileManager {
    /// Returns the shared singleton file manager instance.
    pub fn get_instance() -> &'static Self {
        static INSTANCE: OnceLock<FileManager> = OnceLock::new();
        INSTANCE.get_or_init(|| FileManager {
            permission_manager: PermissionManager::new(),
        })
    }

    /// Resolves and validates the file paths for a task, applying the required
    /// access permissions and populating certificate settings.
    ///
    /// For downloads it processes the save-as path; for uploads it processes the
    /// file specifications. Certificate paths and pins are filled in as needed.
    ///
    /// # Arguments
    /// * `context` - Application context used for path resolution and ACL checks.
    /// * `config` - Task configuration whose paths are resolved in place.
    ///
    /// # Returns
    /// The permission tokens granted for the resolved paths, or an error code.
    pub fn apply(
        &self,
        context: Context,
        config: &mut TaskConfig,
    ) -> Result<Vec<PermissionToken>, i32> {
        let mut tokens = if matches!(config.common_data.action, Action::Download) {
            let mut tokens = vec![];
            if let Some(token) = self.apply_download_path(config, &context)? {
                tokens.push(token);
            }
            tokens
        } else {
            self.apply_upload_path(config, &context)?
        };
        Self::get_cert_path(config);
        // test
        Self::get_certificate_pins(config);
        self.apply_cert_path(&mut config.certs_path, &mut tokens)?;
        Ok(tokens)
    }

    /// Resolves the download destination path and grants the associated
    /// permission.
    ///
    /// Parses the save-as path, distinguishes user-provided files from sandbox
    /// files, converts relative/URI paths to an absolute sandbox path, pushes
    /// the resulting `FileSpec`, sets file permissions, and issues a permission
    /// token for the resolved path.
    ///
    /// # Arguments
    /// * `config` - Task configuration whose `saveas` and `file_specs` are
    ///   updated in place.
    /// * `context` - Application context used for path resolution and ACL
    ///   checks.
    ///
    /// # Returns
    /// `Some(token)` when a permission token is granted for a sandbox file, or
    /// `None` for a user-provided file that needs no token.
    ///
    /// # Errors
    /// Returns an `i32` error code if path parsing, conversion, or permission
    /// setting fails.
    fn apply_download_path(
        &self,
        config: &mut TaskConfig,
        context: &Context,
    ) -> Result<Option<PermissionToken>, i32> {
        Self::parse_saveas(config)?;
        if Self::is_user_file(&config.saveas.clone()) {
            Self::check_download_user_file(config)?;
            return Ok(None);
        }
        let path = Self::convert_download_path(config, context)?;
        let file_name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());
        config.file_specs.push(FileSpec {
            name: "file".to_string(),
            path: path.to_string_lossy().to_string(),
            mime_type: file_name
                .rsplit_once('.')
                .map(|(_, name)| name.to_string())
                .unwrap_or_default(),
            file_name,
            // todo: check
            is_user_file: false,
            fd: None,
        });
        Self::chmod_download_file(&path, config)?;
        Ok(Some(self.permission_manager.grant(&path)?))
    }

    /// Normalizes the `saveas` field of a download task configuration.
    ///
    /// Derives the file name from the request URL when the save-as path is
    /// empty or `"./"`, and rejects paths that end with a trailing slash.
    ///
    /// # Arguments
    /// * `config` - Task configuration whose `saveas` field is normalized in
    ///   place.
    ///
    /// # Errors
    /// Returns `401` if the save-as path cannot be derived or ends with `/`.
    fn parse_saveas(config: &mut TaskConfig) -> Result<(), i32> {
        config.saveas.trim();
        if config.saveas.is_empty() || config.saveas == "./" {
            config.saveas = if let Some(path) = config
                .url
                .rsplit_once('/')
                .map(|(_, name)| name.to_string())
            {
                path
            } else {
                error!("ParseSaveas error");
                return Err(401);
            };
            return Ok(());
        }
        if config.saveas.ends_with('/') {
            error!("ParseSaveas error");
            Err(401)
        } else {
            Ok(())
        }
    }

    /// Resolves upload file paths and creates the temporary body files.
    ///
    /// Validates each file spec, grants permission tokens for sandbox files,
    /// and creates temporary multipart body files under the cache directory
    /// with `0o644` permissions.
    ///
    /// # Arguments
    /// * `config` - Task configuration whose `file_specs` and `body_file_paths`
    ///   are updated in place.
    /// * `context` - Application context used for path resolution and ACL
    ///   checks.
    ///
    /// # Returns
    /// The permission tokens granted for the resolved upload files and body
    /// files.
    ///
    /// # Errors
    /// Returns an `i32` error code if the file count exceeds the limit, a path
    /// is invalid, or a body file cannot be created.
    fn apply_upload_path(
        &self,
        config: &mut TaskConfig,
        context: &Context,
    ) -> Result<Vec<PermissionToken>, i32> {
        // SDK version must be greater than 15
        const MAX_UPLOAD_FILES: i32 = 100;
        if config.file_specs.len() as i32 > MAX_UPLOAD_FILES {
            return Err(401);
        }
        let mut tokens = Vec::new();
        for file_spec in &mut config.file_specs {
            if Self::is_user_file(&file_spec.path) {
                if config.version == Version::API9 {
                    return Err(401);
                }
                if matches!(config.common_data.mode, Mode::BackGround) {
                    return Err(401);
                }
                file_spec.is_user_file = true;
                Self::check_upload_user_file(file_spec, context)?;
            } else {
                tokens.push(self.check_upload_file(file_spec, context, &config.version)?);
            }
        }

        let len = if config.common_data.multipart {
            1
        } else {
            config.file_specs.len()
        };

        let file_path = context.get_cache_dir();
        if file_path.is_empty() {
            error!("internal to cache error");
            return Err(401); // E_PARAMETER_CHECK
        }

        for i in 0..len {
            // Generate timestamp
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos();

            let path = format!("{}/tmp_body_{}_{}", file_path, i, now);

            // Validate path validity (simplified version; IsPathValid logic needs to be
            // implemented)
            if path.contains("..") || path.contains("//") {
                error!("Upload IsPathValid error");
                return Err(401); // E_PARAMETER_CHECK
            }

            // Create file
            let body_file = match std::fs::OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .truncate(true)
                .open(&path)
            {
                Ok(file) => file,
                Err(e) => {
                    error!("UploadBodyFiles failed to open file: {}", e);
                    return Err(13400001); // E_FILE_IO
                }
            };

            if let Err(e) = fs::set_permissions(&path, fs::Permissions::from_mode(0o644)) {
                error!("body chmod fail: {}", e);
            }

            tokens.push(self.permission_manager.grant(&PathBuf::from(&path))?);
            config.body_file_paths.push(path);
        }
        Ok(tokens)
    }

    // todo: check
    /// Copies certificate files into the shared certs directory and grants
    /// permission tokens for them.
    ///
    /// Creates the shared certs directory if missing, copies each certificate
    /// file from the supplied folders into it (skipping files that already
    /// exist), sets `0o755` permissions, and issues permission tokens. The
    /// `certs_path` vector is replaced with the shared certs directory on
    /// success.
    ///
    /// # Arguments
    /// * `certs_path` - Caller-supplied certificate folders; replaced in place
    ///   with the shared certs directory path after copying.
    /// * `tokens` - Permission token vector extended with tokens for each
    ///   copied certificate file.
    ///
    /// # Errors
    /// Returns `13400001` if the directory cannot be created or a folder path
    /// is invalid.
    fn apply_cert_path(
        &self,
        certs_path: &mut Vec<String>,
        tokens: &mut Vec<PermissionToken>,
    ) -> Result<(), i32> {
        let new_path = PathBuf::from(CERTS_PATH);
        if !new_path.exists() {
            fs::create_dir_all(new_path.as_path()).map_err(|e| {
                error!("Failed to create directory {:?}: {}", new_path, e);
                13400001
            })?;
        }

        for folder_path in certs_path.as_slice() {
            let folder = PathBuf::from(folder_path);
            if !folder.exists() || !folder.is_dir() {
                error!("bad certs_path");
                return Err(13400001);
            }

            if let Ok(entries) = fs::read_dir(&folder) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let exist_file_path = folder.join(path.file_name().unwrap_or_default());
                    let new_file_path = new_path.join(path.file_name().unwrap_or_default());

                    if !new_file_path.exists() {
                        if let Err(e) = fs::copy(&exist_file_path, &new_file_path) {
                            error!(
                                "Failed to copy file from {:?} to {:?}: {}",
                                exist_file_path, new_file_path, e
                            );
                            continue;
                        }
                    }

                    if let Err(e) =
                        fs::set_permissions(&new_file_path, fs::Permissions::from_mode(0o755))
                    {
                        error!("Failed to set permissions for {:?}: {}", new_file_path, e);
                    }

                    tokens.push(self.permission_manager.grant(&new_file_path)?);
                }
            }
        }

        if !certs_path.is_empty() {
            certs_path.clear();
            certs_path.push(CERTS_PATH.to_string());
        }

        Ok(())
    }

    /// Validates and prepares a user-provided download destination file.
    ///
    /// Rejects API9, background mode, and non-overwrite configurations, then
    /// resolves the file URI to a real path, opens/creates the target file,
    /// and pushes a `FileSpec` with the resulting file descriptor.
    ///
    /// # Arguments
    /// * `config` - Task configuration whose `file_specs` is extended with the
    ///   resolved user file.
    ///
    /// # Errors
    /// Returns `401` for unsupported configurations, or `13400001` if the
    /// target file cannot be opened.
    fn check_download_user_file(config: &mut TaskConfig) -> Result<(), i32> {
        if matches!(config.version, Version::API9) {
            return Err(401);
        }
        if matches!(config.common_data.mode, Mode::BackGround) {
            return Err(401);
        }
        if matches!(config.common_data.action, Action::Download) {
            if !config.overwrite {
                return Err(401);
            }
            let_cxx_string!(target_file = config.saveas.clone());
            let file_uri = request_utils::wrapper::FileUriGetRealPath(&target_file);
            // todo first_init, fdsan
            let fd = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_uri)
                .map_err(|e| {
                    error!("open fail: {}", e);
                    13400001
                })?
                .into_raw_fd();
            let file_name = config
                .saveas
                .clone()
                .rsplit_once('/')
                .map(|(_, name)| name.to_string())
                .unwrap_or_default();
            let file = FileSpec {
                name: "file".to_string(),
                path: config.saveas.clone(),
                file_name: file_name.clone(),
                mime_type: file_name
                    .rsplit_once('.')
                    .map(|(_, name)| name.to_string())
                    .unwrap_or_default(),
                is_user_file: true,
                fd: Some(fd),
            };
            config.file_specs.push(file);
        } else {
        }

        Ok(())
    }

    /// Opens a file through the data ability layer.
    ///
    /// # Arguments
    /// * `context` - Application context passed to the data ability.
    /// * `target_file` - File URI or path to open.
    ///
    /// # Returns
    /// The raw file descriptor on success, or a negative value on failure.
    fn data_ability_open_file(context: &Context, target_file: String) -> i32 {
        let_cxx_string!(target_file = target_file);
        request_data_ability::dataability::DataAbilityOpenFile(&context.inner, &target_file)
    }

    /// Validates a user-provided upload file and populates its descriptor.
    ///
    /// Opens the file via the data ability layer, stores the descriptor on the
    /// file spec, and fills in the file name and mime type when missing.
    ///
    /// # Arguments
    /// * `file_spec` - File specification updated with `fd`, `file_name`,
    ///   `mime_type`, and `name`.
    /// * `context` - Application context used for data ability access.
    ///
    /// # Errors
    /// Returns `401` if the data ability fails to open the file.
    fn check_upload_user_file(file_spec: &mut FileSpec, context: &Context) -> Result<(), i32> {
        let fd = Self::data_ability_open_file(context, file_spec.path.clone());
        if fd < 0 {
            return Err(401);
        }
        file_spec.fd = Some(fd);
        file_spec.file_name = file_spec
            .path
            .clone()
            .rsplit_once('/')
            .map(|(_, name)| name.to_string())
            .unwrap_or_default();
        if file_spec.mime_type.is_empty() {
            file_spec.mime_type = file_spec
                .file_name
                .rsplit_once('.')
                .map(|(_, name)| name.to_string())
                .unwrap_or_default();
        }
        if file_spec.name.is_empty() {
            file_spec.name.push_str("file");
        }
        Ok(())
    }

    /// Returns whether the path points to a user-managed file.
    ///
    /// A path is considered user-managed when it starts with the `file://docs/`
    /// or `file://media/` prefix.
    fn is_user_file(path: &str) -> bool {
        path.starts_with(DOCS_PREFIX) || path.starts_with(MEDIA_PREFIX)
    }

    /// Converts the download save-as path to an absolute sandbox path.
    ///
    /// For API9, absolute paths are used as-is and `internal://cache/` paths
    /// are resolved against the cache directory. For API10, the path is
    /// converted to an absolute path and verified to reside under one of the
    /// allowed application base areas, creating parent directories as needed.
    ///
    /// # Arguments
    /// * `config` - Task configuration whose `saveas` is updated with the
    ///   resolved absolute path.
    /// * `context` - Application context used for path resolution.
    ///
    /// # Returns
    /// The resolved absolute `PathBuf`.
    ///
    /// # Errors
    /// Returns `401` if the path is empty, too long, or outside the allowed
    /// application areas.
    fn convert_download_path(config: &mut TaskConfig, context: &Context) -> Result<PathBuf, i32> {
        let saveas = config.saveas.clone();
        match config.version {
            Version::API9 => {
                if saveas.starts_with(ABSOLUTE_PREFIX) {
                    // if saveas.len() == ABSOLUTE_PREFIX.len() {
                    //     return Err(401);
                    // }
                    return Ok(PathBuf::from(saveas));
                } else {
                    let file_name = match saveas.find(INTERNAL_PATTERN) {
                        Some(0) => saveas.split_at(INTERNAL_PATTERN.len()).1,
                        _ => &saveas,
                    };
                    if file_name.is_empty() {
                        return Err(401);
                    }
                    let cache_dir = context.get_cache_dir();

                    if cache_dir.len() + file_name.len() + 1 > MAX_FILE_PATH_LENGTH {
                        return Err(401);
                    }
                    config.saveas = format!("{}/{}", cache_dir, file_name);
                    // todo: realpath?
                    // todo: check api9 path verify
                    Ok(PathBuf::from(config.saveas.clone()))
                }
            }

            Version::API10 => {
                let absolute_path = Self::convert_to_absolute_path(&context, &saveas)?;
                if context.get_base_dir().is_empty() {
                    error!("base dir empty");
                    return Err(401);
                }

                if !absolute_path.starts_with(AREA1)
                    && !absolute_path.starts_with(AREA2)
                    && !absolute_path.starts_with(AREA5)
                {
                    error!("not belong app");
                    return Err(401);
                }
                config.saveas = absolute_path.to_string_lossy().to_string();
                if let Some(parent) = absolute_path.parent() {
                    if let Err(e) = fs::create_dir_all(parent) {
                        error!("dir create error: {}", e);
                        return Err(401);
                    }
                }
                Ok(absolute_path)
            }
        }
    }

    /// Converts an upload file spec path to an absolute sandbox path.
    ///
    /// For API9, absolute paths are used as-is and `internal://cache/` paths
    /// are resolved against the cache directory. For API10, the path is
    /// converted to an absolute path and verified to reside under one of the
    /// allowed application base areas.
    ///
    /// # Arguments
    /// * `file_spec` - File specification whose `path` is updated with the
    ///   resolved absolute path.
    /// * `version` - API version governing path resolution rules.
    /// * `context` - Application context used for path resolution.
    ///
    /// # Returns
    /// The resolved absolute `PathBuf`.
    ///
    /// # Errors
    /// Returns `401` or `13400001` if the path is empty, too long, or outside
    /// the allowed application areas.
    fn convert_upload_path(
        file_spec: &mut FileSpec,
        version: &Version,
        context: &Context,
    ) -> Result<PathBuf, i32> {
        let path = file_spec.path.clone();
        match version {
            Version::API9 => {
                if path.starts_with(ABSOLUTE_PREFIX) {
                    if path.len() == ABSOLUTE_PREFIX.len() {
                        return Err(401);
                    }
                    return Ok(PathBuf::from(path));
                } else {
                    let file_name = match path.find(INTERNAL_PATTERN) {
                        Some(0) => path.split_at(INTERNAL_PATTERN.len()).1,
                        _ => &path,
                    };
                    if file_name.is_empty() {
                        return Err(13400001);
                    }
                    let cache_dir = context.get_cache_dir();

                    if cache_dir.len() + file_name.len() + 1 > MAX_FILE_PATH_LENGTH {
                        return Err(401);
                    }
                    file_spec.path = format!("{}/{}", cache_dir, file_name);
                    // todo: realpath?
                    // todo: check api9 path verify
                    Ok(PathBuf::from(file_spec.path.clone()))
                }
            }

            Version::API10 => {
                let absolute_path = Self::convert_to_absolute_path(&context, &path)?;
                if context.get_base_dir().is_empty() {
                    return Err(401);
                }
                if !absolute_path.starts_with(AREA1)
                    && !absolute_path.starts_with(AREA2)
                    && !absolute_path.starts_with(AREA5)
                {
                    return Err(401);
                }
                file_spec.path = absolute_path.to_string_lossy().to_string();
                Ok(absolute_path)
            }
        }
    }

    /// Normalizes a path by collapsing `.` and `..` segments.
    ///
    /// Splits the path on `/`, discards empty and `.` segments, and resolves
    /// `..` by popping the previous segment. The result is an absolute path
    /// with redundant separators removed.
    ///
    /// # Arguments
    /// * `path` - Path to normalize.
    ///
    /// # Returns
    /// The normalized absolute path string.
    ///
    /// # Errors
    /// Returns `401` if a `..` segment would escape above the root.
    fn normalize(path: &str) -> Result<String, i32> {
        let mut stk = Vec::new();
        for seg in path.split('/') {
            match seg {
                "" | "." => {}
                ".." => {
                    if stk.pop().is_none() {
                        error!("bad path with ..");
                        return Err(401);
                    }
                }
                _ => stk.push(seg),
            }
        }
        Ok(format!("/{}", stk.join("/")))
    }

    /// Converts a user-supplied path to an absolute sandbox path.
    ///
    /// Handles absolute paths, `file://` URIs (validating the bundle name
    /// against the context), `internal://` URIs resolved against the base
    /// directory, and relative paths resolved against the cache directory.
    /// Each result is normalized to remove redundant segments.
    ///
    /// # Arguments
    /// * `context` - Application context providing bundle name and directory
    ///   roots.
    /// * `path` - Path to convert.
    ///
    /// # Returns
    /// The resolved absolute `PathBuf`.
    ///
    /// # Errors
    /// Returns `401` or `13400001` if the path is empty, malformed, or
    /// references a foreign bundle.
    fn convert_to_absolute_path(context: &Context, path: &str) -> Result<PathBuf, i32> {
        if let Some(0) = path.find(ABSOLUTE_PREFIX) {
            // if path.len() == ABSOLUTE_PREFIX.len() {
            //     return Err(DownloadPathError::EmptyPath);
            // }
            return Ok(PathBuf::from(Self::normalize(path)?));
        }

        if path.starts_with(FILE_PREFIX) {
            let path = path.split_at(FILE_PREFIX.len()).1;
            if path.is_empty() {
                error!("convert_to_absolute_path path empty");
                return Err(401);
            }
            let Some(index) = path.find('/') else {
                error!("convert_to_absolute_path / not found ");
                return Err(401);
            };
            let (bundle_name, path) = path.split_at(index);
            if bundle_name != context.get_bundle_name() {
                error!("path bundlename error");
                return Err(401);
            }
            return Ok(PathBuf::from(Self::normalize(path)?));
        }

        if let Some(0) = path.find(INTERNAL_PREFIX) {
            let path = path.split_at(INTERNAL_PREFIX.len()).1;
            if path.is_empty() {
                return Err(13400001);
            }
            let base_dir = context.get_base_dir();
            return Ok(PathBuf::from(Self::normalize(&format!(
                "{}/{}",
                base_dir, path
            ))?));
        }

        let path = if let Some(0) = path.find(RELATIVE_PREFIX) {
            path.split_at(RELATIVE_PREFIX.len()).1
        } else {
            path
        };

        if path.is_empty() {
            return Err(13400001);
        }
        let cache_dir = context.get_cache_dir();

        Ok(PathBuf::from(Self::normalize(&format!(
            "{}/{}",
            cache_dir, path
        ))?))
    }

    /// Creates and sets permissions on the download destination file.
    ///
    /// When `overwrite` is disabled and the file already exists, returns an
    /// error. Otherwise creates the file if needed and applies `0o644`
    /// permissions.
    ///
    /// # Arguments
    /// * `path` - Destination file path.
    /// * `config` - Task configuration supplying the `overwrite` flag and API
    ///   version.
    ///
    /// # Errors
    /// Returns `13400002` (API9) or `13400001` (API10) when the file exists
    /// without overwrite, and `13400001` if the file cannot be created or
    /// chmodded.
    fn chmod_download_file(path: &PathBuf, config: &TaskConfig) -> Result<(), i32> {
        // todo first init
        if !config.overwrite && path.exists() {
            error!("file exists");
            if config.version == Version::API9 {
                return Err(13400002);
            } else {
                return Err(13400001);
            }
        }

        let _ = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .map_err(|_| {
                error!("open fail");
                13400001
            })?;

        if let Err(_) = fs::set_permissions(&path, fs::Permissions::from_mode(0o644)) {
            error!("permission fail");
            return Err(13400001);
        }

        Ok(())
    }

    /// Validates an upload source file exists and sets its permissions.
    ///
    /// Checks that the path points to an existing regular file that can be
    /// opened for reading, then applies `0o644` permissions.
    ///
    /// # Arguments
    /// * `path` - Upload source file path.
    /// * `version` - API version governing the error codes returned.
    ///
    /// # Errors
    /// Returns `13400001` (API10) or `13400002` (API9) if the file does not
    /// exist or cannot be opened.
    fn chmod_upload_file(path: &PathBuf, version: &Version) -> Result<(), i32> {
        if !path.exists() || !path.is_file() {
            error!("path error: path: {}", path.to_string_lossy().to_string());
            return Err(if matches!(version, Version::API10) {
                13400001
            } else {
                13400002
            });
        }

        match File::open(&path) {
            Ok(_) => {
                info!("upload file open ok");
            }
            Err(e) => {
                error!("GetFd failed to open file errno {}", e);
                return Err(if matches!(version, Version::API10) {
                    13400001
                } else {
                    13400002
                });
            }
        }

        if let Err(e) = fs::set_permissions(&path, fs::Permissions::from_mode(0o644)) {
            error!("upload file chmod fail: {}", e);
        }

        Ok(())
    }

    /// Validates a single sandbox upload file and grants its permission token.
    ///
    /// Converts the file spec path to an absolute path, validates and chmods
    /// the file, issues a permission token, and fills in the file name, mime
    /// type, and field name when missing.
    ///
    /// # Arguments
    /// * `file_spec` - File specification updated with the resolved path and
    ///   metadata.
    /// * `context` - Application context used for path resolution.
    /// * `version` - API version governing path resolution rules.
    ///
    /// # Returns
    /// The permission token granted for the resolved upload file.
    ///
    /// # Errors
    /// Returns an `i32` error code if path conversion, validation, or
    /// permission granting fails.
    fn check_upload_file(
        &self,
        file_spec: &mut FileSpec,
        context: &Context,
        version: &Version,
    ) -> Result<PermissionToken, i32> {
        let path_buf = Self::convert_upload_path(file_spec, version, context)?;
        Self::chmod_upload_file(&path_buf, version)?;
        let token = self.permission_manager.grant(&path_buf)?;
        file_spec.path = path_buf.to_string_lossy().to_string();
        file_spec.file_name = file_spec
            .path
            .clone()
            .rsplit_once('/')
            .map(|(_, name)| name.to_string())
            .unwrap_or_default();
        if file_spec.mime_type.is_empty() {
            file_spec.mime_type = file_spec
                .file_name
                .rsplit_once('.')
                .map(|(_, name)| name.to_string())
                .unwrap_or_default();
        }
        if file_spec.name.is_empty() {
            file_spec.name.push_str("file");
        }
        Ok(token)
    }

    /// Populates trusted certificate paths for HTTPS URLs.
    ///
    /// For HTTPS URLs, resolves the hostname and queries the trust anchors for
    /// it, storing the resulting certificate paths on the config. Non-HTTPS
    /// URLs are skipped.
    ///
    /// # Arguments
    /// * `config` - Task configuration whose `certs_path` is populated in
    ///   place.
    fn get_cert_path(config: &mut TaskConfig) {
        if !config.url.starts_with("https://") {
            debug!("Using Http");
            return;
        }

        let hostname = crate::verify::url::get_hostname_from_url(&config.url);
        debug!("Hostname is {}", hostname);

        let_cxx_string!(hostname_str = hostname);
        let certs_path = request_utils::wrapper::GetTrustAnchorsForHostName(&hostname_str);
        config.certs_path = certs_path;
    }

    /// Populates certificate pins for the task URL hostname.
    ///
    /// Resolves the hostname from the task URL and queries the certificate
    /// pins associated with it, storing them on the config.
    ///
    /// # Arguments
    /// * `config` - Task configuration whose `certificate_pins` is populated in
    ///   place.
    fn get_certificate_pins(config: &mut TaskConfig) {
        let hostname = crate::verify::url::get_hostname_from_url(&config.url);
        debug!("Hostname is {}", hostname);

        let_cxx_string!(hostname_str = hostname);
        let certificate_pins = request_utils::wrapper::GetCertificatePinsForHostName(&hostname_str);
        config.certificate_pins = certificate_pins;
    }

    /// Reads the full contents of a file into a byte vector.
    ///
    /// # Arguments
    /// * `file_path` - Path of the file to read.
    ///
    /// # Returns
    /// `Some(bytes)` on success, or `None` if the file cannot be read.
    pub fn read_bytes_from_file(file_path: &str) -> Option<Vec<u8>> {
        match fs::read(file_path) {
            Ok(data) => Some(data),
            Err(e) => {
                error!("Failed to read file {}: {}", file_path, e);
                None
            }
        }
    }
}
