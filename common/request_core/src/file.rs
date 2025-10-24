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

//! File specifications for network operations.
//! 
//! This module defines structures for representing files used in network requests,
//! including uploads and downloads, with support for both system-managed and user-provided files.

use std::os::fd::RawFd;

/// Specification for a file used in network operations.
///
/// Represents a file with metadata for use in upload tasks or file-related operations.
/// Supports both system-managed files and user-provided file descriptors.
///
/// # Examples
///
/// ```rust
/// let mut file_spec = FileSpec::new();
/// file_spec.name = "profile_picture".to_string();
/// file_spec.path = "/data/profile.jpg".to_string();
/// file_spec.file_name = "profile.jpg".to_string();
/// file_spec.mime_type = "image/jpeg".to_string();
/// ```
#[derive(Clone, Debug)]
pub struct FileSpec {
    /// Form field name for the file in multi-part requests.
    pub name: String,
    /// Absolute path to the file on the file system.
    pub path: String,
    /// Original name of the file.
    pub file_name: String,
    /// MIME type of the file content.
    pub mime_type: String,
    /// Flag indicating if the file is provided by the user.
    pub is_user_file: bool,
    /// File descriptor for user-provided files.
    ///
    /// # Safety
    ///
    /// The file descriptor must be valid and properly managed to avoid resource leaks.
    pub fd: Option<RawFd>,
}

impl FileSpec {
    /// Creates a new empty `FileSpec` with default values.
    ///
    /// # Notes
    ///
    /// The returned instance has empty string values and is not configured as a user file.
    /// Fields should be populated before using the specification in actual operations.
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
            path: "".to_owned(),
            file_name: "".to_owned(),
            mime_type: "".to_owned(),
            is_user_file: false,
            fd: None,
        }
    }
}
