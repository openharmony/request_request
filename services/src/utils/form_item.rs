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

use std::fs::File;
use std::os::fd::{IntoRawFd, RawFd};

/// Specifies details about a file for upload operations.
///
/// Contains metadata about a file including its name, path, MIME type, and
/// whether it's a user-provided file with an associated file descriptor.
#[derive(Clone, Debug)]
pub struct FileSpec {
    /// The form field name associated with this file.
    pub name: String,
    /// The full path to the file on disk.
    pub path: String,
    /// The name of the file without directory information.
    pub file_name: String,
    /// The MIME type of the file (e.g., "image/jpeg").
    pub mime_type: String,
    /// Flag indicating whether this is a user-provided file.
    pub is_user_file: bool,
    /// File descriptor for the opened file, only valid when `is_user_file` is true.
    pub fd: Option<RawFd>,
}

impl FileSpec {
    /// Creates a new file specification for a user-provided file.
    ///
    /// # Parameters
    /// - `file`: The user-provided `File` object to be uploaded.
    ///
    /// # Returns
    /// A new `FileSpec` with `is_user_file` set to `true` and the file descriptor
    /// extracted from the provided file.
    ///
    /// # Safety
    /// This function consumes the `File` and converts it to a raw file descriptor.
    /// It's the caller's responsibility to ensure the file descriptor is properly
    /// managed and closed when no longer needed.
    ///
    /// # Examples
    /// ```
    /// use std::fs::File;
    /// use request_services::utils::form_item::FileSpec;
    /// 
    /// // Open a file
    /// let file = File::open("example.txt").expect("Failed to open file");
    /// 
    /// // Create a file specification for upload
    /// let mut file_spec = FileSpec::user_file(file);
    /// 
    /// // Set additional metadata
    /// file_spec.name = "upload_file".to_string();
    /// file_spec.file_name = "example.txt".to_string();
    /// file_spec.mime_type = "text/plain".to_string();
    /// ```
    pub fn user_file(file: File) -> Self {
        // Create a file spec with empty metadata strings but set is_user_file flag
        // and extract the file descriptor from the provided File object
        Self {
            name: "".to_string(),
            path: "".to_string(),
            file_name: "".to_string(),
            mime_type: "".to_string(),
            is_user_file: true,
            fd: Some(file.into_raw_fd()),
        }
    }
}

/// Represents a key-value pair in a form submission.
///
/// Used for including text-based form data in requests alongside file uploads.
#[derive(Clone, Debug)]
pub(crate) struct FormItem {
    /// The name of the form field.
    pub(crate) name: String,
    /// The value associated with the form field.
    pub(crate) value: String,
}
