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
use std::os::fd::{AsRawFd, RawFd};

/// File Spec
#[derive(Clone, Debug)]
pub struct FileSpec {
    /// Name
    pub name: String,
    /// path
    pub path: String,
    /// file_name
    pub file_name: String,
    /// mime_type
    pub mime_type: String,
    /// is_user_file
    pub is_user_file: bool,
    /// Only for user file.
    pub fd: Option<RawFd>,
}

impl FileSpec {
    /// Create a new file spec with user file.
    pub fn user_file(f: &File) -> Self {
        Self {
            name: "".to_string(),
            path: "".to_string(),
            file_name: "".to_string(),
            mime_type: "".to_string(),
            is_user_file: true,
            fd: Some(f.as_raw_fd()),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FormItem {
    pub(crate) name: String,
    pub(crate) value: String,
}
